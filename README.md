# Windows Kit binary finder

A simple tool to find binaries in Windows Kits, since it's a bit annoying.

## Installation

(Not yet published; build locally & add to your path?)

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

## TODO

* [X] License
* [ ] Support includes & libs etc
* [X] List available kits
* [X] Get the base path
* [ ] Add a bunch more tools!
* [ ] Document each option/flag/etc.
* [ ] Document usage!
* [ ] Publish tool!