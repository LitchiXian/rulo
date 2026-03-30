/// i64 ↔ JSON string，防止雪花 ID 在前端 Number 丢失精度（53 位限制）
///
/// 用法：
///   序列化+反序列化均需要的字段（如实体 struct）: `#[serde(with = "crate::util::serde_util::i64_str")]`
///   仅序列化（响应 struct）:                       `#[serde(serialize_with = "crate::util::serde_util::i64_str::serialize")]`
///   仅反序列化（请求 DTO）:                        `#[serde(deserialize_with = "crate::util::serde_util::i64_str::deserialize")]`
///   Option<i64> 字段对应使用 opt_i64_str
///   Vec<i64>    字段对应使用 vec_i64_str

pub mod i64_str {
    use serde::{Deserializer, Serializer, de};

    pub fn serialize<S: Serializer>(val: &i64, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&val.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = i64;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("i64 or string representation of i64")
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<i64, E> { Ok(v) }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<i64, E> {
                i64::try_from(v).map_err(de::Error::custom)
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<i64, E> {
                v.parse().map_err(de::Error::custom)
            }
        }
        d.deserialize_any(Visitor)
    }
}

/// Option<i64> ↔ Option<JSON string>
pub mod opt_i64_str {
    use serde::{Deserializer, Serializer, de};

    pub fn serialize<S: Serializer>(val: &Option<i64>, s: S) -> Result<S::Ok, S::Error> {
        match val {
            Some(v) => s.serialize_some(&v.to_string()),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i64>, D::Error> {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Option<i64>;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("null, i64, or string representation of i64")
            }
            fn visit_none<E: de::Error>(self) -> Result<Option<i64>, E> { Ok(None) }
            fn visit_unit<E: de::Error>(self) -> Result<Option<i64>, E> { Ok(None) }
            fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Option<i64>, D::Error> {
                super::i64_str::deserialize(d).map(Some)
            }
        }
        d.deserialize_option(Visitor)
    }
}

/// Vec<i64> ↔ JSON array of string
pub mod vec_i64_str {
    use serde::{Deserializer, Serializer, de};
    use serde::ser::SerializeSeq;

    pub fn serialize<S: Serializer>(val: &Vec<i64>, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(val.len()))?;
        for v in val {
            seq.serialize_element(&v.to_string())?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<i64>, D::Error> {
        let raw: Vec<serde_json::Value> = de::Deserialize::deserialize(d)?;
        raw.into_iter()
            .map(|v| match v {
                serde_json::Value::Number(n) => {
                    n.as_i64().ok_or_else(|| de::Error::custom("number out of i64 range"))
                }
                serde_json::Value::String(s) => s.parse().map_err(de::Error::custom),
                _ => Err(de::Error::custom("expected number or string")),
            })
            .collect()
    }
}
