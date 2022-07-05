
use serde::{Deserialize, Serialize,};
#[derive(Debug, Serialize, Deserialize, Clone,Default)]
pub struct Config {
    
    pub port : u16,
}
pub fn new() -> Config {
    Config {
        port: 3000,
    }
}