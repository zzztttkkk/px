use core::str;
use std::{collections::HashMap, env};

use config::Config;
use value::ValueItem;

mod config;
mod exec;
mod value;

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
    let cmd = cmds
        .get_mut(&requirename)
        .expect(format!("command `{}` not found", &args[1]).as_str());

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

    if args.len() > 2 {
        if cmd.args.is_none() {
            cmd.args = Some(vec![]);
        }
        let cargs = cmd.args.as_mut().unwrap();
        for argv in &args[2..] {
            cargs.push(ValueItem::String(argv.clone()))
        }
    }

    match cmd.matrix.as_ref() {
        Some(matrix) => {
            if matrix.is_empty() {
                crate::exec::exec(requirename.clone(), cmd, &values, None);
                return;
            }

            let mut tmp: Vec<ValueItem> = vec![ValueItem::Bool(false); matrix.len()];
            let mut idxs: Vec<usize> = vec![0; matrix.len()];
            loop {
                for (ridx, cidx) in idxs.iter().enumerate() {
                    tmp[ridx] = (matrix[ridx][*cidx]).clone();
                }

                crate::exec::exec(requirename.clone(), cmd, &values, Some(&tmp));

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
            crate::exec::exec(requirename.clone(), cmd, &values, None);
        }
    }
}
