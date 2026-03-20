# winky - Windows Kit binary finder

`winky` is a simple tool to find binaries in Windows Kits, since it's a bit annoying.

## What is this?

[Windows Kits](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/) (aka the Windows SDK) have loads of useful tools for the aspiring Windows developer. For example, `inspect.exe` and `acceevent.exe` are fantastic accessibility testing tools, and `makepri.exe` can generate .PRI files for XAML apps.

Unfortunately, the paths can be annoying. For example:

```
C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\accevent.exe
```

And they change with new builds of Windows. It's a tiny problem that deserves a tiny solution.

Enter `winky`.

## Usage

```pwsh
# Get the path to accevent from the latest kit
winky tool accevent

# Get the base bin path from the latest kit
winky bin

# List all available kits.
winky bin --list
```

---

## Installation

### Chocolatey (Recommended)
```powershell
choco install winky
```

### Direct Download
Download the latest `winky.exe` from the [Releases page](https://github.com/citelao/winky/releases) and add it to your PATH.

### Build from Source
```bash
cargo install --git https://github.com/citelao/winky
```

## Contributing

```bash
cargo run -- --help
cargo test
```

Before pushing to GitHub (it'll run in CI, too):

```bash
cargo check
cargo fmt
```

### Publishing a new version

```bash
# Initial setup
cargo install cargo-release

cargo release patch # or minor or major
# ... dry-run
cargo release patch --execute
```

## TODO

* [X] License
* [ ] Support includes & libs etc
* [X] List available kits
* [X] Get the base path
* [ ] Match architecture to current OS.
* [ ] Add a bunch more tools!
* [ ] Document each option/flag/etc.
* [ ] Document usage!
* [ ] Publish tool!