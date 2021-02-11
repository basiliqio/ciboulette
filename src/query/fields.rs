use super::*;
use serde::de::Visitor;
use std::fmt::Formatter;

pub struct CibouletteQueryParametersFieldVisitor;

pub enum CibouletteQueryParametersField<'a> {
    Include,
    Sparse(Vec<&'a str>),
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
        let preemptive_val = match value {
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
                            typed_param::parse_typed_query_param(&value).unwrap_or_default();
                        let page_type: Cow<'de, str> = match page_type_vec.len() {
                            0 => Cow::Borrowed(""),
                            1 => Cow::Borrowed(page_type_vec.pop().unwrap()),
                            _ => Cow::Owned(
                                page_type_vec.join("."), // FIXME Try not to allocate more
                            ),
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
                        typed_param::parse_typed_query_param(value).unwrap_or_default(),
                    )),
                    "filter" => {
                        let mut type_vec =
                            typed_param::parse_typed_query_param(value).unwrap_or_default();
                        let type_ = match type_vec.len() {
                            0 => Cow::Borrowed(""),
                            1 => Cow::Borrowed(type_vec.pop().unwrap()),
                            _ => Cow::Owned(
                                type_vec.join("."), // FIXME Try not to allocate more
                            ),
                        };
                        Ok(CibouletteQueryParametersField::FilterTyped(type_))
                    }
                    _ => Ok(CibouletteQueryParametersField::Meta(Cow::Borrowed(value))),
                }
            }
            None => Ok(CibouletteQueryParametersField::Meta(Cow::Borrowed(value))),
        }
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
