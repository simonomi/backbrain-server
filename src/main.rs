#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use tungstenite::{Message, accept};

use crate::{children::Children, commit::Commit, content::Content};
use std::{collections::{HashMap, HashSet}, env, net::TcpListener, sync::RwLock, thread::spawn};

mod node;
mod commit;
mod syncable;
mod content;
mod children;

struct Generation(usize);

// TODO: persistence
struct Model {
	nodes: HashMap<node::ID, HashSet<commit::ID>>,
	
	content: HashMap<commit::ID, Commit<Content>>,
	children: HashMap<commit::ID, Commit<Children>>,
	
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

fn main() {
	let peers: Vec<Peer>;
	let model: RwLock<Model>;
	
	let backbrain_address = env::var("BACKBRAIN_ADDRESS").unwrap();
	
	let server = TcpListener::bind(&backbrain_address).unwrap();
	
	println!("started at {backbrain_address}");
	
	for stream in server.incoming() {
		spawn(move || {
			let mut websocket = accept(stream.unwrap()).unwrap();
			
			loop {
				let message = websocket.read().unwrap();
				
				if message.is_close() {
					break;
				} else if let Ok(text) = message.into_text() {
					println!("{text}");
					
					let response = Message::Text("loud and clear".into());
					
					websocket.send(response).unwrap();
				}
			}
		});
	}
}
