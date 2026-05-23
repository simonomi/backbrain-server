#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use itertools::Itertools;
use tungstenite::{Message, accept};

use crate::{checksum::Checksum, commit::Commit, commits::Commits, content::Content};
use std::{collections::{HashMap, HashSet}, env, net::TcpListener, sync::{LazyLock, RwLock}, thread::spawn};

mod node;
mod commit;
mod syncable;
mod content;
mod children;
mod checksum;
mod commits;

struct Generation(usize);

// TODO: persistence
#[derive(Default)]
pub struct Model {
	commits: Commits,
	// we will need state for permissions, but not yet
	
	history: Vec<HashSet<(node::ID, commit::ID)>>,
}

impl Model {
	const fn current_generation(&self) -> Generation {
		Generation(self.history.len())
	}
}

struct Peer {
	public_key: PublicKey,
	last_synced_at: Generation
}

struct PublicKey;

static MODEL_LOCK: LazyLock<RwLock<Model>> = LazyLock::new(RwLock::default);

fn main() {
	// let peers: Vec<Peer>;
	
	let backbrain_address = env::var("BACKBRAIN_ADDRESS").unwrap();
	
	let server = TcpListener::bind(&backbrain_address).unwrap();
	
	println!("started at {backbrain_address}");
	
	for stream in server.incoming() {
		spawn(|| {
			let mut websocket = accept(stream.unwrap()).unwrap();
			
			websocket.send(Message::Text("full".into())).unwrap();
			
			let message = websocket.read().unwrap();
			
			if !message.into_text().is_ok_and(|text| text == "full") {
				println!("invalid sync type, closing connection");
				websocket.close(None).unwrap();
				websocket.flush().unwrap();
				return;
			}
			
			let model = MODEL_LOCK.read().unwrap();
			
			let checksums: HashMap<node::ID, Checksum> = model.commits.content.keys()
				.chain(model.commits.children.keys())
				.unique()
				.cloned()
				.map(|node_id| {
					let checksum = model.checksum(&node_id);
					(node_id, checksum)
				})
				.collect();
			
			let bytes = serde_json::to_vec(&checksums).unwrap();
			
			websocket.send(Message::binary(bytes)).unwrap();
			
			let string = serde_json::to_string(&checksums).unwrap();
			println!("sent checksums: {string}");
			
			let message = websocket.read().unwrap();
			
			let bytes = message.into_data();
			
			println!("received checksums: {}", str::from_utf8(&bytes).unwrap());
			
			let other_checksums: HashMap<node::ID, Checksum> = serde_json::from_slice(&bytes)
				.unwrap();
			
			let differing_node_ids: HashSet<node::ID> = checksums.iter()
				.filter(|(node_id, checksum)| other_checksums.get(node_id) != Some(*checksum))
				.map(|(node_id, _)| node_id)
				.cloned()
				.collect();
			
			// TODO: can this borrow instead of cloning everything?
			let commits = Commits {
				content: differing_node_ids.iter()
					.map(|node_id| (
						node_id.clone(),
						model.commits.content.get(node_id).unwrap().clone()
					))
					.collect(),
				children: differing_node_ids.iter()
					.map(|node_id| (
						node_id.clone(),
						model.commits.children.get(node_id).unwrap().clone()
					))
					.collect()
			};
			
			drop(model);
			
			let bytes = serde_json::to_vec(&commits).unwrap();
			
			websocket.send(Message::binary(bytes)).unwrap();
			
			let string = serde_json::to_string(&commits).unwrap();
			println!("sent commits: {string}");
			
			let message = websocket.read().unwrap();
			
			let bytes = message.into_data();
			
			println!("received commits: {}", str::from_utf8(&bytes).unwrap());
			
			let new_commits: Commits = serde_json::from_slice(&bytes)
				.unwrap();
			
			let mut model = MODEL_LOCK.write().unwrap();
			
			model.commits.extend(new_commits);
		});
	}
}
