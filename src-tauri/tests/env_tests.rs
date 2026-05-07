use std::env;
use std::sync::Mutex;
use sha2::Digest;
use tauri::test::{mock_builder, mock_context};

#[test]
pub fn test_string () {
    println!("{}","aa".eq("aa"));
    let mutex = Mutex::new("1".to_string());
    // println!("{:?}", mutex.lock().unwrap().cmp(&"0".to_string()));
    let guard = mutex.lock().unwrap();
    println!("{}", guard.eq(&"2".to_string()));
}


#[test]
pub fn test_baidu() {
    let s = "aaa";
    println!("{} {}", s.len(), s.chars().count());
    let s = "你好";
    println!("{} {}", s.len(), s.chars().count());
}

#[test]
pub fn test_md5 () {
    let digest = md5::compute("aa");
    println!("{:?} {}", digest, hex::encode(digest.0));
}

#[test]
pub fn test_sha2 () {
    let array = sha2::Sha256::digest("aa");
    let string = hex::encode(array);
    println!("{:?} {:?}", string, array);
}