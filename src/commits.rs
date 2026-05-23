use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{children::Children, commit::{self, Commit}, content::Content, node};

#[derive(Default, Serialize, Deserialize)]
pub struct Commits {
	pub content: HashMap<node::ID, HashMap<commit::ID, Commit<Content>>>,
	pub children: HashMap<node::ID, HashMap<commit::ID, Commit<Children>>>,
}

impl Commits {
	pub fn extend(&mut self, other: Self) {
		self.content.extend(other.content);
		self.children.extend(other.children);
	}
}
