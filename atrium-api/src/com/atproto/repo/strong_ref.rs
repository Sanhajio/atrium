// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.repo.strongRef` namespace.
//! A URI with a content-hash fingerprint.

// com.atproto.repo.strongRef
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub cid: String,
    pub uri: String,
}
