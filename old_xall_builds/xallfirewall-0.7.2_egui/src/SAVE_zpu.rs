use crossbeam_channel::{unbounded, Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use xallfirewall_lib::interface1;
use xallfirewall_lib::interface2;
use xallfirewall_lib::ship;
use xallfirewall_lib::bullets;
use xallfirewall_lib::targets;
use xallfirewall_lib::ui_processor;
use xallfirewall_lib::kernel_key;
use xallfirewall_lib::network_monitor;
use xallfirewall_lib::network_controller;
use xallfirewall_lib::monitored;
use xallfirewall_lib::safe;
use xallfirewall_lib::firewall_control;
use xallfirewall_lib::defender_control;
use xallfirewall_lib::utils;
use xallfirewall_lib::callbacks;
use xallfirewall_lib::initialize_lib;

fn main() {
    println!("Starting XALLFIREWALL...");

    // Create channels for communication
    let (tx_main, rx_main): (Sender<String>, Receiver<String>) = unbounded();
    let (tx_ui, rx_ui): (Sender<String>, Receiver<String>) = unbounded();

    // Initialize the library and modules
    initialize_lib();
    initialize_modules(tx_main.clone());

    // Forward messages from the main channel to the UI channel
    let rx_main_clone = rx_main.clone();
    let tx_ui_clone = tx_ui.clone();
    thread::spawn(move || {
        for received in rx_main_clone {
            tx_ui_clone.send(received).unwrap();
        }
    });

    // Create a flag to check if interface1 is loaded
    let interface1_loaded = Arc::new(AtomicBool::new(false));
    let interface1_loaded_clone = Arc::clone(&interface1_loaded);

    // Start the output window on the main thread
    interface1::create_output_window(rx_ui);
    interface1_loaded.store(true, Ordering::SeqCst);

    // Wait until interface1 is fully loaded
    while !interface1_loaded.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    println!("Interface1 window loaded and ready.");

    // Wait for the final confirmation message before launching interface2
    wait_for_initialization_complete(rx_main);

    // Launch the interface2 window
    if !check_fail(|| {
        interface2::create_interface2_window();
        Ok(())
    }) {
        eprintln!("Failed to load interface2 window.");
    }
}

fn initialize_modules(tx: Sender<String>) {
    if !check_fail(|| tx.send("Initializing core processing logic...".to_string()).map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)) {
        eprintln!("Failed to send initialization message");
        return;
    }

    initialize();

    if !check_fail(|| tx.send("zpu module initialized.".to_string()).map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)) {
        eprintln!("Failed to send ZPU module initialization message");
        return;
    }

    thread::sleep(Duration::from_millis(500));

    // Load all modules with check/fail mechanism
    let modules: Vec<Box<dyn Fn(Sender<String>) -> Result<(), Box<dyn std::fmt::Debug>> + Send>> = vec![
        Box::new(interface1::load_module),
        Box::new(ship::load_module),
        Box::new(bullets::load_module),
        Box::new(targets::load_module),
        Box::new(ui_processor::load_module),
        Box::new(kernel_key::load_module),
        Box::new(network_monitor::load_module),
        Box::new(network_controller::load_module),
        Box::new(monitored::load_module),
        Box::new(safe::load_module),
        Box::new(firewall_control::load_module),
        Box::new(defender_control::load_module),
        Box::new(utils::load_module),
        Box::new(callbacks::load_module),
    ];

    for module in modules {
        if let Err(e) = module(tx.clone()) {
            eprintln!("Failed to load module: {:?}", e);
        }
        thread::sleep(Duration::from_millis(500));
    }

    // Final callback to confirm all initializations are complete
    if !check_fail(|| tx.send("All scripts loaded successfully. XALLFIREWALL is ready!".to_string()).map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)) {
        eprintln!("Failed to send final initialization message");
        return;
    }
}

fn initialize() {
    println!("Core processing logic initialized.");
    // Add core initialization logic here
}

fn check_fail<F>(func: F) -> bool
where
    F: Fn() -> Result<(), Box<dyn std::fmt::Debug>> + Copy,
{
    for _ in 0..5 {
        let result = func();
        if result.is_ok() {
            return true;
        }
        thread::sleep(Duration::from_millis(500));
    }
    false
}

fn wait_for_initialization_complete(rx: Receiver<String>) {
    for message in rx.iter() {
        if message == "All scripts loaded successfully. XALLFIREWALL is ready!" {
            break;
        }
    }
}
