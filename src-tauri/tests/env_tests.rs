use std::env;

#[test]
pub fn test_env() {
    let result = env::var("XDG_SESSION_TYPE");
    println!("{:?} {:?}", result, result.is_ok());
}