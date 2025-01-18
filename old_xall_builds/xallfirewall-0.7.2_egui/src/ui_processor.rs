use crossbeam_channel::Sender;

pub fn load_module(tx: Sender<String>) -> Result<(), Box<dyn std::fmt::Debug>> {
    tx.send("ui_processor module loaded.".to_string()).map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)?;
    println!("Sent: ui_processor module loaded.");
    Ok(())
}
