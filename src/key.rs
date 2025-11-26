pub mod key {
    use std::process::exit;
    use std::time::Duration;
    use std::{
        any::Any,
        collections::HashSet,
        sync::{Arc, Mutex},
    };

    use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};

    /// 初始化键盘事件处理器
    pub fn init_handler() -> (DeviceEventsHandler, Box<dyn Any>, Box<dyn Any>) {
        let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
            .expect("Could not initialize event handler.");

        // 创建集合记录同一时间按下的按键
        let keys_set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

        let keys_set_clone = Arc::clone(&keys_set);
        let press_key_guard = event_handler.on_key_down(move |key| {
            record_press_key(&key, &keys_set_clone);
            if let Ok(set) = keys_set_clone.lock() {
                println!("Currently pressed keys: {:?}", set);
            }
        });

        let keys_set_clone_2 = Arc::clone(&keys_set);
        let release_key_guard = event_handler.on_key_up(move |key| {
            record_release_key(key, &keys_set_clone_2);
            if let Ok(set) = keys_set_clone_2.lock() {
                println!("Currently pressed keys: {:?}", set);
            }
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

    /// 释放按键时将其从集合中移除
    fn record_release_key(key: &Keycode, keys_set: &Arc<Mutex<HashSet<String>>>) {
        if let Ok(mut set) = keys_set.lock() {
            set.remove(&key.to_string());
        }
    }
}
