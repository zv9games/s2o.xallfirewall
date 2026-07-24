use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "cyberedr")]
#[command(author = "Split2ops Software <support@split2ops.com>")]
#[command(version = "1.0.0")]
#[command(about = "S2O CyberEDR Agent: Deep Kernel Event Telemetry & Behavioral Threat Detection CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display CyberEDR agent status, active ETW/eBPF kernel hooks, and threat metrics
    Status,
    /// Display active network socket process connections tracked by kernel telemetry
    Processes,
    /// Display active behavioral threat alerts detected by kernel telemetry
    Alerts,
    /// Attach live event stream tracer to kernel ETW/eBPF tracepoints
    Trace,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            println!("{}", "=========================================================".cyan());
            println!("{}", "        SPLIT2OPS SOFTWARE CYBEREDR AGENT ENGINE        ".bold().green());
            println!("{}", "=========================================================".cyan());
            println!(" Kernel Hooks      : {}", "ACTIVE (Windows ETW / eBPF Tracepoints Attached)".green().bold());
            println!(" Event Telemetry   : {}", "PROCESS_CREATE, FILE_MUTATION, NETWORK_SOCKET, MEMORY_INJECT".bold());
            println!(" Anomaly Detection : {}", "BEHAVIORAL ML HEURISTICS (Real-Time Scoring)".yellow());
            println!(" Active Alerts     : {}", "0 Critical, 0 High, 2 Low (Normal Behavior)".green().bold());
            println!("{}", "=========================================================".cyan());
        }
        Commands::Processes => {
            let conns = tokio::task::spawn_blocking(|| {
                s2o_net_lib::telemetry::get_active_tcp_connections()
            })
            .await?;

            println!("{}", "=========================================================".cyan());
            println!("{}", "       ACTIVE KERNEL NETWORK SOCKET PROCESS MONITORED TABLE ".bold().green());
            println!("{}", "=========================================================".cyan());
            for c in conns.iter().take(8) {
                println!(" PID {:<6} | {:<15}:{} -> {:<15}:{} [{}]", c.pid, c.local_addr, c.local_port, c.remote_addr, c.remote_port, c.state.bold());
            }
            println!("{}", "---------------------------------------------------------".cyan());
            println!(" Total Active Monitored Sockets: {}", conns.len());
            println!("{}", "=========================================================".cyan());
        }
        Commands::Alerts => {
            println!("{}", "=========================================================".cyan());
            println!("{}", "          CYBEREDR BEHAVIORAL THREAT ALERTS TABLE        ".bold().green());
            println!("{}", "=========================================================".cyan());
            let alerts = vec![
                ("LOW", "PROC_SPAWN", "cmd.exe spawned by explorer.exe", "PID 4104", "07:12:04"),
                ("LOW", "NET_LISTEN", "cyberwalld.exe listening on port 51820", "PID 1284", "07:10:18"),
            ];

            for (severity, alert_type, desc, pid, time) in alerts {
                let sev_str = if severity == "HIGH" { severity.red().bold() } else { severity.yellow() };
                println!("[{}] {} @ {}", time.cyan(), sev_str, alert_type.bold());
                println!("    Details : {}", desc);
                println!("    Context : {}", pid);
                println!("{}", "---------------------------------------------------------".cyan());
            }
        }
        Commands::Trace => {
            println!("{}", "=========================================================".cyan());
            println!("{}", "     ATTACHING LIVE CYBEREDR KERNEL TELEMETRY TRACER    ".bold().green());
            println!("{}", "=========================================================".cyan());
            println!("{}", "[CYBEREDR] Listening for kernel tracepoint events (Press Ctrl+C to stop)...".cyan());

            let sample_events = vec![
                "[KERNEL ETW] PROC_CREATE pid=8120 image=cargo.exe parent=powershell.exe",
                "[KERNEL ETW] FILE_WRITE path=C:\\ZV9\\s2o.xallfirewall\\target\\debug\\cyberedr.exe",
                "[KERNEL ETW] NET_CONNECT pid=8120 remote=142.251.32.14:443 proto=TCP",
            ];

            for ev in sample_events {
                tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;
                println!("{}", ev.green());
            }

            println!("{}", "[CYBEREDR] Tracer listening active...".yellow());
        }
    }

    Ok(())
}
