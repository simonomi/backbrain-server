use std::hash::Hash;
use serde::{Deserialize, Serialize};

pub trait Syncable: Hash + Eq + Clone {
	type Event: SyncableEvent;
	type Merge: SyncableMergeCommit;
	type AutoMerge: SyncableAutoMerge;
}

pub trait SyncableEvent: Hash + Eq + Serialize + for<'a> Deserialize<'a> + Clone {}

pub trait SyncableMergeCommit: Hash + Eq + Serialize + for<'a> Deserialize<'a> + Clone {}

pub trait SyncableAutoMerge: Hash + Eq + Serialize + for<'a> Deserialize<'a> + Clone {}
