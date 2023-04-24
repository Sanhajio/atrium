// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.repo.uploadBlob` namespace.

/// Upload a new blob to be added to repo in a later request.
pub trait UploadBlob {
    fn upload_blob(&self) -> Result<Output, Error>;
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    // pub blob: ...,
}

pub enum Error {
}
