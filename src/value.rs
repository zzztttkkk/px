use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

#[derive(Debug, Serialize)]
pub enum ValueItem {
    Int(i64),
    Uint(u64),
    Double(f64),
    Bool(bool),
    String(String),
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
