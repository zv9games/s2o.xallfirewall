CHANGELOG

[package]
name = "xallfirewall"
version = "0.7.2"
authors = ["Gregory <gregor.whitelight@gmail.com>"]
edition = "2021"

[dependencies]

eframe = { version = "0.17.0" }
tokio = { version = "1.5", features = ["full"] }
rand = "0.8"
chrono = "0.4"
winit = "0.26.1"
crossbeam = "0.8.0"
crossbeam-channel = "0.5.0"
windivert = { version = "0.6.0", features = ["vendored"] }
windivert-sys = "0.10.0"


[build]
rustc-link-lib = ["WinDivert"]


[[bin]]
name = "xallfirewall"
path = "src/main.rs"

[[bin]]
name = "loader"
path = "src/interface1.rs"

[[bin]]
name = "platform"
path = "src/interface2.rs"

[[bin]]
name = "network"
path = "src/interface3.rs"

[[bin]]
name = "security"
path = "src/interface4.rs"


project files:
cargo.toml
build.rs
main.rs
interface1.rs
interface2.rs
interface3.rs
interface4.rs
ship.rs
bullets.rs
targets.rs

