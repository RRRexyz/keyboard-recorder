use keyboard_recorder::key::key::init_handler;

fn main() {
    let (_handler, _press_guard, _release_guard) = init_handler();

    loop {}
}
