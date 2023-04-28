// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.updateAccountHandle` namespace.

/// Administrative action to update an account's handle
#[async_trait::async_trait]
pub trait UpdateAccountHandle: crate::xrpc::XrpcClient {
    async fn update_account_handle(&self, input: Input) -> Result<(), Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send_unit(
            self,
            http::Method::POST,
            "com.atproto.admin.updateAccountHandle",
            Option::<()>::None,
            Some(input),
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub did: String,
    pub handle: String,
}

pub enum Error {
}