use super::*;
use serde::de::{DeserializeSeed, Deserializer};

/// ## Element of a sorting vector.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteSortingElement<'a> {
    type_: &'a CibouletteResourceType,
    direction: CibouletteSortingDirection,
    field: Cow<'a, str>,
}

impl<'a> CibouletteSortingElement<'a> {
    /// Create a new sorting element
    pub fn new(
        type_: &'a CibouletteResourceType,
        direction: CibouletteSortingDirection,
        field: Cow<'a, str>,
    ) -> Self {
        CibouletteSortingElement {
            type_,
            direction,
            field,
        }
    }
}

/// ## Builder object for [CibouletteQueryParameters](CibouletteQueryParameters)
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteQueryParametersBuilder<'a> {
    pub(super) include: Option<Vec<Vec<Cow<'a, str>>>>,
    pub(super) sparse: BTreeMap<Vec<Cow<'a, str>>, Vec<Cow<'a, str>>>,
    pub(super) sorting: Vec<(CibouletteSortingDirection, Cow<'a, str>)>,
    pub(super) page: BTreeMap<CiboulettePageType<'a>, Cow<'a, str>>,
    pub(super) filter: Option<Cow<'a, str>>,
    pub(super) filter_typed: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    pub(super) meta: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
}

/// ## Query parameters for `json:api`
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteQueryParameters<'a> {
    include: Option<Vec<&'a CibouletteResourceType>>,
    sparse: BTreeMap<&'a CibouletteResourceType, Vec<Cow<'a, str>>>,
    sorting: Vec<CibouletteSortingElement<'a>>,
    page: BTreeMap<CiboulettePageType<'a>, Cow<'a, str>>,
    filter: Option<Cow<'a, str>>,
    filter_typed: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    meta: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'de> Deserialize<'de> for CibouletteQueryParametersBuilder<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<CibouletteQueryParametersBuilder<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = CibouletteQueryParametersBuilderVisitor;

        visitor.deserialize(deserializer)
    }
}

impl<'a> CibouletteQueryParametersBuilder<'a> {
    /// Check that a relationship exists between a chain of types.
    ///
    /// i.e. "author.comments" makes sense because the author has comments
    ///
    /// but "comments.email" may not make sense
    /// if there is no relationship between those two resources.
    #[inline]
    fn check_relationship_exists(
        bag: &'a CibouletteBag,
        type_list: &[Cow<'a, str>],
    ) -> Result<&'a CibouletteResourceType, CibouletteError> {
        let mut wtype: &CibouletteResourceType;
        let mut types_iter = type_list.iter();

        {
            let type_ = types_iter
                .next()
                .ok_or_else(|| CibouletteError::UnknownType("<empty>".to_string()))?;
            wtype = bag
                .map()
                .get(type_.as_ref())
                .ok_or_else(|| CibouletteError::UnknownType(type_.to_string()))?;
        }
        for type_ in types_iter {
            let curr_type_name = match wtype.relationships().get(type_.as_ref()) {
                Some(name) => name,
                None => {
                    return Err(CibouletteError::UnknownRelationship(
                        wtype.name().clone(),
                        type_.to_string(),
                    ))
                }
            };
            let curr_type = bag.map().get(curr_type_name).ok_or_else(|| {
                CibouletteError::UnknownRelationship(wtype.name().clone(), type_.to_string())
            })?;
            wtype = curr_type;
        }
        Ok(wtype)
    }

    /// Checks that a field exists in a give resource type
    #[inline]
    fn check_field_exists(
        type_: &'a CibouletteResourceType,
        field: &str,
    ) -> Result<(), CibouletteError> {
        match type_.schema() {
            MessyJson::Obj(obj) => match obj.properties().contains_key(field) {
                true => Ok(()),
                false => Err(CibouletteError::UnknownField(
                    type_.name().clone(),
                    field.to_string(),
                )),
            },
            _ => Err(CibouletteError::UnknownField(
                type_.name().clone(),
                field.to_string(),
            )),
        }
    }

    /// Checks that fields exists in a give resource type
    #[inline]
    fn check_fields_exists(
        type_: &'a CibouletteResourceType,
        field_list: &[Cow<'a, str>],
    ) -> Result<(), CibouletteError> {
        let mut curr_obj: &MessyJson = type_.schema();
        let mut iter = field_list.iter().peekable();

        while let Some(field) = iter.next() {
            match curr_obj {
                MessyJson::Bool(_)
                | MessyJson::Null
                | MessyJson::Number(_)
                | MessyJson::String(_)
                | MessyJson::Array(_) => match iter.peek().is_some() {
                    // Cannot points to those types, so if there is more, it means that it's an error
                    false => return Ok(()),
                    true => {
                        return Err(CibouletteError::UnknownField(
                            type_.name().clone(),
                            field.to_string(),
                        ))
                    }
                },
                MessyJson::Obj(obj) => {
                    curr_obj = obj.properties().get(field.as_ref()).ok_or_else(|| {
                        CibouletteError::UnknownField(type_.name().clone(), field.to_string())
                    })?;
                    match iter.peek().is_some() {
                        true => continue,
                        false => return Ok(()),
                    }
                }
            }
        }
        match field_list.len() {
            0 => Err(CibouletteError::UnknownField(
                type_.name().clone(),
                "<empty>".to_string(),
            )),
            _ => Err(CibouletteError::UnknownField(
                type_.name().clone(),
                field_list.join("."),
            )),
        }
    }

    /// Build a [CibouletteQueryParametersBuilder](CibouletteQueryParametersBuilder) from the builder
    pub fn build(
        self,
        bag: &'a CibouletteBag,
        main_type: Option<&'a CibouletteResourceType>,
    ) -> Result<CibouletteQueryParameters<'a>, CibouletteError> {
        let mut sparse: BTreeMap<&'a CibouletteResourceType, Vec<Cow<'a, str>>> = BTreeMap::new();
        let mut sorting: Vec<CibouletteSortingElement> = Vec::with_capacity(self.sorting.len());

        // Check for include relationships and build the array
        let include: Option<Vec<&'a CibouletteResourceType>> = match self.include {
            None => None,
            Some(include) => {
                let mut res: Vec<&'a CibouletteResourceType> = Vec::with_capacity(include.len());
                for types in include.into_iter() {
                    res.push(Self::check_relationship_exists(bag, types.as_slice())?)
                }
                Some(res)
            }
        };

        // Check for sparse fields, checking that fields exists
        for (types, fields) in self.sparse.into_iter() {
            let rel = Self::check_relationship_exists(bag, types.as_slice())?;
            Self::check_fields_exists(rel, fields.as_slice())?;
            sparse.insert(rel, fields);
        }

        // Check for the sort fields, checking fields exists
        match (main_type, self.sorting.len()) {
            (_, 0) => (),
            (Some(main_type), _) => {
                for (direction, field) in self.sorting.into_iter() {
                    Self::check_field_exists(main_type, &field)?;
                    sorting.push(CibouletteSortingElement::new(main_type, direction, field))
                }
            }
            (None, _) => return Err(CibouletteError::IncompatibleSorting),
        };

        let res = CibouletteQueryParameters {
            include,
            page: self.page,
            meta: self.meta,
            filter: self.filter,
            filter_typed: self.filter_typed,
            sparse,
            sorting,
        };
        Ok(res)
    }
}
