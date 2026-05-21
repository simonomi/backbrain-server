use crate::syncable::Syncable;
use std::collections::HashSet;

pub struct ID;

pub struct Commit<Data: Syncable> {
	// id: ID, // ?
	kind: CommitKind<Data>
	// createdAt: Date
}

pub enum CommitKind<Data: Syncable> {
	Regular(Data::Event, Option<Predecessor<Data>>),
	Merge(Data::Merge, HashSet<Predecessor<Data>>)
}

pub enum Predecessor<Data: Syncable> {
	Commit(ID),
	AutoMerge(Data::AutoMerge, HashSet<Self>)
}
