mod interface;
mod interface1;
mod modules;

use crossbeam_channel::{unbounded, Sender, Receiver};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting XALLFIREWALL...");

    // Create channels for communication
    let (tx_main, rx_main): (Sender<String>, Receiver<String>) = unbounded();
    let (tx_ui, _rx_ui): (Sender<String>, Receiver<String>) = unbounded();
    let (tx_ready, rx_ready): (Sender<()>, Receiver<()>) = unbounded();
    let (transition_sender, transition_receiver): (Sender<()>, Receiver<()>) = unbounded();

    // Run initialization in a separate thread
    let tx_main_clone = tx_main.clone();
    thread::spawn(move || {
        println!("Debug: Initializing library...");
        modules::initialize_lib();
        println!("Debug: Library initialized.");
        println!("Debug: Initializing modules...");
        modules::initialize_modules(tx_main_clone);
        println!("Debug: Modules initialized.");
        tx_ready.send(()).expect("Failed to send ready signal");
    });

    // Forward messages from the main channel to the UI channel
    let rx_main_clone = rx_main.clone();
    let tx_ui_clone = tx_ui.clone();
    thread::spawn(move || {
        println!("Debug: Forwarding messages...");
        forward_messages(rx_main_clone, tx_ui_clone);
    });

    // Wait for the signal that initialization is done
    if rx_ready.recv().is_ok() {
        println!("Debug: Initialization complete. Starting output window...");
        interface::create_output_window(transition_sender);

        // Wait for the transition signal to transform the window
        thread::sleep(Duration::from_secs(4)); // Sleep for 4 seconds
        if transition_receiver.recv_timeout(Duration::from_secs(1)).is_ok() {
            println!("Debug: Output window signaling completion...");
        } else {
            println!("Debug: Timeout occurred. Transforming to platform window...");
        }

        // Transform the output window to the platform window
        interface::transform_to_platform_window();
    } else {
        eprintln!("Error: Failed to receive ready signal from initialization.");
    }

    // Wait for the final confirmation message
    println!("Debug: Waiting for final confirmation...");
    wait_for_initialization_complete(rx_main);
}

fn forward_messages(rx_main: Receiver<String>, tx_ui: Sender<String>) {
    for received in rx_main {
        println!("Debug: Forwarded message: {}", received);
        tx_ui.send(received).unwrap();
    }
}

fn wait_for_initialization_complete(rx: Receiver<String>) {
    for message in rx.iter() {
        println!("Debug: Received message: {}", message);
        if message == "All scripts loaded successfully. XALLFIREWALL is ready!" {
            break;
        }
    }
    println!("Debug: Final confirmation received.");
}
