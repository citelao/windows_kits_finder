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
    #[arg(long)]
    architecture: Option<String>,

    #[arg(long)]
    kit_version: Option<String>,

    #[arg(long)]
    kit_dir: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands
{
    Tool {
        #[command(flatten)]
        subargs: BinaryArg,

        // Start the tool?
        #[arg(long)]
        run: bool,

        // Allow missing tools (don't error out if the tool is not found)
        #[arg(long)]
        allow_missing: bool,
    },

    // List all available Windows Kits
    Kits,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct BinaryArg {
    #[arg(value_enum)]
    binary: Option<KnownBinary>,

    #[arg(long)]
    custom_path: Option<String>,

    // Just list all known binaries
    #[arg(long)]
    list: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum KnownBinary {
    Accevent,
    Inspect,

    MakePri,

    Custom(String),
}

impl KnownBinary {
    fn to_string(&self) -> String {
        match self {
            KnownBinary::Accevent => "accevent.exe".to_string(),
            KnownBinary::Inspect => "inspect.exe".to_string(),
            KnownBinary::MakePri => "makepri.exe".to_string(),
            KnownBinary::Custom(s) => s.clone(),
        }
    }

    fn get_subdir(&self) -> String {
        match self {
            KnownBinary::Accevent => "accevent.exe".to_string(),
            KnownBinary::Inspect => "inspect.exe".to_string(),
            KnownBinary::MakePri => "makepri.exe".to_string(),
            KnownBinary::Custom(s) => s.clone(),
        }
    }
}

impl ValueEnum for KnownBinary {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            KnownBinary::Accevent,
            KnownBinary::Inspect,
            KnownBinary::MakePri,
        ]
    }
    
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            KnownBinary::Accevent => Some(PossibleValue::new("accevent")),
            KnownBinary::Inspect => Some(PossibleValue::new("inspect")),
            KnownBinary::MakePri => Some(PossibleValue::new("makepri")),
            _ => None,
        }
    }

}


#[derive(Error, Debug, PartialEq)]
pub enum OurError {
    #[error("kit version not found: {desired} (maybe you want {potential}?)")]
    BinDirNotFound{ desired: String, potential: String },

    #[error("tool not found: {0} ({1})")]
    ToolNotFound(String, String),

    #[error("tool failed: {0} - {1}")]
    ToolFailed(String, String),

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

    match args.command {
        Commands::Kits => {
            // Write all bin_dirs in reverse order
            println!("Available Windows Kits:");
            for bin_dir in bin_dirs.iter().rev() {
                let kit_name = bin_dir.file_name().unwrap().to_string_lossy();
                let is_default = if bin_dir == bin_dir_to_use { "(default)".to_string().dimmed() } else { "".to_string().into() };
                println!(" - {} {}", kit_name, is_default);
            }
        },
        Commands::Tool { subargs, run, allow_missing } => {
            if subargs.list {
                // List all known binaries
                println!("Known tools:");
                for binary in KnownBinary::value_variants() {
                    println!(" - {}", binary.to_possible_value().unwrap().get_name());
                }
                println!(" - {}", "custom (use --custom-path to specify a path)".dimmed());
                return Ok(());
            }

            let binary = match subargs.binary {
                Some(k) => k,
                None => KnownBinary::Custom(subargs.custom_path.unwrap()),
            };
            let tool_path = bin_dir_to_use.join(architecture).join(binary.get_subdir());
        
            // If the tool doesn't exist, print an error message and exit
            if !tool_path.exists() {
                if allow_missing {
                    // Write a warning to stderr
                    let warning = format!("Warning: tool not found: {}", tool_path.display());
                    eprintln!("{}", warning.yellow());
                } else {
                    return Err(OurError::ToolNotFound(binary.to_string(), tool_path.display().to_string()));
                }
            }

            if run {
                // If the tool exists, run it
                let status = std::process::Command::new(&tool_path)
                    .status();

                match status {
                    Ok(s) => {
                        if !s.success() {
                            return Err(OurError::ToolFailed(binary.to_string(), format!("Process exited with status: {}", s)));
                        }
                    }
                    Err(e) => {
                        return Err(OurError::ToolFailed(binary.to_string(), e.to_string()));
                    }
                }

                return Ok(());
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
        assert!(result.unwrap_err() == OurError::ToolNotFound("afakeexe.exe".to_string(), "path/to/afakeexe.exe".to_string()));

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