use super::*;
use serde::de::{Deserializer, Visitor};
use std::fmt::Formatter;

#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteTopLevelBuilder<'a> {
    data: Option<CibouletteResourceSelectorBuilder<'a>>,
    errors: Option<CibouletteErrorObj<'a>>,
    meta: Option<Value>,
    links: Option<CibouletteLink<'a>>,
    included: Vec<CibouletteResourceBuilder<'a>>,
    jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CibouletteTopLevel<'a> {
    data: Option<CibouletteResourceSelector<'a>>,
    errors: Option<CibouletteErrorObj<'a>>,
    meta: Option<Value>,
    links: Option<CibouletteLink<'a>>,
    included: Vec<CibouletteResource<'a>>,
    jsonapi: Option<Cow<'a, str>>, // TODO Semver
}

const CIBOULETTE_TOP_LEVEL_FIELDS: &[&str] =
    &["data", "errors", "meta", "links", "included", "jsonapi"];

#[derive(Clone, Debug)]
pub struct CibouletteTopLevelBuilderVisitor;

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
            meta,
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

    fn check_linked_obj_uniqueness_single(
        linked_set: &mut BTreeSet<(&'a str, &'a str)>,
        obj: &'a CibouletteResource,
    ) -> Result<(), CibouletteError> {
        for (_link_name, rel) in obj.relationships().iter() {
            match rel.data() {
                Some(CibouletteResourceIdentifierSelector::One(el)) => {
                    if !linked_set.insert((el.type_(), el.id())) {
                        return Err(CibouletteError::UniqObj(
                            el.type_().to_string(),
                            el.id().to_string(),
                        ));
                    }
                }
                Some(CibouletteResourceIdentifierSelector::Many(els)) => {
                    for el in els.iter() {
                        if !linked_set.insert((el.type_(), el.id())) {
                            return Err(CibouletteError::UniqObj(
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

    fn check_linked_obj_uniqueness(&'a self) -> Result<BTreeSet<(&str, &str)>, CibouletteError> {
        let mut linked_set: BTreeSet<(&str, &str)> = BTreeSet::new();

        if let Some(data) = self.data() {
            return match data {
                CibouletteResourceSelector::One(obj) => {
                    Self::check_linked_obj_uniqueness_single(&mut linked_set, &obj)?;
                    Ok(linked_set)
                }
                CibouletteResourceSelector::Many(objs) => {
                    for obj in objs.iter() {
                        Self::check_linked_obj_uniqueness_single(&mut linked_set, &obj)?;
                    }
                    Ok(linked_set)
                }
            };
        }
        Ok(linked_set)
    }

    fn check_included_obj_uniqueness(
        &'a self,
        linked_set: &'a mut BTreeSet<(&'a str, &'a str)>,
    ) -> Result<(), CibouletteError> {
        for included_el in self.included().iter() {
            if linked_set.insert((
                included_el.identifier().type_(),
                included_el.identifier().id(),
            )) {
                return Err(CibouletteError::MissingLink(
                    included_el.identifier().type_().to_string(),
                    included_el.identifier().id().to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn check(&self) -> Result<(), CibouletteError> {
        let mut linked_set: BTreeSet<(&str, &str)>;

        self.check_obj_uniqueness()?;
        linked_set = self.check_linked_obj_uniqueness()?;
        if let Some(CibouletteResourceSelector::Many(_)) = self.data()
        //TODO CHECK for full linkage
        {
            self.check_included_obj_uniqueness(&mut linked_set)?;
        }
        Ok(())
    }
}
