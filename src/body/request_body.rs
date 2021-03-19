use super::*;
use serde::de::{Deserializer, Visitor};
use std::fmt::Formatter;

/// ## Builder object for [CibouletteBody](CibouletteBody)
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteBodyBuilder<'a> {
    data: CibouletteBodyDataBuilder<'a>,
    errors: Option<CibouletteErrorObj<'a>>,
    meta: Value,
    links: Option<CibouletteBodyLink<'a>>,
    included: Vec<CibouletteResourceBuilder<'a>>,
    jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

/// ## A `json:api` [document](https://jsonapi.org/format/#document-top-level) object
#[derive(Debug, Getters, MutGetters, Default)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteBody<'a> {
    pub data: CibouletteBodyData<'a>,
    pub errors: Option<CibouletteErrorObj<'a>>,
    pub meta: Value,
    pub links: Option<CibouletteBodyLink<'a>>,
    pub included: Vec<CibouletteResource<'a, CibouletteResourceIdentifierPermissive<'a>>>,
    pub jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

const CIBOULETTE_TOP_LEVEL_FIELDS: &[&str] =
    &["data", "errors", "meta", "links", "included", "jsonapi"];

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
        let mut jsonapi: Option<Cow<'de, str>> = None;

        while let Some(key) = match serde::de::MapAccess::next_key::<CibouletteBodyField>(&mut map)
        {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        } {
            match key {
                CibouletteBodyField::Data => {
                    super::handle_ident_in_map_stateful(&mut data, &mut map, "data")?
                }
                CibouletteBodyField::Errors => {
                    super::handle_ident_in_map_stateless(&mut errors, &mut map, "errors")?
                }
                CibouletteBodyField::Meta => {
                    super::handle_ident_in_map_stateless(&mut meta, &mut map, "meta")?
                }
                CibouletteBodyField::Links => {
                    super::handle_ident_in_map_stateless(&mut links, &mut map, "links")?
                }
                CibouletteBodyField::Included => {
                    super::handle_ident_in_map_stateful(&mut included, &mut map, "included")?
                }
                CibouletteBodyField::Jsonapi => {
                    super::handle_ident_in_map_stateless(&mut jsonapi, &mut map, "jsonapi")?
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

        let included = match included {
            Some(included) => match included {
                CibouletteResourceSelectorBuilder::Many(included) => Ok(included),
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
            meta: meta.unwrap_or_default(),
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

impl<'a> CibouletteBodyBuilder<'a> {
    /// Check that every objects in `data` is unique by `type` and `id`
    ///
    /// Shouldn't be called if creating an
    fn check_obj_uniqueness(
        data: &CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>,
    ) -> Result<(), CibouletteError> {
        let mut obj_set: BTreeSet<(&str, &CibouletteId<'a>)> = BTreeSet::new();

        match data {
            CibouletteResourceSelector::One(_) => Ok(()),
            CibouletteResourceSelector::Many(objs) => {
                for obj in objs.iter() {
                    match obj.identifier().id() {
                        Some(id) => {
                            if !obj_set.insert((obj.identifier().type_(), id)) {
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
    fn check_relationships_uniqueness_single<'b>(
        linked_set: &mut BTreeSet<(&'b str, &'b CibouletteId<'b>)>,
        obj: &'b CibouletteResource<CibouletteResourceIdentifierPermissive<'a>>,
    ) -> Result<(), CibouletteError> {
        for (_link_name, rel) in obj.relationships().iter() {
            match rel.data() {
                CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(el)) => {
                    if !linked_set.insert((el.type_(), el.id())) {
                        return Err(CibouletteError::UniqRelationshipObject(
                            el.type_().to_string(),
                            el.id().to_string(),
                        ));
                    }
                }
                CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(els)) => {
                    for el in els.iter() {
                        if !linked_set.insert((el.type_(), el.id())) {
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
    fn check_relationships_uniqueness<'b>(
        data: &'b CibouletteResourceSelector<'a, CibouletteResourceIdentifierPermissive<'a>>,
    ) -> Result<BTreeSet<(&'b str, &'b CibouletteId<'b>)>, CibouletteError> {
        let mut linked_set = BTreeSet::new();

        match data {
            CibouletteResourceSelector::One(obj) => {
                Self::check_relationships_uniqueness_single(&mut linked_set, &obj)?;
                Ok(linked_set)
            }
            CibouletteResourceSelector::Many(objs) => {
                for obj in objs.iter() {
                    let mut linked_set_inner: BTreeSet<(&str, &CibouletteId)> = BTreeSet::new();
                    Self::check_relationships_uniqueness_single(&mut linked_set_inner, &obj)?;
                    linked_set.append(&mut linked_set_inner);
                }
                Ok(linked_set)
            }
        }
    }

    /// Check that every object in `included` is unique by `type` and `id`.
    /// Also check for linkage error in case of a compound document
    fn check_included<'b>(
        included: &'b [CibouletteResource<'a, CibouletteResourceIdentifierPermissive<'a>>],
        check_full_linkage: bool,
    ) -> Result<BTreeSet<(&'b str, &'b CibouletteId<'b>)>, CibouletteError> {
        let mut linked_set: BTreeSet<(&str, &CibouletteId)> = BTreeSet::new();

        for obj in included.iter() {
            match obj.identifier().id() {
                Some(id) => {
                    if !linked_set.insert((obj.identifier().type_(), id)) {
                        return Err(CibouletteError::UniqObj(
                            obj.identifier().type_().to_string(),
                            id.to_string(),
                        ));
                    }
                    if check_full_linkage && obj.attributes().is_none() {
                        return Err(CibouletteError::NoCompleteLinkage(
                            obj.identifier().type_().to_string(),
                            id.to_string(),
                        ));
                    }
                }
                None => continue, //FIXME
            }
        }
        Ok(linked_set)
    }

    /// Checks for key clash like `included` without `data`, or `data` with `errors`
    #[inline]
    fn check_key_clash<'b>(
        data: &'b CibouletteBodyData<'a>,
        included: &'b [CibouletteResource<'a, CibouletteResourceIdentifierPermissive<'a>>],
        errors: &'b Option<CibouletteErrorObj<'a>>,
    ) -> Result<(), CibouletteError> {
        let is_data_null = matches!(data, CibouletteBodyData::Null(_));

        if is_data_null && !included.is_empty() {
            return Err(CibouletteError::KeyClash(
                "included".to_string(),
                CibouletteClashDirection::With,
                "data".to_string(),
            ));
        }
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
    pub fn check<'b>(
        intention: &CibouletteIntention,
        data: &'b CibouletteBodyData<'a>,
        included: &'b [CibouletteResource<'a, CibouletteResourceIdentifierPermissive<'a>>],
        errors: &'b Option<CibouletteErrorObj<'a>>,
    ) -> Result<(), CibouletteError> {
        Self::check_key_clash(&data, &included, &errors)?;
        match data {
            CibouletteBodyData::Object(data) => {
                let rel_set: BTreeSet<(&str, &CibouletteId)>;

                Self::check_obj_uniqueness(&data)?;
                rel_set = Self::check_relationships_uniqueness(&data)?;
                let (check_full_linkage, included_set) = match &data {
                    CibouletteResourceSelector::Many(_) => {
                        let included_set = Self::check_included(&included, true)?;
                        (true, included_set)
                    }
                    CibouletteResourceSelector::One(inner_data) => {
                        let included_set = Self::check_included(&included, false)?;
                        inner_data.check_member_name()?;
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

    /// Build a [CibouletteBody](CibouletteBody) from the builder
    pub fn build(
        self,
        bag: &'a CibouletteStore<'a>,
        intention: &CibouletteIntention,
        main_type: &CibouletteResourceType<'a>,
    ) -> Result<CibouletteBody<'a>, CibouletteError> {
        let res: CibouletteBody<'a>;

        let data = self.data.build(&bag, &intention, &main_type)?;
        let mut included: Vec<CibouletteResource<'a, CibouletteResourceIdentifierPermissive>> =
            Vec::with_capacity(self.included.len());
        for i in self.included.into_iter() {
            included.push(i.build(&bag, &intention, &main_type)?);
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

impl<'a> CibouletteBody<'a> {
    /// Get the main type of the request
    /// If it's a single document request, the type of the document is used.
    /// If it's a compound document request and all the document are the same type, then this type is used.
    /// Else `None` is returned
    pub fn get_main_type(
        &self,
        bag: &'a CibouletteStore<'a>,
    ) -> Option<&CibouletteResourceType<'a>> {
        match self.data() {
            CibouletteBodyData::Object(data) => match data {
                CibouletteResourceSelector::One(x) => bag.get_type(x.identifier().type_().as_ref()),
                CibouletteResourceSelector::Many(types) => {
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
                    bag.get_type(first_type.as_ref())
                }
            },
            CibouletteBodyData::Null(_) => None,
        }
    }

    /// Check if the request is a compound document
    pub fn is_compound(&self) -> bool {
        matches!(
            self.data(),
            CibouletteBodyData::Object(obj)
            if matches!(obj, CibouletteResourceSelector::Many(_))
        )
    }

    /// Check if the request has data
    pub fn has_data(&self) -> bool {
        matches!(self.data(), CibouletteBodyData::Object(_))
    }

    /// Check if the request has all its `id` set (not always the case in creating requests)
    ///
    /// true if there is no data
    pub fn has_all_ids(&self) -> bool {
        if let CibouletteBodyData::Object(data) = self.data() {
            match data {
                CibouletteResourceSelector::One(r) => r.identifier().id().is_some(),
                CibouletteResourceSelector::Many(rs) => {
                    !rs.iter().any(|r| !r.identifier().id().is_some())
                }
            }
        } else {
            true
        }
    }
}
