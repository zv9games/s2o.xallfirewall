use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;

pub fn initialize_lib() {
    // Implementation of library initialization
}

pub fn initialize_modules(tx: Sender<String>) {
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

    let modules: Vec<Box<dyn Fn(Sender<String>) -> Result<(), Box<dyn std::fmt::Debug>> + Send>> = vec![
        Box::new(crate::interface1::load_module),
        // Add more modules here as needed
    ];

    for module in modules {
        if let Err(e) = module(tx.clone()) {
            eprintln!("Failed to load module: {:?}", e);
        }
        thread::sleep(Duration::from_millis(500));
    }

    if !check_fail(|| tx.send("All scripts loaded successfully. XALLFIREWALL is ready!".to_string()).map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)) {
        eprintln!("Failed to send final initialization message");
        return;
    }
}

fn initialize() {
    println!("Core processing logic initialized.");
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
