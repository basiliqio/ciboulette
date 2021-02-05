#![warn(clippy::all)]
mod body;
mod config;
mod errors;
#[cfg(test)]
mod tests;

use body::resource_obj::CibouletteResourceVisitor;
use body::resource_obj_selector::CibouletteResourceSelectorVisitor;
use either::Either;
use getset::{Getters, MutGetters};
use messy_json::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

pub use body::bag::CibouletteBag;
pub use body::errors_obj::{CibouletteErrorLink, CibouletteErrorObj, CibouletteErrorSource};
pub use body::link::{CibouletteLink, CibouletteLinkObj};
pub use body::relationship::CibouletteRelationship;
pub use body::resource_identifier::{
    CibouletteResourceIdentifier, CibouletteResourceIdentifierSelector,
};
pub use body::resource_obj::CibouletteResource;
pub use body::resource_obj_selector::CibouletteResourceSelector;
pub use body::resource_type::CibouletteResourceType;
pub use body::top_level::CibouletteTopLevel;
pub use config::CibouletteConfig;
pub use errors::CibouletteError;
