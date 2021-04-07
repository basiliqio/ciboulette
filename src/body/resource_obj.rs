use super::*;
use serde::de::{DeserializeSeed, Deserializer, Visitor};
use serde_json::value::RawValue;
use std::fmt::Formatter;
const CIBOULETTE_RESOURCE_FIELDS: &[&str] =
    &["id", "type", "meta", "attributes", "relationships", "links"];

/// ## Builder object for [CibouletterResource](CibouletterResource)
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteResourceBuilder<'request> {
    identifier: CibouletteResourceIdentifierBuilder<'request>,
    attributes: Option<&'request RawValue>,
    relationships: BTreeMap<Cow<'request, str>, CibouletteRelationshipObjectBuilder<'request>>,
    links: Option<CibouletteLink<'request>>,
    meta: Option<Value>,
}

/// ## A `json:api` [resource](https://jsonapi.org/format/#document-resource-objects) object
#[derive(Debug, Getters, MutGetters, Clone, Serialize)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResource<'request, 'store, B, T> {
    #[serde(flatten)]
    pub identifier: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<B>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub relationships: BTreeMap<ArcStr, CibouletteRelationshipObject<'request>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteLink<'request>>,
    #[serde(skip_serializing)]
    pub type_: Arc<CibouletteResourceType<'store>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}

impl<'request, 'store, B>
    TryFrom<
        CibouletteResource<'request, 'store, B, CibouletteResourceIdentifierPermissive<'request>>,
    > for CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>
{
    type Error = CibouletteError;

    fn try_from(
        value: CibouletteResource<
            'request,
            'store,
            B,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
    ) -> Result<Self, Self::Error> {
        let CibouletteResource::<
            'request,
            'store,
            B,
            CibouletteResourceIdentifierPermissive<'request>,
        > {
            identifier,
            attributes,
            relationships,
            links,
            type_,
            meta,
        } = value;

        Ok(
            CibouletteResource::<'request, 'store, B, CibouletteResourceIdentifier<'request>> {
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
    From<CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>>
    for CibouletteResource<'request, 'store, B, CibouletteResourceIdentifierPermissive<'request>>
{
    fn from(
        value: CibouletteResource<'request, 'store, B, CibouletteResourceIdentifier<'request>>,
    ) -> Self {
        let CibouletteResource::<'request, 'store, B, CibouletteResourceIdentifier<'request>> {
            identifier,
            attributes,
            relationships,
            links,
            type_,
            meta,
        } = value;

        CibouletteResource::<'request, 'store, B, CibouletteResourceIdentifierPermissive<'request>> {
            identifier: identifier.into(),
            attributes,
            relationships,
            links,
            type_,
            meta,
        }
    }
}

impl<'request> CibouletteResourceBuilder<'request> {
    pub fn deserialize<R>(d: &mut serde_json::Deserializer<R>) -> Result<Self, serde_json::Error>
    where
        R: serde_json::de::Read<'request>,
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
        let mut id: Option<CibouletteIdBuilder<'de>> = None;
        let mut type_: Option<Cow<'de, str>> = None;
        let mut meta: Option<Value> = None;
        let mut attributes: Option<&'de RawValue> = None;
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
        bag: &'store CibouletteStore<'store>,
        intention: &CibouletteIntention,
    ) -> Result<
        CibouletteResource<
            'request,
            'store,
            MessyJsonObjectValue<'store>,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
        CibouletteError,
    >
    where
        'request: 'store,
    {
        let current_type: &'store Arc<CibouletteResourceType<'store>> =
            bag.get_type(self.identifier().type_().as_ref())?;
        let attributes: Option<MessyJsonObjectValue<'store>> = match self.attributes {
            Some(attributes) => {
                let type_ident = self.identifier().type_().as_ref();
                let resource_type: &'store Arc<CibouletteResourceType<'store>> =
                    bag.get_type(type_ident)?;
                let mut deserializer = serde_json::Deserializer::from_str(attributes.get());
                let container: MessyJsonValueContainer<'store> = resource_type
                    .schema()
                    .builder(matches!(intention, CibouletteIntention::Update))
                    .deserialize(&mut deserializer)?;
                match container.take() {
                    MessyJsonValue::Obj(obj) => Some(obj),
                    _ => return Err(CibouletteError::AttributesIsNotAnObject),
                }
            }
            None => None,
        };
        let mut relationships: BTreeMap<ArcStr, CibouletteRelationshipObject<'request>> =
            BTreeMap::new();
        for (k, v) in self.relationships {
            match current_type.relationships().get_key_value(k.as_ref()) {
                Some((k, _)) => {
                    relationships.insert(k.clone(), v.build(&current_type)?);
                }
                None => {
                    return Err(CibouletteError::UnknownRelationship(
                        current_type.name().to_string(),
                        k.to_string(),
                    ))
                }
            }
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

impl<'request, 'store, T> CibouletteResource<'request, 'store, MessyJsonObjectValue<'store>, T>
where
    'store: 'request,
{
    #[inline]
    fn check_member_name_inner(val: &MessyJsonValue<'store>) -> Option<String> {
        match val {
            MessyJsonValue::Obj(map) => {
                for (k, v) in map.iter() {
                    if !crate::member_name::check_member_name(&*k) {
                        return Some(k.to_string());
                    }
                    if let Some(x) = Self::check_member_name_inner(v) {
                        return Some(x);
                    }
                }
                None
            }
            MessyJsonValue::Array(arr) => {
                for element in arr.iter() {
                    if let Some(x) = Self::check_member_name_inner(element) {
                        return Some(x);
                    }
                }
                None
            }
            _ => None,
        }
    }
    #[inline]
    fn check_member_name_top(
        val: &BTreeMap<Cow<'request, str>, MessyJsonValue<'store>>,
    ) -> Option<String> {
        for (k, v) in val.iter() {
            if !crate::member_name::check_member_name(&*k) {
                return Some(k.to_string());
            }
            if let Some(x) = Self::check_member_name_inner(v) {
                return Some(x);
            }
        }
        None
    }

    pub fn check_member_name(&self) -> Result<(), CibouletteError> {
        match self.attributes() {
            Some(attributes) => {
                if let Some(x) = Self::check_member_name_top(attributes) {
                    return Err(CibouletteError::InvalidMemberName(x));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
