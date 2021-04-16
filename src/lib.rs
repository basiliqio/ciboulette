#![warn(clippy::all)]
mod body;
mod config;
mod error_request;
mod errors;
mod id;
mod inbound_request;
mod intention;
mod member_name;
mod outbound_request;
mod path;
mod query;
mod request_selector;
mod serde_utils;
pub mod store;

#[cfg(test)]
mod tests;

use arcstr::ArcStr;
use body::resource_obj::CibouletteResourceBuilderVisitor;
use getset::{Getters, MutGetters};
use messy_json::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_utils::{handle_ident_in_map_stateful, handle_ident_in_map_stateless};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use url::Url;
use uuid::Uuid;

pub use body::body_optional_data::CibouletteOptionalData;
pub use body::errors_obj::{CibouletteErrorLink, CibouletteErrorObj, CibouletteErrorSource};
pub use body::link::{
    CibouletteBodyLink, CibouletteBodyPagination, CibouletteLink, CibouletteLinkObj,
};
pub use body::relationship::{CibouletteRelationshipObject, CibouletteRelationshipObjectBuilder};
pub use body::request_body::{CibouletteBody, CibouletteBodyBuilder, CibouletteJsonApiVersion};
pub use body::request_body_data::{CibouletteBodyData, CibouletteBodyDataBuilder};
pub use body::resource_identifier::{
    CibouletteResourceIdentifier, CibouletteResourceIdentifierBuilder,
    CibouletteResourceIdentifierPermissive, CibouletteResourceIdentifierSelector,
    CibouletteResourceIdentifierSelectorBuilder,
};
pub use body::resource_obj::{CibouletteResource, CibouletteResourceBuilder};
pub use body::resource_obj_selector::{
    CibouletteResourceSelector, CibouletteResourceSelectorBuilder,
};
pub use body::resource_type::CibouletteResourceType;
pub use body::resource_type_builder::CibouletteResourceTypeBuilder;
pub use id::{CibouletteId, CibouletteIdBuilder, CibouletteIdType};
pub use intention::CibouletteIntention;
pub use outbound_request::request::CibouletteOutboundRequest;
pub use outbound_request::status::CibouletteResponseStatus;

pub use inbound_request::create::CibouletteCreateRequest;
pub use inbound_request::delete::CibouletteDeleteRequest;
pub use inbound_request::read::CibouletteReadRequest;
pub use inbound_request::update::{
    CibouletteUpdateRelationship, CibouletteUpdateRequest, CibouletteUpdateRequestType,
};
pub use outbound_request::element::CibouletteResponseElement;
pub use outbound_request::request_data_builder::CibouletteOutboundRequestDataBuilder;

pub use query::{
    CiboulettePageType, CibouletteQueryParameters, CibouletteQueryParametersBuilder,
    CibouletteSortingDirection, CibouletteSortingElement,
};
pub use request_selector::CibouletteRequestSelector;

pub use config::CibouletteConfig;
pub use error_request::CibouletteErrorRequest;
pub use errors::{CibouletteClashDirection, CibouletteError, CiboulettePathType};
pub use inbound_request::request::{
    CibouletteInboundRequest, CibouletteInboundRequestBuilder, CibouletteInboundRequestCommons,
};
pub use member_name::check_member_name;
pub use outbound_request::type_::{CibouletteResponseQuantity, CibouletteResponseRequiredType};

pub use path::path_container::{CiboulettePath, CiboulettePathBuilder};
pub use store::relationships_options::{
    CibouletteRelationshipManyToManyOption, CibouletteRelationshipOneToManyOption,
    CibouletteRelationshipOption,
};
pub use store::relationships_options_builder::{
    CibouletteRelationshipManyToManyOptionBuilder, CibouletteRelationshipOneToManyOptionBuilder,
    CibouletteRelationshipOptionBuilder,
};
pub use store::{CibouletteStore, CibouletteStoreBuilder};
