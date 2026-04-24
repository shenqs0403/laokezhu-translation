use rusqlite::params;
use crate::dao::DB_CONN;

// keyValue操作
static SQL_INSERT_OR_UPDATE: &str = "insert or replace into  key_value (config_key, config_value) values (?1,?2)";
static QUERY_ONE: &str = "select config_value from key_value where config_key = ?1";

pub fn create_key_value_table_and_init_data() -> anyhow::Result<()> {
    let create = r#"create table key_value
        (
            config_key text primary key ,
            config_value any not null
        )"#;
    let conn = DB_CONN.lock().unwrap();
    conn.execute(create, [])?;
    conn.execute(SQL_INSERT_OR_UPDATE,["shortcut","Alt+ContrlLeft+KeyQ"])?;
    conn.execute(SQL_INSERT_OR_UPDATE,params!["swipe",false])?;
    Ok(())
}