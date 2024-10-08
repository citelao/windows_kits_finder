use std::path::PathBuf;
use std::env;

pub fn get_kit_dir() -> PathBuf {
    let program_files_x86 = env::var("ProgramFiles(x86)").unwrap();
    let base_path = PathBuf::from(program_files_x86).join("Windows Kits");
    base_path
}

pub fn get_kit_bin_dirs(kit_dir: PathBuf) -> Vec<PathBuf> {
    let bin_dir = kit_dir.join("10").join("bin");

    // List all the directories in the bin directory
    //
    // Should look like:
    //
    // * C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0
    // * C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0
    // * ...
    // * arm (with some weird XAML DLLs)
    // * arm64 (ditto)
    // * x64 (ditto)
    // * x86 (ditto)
    //
    // We filter out the non-"version number" directories
    let mut bin_dirs = Vec::new();
    for entry in bin_dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        const BAD_PATHS: [&str; 4] = ["arm", "arm64", "x64", "x86"];

        if path.is_dir() && !BAD_PATHS.contains(&path.file_name().unwrap().to_str().unwrap()) {
            bin_dirs.push(path);
        }
    }

    // Sort the directories by version number
    bin_dirs.sort();

    bin_dirs
}