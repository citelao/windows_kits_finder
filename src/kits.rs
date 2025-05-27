use std::env;
use std::path::PathBuf;

pub fn get_kit_dir() -> PathBuf {
    // TODO: does this need to have special handling if in a 32-bit environment?
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finds_bin_dirs() {
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
        let bin_dirs = get_kit_bin_dirs(temp_kit_dir.path().to_path_buf());

        // We expect two directories: 10.0.19041.0 and 10.0.22000.0, in that order.
        assert_eq!(bin_dirs.len(), 2);
        assert_eq!(bin_dirs[0].file_name().unwrap(), "10.0.19041.0");
        assert_eq!(bin_dirs[1].file_name().unwrap(), "10.0.22000.0");
    }

    #[test]
    fn test_ignores_bad_paths() {
        // - kit
        //     - 10
        //         - bin
        //             - arm
        //             - arm64
        //             - x64
        //             - x86
        let temp_kit_dir = assert_fs::TempDir::new().unwrap();
        let bin_dir = temp_kit_dir.join("10").join("bin");
        std::fs::create_dir_all(bin_dir.join("arm")).unwrap();
        std::fs::create_dir_all(bin_dir.join("arm64")).unwrap();
        std::fs::create_dir_all(bin_dir.join("x64")).unwrap();
        std::fs::create_dir_all(bin_dir.join("x86")).unwrap();

        let bin_dirs = get_kit_bin_dirs(temp_kit_dir.path().to_path_buf());

        // We expect no directories, since all are bad paths.
        assert!(bin_dirs.is_empty());
    }
}
