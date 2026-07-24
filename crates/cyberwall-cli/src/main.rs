use clap::{Parser, Subcommand};
use colored::*;
use cyberwall_backend_windows::WindowsFirewallEngine;
use cyberwall_core::FirewallEngine;

#[derive(Parser)]
#[command(name = "cyberwall")]
#[command(author = "Split2ops Software <support@split2ops.com>")]
#[command(version = "1.0.0")]
#[command(about = "Split2ops Cyberwall Enterprise Commercial Firewall CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display live OS firewall status, profile breakdown, and backend engine info
    Status {
        /// Output status as JSON
        #[arg(long)]
        json: bool,
    },
    /// Enable the OS firewall across all profiles
    Enable,
    /// Disable the OS firewall across all profiles
    Disable,
    /// Engage emergency outbound isolation shield (airplane/lockdown mode)
    Lock,
    /// Disengage outbound isolation shield
    Unlock,
    /// List active OS firewall filtering rules
    Rules,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let engine = WindowsFirewallEngine::new();

    match cli.command {
        Commands::Status { json } => {
            let status = engine.get_status().await?;
            if json {
                println!("{}", serde_json::to_string_pretty(&status)?);
            } else {
                println!("{}", "=========================================================".cyan());
                println!("{}", "       SPLIT2OPS SOFTWARE CYBERWALL ENTERPRISE CLI       ".bold().green());
                println!("{}", "=========================================================".cyan());
                println!(" Platform Engine   : {}", status.platform.bold());
                println!(" Backend Driver    : {}", status.backend_driver.yellow());
                println!(" Firewall Status   : {}", if status.enabled { "ENABLED (Green)".green().bold() } else { "DISABLED (Red)".red().bold() });
                println!(" Outbound Shield   : {}", if status.outbound_blocked { "BLOCKED (Red)".red().bold() } else { "NORMAL (Allow)".green() });
                println!(" Windows Defender  : {}", if status.defender_active { "ACTIVE (Green)".green() } else { "INACTIVE (Red)".red() });
                println!("{}", "---------------------------------------------------------".cyan());
                println!(" Private Profile   : {}", if status.profile_private { "ON".green() } else { "OFF".red() });
                println!(" Public Profile    : {}", if status.profile_public { "ON".green() } else { "OFF".red() });
                println!(" Domain Profile    : {}", if status.profile_domain { "ON".green() } else { "OFF".red() });
                println!("{}", "=========================================================".cyan());
            }
        }
        Commands::Enable => {
            println!("[CYBERWALL CLI] Transmitting ENABLE signal to OS Kernel...");
            engine.set_enabled(true).await?;
            println!("{}", "[CYBERWALL CLI] SUCCESS: OS Firewall enabled across all profiles.".green().bold());
        }
        Commands::Disable => {
            println!("[CYBERWALL CLI] Transmitting DISABLE signal to OS Kernel...");
            engine.set_enabled(false).await?;
            println!("{}", "[CYBERWALL CLI] SUCCESS: OS Firewall disabled across all profiles.".yellow().bold());
        }
        Commands::Lock => {
            println!("[CYBERWALL CLI] Engaging Emergency Outbound Isolation Shield...");
            engine.set_outbound_block(true).await?;
            println!("{}", "[CYBERWALL CLI] ALERT: Outbound network traffic is now BLOCKED!".red().bold());
        }
        Commands::Unlock => {
            println!("[CYBERWALL CLI] Disengaging Outbound Isolation Shield...");
            engine.set_outbound_block(false).await?;
            println!("{}", "[CYBERWALL CLI] SUCCESS: Outbound network traffic RESTORED.".green().bold());
        }
        Commands::Rules => {
            let rules = engine.list_rules().await?;
            println!("{}", "=========================================================".cyan());
            println!("{}", "            SPLIT2OPS ACTIVE FIREWALL RULES             ".bold().green());
            println!("{}", "=========================================================".cyan());
            for (idx, rule) in rules.iter().enumerate() {
                println!("{}. Rule Name : {}", idx + 1, rule.name.bold());
                println!("   Action    : {:?}", rule.action);
                println!("   Direction : {:?}", rule.direction);
                println!("   App Path  : {:?}", rule.application);
                println!("{}", "---------------------------------------------------------".cyan());
            }
        }
    }

    Ok(())
}
