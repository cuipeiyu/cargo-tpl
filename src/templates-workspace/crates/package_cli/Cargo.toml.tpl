[package]
name = "package_cli"
version = "0.1.0"
description = ""

authors.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = { workspace = true }
package_lib = { path = "../package_lib" }
