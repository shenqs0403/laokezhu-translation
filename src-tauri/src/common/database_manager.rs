use std::sync::Mutex;
use lazy_static::lazy_static;
use rusqlite::Connection;
use tauri::AppHandle;
use tauri_plugin_log::log::debug;

// 数据库操作
lazy_static! {
    pub static ref DB_CONNECTION: Mutex<Connection> = Mutex::new(Connection::open("translate.sqlite").unwrap());
}

pub fn init_database(app_handle: &AppHandle) -> anyhow::Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute_batch(r#"
    BEGIN;
        create table properties
        (
            config_key text primary key,
            config_value any not null,
            description text
        );
        create table engine
        (
            engine_name text primary key ,
            url text not null,
            appid text not null ,
            psd text not null ,
            enable int default 0
        );
    COMMIT;
    "#)?;
    debug!("系统表创建成功，开始支持初始化数据");
    conn.execute_batch(r#"
    BEGIN;
        insert into properties (config_key, config_value) values ('basic.enable','baidu');
        insert into properties (config_key, config_value) values ('basic.shortcut','Control+Alt+KeyQ');
        insert into properties (config_key, config_value) values ('basic.swipe',false);
        insert into engine (engine_name, url, appid, psd, enable) values ('baidu','https://fanyi-api.baidu.com/ait/api/aiTextTranslate','','',1);
        insert into engine (engine_name, url, appid, psd, enable) values ('youdao','https://openapi.youdao.com/api','','',0);
    COMMIT;
    "#)?;
    Ok(())
}