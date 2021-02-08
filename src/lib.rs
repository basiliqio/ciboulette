#![warn(clippy::all)]
mod body;
mod config;
mod errors;
#[cfg(test)]
mod tests;

use body::resource_obj::CibouletteResourceBuilderVisitor;
use getset::{Getters, MutGetters};
use messy_json::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub use body::bag::CibouletteBag;
pub use body::errors_obj::{CibouletteErrorLink, CibouletteErrorObj, CibouletteErrorSource};
pub use body::link::{CibouletteLink, CibouletteLinkObj};
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
pub use config::CibouletteConfig;
pub use errors::{CibouletteClashDirection, CibouletteError};
