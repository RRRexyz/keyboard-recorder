pub mod db {
    use anyhow::{Context, Result};
    use rusqlite::Connection;

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
                key TEXT NOT NULL,
                press_times INTEGER NOT NULL 
            )",
            DB_NAME
        );
        conn.execute(&create_statement, ())
            .context("Cannot create table in database.")?;

        Ok(conn)
    }
}
