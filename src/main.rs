use clap::Parser;
use colored::*;
use kits::get_kit_dir;
use thiserror::Error;

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

#[derive(Error, Debug, PartialEq)]
pub enum OurError {
    #[error("kit version not found: {desired} (maybe you want {potential}?)")]
    BinDirNotFound{ desired: String, potential: String },

    #[error("tool not found: {0}")]
    ToolNotFound(String),

    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
}

fn do_it(args: Args) -> Result<(), OurError> {
    let architecture = args.architecture.unwrap_or("x64".to_string());

    let kit_dir_to_use = args.kit_dir.map_or_else(|| get_kit_dir(), |dir| std::path::PathBuf::from(dir));
    let bin_dirs = kits::get_kit_bin_dirs(kit_dir_to_use);

    let bin_dir_to_use = if let Some(kit_version) = args.kit_version {
        if let Some(found_dir) = bin_dirs.iter().find(|dir| dir.file_name().unwrap().to_str().unwrap() == kit_version) {
            found_dir
        } else {
            // Get leaf folder name
            let latest_version = bin_dirs.last().map(|dir| dir.file_name().unwrap().to_str().unwrap());
            return Err(OurError::BinDirNotFound{ desired: kit_version, potential: latest_version.unwrap().to_string() });
        }
    } else {
        bin_dirs.last().unwrap()
    };

    let tool_path = bin_dir_to_use.join(architecture).join(args.binary.clone());

    // If the tool doesn't exist, print an error message and exit
    if !tool_path.exists() {
        if args.allow_missing {
            // Write a warning to stderr
            let warning = format!("Warning: tool not found: {}", tool_path.display());
            eprintln!("{}", warning.yellow());
        } else {
            return Err(OurError::ToolNotFound(args.binary));
        }
    }

    // Print the path to the tool
    println!("{}", tool_path.display());
    Ok(())
}

fn main() {
    let args = Args::parse();
    match do_it(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            let error = format!("{}: {}", "Error".bold(), e);
            eprintln!("{}", error.red());
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // - kit
        //     - 10
        //         - bin
        //             - 10.0.19041.0
        //             - 10.0.22000.0
        //                 - x64
        //                     - accevent.exe
        let temp_kit_dir = assert_fs::TempDir::new().unwrap();
        let bin_dir = temp_kit_dir.join("10").join("bin");
        std::fs::create_dir_all(bin_dir.join("10.0.19041.0")).unwrap();
        std::fs::create_dir_all(bin_dir.join("10.0.22000.0").join("x64")).unwrap();
        std::fs::write(bin_dir.join("10.0.22000.0").join("x64").join("accevent.exe"), "").unwrap();

        let args = Args {
            binary: "accevent.exe".to_string(),
            architecture: Some("x64".to_string()),
            kit_version: None,
            allow_missing: false,
            kit_dir: Some(temp_kit_dir.path().to_str().unwrap().to_string()),
        };

        let result = do_it(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tool_not_found() {
        // - kit
        //     - 10
        //         - bin
        //             - 10.0.19041.0
        //             - 10.0.22000.0
        //                 - x64
        //                     - accevent.exe
        let temp_kit_dir = assert_fs::TempDir::new().unwrap();
        let bin_dir = temp_kit_dir.join("10").join("bin");
        std::fs::create_dir_all(bin_dir.join("10.0.19041.0")).unwrap();
        std::fs::create_dir_all(bin_dir.join("10.0.22000.0").join("x64")).unwrap();
        std::fs::write(bin_dir.join("10.0.22000.0").join("x64").join("accevent.exe"), "").unwrap();

        let args = Args {
            binary: "afakeexe.exe".to_string(),
            architecture: Some("x64".to_string()),
            kit_version: None,
            allow_missing: false,
            kit_dir: Some(temp_kit_dir.path().to_str().unwrap().to_string()),
        };

        let result = do_it(args);
        assert!(result.is_err());
        assert!(result.unwrap_err() == OurError::ToolNotFound("afakeexe.exe".to_string()));

        // Test with allow_missing
        let args = Args {
            binary: "afakeexe.exe".to_string(),
            architecture: Some("x64".to_string()),
            kit_version: None,
            allow_missing: true,
            kit_dir: Some(temp_kit_dir.path().to_str().unwrap().to_string()),
        };

        let result = do_it(args);
        assert!(result.is_ok());
        assert!(result.unwrap() == ());
    }

    #[test]
    fn test_version_not_found() {
        // - kit
        //     - 10
        //         - bin
        //             - 10.0.19041.0
        //             - 10.0.22000.0
        //                 - x64
        //                     - accevent.exe
        let temp_kit_dir = assert_fs::TempDir::new().unwrap();
        let bin_dir = temp_kit_dir.join("10").join("bin");
        std::fs::create_dir_all(bin_dir.join("10.0.19041.0")).unwrap();
        std::fs::create_dir_all(bin_dir.join("10.0.22000.0").join("x64")).unwrap();
        std::fs::write(bin_dir.join("10.0.22000.0").join("x64").join("accevent.exe"), "").unwrap();

        let args = Args {
            binary: "accevent.exe".to_string(),
            architecture: Some("x64".to_string()),
            kit_version: Some("10.0.12345.0".to_string()),
            allow_missing: false,
            kit_dir: Some(temp_kit_dir.path().to_str().unwrap().to_string()),
        };

        let result = do_it(args);
        assert!(result.is_err());
        assert!(result.unwrap_err() == OurError::BinDirNotFound{ desired: "10.0.12345.0".to_string(), potential: "10.0.22000.0".to_string() });
    }
}