use std::collections::HashMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::{Model, commit::{self, Commit}, node, syncable::Syncable};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Checksum {
	#[serde(with = "hex::serde")]
	digest: [u8; 16]
}

impl Model {
	#[must_use]
	pub fn checksum(&self, node_id: &node::ID) -> Checksum {
		let mut md5 = md5::Context::new();
		
		if let Some(content) = self.commits.content.get(node_id) {
			combine(&mut md5, content);
		}
		
		if let Some(children) = self.commits.children.get(node_id) {
			combine(&mut md5, children);
		}
		
		Checksum { digest: *md5.finalize() }
	}
}

fn combine<Data: Syncable>(
	context: &mut md5::Context,
	commits: &HashMap<commit::ID, Commit<Data>>
) {
	for uuid in commits.keys().map(|x| x.0).sorted() {
		context.consume(uuid);
	}
}
