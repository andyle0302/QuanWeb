use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeMap};
use serde::de::Deserializer;
use serde_value::Value;

// Ref: https://github.com/sirgallifrey/serde_either/blob/main/src/enums.rs
#[derive(Debug, PartialEq)]
pub enum ApiErrorDetail {
    String(String),
    HashMap(std::collections::HashMap<String, String>),
}

impl Default for ApiErrorDetail {
    fn default() -> Self {
        Self::String("".to_string())
    }
}

impl Clone for ApiErrorDetail {
    fn clone(&self) -> Self {
        match self {
            Self::String(as_single) => Self::String(as_single.clone()),
            Self::HashMap(as_hashmap) => Self::HashMap(as_hashmap.clone()),
        }
    }
}

impl Serialize for ApiErrorDetail {
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
    Se: Serializer,
    {
        match self {
            Self::String(as_string) => serializer.serialize_str(as_string),
            Self::HashMap(as_hashmap) => serializer.serialize_map(Some(as_hashmap.len()))?.end(),
        }
    }
}

impl<'de> Deserialize<'de> for ApiErrorDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = Value::deserialize(deserializer)?;
        match  value {
            Value::String(as_string) => Ok(Self::String(as_string)),
            Value::Map(as_map) => {
                let hm: HashMap<String, String> = as_map.into_iter().flat_map(|(k, v)| {
                    let key = String::deserialize(k).ok()?;
                    let value = String::deserialize(v).ok()?;
                    Some((key, value))
                }).collect();
                Ok(Self::HashMap(hm))
            },
            _ => Err(serde::de::Error::custom("expected a string or a map")),
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApiErrorShape {
    pub detail: ApiErrorDetail,
    pub origin: Option<String>,
}

impl From<String> for ApiErrorShape {
    fn from(detail: String) -> Self {
        Self {
            detail: ApiErrorDetail::String(detail),
            ..Default::default()
        }
    }
}

impl From<HashMap<String, String>> for ApiErrorShape {
    fn from(detail: HashMap<String, String>) -> Self {
        Self {
            detail: ApiErrorDetail::HashMap(detail),
            ..Default::default()
        }
    }
}

impl Into<ApiErrorDetail> for String {
    fn into(self) -> ApiErrorDetail {
        ApiErrorDetail::String(self)
    }
}
