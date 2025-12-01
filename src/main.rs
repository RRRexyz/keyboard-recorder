use keyboard_recorder::{db::db::init_database, key::key::init_handler};
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let conn = Arc::new(Mutex::new(
        init_database().expect("Failed to init database."),
    ));

    let conn_clone = Arc::clone(&conn);
    let (_handler, _press_guard, _release_guard) = init_handler(conn_clone);

    println!("Keyboard recorder is running. Press Esc to exit.");
    thread::park();
}
