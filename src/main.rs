mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::PngMeArgs::parse();
    
    match cli {
        args::PngMeArgs::Encode(args) => commands::encode(args),
        args::PngMeArgs::Decode(args) => commands::decode(args),
        _ => todo!()
    }

    Ok(())
}