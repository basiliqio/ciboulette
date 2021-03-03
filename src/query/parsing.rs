use super::*;
use serde::de::{DeserializeSeed, Deserializer};

/// ## Element of a sorting vector.
#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CibouletteSortingElement<'a> {
    pub type_: &'a CibouletteResourceType<'a>,
    pub direction: CibouletteSortingDirection,
    pub field: Cow<'a, str>,
}

impl<'a> CibouletteSortingElement<'a> {
    /// Create a new sorting element
    pub fn new(
        type_: &'a CibouletteResourceType<'a>,
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
#[derive(Debug, Getters, Default, Clone)]
#[getset(get = "pub")]
pub struct CibouletteQueryParameters<'a> {
    pub include: BTreeSet<&'a CibouletteResourceType<'a>>,
    pub sparse: BTreeMap<&'a CibouletteResourceType<'a>, Vec<Cow<'a, str>>>,
    pub sorting: Vec<CibouletteSortingElement<'a>>,
    pub page: BTreeMap<CiboulettePageType<'a>, Cow<'a, str>>,
    pub filter: Option<Cow<'a, str>>,
    pub filter_typed: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    pub meta: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
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
    // }
    /// Check that a relationship exists between a chain of types.
    ///
    /// i.e. "author.comments" makes sense because the author has comments
    ///
    /// but "comments.email" may not make sense
    /// if there is no relationship between those two resources.
    #[inline]
    fn check_relationship_exists(
        bag: &'a CibouletteStore<'a>,
        type_list: &[Cow<'a, str>],
    ) -> Result<&'a CibouletteResourceType<'a>, CibouletteError> {
        let mut wtype: (petgraph::graph::NodeIndex<u16>, &CibouletteResourceType);
        let mut types_iter = type_list.iter();

        let type_ = types_iter
            .next()
            .ok_or_else(|| CibouletteError::UnknownType("<empty>".to_string()))?;
        wtype = bag
            .get_type_with_index(type_.as_ref())
            .ok_or_else(|| CibouletteError::UnknownType(type_.to_string()))?;
        for type_ in types_iter {
            let rel_edge = match wtype.1.relationships().get(type_.as_ref()) {
                Some(i) => i,
                None => {
                    return Err(CibouletteError::UnknownRelationship(
                        wtype.1.name().clone(),
                        type_.to_string(),
                    ))
                }
            };
            let nodes = bag.graph().edge_endpoints(*rel_edge).ok_or_else(|| {
                CibouletteError::RelNotInGraph(wtype.1.name().clone(), type_.clone().into_owned())
            })?; // Get the nodes
            let next_node = match nodes.0 == wtype.0 {
                // Extract the next node
                true => nodes.1,
                false => nodes.0,
            };
            let curr_type = (
                next_node,
                bag.graph()
                    .node_weight(next_node)
                    .ok_or_else(|| CibouletteError::TypeNotInGraph(type_.clone().into_owned()))?,
            );
            wtype = curr_type;
        }
        Ok(wtype.1)
    }

    /// Checks that a field exists in a give resource type
    #[inline]
    fn check_field_exists(
        type_: &'a CibouletteResourceType<'a>,
        field: &str,
    ) -> Result<(), CibouletteError> {
        match type_.schema().properties().contains_key(field) {
            true => Ok(()),
            false => Err(CibouletteError::UnknownField(
                type_.name().clone(),
                field.to_string(),
            )),
        }
    }

    /// Checks that fields exists in a give resource type
    #[inline]
    fn check_fields_exists(
        type_: &'a CibouletteResourceType<'a>,
        field_list: &[Cow<'a, str>],
    ) -> Result<(), CibouletteError> {
        let curr_obj: &MessyJsonObject = type_.schema();
        let mut iter = field_list.iter().peekable();

        while let Some(field) = iter.next() {
            curr_obj.properties().get(field.as_ref()).ok_or_else(|| {
                CibouletteError::UnknownField(type_.name().clone(), field.to_string())
            })?;
            match iter.peek().is_some() {
                true => continue,
                false => return Ok(()),
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
        bag: &'a CibouletteStore<'a>,
        main_type: Option<&'a CibouletteResourceType<'a>>,
    ) -> Result<CibouletteQueryParameters<'a>, CibouletteError> {
        let mut sparse: BTreeMap<&'a CibouletteResourceType, Vec<Cow<'a, str>>> = BTreeMap::new();
        let mut sorting: Vec<CibouletteSortingElement> = Vec::with_capacity(self.sorting.len());

        // Check for include relationships and build the array
        let include: BTreeSet<&'a CibouletteResourceType> = match self.include {
            None => BTreeSet::default(),
            Some(include) => {
                let mut res: BTreeSet<&'a CibouletteResourceType> = BTreeSet::new();
                for types in include.into_iter() {
                    res.insert(Self::check_relationship_exists(bag, types.as_slice())?);
                }
                res
            }
        };

        // Check for sparse fields, checking that fields exists
        for (types, fields) in self.sparse.into_iter() {
            let rel = Self::check_relationship_exists(bag, types.as_slice())?;
            if !fields.is_empty() {
                Self::check_fields_exists(rel, fields.as_slice())?;
            }
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
