// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.resolveModerationReports` namespace.

/// Resolve moderation reports by an action.
pub trait ResolveModerationReports {
    fn resolve_moderation_reports(&self, input: Input) -> Result<Output, Error>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub action_id: i32,
    pub created_by: String,
    pub report_ids: Vec<i32>,
}

pub type Output = crate::com::atproto::admin::defs::ActionView;

pub enum Error {
}
