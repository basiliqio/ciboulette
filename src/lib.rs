#![warn(clippy::all)]
mod check;
mod config;
mod errors;
mod link;
mod relationship;
mod resource_identifier;
mod resource_obj;
mod resource_type;

#[cfg(test)]
mod tests;

use either::Either;
use getset::{Getters, MutGetters};
use messy_json::*;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

pub use config::CibouletteConfig;
pub use errors::CibouletteError;
pub use link::CibouletteLink;
pub use relationship::CibouletteRelationship;
pub use resource_identifier::{CibouletteResourceIdentifier, CibouletteResourceIdentifierSelector};
pub use resource_obj::CibouletteResourceObject;
pub use resource_type::CibouletteResourceType;
