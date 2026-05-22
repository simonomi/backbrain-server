use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::syncable::Syncable;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ID(pub Uuid);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit<Data: Syncable> {
	kind: CommitKind<Data>,
	created_at: f64
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommitKind<Data: Syncable> {
	Regular { event: Data::Event, predecessor: Option<Predecessor<Data>> },
	Merge { merge: Data::Merge, predecessors: HashSet<Predecessor<Data>> }
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Predecessor<Data: Syncable> {
	Commit(ID),
	AutoMerge { auto_merge: Data::AutoMerge, predecessors: Vec<Self> }
}
