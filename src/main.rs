mod arrays;
mod entities;
mod region;
mod bb;
mod uuid;
mod vec;
use std::{
    fs::{self, File},
    io::{Read, Write},
};

use flate2::bufread::{GzDecoder, ZlibDecoder};
use nbt::error;
use serde::Deserialize;
use vec::Vec3D;

use crate::{entities::EntityChunk, region::RegionChunk};

#[derive(Debug)]
struct ChunkInfo {
    // Offset of where the chunk is located in the file.
    chunk_offset_bytes: usize,
    // Size of the chunk.
    size: usize,
    // Timestamp of the last time the chunk was modified.
    timestamp: u32,
}

#[derive(Debug)]
struct ChunkHeader {
    length: u32,
    compression_scheme: CompressionScheme,
}

impl From<[u8; 5]> for ChunkHeader {
    fn from(value: [u8; 5]) -> Self {
        Self {
            length: u32::from_be_bytes(value[0..4].try_into().unwrap()),
            compression_scheme: CompressionScheme::from(value[4]),
        }
    }
}

#[derive(Debug)]
enum CompressionScheme {
    Gzip,
    Zlib,
}

impl From<u8> for CompressionScheme {
    fn from(value: u8) -> Self {
        match value {
            1 => CompressionScheme::Gzip,
            2 => CompressionScheme::Zlib,
            _ => unreachable!("Unknown compression scheme {}", value),
        }
    }
}

#[test]
fn test() {
    extern crate uuid as test;
    use std::io::Cursor;
    use test::Uuid as uuid_parser;
    use uuid::Uuid;
    let x = "279afc35-e8a9-4927-adb0-19b33499ec6c";
    let uuid = Uuid::new(x);
    let x = uuid.to_u128();
    println!("UUID: {:#?}", x);
    println!("UUID: {:#?}", uuid_parser::from_u128(x));

    let filename = "output.nbt";
    let bytes = fs::read(filename).unwrap();
    let cursor = Cursor::new(bytes);
    let x: EntityChunk = nbt::from_reader(cursor).unwrap();
    println!("Here: {:#?}", x);
}

fn main() {
    // let filename = "r.0.-1.mca";
    // parse_mca::<EntityChunk>(filename).unwrap();

    let filename = "region/r.0.0.mca";
    println!("{:#?}", parse_mca::<RegionChunk>(filename).unwrap());
}

pub fn parse_mca<'de, T>(filename: &str) -> error::Result<Vec<T>>
where
    T: Deserialize<'de>,
{
    let bytes = fs::read(filename)?;

    let mut chunk_infos = Vec::new();
    const CHUNK_SIZE: usize = 4096;
    // The first 8KiB of the MCA file is the header which contains the location and timestamp tables for each chunk.
    for (byte_offset, chunk_bytes) in bytes[0..CHUNK_SIZE].chunks(4).enumerate() {
        let int_offset = byte_offset * 4;
        let chunk_offset = u32::from_be_bytes([0, chunk_bytes[0], chunk_bytes[1], chunk_bytes[2]]);
        let size = chunk_bytes[3];
        // If chunk offset and size are 0 then the chunk hasn't been generated yet.
        if chunk_offset != 0 && size != 0 {
            // Should always be a 4 byte timestamp.
            let timestamp_bytes = &bytes[(CHUNK_SIZE + int_offset)..(CHUNK_SIZE + int_offset + 4)];
            let timestamp = u32::from_be_bytes(timestamp_bytes.try_into().expect(&format!(
                "Only expected 4 bytes but got {}",
                timestamp_bytes.len()
            )));
            chunk_infos.push(ChunkInfo {
                chunk_offset_bytes: (chunk_offset as usize) * CHUNK_SIZE,
                size: (size as usize) * CHUNK_SIZE,
                timestamp,
            });
        }
    }
    let mut chunks: Vec<T> = Vec::with_capacity(chunk_infos.len());
    for chunk_info in chunk_infos {
        // Read first 5 bytes as chunk header
        let mut current_offset = chunk_info.chunk_offset_bytes;
        let header_bytes: [u8; 5] =
            bytes[current_offset..current_offset + 5]
                .try_into()
                .or(Err(error::Error::Message(
                    "Coult not slice the chunk header.".into(),
                )))?;
        current_offset += 5;
        // Parse chunk header into meaningful parts
        let header = ChunkHeader::from(header_bytes);
        println!("Chunk: {:#?}", chunk_info);
        println!("Header: {:#?}", header);
        // Read from chunk header to chunk_header + chunk_length
        let nbt_bytes = &bytes[current_offset..current_offset + header.length as usize];
        let mut decompressed = Vec::new();
        // Decode using the specified compression method
        let mut reader = decompress_bytes_with_scheme(nbt_bytes, header.compression_scheme);
        reader.read_to_end(&mut decompressed).unwrap();

        let mut file = File::create("output.nbt").unwrap();
        file.write_all(&decompressed).unwrap();

        chunks.push(nbt::from_slice(decompressed).unwrap());
    }
    Ok(chunks)
}

fn decompress_bytes_with_scheme<'a>(
    bytes: &'a [u8],
    compression_scheme: CompressionScheme,
) -> Box<dyn Read + 'a> {
    match compression_scheme {
        CompressionScheme::Gzip => Box::new(GzDecoder::new(bytes)),
        CompressionScheme::Zlib => Box::new(ZlibDecoder::new(bytes)),
    }
}
