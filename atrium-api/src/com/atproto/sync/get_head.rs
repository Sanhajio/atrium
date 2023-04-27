// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.sync.getHead` namespace.

/// Gets the current HEAD CID of a repo.
#[async_trait::async_trait]
pub trait GetHead: crate::xrpc::XrpcClient {
    async fn get_head(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.sync.getHead",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    /// The DID of the repo.
    pub did: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub root: String,
}

pub enum Error {
}
