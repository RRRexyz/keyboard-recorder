use keyboard_recorder::{db::db::init_database, key::key::init_handler};

fn main() {
    let _conn = init_database().expect("Failed to init database.");

    let (_handler, _press_guard, _release_guard) = init_handler();

    loop {}
}
