use std::env;
use std::sync::Mutex;
use sha2::Digest;

#[test]
pub fn test_string () {
    println!("{}","aa".eq("aa"));
    let mutex = Mutex::new("1".to_string());
    // println!("{:?}", mutex.lock().unwrap().cmp(&"0".to_string()));
    let guard = mutex.lock().unwrap();
    println!("{}", guard.eq(&"2".to_string()));
}

#[test]
pub fn test_env() {
    let result = env::var("XDG_SESSION_TYPE");
    println!("{:?} {:?}", result, result.is_ok());
}

#[test]
pub fn test_baidu() {
    let string = "zh_CN".to_string();
    let x = string.split("_").next().unwrap();
    println!("{:?}", x);
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