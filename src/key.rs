pub mod key {
    use std::any::Any;
    use std::time::Duration;

    use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};

    pub fn init_handler() -> (DeviceEventsHandler, Box<dyn Any>, Box<dyn Any>) {
        let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
            .expect("Could not initialize event handler.");
        let press_key_guard = event_handler.on_key_down(|key| {
            println!("Key pressed: {:?}", key);
        });
        let release_key_guard = event_handler.on_key_up(|key| {
            println!("Key released: {:?}", key);
            if let Keycode::Escape = key {
                std::process::exit(0);
            }
        });
        (
            event_handler,
            Box::new(press_key_guard),
            Box::new(release_key_guard),
        )
    }
}
