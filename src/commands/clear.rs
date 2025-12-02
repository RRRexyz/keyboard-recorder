use anyhow::Result;

use crate::{clt::ClearArgs, db::clear_records, logging};

/// Execute the `kero clear` command.
pub fn run(args: ClearArgs) -> Result<()> {
    let backup_path = clear_records(args.backup)?;
    match (args.backup, backup_path) {
        (true, Some(path)) => logging::info(format!("Records cleared. Backup stored as {}.", path)),
        (true, None) => {
            logging::info("Records cleared. Backup skipped because no database file was present.")
        }
        _ => logging::info("Records cleared."),
    }
    Ok(())
}
