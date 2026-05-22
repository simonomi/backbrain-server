use std::hash::Hash;
use serde::{Deserialize, Serialize};

pub trait Syncable: Hash + Eq {
	type Event: SyncableEvent;
	type Merge: SyncableMergeCommit;
	type AutoMerge: SyncableAutoMerge;
}

pub trait SyncableEvent: Hash + Eq + Serialize + for<'a> Deserialize<'a> {}

pub trait SyncableMergeCommit: Hash + Eq + Serialize + for<'a> Deserialize<'a> {}

pub trait SyncableAutoMerge: Hash + Eq + Serialize + for<'a> Deserialize<'a> {}
