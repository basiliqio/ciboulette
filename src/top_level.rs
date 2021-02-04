use super::*;
use serde::{
    de::DeserializeOwned, de::DeserializeSeed, de::MapAccess, de::SeqAccess, de::Visitor,
    Deserializer,
};
use std::fmt::Formatter;

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

impl<'a> CibouletteTopLevel<'a> {
    pub fn deserialize<R>(
        d: &mut serde_json::Deserializer<R>,
        bag: &'a CibouletteBag,
    ) -> Result<Self, serde_json::Error>
    where
        R: serde_json::de::Read<'a>,
    {
        let visitor = CibouletteTopLevelVisitor(bag);

        visitor.deserialize(d)
    }
}

#[derive(Clone, Debug)]
pub struct CibouletteTopLevelVisitor<'a>(&'a CibouletteBag<'a>);

impl<'a> CibouletteTopLevelVisitor<'a> {
    #[inline]
    pub fn new(bag: &'a CibouletteBag<'a>) -> Self {
        CibouletteTopLevelVisitor(bag)
    }
}

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
impl<'de> serde::de::Visitor<'de> for CibouletteTopLevelVisitor<'de> {
    type Value = CibouletteTopLevel<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResource")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut data: Option<CibouletteResourceSelector<'de>> = None;
        let mut errors: Option<CibouletteErrorObj<'de>> = None;
        let mut meta: Option<Value> = None;
        let mut links: Option<CibouletteLink<'de>> = None;
        let mut included: Option<CibouletteResourceSelector<'de>> = None; // TODO CHECK THAT it's many
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
                CibouletteTopLevelField::Data => super::handle_ident_in_map_stateful(
                    &mut data,
                    &mut map,
                    "data",
                    CibouletteResourceSelectorVisitor::new(self.0),
                )?,
                CibouletteTopLevelField::Errors => {
                    super::handle_ident_in_map_stateless(&mut errors, &mut map, "errors")?
                }
                CibouletteTopLevelField::Meta => {
                    super::handle_ident_in_map_stateless(&mut meta, &mut map, "meta")?
                }
                CibouletteTopLevelField::Links => {
                    super::handle_ident_in_map_stateless(&mut links, &mut map, "links")?
                }
                CibouletteTopLevelField::Included => super::handle_ident_in_map_stateful(
                    &mut included,
                    &mut map,
                    "included",
                    CibouletteResourceSelectorVisitor::new(self.0),
                )?,
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
                CibouletteResourceSelector::Many(included) => Ok(included),
                CibouletteResourceSelector::Null => Ok(Vec::new()),
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
        Ok(CibouletteTopLevel {
            data,
            errors,
            meta,
            links,
            included,
            jsonapi,
        })
    }
}

impl<'de> DeserializeSeed<'de> for CibouletteTopLevelVisitor<'de> {
    type Value = CibouletteTopLevel<'de>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "CibouletteResource",
            CIBOULETTE_TOP_LEVEL_FIELDS,
            CibouletteTopLevelVisitor::new(self.0),
        )
    }
}
