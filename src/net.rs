use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::thread::{spawn, JoinHandle};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::str;
use std;
use bincode::{serialize, deserialize};

use input::Input;

const BUFFER_SIZE: usize = 32768;

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    SpawnEntity,
    UpdateEntity,
    DestroyEntity
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Hello,
    CommandInput(Input),
    Say
}

pub struct NetChanServer {
    pub channel : (Sender<ServerMessage>, Receiver<ClientMessage>),
    thread : JoinHandle<()>
}

pub struct NetChan {
    pub channel : (Sender<ClientMessage>, Receiver<ServerMessage>),
    thread : JoinHandle<()>,
    serverAddr: SocketAddr
}

impl NetChanServer {
    pub fn new() -> std::io::Result<NetChanServer> {
        let socket = UdpSocket::bind("127.0.0.1:32768")?;
        let (txServer, rxServer) = channel::<ServerMessage>();
        let (txClient, rxClient) = channel::<ClientMessage>();

        let thread = spawn(move || {
            println!("Command Server Thread started");
            let mut buf = [0; BUFFER_SIZE];

            let txClient = txClient;

            loop {
                match socket.recv_from(&mut buf) {
                    Ok((amt, src)) => {
                        println!("amt: {}", amt);
                        println!("src: {}", src);
                        let msg : ClientMessage = deserialize(&buf).unwrap();
                        println!("{:?}", msg);
                    },
                    Err(e) => {
                        println!("couldn't receive a datagram: {}", e);
                    }
                }
            }
        });

        Ok(NetChanServer {
            channel: (txServer, rxClient),
            thread: thread,
        })
    }
}
impl NetChan {
    pub fn new<A: ToSocketAddrs>(addr: A) ->  std::io::Result<NetChan> {
        let socket = UdpSocket::bind("127.0.0.1:32767")?;
        let serverAddr = addr.to_socket_addrs().unwrap().next().unwrap();

        socket.connect(serverAddr)?;
        let (txServer, rxServer) = channel::<ServerMessage>();
        let (txClient, rxClient) = channel::<ClientMessage>();
        let thread = spawn(move || {
            println!("Command client Thread started");
            let mut buf = [0; BUFFER_SIZE];
            
            loop {
                let msg = rxClient.recv().unwrap();
                let serialized = serialize(&msg).unwrap();

                socket.send_to(&serialized, serverAddr);

                println!("{:?} -> serialized to {:?} ", msg, serialized);
            }
        });

        Ok(NetChan {
            channel: (txClient, rxServer),
            thread: thread,
            serverAddr: serverAddr
        })

    }
}