// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `app.bsky.notification.getUnreadCount` namespace.

#[async_trait::async_trait]
pub trait GetUnreadCount: crate::xrpc::XrpcClient {
    async fn get_unread_count(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "app.bsky.notification.getUnreadCount",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub count: i32,
}

pub enum Error {
}
