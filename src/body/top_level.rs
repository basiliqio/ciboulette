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
        let data = match self.data {
            Some(data) => Some(data.build(bag)?),
            None => None,
        };
        let mut included: Vec<CibouletteResource<'a>> = Vec::with_capacity(self.included.len());
        for i in self.included.into_iter() {
            included.push(i.build(&bag)?);
        }
        // TODO verify that every object is uniq by (`type`, `id`)
        // TODO verify that every included object is uniq by (`type` and `id`)
        // TODO verify full linkage
        Ok(CibouletteTopLevel {
            data,
            errors: self.errors,
            meta: self.meta,
            links: self.links,
            jsonapi: self.jsonapi,
            included,
        })
    }
}
