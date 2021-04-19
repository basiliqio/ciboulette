use super::*;
use std::cmp::{Ord, Ordering};

#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifierBuilder<'request> {
    #[serde(rename = "type")]
    pub type_: Cow<'request, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CibouletteIdBuilder<'request>>,
}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifier<'request> {
    #[serde(rename = "type")]
    pub type_: Cow<'request, str>,
    pub id: CibouletteId<'request>,
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
#[derive(Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifierPermissive<'request> {
    #[serde(rename = "type")]
    pub type_: Cow<'request, str>,
    pub id: Option<CibouletteId<'request>>,
}

impl<'request> CibouletteResourceIdentifierBuilder<'request> {
    pub fn build_from_store(
        self,
        store: &CibouletteStore,
    ) -> Result<CibouletteResourceIdentifier<'request>, CibouletteError> {
        Ok(CibouletteResourceIdentifier {
            id: match self.id {
                Some(id) => id.build(&store.get_type(&self.type_)?.id_type())?,
                None => return Err(CibouletteError::MissingId),
            },
            type_: self.type_,
        })
    }

    pub fn build_relationships(
        self,
        store: &CibouletteStore,
        main_type: &CibouletteResourceType,
    ) -> Result<CibouletteResourceIdentifier<'request>, CibouletteError> {
        let rel = main_type.get_relationship(store, &self.type_)?;
        let id_type = rel.id_type();
        Ok(CibouletteResourceIdentifier {
            id: match self.id {
                Some(id) => id.build(id_type)?,
                None => return Err(CibouletteError::MissingId),
            },
            type_: self.type_,
        })
    }
    pub fn build(
        self,
        type_: &CibouletteResourceType,
    ) -> Result<CibouletteResourceIdentifier<'request>, CibouletteError> {
        Ok(CibouletteResourceIdentifier {
            type_: self.type_,
            id: match self.id {
                Some(id) => id.build(&type_.id_type())?,
                None => return Err(CibouletteError::MissingId),
            },
        })
    }

    pub fn build_permissive(
        self,
        type_: &CibouletteResourceType,
    ) -> Result<CibouletteResourceIdentifierPermissive<'request>, CibouletteError> {
        Ok(CibouletteResourceIdentifierPermissive {
            type_: self.type_,
            id: match self.id {
                Some(id) => Some(id.build(&type_.id_type())?),
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
    pub fn new(id: CibouletteId<'request>, type_: Cow<'request, str>) -> Self {
        CibouletteResourceIdentifier { id, type_ }
    }
}

impl<'request> CibouletteResourceIdentifierPermissive<'request> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<CibouletteId<'request>>, type_: Cow<'request, str>) -> Self {
        CibouletteResourceIdentifierPermissive { id, type_ }
    }
}

impl<'request> CibouletteResourceIdentifierBuilder<'request> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<CibouletteIdBuilder<'request>>, type_: Cow<'request, str>) -> Self {
        CibouletteResourceIdentifierBuilder { id, type_ }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteResourceIdentifierSelectorBuilder<'request> {
    One(CibouletteResourceIdentifierBuilder<'request>),
    Many(Vec<CibouletteResourceIdentifierBuilder<'request>>),
}

impl<'request> CibouletteResourceIdentifierSelectorBuilder<'request> {
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
