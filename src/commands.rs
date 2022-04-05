use std::str::FromStr;
use std::error::Error;

use crate::png::{Png, PngError};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::args;


pub fn encode(encode_args: &args::EncodeArgs) -> Result<(), Box<dyn Error>> {
    let file_contents = std::fs::read(&encode_args.file)?;

    let mut png = Png::try_from(file_contents.as_ref())?;

    let chunk_type = ChunkType::from_str(&encode_args.chunk_type)?;

    let write_path = match &encode_args.output {
        Some(path) => path,
        None => &encode_args.file
    };

    println!(
        "Writing message `{}` as type {} to file {}",
        encode_args.message,
        encode_args.chunk_type,
        &write_path.to_str().unwrap()
    );

    let chunk = Chunk::new(chunk_type, encode_args.message.as_bytes().to_vec());

    png.append_chunk(chunk);

    std::fs::write(&write_path, &png.as_bytes())?;

    println!("Done.");

    Ok(())
}

pub fn decode(decode_args: &args::DecodeArgs) -> Result<(), Box<dyn Error>> {
    let file_contents = std::fs::read(&decode_args.file).unwrap();

    let png = Png::try_from(file_contents.as_ref()).unwrap();

    let chunk_by_type = png.chunk_by_type(&decode_args.chunk_type);

    let chunk = chunk_by_type.ok_or_else(|| PngError::ChunkTypeNotFound(decode_args.chunk_type.to_owned()))?;

    println!("{}", chunk.data_as_string()?);

    Ok(())
}

pub fn remove(remove_args: &args::RemoveArgs) {
    let file_contents = std::fs::read(&remove_args.file).unwrap();

    let mut png = Png::try_from(file_contents.as_ref()).unwrap();

    png.remove_chunk(&remove_args.chunk_type).unwrap();

    std::fs::write(&remove_args.file, png.as_bytes()).unwrap();

    println!("Chunk `{}` removed from `{}`", &remove_args.chunk_type, &remove_args.file.to_string_lossy())
}

pub fn print(print_args: &args::PrintArgs) {
    let file_contents = std::fs::read(&print_args.file).unwrap();

    let png = Png::try_from(file_contents.as_ref()).unwrap();

    let chunk_types: Vec<_> = png.chunks().iter().map(|c| c.chunk_type().to_string()).collect();

    println!("{}", chunk_types.join(", "))
}
