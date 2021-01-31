use super::*;
use serde::de::{DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CibouletteResourceSchemaNumberType {
    U64,
    U128,
}

impl Default for CibouletteResourceSchemaNumberType {
    fn default() -> Self {
        CibouletteResourceSchemaNumberType::U64
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct CibouletteResourceSchemaArray<'a> {
    optional: bool,
    items: &'a CibouletteResourceSchema<'a>,
}

impl<'a> CibouletteResourceSchemaArray<'a> {
    pub fn new(items: &'a CibouletteResourceSchema<'a>, optional: bool) -> Self {
        CibouletteResourceSchemaArray { items, optional }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct CibouletteResourceSchemaObject<'a> {
    optional: bool,
    properties: HashMap<String, &'a CibouletteResourceSchema<'a>>,
}

impl<'a> CibouletteResourceSchemaObject<'a> {
    pub fn new(
        properties: HashMap<String, &'a CibouletteResourceSchema<'a>>,
        optional: bool,
    ) -> Self {
        CibouletteResourceSchemaObject {
            properties,
            optional,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct CibouletteResourceSchemaNumeric {
    optional: bool,
    type_: CibouletteResourceSchemaNumberType,
}

impl CibouletteResourceSchemaNumeric {
    pub fn new(type_: CibouletteResourceSchemaNumberType, optional: bool) -> Self {
        CibouletteResourceSchemaNumeric { type_, optional }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct CibouletteResourceSchemaScalar {
    optional: bool,
}

impl CibouletteResourceSchemaScalar {
    pub fn new(optional: bool) -> Self {
        CibouletteResourceSchemaScalar { optional }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CibouletteResourceSchema<'a> {
    Array(CibouletteResourceSchemaArray<'a>),
    Bool(CibouletteResourceSchemaScalar),
    Number(CibouletteResourceSchemaNumeric),
    Obj(CibouletteResourceSchemaObject<'a>),
    String(CibouletteResourceSchemaScalar),
    Null,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CibouletteResourceSchemaValue<'a> {
    Array(Vec<CibouletteResourceSchemaValue<'a>>),
    Bool(bool),
    Number(u128),
    Obj(HashMap<Cow<'a, str>, CibouletteResourceSchemaValue<'a>>),
    String(Cow<'a, str>),
    Null,
}

impl<'de> Visitor<'de> for &'de CibouletteResourceSchema<'de> {
    type Value = CibouletteResourceSchemaValue<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "anything")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        match self {
            CibouletteResourceSchema::Array(arr_type) => {
                let mut res: Vec<Self::Value> = Vec::with_capacity(seq.size_hint().unwrap_or(10));
                while let Some(elem) = seq.next_element_seed(*arr_type.items())? {
                    res.push(elem)
                }
                Ok(CibouletteResourceSchemaValue::Array(res))
            }
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Seq,
                &"other",
            )),
        }
    }
    fn visit_map<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        match self {
            CibouletteResourceSchema::Obj(obj_type) => {
                let mut res: HashMap<Cow<'de, str>, Self::Value> =
                    HashMap::with_capacity(seq.size_hint().unwrap_or(10));
                while let Some(key_seed) = seq.next_key_seed(&CibouletteResourceSchema::String(
                    CibouletteResourceSchemaScalar { optional: false },
                ))? {
                    let (val_schema, key_str) = match key_seed {
                        CibouletteResourceSchemaValue::String(val) => (
                            obj_type.properties().get(&*val).ok_or_else(|| {
                                serde::de::Error::unknown_field(
									&*val,
									&[] // TODO
									// &obj_type
									// 	.keys()
									// 	.filter_map(|s| match res.contains_key(s.as_str()) {
									// 		false => Some(s.as_str()),
									// 		true => None,
									// 	})
									// 	.collect::<Vec<&str>>(),
								)
                            })?,
                            val,
                        ),
                        _ => {
                            return Err(serde::de::Error::invalid_type(
                                // TODO better
                                serde::de::Unexpected::Map,
                                &"other",
                            ));
                        }
                    };
                    let val = seq.next_value_seed(*val_schema)?;
                    res.insert(key_str, val);
                }
                Ok(CibouletteResourceSchemaValue::Obj(res))
            }
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Map,
                &"other",
            )),
        }
    }
    fn visit_bool<A>(self, v: bool) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self {
            CibouletteResourceSchema::Bool(_) => Ok(CibouletteResourceSchemaValue::Bool(v)),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Bool(v),
                &"other",
            )),
        }
    }
    fn visit_borrowed_str<A>(self, v: &'de str) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self {
            CibouletteResourceSchema::String(_) => {
                Ok(CibouletteResourceSchemaValue::String(Cow::from(v)))
            }
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Str(v),
                &"other",
            )),
        }
    }

    fn visit_u64<A>(self, v: u64) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self {
            CibouletteResourceSchema::Number(_) => {
                Ok(CibouletteResourceSchemaValue::Number(v as u128))
            }
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other("number"),
                &"other",
            )),
        }
    }

    fn visit_u128<A>(self, v: u128) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self {
            CibouletteResourceSchema::Number(_) => Ok(CibouletteResourceSchemaValue::Number(v)),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other("number"),
                &"other",
            )),
        }
    }

    fn visit_none<A>(self) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self {
            CibouletteResourceSchema::Null => Ok(CibouletteResourceSchemaValue::Null),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other("null"),
                &"other",
            )),
        }
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            CibouletteResourceSchema::Bool(_) => deserializer.deserialize_bool(self),
            CibouletteResourceSchema::String(_) => deserializer.deserialize_str(self),
            CibouletteResourceSchema::Number(opt) => match opt.type_ {
                CibouletteResourceSchemaNumberType::U64 => deserializer.deserialize_u64(self),
                CibouletteResourceSchemaNumberType::U128 => deserializer.deserialize_u128(self),
            },
            CibouletteResourceSchema::Obj(_) => deserializer.deserialize_map(self),
            CibouletteResourceSchema::Array(_) => deserializer.deserialize_seq(self),
            CibouletteResourceSchema::Null => deserializer.deserialize_option(self),
        }
    }
}

impl<'de> DeserializeSeed<'de> for &'de CibouletteResourceSchema<'de> {
    type Value = CibouletteResourceSchemaValue<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            CibouletteResourceSchema::Bool(opt) => match opt.optional {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_bool(self),
            },
            CibouletteResourceSchema::String(opt) => match opt.optional {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_str(self),
            },
            CibouletteResourceSchema::Number(opt) => match opt.optional {
                true => deserializer.deserialize_option(self),
                false => match opt.type_ {
                    CibouletteResourceSchemaNumberType::U64 => deserializer.deserialize_u64(self),
                    CibouletteResourceSchemaNumberType::U128 => deserializer.deserialize_u128(self),
                },
            },
            CibouletteResourceSchema::Obj(opt) => match opt.optional {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_map(self),
            },
            CibouletteResourceSchema::Array(opt) => match opt.optional {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_seq(self),
            },
            CibouletteResourceSchema::Null => deserializer.deserialize_option(self),
        }
    }
}
