use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::dao::engine_dao::create_engine_table_and_init_data;
use crate::dao::key_value_dao::create_key_value_table_and_init_data;

pub mod engine_dao;
pub mod key_value_dao;

lazy_static! {
    pub static ref DB_CONN: Mutex<rusqlite::Connection> = Mutex::new(rusqlite::Connection::open("laokezhu.sqlite").unwrap());
}

pub fn init() -> anyhow::Result<()> {
    create_engine_table_and_init_data()?;
    create_key_value_table_and_init_data()?;
    Ok(())
}

/// 引擎持久化对象
pub struct Engine {
    /// 引擎代码
    pub engine_name: String,
    /// 请求url
    pub url: String,
    /// appid 各个引擎所需不一样，如果不需要则是空字符串
    pub appid: String,
    /// 引擎的密钥，各个引擎所需不一样，如果不需要则是空字符串
    pub engine_key: String,
    /// 是否启用，true-启用，false-停用
    pub enable: bool,
}

/// key-value存储对象
pub struct KeyValue {
    /// key主键，唯一，非空
    pub key: String,
    /// 值
    pub value: String,
}