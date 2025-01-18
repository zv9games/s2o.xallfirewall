use s2o_net_lib::capture::capture_packets;
use std::sync::atomic::{AtomicBool};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::error::Error;
use eframe::{egui, epi};
use winapi::um::winsock2::{WSADATA, WSAStartup, WSACleanup, SOCKET, INVALID_SOCKET, SOCK_RAW, IPPROTO_IP, socket, WSAGetLastError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel(100);
    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    // Initialize Winsock
    let mut wsa_data: WSADATA = unsafe { std::mem::zeroed() };
    if unsafe { WSAStartup(0x0202, &mut wsa_data) } != 0 {
        eprintln!("WSAStartup failed with error: {}", unsafe { WSAGetLastError() });
        return Err("WSAStartup failed".into());
    }

    // Create the socket
    let socket: SOCKET = unsafe { socket(winapi::um::winsock2::AF_INET, SOCK_RAW, IPPROTO_IP) };
    if socket == INVALID_SOCKET {
        eprintln!("Failed to create socket with error: {}", unsafe { WSAGetLastError() });
        unsafe { WSACleanup() };
        return Err("Failed to create socket".into());
    }

    let stop_signal = Arc::new(AtomicBool::new(false));

    tokio::spawn({
        let tx = tx.clone();
        let stop_signal = stop_signal.clone();
        async move {
            capture_packets(socket, move |packet| handle_packet(tx.clone(), packet), stop_signal);
        }
    });

    let app = NetworkApp { rx };
    eframe::run_native(Box::new(app), eframe::NativeOptions::default());

    // Cleanup Winsock
    unsafe { WSACleanup() };

    // Ok(()) can be removed because it's unreachable
}

struct NetworkApp {
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl epi::App for NetworkApp {
    fn name(&self) -> &str {
        "Network Activity Dashboard"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(mut rx) = self.rx.try_lock() {
                ui.label("Network Activity:");
                let mut packet_received = false;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    while let Ok(packet_data) = rx.try_recv() {
                        ui.label(packet_data);
                        packet_received = true;
                    }
                });

                if !packet_received {
                    ui.label("No packets captured yet.");
                }
            }
        });

        ctx.request_repaint();
    }
}

// Helper function to send packet data
fn handle_packet(tx: Arc<Mutex<mpsc::Sender<String>>>, packet: &[u8]) {
    let packet_data_str = format!("{:?}", packet);
    let tx_cloned = tx.clone();
    tokio::spawn(async move {
        tx_cloned.lock().await.send(packet_data_str).await.unwrap();
    });
}
