// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `app.bsky.embed.images` namespace.
//! A set of images embedded in some other form of content

// app.bsky.embed.images
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub images: Vec<Image>,
}

// app.bsky.embed.images#image
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub alt: String,
    // pub image: ...,
}

// app.bsky.embed.images#view
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub images: Vec<ViewImage>,
}

// app.bsky.embed.images#viewImage
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewImage {
    pub alt: String,
    pub fullsize: String,
    pub thumb: String,
}
