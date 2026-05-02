use rusqlite::{params};
use tauri_plugin_log::log::debug;
use crate::dao::{Engine, DB_CONN};
static SQL_UPDATE_BY_ENGINE_NAME: &str = "update engine set enable = ?1,url = ?2,appid = ?3,engine_key = ?4,region=?5 where engine_name = ?6";
static SQL_DISABLE_ALL: &str = "update engine set enable = false";
static SQL_ENABLE_BY_ENGINE_NAME: &str = "update engine set enable = 1 where engine_name = ?1";
static SQL_QUERY_BY_ENGINE_NAME: &str = "select * from engine where engine_name = ?1";
static SQL_QUERY_BY_ENABLE: &str = "select * from engine where enable = 1";
static SQL_QUERY_ALL: &str = "select * from engine order by engine_name";
pub static SQL_INSERT: &str = "insert into engine (engine_name,engine_zh_name, url, appid, engine_key, enable) values (?1, ?2, ?3, ?4, ?5, ?6)";

/// 创建数据库表并初始化数据
pub fn create_engine_table_and_init_data() -> anyhow::Result<()> {
    let create = r#"create table engine
        (
            engine_name text primary key ,
            engine_zh_name text not null,
            url text not null,
            appid text not null,
            engine_key text not null ,
            enable int not null default 0
        )"#;
    let conn = DB_CONN.lock().unwrap();
    conn.execute(create,[])?;
    conn.execute(SQL_INSERT,params!["baidu","百度","https://fanyi-api.baidu.com/api/trans/vip/translate","","",false])?;
    conn.execute(SQL_INSERT,params!["youdao","有道","https://openapi.youdao.com/api","","",false])?;
    Ok(())
}

pub fn save_engine(engine: Engine) -> anyhow::Result<usize> {
    let conn = DB_CONN.lock().unwrap();
    let i = conn.execute(SQL_UPDATE_BY_ENGINE_NAME, params![engine.enable,engine.url,engine.appid,engine.engine_key,engine.region,engine.engine_name])?;
    debug!("修改引擎成功{}",i);
    if engine.enable {
        conn.execute(SQL_DISABLE_ALL, params![])?;
        debug!("禁用所有引擎完成");
        conn.execute(SQL_ENABLE_BY_ENGINE_NAME, params![engine.engine_name])?;
        debug!("启用 {} 引擎完成",engine.engine_name)
    }
    Ok(i)
}

pub fn select_by_engine_name(engine_name: &str) -> anyhow::Result<Engine> {
    let conn = DB_CONN.lock().unwrap();
    conn.query_one(SQL_QUERY_BY_ENGINE_NAME, [engine_name],|row| row_to_engine(row))
        .map_err(anyhow::Error::from)
}

pub fn select_enable_engine() -> anyhow::Result<Engine> {
    let conn = DB_CONN.lock().unwrap();
    conn.query_one(SQL_QUERY_BY_ENABLE,[],|row| row_to_engine(row))
        .map_err(anyhow::Error::from)
}

pub fn select_all_engine() -> anyhow::Result<Vec<Engine>> {
    let conn = DB_CONN.lock().unwrap();
    let mut statement = conn.prepare(SQL_QUERY_ALL)?;
    let rows = statement.query_map([], |row| row_to_engine(row))?;
    let mut engines: Vec<Engine> = Vec::new();
    for row in rows {
        engines.push(row?);
    }
    Ok(engines)
}

fn row_to_engine(row: &rusqlite::Row) -> rusqlite::Result<Engine> {
    Ok(Engine{
        engine_name: row.get(0)?,
        engine_zh_name: row.get(1)?,
        url: row.get(2)?,
        appid: row.get(3)?,
        engine_key: row.get(4)?,
        enable: row.get(5)?,
        region: row.get(6).unwrap_or_else(|_| "".to_string()),
    })
}