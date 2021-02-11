use super::*;
use serde::de::{DeserializeSeed, Deserializer, Visitor};
use std::fmt::Formatter;
const CIBOULETTE_QUERY_PARAMETERS_FIELDS: &[&str] =
    &["include", "fields[*]", "sorting", "page", "filter"];

#[derive(Clone, Copy, Debug)]
pub struct CibouletteQueryParametersBuilderVisitor;
pub struct CibouletteQueryParametersFieldVisitor;

pub enum CibouletteQueryParametersField<'a> {
    Include,
    Sparse(Vec<&'a str>),
    Sorting,
    Page(CiboulettePageType<'a>),
    Filter,
    FilterTyped(Cow<'a, str>),
    Meta(Cow<'a, str>),
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum CiboulettePageType<'a> {
    Number,
    Size,
    Offset,
    Limit,
    Cursor,
    Other(Cow<'a, str>),
}

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
    include: Option<Vec<Vec<Cow<'a, str>>>>,
    sparse: BTreeMap<Vec<&'a str>, Vec<&'a str>>,
    sorting: Vec<(CibouletteSortingDirection, Vec<&'a str>)>,
    page: BTreeMap<CiboulettePageType<'a>, Cow<'a, str>>,
    filter: Option<Cow<'a, str>>,
    filter_typed: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    meta: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteQueryParameters<'a> {
    include: Option<Vec<Vec<Cow<'a, str>>>>,
    sparse: BTreeMap<&'a CibouletteResourceType, Vec<&'a str>>,
    sorting: Vec<CibouletteSortingElement<'a>>,
    page: BTreeMap<CiboulettePageType<'a>, Cow<'a, str>>,
    filter: Option<Cow<'a, str>>,
    filter_typed: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    meta: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'de> Visitor<'de> for CibouletteQueryParametersFieldVisitor {
    type Value = CibouletteQueryParametersField<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "field identifier")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let preemptive_val = match value {
            "include" => Some(CibouletteQueryParametersField::Include),
            "sort" => Some(CibouletteQueryParametersField::Sorting),
            "filter" => Some(CibouletteQueryParametersField::Filter),
            _ => None,
        };
        if let Some(preemptive_val) = preemptive_val {
            return Ok(preemptive_val);
        }
        let has_type = value.find('[');
        match has_type {
            Some(type_end_index) => {
                let type_ = &value[0..type_end_index];
                match type_ {
                    "page" => {
                        let mut page_type_vec =
                            typed_param::parse_typed_query_param(&value).unwrap_or_default();
                        let page_type: Cow<'de, str> = match page_type_vec.len() {
                            0 => Cow::Borrowed(""),
                            1 => Cow::Borrowed(page_type_vec.pop().unwrap()),
                            _ => Cow::Owned(
                                page_type_vec.join("."), // FIXME Try not to allocate more
                            ),
                        };
                        match page_type.as_ref() {
                            "limit" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Limit,
                            )),
                            "size" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Size,
                            )),
                            "offset" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Offset,
                            )),
                            "number" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Number,
                            )),
                            "cursor" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Cursor,
                            )),
                            _ => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Other(page_type),
                            )),
                        }
                    }
                    "fields" => Ok(CibouletteQueryParametersField::Sparse(
                        typed_param::parse_typed_query_param(value).unwrap_or_default(),
                    )),
                    "filter" => {
                        let mut type_vec =
                            typed_param::parse_typed_query_param(value).unwrap_or_default();
                        let type_ = match type_vec.len() {
                            0 => Cow::Borrowed(""),
                            1 => Cow::Borrowed(type_vec.pop().unwrap()),
                            _ => Cow::Owned(
                                type_vec.join("."), // FIXME Try not to allocate more
                            ),
                        };
                        Ok(CibouletteQueryParametersField::FilterTyped(type_))
                    }
                    _ => Ok(CibouletteQueryParametersField::Meta(Cow::Borrowed(value))),
                }
            }
            None => Ok(CibouletteQueryParametersField::Meta(Cow::Borrowed(value))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for CibouletteQueryParametersField<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserializer::deserialize_identifier(
            deserializer,
            CibouletteQueryParametersFieldVisitor,
        )
    }
}

impl<'de> serde::de::Visitor<'de> for CibouletteQueryParametersBuilderVisitor {
    type Value = CibouletteQueryParametersBuilder<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResource")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut sparse: BTreeMap<Vec<&'de str>, Vec<&'de str>> = BTreeMap::new();
        let mut sorting: Vec<(CibouletteSortingDirection, Vec<&'de str>)> = Vec::new();
        let mut page: BTreeMap<CiboulettePageType<'de>, Cow<'de, str>> = BTreeMap::new();
        let mut filter_typed: BTreeMap<Cow<'de, str>, Cow<'de, str>> = BTreeMap::new();
        let mut meta: Vec<(Cow<'de, str>, Cow<'de, str>)> = Vec::new();
        let mut include: Option<Vec<Vec<Cow<'de, str>>>> = None;
        let mut filter: Option<Cow<'de, str>> = None;

        while let Some(key) =
            match serde::de::MapAccess::next_key::<CibouletteQueryParametersField>(&mut map) {
                Ok(val) => val,
                Err(err) => {
                    return Err(err);
                }
            }
        {
            match key {
                CibouletteQueryParametersField::Include => {
                    super::handle_ident_in_map_stateless(&mut include, &mut map, "include")?
                }
                CibouletteQueryParametersField::Sparse(type_) => {
                    if sparse
                        .insert(
                            type_,
                            explode_by_comma(&serde::de::MapAccess::next_value::<&'de str>(
                                &mut map,
                            )?),
                        )
                        .is_some()
                    {
                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                            "fields[<type>]",
                        ));
                    }
                }
                CibouletteQueryParametersField::Sorting => {
                    sorting = super::sorting::parse_sorting(serde::de::MapAccess::next_value::<
                        &'de str,
                    >(&mut map)?);
                }
                CibouletteQueryParametersField::Page(type_) => {
                    if page
                        .insert(
                            type_,
                            serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map)?,
                        )
                        .is_some()
                    {
                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                            "page[<type>]",
                        ));
                    }
                }
                CibouletteQueryParametersField::Filter => {
                    super::handle_ident_in_map_stateless(&mut filter, &mut map, "filter")?
                }
                CibouletteQueryParametersField::FilterTyped(type_) => {
                    if filter_typed
                        .insert(
                            type_,
                            serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map)?,
                        )
                        .is_some()
                    {
                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                            "filter[<type>]",
                        ));
                    }
                }
                CibouletteQueryParametersField::Meta(key) => {
                    meta.push((
                        key,
                        serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map)?,
                    ));
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

        Ok(CibouletteQueryParametersBuilder {
            include,
            sparse,
            filter,
            filter_typed,
            page,
            sorting,
            meta,
        })
    }
}

impl<'de> DeserializeSeed<'de> for CibouletteQueryParametersBuilderVisitor {
    type Value = CibouletteQueryParametersBuilder<'de>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "CibouletteQueryParametersField",
            CIBOULETTE_QUERY_PARAMETERS_FIELDS,
            CibouletteQueryParametersBuilderVisitor,
        )
    }
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

            if !wtype.relationships().contains(*type_) {
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
            include: self.include,
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

// impl<'a> CibouletteQueryParameters<'a> {
// 	/// Check a [CibouletteQueryParametersBuilder](CibouletteQueryParametersBuilder)
// 	pub fn check(&self, bag: &'a CibouletteBag) -> Result<(), CibouletteError> {
// 		for (types, fields) in self.sparse.iter() {
// 			let rel = Self::check_relationship_exists(bag, types)?;
// 			let field = Self::check_field_exists(rel, fields)?;
// 		}
// 		Ok(())
// 	}
// }
