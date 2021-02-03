use super::*;
use serde::{de::DeserializeSeed, de::MapAccess, de::SeqAccess, de::Visitor, Deserializer};
use std::fmt::Formatter;
use std::marker::PhantomData;
const FIELDS: &[&str] = &["id", "type", "meta", "attributes", "relationships", "links"];

#[derive(Debug, Getters)]
// #[serde(rename = "camelCase")]
#[getset(get = "pub")]
pub struct CibouletteResource<'a> {
    // #[serde(flatten)]
    identifier: CibouletteResourceIdentifier<'a>,
    attributes: Option<MessyJsonValueContainer<'a>>,
    relationships: Option<HashMap<Cow<'a, str>, CibouletteRelationship<'a>>>,
    links: Option<CibouletteLink<'a>>,
}

impl<'a> CibouletteResource<'a> {
    pub fn deserialize<R>(
        d: &mut serde_json::Deserializer<R>,
        bag: &'a CibouletteBag,
    ) -> Result<Self, serde_json::Error>
    where
        R: serde_json::de::Read<'a>,
    {
        let visitor = CibouletteResourceVisitor(bag);

        visitor.deserialize(d)
    }
}

#[derive(Clone, Debug)]
pub struct CibouletteResourceVisitor<'a>(&'a CibouletteBag<'a>);
enum Field {
    Id,
    Type,
    Meta,
    Attributes,
    Relationships,
    Links,
    Ignore,
}
struct FieldVisitor;
impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "field identifier")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            "id" => Ok(Field::Id),
            "type" => Ok(Field::Type),
            "meta" => Ok(Field::Meta),
            "attributes" => Ok(Field::Attributes),
            "relationships" => Ok(Field::Relationships),
            "links" => Ok(Field::Links),
            _ => Ok(Field::Ignore),
        }
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            b"id" => Ok(Field::Id),
            b"type" => Ok(Field::Type),
            b"meta" => Ok(Field::Meta),
            b"attributes" => Ok(Field::Attributes),
            b"relationships" => Ok(Field::Relationships),
            b"links" => Ok(Field::Links),
            _ => Ok(Field::Ignore),
        }
    }
}
impl<'de> serde::Deserialize<'de> for Field {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
    }
}
impl<'de> serde::de::Visitor<'de> for CibouletteResourceVisitor<'de> {
    type Value = CibouletteResource<'de>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResource")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut id: Option<Cow<'de, str>> = None;
        let mut type_: Option<Cow<'de, str>> = None;
        let mut meta: Option<HashMap<Cow<'de, str>, Value>> = None;
        let mut attributes: Option<MessyJsonValueContainer<'de>> = None;
        let mut relationships: Option<HashMap<Cow<'de, str>, CibouletteRelationship<'de>>> = None;
        let mut links: Option<CibouletteLink<'de>> = None;
        while let Some(key) = match serde::de::MapAccess::next_key::<Field>(&mut map) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        } {
            match key {
                Field::Id => {
                    if Option::is_some(&id) {
                        return Err(<A::Error as serde::de::Error>::duplicate_field("id"));
                    }
                    id = Some(
                        match serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map) {
                            Ok(val) => val,
                            Err(err) => {
                                return Err(err);
                            }
                        },
                    );
                }
                Field::Type => {
                    if Option::is_some(&type_) {
                        return Err(<A::Error as serde::de::Error>::duplicate_field("type"));
                    }
                    type_ = Some(
                        match serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map) {
                            Ok(val) => val,
                            Err(err) => {
                                return Err(err);
                            }
                        },
                    );
                }
                Field::Meta => {
                    if Option::is_some(&meta) {
                        return Err(<A::Error as serde::de::Error>::duplicate_field("meta"));
                    }
                    meta = Some(
                        match serde::de::MapAccess::next_value::<HashMap<Cow<'de, str>, Value>>(
                            &mut map,
                        ) {
                            Ok(val) => val,
                            Err(err) => {
                                return Err(err);
                            }
                        },
                    );
                }
                Field::Attributes => {
                    if Option::is_some(&attributes) {
                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                            "attributes",
                        ));
                    }
                    // attributes = Some( // TODO
                    // 	match serde::de::MapAccess::next_value_seed::<MessyJsonBuilder<'de>>(&mut map) {
                    // 		Ok(val) => val,
                    // 		Err(err) => {
                    // 			return Err(err);
                    // 		}
                    // 	},
                    // );
                }
                Field::Relationships => {
                    if Option::is_some(&relationships) {
                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                            "relationships",
                        ));
                    }
                    relationships = Some(
                        match serde::de::MapAccess::next_value::<
                            HashMap<Cow<'de, str>, CibouletteRelationship<'de>>,
                        >(&mut map)
                        {
                            Ok(val) => val,
                            Err(err) => {
                                return Err(err);
                            }
                        },
                    );
                }
                Field::Links => {
                    if Option::is_some(&links) {
                        return Err(<A::Error as serde::de::Error>::duplicate_field("links"));
                    }
                    links = Some(
                        match serde::de::MapAccess::next_value::<CibouletteLink<'de>>(&mut map) {
                            Ok(val) => val,
                            Err(err) => {
                                return Err(err);
                            }
                        },
                    );
                }
                _ => {
                    let _ =
                        match serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut map) {
                            Ok(val) => val,
                            Err(err) => {
                                return Err(err);
                            }
                        };
                }
            }
        }

        let id = id.ok_or(<A::Error as serde::de::Error>::missing_field("id"))?;
        let type_ = type_.ok_or(<A::Error as serde::de::Error>::missing_field("type"))?;
        Ok(CibouletteResource {
            identifier: CibouletteResourceIdentifier::new(id, type_, meta.unwrap_or_default()),
            attributes,
            relationships,
            links,
        })
    }
}
#[derive(Clone, Debug)]
pub struct CibouletteResourceSelectorVisitor<'a>(&'a CibouletteBag<'a>);

impl<'de> serde::de::Visitor<'de> for CibouletteResourceSelectorVisitor<'de> {
    type Value = CibouletteResourceSelector<'de>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResourceSelector")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut res: Vec<CibouletteResource<'de>> =
            Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(v) = seq.next_element_seed(CibouletteResourceVisitor(self.0))? {
            res.push(v);
        }
        Ok(CibouletteResourceSelector::Many(res))
    }

    #[inline]
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let v = CibouletteResourceVisitor(self.0);
        Ok(CibouletteResourceSelector::One(
            <CibouletteResourceVisitor as Visitor>::visit_map(v, map)?,
        ))
    }
}

#[derive(Debug)]
// #[serde(untagged)]
pub enum CibouletteResourceSelector<'a> {
    One(CibouletteResource<'a>),
    Many(Vec<CibouletteResource<'a>>),
    Null,
}

impl<'a> CibouletteResourceSelector<'a> {
    pub fn deserialize<R>(
        d: &mut serde_json::Deserializer<R>,
        bag: &'a CibouletteBag,
    ) -> Result<Self, serde_json::Error>
    where
        R: serde_json::de::Read<'a>,
    {
        let visitor = CibouletteResourceSelectorVisitor(bag);

        visitor.deserialize(d)
    }
}

impl<'de> DeserializeSeed<'de> for CibouletteResourceVisitor<'de> {
    type Value = CibouletteResource<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "CibouletteResource",
            FIELDS,
            CibouletteResourceVisitor(self.0),
        )
    }
}

impl<'de> DeserializeSeed<'de> for CibouletteResourceSelectorVisitor<'de> {
    type Value = CibouletteResourceSelector<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CibouletteResourceSelectorVisitor(self.0))
        // if let Ok(res) = deserializer.deserialize_seq(CibouletteResourceSelectorVisitor(self.0))
        // {
        // 	return Ok(res);
        // }

        // if let Ok(res) = deserializer.deserialize_struct("CibouletteResource", FIELDS, CibouletteResourceSelectorVisitor(self.0))
        // {
        // 	return Ok(res);
        // }

        // if let Ok(res) = deserializer.deserialize_option(CibouletteResourceSelectorVisitor(self.0))
        // {
        // 	return Ok(res);
        // }

        // Err(<D::Error as serde::de::Error>::custom("Cannot deserialize resource selector. Should be either an object, an array or null"))
    }
}
