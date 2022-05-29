use std::{
    net::{IpAddr, Ipv4Addr},
    thread,
};

use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum DataType {
    Coords { x: f32, y: f32, z: f32 },
}

#[tokio::main]
async fn main() -> Result<(), ErrorKind> {
    let mut socket = Socket::bind(format!("{}:42069", Ipv4Addr::LOCALHOST))?;
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();

                    if msg == b"Bye!" {
                        break;
                    }

                    let msg = String::from_utf8_lossy(msg);
                    let ip = packet.addr().ip();

                    println!("Received {:?} from {:?}", msg, ip);

                    sender
                        .send(Packet::reliable_unordered(
                            packet.addr(),
                            "Copy that!".as_bytes().to_vec(),
                        ))
                        .expect("This should send");
                }
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
