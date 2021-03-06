use super::*;
use serde::de::{DeserializeSeed, Deserializer, Visitor};
use std::fmt::Formatter;
const CIBOULETTE_RESOURCE_FIELDS: &[&str] =
    &["id", "type", "meta", "attributes", "relationships", "links"];

/// ## Builder object for [CibouletterResource](CibouletterResource)
#[derive(Debug, Getters, Serialize)]
#[getset(get = "pub")]
pub struct CibouletteResourceBuilder<'request> {
    identifier: CibouletteResourceIdentifierBuilder<'request>,
    #[serde(skip_serializing_if = "CibouletteOptionalData::is_absent")]
    attributes: CibouletteOptionalData<MessyJsonValueRaw<'request>>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    relationships: BTreeMap<Cow<'request, str>, CibouletteRelationshipObjectBuilder<'request>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<CibouletteLink<'request>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Value>,
}

/// ## A `json:api` [resource](https://jsonapi.org/format/#document-resource-objects) object
#[derive(Debug, Getters, MutGetters, Clone, Serialize)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResource<'request, B, T> {
    #[serde(flatten)]
    pub identifier: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<B>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub relationships: BTreeMap<ArcStr, CibouletteRelationshipObject<'request>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteLink<'request>>,
    #[serde(skip_serializing)]
    pub type_: Arc<CibouletteResourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}

impl<'request, 'store, B>
    TryFrom<CibouletteResource<'request, B, CibouletteResourceIdentifierPermissive<'request>>>
    for CibouletteResource<'request, B, CibouletteResourceIdentifier<'request>>
{
    type Error = CibouletteError;

    fn try_from(
        value: CibouletteResource<'request, B, CibouletteResourceIdentifierPermissive<'request>>,
    ) -> Result<Self, Self::Error> {
        let CibouletteResource::<'request, B, CibouletteResourceIdentifierPermissive<'request>> {
            identifier,
            attributes,
            relationships,
            links,
            type_,
            meta,
        } = value;

        Ok(
            CibouletteResource::<'request, B, CibouletteResourceIdentifier<'request>> {
                identifier: identifier.try_into()?,
                attributes,
                relationships,
                links,
                type_,
                meta,
            },
        )
    }
}

impl<'request, 'store, B>
    From<CibouletteResource<'request, B, CibouletteResourceIdentifier<'request>>>
    for CibouletteResource<'request, B, CibouletteResourceIdentifierPermissive<'request>>
{
    fn from(
        value: CibouletteResource<'request, B, CibouletteResourceIdentifier<'request>>,
    ) -> Self {
        let CibouletteResource::<'request, B, CibouletteResourceIdentifier<'request>> {
            identifier,
            attributes,
            relationships,
            links,
            type_,
            meta,
        } = value;

        CibouletteResource::<'request, B, CibouletteResourceIdentifierPermissive<'request>> {
            identifier: identifier.into(),
            attributes,
            relationships,
            links,
            type_,
            meta,
        }
    }
}

impl<'de> Deserialize<'de> for CibouletteResourceBuilder<'de> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
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
        let mut meta: Option<Value> = None;
        let mut attributes: CibouletteOptionalData<MessyJsonValueRaw<'de>> =
            CibouletteOptionalData::Null(false);
        let mut relationships: Option<
            BTreeMap<Cow<'de, str>, CibouletteRelationshipObjectBuilder<'de>>,
        > = None;
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
                    crate::serde_utils::handle_ident_in_map_stateless(&mut id, &mut map, "id")?
                }
                CibouletteResourceField::Type => {
                    crate::serde_utils::handle_ident_in_map_stateless(&mut type_, &mut map, "type")?
                }
                CibouletteResourceField::Meta => {
                    crate::serde_utils::handle_ident_in_map_stateless(&mut meta, &mut map, "meta")?
                }
                CibouletteResourceField::Attributes => {
                    crate::serde_utils::handle_ident_in_map_stateless_ciboulette_optional(
                        &mut attributes,
                        &mut map,
                        "attributes",
                    )?
                }
                CibouletteResourceField::Relationships => {
                    crate::serde_utils::handle_ident_in_map_stateless(
                        &mut relationships,
                        &mut map,
                        "relationships",
                    )?
                }
                CibouletteResourceField::Links => {
                    crate::serde_utils::handle_ident_in_map_stateless(
                        &mut links, &mut map, "links",
                    )?
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

        let id = id;
        let type_ = type_.ok_or_else(|| <A::Error as serde::de::Error>::missing_field("type"))?;
        let relationships = relationships.unwrap_or_default();
        Ok(CibouletteResourceBuilder {
            identifier: CibouletteResourceIdentifierBuilder::new(id, type_),
            attributes,
            relationships,
            links,
            meta,
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

impl<'request> CibouletteResourceBuilder<'request> {
    /// ## build the [CibouletteResource](CibouletteResource) from the builder
    pub fn build<'store>(
        self,
        bag: &'store CibouletteStore,
        intention: &CibouletteIntention,
    ) -> Result<
        CibouletteResource<
            'request,
            MessyJsonObjectValue<'request>,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
        CibouletteError,
    > {
        let current_type: &Arc<CibouletteResourceType> =
            bag.get_type(self.identifier().type_().as_ref())?;
        let resource_type: Arc<CibouletteResourceType> =
            bag.get_type(self.identifier().type_().as_ref())?.clone();
        let attributes: Option<MessyJsonObjectValue<'request>> = match self.attributes {
            CibouletteOptionalData::Object(attributes) => {
                let deserializer_settings = matches!(intention, CibouletteIntention::Update);
                let container_builder = resource_type.schema().builder(MessyJsonSettings {
                    all_optional: deserializer_settings,
                    preserve_mandatory: deserializer_settings,
                });
                let container = container_builder.deserialize(attributes)?;
                match container.take() {
                    MessyJsonValue::Obj(obj) => Some(obj),
                    _ => return Err(CibouletteError::AttributesIsNotAnObject),
                }
            }
            _ => None,
        };
        let mut relationships: BTreeMap<ArcStr, CibouletteRelationshipObject<'request>> =
            BTreeMap::new();
        for (k, v) in self.relationships {
            let (rel_alias, rel_type) =
                current_type.get_relationship_with_alias(&bag, k.as_ref())?;
            relationships.insert(rel_alias, v.build(&rel_type)?);
        }
        Ok(CibouletteResource {
            identifier: self.identifier.build_permissive(&current_type)?,
            attributes,
            links: self.links,
            relationships,
            type_: current_type.clone(),
            meta: self.meta,
        })
    }
}
