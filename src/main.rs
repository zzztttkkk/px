use core::str;

use config::Config;

mod config;
mod value;

fn main() {
    let bytes = std::fs::read("./px.toml").unwrap();
    let content = str::from_utf8(&bytes).unwrap();
    let cfg = toml::from_str::<Config>(content).unwrap();
    println!("{:?}", cfg);
}
