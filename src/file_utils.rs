use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn get_chunks(mut file: File, chunk_size: u64) -> Vec<Vec<u8>> {
    let mut vecc = vec![];
    file.read_to_end(&mut vecc).expect("could not read");
    let size = file.metadata().expect("Metadata error").len();

    let mut reader = BufReader::new(&vecc[..]);

    let mut chunks: Vec<Vec<u8>> = Vec::new();

    for _ in (0..size).step_by(chunk_size as usize) {
        let chunk = read_n(&mut reader, chunk_size);
        chunks.push(chunk);
    }

    chunks
}

fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
    where
        R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let _ = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    buf
}
