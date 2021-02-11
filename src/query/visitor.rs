use super::*;
use serde::de::{DeserializeSeed, Deserializer};
use std::fmt::Formatter;

const CIBOULETTE_QUERY_PARAMETERS_FIELDS: &[&str] = &[
    "include",
    "fields[*]",
    "sorting",
    "page",
    "page[*]",
    "filter",
    "filter[*]",
];

#[derive(Clone, Copy, Debug)]
pub struct CibouletteQueryParametersBuilderVisitor;

impl<'de> serde::de::Visitor<'de> for CibouletteQueryParametersBuilderVisitor {
    type Value = CibouletteQueryParametersBuilder<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "struct CibouletteResource")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut sparse: BTreeMap<Vec<&'de str>, Vec<&'de str>> = BTreeMap::new();
        let mut sorting: Vec<(CibouletteSortingDirection, Vec<&'de str>)> = Vec::new();
        let mut page: BTreeMap<CiboulettePageType<'de>, Cow<'de, str>> = BTreeMap::new();
        let mut filter_typed: BTreeMap<Cow<'de, str>, Cow<'de, str>> = BTreeMap::new();
        let mut meta: Vec<(Cow<'de, str>, Cow<'de, str>)> = Vec::new();
        let mut include: Option<Vec<Vec<Cow<'de, str>>>> = None;
        let mut filter: Option<Cow<'de, str>> = None;

        while let Some(key) =
            match serde::de::MapAccess::next_key::<CibouletteQueryParametersField>(&mut map) {
                Ok(val) => val,
                Err(err) => {
                    return Err(err);
                }
            }
        {
            match key {
                CibouletteQueryParametersField::Include => {
                    super::handle_ident_in_map_stateless(&mut include, &mut map, "include")?
                }
                CibouletteQueryParametersField::Sparse(type_) => {
                    if sparse
                        .insert(
                            type_,
                            explode_by_comma(&serde::de::MapAccess::next_value::<&'de str>(
                                &mut map,
                            )?),
                        )
                        .is_some()
                    {
                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                            "fields[<type>]",
                        ));
                    }
                }
                CibouletteQueryParametersField::Sorting => {
                    sorting = super::sorting::parse_sorting(serde::de::MapAccess::next_value::<
                        &'de str,
                    >(&mut map)?);
                }
                CibouletteQueryParametersField::Page(type_) => {
                    if page
                        .insert(
                            type_,
                            serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map)?,
                        )
                        .is_some()
                    {
                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                            "page[<type>]",
                        ));
                    }
                }
                CibouletteQueryParametersField::Filter => {
                    super::handle_ident_in_map_stateless(&mut filter, &mut map, "filter")?
                }
                CibouletteQueryParametersField::FilterTyped(type_) => {
                    if filter_typed
                        .insert(
                            type_,
                            serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map)?,
                        )
                        .is_some()
                    {
                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                            "filter[<type>]",
                        ));
                    }
                }
                CibouletteQueryParametersField::Meta(key) => {
                    meta.push((
                        key,
                        serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map)?,
                    ));
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

        Ok(CibouletteQueryParametersBuilder {
            include,
            sparse,
            filter,
            filter_typed,
            page,
            sorting,
            meta,
        })
    }
}

impl<'de> DeserializeSeed<'de> for CibouletteQueryParametersBuilderVisitor {
    type Value = CibouletteQueryParametersBuilder<'de>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "CibouletteQueryParametersField",
            CIBOULETTE_QUERY_PARAMETERS_FIELDS,
            CibouletteQueryParametersBuilderVisitor,
        )
    }
}
