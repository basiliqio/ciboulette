use super::*;

pub mod bag;
pub mod check;
pub mod errors_obj;
pub mod link;
pub mod relationship;
pub mod resource_identifier;
pub mod resource_obj;
pub mod resource_obj_selector;
pub mod resource_type;
pub mod serde_utils;
pub mod top_level;

#[cfg(test)]
mod tests;

use serde_utils::{handle_ident_in_map_stateful, handle_ident_in_map_stateless};
