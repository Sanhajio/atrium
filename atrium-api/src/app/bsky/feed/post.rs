// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.post` namespace.

// app.bsky.feed.post
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Box<RecordEmbedEnum>>,
    /// Deprecated: replaced by app.bsky.richtext.facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
    pub text: String,
}

// app.bsky.feed.post#entity
/// Deprecated: use facets instead.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub index: TextSlice,
    /// Expected values are 'mention' and 'link'.
    pub r#type: String,
    pub value: String,
}

// app.bsky.feed.post#replyRef
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplyRef {
    pub parent: crate::com::atproto::repo::strong_ref::Main,
    pub root: crate::com::atproto::repo::strong_ref::Main,
}

// app.bsky.feed.post#textSlice
/// Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextSlice {
    pub end: i32,
    pub start: i32,
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum RecordEmbedEnum {
    #[serde(rename = "app.bsky.embed.images")]
    AppBskyEmbedImagesMain(crate::app::bsky::embed::images::Main),
    #[serde(rename = "app.bsky.embed.external")]
    AppBskyEmbedExternalMain(crate::app::bsky::embed::external::Main),
    #[serde(rename = "app.bsky.embed.record")]
    AppBskyEmbedRecordMain(crate::app::bsky::embed::record::Main),
    #[serde(rename = "app.bsky.embed.recordWithMedia")]
    AppBskyEmbedRecordWithMediaMain(crate::app::bsky::embed::record_with_media::Main),
}
