#![warn(clippy::all)]
mod body;
mod config;
mod errors;
mod inbound_request;
mod member_name;
mod query;
mod serde_utils;

use body::resource_obj::CibouletteResourceBuilderVisitor;
use getset::{Getters, MutGetters};
use messy_json::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_utils::{handle_ident_in_map_stateful, handle_ident_in_map_stateless};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap};

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

pub use query::{
    CiboulettePageType, CibouletteQueryParameters, CibouletteQueryParametersBuilder,
    CibouletteSortingDirection, CibouletteSortingElement,
};

pub use body::create_request_body::CibouletteCreateRequestBody;
pub use body::delete_request_body::CibouletteDeleteRequestBody;
pub use body::read_request_body::CibouletteReadRequestBody;
pub use body::update_request_body::CibouletteUpdateRequestBody;

pub use config::CibouletteConfig;
pub use errors::{CibouletteClashDirection, CibouletteError};
pub use inbound_request::{CibouletteIntention, CibouletteRequest, CibouletteRequestBuilder};
pub use member_name::check_member_name;
