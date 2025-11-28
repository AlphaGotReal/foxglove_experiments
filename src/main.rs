use std::fs;
use anyhow::{Context, Result};
use camino::Utf8Path;
use memmap2::Mmap;
use mcap::McapReader;

fn read_mcap_file(path: &Utf8Path) -> Result<()> {
    let file = fs::File::open(path).context("Failed to open MCAP file")?;
    let mmap = unsafe { Mmap::map(&file).context("Failed to memory map MCAP file")? };
    let reader = McapReader::new(&mmap).context("Failed to create MCAP reader")?;

    // Iterate over messages
    for message in reader.messages() {
        let message = message.context("Failed to read message")?;
        println!("Channel: {:?}", message.channel);
        println!("Schema: {:?}", message.schema);
        println!("Timestamp: {:?}", message.log_time);
        // Access message.data for the raw message payload
        // You might need to deserialize message.data based on its schema
    }

    Ok(())
}

fn main() -> Result<()> {
    let mcap_path = Utf8Path::new("data/smd.mcap"); // Replace with your MCAP file path
    read_mcap_file(mcap_path)?;
    Ok(())
}
