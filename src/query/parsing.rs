use super::*;
use serde::de::{DeserializeSeed, Deserializer};
/// ## Element of a sorting vector.
#[derive(Debug, Getters, Clone, Ord, PartialEq, Eq, PartialOrd)]
#[getset(get = "pub")]
pub struct CibouletteSortingElement {
    pub rel_chain: Vec<CibouletteResourceRelationshipDetails>,
    pub direction: CibouletteSortingDirection,
    pub field: ArcStr,
}

impl CibouletteSortingElement {
    /// Create a new sorting element
    pub fn new(
        rel_chain: Vec<CibouletteResourceRelationshipDetails>,
        direction: CibouletteSortingDirection,
        field: ArcStr,
    ) -> Self {
        CibouletteSortingElement {
            rel_chain,
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
    pub(super) sparse: BTreeMap<Cow<'request, str>, Vec<Cow<'request, str>>>,
    pub(super) sorting: Vec<(CibouletteSortingDirection, Cow<'request, str>)>,
    pub(super) page: BTreeMap<CiboulettePageType<'request>, Cow<'request, str>>,
    pub(super) filter: Option<Cow<'request, str>>,
    pub(super) filter_typed: BTreeMap<Cow<'request, str>, Cow<'request, str>>,
    pub(super) meta: BTreeMap<Cow<'request, str>, Cow<'request, str>>,
}

/// ## Query parameters for `json:api`
#[derive(Debug, Getters, Default, Clone)]
#[getset(get = "pub")]
pub struct CibouletteQueryParameters<'request> {
    pub include: Vec<Vec<CibouletteResourceRelationshipDetails>>,
    pub sparse: BTreeMap<Arc<CibouletteResourceType>, Vec<ArcStr>>,
    pub sorting: Vec<CibouletteSortingElement>,
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
    pub(super) fn check_relationship_exists(
        store: &CibouletteStore,
        main_type: &Arc<CibouletteResourceType>,
        rel_list: &[Cow<'request, str>],
    ) -> Result<Vec<CibouletteResourceRelationshipDetails>, CibouletteError> {
        let mut current_type = main_type.clone();
        let mut res: Vec<CibouletteResourceRelationshipDetails> = Vec::new();
        let mut first = true;
        for rel in rel_list {
            if first {
                first = false;
                if current_type.name().as_str() == rel.as_ref() {
                    continue;
                }
            }
            let tmp = current_type.get_relationship_details(store, rel)?.clone();
            current_type = tmp.related_type().clone();
            res.push(tmp);
        }
        if res.is_empty() {
            return Err(CibouletteError::UnknownRelationship(
                main_type.name().to_string(),
                "<empty>".to_string(),
            ));
        }
        Ok(res)
    }

    /// Checks that a field exists in a give resource type
    #[inline]
    pub(super) fn check_field_exists(
        type_: &Arc<CibouletteResourceType>,
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
    pub(super) fn check_fields_exists(
        type_: &CibouletteResourceType,
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

    /// Build a [CibouletteQueryParametersBuilder](CibouletteQueryParametersBuilder) from the builder
    pub fn build(
        self,
        bag: &CibouletteStore,
        main_type: Arc<CibouletteResourceType>,
    ) -> Result<CibouletteQueryParameters<'request>, CibouletteError> {
        let mut sparse: BTreeMap<Arc<CibouletteResourceType>, Vec<ArcStr>> = BTreeMap::new();
        let mut sorting: Vec<CibouletteSortingElement> = Vec::with_capacity(self.sorting.len());

        // Check for include relationships and build the array
        let include: Vec<Vec<CibouletteResourceRelationshipDetails>> = match self.include {
            None => Vec::default(),
            Some(include) => {
                let mut res: Vec<Vec<CibouletteResourceRelationshipDetails>> = Vec::new();
                for types in include.into_iter() {
                    res.push(Self::check_relationship_exists(
                        bag,
                        &main_type,
                        types.as_slice(),
                    )?);
                }
                res
            }
        };

        // Check for sparse fields, checking that fields exists
        for (type_, fields) in self.sparse.into_iter() {
            let rel = bag.get_type(type_.as_ref())?;
            let fields = match fields.is_empty() {
                true => vec![],
                false => Self::check_fields_exists(&rel, fields)?,
            };
            sparse.insert(rel.clone(), fields);
        }

        // Check for the sort fields, checking fields exists
        if !self.sorting.is_empty() {
            for (direction, field) in self.sorting.into_iter() {
                sorting.push(sorting::extract_type(
                    &bag,
                    main_type.clone(),
                    direction,
                    field,
                )?)
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
