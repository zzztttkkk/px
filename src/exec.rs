use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    config::Command,
    value::{ValueItem, ValueReplacer, VALUE_REGEXP},
};

pub fn exec(
    program: String,
    cmdcfg: &Command,
    values: &HashMap<String, ValueItem>,
    matrix: Option<&Vec<ValueItem>>,
    procarc: Arc<Mutex<(u32, bool)>>,
) {
    let program = match cmdcfg.program.as_ref() {
        Some(tmp) => {
            if tmp.is_empty() {
                program
            } else {
                tmp.clone()
            }
        }
        None => program,
    };
    let mut cmd = std::process::Command::new(&program);
    cmdcfg.dir.as_ref().map(|v| {
        if !v.is_empty() {
            cmd.current_dir(v.clone());
        }
    });

    cmdcfg.args.as_ref().map(|vs| {
        for argv in vs.iter() {
            match argv {
                ValueItem::String(txt) => {
                    let mut replacer = ValueReplacer {
                        values,
                        matrix,
                        errors: None,
                    };
                    let txt = VALUE_REGEXP.replace_all(&txt, &mut replacer).into_owned();
                    if replacer.errors.is_some() {
                        panic!("{}", replacer.errors.unwrap());
                    }
                    cmd.arg(txt);
                }
                _ => {
                    cmd.arg(argv.to_string());
                }
            }
        }
    });

    let mut child = cmd
        .spawn()
        .expect(format!("failed to spawn process: `{}`", &program).as_str());

    let keepcp = cmdcfg.keepchildprocess.map_or(false, |v| v);

    let mut mg = procarc.lock().unwrap();
    *mg = (child.id(), keepcp);
    std::mem::drop(mg);

    let es = child.wait().expect(format!("wait process failed").as_str());

    if keepcp {
    } else {
        _ = kill_tree::blocking::kill_tree(child.id());
    }

    match es.code() {
        Some(code) => {
            if code != 0 {
                std::process::exit(code);
            }
        }
        None => {}
    }
}
