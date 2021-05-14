use itertools::Itertools;

use super::*;
use serde::ser::SerializeStruct;
use std::cmp::{Ord, Ordering};

/// ## Builder for resource identifier
///
/// When building, the `id` is optional.
#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifierBuilder<'request> {
    #[serde(rename = "type")]
    pub type_: Cow<'request, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Cow<'request, str>>,
}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
///
/// The `id` is not optional in that case
#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifier<'request> {
    pub type_: Cow<'request, str>,
    pub id: CibouletteIdSelector<'request>,
}

impl<'request> Ord for CibouletteResourceIdentifier<'request> {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_cmp = self.type_.cmp(other.type_());
        match type_cmp {
            Ordering::Equal => self.id.cmp(other.id()),
            _ => type_cmp,
        }
    }
}

impl<'request> PartialOrd for CibouletteResourceIdentifier<'request> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_cmp = self.type_.cmp(other.type_());
        match type_cmp {
            Ordering::Equal => Some(self.id.cmp(other.id())),
            _ => Some(type_cmp),
        }
    }
}

impl<'request> PartialEq for CibouletteResourceIdentifier<'request> {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.id == other.id
    }
}

impl<'request> Eq for CibouletteResourceIdentifier<'request> {}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
///
/// The `id` is optional in that case
#[derive(Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifierPermissive<'request> {
    pub type_: Cow<'request, str>,
    pub id: Option<CibouletteIdSelector<'request>>,
}

impl<'request> CibouletteResourceIdentifierBuilder<'request> {
    /// Build the resource identifier, with type being a relationship alias for the `main_type`
    pub fn build_relationships(
        self,
        store: &CibouletteStore,
        main_type: &CibouletteResourceType,
    ) -> Result<CibouletteResourceIdentifier<'request>, CibouletteError> {
        let rel = main_type.get_relationship(store, &self.type_)?;

        Ok(CibouletteResourceIdentifier {
            id: match self.id {
                Some(id) => CibouletteIdSelector::build_id(rel.ids(), id)?,
                None => return Err(CibouletteError::MissingId),
            },
            type_: self.type_,
        })
    }
    /// Build the resource identifier providing the type that is beeing parsed
    pub fn build(
        self,
        type_: &CibouletteResourceType,
    ) -> Result<CibouletteResourceIdentifier<'request>, CibouletteError> {
        Ok(CibouletteResourceIdentifier {
            type_: self.type_,
            id: match self.id {
                Some(id) => CibouletteIdSelector::build_id(type_.ids(), id)?,
                None => return Err(CibouletteError::MissingId),
            },
        })
    }

    /// Build the resource identifier providing the type that is beeing parsed.
    ///
    /// Allows the `id` key to be empty (for `POST` for instance)
    pub fn build_permissive(
        self,
        type_: &CibouletteResourceType,
    ) -> Result<CibouletteResourceIdentifierPermissive<'request>, CibouletteError> {
        Ok(CibouletteResourceIdentifierPermissive {
            type_: self.type_,
            id: match self.id {
                Some(id) => Some(CibouletteIdSelector::build_id(type_.ids(), id)?),
                None => None,
            },
        })
    }
}

impl<'request> TryFrom<CibouletteResourceIdentifierPermissive<'request>>
    for CibouletteResourceIdentifier<'request>
{
    type Error = CibouletteError;

    fn try_from(
        value: CibouletteResourceIdentifierPermissive<'request>,
    ) -> Result<Self, Self::Error> {
        let CibouletteResourceIdentifierPermissive { type_, id } = value;

        Ok(CibouletteResourceIdentifier {
            type_,
            id: id.ok_or(CibouletteError::MissingId)?,
        })
    }
}

impl<'request> From<CibouletteResourceIdentifier<'request>>
    for CibouletteResourceIdentifierPermissive<'request>
{
    fn from(value: CibouletteResourceIdentifier<'request>) -> Self {
        let CibouletteResourceIdentifier { type_, id } = value;

        CibouletteResourceIdentifierPermissive {
            type_,
            id: Some(id),
        }
    }
}

impl<'request> CibouletteResourceIdentifier<'request> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: CibouletteIdSelector<'request>, type_: Cow<'request, str>) -> Self {
        CibouletteResourceIdentifier { id, type_ }
    }

    /// Create a comma separated string of the identifiers
    pub fn id_to_string(&self) -> String {
        match self.id() {
            CibouletteIdSelector::Single(x) => x.to_string(),
            CibouletteIdSelector::Multi(x) => x
                .iter()
                .map(|x| match x {
                    CibouletteId::Text(id) => base64::encode(id.as_ref()),
                    _ => x.to_string(),
                })
                .join(","),
        }
    }
}

impl<'request> CibouletteResourceIdentifierPermissive<'request> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<CibouletteIdSelector<'request>>, type_: Cow<'request, str>) -> Self {
        CibouletteResourceIdentifierPermissive { id, type_ }
    }

    /// Create a comma separated string of the identifiers
    pub fn id_to_string(&self) -> String {
        match self.id() {
            Some(CibouletteIdSelector::Single(x)) => x.to_string(),
            Some(CibouletteIdSelector::Multi(x)) => x.iter().join(","),
            None => String::default(),
        }
    }
}

impl<'request> CibouletteResourceIdentifierBuilder<'request> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<Cow<'request, str>>, type_: Cow<'request, str>) -> Self {
        CibouletteResourceIdentifierBuilder { id, type_ }
    }
}

/// ## A selector for resource identifier, to either select one or many resource identifiers
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteResourceIdentifierSelectorBuilder<'request> {
    One(CibouletteResourceIdentifierBuilder<'request>),
    Many(Vec<CibouletteResourceIdentifierBuilder<'request>>),
}

impl<'request> CibouletteResourceIdentifierSelectorBuilder<'request> {
    /// Build the underlyings resource identifiers
    pub fn build(
        self,
        type_: &CibouletteResourceType,
    ) -> Result<CibouletteResourceIdentifierSelector<'request>, CibouletteError> {
        match self {
            CibouletteResourceIdentifierSelectorBuilder::One(x) => {
                Ok(CibouletteResourceIdentifierSelector::One(x.build(type_)?))
            }
            CibouletteResourceIdentifierSelectorBuilder::Many(ids) => {
                let mut res: Vec<CibouletteResourceIdentifier<'request>> =
                    Vec::with_capacity(ids.len());

                for id in ids.into_iter() {
                    res.push(id.build(&type_)?);
                }
                Ok(CibouletteResourceIdentifierSelector::Many(res))
            }
        }
    }
}

/// ## A selector between a single or multiple `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) objects
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteResourceIdentifierSelector<'request> {
    One(CibouletteResourceIdentifier<'request>),
    Many(Vec<CibouletteResourceIdentifier<'request>>),
}

impl<'request, B> From<CibouletteResource<'request, B, CibouletteResourceIdentifier<'request>>>
    for CibouletteResourceIdentifierSelector<'request>
{
    fn from(obj: CibouletteResource<'request, B, CibouletteResourceIdentifier<'request>>) -> Self {
        CibouletteResourceIdentifierSelector::One(obj.identifier)
    }
}

impl<'request, B>
    TryFrom<CibouletteResource<'request, B, CibouletteResourceIdentifierPermissive<'request>>>
    for CibouletteResourceIdentifierSelector<'request>
{
    type Error = CibouletteError;

    fn try_from(
        obj: CibouletteResource<'request, B, CibouletteResourceIdentifierPermissive<'request>>,
    ) -> Result<Self, Self::Error> {
        Ok(CibouletteResourceIdentifierSelector::One(
            obj.identifier.try_into()?,
        ))
    }
}

impl<'request, B>
    From<CibouletteResourceSelector<'request, B, CibouletteResourceIdentifier<'request>>>
    for CibouletteResourceIdentifierSelector<'request>
{
    fn from(
        obj: CibouletteResourceSelector<'request, B, CibouletteResourceIdentifier<'request>>,
    ) -> Self {
        match obj {
            CibouletteResourceSelector::One(x) => {
                CibouletteResourceIdentifierSelector::One(x.identifier)
            }
            CibouletteResourceSelector::Many(x) => CibouletteResourceIdentifierSelector::Many(
                x.into_iter().map(|x| x.identifier).collect(),
            ),
        }
    }
}

impl<'request, B>
    TryFrom<
        CibouletteResourceSelector<'request, B, CibouletteResourceIdentifierPermissive<'request>>,
    > for CibouletteResourceIdentifierSelector<'request>
{
    type Error = CibouletteError;

    fn try_from(
        obj: CibouletteResourceSelector<
            'request,
            B,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
    ) -> Result<Self, Self::Error> {
        match obj {
            CibouletteResourceSelector::One(x) => Ok(CibouletteResourceIdentifierSelector::One(
                x.identifier.try_into()?,
            )),
            CibouletteResourceSelector::Many(x) => {
                let mut res: Vec<CibouletteResourceIdentifier<'request>> =
                    Vec::with_capacity(x.len());

                for x in x.into_iter() {
                    res.push(x.identifier.try_into()?);
                }
                Ok(CibouletteResourceIdentifierSelector::Many(res))
            }
        }
    }
}

impl<'request> Serialize for CibouletteResourceIdentifier<'request> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CibouletteResourceIdentifier", 3)?;
        state.serialize_field("type", &self.type_)?;
        match self.id.len() {
            0 => state.skip_field("id")?,
            1 => state.serialize_field(
                "id",
                self.id().get(0).map_err(|_| {
                    serde::ser::Error::custom("Wrong number of id when serializing")
                })?,
            )?,
            _ => state.serialize_field("id", &self.id_to_string())?,
        };
        state.end()
    }
}

impl<'request> Serialize for CibouletteResourceIdentifierPermissive<'request> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CibouletteResourceIdentifier", 3)?;
        state.serialize_field("type", &self.type_)?;
        if let Some(id) = &self.id {
            match id.len() {
                0 => state.skip_field("id")?,
                1 => state.serialize_field(
                    "id",
                    id.get(0).map_err(|_| {
                        serde::ser::Error::custom("Wrong number of id when serializing")
                    })?,
                )?,
                _ => state.serialize_field("id", &self.id_to_string())?,
            };
        } else {
            state.skip_field("id")?
        }
        state.end()
    }
}
