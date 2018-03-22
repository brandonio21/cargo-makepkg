cargo-makepkg
=============
![cargo-makepkg is on crates.io](https://img.shields.io/crates/v/cargo-makepkg.svg "Find package on crates.io")


A Cargo extension similar to [cargo-deb](https://github.com/mmstick/cargo-deb)
which allows building of Arch Linux packages from cargo. This makes it quick-and-easy to send your Rust projects to your Arch using friends!

This process involves building a `PKGBUILD` file and invoking `makepkg` to create a `.tar.xz` package and a `.SRCINFO` file.

Usage
-----
```
cargo install cargo-makepkg
cargo makepkg
cat target/archlinux/<version>/PKGBUILD
```

Information will be extracted from `Cargo.toml` to create a `PKGBUILD` file.
If you have any differing information, it can be specified in the `[package.metadata.archlinux_pkgbuild]`
section of `Cargo.toml`. See `src/config/arch.rs` for available fields.

Similar Projects
----------------
cargo-makepkg is technically a heavily-modified fork of [cargo-arch](https://github.com/wdv4758h/cargo-arch), which builds `PKGBUILD` files based on the contents of `Cargo.toml`.

[cargo-pkgbuild](https://github.com/kstep/cargo-pkgbuild) is similar to `cargo-arch` and accomplishes the same goal in a much simpler way.
