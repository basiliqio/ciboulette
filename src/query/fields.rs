use super::*;
use serde::de::Visitor;
use std::fmt::Formatter;

pub struct CibouletteQueryParametersFieldVisitor;

/// Field of `json:api` query parameters object
pub enum CibouletteQueryParametersField<'request> {
    /// `include` parameter
    Include,
    /// `field[*]` parameter, filling the vector with types separated by '.'
    Sparse(Vec<Cow<'request, str>>),
    /// The `sort` parameter
    Sorting,
    /// The page[<type>] parameter, parsing the inner type
    Page(CiboulettePageType<'request>),
    /// The simple `filter` parameter
    Filter,
    /// The typed `filter[<type>]` parameter with the type as argument
    FilterTyped(Cow<'request, str>),
    /// Any other parameter
    Meta(Cow<'request, str>),
}

/// The page type used in the [CibouletteQueryParametersField](CibouletteQueryParametersField)
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CiboulettePageType<'request> {
    Number,
    Size,
    Offset,
    Limit,
    Cursor,
    Other(Cow<'request, str>),
}

impl CibouletteQueryParametersFieldVisitor {
    #[inline]
    fn parse_str(value: Cow<'_, str>) -> CibouletteQueryParametersField<'_> {
        // Check the simplier types for match
        let preemptive_val = match value.as_ref() {
            "include" => Some(CibouletteQueryParametersField::Include),
            "sort" => Some(CibouletteQueryParametersField::Sorting),
            "filter" => Some(CibouletteQueryParametersField::Filter),
            _ => None,
        };
        if let Some(preemptive_val) = preemptive_val {
            // Return then in case of match
            return preemptive_val;
        }
        let has_type = value.find('['); // Is it a typed parameter ?
        match has_type {
            Some(type_end_index) => {
                // Yes, then which one is it
                let type_ = &value[0..type_end_index];
                match type_ {
                    "page" => {
                        let page_type =
                            typed_param::parse_typed_query_param(&value[type_end_index..]) // Parse inner parameter
                                .unwrap_or_default();
                        match page_type.as_ref() {
                            "limit" => {
                                CibouletteQueryParametersField::Page(CiboulettePageType::Limit)
                            }
                            "size" => {
                                CibouletteQueryParametersField::Page(CiboulettePageType::Size)
                            }
                            "offset" => {
                                CibouletteQueryParametersField::Page(CiboulettePageType::Offset)
                            }
                            "number" => {
                                CibouletteQueryParametersField::Page(CiboulettePageType::Number)
                            }
                            "cursor" => {
                                CibouletteQueryParametersField::Page(CiboulettePageType::Cursor)
                            }
                            _ => CibouletteQueryParametersField::Page(CiboulettePageType::Other(
                                Cow::Owned(page_type.into_owned()),
                            )),
                        }
                    }
                    "fields" => CibouletteQueryParametersField::Sparse(
                        typed_param::parse_typed_query_params(&value[type_end_index..]) // Extract parameters
                            .unwrap_or_default(),
                    ),
                    "filter" => {
                        let type_ = typed_param::parse_typed_query_param(&value[type_end_index..])
                            .unwrap_or_default();
                        CibouletteQueryParametersField::FilterTyped(Cow::Owned(type_.into_owned()))
                    }
                    _ => CibouletteQueryParametersField::Meta(value),
                }
            }
            None => CibouletteQueryParametersField::Meta(value),
        }
    }
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
        Ok(Self::parse_str(Cow::Borrowed(value)))
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Self::parse_str(Cow::Owned(value.to_string())))
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Self::parse_str(Cow::Owned(value)))
    }

    #[inline]
    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(
            std::str::from_utf8(value)
                .map_err(|e| serde::de::Error::custom(format!("UTF8 error : {}", e)))?,
        )
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
