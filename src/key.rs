use std::time::Duration;
use std::{
    any::Any,
    collections::HashSet,
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
};

use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use rusqlite::Connection;

use crate::{db::insert_keys_to_db, logging};

/// 初始化键盘事件处理器
pub fn init_handler(
    conn: Arc<Mutex<Connection>>,
) -> (DeviceEventsHandler, Box<dyn Any>, Box<dyn Any>) {
    let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
        .expect("Could not initialize event handler.");

    // 创建集合记录同一时间按下的按键
    let keys_set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    // 如果上一次事件是某按键被按下，则设置为 true，否则为 false
    let last_down_flag = Arc::new(AtomicBool::new(false));

    let keys_set_clone = Arc::clone(&keys_set);
    let last_down_flag_clone = Arc::clone(&last_down_flag);
    let press_key_guard = event_handler.on_key_down(move |key| {
        record_press_key(key, &keys_set_clone);
        last_down_flag_clone.store(true, Ordering::SeqCst);
    });

    let keys_set_clone_2 = Arc::clone(&keys_set);
    let last_down_flag_clone_2 = Arc::clone(&last_down_flag);
    let conn_clone = Arc::clone(&conn);
    let release_key_guard = event_handler.on_key_up(move |key| {
        let keys_snapshot = {
            if !last_down_flag_clone_2.load(Ordering::SeqCst) {
                None
            } else if let Ok(set) = keys_set_clone_2.lock() {
                if set.is_empty() {
                    None
                } else {
                    Some(set.clone())
                }
            } else {
                None
            }
        };

        if let Some(snapshot) = keys_snapshot {
            logging::info(format!("Currently pressed keys: {:?}", snapshot));
            if let Ok(db_conn) = conn_clone.lock() {
                insert_keys_to_db(&db_conn, &snapshot);
            }
        }

        record_release_key(key, &keys_set_clone_2);
        last_down_flag_clone_2.store(false, Ordering::SeqCst);
    });
    (
        event_handler,
        Box::new(press_key_guard),
        Box::new(release_key_guard),
    )
}

/// 按下按键时将其写入集合
fn record_press_key(key: &Keycode, keys_set: &Arc<Mutex<HashSet<String>>>) {
    if let Ok(mut set) = keys_set.lock() {
        set.insert(key.to_string());
    }
}

/// 有按键被释放时将其从集合中移除
fn record_release_key(key: &Keycode, keys_set: &Arc<Mutex<HashSet<String>>>) {
    if let Ok(mut set) = keys_set.lock() {
        set.remove(&key.to_string());
    }
}
