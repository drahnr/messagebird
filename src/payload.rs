use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "encoding")]
pub enum PayloadEncoding {
    Plain,
    Unicode,
    Auto,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PayloadType {
    Sms,
    Binary,
    Flash,
}



#[derive(Debug)]
pub enum Payload {
    Bytes(Vec<u8>),
    Text(String),
}

impl Default for Payload {
    fn default() -> Self {
        Payload::Text("default".to_string())
    }
}

// You can even choose to implement multiple traits, like Lower and UpperHex
impl fmt::LowerHex for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Payload::Bytes(ref bytes) => {
                for byte in bytes {
                    write!(f, "{:x} ", byte)?;
                }
            },
            Payload::Text(ref s) => {
                for byte in s.as_bytes() {
                write!(f, "{:x} ", byte)?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Payload {
    type Err = MessageBirdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Payload::Text(String::from(s)))
    }
}

impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Payload::Bytes(_) => {
                let data = format!("{:x}", self);
                serializer.serialize_str(data.as_str())
            }
            Payload::Text(ref s) => {
                serializer.serialize_str(s.as_str())
            }
        }
    }
}

struct PayloadVisitor;

impl<'de> Visitor<'de> for PayloadVisitor {
    type Value = Payload;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid date time formatted str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        unimplemented!("not clear yet how to do this without knowing the the payload_type in advance")
        // Payload::from_str(value)
        //     .map_err(|e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PayloadVisitor)
    }
}