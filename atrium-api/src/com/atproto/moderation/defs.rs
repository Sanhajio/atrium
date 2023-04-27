// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.moderation.defs` namespace.

// com.atproto.moderation.defs#reasonMisleading
/// Misleading identity, affiliation, or content
pub struct ReasonMisleading;

// com.atproto.moderation.defs#reasonOther
/// Other: reports not falling under another report category
pub struct ReasonOther;

// com.atproto.moderation.defs#reasonRude
/// Rude, harassing, explicit, or otherwise unwelcoming behavior
pub struct ReasonRude;

// com.atproto.moderation.defs#reasonSexual
/// Unwanted or mis-labeled sexual content
pub struct ReasonSexual;

// com.atproto.moderation.defs#reasonSpam
/// Spam: frequent unwanted promotion, replies, mentions
pub struct ReasonSpam;

// com.atproto.moderation.defs#reasonType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ReasonType;

// com.atproto.moderation.defs#reasonViolation
/// Direct violation of server rules, laws, terms of service
pub struct ReasonViolation;
