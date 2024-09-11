use core::str;
use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
};

use config::{Command, Config};
use value::ValueItem;

mod config;
mod value;

fn trytostring(v: &Vec<u8>) -> Option<String> {
    match String::from_utf8(v.to_vec()) {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() || args.len() < 2 {
        println!("empty args");
        return;
    }

    let bytes = std::fs::read("./px.toml").unwrap();
    let content = str::from_utf8(&bytes).unwrap();
    let cfg = toml::from_str::<Config>(content).unwrap();
    if cfg.cmds.is_none() {
        println!("empty commands");
        return;
    }
    let mut cmds: HashMap<String, config::Command> = HashMap::new();
    cfg.cmds.as_ref().map(|vs| {
        for (key, cmd) in vs {
            let name: String = match &cmd.name {
                Some(tmp) => tmp.to_uppercase(),
                None => key.to_uppercase(),
            };
            cmds.insert(name, cmd.clone());
        }
    });
    if cmds.is_empty() {
        println!("empty commands");
        return;
    }
    let requirename: String = args[1].to_uppercase();
    let cmd = cmds.get(&requirename).expect("");

    let mut values: HashMap<String, ValueItem> = HashMap::new();
    cfg.values.as_ref().map(|vs| {
        for (k, v) in vs {
            values.insert(k.clone(), v.clone());
        }
    });
    cmd.values.as_ref().map(|vs| {
        for (k, v) in vs {
            values.insert(k.clone(), v.clone());
        }
    });

    exec(cmd, &values);
}

fn exec(cmd: &Command, values: &HashMap<String, ValueItem>) {
    println!("cmd: {:?}\r\nvalues: {:?}", cmd, values);
}
