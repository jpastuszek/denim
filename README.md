[![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]

Denim is an alternative way to make and run Rust "scripts" with focus on minimal runtime overhead and ease of script development.

# Features
* Tool set to create, build and tests scripts without interrupting callers.
* Full real-time output of `cargo` command runs.
* Support for `Cargo.toml` definitions within script source.
* Sensible script templates.
* Very low execution overhead after script was built.
* #! support.

# Non-goals
* Run inline scripts - use `cargo script(er)` for this.
* Non Linux support.

# Installation

You will need Rust installed (tested with 1.37.0).
```sh
cargo install denim
```

# Examples

Crate new minimal script and build it.
```sh
denim new --bare hello_world
```

Crate new [cotton](https://github.com/jpastuszek/cotton) prelude script and build it (this will take a moment).
```sh
denim new hello_world
```

Now you can run the script as any other binary.
Note that you can also run the script without building it first - the build will be done silently before program is executed.
```sh
./hello_world
```

After making changes to script it needs to be rebuild for them to take effect.
Executing script directly (e.g. `./hello_world`) will execute last built version until changed script builds successfully.
```sh
denim build hello_world
```

Rebuild and run script after making changes.
```sh
denim exec hello_world
```

Run tests.
```sh
denim test hello_world
```

Check script.
```sh
denim check hello_world
```

[crates.io]: https://crates.io/crates/denim
[Latest Version]: https://img.shields.io/crates/v/denim.svg
[Documentation]: https://docs.rs/denim/badge.svg
[docs.rs]: https://docs.rs/denim
[License]: https://img.shields.io/crates/l/denim.svg
