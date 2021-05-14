use super::*;
use itertools::Itertools;
#[cfg(feature = "sqlx_postgres")]
use sqlx::{TypeInfo, ValueRef};
use std::str::FromStr;
use std::{fmt::Formatter, usize};

/// ## Resource id type
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize)]
#[serde(untagged)]
pub enum CibouletteId<'request> {
    /// Serial or number id
    Number(u64),
    /// Uuid id
    Uuid(Uuid),
    /// Text id
    Text(Cow<'request, str>),
}

/// ## Resource id type selector
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CibouletteIdSelector<'request> {
    Single(CibouletteId<'request>),
    Multi(Vec<CibouletteId<'request>>),
}

impl<'request> CibouletteIdSelector<'request> {
    pub fn len(&self) -> usize {
        match self {
            CibouletteIdSelector::Single(_) => 1,
            CibouletteIdSelector::Multi(x) => x.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            CibouletteIdSelector::Single(_) => false,
            CibouletteIdSelector::Multi(x) => x.is_empty(),
        }
    }

    pub fn get(&self, i: usize) -> Result<&CibouletteId, CibouletteError> {
        match self {
            CibouletteIdSelector::Single(x) if i == 0 => Ok(x),
            CibouletteIdSelector::Multi(x) => Ok(x
                .get(i)
                .ok_or_else(|| CibouletteError::WrongIdNumber(i, x.len()))?),
            _ => Err(CibouletteError::WrongIdNumber(i, 1)),
        }
    }

    pub fn build_id(
        id_selector: &CibouletteIdTypeSelector,
        id_str: Cow<'request, str>,
    ) -> Result<CibouletteIdSelector<'request>, CibouletteError> {
        let res = match id_selector {
            CibouletteIdTypeSelector::Single(x) => {
                CibouletteIdSelector::Single(match x {
                    CibouletteIdType::Text(_) => CibouletteId::Text(Cow::Owned(id_str.to_string())), // TODO Better
                    CibouletteIdType::Number(_) => {
                        CibouletteId::Number(u64::from_str(id_str.as_ref())?)
                    }
                    CibouletteIdType::Uuid(_) => {
                        CibouletteId::Uuid(Uuid::from_str(id_str.as_ref())?)
                    }
                })
            }
            CibouletteIdTypeSelector::Multi(x) => {
                let mut res = Vec::with_capacity(2);

                for (i, id) in id_str.split(',').enumerate() {
                    let id_type = x
                        .get(i)
                        .ok_or_else(|| CibouletteError::WrongIdNumber(i, x.len()))?;
                    res.push(match id_type {
                        CibouletteIdType::Text(_) => CibouletteId::Text(Cow::Owned(id.to_string())), // TODO Better
                        CibouletteIdType::Number(_) => CibouletteId::Number(u64::from_str(id)?),
                        CibouletteIdType::Uuid(_) => CibouletteId::Uuid(Uuid::from_str(id)?),
                    });
                }
                CibouletteIdSelector::Multi(res)
            }
        };
        Ok(res)
    }
}

impl<'request> std::fmt::Display for CibouletteIdSelector<'request> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CibouletteIdSelector::Single(x) => write!(f, "{}", x),
            CibouletteIdSelector::Multi(x) => write!(f, "{}", x.iter().join(",")),
        }
    }
}

impl<'request> std::fmt::Display for CibouletteId<'request> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CibouletteId::Number(x) => write!(f, "{}", x),
            CibouletteId::Uuid(x) => write!(f, "{}", x),
            CibouletteId::Text(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(feature = "sqlx_postgres")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CibouletteId<'r>
where
    &'r str: sqlx::Decode<'r, sqlx::Postgres>,
    f64: sqlx::Decode<'r, sqlx::Postgres>,
{
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<CibouletteId<'r>, Box<dyn std::error::Error + 'static + Send + Sync>> {
        match value.type_info().name() {
            "TEXT" => Ok(CibouletteId::Text(Cow::Borrowed(
                <&'r str as sqlx::Decode<sqlx::Postgres>>::decode(value)?,
            ))),
            "SERIAL" => Ok(CibouletteId::Uuid(Uuid::parse_str(
                <&'r str as sqlx::Decode<sqlx::Postgres>>::decode(value)?,
            )?)),
            "UUID" => Ok(CibouletteId::Number(
                <f64 as sqlx::Decode<sqlx::Postgres>>::decode(value)? as u64,
            )),
            _ => Err(Box::new(CibouletteError::UnknownIdType(
                value.type_info().name().to_string(),
            ))),
        }
    }
}

#[cfg(feature = "sqlx_postgres")]
impl<'r> sqlx::Type<sqlx::Postgres> for CibouletteId<'r> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match ty.name() {
            "UUID" | "TEXT" | "SERIAL" => true,
            _ => false,
        }
    }
}

/// ## Type of resource id
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CibouletteIdType {
    Number(ArcStr),
    Text(ArcStr),
    Uuid(ArcStr),
}

impl<'request> std::fmt::Display for CibouletteIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CibouletteIdType::Number(x) => write!(f, "{} (number)", x),
            CibouletteIdType::Text(x) => write!(f, "{} (text)", x),
            CibouletteIdType::Uuid(x) => write!(f, "{} (uuid)", x),
        }
    }
}

/// ## Type of resource id
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CibouletteIdTypeSelector {
    Single(CibouletteIdType),
    Multi(Vec<CibouletteIdType>),
}

impl CibouletteIdTypeSelector {
    pub fn get(&self, i: usize) -> Result<&CibouletteIdType, CibouletteError> {
        match self {
            CibouletteIdTypeSelector::Single(x) if i == 0 => Ok(x),
            CibouletteIdTypeSelector::Multi(x) => Ok(x
                .get(i)
                .ok_or_else(|| CibouletteError::WrongIdNumber(i, x.len()))?),
            _ => Err(CibouletteError::WrongIdNumber(i, 1)),
        }
    }
}
