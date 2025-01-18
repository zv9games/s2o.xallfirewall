/*

CHANGELOG.rs  // Documentation of changes and updates
// This file documents all changes and updates in the project.

***BINARY INITIALIZATIONS
build.rs  // Need Initial setup
Cargo.toml  // Project configuration and dependency management
zpu.rs  // MAIN ENTRY POINT of the application, core processing logic, prepares and initializes all scripts. 
interface1.rs  // The main screen
interface2.rs // Network Security command board and other settings
ship.rs  // design and controller logic
bullets.rs  // design and movement logic
targets.rs  // design and logic
ui_processor.rs  // For future UI processes, not sure if needed. 
kernel_key.rs  // We needed this for Windivert. We have since changed toolkits, not sure if needed.
network_monitor.rs // A toolbag for having on command network status
network_controller.rs // The brain of the network/os toolbag
monitored.rs  // Not sure if needed. 
safe.rs  // Blocks all network traffic ( lock down security module)
firewall_control.rs // Firewall rules
defender_control.rs // Windows security services key. 
utils.rs  // General utility functions, a catch all if we need something.
lib.rs  // Library module exports
callbacks.rs  // Not sure if needed. Callback functions and handlers
CHANGELOG.rs  // Documentation of changes and updates

XALLFIREWALL Version 0.7.2

[package]
name = "xallfirewall"
version = "0.7.2"
authors = ["Gregory <CEO@google.com>"]
edition = "2021"

[dependencies]
pcap = "2.2.0"
eframe = { version = "0.17.0" }
tokio = { version = "1.5", features = ["full"] }
rand = "0.8"
chrono = "0.4"
winit = "0.26.1"
pacman = "1.0"

[lib]
name = "xallfirewall"
path = "src/lib.rs"

[[bin]]
name = "xallfirewall" 
path = "src/zpu.rs" 


Hi Ai,

Let's start with zpu then. We'll start with a timer, and every .5 seconds calling load module in the rest of the scripts,
awaiting a callback, and then output xallfirewall ready. 












:) */
