use rusqlite::params;
use crate::dao::{KeyValue, DB_CONN};

pub static KEY_SHORTCUT: &str = "basic.shortcut";
pub static KEY_SWIPE: &str = "basic.swipe";
pub static KEY_DB_VERSION: &str = "version";

// keyValue操作
static SQL_INSERT_OR_UPDATE: &str = "insert or replace into  key_value (config_key, config_value) values (?1,?2)";
static QUERY_ONE: &str = "select * from key_value where config_key = ?1";

pub fn create_key_value_table_and_init_data() -> anyhow::Result<()> {
    let create = r#"create table key_value(
            config_key text primary key ,
            config_value text not null)"#;
    let conn = DB_CONN.lock().unwrap();
    conn.execute(create, [])?;
    conn.execute(SQL_INSERT_OR_UPDATE,[KEY_SHORTCUT,"Alt+Control+KeyQ"])?;
    conn.execute(SQL_INSERT_OR_UPDATE,params![KEY_SWIPE,300])?;
    conn.execute(SQL_INSERT_OR_UPDATE,params![KEY_DB_VERSION,0])?;
    Ok(())
}

pub fn set_item(key: String, value: String) -> tauri::Result<usize> {
    let conn = DB_CONN.lock().unwrap();
    let i = conn.execute(SQL_INSERT_OR_UPDATE, [key, value])
        .map_err(anyhow::Error::from)?;
    Ok(i)
}

pub fn get_item(key: String) -> tauri::Result<KeyValue> {
    let conn = DB_CONN.lock().unwrap();
    let value = conn.query_one(QUERY_ONE, [key], row_to_key_value)
        .map_err(anyhow::Error::from)?;
    Ok(value)
}

fn row_to_key_value(row: &rusqlite::Row) -> rusqlite::Result<KeyValue> {
    Ok(KeyValue{ key: row.get(0)?, value: row.get(1)? })
}