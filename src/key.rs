pub mod key {
    use std::process::exit;
    use std::time::Duration;
    use std::{
        any::Any,
        collections::HashSet,
        sync::atomic::{AtomicBool, Ordering},
        sync::{Arc, Mutex},
    };

    use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};

    /// 初始化键盘事件处理器
    pub fn init_handler() -> (DeviceEventsHandler, Box<dyn Any>, Box<dyn Any>) {
        let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
            .expect("Could not initialize event handler.");

        // 创建集合记录同一时间按下的按键
        let keys_set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
        // 如果上一次事件是某按键被按下，则设置为 true，否则为 false
        let last_down_flag = Arc::new(AtomicBool::new(false));

        let keys_set_clone = Arc::clone(&keys_set);
        let last_down_flag_clone = Arc::clone(&last_down_flag);
        let press_key_guard = event_handler.on_key_down(move |key| {
            record_press_key(&key, &keys_set_clone);
            last_down_flag_clone.store(true, Ordering::SeqCst);
        });

        let keys_set_clone_2 = Arc::clone(&keys_set);
        let last_down_flag_clone_2 = Arc::clone(&last_down_flag);
        let release_key_guard = event_handler.on_key_up(move |key| {
            if let Ok(set) = keys_set_clone_2.lock() {
                if last_down_flag_clone_2.load(Ordering::SeqCst) {
                    println!("Currently pressed keys: {:?}", set);
                }
            }
            record_release_key(&key, &keys_set_clone_2);
            last_down_flag_clone_2.store(false, Ordering::SeqCst);
            if let Keycode::Escape = key {
                exit(0);
            }
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

    /// 有按键被释放时清空集合
    fn record_release_key(key: &Keycode, keys_set: &Arc<Mutex<HashSet<String>>>) {
        if let Ok(mut set) = keys_set.lock() {
            set.remove(&key.to_string());
        }
    }
}
