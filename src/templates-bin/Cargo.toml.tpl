[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
authors = ["{author}"]
rust-version = "1.73"
license = "MIT OR Apache-2.0"

[[bin]]
name = "{name}"
path = "src/main.rs"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = { version = "0.8.5", features = [] }
