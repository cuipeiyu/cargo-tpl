[workspace]
resolver = "2"
members = [ "crates/*" ]

[workspace.package]
rust-version = "1.73"
edition = "2021"
license = "MIT OR Apache-2.0"
authors = ["{author}"]

[workspace.dependencies]
# local crates
# package_cli = { path = "./crates/package_cli", version = "0.0.0" }
package_lib = { path = "./crates/package_lib", version = "0.0.0" }

# non-local crates
rand = { version = "0.8.5", features = [] }
