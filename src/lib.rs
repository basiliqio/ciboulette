#![warn(clippy::all)]
mod body;
mod config;
mod errors;
mod inbound_request;
mod member_name_regex;
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

pub use body::bag::CibouletteBag;
pub use body::errors_obj::{CibouletteErrorLink, CibouletteErrorObj, CibouletteErrorSource};
pub use body::link::{
    CibouletteLink, CibouletteLinkObj, CibouletteTopLevelLink, CibouletteTopLevelPagination,
};
pub use body::relationship::CibouletteRelationship;
pub use body::resource_identifier::{
    CibouletteResourceIdentifier, CibouletteResourceIdentifierSelector,
};
pub use body::resource_obj::{CibouletteResource, CibouletteResourceBuilder};
pub use body::resource_obj_selector::{
    CibouletteResourceSelector, CibouletteResourceSelectorBuilder,
};
pub use body::resource_type::CibouletteResourceType;
pub use body::top_level::{CibouletteTopLevel, CibouletteTopLevelBuilder};

pub use query::{
    CiboulettePageType, CibouletteQueryParameters, CibouletteQueryParametersBuilder,
    CibouletteSortingDirection, CibouletteSortingElement,
};

pub use config::CibouletteConfig;
pub use errors::{CibouletteClashDirection, CibouletteError};
pub use member_name_regex::check_member_name;
