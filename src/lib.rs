//! # Introduction
//!
//! **_Ciboulette_** is a **`JSON:API`** library.
//!
//! It allows one to parse request and build response respecting the `JSON:API`
//! [specifications](https://jsonapi.org/format/).
//!
//! It aims to have a low memory footprint and be **fast**.
//!
//! # High level view of components
//!
//! At a high level, an `API` is constitued of [resource types](CibouletteResourceType). The resource type are organized
//! in a graph representing their [relationships](CibouletteResourceRelationshipDetails) as edges.
//!
//! In addition to the graph, an adgacent map is used to efficiently retrieve [resource types](CibouletteResourceType)
//! by their alias. This whole structure is held in a [store](CibouletteStore).
//!
//! ## Resource types
//!
//! The [resource types](CibouletteResourceType) can be built using a [resource type builder](CibouletteResourceTypeBuilder).
//! It's made of :
//!
//! - A name, that will later be used as an alias to fetch the [resource types](CibouletteResourceType) from the [store](CibouletteStore)'s graph.
//! - A id type, which will be used to deserialize the ids the requests.
//! - A [schema](messy_json::MessyJsonObject) which will be used to deserialize the body of the requests and serialize the response.
//!
//! ## Relationship options
//!
//! ### Many-to-Many
//!
//! The [option struct](CibouletteRelationshipManyToManyOption) map a resource "A" to another resource "C" through another resource "B" (bucket)
//!
//! ```ascii
//!     Resource A                      Resource B (bucket)                    Resource C
//! ┌─────────────────┐  ┌─────────────────────────────────────────────┐  ┌─────────────────┐
//! │                 │  │                                             │  │                 │
//! │  peoples──►id───┼──┼──►people_id◄──people-article──►article_id◄──┼──┼──id◄──articles  │
//! │                 │  │                                             │  │                 │
//! └─────────────────┘  └─────────────────────────────────────────────┘  └─────────────────┘
//! ```
//!
//! When creating a Many-to-Many relationships (`A <-> C`), we'll also create a Many-to-One relationship between the table
//! `A -> B`, `C -> B`, `B -> A` and `B -> C` so that we can reference the relationship directly.
//!
//! ### One-to-Many / Many-to-One
//!
//! The [option struct](CibouletteRelationshipOneToManyOption) map a "many" resource to a "one" resource.
//!
//! ```ascii
//!                     Many table                                          One table
//! ┌──────────────────────────────────────────────────┐  ┌──────────────────────────────────────┐
//! │                                                  │  │                                      │
//! │ many_table_element_0──────────►many_table_key_0──┼──┼──►one_table_id◄───one_table_element  │
//! │                                                  │  │     ▲      ▲                         │
//! │                                                  │  │     │      │                         │
//! │ many_table_element_1──────────►many_table_key_1──┼──┼─────┘      │                         │
//! │                                                  │  │            │                         │
//! │                                                  │  │            │                         │
//! │ many_table_element_2──────────►many_table_key_2──┼──┼────────────┘                         │
//! │                                                  │  │                                      │
//! │                                                  │  └──────────────────────────────────────┘
//! └──────────────────────────────────────────────────┘
//! ```
//!
//! In the option a field is used to determined if a Many-to-One/One-to-Many relationship is part of Many-to-Many relationship.
//!
//! ## Requests
//!
//! Every requests boils down to the same components. But there is some quirks :
//!
//! - Creation request can be valid without resource [id](CibouletteResourceIdentifierPermissive),
//! - Update request can have a body of [resource identifier](CibouletteResourceIdentifier).
//!
//! Every requests must first be deserialized using the [request builder](CibouletteRequestBuilder). Then it can be built
//! into an generic [request](CibouletteRequest). From that, one can convert to the desired request type depending on the
//! [intention](CibouletteIntention). Trying to convert a generic [request](CibouletteRequest) to an incompatible sub-type
//! will result in an [error](CibouletteError). The correct conversion map goes like this :
//!
//! | Intention                             | Request type                              |
//! |---------------------------------------|-------------------------------------------|
//! | [Create](CibouletteIntention::Create) | [Create request](CibouletteCreateRequest) |
//! | [Read](CibouletteIntention::Read)     | [Read request](CibouletteReadRequest)     |
//! | [Update](CibouletteIntention::Update) | [Update request](CibouletteUpdateRequest) |
//! | [Delete](CibouletteIntention::Delete) | [Delete request](CibouletteDeleteRequest) |
//!
//! Every sub-type of requests implement a [common trait](CibouletteRequestCommons) to allow for genericity.
//!
//! ## Responses
//!
//! A response is built from a [request](CibouletteRequestCommons) and a list of [response element](CibouletteResponseElement).
//!
//! Depending on the [request](CibouletteRequestCommons), the [response](CibouletteResponse) will be built to the correct format.
//!
//! ### Response elements
//!
//! Each response should have a single main [resource type](CibouletteResourceType).
//!
//!
//! The [response elements](CibouletteResponseElement) are composed as follow for an element part of the main [resource type](CibouletteResourceType):
//!
//! | Response element field | Always required                                        | Description                                                                          |
//! |------------------------|:------------------------------------------------------:|--------------------------------------------------------------------------------------|
//! | `type`                 | ✅                                                     | The current element [resource type](CibouletteResourceType)                          |
//! | `identifier`           | ✅                                                     | The current element [resource identifier](CibouletteResourceIdentifier)              |
//! | `data`                 | ❌                                                     | The `JSON` data of the resource, if any                                              |
//! | `related`.`rel_chain`  | ❌<br />(only for related data)                        | Chain of relation metadata from the main [resource type](CibouletteResourceType)     |
//! | `related`.`element`    | ❌<br />(only for related data)                        | The [resource identifier](CibouletteResourceIdentifier) of the element it relates to |
//!
//!
//!
//!
//!
//!
//!
//!

#![warn(clippy::all)]

#[macro_use]
mod macros;
mod body;
mod config;
mod error_request;
mod errors;
mod id;
mod intention;
mod member_name;
mod path;
mod query;
mod request_selector;
mod requests;
mod responses;
mod selector;
mod serde_utils;
mod store;

#[cfg(test)]
mod tests;

use arcstr::ArcStr;
use getset::{Getters, MutGetters};
use messy_json::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use url::Url;
use uuid::Uuid;

pub use body::body_optional_data::CibouletteOptionalData;
pub use body::errors_obj::{CibouletteErrorLink, CibouletteErrorObj, CibouletteErrorSource};
pub use body::link::{
    CibouletteBodyLink, CibouletteBodyPagination, CibouletteLink, CibouletteLinkObj,
    CibouletteLinkSelector,
};
pub use body::relationship::{CibouletteRelationshipObject, CibouletteRelationshipObjectBuilder};
pub use body::request_body::{CibouletteBody, CibouletteBodyBuilder, CibouletteJsonApiVersion};
pub use body::request_body_data::{
    CibouletteBodyData, CibouletteBodyDataBuilder, CibouletteBodyDataPermissive,
};
pub use body::resource_identifier::{
    CibouletteResourceIdentifier, CibouletteResourceIdentifierBuilder,
    CibouletteResourceIdentifierPermissive, CibouletteResourceIdentifierSelector,
};
pub use body::resource_obj::{CibouletteResource, CibouletteResourceBuilder};
pub use body::resource_obj_selector::{
    CibouletteResourceSelector, CibouletteResourceSelectorBuilder,
};
pub use body::resource_type::{CibouletteResourceRelationshipDetails, CibouletteResourceType};
pub use body::resource_type_builder::CibouletteResourceTypeBuilder;
pub use id::{CibouletteId, CibouletteIdSelector, CibouletteIdType, CibouletteIdTypeSelector};
pub use intention::CibouletteIntention;
pub use responses::request::CibouletteResponse;
pub use responses::status::CibouletteResponseStatus;

pub use query::{
    CiboulettePageType, CibouletteQueryParameters, CibouletteQueryParametersBuilder,
    CibouletteSortingDirection, CibouletteSortingElement,
};
pub use request_selector::CibouletteRequestSelector;
pub use requests::create::CibouletteCreateRequest;
pub use requests::delete::CibouletteDeleteRequest;
pub use requests::read::CibouletteReadRequest;
pub use requests::update::{
    CibouletteUpdateRelationshipBody, CibouletteUpdateRequest, CibouletteUpdateRequestType,
};
pub use responses::body::{
    CibouletteResponseBody, CibouletteResponseBodyData, CibouletteResponseRelationshipObject,
    CibouletteResponseResource, CibouletteResponseResourceSelector,
};
pub use responses::element::CibouletteResponseElement;
pub use responses::element_identifier::{
    CibouletteResourceResponseIdentifier, CibouletteResourceResponseIdentifierBuilder,
    CibouletteResourceResponseIdentifierSelector,
    CibouletteResourceResponseIdentifierSelectorBuilder,
};
pub use responses::request_data_builder::CibouletteResponseDataBuilder;

pub use config::CibouletteConfig;
pub use error_request::CibouletteErrorRequest;
pub use errors::{CibouletteClashDirection, CibouletteError, CiboulettePathType};
pub use member_name::check_member_name;
pub use requests::request::{
    CibouletteRequest, CibouletteRequestBuilder, CibouletteRequestCommons,
};
pub use responses::type_::{CibouletteResponseQuantity, CibouletteResponseRequiredType};

pub use path::path_container::{CiboulettePath, CiboulettePathBuilder};
pub use selector::{CibouletteSelector, CibouletteSelectorError, CibouletteSelectorIterator};
pub use store::{
    CiboulettePaginationEncoding, CibouletteResourceTypeConfiguration,
    CibouletteResourceTypePaginationConfiguration,
    CibouletteResourceTypePaginationConfigurationBuilder,
};
pub use store::{
    CibouletteRelationshipManyToManyOption, CibouletteRelationshipManyToManyOptionBuilder,
    CibouletteRelationshipOneToManyOption, CibouletteRelationshipOneToManyOptionBuilder,
    CibouletteRelationshipOption, CibouletteRelationshipOptionBuilder, CibouletteStore,
    CibouletteStoreBuilder,
};

#[cfg(test)]
pub mod test_helper;

#[cfg(not(test))]
#[cfg(feature = "test_utils")]
pub mod test_helper;
