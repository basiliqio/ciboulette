use super::*;

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifier<'a> {
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    pub id: Cow<'a, str>,
    #[serde(default)]
    pub meta: Value,
}

/// ## A `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) object
#[derive(Deserialize, Serialize, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteResourceIdentifierPermissive<'a> {
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    pub id: Option<Cow<'a, str>>,
    #[serde(default)]
    pub meta: Value,
}

impl<'a> TryFrom<CibouletteResourceIdentifierPermissive<'a>> for CibouletteResourceIdentifier<'a> {
    type Error = CibouletteError;

    fn try_from(value: CibouletteResourceIdentifierPermissive<'a>) -> Result<Self, Self::Error> {
        let CibouletteResourceIdentifierPermissive { type_, id, meta } = value;

        Ok(CibouletteResourceIdentifier {
            type_,
            meta,
            id: id.ok_or(CibouletteError::MissingId)?,
        })
    }
}

impl<'a> From<CibouletteResourceIdentifier<'a>> for CibouletteResourceIdentifierPermissive<'a> {
    fn from(value: CibouletteResourceIdentifier<'a>) -> Self {
        let CibouletteResourceIdentifier { type_, id, meta } = value;

        CibouletteResourceIdentifierPermissive {
            type_,
            meta,
            id: Some(id),
        }
    }
}

impl<'a> CibouletteResourceIdentifier<'a> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Cow<'a, str>, type_: Cow<'a, str>, meta: Value) -> Self {
        CibouletteResourceIdentifier { id, type_, meta }
    }
}

impl<'a> CibouletteResourceIdentifierPermissive<'a> {
    /// Create a new resource identifier from an id, a type an potentially a meta argument
    pub fn new(id: Option<Cow<'a, str>>, type_: Cow<'a, str>, meta: Value) -> Self {
        CibouletteResourceIdentifierPermissive { id, type_, meta }
    }
}

/// ## A selector between a single or multiple `json:api` [resource identifier](https://jsonapi.org/format/#document-resource-identifier-objects) objects
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteResourceIdentifierSelector<'a> {
    One(CibouletteResourceIdentifier<'a>),
    Many(Vec<CibouletteResourceIdentifier<'a>>),
}

impl<'a> From<CibouletteResource<'a, CibouletteResourceIdentifier<'a>>>
    for CibouletteResourceIdentifierSelector<'a>
{
    fn from(obj: CibouletteResource<'a, CibouletteResourceIdentifier<'a>>) -> Self {
        CibouletteResourceIdentifierSelector::One(obj.identifier)
    }
}

impl<'a> TryFrom<CibouletteResource<'a, CibouletteResourceIdentifierPermissive<'a>>>
    for CibouletteResourceIdentifierSelector<'a>
{
    type Error = CibouletteError;

    fn try_from(
        obj: CibouletteResource<'a, CibouletteResourceIdentifierPermissive<'a>>,
    ) -> Result<Self, Self::Error> {
        Ok(CibouletteResourceIdentifierSelector::One(
            obj.identifier.try_into()?,
        ))
    }
}

impl<'a> From<CibouletteResourceSelector<'a, CibouletteResourceIdentifier<'a>>>
    for CibouletteResourceIdentifierSelector<'a>
{
    fn from(obj: CibouletteResourceSelector<'a, CibouletteResourceIdentifier<'a>>) -> Self {
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

impl<'a> TryFrom<CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>>
    for CibouletteResourceIdentifierSelector<'a>
{
    type Error = CibouletteError;

    fn try_from(
        obj: CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>,
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
