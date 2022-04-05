use clap::{Parser};
use std::path::PathBuf;


#[derive(Parser)]
#[clap(author, version, about)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

/// Encodes a message to a png file
#[derive(Parser)]
pub struct EncodeArgs {
    /// PNG file to encode message to.
    pub file: PathBuf,
    /// The chunk type.
    pub chunk_type: String,
    /// The message to be encoded.
    pub message: String,
    /// File to write to
    pub output: Option<PathBuf>
}

/// Decodes a message from a png file
#[derive(Parser)]
pub struct DecodeArgs {
    pub file: PathBuf,
    pub chunk_type: String
}

/// Removes a chunk from a png file
#[derive(Parser)]
pub struct RemoveArgs {
    pub file: PathBuf,
    pub chunk_type: String
}

/// List all chunks in the given png file
#[derive(Parser)]
pub struct PrintArgs {
    pub file: PathBuf
}
