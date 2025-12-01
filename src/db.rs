pub mod db {
    use std::collections::HashSet;

    use anyhow::{Context, Result};
    use rusqlite::{Connection, params};

    /// 数据库名称
    const DB_NAME: &str = "keyboard";

    /// 获取数据库路径
    fn db_path() -> String {
        format!("{}.db", DB_NAME)
    }

    /// 初始化数据库，返回数据库连接
    pub fn init_database() -> Result<Connection> {
        let conn = Connection::open(&db_path()).context("Cannot connect to database.")?;
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
                    eprintln!("Failed to update keys in database: {}", e);
                }
            }
            Ok(None) => {
                if let Err(e) = conn.execute(&insert_statement, params![keys, single, 1]) {
                    eprintln!("Failed to insert keys into database: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to query keys from database: {}", e);
            }
        }
    }
}
