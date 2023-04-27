// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.getLikes` namespace.

#[async_trait::async_trait]
pub trait GetLikes: crate::xrpc::XrpcClient {
    async fn get_likes(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "app.bsky.feed.getLikes",
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
    pub cid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    pub uri: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub likes: Vec<Like>,
    pub uri: String,
}

pub enum Error {
}

// app.bsky.feed.getLikes#like
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    pub actor: crate::app::bsky::actor::defs::ProfileView,
    pub created_at: String,
    pub indexed_at: String,
}
