use std::collections::HashMap;

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

#[derive(Debug, Serialize, Clone)]
pub enum ValueItem {
    Int(i64),
    Uint(u64),
    Double(f64),
    Bool(bool),
    String(String),
}

impl ValueItem {
    pub fn to_string(&self) -> String {
        match self {
            ValueItem::Int(v) => format!("{}", *v).to_string(),
            ValueItem::Uint(v) => format!("{}", *v).to_string(),
            ValueItem::Double(v) => format!("{}", *v).to_string(),
            ValueItem::Bool(v) => format!("{}", *v).to_string(),
            ValueItem::String(v) => v.clone(),
        }
    }
}

struct ValueItemVisitor;

impl<'de> Visitor<'de> for ValueItemVisitor {
    type Value = ValueItem;

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Bool(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Int(v as i64))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Int(v as i64))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Int(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Uint(v as u64))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Uint(v as u64))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Uint(v as u64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Uint(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::String(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::String(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Double(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueItem::Double(v))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(format!("expect `string`, `number` or `bool`").as_str())
    }
}

impl<'de> Deserialize<'de> for ValueItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueItemVisitor {})
    }
}

pub static VALUE_REGEXP: std::sync::LazyLock<&'static regex::Regex> =
    std::sync::LazyLock::new(|| {
        return Box::leak(Box::new(
            regex::Regex::new(r"\$\{\s*[\w#]+(\.\d)?\s*\}").unwrap(),
        ));
    });

pub struct ValueReplacer<'a> {
    pub values: &'a HashMap<String, ValueItem>,
    pub matrix: Option<&'a Vec<ValueItem>>,
    pub errors: Option<String>,
}

fn matrix_idx(txt: &str) -> Option<usize> {
    match txt.find('.') {
        Some(idx) => {
            let name = &(txt[..idx]);
            if name != "#" && name != "matrix" {
                return None;
            }
            let seq = &(txt[(idx + 1)..]);
            match seq.parse::<usize>() {
                Ok(num) => {
                    return Some(num);
                }
                Err(_) => {
                    return None;
                }
            }
        }
        None => None,
    }
}

impl<'a> regex::Replacer for &mut ValueReplacer<'a> {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        if self.errors.is_some() {
            return;
        }
        let name = &caps[0];
        let name = &(name[2..name.len() - 1]).trim();

        match matrix_idx(name) {
            Some(idx) => match self.matrix.as_ref() {
                Some(ms) => {
                    if idx >= ms.len() {
                        self.errors = Some(format!(
                            "out of matrix range, can not get value for `{}`",
                            name
                        ));
                        return;
                    }
                    dst.push_str(ms[idx].to_string().as_str());
                    return;
                }
                None => {
                    self.errors = Some(format!("empty matrix, can not get value for `{}`", name));
                    return;
                }
            },
            None => match self.values.get(name.trim()) {
                Some(ele) => {
                    dst.push_str(ele.to_string().as_str());
                    return;
                }
                None => {
                    self.errors = Some(format!("undefined, can not get value for `{}`", name));
                    return;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::value::VALUE_REGEXP;

    #[test]
    fn test_regexp() {
        println!("{}", VALUE_REGEXP.is_match("${121.1}"));

        let matchs: Vec<_> = VALUE_REGEXP.find_iter("${a}-${b}-${c.1}").collect();
        for me in matchs {
            println!(">>> {}", me.as_str())
        }
    }
}
