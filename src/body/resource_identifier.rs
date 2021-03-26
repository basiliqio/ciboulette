use super::*;
use std::cmp::{Ord, Ordering};

#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifierBuilder<'a> {
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    pub id: Option<CibouletteIdBuilder<'a>>,
}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifier<'a> {
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    pub id: CibouletteId<'a>,
}

impl<'a> Ord for CibouletteResourceIdentifier<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_cmp = self.type_.cmp(other.type_());
        match type_cmp {
            Ordering::Equal => self.id.cmp(other.id()),
            _ => type_cmp,
        }
    }
}

impl<'a> PartialOrd for CibouletteResourceIdentifier<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_cmp = self.type_.cmp(other.type_());
        match type_cmp {
            Ordering::Equal => Some(self.id.cmp(other.id())),
            _ => Some(type_cmp),
        }
    }
}

impl<'a> PartialEq for CibouletteResourceIdentifier<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.id == other.id
    }
}

impl<'a> Eq for CibouletteResourceIdentifier<'a> {}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifierPermissive<'a> {
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    pub id: Option<CibouletteId<'a>>,
}

impl<'a> CibouletteResourceIdentifierBuilder<'a> {
    pub fn build_from_store(
        self,
        store: &'a CibouletteStore<'a>,
    ) -> Result<CibouletteResourceIdentifier<'a>, CibouletteError> {
        Ok(CibouletteResourceIdentifier {
            id: match self.id {
                Some(id) => id.build(&store.get_type(&self.type_)?.id_type())?,
                None => return Err(CibouletteError::MissingId),
            },
            type_: self.type_,
        })
    }
    pub fn build(
        self,
        type_: &CibouletteResourceType<'a>,
    ) -> Result<CibouletteResourceIdentifier<'a>, CibouletteError> {
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
        type_: &CibouletteResourceType<'a>,
    ) -> Result<CibouletteResourceIdentifierPermissive<'a>, CibouletteError> {
        Ok(CibouletteResourceIdentifierPermissive {
            type_: self.type_,
            id: match self.id {
                Some(id) => Some(id.build(&type_.id_type())?),
                None => None,
            },
        })
    }
}

impl<'a> TryFrom<CibouletteResourceIdentifierPermissive<'a>> for CibouletteResourceIdentifier<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteResourceIdentifierPermissive<'a>) -> Result<Self, Self::Error> {
        let CibouletteResourceIdentifierPermissive { type_, id } = value;

        Ok(CibouletteResourceIdentifier {
            type_,
            id: id.ok_or(CibouletteError::MissingId)?,
        })
    }
}

impl<'a> From<CibouletteResourceIdentifier<'a>> for CibouletteResourceIdentifierPermissive<'a> {
    fn from(value: CibouletteResourceIdentifier<'a>) -> Self {
        let CibouletteResourceIdentifier { type_, id } = value;

        CibouletteResourceIdentifierPermissive {
            type_,
            id: Some(id),
        }
    }
}

impl<'a> CibouletteResourceIdentifier<'a> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: CibouletteId<'a>, type_: Cow<'a, str>) -> Self {
        CibouletteResourceIdentifier { id, type_ }
    }
}

impl<'a> CibouletteResourceIdentifierPermissive<'a> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<CibouletteId<'a>>, type_: Cow<'a, str>) -> Self {
        CibouletteResourceIdentifierPermissive { id, type_ }
    }
}

impl<'a> CibouletteResourceIdentifierBuilder<'a> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<CibouletteIdBuilder<'a>>, type_: Cow<'a, str>) -> Self {
        CibouletteResourceIdentifierBuilder { id, type_ }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteResourceIdentifierSelectorBuilder<'a> {
    One(CibouletteResourceIdentifierBuilder<'a>),
    Many(Vec<CibouletteResourceIdentifierBuilder<'a>>),
}

impl<'a> CibouletteResourceIdentifierSelectorBuilder<'a> {
    pub fn build(
        self,
        type_: &CibouletteResourceType<'a>,
    ) -> Result<CibouletteResourceIdentifierSelector<'a>, CibouletteError> {
        match self {
            CibouletteResourceIdentifierSelectorBuilder::One(x) => {
                Ok(CibouletteResourceIdentifierSelector::One(x.build(type_)?))
            }
            CibouletteResourceIdentifierSelectorBuilder::Many(ids) => {
                let mut res: Vec<CibouletteResourceIdentifier<'a>> = Vec::with_capacity(ids.len());

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
pub enum CibouletteResourceIdentifierSelector<'a> {
    One(CibouletteResourceIdentifier<'a>),
    Many(Vec<CibouletteResourceIdentifier<'a>>),
}

impl<'a, B> From<CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>>
    for CibouletteResourceIdentifierSelector<'a>
{
    fn from(obj: CibouletteResource<'a, B, CibouletteResourceIdentifier<'a>>) -> Self {
        CibouletteResourceIdentifierSelector::One(obj.identifier)
    }
}

impl<'a, B> TryFrom<CibouletteResource<'a, B, CibouletteResourceIdentifierPermissive<'a>>>
    for CibouletteResourceIdentifierSelector<'a>
{
    type Error = CibouletteError;

    fn try_from(
        obj: CibouletteResource<'a, B, CibouletteResourceIdentifierPermissive<'a>>,
    ) -> Result<Self, Self::Error> {
        Ok(CibouletteResourceIdentifierSelector::One(
            obj.identifier.try_into()?,
        ))
    }
}

impl<'a, B> From<CibouletteResourceSelector<'a, B, CibouletteResourceIdentifier<'a>>>
    for CibouletteResourceIdentifierSelector<'a>
{
    fn from(obj: CibouletteResourceSelector<'a, B, CibouletteResourceIdentifier<'a>>) -> Self {
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

impl<'a, B> TryFrom<CibouletteResourceSelector<'a, B, CibouletteResourceIdentifierPermissive<'a>>>
    for CibouletteResourceIdentifierSelector<'a>
{
    type Error = CibouletteError;

    fn try_from(
        obj: CibouletteResourceSelector<'a, B, CibouletteResourceIdentifierPermissive<'a>>,
    ) -> Result<Self, Self::Error> {
        match obj {
            CibouletteResourceSelector::One(x) => Ok(CibouletteResourceIdentifierSelector::One(
                x.identifier.try_into()?,
            )),
            CibouletteResourceSelector::Many(x) => {
                let mut res: Vec<CibouletteResourceIdentifier<'a>> = Vec::with_capacity(x.len());

                for x in x.into_iter() {
                    res.push(x.identifier.try_into()?);
                }
                Ok(CibouletteResourceIdentifierSelector::Many(res))
            }
        }
    }
}
