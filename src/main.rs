use clap::Parser;

mod kits;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args
{
    // TODO: well-known bins?
    binary: String,

    // TODO: well-known archs?
    architecture: String,

    kit_version: Option<String>,

    // TODO: list all kits?
}

fn main() {
    let args = Args::parse();

    let bin_dirs = kits::get_kit_bin_dirs();
    let most_recent = bin_dirs.last().unwrap();

    println!("Hello, {0} {1}!", args.binary, most_recent.display());
}
