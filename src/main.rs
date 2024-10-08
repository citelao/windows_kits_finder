use std::error::Error;

use clap::Parser;
use colored::*;
use kits::get_kit_dir;

mod kits;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args
{
    // TODO: well-known bins?
    binary: String,

    // TODO: well-known archs?
    architecture: Option<String>,

    #[arg(long)]
    kit_version: Option<String>,

    // TODO: list all kits?

    #[arg(long)]
    allow_missing: bool,

    #[arg(long)]
    kit_dir: Option<String>,
}

fn do_it(args: Args) -> Result<(), Box<dyn Error>> {
    let architecture = args.architecture.unwrap_or("x64".to_string());

    let kit_dir_to_use = args.kit_dir.map_or_else(|| get_kit_dir(), |dir| std::path::PathBuf::from(dir));
    let bin_dirs = kits::get_kit_bin_dirs(kit_dir_to_use);

    let bin_dir_to_use = if let Some(kit_version) = args.kit_version {
        if let Some(found_dir) = bin_dirs.iter().find(|dir| dir.file_name().unwrap().to_str().unwrap() == kit_version) {
            found_dir
        } else {
            // Get leaf folder name
            let latest_version = bin_dirs.last().map(|dir| dir.file_name().unwrap().to_str().unwrap());

            let error = format!("{}: kit version not found: {}; maybe you want '{}'", "Error".bold(), kit_version, latest_version.unwrap());
            eprintln!("{}", error.red());
            std::process::exit(1);
        }
    } else {
        bin_dirs.last().unwrap()
    };

    let tool_path = bin_dir_to_use.join(architecture).join(args.binary);

    // If the tool doesn't exist, print an error message and exit
    if !tool_path.exists() {
        if args.allow_missing {
            // Write a warning to stderr
            let warning = format!("Warning: tool not found: {}", tool_path.display());
            eprintln!("{}", warning.yellow());
        } else {
            let error = format!("{}: tool not found: {}", "Error".bold(), tool_path.display());
            eprintln!("{}", error.red());
            std::process::exit(1);
        }
    }

    // Print the path to the tool
    println!("{}", tool_path.display());
    Ok(())
}

fn main() {
    let args = Args::parse();
    let result = do_it(args);
    result.unwrap();
}
