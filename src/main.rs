use clap::Parser;

mod kits;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args
{
    // TODO: well-known bins?
    binary: String,

    // TODO: well-known archs?
    architecture: Option<String>,

    kit_version: Option<String>,

    // TODO: list all kits?
}

fn main() {
    let args = Args::parse();
    let architecture = args.architecture.unwrap_or("x64".to_string());

    let bin_dirs = kits::get_kit_bin_dirs();
    let most_recent = bin_dirs.last().unwrap();

    let tool_path = most_recent.join(architecture).join(args.binary);

    // If the tool doesn't exist, print an error message and exit
    if !tool_path.exists() {
        eprintln!("Error: tool not found: {}", tool_path.display());
        std::process::exit(1);
    }

    // Print the path to the tool
    println!("{}", tool_path.display());
}
