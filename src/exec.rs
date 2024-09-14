use std::collections::HashMap;

use crate::{
    config::Command,
    value::{ValueItem, ValueReplacer, VALUE_REGEXP},
};

pub fn exec(
    program: String,
    cmdcfg: &Command,
    values: &HashMap<String, ValueItem>,
    matrix: Option<&Vec<ValueItem>>,
) {
    let program = match cmdcfg.cmd.as_ref() {
        Some(t) => {
            if t.is_empty() {
                program
            } else {
                t.clone()
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
                    let txt = (*VALUE_REGEXP)
                        .replace_all(&txt, &mut replacer)
                        .into_owned();
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

    cmd.spawn()
        .expect(format!("failed to spawn process: `{}`", &program).as_str())
        .wait()
        .expect(format!("wait process failed").as_str());
}
