// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `app.bsky.notification.updateSeen` namespace.

/// Notify server that the user has seen notifications.
#[async_trait::async_trait]
pub trait UpdateSeen: crate::xrpc::XrpcClient {
    async fn update_seen(&self, input: Input) -> Result<(), Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "app.bsky.notification.updateSeen",
            Option::<()>::None,
            Some(input),
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub seen_at: String,
}

pub enum Error {
}
