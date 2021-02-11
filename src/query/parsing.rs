use super::*;
use serde::de::{DeserializeSeed, Deserializer};

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteSortingElement<'a> {
    type_: &'a CibouletteResourceType,
    direction: CibouletteSortingDirection,
    field: Vec<&'a str>,
}

impl<'a> CibouletteSortingElement<'a> {
    pub fn new(
        type_: &'a CibouletteResourceType,
        direction: CibouletteSortingDirection,
        field: Vec<&'a str>,
    ) -> Self {
        CibouletteSortingElement {
            type_,
            direction,
            field,
        }
    }
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteQueryParametersBuilder<'a> {
    pub(super) include: Option<Vec<Vec<&'a str>>>,
    pub(super) sparse: BTreeMap<Vec<&'a str>, Vec<&'a str>>,
    pub(super) sorting: Vec<(CibouletteSortingDirection, Vec<&'a str>)>,
    pub(super) page: BTreeMap<CiboulettePageType<'a>, Cow<'a, str>>,
    pub(super) filter: Option<Cow<'a, str>>,
    pub(super) filter_typed: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    pub(super) meta: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteQueryParameters<'a> {
    include: Option<Vec<&'a CibouletteResourceType>>,
    sparse: BTreeMap<&'a CibouletteResourceType, Vec<&'a str>>,
    sorting: Vec<CibouletteSortingElement<'a>>,
    page: BTreeMap<CiboulettePageType<'a>, Cow<'a, str>>,
    filter: Option<Cow<'a, str>>,
    filter_typed: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    meta: Vec<(Cow<'a, str>, Cow<'a, str>)>,
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
    fn check_relationship_exists(
        bag: &'a CibouletteBag,
        type_list: &[&'a str],
    ) -> Result<&'a CibouletteResourceType, CibouletteError> {
        let mut wtype: &CibouletteResourceType;
        let mut types_iter = type_list.iter();

        {
            let type_ = types_iter
                .next()
                .ok_or_else(|| CibouletteError::UnknownType("<empty>".to_string()))?;
            wtype = bag
                .map()
                .get(*type_)
                .ok_or_else(|| CibouletteError::UnknownType(type_.to_string()))?;
        }
        for type_ in types_iter {
            let curr_type = bag
                .map()
                .get(*type_)
                .ok_or_else(|| CibouletteError::UnknownType(type_.to_string()))?;

            if !wtype.relationships().contains_key(*type_) {
                return Err(CibouletteError::UnknownRelationship(
                    wtype.name().clone(),
                    type_.to_string(),
                ));
            }
            wtype = curr_type;
        }
        Ok(wtype)
    }

    fn check_field_exists(
        type_: &'a CibouletteResourceType,
        field_list: &[&'a str],
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
                    false => return Ok(()),
                    true => {
                        return Err(CibouletteError::UnknownField(
                            type_.name().clone(),
                            field.to_string(),
                        ))
                    }
                },
                MessyJson::Obj(obj) => {
                    curr_obj = obj.properties().get(*field).ok_or_else(|| {
                        CibouletteError::UnknownField(type_.name().clone(), "<empty>".to_string())
                    })?;
                    continue;
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
        main_type: &'a CibouletteResourceType,
    ) -> Result<CibouletteQueryParameters<'a>, CibouletteError> {
        let mut sparse: BTreeMap<&'a CibouletteResourceType, Vec<&'a str>> = BTreeMap::new();
        let mut sorting: Vec<CibouletteSortingElement> = Vec::new();
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
        for (types, fields) in self.sparse.into_iter() {
            let rel = Self::check_relationship_exists(bag, types.as_slice())?;
            Self::check_field_exists(rel, fields.as_slice())?;
            sparse.insert(rel, fields);
        }

        for (direction, fields_and_rel) in self.sorting.into_iter() {
            let mut fields = &fields_and_rel[..];
            match fields_and_rel.len() {
                0 => return Err(CibouletteError::UnknownType("<empty>".to_string())),
                1 => {
                    let rel = Self::check_relationship_exists(bag, fields_and_rel.as_slice())?;

                    sorting.push(CibouletteSortingElement::new(rel, direction, vec![]));
                }
                _ => {
                    let rel = match Self::check_relationship_exists(
                        bag,
                        &fields_and_rel.as_slice()[0..1],
                    ) {
                        Ok(x) => {
                            fields = &fields_and_rel[1..fields_and_rel.len()];
                            x
                        }
                        Err(_) => main_type,
                    };
                    Self::check_field_exists(rel, fields)?;
                    sorting.push(CibouletteSortingElement::new(
                        rel,
                        direction,
                        fields.to_vec(),
                    ));
                }
            }
        }

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
