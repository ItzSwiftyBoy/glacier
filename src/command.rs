use std::fs::File;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct GlacierArgs {
    #[arg(short, long)]
    filepath: String,
}

pub fn args_parse() {
    let args = GlacierArgs::parse();

    let file = File::open(&args.filepath);
}
