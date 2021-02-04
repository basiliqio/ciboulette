#![warn(clippy::all)]
mod bag;
mod check;
mod config;
mod errors;
mod errors_obj;
mod link;
mod relationship;
mod resource_identifier;
mod resource_obj;
mod resource_obj_selector;
mod resource_type;
mod serde_utils;
#[cfg(test)]
mod tests;
mod top_level;

use either::Either;
use getset::{Getters, MutGetters};
use messy_json::*;
use resource_obj::CibouletteResourceVisitor;
use resource_obj_selector::CibouletteResourceSelectorVisitor;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_utils::{handle_ident_in_map_stateful, handle_ident_in_map_stateless};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

pub use bag::CibouletteBag;
pub use config::CibouletteConfig;
pub use errors::CibouletteError;
pub use errors_obj::{CibouletteErrorLink, CibouletteErrorObj, CibouletteErrorSource};
pub use link::{CibouletteLink, CibouletteLinkObj};
pub use relationship::CibouletteRelationship;
pub use resource_identifier::{CibouletteResourceIdentifier, CibouletteResourceIdentifierSelector};
pub use resource_obj::CibouletteResource;
pub use resource_obj_selector::CibouletteResourceSelector;
pub use resource_type::CibouletteResourceType;
pub use top_level::CibouletteTopLevel;
