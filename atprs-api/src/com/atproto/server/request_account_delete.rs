// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.server.requestAccountDelete` namespace.

/// Initiate a user account deletion via email.
pub trait RequestAccountDelete {
    fn request_account_delete(&self) -> Result<(), Error>;
}

pub enum Error {
}
