use super::*;

pub mod create_request_body;
pub mod delete_request_body;
pub mod errors_obj;
pub mod link;
pub mod read_request_body;
pub mod relationship;
pub mod request_body;
pub mod request_body_selector;
pub mod resource_identifier;
pub mod resource_obj;
pub mod resource_obj_selector;
pub mod resource_type;
pub mod store;
pub mod update_request_body;

#[cfg(test)]
mod tests;
