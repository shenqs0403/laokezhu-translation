use std::env;

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