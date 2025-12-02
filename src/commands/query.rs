use anyhow::Result;

use crate::{
    clt::QueryArgs,
    db::{KeyRecord, fetch_records},
};

/// Execute the `kero query` command.
pub fn run(args: QueryArgs) -> Result<()> {
    let filter = if args.single_only {
        Some(true)
    } else if args.combo_only {
        Some(false)
    } else {
        None
    };

    let records = fetch_records(filter)?;
    if records.is_empty() {
        println!("No key records found.");
        return Ok(());
    }

    print_records(&records);
    Ok(())
}

fn print_records(records: &[KeyRecord]) {
    let keys_width = records
        .iter()
        .map(|r| r.keys.len())
        .max()
        .unwrap_or(4)
        .max("Keys".len());
    let type_width = ["Type".len(), "Single".len(), "Combo".len()]
        .into_iter()
        .max()
        .unwrap_or(4);
    let count_width = records
        .iter()
        .map(|r| r.press_times.to_string().len())
        .max()
        .unwrap_or(5)
        .max("Count".len());

    let divider = format!(
        "+-{keys_dash}-+-{type_dash}-+-{count_dash}-+",
        keys_dash = "-".repeat(keys_width),
        type_dash = "-".repeat(type_width),
        count_dash = "-".repeat(count_width)
    );
    let header = format!(
        "| {keys:^keys_width$} | {kind:^type_width$} | {count:^count_width$} |",
        keys = "Keys",
        kind = "Type",
        count = "Count",
        keys_width = keys_width,
        type_width = type_width,
        count_width = count_width
    );

    println!("{}", divider);
    println!("{}", header);
    println!("{}", divider);
    for record in records {
        let entry = format!(
            "| {keys:<keys_width$} | {kind:^type_width$} | {count:>count_width$} |",
            keys = &record.keys,
            kind = if record.single { "Single" } else { "Combo" },
            count = record.press_times,
            keys_width = keys_width,
            type_width = type_width,
            count_width = count_width
        );
        println!("{}", entry);
    }
    println!("{}", divider);
}
