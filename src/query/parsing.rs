use super::*;
use serde::de::Visitor;
use std::fmt::Formatter;


#[derive(Debug)]
pub enum CibouletteSortingDirection {
    Asc,
	Desc,
}

#[derive(Clone, Copy, Debug)]
pub struct CibouletteQueryParametersBuilderVisitor;
pub struct CibouletteQueryParametersFieldVisitor;

enum CibouletteQueryParametersField<'a> {
    Include,
    Sparse(Vec<Cow<'a, str>>),
    Sorting,
    Page,
    Filter,
    Meta,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CibouletteQueryParametersBuilder<'a> {
    include: Vec<Vec<Cow<'a, str>>>,
	sparse: BTreeMap<Cow<'a, str>, Vec<Cow<'a, str>>>,
	sorting: Vec<(CibouletteSortingDirection, Vec<Cow<'a, str>>)>,
	page: Option<Cow<'a, str>>,
	filter: Option<Cow<'a, str>>,
	meta: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> CibouletteQueryParametersBuilder<'a> {
    pub fn deserialize<R>(d: &mut serde_json::Deserializer<R>) -> Result<Self, serde_json::Error>
    where
        R: serde_json::de::Read<'a>,
    {
        let visitor = CibouletteQueryParametersBuilderVisitor;

        visitor.deserialize(d)
    }
}

impl<'de> Visitor<'de> for CibouletteQueryParametersFieldVisitor {
    type Value = CibouletteQueryParametersField<'de>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        Formatter::write_str(formatter, "field identifier")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let preemptive_val = match value {
            "include" => Some(CibouletteQueryParametersField::Include),
            "sort" => Some(CibouletteQueryParametersField::Sorting),
            "page" => Some(CibouletteQueryParametersField::Page),
            "filter" => Some(CibouletteQueryParametersField::Filter),
            _ => None,
        };
		if let Some(preemptive_val) = preemptive_val
		{
			return Ok(preemptive_val);
		}
    }
}

// impl<'de> serde::Deserialize<'de> for CibouletteResourceField {
//     #[inline]
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         serde::Deserializer::deserialize_identifier(deserializer, CibouletteResourceFieldVisitor)
//     }
// }

// impl<'de> serde::de::Visitor<'de> for CibouletteResourceBuilderVisitor {
//     type Value = CibouletteResourceBuilder<'de>;

//     #[inline]
//     fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
//         Formatter::write_str(formatter, "struct CibouletteResource")
//     }

//     #[inline]
//     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//     where
//         A: serde::de::MapAccess<'de>,
//     {
//         let mut id: Option<Cow<'de, str>> = None;
//         let mut type_: Option<Cow<'de, str>> = None;
//         let mut meta: Option<Value> = None;
//         let mut attributes: Option<&'de RawValue> = None;
//         let mut relationships: Option<HashMap<Cow<'de, str>, CibouletteRelationship<'de>>> = None;
//         let mut links: Option<CibouletteLink<'de>> = None;
//         while let Some(key) =
//             match serde::de::MapAccess::next_key::<CibouletteResourceField>(&mut map) {
//                 Ok(val) => val,
//                 Err(err) => {
//                     return Err(err);
//                 }
//             }
//         {
//             match key {
//                 CibouletteResourceField::Id => {
//                     super::handle_ident_in_map_stateless(&mut id, &mut map, "id")?
//                 }
//                 CibouletteResourceField::Type => {
//                     super::handle_ident_in_map_stateless(&mut type_, &mut map, "type")?
//                 }
//                 CibouletteResourceField::Meta => {
//                     super::handle_ident_in_map_stateless(&mut meta, &mut map, "meta")?
//                 }
//                 CibouletteResourceField::Attributes => {
//                     super::handle_ident_in_map_stateless(&mut attributes, &mut map, "attributes")?
//                 }
//                 CibouletteResourceField::Relationships => super::handle_ident_in_map_stateless(
//                     &mut relationships,
//                     &mut map,
//                     "relationships",
//                 )?,
//                 CibouletteResourceField::Links => {
//                     super::handle_ident_in_map_stateless(&mut links, &mut map, "links")?
//                 }
//                 _ => {
//                     let _ =
//                         match serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut map) {
//                             Ok(val) => val,
//                             Err(err) => {
//                                 return Err(err);
//                             }
//                         };
//                 }
//             }
//         }

//         let id = id.ok_or_else(|| <A::Error as serde::de::Error>::missing_field("id"))?;
//         let type_ = type_.ok_or_else(|| <A::Error as serde::de::Error>::missing_field("type"))?;
//         let relationships = relationships.unwrap_or_default();
//         Ok(CibouletteResourceBuilder {
//             identifier: CibouletteResourceIdentifier::new(id, type_, meta.unwrap_or_default()),
//             attributes,
//             relationships,
//             links,
//         })
//     }
// }
