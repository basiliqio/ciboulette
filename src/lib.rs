#![warn(clippy::all)]
mod check;
mod config;
mod errors;
mod link;
mod relationship;
mod resource_identifier;
mod resource_obj;
mod resource_schema;
mod resource_type;

use either::Either;
use getset::{CopyGetters, Getters, MutGetters};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

pub use errors::CibouletteError;
pub use link::CibouletteLink;
pub use relationship::CibouletteRelationship;
pub use resource_identifier::{CibouletteResourceIdentifier, CibouletteResourceIdentifierSelector};
pub use resource_schema::{CibouletteResourceSchema, CibouletteResourceSchemaValue};
