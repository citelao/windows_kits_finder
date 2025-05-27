use clap::{builder::PossibleValue, Args, Parser, Subcommand, ValueEnum};
use colored::*;
use kits::get_kit_dir;
use thiserror::Error;

mod kits;

#[derive(Parser, Debug)]
#[command(name = "Kits Tool")]
#[command(version = "0.1")]
#[command(about = "Find binaries from Windows Kits", long_about = None)]
struct CliArgs
{
    #[command(subcommand)]
    command: Commands,

    // TODO: well-known archs?
    #[arg(value_enum)]
    architecture: Option<KnownArchitecture>,

    #[arg(long)]
    kit_version: Option<String>,

    // TODO: list all kits?

    #[arg(long)]
    allow_missing: bool,

    #[arg(long)]
    kit_dir: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands
{
    Tool {
        #[command(flatten)]
        subargs: BinaryArg,
    },

    List,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct BinaryArg {
    #[arg(value_enum)]
    binary: Option<KnownBinary>,

    #[arg(long)]
    custom_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
enum KnownArchitecture {
    X64,
    X86,
    Arm,
    Arm64,
    Custom(String),
}

impl KnownArchitecture {
    fn to_string(&self) -> String {
        match self {
            KnownArchitecture::X64 => "x64".to_string(),
            KnownArchitecture::X86 => "x86".to_string(),
            KnownArchitecture::Arm => "arm".to_string(),
            KnownArchitecture::Arm64 => "arm64".to_string(),
            KnownArchitecture::Custom(s) => s.clone(),
        }
    }
}

impl ValueEnum for KnownArchitecture {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            KnownArchitecture::X64,
            KnownArchitecture::X86,
            KnownArchitecture::Arm,
            KnownArchitecture::Arm64,
        ]
    }
    
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            KnownArchitecture::X64 => Some(PossibleValue::new("x64")),
            KnownArchitecture::X86 => Some(PossibleValue::new("x86")),
            KnownArchitecture::Arm => Some(PossibleValue::new("arm")),
            KnownArchitecture::Arm64 => Some(PossibleValue::new("arm64")),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum KnownBinary {
    Accevent,
    Inspect,

    Custom(String),
}

impl KnownBinary {
    fn to_string(&self) -> String {
        match self {
            KnownBinary::Accevent => "accevent.exe".to_string(),
            KnownBinary::Inspect => "inspect.exe".to_string(),
            KnownBinary::Custom(s) => s.clone(),
        }
    }

    fn get_subdir(&self) -> String {
        match self {
            KnownBinary::Accevent => "accevent.exe".to_string(),
            KnownBinary::Inspect => "inspect.exe".to_string(),
            KnownBinary::Custom(s) => s.clone(),
        }
    }
}

impl ValueEnum for KnownBinary {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            KnownBinary::Accevent,
            KnownBinary::Inspect,
        ]
    }
    
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            KnownBinary::Accevent => Some(PossibleValue::new("accevent")),
            KnownBinary::Inspect => Some(PossibleValue::new("inspect")),
            _ => None,
        }
    }

}


#[derive(Error, Debug, PartialEq)]
pub enum OurError {
    #[error("kit version not found: {desired} (maybe you want {potential}?)")]
    BinDirNotFound{ desired: String, potential: String },

    #[error("tool not found: {0}")]
    ToolNotFound(String),

    #[error("`{0}` is not implemented yet")]
    NotImplemented(String),

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

fn do_it(args: CliArgs) -> Result<(), OurError> {
    let architecture = args.architecture.unwrap_or(KnownArchitecture::X64);

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

    match args.command {
        Commands::List => {
            // Write all bin_dirs
            for bin_dir in &bin_dirs {
                println!("{}", bin_dir.display());

                // List all archs
                if let Ok(entries) = std::fs::read_dir(bin_dir) {
                    for entry in entries.flatten() {
                        if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                            println!("  - {}", entry.file_name().to_string_lossy());
                        }
                    }
                } else {
                    eprintln!("Could not read directory: {}", bin_dir.display());
                }
            }
        },
        Commands::Tool { subargs } => {
            let binary = match subargs.binary {
                Some(k) => k,
                None => KnownBinary::Custom(subargs.custom_path.unwrap()),
            };
            let tool_path = bin_dir_to_use.join(architecture.to_string()).join(binary.get_subdir());
        
            // If the tool doesn't exist, print an error message and exit
            if !tool_path.exists() {
                if args.allow_missing {
                    // Write a warning to stderr
                    let warning = format!("Warning: tool not found: {}", tool_path.display());
                    eprintln!("{}", warning.yellow());
                } else {
                    return Err(OurError::ToolNotFound(binary.to_string()));
                }
            }
        
            // Print the path to the tool
            println!("{}", tool_path.display());
        },
    }
    
    Ok(())
}

fn main() {
    let args = CliArgs::parse();
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