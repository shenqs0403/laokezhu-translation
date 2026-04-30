use std::env;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rusqlite::Transaction;
use serde::{Deserialize, Serialize};
use tauri_plugin_log::log::debug;
use crate::dao::engine_dao::create_engine_table_and_init_data;
use crate::dao::key_value_dao::{create_key_value_table_and_init_data, get_item, set_item, KEY_DB_VERSION};

pub mod engine_dao;
pub mod key_value_dao;

static CURRENT_VERSION: i32 = 1;
lazy_static! {
    // pub static ref DB_CONN: Mutex<rusqlite::Connection> = Mutex::new(rusqlite::Connection::open(env::home_dir().unwrap().join(".config").join("laokezhu").join("translate.sqlite")).unwrap());
    pub static ref DB_CONN: Mutex<rusqlite::Connection> = Mutex::new(rusqlite::Connection::open("translate.sqlite").unwrap());
}

/// 初始化
pub fn init() -> anyhow::Result<()> {
    create_engine_table_and_init_data()?;
    create_key_value_table_and_init_data()?;
    Ok(())
}

pub fn upgrade() -> anyhow::Result<()> {
    debug!("upgrade called");
    let value = get_item(KEY_DB_VERSION.to_string()).unwrap_or_else(|e| KeyValue::new(KEY_DB_VERSION, "0"));
    debug!("upgrade called: {:?}", value);
    let mut db_version = value.value.parse::<i32>().unwrap_or(0);
    debug!("db_version: {}", db_version);
    db_version = start_trans(db_version)?;
    set_item(KEY_DB_VERSION.to_string(), db_version.to_string())?;
    debug!("执行升级完成");
    Ok(())
}

fn start_trans(mut db_version: i32) -> anyhow::Result<i32> {
    let mut conn = DB_CONN.lock().unwrap();
    let trans = conn.transaction()?;

    // 查询当前数据库版本
    let upgrade_sql_arr = vec![UPGRADE_SQL_V_0];
    while db_version < CURRENT_VERSION {
        debug!("正在执行升级版本：{}",db_version);
        let sql = upgrade_sql_arr.get(db_version as usize).unwrap();
        debug!("正在执行：{}",sql.clone());
        trans.execute_batch(sql)?;
        db_version += 1;
    }
    trans.commit()?;
    Ok(db_version)
}

#[derive(Serialize,Deserialize,Debug,Default,Clone)]
/// 引擎持久化对象
pub struct Engine {
    /// 引擎代码
    pub engine_name: String,
    // 引擎中文名称
    pub engine_zh_name: String,
    /// 请求url
    pub url: String,
    /// appid 各个引擎所需不一样，如果不需要则是空字符串
    pub appid: String,
    /// 引擎的密钥，各个引擎所需不一样，如果不需要则是空字符串
    pub engine_key: String,
    /// 是否启用，true-启用，false-停用
    pub enable: bool,
    /// 区域，在腾讯翻译的时候使用，其他都是空字符串
    pub region: String,
}

/// key-value存储对象
#[derive(Serialize,Deserialize,Debug,Default,Clone)]
pub struct KeyValue {
    /// key主键，唯一，非空
    pub key: String,
    /// 值
    pub value: String,
}

impl KeyValue {
    pub fn new(key: &str, value: &str) -> Self {
        KeyValue{ key: "".to_string(), value: "".to_string() }
    }
}

static UPGRADE_SQL_V_0: &str = r#"
alter table engine add region text;
update engine set region = '';
insert into engine (engine_name,engine_zh_name, url, appid, engine_key, enable) values ('aliyun', '阿里云', 'https://mt.aliyuncs.com', '', '', false);
insert into engine (engine_name,engine_zh_name, url, appid, engine_key, enable) values ('tencent', '腾讯', 'https://tmt.tencentcloudapi.com', '', '', false);
"#;