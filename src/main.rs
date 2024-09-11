use core::str;

use config::Config;

mod config;
mod value;

fn main() {
    match std::fs::metadata("./px.toml") {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(_) => {
            let bytes = std::fs::read("./px.toml").unwrap();
            let content = str::from_utf8(&bytes).unwrap();
            let cfg: Config = toml::from_str(content).unwrap();
            println!("{:?}", cfg);
        }
    }
}
