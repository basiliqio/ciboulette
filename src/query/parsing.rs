use super::*;
use itertools::Itertools;
use serde::de::{DeserializeSeed, Deserializer};
/// ## Element of a sorting vector.
#[derive(Debug, Getters, Clone, Hash)]
#[getset(get = "pub")]
pub struct CibouletteSortingElement<'store> {
    pub type_: Arc<CibouletteResourceType<'store>>,
    pub direction: CibouletteSortingDirection,
    pub field: ArcStr,
}

impl<'store> CibouletteSortingElement<'store> {
    /// Create a new sorting element
    pub fn new(
        type_: Arc<CibouletteResourceType<'store>>,
        direction: CibouletteSortingDirection,
        field: ArcStr,
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
pub struct CibouletteQueryParametersBuilder<'request> {
    pub(super) include: Option<Vec<Vec<Cow<'request, str>>>>,
    pub(super) sparse: BTreeMap<Vec<Cow<'request, str>>, Vec<Cow<'request, str>>>,
    pub(super) sorting: Vec<(CibouletteSortingDirection, Cow<'request, str>)>,
    pub(super) page: BTreeMap<CiboulettePageType<'request>, Cow<'request, str>>,
    pub(super) filter: Option<Cow<'request, str>>,
    pub(super) filter_typed: BTreeMap<Cow<'request, str>, Cow<'request, str>>,
    pub(super) meta: BTreeMap<Cow<'request, str>, Cow<'request, str>>,
}

/// ## Query parameters for `json:api`
#[derive(Debug, Getters, Default, Clone, Hash)]
#[getset(get = "pub")]
pub struct CibouletteQueryParameters<'request, 'store> {
    pub include: BTreeSet<&'store CibouletteResourceType<'store>>,
    pub sparse: BTreeMap<&'store CibouletteResourceType<'store>, Vec<ArcStr>>,
    pub sorting: Vec<CibouletteSortingElement<'store>>,
    pub sorting_map:
        BTreeMap<Arc<CibouletteResourceType<'store>>, Vec<CibouletteSortingElement<'store>>>,
    pub page: BTreeMap<CiboulettePageType<'request>, Cow<'request, str>>,
    pub filter: Option<Cow<'request, str>>,
    pub filter_typed: BTreeMap<Cow<'request, str>, Cow<'request, str>>,
    pub meta: BTreeMap<Cow<'request, str>, Cow<'request, str>>,
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

impl<'request> CibouletteQueryParametersBuilder<'request> {
    // }
    /// Check that a relationship exists between a chain of types.
    ///
    /// i.e. "author.comments" makes sense because the author has comments
    ///
    /// but "comments.email" may not make sense
    /// if there is no relationship between those two resources.
    #[inline]
    pub(super) fn check_relationship_exists<'store>(
        bag: &'store CibouletteStore<'store>,
        type_list: &[Cow<'request, str>],
    ) -> Result<&'store Arc<CibouletteResourceType<'store>>, CibouletteError> {
        let mut wtype: (
            petgraph::graph::NodeIndex<u16>,
            &Arc<CibouletteResourceType>,
        );
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
                        wtype.1.name().to_string(),
                        type_.to_string(),
                    ))
                }
            };
            let nodes = bag.graph().edge_endpoints(*rel_edge).ok_or_else(|| {
                CibouletteError::RelNotInGraph(
                    wtype.1.name().to_string(),
                    type_.clone().into_owned(),
                )
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
    pub(super) fn check_field_exists<'store>(
        type_: &Arc<CibouletteResourceType<'store>>,
        field: &str,
    ) -> Result<ArcStr, CibouletteError> {
        match type_.schema().properties().get_key_value(field) {
            Some((k, _)) => Ok(k.clone()),
            None => Err(CibouletteError::UnknownField(
                type_.name().to_string(),
                field.to_string(),
            )),
        }
    }

    /// Checks that fields exists in a give resource type
    #[inline]
    pub(super) fn check_fields_exists<'store>(
        type_: &'store CibouletteResourceType<'store>,
        field_list: Vec<Cow<'request, str>>,
    ) -> Result<Vec<ArcStr>, CibouletteError> {
        let curr_obj: &MessyJsonObject = type_.schema();
        let mut res: Vec<ArcStr> = Vec::with_capacity(field_list.len());
        let mut iter = field_list.iter().peekable();

        while let Some(field) = iter.next() {
            let (k, _) = curr_obj
                .properties()
                .get_key_value(field.as_ref())
                .ok_or_else(|| {
                    CibouletteError::UnknownField(type_.name().to_string(), field.to_string())
                })?;
            res.push(k.clone());
            match iter.peek().is_some() {
                true => continue,
                false => return Ok(res),
            }
        }
        match field_list.len() {
            0 => Err(CibouletteError::UnknownField(
                type_.name().to_string(),
                "<empty>".to_string(),
            )),
            _ => Err(CibouletteError::UnknownField(
                type_.name().to_string(),
                field_list.join("."),
            )),
        }
    }

    /// Extract a sorting map from a list of sorting elements
    fn extract_sorting_map<'store>(
        #[allow(clippy::ptr_arg)] sorting: &Vec<CibouletteSortingElement<'store>>,
    ) -> BTreeMap<Arc<CibouletteResourceType<'store>>, Vec<CibouletteSortingElement<'store>>> {
        match sorting.len() {
            0 => BTreeMap::default(),
            _ => {
                let mut sorting_map: BTreeMap<
                    Arc<CibouletteResourceType<'store>>,
                    Vec<CibouletteSortingElement<'store>>,
                > = BTreeMap::new();

                for (k, v) in sorting
                    .clone()
                    .into_iter()
                    .group_by(|x| x.type_().clone())
                    .into_iter()
                {
                    let insert_res = sorting_map.insert(k.clone(), v.into_iter().collect());
                    if let Some(mut old_el) = insert_res {
                        if let Some(new_el) = sorting_map.get_mut(&k) {
                            new_el.append(&mut old_el);
                        }
                    }
                }
                sorting_map
            }
        }
    }

    /// Build a [CibouletteQueryParametersBuilder](CibouletteQueryParametersBuilder) from the builder
    pub fn build<'store>(
        self,
        bag: &'store CibouletteStore<'store>,
        main_type: Option<Arc<CibouletteResourceType<'store>>>,
    ) -> Result<CibouletteQueryParameters<'request, 'store>, CibouletteError> {
        let mut sparse: BTreeMap<&'store CibouletteResourceType<'store>, Vec<ArcStr>> =
            BTreeMap::new();
        let mut sorting: Vec<CibouletteSortingElement<'store>> =
            Vec::with_capacity(self.sorting.len());

        // Check for include relationships and build the array
        let include: BTreeSet<&'store CibouletteResourceType<'store>> = match self.include {
            None => BTreeSet::default(),
            Some(include) => {
                let mut res: BTreeSet<&'store CibouletteResourceType<'store>> = BTreeSet::new();
                for types in include.into_iter() {
                    res.insert(Self::check_relationship_exists(bag, types.as_slice())?);
                }
                res
            }
        };

        // Check for sparse fields, checking that fields exists
        for (types, fields) in self.sparse.into_iter() {
            let rel = Self::check_relationship_exists(bag, types.as_slice())?;
            let fields = match fields.is_empty() {
                true => vec![],
                false => Self::check_fields_exists(&rel, fields)?,
            };
            sparse.insert(rel, fields);
        }

        // Check for the sort fields, checking fields exists
        match (main_type, self.sorting.len()) {
            (_, 0) => (),
            (Some(main_type), _) => {
                for (direction, field) in self.sorting.into_iter() {
                    sorting.push(sorting::extract_type(
                        &bag,
                        main_type.clone(),
                        direction,
                        field,
                    )?)
                }
            }
            (None, _) => return Err(CibouletteError::IncompatibleSorting),
        };

        let sorting_map = Self::extract_sorting_map(&sorting);
        let res = CibouletteQueryParameters {
            include,
            page: self.page,
            meta: self.meta,
            filter: self.filter,
            filter_typed: self.filter_typed,
            sparse,
            sorting_map,
            sorting,
        };
        Ok(res)
    }
}
