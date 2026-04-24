use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::params;
use crate::dao::{Engine, DB_CONN};
static SQL_UPDATE_BY_ENGINE_NAME: &str = "update engine set enable = ?1,url = ?2,appid = ?3,engine_key = ?4 where engine_name = ?5";
static SQL_DISABLE_ALL: &str = "update engine set enable = false";
static SQL_ENABLE_BY_ENGINE_NAME: &str = "update engine set enable = ?1 where engine_name = ?2";
static SQL_QUERY_BY_ENGINE_NAME: &str = "select * from engine where engine_name = ?";
static SQL_QUERY_BY_ENABLE: &str = "select * from engine where enable = ?1";
static SQL_QUERY_ALL: &str = "select * from engine ";

/// 创建数据库表并初始化数据
pub fn create_engine_table_and_init_data() -> anyhow::Result<()> {
    let insert = "insert into engine (engine_name,engine_zh_name, url, appid, engine_key, enable) values (?1, ?2, ?3, ?4, ?5, ?6)";
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
    conn.execute(insert,params!["baidu","百度","","","",false])?;
    conn.execute(insert,params!["youdao","有道","","","",false])?;
    Ok(())
}

pub fn save_engine(engine: Engine) -> anyhow::Result<usize> {
    let conn = DB_CONN.lock().unwrap();
    let i = conn.execute(SQL_UPDATE_BY_ENGINE_NAME, params![engine.enable,engine.url,engine.appid,engine.engine_key,engine.engine_name])?;
    if engine.enable {
        conn.execute(SQL_DISABLE_ALL, params![])?;
        conn.execute(SQL_QUERY_BY_ENGINE_NAME, params![engine.engine_name])?;
    }
    Ok(i)
}

pub fn select_enable_engine() -> anyhow::Result<Engine> {
    let conn = DB_CONN.lock().unwrap();
    conn.query_one(SQL_QUERY_BY_ENABLE,[],|row| row_to_engine(row))
        .map_err(|r| anyhow::anyhow!("{:?}", r))
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
        url: row.get(1)?,
        appid: row.get(2)?,
        engine_key: row.get(3)?,
        enable: row.get(4)?,
    })
}