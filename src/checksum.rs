use std::collections::HashMap;
use itertools::Itertools;
use crate::{Model, commit::{self, Commit}, node, syncable::Syncable};

impl Model {
	pub fn checksum(&self, node_id: &node::ID) -> md5::Digest {
		let mut md5 = md5::Context::new();
		
		combine(&mut md5, self.content.get(node_id).unwrap());
		combine(&mut md5, self.children.get(node_id).unwrap());
		
		md5.finalize()
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
