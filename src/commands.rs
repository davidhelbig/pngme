use std::path::PathBuf;
use std::str::FromStr;

use crate::png::Png;
use crate::chunk::{Chunk, self};
use crate::chunk_type::ChunkType;
use crate::args;


pub fn encode(encode_args: args::EncodeArgs) {
    let file_contents = std::fs::read(&encode_args.file).unwrap();

    let mut png = Png::try_from(file_contents.as_ref()).unwrap();

    let chunk_type = ChunkType::from_str(&encode_args.chunk_type).unwrap();

    let write_path = match encode_args.output {
        Some(path) => path,
        None => encode_args.file
    };

    println!(
        "Writing message `{}` as type {} to file {}",
        encode_args.message,
        encode_args.chunk_type,
        &write_path.to_str().unwrap()
    );

    let chunk = Chunk::new(chunk_type, encode_args.message.as_bytes().to_vec());

    png.append_chunk(chunk);

    std::fs::write(&write_path, &png.as_bytes()).unwrap();

    println!("Done.")

}

pub fn decode(decode_args: args::DecodeArgs) {
    let file_contents = std::fs::read(&decode_args.file).unwrap();

    let mut png = Png::try_from(file_contents.as_ref()).unwrap();

    let chunk_by_type = png.chunk_by_type(&decode_args.chunk_type);

    if let Some(chunk) = chunk_by_type {
        println!("{}", chunk.data_as_string().unwrap())
    } else {
        eprintln!(
            "No chunk with type `{}` found in {}",
            decode_args.chunk_type,
            &decode_args.file.to_string_lossy()
        )
    }

}