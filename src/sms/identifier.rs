use super::*;

use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

use std::fmt;
use std::string::ToString;

/// Unique message identifier
///
/// Consists 32 alphanumeric characters (may change!)
///
/// Generate by the MessageBird backend on posting the message.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Identifier(String);

impl Default for Identifier {
    fn default() -> Self {
        Identifier("00000000000000000000000000000000".to_string())
    }
}

impl Identifier {
    pub fn new(raw: String) -> Self {
        Identifier(raw)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Identifier {
    type Err = MessageBirdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // XXX: taken from the example on the webpage
        const VALID_LENGTH: usize = 32;
        if s.len() != VALID_LENGTH {
            Err(MessageBirdError::TypeError {
                msg: format!(
                    "unexpected id length {}, expected {}",
                    s.len(),
                    VALID_LENGTH
                ),
            })
        } else {
            Ok(Self::new(String::from(s)))
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

struct IdentifierVisitor;

impl<'de> Visitor<'de> for IdentifierVisitor {
    type Value = Identifier;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid identifier str with 32 characters")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Identifier::from_str(value)
            .map_err(|_e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IdentifierVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static RAW: &str = r#"
"01238dsfusd98ufe89hsdkncksadfkkr"
"#;

    deser_roundtrip!(identifier_deser, Identifier, RAW);
    serde_roundtrip!(identifier_serde, Identifier, Identifier::default());
}
