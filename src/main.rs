#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use itertools::Itertools;
use tungstenite::{Message, accept};

use crate::{children::Children, commit::Commit, content::Content};
use std::{collections::{HashMap, HashSet}, env, net::TcpListener, sync::{LazyLock, RwLock}, thread::spawn};

mod node;
mod commit;
mod syncable;
mod content;
mod children;
mod checksum;

struct Generation(usize);

// TODO: persistence
#[derive(Default)]
pub struct Model {
	content: HashMap<node::ID, HashMap<commit::ID, Commit<Content>>>,
	children: HashMap<node::ID, HashMap<commit::ID, Commit<Children>>>,
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
			
			loop {
				let message = websocket.read().unwrap();
				
				if message.is_close() {
					break;
				} else if let Ok(text) = message.into_text() {
					println!("{text}");
					
					let response = Message::Text("loud and clear".into());
					
					websocket.send(response).unwrap();
					
					let model = MODEL_LOCK.read().unwrap();
					
					let checksums: HashMap<node::ID, md5::Digest> = model.content.keys()
						.chain(model.children.keys())
						.unique()
						.cloned()
						.map(|node_id| {
							let checksum = model.checksum(&node_id);
							(node_id, checksum)
						})
						.collect();
					
					drop(model);
					
					dbg!(checksums);
				}
			}
		});
	}
}
