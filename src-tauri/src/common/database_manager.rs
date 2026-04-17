use std::sync::Mutex;
use lazy_static::lazy_static;
use rusqlite::Connection;
use tauri::AppHandle;

// 数据库操作
lazy_static! {
    pub static ref DB_CONNECTION: Mutex<Connection> = Mutex::new(Connection::open("translate.sqlite").unwrap());
}

pub fn init_database(app_handle: &AppHandle) -> anyhow::Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute_batch(r#"

    "#)?;
    Ok(())
}