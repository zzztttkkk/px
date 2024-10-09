use core::str;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

use config::Config;
use value::ValueItem;

mod config;
mod exec;
mod value;

fn getoutput(cmd: String) -> String {
    let parts: Vec<String> = cmd
        .split(" ")
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .collect();
    if parts.len() < 2 {
        return cmd;
    }
    let mut program = std::process::Command::new(&parts[1]);
    program.args(&parts[2..]);
    let output = program
        .output()
        .expect(format!("exec `{}` failed", cmd).as_str());
    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        match info.payload().downcast_ref::<String>() {
            Some(msg) => {
                println!("Paniced: {}", msg);
            }
            None => {
                println!("Paniced @ {:?}", info.location());
            }
        }
    }));

    let args: Vec<String> = env::args().collect();

    let bytes = std::fs::read("./px.toml").expect("read `./px.toml` failed");
    let content = str::from_utf8(&bytes).expect("read `px.toml` failed");
    let cfg = toml::from_str::<Config>(content).expect("parse `px.toml` failed");
    if cfg.cmds.is_none() {
        panic!("empty commands");
    }
    let mut cmds: HashMap<String, config::Command> = HashMap::new();
    cfg.cmds.as_ref().map(|vs| {
        for (key, cmd) in vs {
            cmds.insert(key.to_lowercase(), cmd.clone());
        }
    });

    let mut requirename: String = String::new();
    if args.len() == 1 {
        let mut names: Vec<_> = cmds.keys().collect();
        names.sort();
        let cs: Vec<_> = names
            .iter()
            .enumerate()
            .map(|(idx, key)| format!("{}: {}", idx, key))
            .collect();
        println!("please choose one command: {:?}", cs);
        let mut line = String::new();
        _ = std::io::stdin().read_line(&mut line);
        let line = line.trim().to_lowercase();
        if line.is_empty() {
            return;
        }
        match line.parse::<usize>() {
            Ok(idx) => {
                if idx >= names.len() {
                    panic!("bad index");
                }
                requirename = names[idx].clone();
            }
            Err(_) => {
                let mut found = false;
                for name in names.iter() {
                    if name.to_lowercase() == line {
                        requirename = line.clone();
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("bad command: `{}`", &line)
                }
            }
        }
    } else {
        requirename = args[1].to_lowercase();
    }

    let cmd = cmds
        .get_mut(&requirename)
        .expect(format!("command `{}` not found", requirename).as_str());

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

    for (_, v) in &mut values {
        match v {
            ValueItem::String(sv) => {
                if sv.starts_with("$ ") {
                    *v = ValueItem::String(getoutput(sv.clone()));
                }
            }
            _ => {}
        }
    }

    if args.len() > 2 {
        if cmd.args.is_none() {
            cmd.args = Some(vec![]);
        }
        let cargs = cmd.args.as_mut().unwrap();
        for argv in &args[2..] {
            cargs.push(ValueItem::String(argv.clone()))
        }
    }

    let currentproc = Arc::new(Mutex::new((0 as u32, false)));
    let _ccpidc = currentproc.clone();
    ctrlc::set_handler(move || {
        let mg = _ccpidc.lock().unwrap();
        let (pid, keepcp) = *mg;
        if pid == 0 {
            return;
        }
        if keepcp {
        } else {
            _ = kill_tree::blocking::kill_tree(pid);
        }
    })
    .expect("Error setting Ctrl-C handler");

    match cmd.matrix.as_ref() {
        Some(matrix) => {
            if matrix.is_empty() {
                crate::exec::exec(requirename.clone(), cmd, &values, None, currentproc.clone());
                return;
            }

            let mut tmp: Vec<ValueItem> = vec![ValueItem::Bool(false); matrix.len()];
            let mut idxs: Vec<usize> = vec![0; matrix.len()];
            loop {
                for (ridx, cidx) in idxs.iter().enumerate() {
                    tmp[ridx] = (matrix[ridx][*cidx]).clone();
                }

                crate::exec::exec(
                    requirename.clone(),
                    cmd,
                    &values,
                    Some(&tmp),
                    currentproc.clone(),
                );

                let mut idx = matrix.len();
                while idx > 0 {
                    idx -= 1;
                    idxs[idx] += 1;
                    if idxs[idx] < matrix[idx].len() {
                        break;
                    }
                    idxs[idx] = 0;
                    if idx == 0 {
                        return;
                    }
                }
            }
        }
        None => {
            crate::exec::exec(requirename.clone(), cmd, &values, None, currentproc.clone());
        }
    }
}
