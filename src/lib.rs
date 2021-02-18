#![warn(clippy::all)]
mod body;
mod config;
mod create_request;
mod delete_request;
mod errors;
mod inbound_request;
mod member_name;
mod path;
mod query;
mod read_request;
mod request_selector;
mod serde_utils;
mod update_request;

#[cfg(test)]
mod tests;

use body::resource_obj::CibouletteResourceBuilderVisitor;
use getset::{Getters, MutGetters};
use messy_json::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_utils::{handle_ident_in_map_stateful, handle_ident_in_map_stateless};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::convert::{TryFrom, TryInto};
use url::Url;

pub use body::errors_obj::{CibouletteErrorLink, CibouletteErrorObj, CibouletteErrorSource};
pub use body::link::{
    CibouletteBodyLink, CibouletteBodyPagination, CibouletteLink, CibouletteLinkObj,
};
pub use body::relationship::{
    CibouletteRelationshipBucket, CibouletteRelationshipObject, CibouletteRelationshipOption,
};
pub use body::request_body::{CibouletteBody, CibouletteBodyBuilder};
pub use body::resource_identifier::{
    CibouletteResourceIdentifier, CibouletteResourceIdentifierPermissive,
    CibouletteResourceIdentifierSelector,
};
pub use body::resource_obj::{CibouletteResource, CibouletteResourceBuilder};
pub use body::resource_obj_selector::{
    CibouletteResourceSelector, CibouletteResourceSelectorBuilder,
};
pub use body::resource_type::CibouletteResourceType;
pub use body::store::CibouletteStore;

pub use create_request::CibouletteCreateRequest;
pub use delete_request::CibouletteDeleteRequest;
pub use query::{
    CiboulettePageType, CibouletteQueryParameters, CibouletteQueryParametersBuilder,
    CibouletteSortingDirection, CibouletteSortingElement,
};
pub use read_request::CibouletteReadRequest;
pub use request_selector::CibouletteRequestSelector;
pub use update_request::CibouletteUpdateRequest;

pub use config::CibouletteConfig;
pub use errors::{CibouletteClashDirection, CibouletteError};
pub use inbound_request::{CibouletteIntention, CibouletteRequest, CibouletteRequestBuilder};
pub use member_name::check_member_name;

pub use path::path_container::{CiboulettePath, CiboulettePathBuilder};
