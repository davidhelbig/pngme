mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::PngMeArgs;
use clap::Parser;

fn main() {
    let cli = args::PngMeArgs::parse();
    
    let result = process_args(cli);

    if let Err(err) = result {
        eprint!("{err}");
        std::process::exit(1);
    }
}

fn process_args(cli: PngMeArgs) -> Result<(), String> {
    match cli {
        args::PngMeArgs::Encode(args) => {
            if let Err(err) = commands::encode(&args) {
                return Err(format!("Error while encoding message to {:?}: {}", &args.file, err));
            }
        },
        args::PngMeArgs::Decode(args) => {
            if let Err(err) = commands::decode(&args) {
                return Err(format!("Error while decoding message from {:?}: {}", &args.file, err))
            }
        },
        args::PngMeArgs::Remove(args) => commands::remove(&args),
        args::PngMeArgs::Print(args) => commands::print(&args)
    }
    Ok(())
}