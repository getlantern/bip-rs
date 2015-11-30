extern crate bip_dht;

use std::net::{ToSocketAddrs};

use bip_dht::{DhtBuilder, Router, MainlineDht};

fn main() {
	let builder = DhtBuilder::with_router(Router::uTorrent)
		.set_source_addr(("0.0.0.0", 6881).to_socket_addrs().unwrap().next().unwrap());
	let dht = MainlineDht::with_builder(builder).unwrap();
	
	loop {
	
	}
}