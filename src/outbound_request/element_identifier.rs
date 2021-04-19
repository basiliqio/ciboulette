use super::*;
use std::borrow::Borrow;
use std::cmp::{Ord, Ordering};

#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceResponseIdentifierBuilder<'request> {
    #[serde(rename = "type")]
    pub type_: Cow<'request, str>,
    pub id: Option<CibouletteIdBuilder<'request>>,
}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceResponseIdentifier<'request> {
    #[serde(rename = "type")]
    pub type_: ArcStr,
    pub id: CibouletteId<'request>,
    #[serde(skip_serializing)]
    pub rel: Option<ArcStr>,
}

impl<'request> Ord for CibouletteResourceResponseIdentifier<'request> {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_cmp = self.type_.cmp(other.type_());
        match type_cmp {
            Ordering::Equal => self.id.cmp(other.id()),
            _ => type_cmp,
        }
    }
}

impl<'request> PartialOrd for CibouletteResourceResponseIdentifier<'request> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_cmp = self.type_.cmp(other.type_());
        match type_cmp {
            Ordering::Equal => Some(self.id.cmp(other.id())),
            _ => Some(type_cmp),
        }
    }
}

impl<'request> PartialEq for CibouletteResourceResponseIdentifier<'request> {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.id == other.id
    }
}

impl<'request> Eq for CibouletteResourceResponseIdentifier<'request> {}

impl<'request> From<CibouletteResourceIdentifierBuilder<'request>>
    for CibouletteResourceResponseIdentifierBuilder<'request>
{
    fn from(other: CibouletteResourceIdentifierBuilder<'request>) -> Self {
        CibouletteResourceResponseIdentifierBuilder::new(other.id, other.type_)
    }
}
impl<'request> CibouletteResourceResponseIdentifierBuilder<'request> {
    pub fn build(
        self,
        store: &CibouletteStore,
    ) -> Result<CibouletteResourceResponseIdentifier<'request>, CibouletteError> {
        let type_ = store.get_type(&self.type_)?;
        Ok(CibouletteResourceResponseIdentifier {
            id: match self.id {
                Some(id) => id.build(type_.id_type())?,
                None => return Err(CibouletteError::MissingId),
            },
            type_: type_.name().clone(),
            rel: None,
        })
    }

    pub fn build_relationships(
        self,
        store: &CibouletteStore,
        main_type: &CibouletteResourceType,
    ) -> Result<CibouletteResourceResponseIdentifier<'request>, CibouletteError> {
        let (rel_alias, rel) = main_type.get_relationship_with_alias(store, &self.type_)?;
        let id_type = rel.id_type();
        Ok(CibouletteResourceResponseIdentifier {
            id: match self.id {
                Some(id) => id.build(id_type)?,
                None => return Err(CibouletteError::MissingId),
            },
            type_: rel.name().clone(),
            rel: Some(rel_alias),
        })
    }
}

impl<'request> CibouletteResourceResponseIdentifierBuilder<'request> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<CibouletteIdBuilder<'request>>, type_: Cow<'request, str>) -> Self {
        CibouletteResourceResponseIdentifierBuilder { id, type_ }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteResourceResponseIdentifierSelectorBuilder<'request> {
    One(CibouletteResourceResponseIdentifierBuilder<'request>),
    Many(Vec<CibouletteResourceResponseIdentifierBuilder<'request>>),
}

impl<'request> CibouletteResourceResponseIdentifierSelectorBuilder<'request> {
    pub fn build(
        self,
        store: &CibouletteStore,
    ) -> Result<CibouletteResourceResponseIdentifierSelector<'request>, CibouletteError> {
        match self {
            CibouletteResourceResponseIdentifierSelectorBuilder::One(x) => Ok(
                CibouletteResourceResponseIdentifierSelector::One(x.build(store)?),
            ),
            CibouletteResourceResponseIdentifierSelectorBuilder::Many(ids) => {
                let mut res: Vec<CibouletteResourceResponseIdentifier<'request>> =
                    Vec::with_capacity(ids.len());

                for id in ids.into_iter() {
                    res.push(id.build(store)?);
                }
                Ok(CibouletteResourceResponseIdentifierSelector::Many(res))
            }
        }
    }
}

/// ## A selector between a single or multiple `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) objects
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteResourceResponseIdentifierSelector<'request> {
    One(CibouletteResourceResponseIdentifier<'request>),
    Many(Vec<CibouletteResourceResponseIdentifier<'request>>),
}

impl<'request, B>
    From<CibouletteResource<'request, B, CibouletteResourceResponseIdentifier<'request>>>
    for CibouletteResourceResponseIdentifierSelector<'request>
{
    fn from(
        obj: CibouletteResource<'request, B, CibouletteResourceResponseIdentifier<'request>>,
    ) -> Self {
        CibouletteResourceResponseIdentifierSelector::One(obj.identifier)
    }
}

impl<'request, B>
    From<CibouletteResourceSelector<'request, B, CibouletteResourceResponseIdentifier<'request>>>
    for CibouletteResourceResponseIdentifierSelector<'request>
{
    fn from(
        obj: CibouletteResourceSelector<
            'request,
            B,
            CibouletteResourceResponseIdentifier<'request>,
        >,
    ) -> Self {
        match obj {
            CibouletteResourceSelector::One(x) => {
                CibouletteResourceResponseIdentifierSelector::One(x.identifier)
            }
            CibouletteResourceSelector::Many(x) => {
                CibouletteResourceResponseIdentifierSelector::Many(
                    x.into_iter().map(|x| x.identifier).collect(),
                )
            }
        }
    }
}
