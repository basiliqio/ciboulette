use super::*;
use itertools::Itertools;
#[cfg(feature = "sqlx_postgres")]
use sqlx::{TypeInfo, ValueRef};
use std::fmt::Formatter;
use std::str::FromStr;

lazy_static::lazy_static! {
    static ref BASE64_CONFIG: base64::Config = {
        base64::Config::new(base64::CharacterSet::UrlSafe, true)
        .decode_allow_trailing_bits(true)
    };
}

/// ## Resource id type
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize)]
#[serde(untagged)]
// TODO custom deserialize
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
pub struct CibouletteIdSelector<'request>(CibouletteSelector<CibouletteId<'request>>);

ciboulette_selector_utils!(CibouletteIdSelector, CibouletteId, 'request);

impl<'request> CibouletteIdSelector<'request> {
    pub fn build_id(
        id_selector: &CibouletteIdTypeSelector,
        id_str: Cow<'request, str>,
    ) -> Result<CibouletteIdSelector<'request>, CibouletteError> {
        let res = match &**id_selector {
            CibouletteSelector::Single(x) => {
                CibouletteIdSelector(CibouletteSelector::Single(match x {
                    CibouletteIdType::Text(_) => CibouletteId::Text(Cow::Owned(String::from_utf8(
                        base64::decode_config(id_str.as_ref(), *BASE64_CONFIG)?,
                    )?)),
                    CibouletteIdType::Number(_) => {
                        CibouletteId::Number(u64::from_str(id_str.as_ref())?)
                    }
                    CibouletteIdType::Uuid(_) => {
                        CibouletteId::Uuid(Uuid::from_str(id_str.as_ref())?)
                    }
                }))
            }
            CibouletteSelector::Multi(x) => {
                let mut res = Vec::with_capacity(2);

                for (i, id) in id_str.split(',').enumerate() {
                    let id_type = x
                        .get(i)
                        .ok_or_else(|| CibouletteError::WrongIdNumber(i, x.len()))?;
                    res.push(match id_type {
                        CibouletteIdType::Text(_) => CibouletteId::Text(Cow::Owned(
                            String::from_utf8(base64::decode_config(id, *BASE64_CONFIG)?)?,
                        )),
                        CibouletteIdType::Number(_) => CibouletteId::Number(u64::from_str(id)?),
                        CibouletteIdType::Uuid(_) => CibouletteId::Uuid(Uuid::from_str(id)?),
                    });
                }
                CibouletteIdSelector(CibouletteSelector::Multi(res))
            }
        };
        Ok(res)
    }
}

impl<'request> std::fmt::Display for CibouletteIdSelector<'request> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &**self {
            CibouletteSelector::Single(x) => write!(f, "{}", x),
            CibouletteSelector::Multi(x) => write!(f, "{}", x.iter().join(",")),
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

impl CibouletteIdType {
    pub fn name(&self) -> &ArcStr {
        match self {
            CibouletteIdType::Text(x) => x,
            CibouletteIdType::Number(x) => x,
            CibouletteIdType::Uuid(x) => x,
        }
    }
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
pub struct CibouletteIdTypeSelector(CibouletteSelector<CibouletteIdType>);

ciboulette_selector_utils!(CibouletteIdTypeSelector, CibouletteIdType);
