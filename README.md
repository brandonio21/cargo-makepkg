cargo-makepkg
=============
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

Similar Projects
----------------
cargo-makepkg is technically a heavily-modified fork of [cargo-arch](https://github.com/wdv4758h/cargo-arch), which builds `PKGBUILD` files based on the contents of `Cargo.toml`.

[cargo-pkgbuild](https://github.com/kstep/cargo-pkgbuild) is similar to `cargo-arch` and accomplishes the same goal in a much simpler way.