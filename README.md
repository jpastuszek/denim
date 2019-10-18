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

# Example
Crate new minimal script and execute it.

```sh
denim new --bare hello_world
./hello_world
```

Crate new Cotton prelude script and execute it.

```sh
denim new hello_world
./hello_world
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
