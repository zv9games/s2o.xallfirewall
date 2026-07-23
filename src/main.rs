use std::sync::atomic::{AtomicBool};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::error::Error;
use eframe::{egui, epi};
use winapi::um::winsock2::{WSADATA, WSAStartup, SOCKET, INVALID_SOCKET, SOCK_RAW, socket, WSAGetLastError};
use winapi::shared::ws2def::{AF_INET, IPPROTO_IP};

// Helper stub for packet capture while divert/raw socket capture is shelved
fn capture_packets<F>(_socket: SOCKET, _handle_packet: F, _stop_signal: Arc<AtomicBool>)
where
    F: Fn(&[u8]) + Send + Sync + 'static,
{
    // Wireframe stub for network packet capture callback
}

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

    // Try creating raw socket (requires Administrator privileges)
    let socket: SOCKET = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_IP as i32) };
    if socket == INVALID_SOCKET {
        let err_code = unsafe { WSAGetLastError() };
        eprintln!(
            "Notice: Raw socket creation returned error {} (WSAEACCES: Access Denied).",
            err_code
        );
        eprintln!("Raw packet capture requires Administrator privileges.");
        eprintln!("  -> To run with Administrator privileges:");
        eprintln!("     1. Right-click PowerShell or CMD and select 'Run as Administrator'.");
        eprintln!("     2. Or in PowerShell, run: Start-Process powershell -Verb RunAs");
        eprintln!("  -> Continuing in driverless IP Helper telemetry mode...\n");
    } else {
        let stop_signal = Arc::new(AtomicBool::new(false));
        tokio::spawn({
            let tx = tx.clone();
            async move {
                capture_packets(socket, move |packet| handle_packet(tx.clone(), packet), stop_signal);
            }
        });
    }

    let app = NetworkApp { rx };
    eframe::run_native(Box::new(app), eframe::NativeOptions::default());
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
            ui.heading("Network Activity Dashboard");
            ui.add_space(5.0);

            let connections = s2o_net_lib::telemetry::get_active_tcp_connections();
            ui.label(format!("Active TCP Sockets (Driverless Telemetry): {}", connections.len()));
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Ok(mut rx) = self.rx.try_lock() {
                    while let Ok(packet_data) = rx.try_recv() {
                        ui.label(format!("[Raw Packet] {}", packet_data));
                    }
                }

                for conn in &connections {
                    ui.horizontal(|ui| {
                        ui.label(format!("PID {:<6}", conn.pid));
                        ui.label(format!("{}:{} -> {}:{}", conn.local_addr, conn.local_port, conn.remote_addr, conn.remote_port));
                        ui.label(format!("[{}]", conn.state));
                    });
                }
            });
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
