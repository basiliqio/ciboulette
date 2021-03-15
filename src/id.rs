use super::*;
use serde::de::{DeserializeSeed, Deserializer, Visitor};
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CibouletteIdBuilder<'a> {
    Number(u64),
    Text(Cow<'a, str>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize)]
pub enum CibouletteId<'a> {
    Number(u64),
    Uuid(Uuid),
    Text(Cow<'a, str>),
}

impl<'a> CibouletteIdBuilder<'a> {
    pub fn build(self, type_: &CibouletteIdType) -> Result<CibouletteId<'a>, CibouletteError> {
        match (self, type_) {
            (CibouletteIdBuilder::Text(x), CibouletteIdType::Text) => {
                Ok(CibouletteId::Text(x.clone()))
            }
            (CibouletteIdBuilder::Text(x), CibouletteIdType::Number) => {
                Ok(CibouletteId::Number(u64::from_str(x.as_ref())?))
            }
            (CibouletteIdBuilder::Text(x), CibouletteIdType::Uuid) => {
                Ok(CibouletteId::Uuid(Uuid::from_str(x.as_ref())?))
            }

            (CibouletteIdBuilder::Number(x), CibouletteIdType::Text) => {
                Ok(CibouletteId::Text(Cow::Owned(x.to_string())))
            }
            (CibouletteIdBuilder::Number(x), CibouletteIdType::Number) => {
                Ok(CibouletteId::Number(x))
            }
            (CibouletteIdBuilder::Number(_), CibouletteIdType::Uuid) => Err(
                CibouletteError::BadIdType(CibouletteIdType::Number, CibouletteIdType::Uuid),
            ),
        }
    }
}

impl<'a> CibouletteId<'a> {
    pub fn parse(id_type: CibouletteIdType, val: Cow<'a, str>) -> Result<Self, CibouletteError> {
        Ok(match id_type {
            CibouletteIdType::Number => CibouletteId::Number(u64::from_str(val.as_ref())?),
            CibouletteIdType::Text => CibouletteId::Text(val),
            CibouletteIdType::Uuid => CibouletteId::Uuid(Uuid::parse_str(val.as_ref())?),
        })
    }
}

impl<'a> ToString for CibouletteId<'a> {
    fn to_string(&self) -> String {
        match self {
            CibouletteId::Number(x) => x.to_string(),
            CibouletteId::Text(x) => x.to_string(),
            CibouletteId::Uuid(x) => x.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CibouletteIdType {
    Number,
    Text,
    Uuid,
}

impl<'a> std::fmt::Display for CibouletteIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CibouletteIdType::Number => write!(f, "number"),
            CibouletteIdType::Text => write!(f, "text"),
            CibouletteIdType::Uuid => write!(f, "uuid"),
        }
    }
}

struct CibouletteIdVisitor(CibouletteIdType);

impl<'de> Visitor<'de> for CibouletteIdVisitor {
    type Value = CibouletteId<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "an id identifier")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match self.0 {
            CibouletteIdType::Text => Ok(CibouletteId::Text(Cow::Borrowed(value))),
            CibouletteIdType::Uuid => {
                Ok(CibouletteId::Uuid(Uuid::parse_str(value).map_err(|e| {
                    serde::de::Error::custom(format!("Failed to deserialize UUID: {}", e))
                })?))
            }
            CibouletteIdType::Number => {
                Ok(CibouletteId::Number(u64::from_str(value).map_err(|e| {
                    serde::de::Error::custom(format!("Failed to deserialize unsigned long: {}", e))
                })?))
            }
        }
    }

    #[inline]
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match self.0 {
            CibouletteIdType::Number => Ok(CibouletteId::Number(value)),
            CibouletteIdType::Text => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Unsigned(value),
                &"a text unique identifier",
            )),
            CibouletteIdType::Uuid => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Unsigned(value),
                &"an UUID",
            )),
        }
    }
}
impl<'de> DeserializeSeed<'de> for CibouletteIdVisitor {
    type Value = CibouletteId<'de>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.0 {
            CibouletteIdType::Number => deserializer.deserialize_u64(self),
            CibouletteIdType::Text => deserializer.deserialize_str(self),
            CibouletteIdType::Uuid => deserializer.deserialize_str(self),
        }
    }
}