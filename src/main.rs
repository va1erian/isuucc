#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate ggez;
extern crate tiled;

mod net;
mod input;
mod engine;
mod gfx;
mod game_state;
use engine::GameEngine;
use std::env;
use ggez::{Context, ContextBuilder, event};
use ggez::conf;
use std::path;

fn client(addr : &String) {
    let client = net::NetChan::new(addr).expect("failed to connect netchannel client");
    let mut ctx = init_context();
    let mut engine = GameEngine::new();

    engine.to_game(client, "level1".to_owned());
    let result = event::run(&mut ctx, &mut engine);
    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }

}

fn init_context() -> Context {
    let mut cb = ContextBuilder::new("isuucc", "ggez")
        .window_setup(conf::WindowSetup::default().title("Blunder of isuucc"))
        .window_mode(conf::WindowMode::default().dimensions(640, 480));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    cb.build().unwrap()
}
fn server() {
    let server = net::NetChanServer::new().expect("failed to start netchannel server");
    let (tx, rx) = server.channel;

    loop {
        let maybe_msg = rx.recv();
        if let Ok(msg) = maybe_msg {
            println!("{:?}", msg)
        } else {
            println!("recv error");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.contains(&String::from("--server")) {
        server();
    } else {
        let addr = args.get(1).expect("expected server address");
        client(addr);
    };
}
