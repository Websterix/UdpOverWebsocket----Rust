extern crate env_logger;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;


mod file_utils;

use std::net::SocketAddr;
use std::path::Path;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::*;
use tokio_codec::BytesCodec;
use std::fs::File;
use file_utils::get_chunks;

fn main() -> Result<(), Box<std::error::Error>> {
    let _ = env_logger::init();

    let chunk_size = 30000u64;

    let file = File::open(Path::new("C:\\Test_Temp\\my_text_doc.txt")).expect("FileOpenError");
    let chunks = get_chunks(file, chunk_size);

    let udp_sock = UdpSocket::bind(&"127.0.0.1:4024".parse()?)?;
    let sender = "127.0.0.1:4023".parse::<SocketAddr>()?;

    let (sink, _stream) = UdpFramed::new(udp_sock, BytesCodec::new()).split();


    let chunks = chunks
        .into_iter()
        .map(|x| Ok(x))
        .collect::<Vec<Result<Vec<u8>, std::io::Error>>>();

    let chunks_stream = futures::stream::iter(chunks);
    let writer = chunks_stream.map(move |b| {
        println!("iterating");
        (b.into(), sender)
    }).forward(sink).then(
        |x: Result<_, std::io::Error>| {
            println!("-->{:?}", x.err());
            Ok(())
        },
    );

    println!("start");

    tokio::run(writer);


    Ok(())
}