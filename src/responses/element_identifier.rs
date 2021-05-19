use super::*;
use element::CibouletteResponseElementAlias;
use serde::ser::SerializeStruct;
use std::cmp::{Ord, Ordering};

/// ## Builder for [CibouletteResourceResponseIdentifier](CibouletteResourceResponseIdentifier)
#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceResponseIdentifierBuilder<'request> {
    #[serde(rename = "type")]
    pub type_: Cow<'request, str>,
    pub id: Option<Cow<'request, str>>,
}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Debug, Getters, MutGetters, Clone, Hash)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceResponseIdentifier<'request> {
    pub type_: ArcStr,
    pub id: CibouletteIdSelector<'request>,
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
    /// Build into a [CibouletteResourceResponseIdentifier](CibouletteResourceResponseIdentifier)
    pub fn build(
        self,
        store: &CibouletteStore,
    ) -> Result<CibouletteResourceResponseIdentifier<'request>, CibouletteError> {
        let type_ = store.get_type(&self.type_)?;
        Ok(CibouletteResourceResponseIdentifier {
            id: match self.id {
                Some(id) => CibouletteIdSelector::build_id(type_.ids(), id)?,
                None => return Err(CibouletteError::MissingId),
            },
            type_: type_.name().clone(),
        })
    }

    /// Build a rel chain into a relationships metadata list and return the related type id type
    fn build_rel_chain(
        store: &CibouletteStore,
        base_type: Arc<CibouletteResourceType>,
        rel_chain: Cow<'request, str>,
    ) -> Result<
        (
            Vec<CibouletteResourceRelationshipDetails>,
            CibouletteIdTypeSelector,
        ),
        CibouletteError,
    > {
        let mut wtype: Arc<CibouletteResourceType> = base_type.clone();
        let mut res: Vec<CibouletteResourceRelationshipDetails> = Vec::new();
        let mut last_id_type = base_type.ids();
        for rel_name in rel_chain.split('.') {
            let rel_details = wtype.get_relationship_details(store, rel_name)?;

            wtype = rel_details.related_type().clone();
            last_id_type = wtype.ids();
            res.push(rel_details);
        }
        Ok((res, last_id_type.clone()))
    }

    /// Build a resource identifier where the type if a relationships alias of the `base_type`
    pub fn build_relationships(
        self,
        store: &CibouletteStore,
        base_type: Arc<CibouletteResourceType>,
    ) -> Result<CibouletteResponseElementAlias<'request>, CibouletteError> {
        let (rel_chain, id_type) = Self::build_rel_chain(store, base_type.clone(), self.type_)?;
        let last_type = rel_chain
            .last()
            .map(|x| x.related_type().clone())
            .unwrap_or_else(|| base_type.clone());
        Ok(CibouletteResponseElementAlias::new(
            rel_chain,
            CibouletteResourceResponseIdentifier {
                type_: last_type.name().clone(),
                id: CibouletteIdSelector::build_id(
                    &id_type,
                    self.id.ok_or(CibouletteError::MissingId)?,
                )?,
            },
        ))
    }
}

impl<'request> CibouletteResourceResponseIdentifierBuilder<'request> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<Cow<'request, str>>, type_: Cow<'request, str>) -> Self {
        CibouletteResourceResponseIdentifierBuilder { type_, id }
    }
}

/// Selector for [CibouletteResourceResponseIdentifierSelectorBuilder](CibouletteResourceResponseIdentifierSelectorBuilder)
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
// TODO custom deserialize
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
// TODO custom deserialize
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
        match obj.take() {
            CibouletteSelector::Single(x) => {
                CibouletteResourceResponseIdentifierSelector::One(x.identifier)
            }
            CibouletteSelector::Multi(x) => CibouletteResourceResponseIdentifierSelector::Many(
                x.into_iter().map(|x| x.identifier).collect(),
            ),
        }
    }
}

impl<'request> Serialize for CibouletteResourceResponseIdentifier<'request> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CibouletteResourceIdentifier", 3)?;
        state.serialize_field("type", &self.type_)?;
        match self.id().len() {
            0 => state.skip_field("id")?,
            1 => state.serialize_field(
                "id",
                self.id().get(0).map_err(|_| {
                    serde::ser::Error::custom(
                        "Wrong number of id when deserializing resource identifier",
                    )
                })?,
            )?,
            _ => state.serialize_field("id", &self.id().to_string())?,
        };
        state.end()
    }
}
