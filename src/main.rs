extern crate piston;
extern crate glutin_window;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

mod net;
mod input;

use std::env;

use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;


fn client(addr : &String) {
    let client = net::NetChan::new(addr).expect("failed to connect netchannel client");

    let mut window: Window = WindowSettings::new("Client input test", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    let (tx, rx) = client.channel;

    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.button_args(){
            use piston::input::Button::Keyboard;
            use piston::input::keyboard::Key;
            println!("{:?}", u);

            match u {
                ButtonArgs{button: Keyboard(x), ..} => {
                    match x {
                        Key::Left => tx.send(net::ClientMessage::CommandInput(input::Input::Left)).unwrap(),
                        Key::Right => tx.send(net::ClientMessage::CommandInput(input::Input::Right)).unwrap(),
                        Key::Up => tx.send(net::ClientMessage::CommandInput(input::Input::Up)).unwrap(),
                        Key::Down => tx.send(net::ClientMessage::CommandInput(input::Input::Down)).unwrap(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
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
