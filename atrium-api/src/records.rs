// This file is generated by atrium-codegen. Do not edit.
//! Collection of ATP repository record type

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum Record {
    #[serde(rename = "app.bsky.actor.profile")]
    AppBskyActorProfile(crate::app::bsky::actor::profile::Record),
    #[serde(rename = "app.bsky.feed.like")]
    AppBskyFeedLike(crate::app::bsky::feed::like::Record),
    #[serde(rename = "app.bsky.feed.post")]
    AppBskyFeedPost(crate::app::bsky::feed::post::Record),
    #[serde(rename = "app.bsky.feed.repost")]
    AppBskyFeedRepost(crate::app::bsky::feed::repost::Record),
    #[serde(rename = "app.bsky.graph.follow")]
    AppBskyGraphFollow(crate::app::bsky::graph::follow::Record),
}
