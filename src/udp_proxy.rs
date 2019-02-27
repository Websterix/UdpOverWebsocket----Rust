extern crate bytes;
extern crate env_logger;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;

use std::net::SocketAddr;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::*;
use tokio_codec::BytesCodec;
use websocket::ClientBuilder;
use websocket::OwnedMessage;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("Initializing Udp Proxy");
    let udp_addr = "127.0.0.1:4023".parse::<SocketAddr>()?;
    let udp_socket = UdpSocket::bind(&udp_addr)?;

    let (_udp_sink, udp_stream) = UdpFramed::new(udp_socket, BytesCodec::new()).split();

    let client = ClientBuilder::new("ws://echo.websocket.org/")
        .unwrap()
        .async_connect_insecure()
        .and_then(move |(duplex, _)| {
            let (mut sink, stream) = duplex.split();
            let ch1 = stream
                .for_each(move |message| {
                    println!("Received Message: {:?}", message);
                    Ok(())
                });

            let mut i = 0;
            let ch2 = udp_stream.for_each(move |(bytes, _addr)| {
                    i += 1;
                    println!("Proxying udp data to websocket.");
                    println!("MessageSize: {}", bytes.len());

                    let _ = sink.start_send(OwnedMessage::Binary(bytes.to_vec()));
                    let _ = sink.poll_complete();

                    println!("{} packet sent", i);

                    Ok(())
                });

            tokio::spawn(ch1.then(|_| Ok(())));
            tokio::spawn(ch2.then(|_| Ok(())));

            Ok(())
        })
        .then(|r| {
            println!("Producer Connection Result {:?}", r.err());
            Ok(())
        });

    tokio::run(client);

    Ok(())
}
