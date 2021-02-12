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

impl CibouletteQueryParametersBuilderVisitor {
    /// Handle the 'include' parameter when parsing query parameters
    #[inline]
    fn build_include<'de, A>(
        mut map: &mut A,
        res: &mut Option<Vec<Vec<Cow<'de, str>>>>,
    ) -> Result<(), A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let include_str: Option<Cow<'de, str>>;
        if res.is_some()
        // Check if already defined
        {
            return Err(serde::de::Error::duplicate_field("include"));
        }
        include_str = Some(serde::de::MapAccess::next_value(&mut map)?); // Parse parameter value
        *res = include_str.map(|x| {
            x.split(',') // Split by ','
                .map(|x| x.split('.').map(str::to_string).map(Cow::Owned).collect()) // Split by '.'
                .collect()
        });
        Ok(())
    }

    /// Handle the 'fields[*]' parameter when parsing query parameters
    #[inline]
    fn build_sparse<'de, A>(
        mut map: &mut A,
        type_: Vec<Cow<'de, str>>,
        res: &mut BTreeMap<Vec<Cow<'de, str>>, Vec<Cow<'de, str>>>,
    ) -> Result<(), A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        if res
            .insert(
                type_,
                serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map)?
                    .split(',')
                    .map(str::to_string)
                    .map(Cow::Owned)
                    .collect(),
            )
            .is_some()
        {
            return Err(<A::Error as serde::de::Error>::duplicate_field(
                "fields[<type>]",
            ));
        }
        Ok(())
    }

    /// Handle the 'sort[*]' parameter when parsing query parameters
    #[inline]
    fn build_sort<'de, A>(
        mut map: &mut A,
        res: &mut Vec<(CibouletteSortingDirection, Cow<'de, str>)>,
    ) -> Result<(), A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        if !res.is_empty() {
            // FIXME If the first `sort` parameter didn't yield any values, this will evaluate as true
            return Err(serde::de::Error::duplicate_field("sort"));
        }
        *res = super::sorting::parse_sorting(&serde::de::MapAccess::next_value::<Cow<'de, str>>(
            &mut map,
        )?);
        Ok(())
    }

    /// Handle the 'page[*]' parameter when parsing query parameters
    #[inline]
    fn build_page<'de, A>(
        mut map: &mut A,
        type_: CiboulettePageType<'de>,
        res: &mut BTreeMap<CiboulettePageType<'de>, Cow<'de, str>>,
    ) -> Result<(), A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        if res
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
        Ok(())
    }

    /// Handle the 'filter[*]' parameter when parsing query parameters
    #[inline]
    fn build_filter_typed<'de, A>(
        mut map: &mut A,
        type_: Cow<'de, str>,
        res: &mut BTreeMap<Cow<'de, str>, Cow<'de, str>>,
    ) -> Result<(), A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        if res
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
        Ok(())
    }
}

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
        let mut sparse: BTreeMap<Vec<Cow<'de, str>>, Vec<Cow<'de, str>>> = BTreeMap::new();
        let mut sorting: Vec<(CibouletteSortingDirection, Cow<'de, str>)> = Vec::new();
        let mut page: BTreeMap<CiboulettePageType<'de>, Cow<'de, str>> = BTreeMap::new();
        let mut filter_typed: BTreeMap<Cow<'de, str>, Cow<'de, str>> = BTreeMap::new();
        let mut meta: BTreeMap<Cow<'de, str>, Cow<'de, str>> = BTreeMap::new();
        let mut include: Option<Vec<Vec<Cow<'de, str>>>> = None;
        let mut filter: Option<Cow<'de, str>> = None;

        while let Some(key) =
            match serde::de::MapAccess::next_key::<CibouletteQueryParametersField>(&mut map) {
                // Parse the next parameter key
                Ok(val) => val,
                Err(err) => {
                    return Err(err);
                }
            }
        {
            match key {
                CibouletteQueryParametersField::Include => {
                    Self::build_include(&mut map, &mut include)?
                }
                CibouletteQueryParametersField::Sparse(type_) => {
                    Self::build_sparse(&mut map, type_, &mut sparse)?
                }
                CibouletteQueryParametersField::Sorting => {
                    Self::build_sort(&mut map, &mut sorting)?
                }
                CibouletteQueryParametersField::Page(type_) => {
                    Self::build_page(&mut map, type_, &mut page)?
                }
                CibouletteQueryParametersField::Filter => {
                    super::handle_ident_in_map_stateless(&mut filter, &mut map, "filter")?
                }
                CibouletteQueryParametersField::FilterTyped(type_) => {
                    Self::build_filter_typed(&mut map, type_, &mut filter_typed)?
                }
                CibouletteQueryParametersField::Meta(key) => {
                    meta.insert(
                        key,
                        serde::de::MapAccess::next_value::<Cow<'de, str>>(&mut map)?,
                    );
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
