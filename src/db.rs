use std::collections::HashSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::{Connection, params};

use crate::logging;

/// 数据库名称
const DB_NAME: &str = "keyboard";

/// 获取数据库路径
fn db_path() -> String {
    format!("{}.db", DB_NAME)
}

/// 初始化数据库，返回数据库连接
pub fn init_database() -> Result<Connection> {
    let conn = Connection::open(db_path()).context("Cannot connect to database.")?;
    let create_statement = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            keys TEXT NOT NULL,
            single BOOLEAN NOT NULL,
            press_times INTEGER NOT NULL 
        )",
        DB_NAME
    );
    conn.execute(&create_statement, params![])
        .context("Cannot create table in database.")?;

    Ok(conn)
}

#[derive(Debug, Clone)]
pub struct KeyRecord {
    pub keys: String,
    pub single: bool,
    pub press_times: isize,
}

fn query_keys_from_db(conn: &Connection, keys: &str) -> Result<Option<(String, bool, isize)>> {
    let query_statement = format!(
        "SELECT keys, single, press_times FROM {} WHERE keys = ?1",
        DB_NAME
    );
    let mut stmt = conn
        .prepare(&query_statement)
        .context("Cannot prepare query statement.")?;
    let mut rows = stmt
        .query(params![keys])
        .context("Cannot execute query statement.")?;

    if let Some(row) = rows.next().context("Cannot fetch row from query result.")? {
        let keys: String = row.get(0).context("Cannot get keys from row.")?;
        let single: bool = row.get(1).context("Cannot get single from row.")?;
        let press_times: isize = row.get(2).context("Cannot get press_times from row.")?;
        Ok(Some((keys, single, press_times)))
    } else {
        Ok(None)
    }
}

pub fn fetch_records(filter_single: Option<bool>) -> Result<Vec<KeyRecord>> {
    let conn = init_database()?;
    let mut query = format!("SELECT keys, single, press_times FROM {}", DB_NAME);
    if filter_single.is_some() {
        query.push_str(" WHERE single = ?1");
    }
    query.push_str(" ORDER BY press_times DESC, keys ASC");

    let mut stmt = conn
        .prepare(&query)
        .context("Cannot prepare fetch statement.")?;

    let mut rows = if let Some(single_value) = filter_single {
        stmt.query(params![single_value])
    } else {
        stmt.query([])
    }
    .context("Cannot execute fetch statement.")?;

    let mut records = Vec::new();
    while let Some(row) = rows.next().context("Cannot fetch row for record.")? {
        records.push(KeyRecord {
            keys: row.get(0).context("Cannot read keys column.")?,
            single: row.get(1).context("Cannot read single column.")?,
            press_times: row.get(2).context("Cannot read press_times column.")?,
        });
    }

    Ok(records)
}

pub fn clear_records(backup: bool) -> Result<Option<String>> {
    let mut backup_path = None;
    if backup {
        backup_path = backup_database()?;
    }

    let conn = init_database()?;
    let delete_statement = format!("DELETE FROM {}", DB_NAME);
    conn.execute(&delete_statement, params![])
        .context("Cannot clear records from database.")?;
    Ok(backup_path)
}

fn backup_database() -> Result<Option<String>> {
    let path = db_path();
    let db_file = Path::new(&path);
    if !db_file.exists() {
        return Ok(None);
    }
    let backup_path = format!("{}.backup", path);
    fs::copy(db_file, &backup_path)
        .with_context(|| format!("Failed to backup database to {}", backup_path))?;
    Ok(Some(backup_path))
}

pub fn insert_keys_to_db(conn: &Connection, keys_set: &HashSet<String>) {
    if keys_set.is_empty() {
        return;
    }

    let insert_statement = format!(
        "INSERT INTO {} (keys, single, press_times) VALUES (?1, ?2, ?3)",
        DB_NAME
    );

    let mut sorted_keys: Vec<String> = keys_set.iter().cloned().collect();
    sorted_keys.sort();
    let keys = sorted_keys.join("+");
    let single = sorted_keys.len() == 1;

    match query_keys_from_db(conn, &keys) {
        Ok(Some((_keys, _single, press_times))) => {
            let update_statement =
                format!("UPDATE {} SET press_times = ?1 WHERE keys = ?2", DB_NAME);
            if let Err(e) = conn.execute(&update_statement, params![press_times + 1, keys]) {
                logging::error(format!("Failed to update keys in database: {}", e));
            }
        }
        Ok(None) => {
            if let Err(e) = conn.execute(&insert_statement, params![keys, single, 1]) {
                logging::error(format!("Failed to insert keys into database: {}", e));
            }
        }
        Err(e) => {
            logging::error(format!("Failed to query keys from database: {}", e));
        }
    }
}
