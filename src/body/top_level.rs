use super::*;
use serde::de::{Deserializer, Visitor};
use std::fmt::Formatter;

/// ## Builder object for [CibouletteTopLevel](CibouletteTopLevel)
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteTopLevelBuilder<'a> {
    data: Option<CibouletteResourceSelectorBuilder<'a>>,
    errors: Option<CibouletteErrorObj<'a>>,
    meta: Value,
    links: Option<CibouletteLink<'a>>,
    included: Vec<CibouletteResourceBuilder<'a>>,
    jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

/// ## A `json:api` [document](https://jsonapi.org/format/#document-top-level) object
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteTopLevel<'a> {
    data: Option<CibouletteResourceSelector<'a>>,
    errors: Option<CibouletteErrorObj<'a>>,
    meta: Value,
    links: Option<CibouletteLink<'a>>,
    included: Vec<CibouletteResource<'a>>,
    jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

const CIBOULETTE_TOP_LEVEL_FIELDS: &[&str] =
    &["data", "errors", "meta", "links", "included", "jsonapi"];

#[derive(Clone, Debug)]
struct CibouletteTopLevelBuilderVisitor;

enum CibouletteTopLevelField {
    Data,
    Errors,
    Meta,
    Links,
    Included,
    Jsonapi,
    Ignore,
}

struct CibouletteTopLevelFieldVisitor;
impl<'de> Visitor<'de> for CibouletteTopLevelFieldVisitor {
    type Value = CibouletteTopLevelField;

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
            "data" => Ok(CibouletteTopLevelField::Data),
            "errors" => Ok(CibouletteTopLevelField::Errors),
            "meta" => Ok(CibouletteTopLevelField::Meta),
            "links" => Ok(CibouletteTopLevelField::Links),
            "included" => Ok(CibouletteTopLevelField::Included),
            "jsonapi" => Ok(CibouletteTopLevelField::Jsonapi),
            _ => Ok(CibouletteTopLevelField::Ignore),
        }
    }

    #[inline]
    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            b"data" => Ok(CibouletteTopLevelField::Data),
            b"errors" => Ok(CibouletteTopLevelField::Errors),
            b"meta" => Ok(CibouletteTopLevelField::Meta),
            b"links" => Ok(CibouletteTopLevelField::Links),
            b"included" => Ok(CibouletteTopLevelField::Included),
            b"jsonapi" => Ok(CibouletteTopLevelField::Jsonapi),
            _ => Ok(CibouletteTopLevelField::Ignore),
        }
    }
}
impl<'de> serde::Deserialize<'de> for CibouletteTopLevelField {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserializer::deserialize_identifier(deserializer, CibouletteTopLevelFieldVisitor)
    }
}
impl<'de> serde::de::Visitor<'de> for CibouletteTopLevelBuilderVisitor {
    type Value = CibouletteTopLevelBuilder<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResource")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut data: Option<CibouletteResourceSelectorBuilder<'de>> = None;
        let mut errors: Option<CibouletteErrorObj<'de>> = None;
        let mut meta: Option<Value> = None;
        let mut links: Option<CibouletteLink<'de>> = None;
        let mut included: Option<CibouletteResourceSelectorBuilder<'de>> = None; // TODO CHECK THAT it's many
        let mut jsonapi: Option<Cow<'de, str>> = None;

        while let Some(key) =
            match serde::de::MapAccess::next_key::<CibouletteTopLevelField>(&mut map) {
                Ok(val) => val,
                Err(err) => {
                    return Err(err);
                }
            }
        {
            match key {
                CibouletteTopLevelField::Data => {
                    super::handle_ident_in_map_stateful(&mut data, &mut map, "data")?
                }
                CibouletteTopLevelField::Errors => {
                    super::handle_ident_in_map_stateless(&mut errors, &mut map, "errors")?
                }
                CibouletteTopLevelField::Meta => {
                    super::handle_ident_in_map_stateless(&mut meta, &mut map, "meta")?
                }
                CibouletteTopLevelField::Links => {
                    super::handle_ident_in_map_stateless(&mut links, &mut map, "links")?
                }
                CibouletteTopLevelField::Included => {
                    super::handle_ident_in_map_stateful(&mut included, &mut map, "included")?
                }
                CibouletteTopLevelField::Jsonapi => {
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
        Ok(CibouletteTopLevelBuilder {
            data,
            errors,
            meta: meta.unwrap_or_default(),
            links,
            included,
            jsonapi,
        })
    }
}

impl<'de> Deserialize<'de> for CibouletteTopLevelBuilder<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<CibouletteTopLevelBuilder<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "CibouletteResource",
            CIBOULETTE_TOP_LEVEL_FIELDS,
            CibouletteTopLevelBuilderVisitor,
        )
    }
}

impl<'a> CibouletteTopLevelBuilder<'a> {
    /// Build a [CibouletteTopLevel](CibouletteTopLevel) from the builder
    pub fn build(self, bag: &'a CibouletteBag) -> Result<CibouletteTopLevel<'a>, CibouletteError> {
        let res: CibouletteTopLevel<'a>;

        let data = match self.data {
            Some(data) => Some(data.build(bag)?),
            None => None,
        };
        let mut included: Vec<CibouletteResource<'a>> = Vec::with_capacity(self.included.len());
        for i in self.included.into_iter() {
            included.push(i.build(&bag)?);
        }
        res = CibouletteTopLevel {
            data,
            errors: self.errors,
            meta: self.meta,
            links: self.links,
            jsonapi: self.jsonapi,
            included,
        };
        res.check()?;
        Ok(res)
    }
}

impl<'a> CibouletteTopLevel<'a> {
    /// Check that every objects in `data` is unique by `type` and `id`
    fn check_obj_uniqueness(&self) -> Result<(), CibouletteError> {
        let mut obj_set: BTreeSet<(&str, &str)> = BTreeSet::new();

        if let Some(data) = self.data() {
            match data {
                CibouletteResourceSelector::One(_) => Ok(()),
                CibouletteResourceSelector::Many(objs) => {
                    for obj in objs.iter() {
                        if !obj_set.insert((obj.identifier().type_(), obj.identifier().id())) {
                            return Err(CibouletteError::UniqObj(
                                obj.identifier().type_().to_string(),
                                obj.identifier().id().to_string(),
                            ));
                        }
                    }
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }

    /// Check that every relationships in `data` is unique by `type` and `id` for a single object
    fn check_relationships_uniqueness_single(
        linked_set: &mut BTreeSet<(&'a str, &'a str)>,
        obj: &'a CibouletteResource,
    ) -> Result<(), CibouletteError> {
        for (_link_name, rel) in obj.relationships().iter() {
            match rel.data() {
                Some(CibouletteResourceIdentifierSelector::One(el)) => {
                    if !linked_set.insert((el.type_(), el.id())) {
                        return Err(CibouletteError::UniqRelationship(
                            el.type_().to_string(),
                            el.id().to_string(),
                        ));
                    }
                }
                Some(CibouletteResourceIdentifierSelector::Many(els)) => {
                    for el in els.iter() {
                        if !linked_set.insert((el.type_(), el.id())) {
                            return Err(CibouletteError::UniqRelationship(
                                el.type_().to_string(),
                                el.id().to_string(),
                            ));
                        }
                    }
                }
                None => (),
            }
        }
        Ok(())
    }

    /// Check that every relationships in `data` is unique by `type` and `id`
    fn check_relationships_uniqueness(&'a self) -> Result<BTreeSet<(&str, &str)>, CibouletteError> {
        let mut linked_set: BTreeSet<(&str, &str)> = BTreeSet::new();

        if let Some(data) = self.data() {
            return match data {
                CibouletteResourceSelector::One(obj) => {
                    Self::check_relationships_uniqueness_single(&mut linked_set, &obj)?;
                    Ok(linked_set)
                }
                CibouletteResourceSelector::Many(objs) => {
                    for obj in objs.iter() {
                        let mut linked_set_inner: BTreeSet<(&str, &str)> = BTreeSet::new();
                        Self::check_relationships_uniqueness_single(&mut linked_set_inner, &obj)?;
                        linked_set.append(&mut linked_set_inner);
                    }
                    Ok(linked_set)
                }
            };
        }
        Ok(linked_set)
    }

    /// Check that every object in `included` is unique by `type` and `id`.
    /// Also check for linkage error in case of a compound document
    fn check_included(
        &'a self,
        check_full_linkage: bool,
    ) -> Result<BTreeSet<(&str, &str)>, CibouletteError> {
        let mut linked_set: BTreeSet<(&str, &str)> = BTreeSet::new();

        for obj in self.included().iter() {
            if !linked_set.insert((obj.identifier().type_(), obj.identifier().id())) {
                return Err(CibouletteError::UniqObj(
                    obj.identifier().type_().to_string(),
                    obj.identifier().id().to_string(),
                ));
            }
            if check_full_linkage && obj.attributes().is_none() {
                return Err(CibouletteError::NoCompleteLinkage(
                    obj.identifier().type_().to_string(),
                    obj.identifier().id().to_string(),
                ));
            }
        }
        Ok(linked_set)
    }

    /// Checks for key clash like `included` without `data`, or `data` with `errors`
    #[inline]
    fn check_key_clash(&self) -> Result<(), CibouletteError> {
        if self.data().is_none() && !self.included().is_empty() {
            return Err(CibouletteError::KeyClash(
                "included".to_string(),
                CibouletteClashDirection::With,
                "data".to_string(),
            ));
        }
        if self.data().is_some() && self.errors().is_some() {
            return Err(CibouletteError::KeyClash(
                "data".to_string(),
                CibouletteClashDirection::Without,
                "errors".to_string(),
            ));
        }
        Ok(())
    }

    /// Perfom all the document checks
    pub fn check(&self) -> Result<(), CibouletteError> {
        let rel_set: BTreeSet<(&str, &str)>;
        let included_set: BTreeSet<(&str, &str)>;

        self.check_key_clash()?;
        let check_full_linkage: bool =
            matches!(self.data(), Some(CibouletteResourceSelector::Many(_)));

        self.check_obj_uniqueness()?;
        rel_set = self.check_relationships_uniqueness()?;
        included_set = self.check_included(check_full_linkage)?;
        if check_full_linkage {
            if let Some((type_, id)) = rel_set.difference(&included_set).into_iter().next() {
                return Err(CibouletteError::MissingLink(
                    type_.to_string(),
                    id.to_string(),
                ));
            }
        }
        Ok(())
    }
}
