use std::fs;

use anyhow::{Context, Result};
use camino::Utf8Path;
use memmap2::Mmap;

fn map_mcap<P: AsRef<Utf8Path>>(p: P) -> Result<Mmap> {
    let fd = fs::File::open(p.as_ref()).context("Couldn't open MCAP file")?;
    unsafe { Mmap::map(&fd) }.context("Couldn't map MCAP file")
}

fn read_it() -> Result<()> {
    let mapped = map_mcap("data/in.mcap")?;

    for message in mcap::MessageStream::new(&mapped)? {
        let msg = message?;
        println!("message: {:?}", msg);
        let channel = msg.channel;
        println!("id: {}, topic: {}", channel.id, channel.topic);
    }
    Ok(())
}

fn main() {
    read_it();
}
