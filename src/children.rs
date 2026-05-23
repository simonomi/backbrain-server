use serde::{Deserialize, Serialize};
use crate::{node, syncable::{Syncable, SyncableAutoMerge, SyncableEvent, SyncableMergeCommit}};

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Children {
	ids: Vec<node::ID>
}

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildrenEvent {
	SetTo(Children)
}

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildrenMerge {
	SetTo(Children)
}

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildrenAutoMerge {
	SameChildren
}

impl Syncable for Children {
	type Event = ChildrenEvent;
	type Merge = ChildrenMerge;
	type AutoMerge = ChildrenAutoMerge;
}

impl SyncableEvent for ChildrenEvent {}

impl SyncableMergeCommit for ChildrenMerge {}

impl SyncableAutoMerge for ChildrenAutoMerge {}
