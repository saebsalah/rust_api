use super::constants;
use std::env;

pub fn get_addr() -> String {
    match env::var(constants::ADDR) {
        // addr="127.0.0.1:8080"
        Ok(v) => v,
        Err(e) => panic!("${} is not set ({})", constants::ADDR, e),
    }
}
