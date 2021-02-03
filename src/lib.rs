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
mod resource_type;
mod serde_utils;
mod top_level;

#[cfg(test)]
mod tests;

use either::Either;
use getset::{Getters, MutGetters};
use messy_json::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

pub use bag::CibouletteBag;
pub use config::CibouletteConfig;
pub use errors::CibouletteError;
pub use errors_obj::{CibouletteErrorLink, CibouletteErrorObj, CibouletteErrorSource};
pub use link::CibouletteLink;
pub use relationship::CibouletteRelationship;
pub use resource_identifier::{CibouletteResourceIdentifier, CibouletteResourceIdentifierSelector};
pub use resource_obj::{CibouletteResource, CibouletteResourceSelector};
pub use resource_type::CibouletteResourceType;
pub use top_level::CibouletteTopLevel;
