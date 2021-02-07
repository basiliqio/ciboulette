use super::*;
use serde::de::{DeserializeSeed, Deserializer, Visitor};
use serde_json::value::RawValue;
use std::fmt::Formatter;
const CIBOULETTE_RESOURCE_FIELDS: &[&str] =
    &["id", "type", "meta", "attributes", "relationships", "links"];

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteResourceBuilder<'a> {
    identifier: CibouletteResourceIdentifier<'a>,
    attributes: Option<&'a RawValue>,
    relationships: HashMap<Cow<'a, str>, CibouletteRelationship<'a>>,
    links: Option<CibouletteLink<'a>>,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteResource<'a> {
    identifier: CibouletteResourceIdentifier<'a>,
    attributes: Option<MessyJsonValueContainer<'a>>,
    relationships: HashMap<Cow<'a, str>, CibouletteRelationship<'a>>,
    links: Option<CibouletteLink<'a>>,
}

impl<'a> CibouletteResourceBuilder<'a> {
    pub fn deserialize<R>(d: &mut serde_json::Deserializer<R>) -> Result<Self, serde_json::Error>
    where
        R: serde_json::de::Read<'a>,
    {
        let visitor = CibouletteResourceBuilderVisitor;

        visitor.deserialize(d)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CibouletteResourceBuilderVisitor;

enum CibouletteResourceField {
    Id,
    Type,
    Meta,
    Attributes,
    Relationships,
    Links,
    Ignore,
}
struct CibouletteResourceFieldVisitor;
impl<'de> Visitor<'de> for CibouletteResourceFieldVisitor {
    type Value = CibouletteResourceField;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "field identifier")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            "id" => Ok(CibouletteResourceField::Id),
            "type" => Ok(CibouletteResourceField::Type),
            "meta" => Ok(CibouletteResourceField::Meta),
            "attributes" => Ok(CibouletteResourceField::Attributes),
            "relationships" => Ok(CibouletteResourceField::Relationships),
            "links" => Ok(CibouletteResourceField::Links),
            _ => Ok(CibouletteResourceField::Ignore),
        }
    }

    #[inline]
    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            b"id" => Ok(CibouletteResourceField::Id),
            b"type" => Ok(CibouletteResourceField::Type),
            b"meta" => Ok(CibouletteResourceField::Meta),
            b"attributes" => Ok(CibouletteResourceField::Attributes),
            b"relationships" => Ok(CibouletteResourceField::Relationships),
            b"links" => Ok(CibouletteResourceField::Links),
            _ => Ok(CibouletteResourceField::Ignore),
        }
    }
}
impl<'de> serde::Deserialize<'de> for CibouletteResourceField {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserializer::deserialize_identifier(deserializer, CibouletteResourceFieldVisitor)
    }
}
impl<'de> serde::de::Visitor<'de> for CibouletteResourceBuilderVisitor {
    type Value = CibouletteResourceBuilder<'de>;

    #[inline]
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
        let mut attributes: Option<&'de RawValue> = None;
        let mut relationships: Option<HashMap<Cow<'de, str>, CibouletteRelationship<'de>>> = None;
        let mut links: Option<CibouletteLink<'de>> = None;
        while let Some(key) =
            match serde::de::MapAccess::next_key::<CibouletteResourceField>(&mut map) {
                Ok(val) => val,
                Err(err) => {
                    return Err(err);
                }
            }
        {
            match key {
                CibouletteResourceField::Id => {
                    super::handle_ident_in_map_stateless(&mut id, &mut map, "id")?
                }
                CibouletteResourceField::Type => {
                    super::handle_ident_in_map_stateless(&mut type_, &mut map, "type")?
                }
                CibouletteResourceField::Meta => {
                    super::handle_ident_in_map_stateless(&mut meta, &mut map, "meta")?
                }
                CibouletteResourceField::Attributes => {
                    super::handle_ident_in_map_stateless(&mut attributes, &mut map, "attributes")?
                }
                CibouletteResourceField::Relationships => super::handle_ident_in_map_stateless(
                    &mut relationships,
                    &mut map,
                    "relationships",
                )?,
                CibouletteResourceField::Links => {
                    super::handle_ident_in_map_stateless(&mut links, &mut map, "links")?
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

        let id = id.ok_or_else(|| <A::Error as serde::de::Error>::missing_field("id"))?;
        let type_ = type_.ok_or_else(|| <A::Error as serde::de::Error>::missing_field("type"))?;
        let relationships = relationships.unwrap_or_default();
        Ok(CibouletteResourceBuilder {
            identifier: CibouletteResourceIdentifier::new(id, type_, meta.unwrap_or_default()),
            attributes,
            relationships,
            links,
        })
    }
}

impl<'de> DeserializeSeed<'de> for CibouletteResourceBuilderVisitor {
    type Value = CibouletteResourceBuilder<'de>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "CibouletteResource",
            CIBOULETTE_RESOURCE_FIELDS,
            CibouletteResourceBuilderVisitor,
        )
    }
}

impl<'a> CibouletteResourceBuilder<'a> {
    pub fn build(self, bag: &'a CibouletteBag) -> Result<CibouletteResource<'a>, CibouletteError> {
        let attributes: Option<MessyJsonValueContainer<'a>> = match self.attributes {
            Some(attributes) => {
                let type_ident = self.identifier().type_().as_ref();
                let resource_type = bag
                    .map()
                    .get(type_ident)
                    .ok_or_else(|| CibouletteError::UnknownType(type_ident.to_string()))?;
                let mut deserializer = serde_json::Deserializer::from_str(attributes.get());
                Some(
                    resource_type
                        .schema()
                        .builder()
                        .deserialize(&mut deserializer)?,
                )
            }
            None => None,
        };
        Ok(CibouletteResource {
            identifier: self.identifier,
            attributes,
            links: self.links,
            relationships: self.relationships,
        })
    }
}
