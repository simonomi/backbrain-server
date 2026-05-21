#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use crate::{children::Children, commit::Commit, content::Content};
use std::collections::HashMap;

mod node;
mod commit;
mod syncable;
mod content;
mod children;

fn main() {
	let content: HashMap<node::ID, HashMap<commit::ID, Commit<Content>>>;
	let children: HashMap<node::ID, HashMap<commit::ID, Commit<Children>>>;
	
	println!("Hello, world!");
}
