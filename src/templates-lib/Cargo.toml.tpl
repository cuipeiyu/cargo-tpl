[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
authors = ["{author}"]
license = "MIT OR Apache-2.0"
include = ["src", "Cargo.toml", "README.md", "LICENSE-*", "benches"]
description = ""
homepage = ""
repository = ""
documentation = "https://docs.rs/{name}"
categories = []
keywords = []
readme = "README.md"
exclude = [ "*.png" ]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = { version = "0.8.5", features = [] }

[dev-dependencies]
criterion = "0.4"

[[bench]]
name = "benchmark"
harness = false
