use super::*;

/// ## A `json:api` inner [link](https://jsonapi.org/format/#document-links) object
#[derive(Debug, Deserialize, Serialize, Getters, Clone, Default)]
#[getset(get = "pub")]
pub struct CibouletteLinkObj<'request> {
    pub href: Cow<'request, str>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub meta: BTreeMap<Cow<'request, str>, Value>,
}

/// ## A selector between simple or complex `json:api` [link](https://jsonapi.org/format/#document-links) inner object
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CibouletteLinkSelector<'request> {
    Simple(Cow<'request, str>),
    Obj(CibouletteLinkObj<'request>),
}

// // TODO BLOCKED See https://github.com/serde-rs/serde/issues/2016
// #[derive(Clone, Copy, Default)]
// struct CibouletteLinkSelectorVisitor;

// impl<'de> serde::de::Visitor<'de> for CibouletteLinkSelectorVisitor {
//     type Value = CibouletteLinkSelector<'de>;

//     #[inline]
//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "a string or a link object")
//     }

//     #[inline]
//     fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         Ok(CibouletteLinkSelector::Simple(v.into()))
//     }

//     #[inline]
//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         Ok(CibouletteLinkSelector::Simple(v.to_string().into()))
//     }

//     #[inline]
//     fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         Ok(CibouletteLinkSelector::Simple(v.into()))
//     }

//     #[inline]
//     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//     where
//         A: serde::de::MapAccess<'de>,
//     {
//         let mut meta: BTreeMap<Cow<'de, str>, Value> = BTreeMap::new();
//         let mut href: Option<Cow<'de, str>> = None;
//         while let Some((k, v)) = map.next_entry::<Cow<'de, str>, Value>()? {
//             if k.as_ref() == "href" {
//                 href = Some(k);
//             } else {
//                 meta.insert(k, v);
//             }
//         }
// 		match href
// 		{
// 			Some(href) => {
// 				Ok(CibouletteLinkSelector::Obj(CibouletteLinkObj {
// 					href,
// 					meta
// 				}))
// 			},
// 			None => {
// 				Err(serde::de::Error::missing_field("href"))
// 			}
// 		}
//     }
// }

// impl<'de> serde::Deserialize<'de> for CibouletteLinkSelector<'de> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::de::Deserializer<'de>,
//     {
//         deserializer.deserialize_any(CibouletteLinkSelectorVisitor::default())
//     }
// }

/// ## A `json:api` [link](https://jsonapi.org/format/#document-links) object
#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteLink<'request> {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<CibouletteLinkSelector<'request>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<CibouletteLinkSelector<'request>>,
}

/// ## A `json:api` top-level [link](https://jsonapi.org/format/#document-links) object with pagination support
#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteBodyPagination<'request> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<Cow<'request, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<Cow<'request, str>>,
}

/// ## A `json:api` top-level [link](https://jsonapi.org/format/#document-links) object with pagination support
#[derive(Debug, Deserialize, Serialize, Getters, Default, Clone)]
#[getset(get = "pub")]
#[serde(default)]
pub struct CibouletteBodyLink<'request> {
    #[serde(flatten)]
    pub inner_link: CibouletteLink<'request>,
    #[serde(flatten)]
    pub pagination: CibouletteBodyPagination<'request>,
}
