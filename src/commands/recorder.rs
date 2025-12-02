use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{Context, Result};

use crate::{db::init_database, key::init_handler, logging};

/// Run the foreground recorder loop.
pub fn run() -> Result<()> {
    let conn = Arc::new(Mutex::new(
        init_database().context("Failed to init database")?,
    ));

    let conn_clone = Arc::clone(&conn);
    let (_handler, _press_guard, _release_guard) = init_handler(conn_clone);

    logging::info("Keyboard recorder is running. Press Esc to exit.");
    thread::park();
    #[allow(unreachable_code)]
    Ok(())
}
