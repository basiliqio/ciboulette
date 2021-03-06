use super::*;
use serde::de::{Deserializer, Visitor};
use std::fmt::Formatter;
/// ## Object holder `json:api` version
#[derive(Debug, Clone, Getters, MutGetters, Deserialize, Serialize)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteJsonApiVersion<'request> {
    version: Cow<'request, str>,
}

impl<'request> CibouletteJsonApiVersion<'request> {
    pub fn new(version: Cow<'request, str>) -> CibouletteJsonApiVersion<'request> {
        CibouletteJsonApiVersion { version }
    }
}

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteBodyBuilder<'request> {
    /// The semver of the `JSON:API` specs
    jsonapi: Option<CibouletteJsonApiVersion<'request>>, // TODO Semver
    /// The data of the request/response. Cannot be set with `errors`.
    data: CibouletteBodyDataBuilder<'request>,
    /// The error object of the response. Cannot be set with `data`.
    errors: Option<CibouletteErrorObj<'request>>,
    meta: Option<Value>,
    /// Links to the current object/relationship
    links: Option<CibouletteBodyLink<'request>>,
    /// The included objects. Cannot be set without `data`
    included: Vec<CibouletteResourceBuilder<'request>>,
}

/// ## A `json:api` [document](https://jsonapi.org/format/#document-top-level) object
///
/// This struct hold the top level document of a request or a response
///
/// Use [CibouletteBodyBuilder](CibouletteBodyBuilder) to be build.
#[derive(Debug, Getters, MutGetters, Clone, Serialize)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteBody<'request, I, B> {
    /// The semver of the `JSON:API` specs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<CibouletteJsonApiVersion<'request>>, // TODO Semver
    /// Links to the current object/relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CibouletteBodyLink<'request>>,
    /// The data of the request/response. Cannot be set with `errors`.
    #[serde(skip_serializing_if = "CibouletteOptionalData::is_absent")]
    pub data: CibouletteBodyData<'request, I, B>,
    /// The error object of the response. Cannot be set with `data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<CibouletteErrorObj<'request>>,
    /// The meta object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    /// The included objects. Cannot be set without `data`
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub included: Vec<CibouletteResource<'request, B, I>>,
}

impl<'request, I, B> Default for CibouletteBody<'request, I, B>
where
    B: Default,
{
    fn default() -> Self {
        CibouletteBody {
            data: CibouletteBodyData::default(),
            errors: Option::default(),
            meta: Option::default(),
            links: Option::default(),
            included: Vec::default(),
            jsonapi: Option::default(),
        }
    }
}

/// The possible top level field, as described in the `JSON:API` specification.
const CIBOULETTE_TOP_LEVEL_FIELDS: &[&str] =
    &["data", "errors", "meta", "links", "included", "jsonapi"];

/// The visitor for the body builder
#[derive(Clone, Debug)]
struct CibouletteBodyBuilderVisitor;

enum CibouletteBodyField {
    Data,
    Errors,
    Meta,
    Links,
    Included,
    Jsonapi,
    Ignore,
}

struct CibouletteBodyFieldVisitor;
impl<'de> Visitor<'de> for CibouletteBodyFieldVisitor {
    type Value = CibouletteBodyField;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "field identifier")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // Match the field key
        match value {
            "data" => Ok(CibouletteBodyField::Data),
            "errors" => Ok(CibouletteBodyField::Errors),
            "meta" => Ok(CibouletteBodyField::Meta),
            "links" => Ok(CibouletteBodyField::Links),
            "included" => Ok(CibouletteBodyField::Included),
            "jsonapi" => Ok(CibouletteBodyField::Jsonapi),
            _ => Ok(CibouletteBodyField::Ignore),
        }
    }

    #[inline]
    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // Match the field key
        match value {
            b"data" => Ok(CibouletteBodyField::Data),
            b"errors" => Ok(CibouletteBodyField::Errors),
            b"meta" => Ok(CibouletteBodyField::Meta),
            b"links" => Ok(CibouletteBodyField::Links),
            b"included" => Ok(CibouletteBodyField::Included),
            b"jsonapi" => Ok(CibouletteBodyField::Jsonapi),
            _ => Ok(CibouletteBodyField::Ignore),
        }
    }
}
impl<'de> serde::Deserialize<'de> for CibouletteBodyField {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserializer::deserialize_identifier(deserializer, CibouletteBodyFieldVisitor)
    }
}
impl<'de> serde::de::Visitor<'de> for CibouletteBodyBuilderVisitor {
    type Value = CibouletteBodyBuilder<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResource")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut data: Option<CibouletteBodyDataBuilder<'de>> = None;
        let mut errors: Option<CibouletteErrorObj<'de>> = None;
        let mut meta: Option<Value> = None;
        let mut links: Option<CibouletteBodyLink<'de>> = None;
        let mut included: Option<CibouletteResourceSelectorBuilder<'de>> = None;
        let mut jsonapi: Option<CibouletteJsonApiVersion<'de>> = None;

        while let Some(key) = match serde::de::MapAccess::next_key::<CibouletteBodyField>(&mut map)
        {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        } {
            match key {
                CibouletteBodyField::Data => {
                    crate::serde_utils::handle_ident_in_map_stateful(&mut data, &mut map, "data")?
                }
                CibouletteBodyField::Errors => crate::serde_utils::handle_ident_in_map_stateless(
                    &mut errors,
                    &mut map,
                    "errors",
                )?,
                CibouletteBodyField::Meta => {
                    crate::serde_utils::handle_ident_in_map_stateless(&mut meta, &mut map, "meta")?
                }
                CibouletteBodyField::Links => crate::serde_utils::handle_ident_in_map_stateless(
                    &mut links, &mut map, "links",
                )?,
                CibouletteBodyField::Included => crate::serde_utils::handle_ident_in_map_stateful(
                    &mut included,
                    &mut map,
                    "included",
                )?,
                CibouletteBodyField::Jsonapi => crate::serde_utils::handle_ident_in_map_stateless(
                    &mut jsonapi,
                    &mut map,
                    "jsonapi",
                )?,
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

        let included = match included {
            Some(included) => match included.take() {
                // `included` must be an array. Error if included is an object or a value
                CibouletteSelector::Multi(included) => Ok(included),
                _ => Err(<A::Error as serde::de::Error>::custom(
                    "`included` must be an array",
                )),
            },
            None => Ok(Vec::new()),
        }?;
        if let (None, None, None) = (&data, &errors, &meta) {
            return Err(<A::Error as serde::de::Error>::custom(
                "At least one of `data`, `errors` or `meta` should be defined.",
            ));
        };
        Ok(CibouletteBodyBuilder {
            data: data.unwrap_or_default(),
            errors,
            meta,
            links,
            included,
            jsonapi,
        })
    }
}

impl<'de> Deserialize<'de> for CibouletteBodyBuilder<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<CibouletteBodyBuilder<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "CibouletteResource",
            CIBOULETTE_TOP_LEVEL_FIELDS,
            CibouletteBodyBuilderVisitor,
        )
    }
}

impl<'request> CibouletteBodyBuilder<'request> {
    /// Check that every objects in `data` is unique by `type` and `id`
    ///
    /// Shouldn't be called if creating an
    fn check_obj_uniqueness<'store>(
        data: &CibouletteResourceSelector<
            'request,
            MessyJsonObjectValue<'store>,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
    ) -> Result<(), CibouletteError> {
        let mut obj_set: BTreeSet<(&str, CibouletteIdSelector<'request>)> = BTreeSet::new();

        match &**data {
            CibouletteSelector::Single(_) => Ok(()), // Must be unique if there's only one.
            CibouletteSelector::Multi(objs) => {
                for obj in objs.iter() {
                    match obj.identifier().id() {
                        Some(id) => {
                            if !obj_set.insert((obj.identifier().type_(), id.clone())) {
                                // If already exists, fails.
                                return Err(CibouletteError::UniqObj(
                                    obj.identifier().type_().to_string(),
                                    id.to_string(),
                                ));
                            }
                        }
                        None => continue, //FIXME
                    }
                }
                Ok(())
            }
        }
    }

    /// Check that every relationships in `data` is unique by `type` and `id` for a single object
    fn check_relationships_uniqueness_single<'store, 'c>(
        linked_set: &mut BTreeSet<(&'c str, CibouletteIdSelector<'c>)>,
        obj: &'c CibouletteResource<
            'request,
            MessyJsonObjectValue<'store>,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
    ) -> Result<(), CibouletteError> {
        for (_link_name, rel) in obj.relationships().iter() {
            match rel.data().inner_deref() {
                CibouletteOptionalData::Object(CibouletteSelector::Single(el)) => {
                    if !linked_set.insert((el.type_(), el.id().clone())) {
                        // If already exists, fails.
                        return Err(CibouletteError::UniqRelationshipObject(
                            el.type_().to_string(),
                            el.id().to_string(),
                        ));
                    }
                }
                CibouletteOptionalData::Object(CibouletteSelector::Multi(els)) => {
                    for el in els.iter() {
                        if !linked_set.insert((el.type_(), el.id().clone())) {
                            // If already exists, fails.
                            return Err(CibouletteError::UniqRelationshipObject(
                                el.type_().to_string(),
                                el.id().to_string(),
                            ));
                        }
                    }
                }
                CibouletteOptionalData::Null(_) => (),
            }
        }
        Ok(())
    }

    /// Check that every relationships in `data` is unique by `type` and `id`
    fn check_relationships_uniqueness<'store, 'c>(
        data: &'c CibouletteResourceSelector<
            'request,
            MessyJsonObjectValue<'store>,
            CibouletteResourceIdentifierPermissive<'request>,
        >,
    ) -> Result<BTreeSet<(&'c str, CibouletteIdSelector<'c>)>, CibouletteError> {
        let mut linked_set = BTreeSet::new();

        match &**data {
            CibouletteSelector::Single(obj) => {
                Self::check_relationships_uniqueness_single(&mut linked_set, &obj)?;
                Ok(linked_set)
            }
            CibouletteSelector::Multi(objs) => {
                for obj in objs.iter() {
                    let mut linked_set_inner: BTreeSet<(&str, CibouletteIdSelector)> =
                        BTreeSet::new();
                    Self::check_relationships_uniqueness_single(&mut linked_set_inner, &obj)?;
                    linked_set.append(&mut linked_set_inner);
                }
                Ok(linked_set)
            }
        }
    }

    /// Check that every object in `included` is unique by `type` and `id`.
    /// Also check for linkage error in case of a compound document
    fn check_included<'store, 'c>(
        included: &'c [CibouletteResource<
            'request,
            MessyJsonObjectValue<'store>,
            CibouletteResourceIdentifierPermissive<'request>,
        >],
        check_full_linkage: bool,
    ) -> Result<BTreeSet<(&'c str, CibouletteIdSelector<'c>)>, CibouletteError> {
        let mut linked_set: BTreeSet<(&str, CibouletteIdSelector)> = BTreeSet::new();

        for obj in included.iter() {
            match obj.identifier().id() {
                Some(id) => {
                    if !linked_set.insert((obj.identifier().type_(), id.clone())) {
                        return Err(CibouletteError::UniqObj(
                            obj.identifier().type_().to_string(),
                            id.to_string(),
                        ));
                    }
                    // Check obj is complete for full linkage
                    if check_full_linkage && obj.attributes().is_none() {
                        return Err(CibouletteError::NoCompleteLinkage(
                            obj.identifier().type_().to_string(),
                            id.to_string(),
                        ));
                    }
                }
                None => return Err(CibouletteError::MissingId),
            }
        }
        Ok(linked_set)
    }

    /// Checks for key clash like `included` without `data`, or `data` with `errors`
    #[inline]
    fn check_key_clash<'c>(
        data: &'c CibouletteBodyData<
            'request,
            CibouletteResourceIdentifierPermissive<'request>,
            MessyJsonObjectValue<'request>,
        >,
        included: &'c [CibouletteResource<
            'request,
            MessyJsonObjectValue<'request>,
            CibouletteResourceIdentifierPermissive<'request>,
        >],
        errors: &'c Option<CibouletteErrorObj<'request>>,
    ) -> Result<(), CibouletteError> {
        let is_data_null = matches!(data, CibouletteBodyData::Null(_));

        // Can't have an empty `data` with an `included` key
        if is_data_null && !included.is_empty() {
            return Err(CibouletteError::KeyClash(
                "included".to_string(),
                CibouletteClashDirection::With,
                "data".to_string(),
            ));
        }
        // Can't have a `data` obj with an `errors` key
        if !is_data_null && errors.is_some() {
            return Err(CibouletteError::KeyClash(
                "data".to_string(),
                CibouletteClashDirection::Without,
                "errors".to_string(),
            ));
        }
        Ok(())
    }

    /// Perfom all the document checks
    pub fn check<'c>(
        intention: &CibouletteIntention,
        data: &'c CibouletteBodyData<
            'request,
            CibouletteResourceIdentifierPermissive<'request>,
            MessyJsonObjectValue<'request>,
        >,
        included: &'c [CibouletteResource<
            'request,
            MessyJsonObjectValue<'request>,
            CibouletteResourceIdentifierPermissive<'request>,
        >],
        errors: &'c Option<CibouletteErrorObj<'request>>,
    ) -> Result<(), CibouletteError> {
        Self::check_key_clash(&data, &included, &errors)?;
        match data {
            CibouletteBodyData::Object(data) => {
                let rel_set: BTreeSet<(&str, CibouletteIdSelector)>;

                Self::check_obj_uniqueness(&data)?;
                rel_set = Self::check_relationships_uniqueness(&data)?;
                let (check_full_linkage, included_set) = match &**data {
                    CibouletteSelector::Multi(_) => {
                        let included_set = Self::check_included(&included, true)?;
                        (true, included_set)
                    }
                    CibouletteSelector::Single(_) => {
                        let included_set = Self::check_included(&included, false)?;
                        (true, included_set)
                    }
                };
                if check_full_linkage && matches!(intention, CibouletteIntention::Read) {
                    if let Some((type_, id)) = rel_set.difference(&included_set).into_iter().next()
                    {
                        return Err(CibouletteError::MissingLink(
                            type_.to_string(),
                            id.to_string(),
                        ));
                    }
                }
            }
            CibouletteBodyData::Null(_) => (),
        };

        Ok(())
    }

    /// Build a [CibouletteBody](CibouletteBody) from the builder.
    ///
    /// Runs `check` before building
    pub fn build(
        self,
        bag: &CibouletteStore,
        intention: &CibouletteIntention,
    ) -> Result<
        CibouletteBody<
            'request,
            CibouletteResourceIdentifierPermissive<'request>,
            MessyJsonObjectValue<'request>,
        >,
        CibouletteError,
    > {
        let res: CibouletteBody<
            'request,
            CibouletteResourceIdentifierPermissive<'request>,
            MessyJsonObjectValue<'request>,
        >;

        let data = self.data.build(&bag, &intention)?;
        let mut included: Vec<
            CibouletteResource<
                'request,
                MessyJsonObjectValue<'request>,
                CibouletteResourceIdentifierPermissive,
            >,
        > = Vec::with_capacity(self.included.len());
        for i in self.included.into_iter() {
            included.push(i.build(&bag, &intention)?);
        }
        Self::check(&intention, &data, &included, &self.errors)?;
        res = CibouletteBody {
            data,
            errors: self.errors,
            meta: self.meta,
            links: self.links,
            jsonapi: self.jsonapi,
            included,
        };
        Ok(res)
    }
}

impl<'request, I, B> CibouletteBody<'request, I, B> {
    /// Check if the request is a compound document
    pub fn is_compound(&self) -> bool {
        matches!(
            self.data(),
            CibouletteBodyData::Object(obj)
            if matches!(&**obj, CibouletteSelector::Multi(_))
        )
    }

    /// Check if the request has data
    pub fn has_data(&self) -> bool {
        matches!(self.data(), CibouletteBodyData::Object(_))
    }
}

impl<'request, B> CibouletteBody<'request, CibouletteResourceIdentifierPermissive<'request>, B> {
    /// Get the main type of the request
    /// If it's a single document request, the type of the document is used.
    /// If it's a compound document request and all the document are the same type, then this type is used.
    /// Else `None` is returned
    pub fn get_main_type(&self, bag: &CibouletteStore) -> Option<Arc<CibouletteResourceType>> {
        match self.data() {
            CibouletteBodyData::Object(data) => match &**data {
                CibouletteSelector::Single(x) => {
                    bag.get_type_if_exists(x.identifier().type_().as_ref())
                }
                CibouletteSelector::Multi(types) => {
                    let mut titer = types.iter();
                    let first_type = match titer.next() {
                        Some(x) => x.identifier().type_(),
                        _ => return None,
                    };
                    for type_ in titer {
                        if type_.identifier().type_() != first_type {
                            return None;
                        }
                    }
                    bag.get_type_if_exists(first_type.as_ref())
                }
            },
            CibouletteBodyData::Null(_) => None,
        }
    }

    /// Check if the request has all its `id` set (not always the case in creating requests)
    ///
    /// true if there is no data
    pub fn has_all_ids(&self) -> bool {
        if let CibouletteBodyData::Object(data) = self.data() {
            match &**data {
                CibouletteSelector::Single(r) => r.identifier().id().is_some(),
                CibouletteSelector::Multi(rs) => !rs.iter().any(|r| !r.identifier().id().is_some()),
            }
        } else {
            true
        }
    }
}
