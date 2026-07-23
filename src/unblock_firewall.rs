use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Resetting Windows Firewall to factory default Microsoft policy...");
    match s2o_net_lib::firewall::FirewallController::reset_to_default_policy() {
        Ok(_) => println!("Successfully restored Windows Firewall to default Microsoft factory settings (BlockInbound, AllowOutbound)!"),
        Err(e) => eprintln!("Error resetting Windows Firewall policy: {:?}", e),
    }

    match s2o_net_lib::firewall::FirewallController::enable_firewall() {
        Ok(_) => println!("Windows Firewall enabled in standard Microsoft mode."),
        Err(e) => eprintln!("Error enabling firewall: {:?}", e),
    }

    Ok(())
}
