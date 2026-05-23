use serde::{Deserialize, Serialize};
use crate::syncable::{Syncable, SyncableAutoMerge, SyncableEvent, SyncableMergeCommit};

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Content {
	Text(String),
	Image,
	Other
}

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentEvent {
	SetTo(Content)
}

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentMerge {
	SetTo(Content)
}

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentAutoMerge {
	SameContent
}

impl Syncable for Content {
	type Event = ContentEvent;
	type Merge = ContentMerge;
	type AutoMerge = ContentAutoMerge;
}

impl SyncableEvent for ContentEvent {}

impl SyncableMergeCommit for ContentMerge {}

impl SyncableAutoMerge for ContentAutoMerge {}
