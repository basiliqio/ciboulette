use super::*;
use serde::de::Visitor;
use std::fmt::Formatter;

pub struct CibouletteQueryParametersFieldVisitor;

pub enum CibouletteQueryParametersField<'a> {
    Include,
    Sparse(Vec<Cow<'a, str>>),
    Sorting,
    Page(CiboulettePageType<'a>),
    Filter,
    FilterTyped(Cow<'a, str>),
    Meta(Cow<'a, str>),
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum CiboulettePageType<'a> {
    Number,
    Size,
    Offset,
    Limit,
    Cursor,
    Other(Cow<'a, str>),
}

impl CibouletteQueryParametersFieldVisitor {
    #[inline]
    fn parse_str<'a, E>(value: Cow<'a, str>) -> Result<CibouletteQueryParametersField<'a>, E>
    where
        E: serde::de::Error,
    {
        let preemptive_val = match value.as_ref() {
            "include" => Some(CibouletteQueryParametersField::Include),
            "sort" => Some(CibouletteQueryParametersField::Sorting),
            "filter" => Some(CibouletteQueryParametersField::Filter),
            _ => None,
        };
        if let Some(preemptive_val) = preemptive_val {
            return Ok(preemptive_val);
        }
        let has_type = value.find('[');
        match has_type {
            Some(type_end_index) => {
                let type_ = &value[0..type_end_index];
                match type_ {
                    "page" => {
                        let mut page_type_vec =
                            typed_param::parse_typed_query_param(&value[type_end_index..])
                                .unwrap_or_default();
                        let page_type: Cow<'a, str> = match page_type_vec.len() {
                            0 => Cow::Borrowed(""),
                            1 => page_type_vec.pop().unwrap(),
                            _ => Cow::Owned(page_type_vec.join(".")),
                        };
                        match page_type.as_ref() {
                            "limit" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Limit,
                            )),
                            "size" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Size,
                            )),
                            "offset" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Offset,
                            )),
                            "number" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Number,
                            )),
                            "cursor" => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Cursor,
                            )),
                            _ => Ok(CibouletteQueryParametersField::Page(
                                CiboulettePageType::Other(page_type),
                            )),
                        }
                    }
                    "fields" => Ok(CibouletteQueryParametersField::Sparse(
                        typed_param::parse_typed_query_param(&value[type_end_index..])
                            .unwrap_or_default(),
                    )),
                    "filter" => {
                        let mut type_vec =
                            typed_param::parse_typed_query_param(&value[type_end_index..])
                                .unwrap_or_default();
                        let type_ = match type_vec.len() {
                            0 => Cow::Borrowed(""),
                            1 => type_vec.pop().unwrap(),
                            _ => Cow::Owned(
                                type_vec.join("."), // FIXME Try not to allocate more
                            ),
                        };
                        Ok(CibouletteQueryParametersField::FilterTyped(type_))
                    }
                    _ => Ok(CibouletteQueryParametersField::Meta(value)),
                }
            }
            None => Ok(CibouletteQueryParametersField::Meta(value)),
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
        Self::parse_str(Cow::Borrowed(value))
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::parse_str(Cow::Owned(value.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::parse_str(Cow::Owned(value))
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
