use super::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CibouletteId<'a> {
    Number(u64),
    Text(Cow<'a, str>),
    Uuid(Uuid),
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CibouletteIdType {
    Number,
    Text,
    Uuid,
}
