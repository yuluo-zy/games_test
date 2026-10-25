//! Validate a saved events response without opening a network connection.
use anyhow::{Context, Result, ensure};
use std::{fs::File, io::Read};
use tg_theme::protocol::EventsResponse;

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let path = args
        .next()
        .context("usage: replay_schedule <events-response.json>")?;
    ensure!(args.next().is_none(), "expected one events response path");
    let mut bytes = Vec::new();
    File::open(path)?
        .take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 1024 * 1024, "events response exceeds 1 MiB");
    let response: EventsResponse = serde_json::from_slice(&bytes)?;
    let events = response.validate(0, 1024)?;
    println!("Validated {} scheduled events", events.len());
    for event in events {
        println!(
            "{}: {:?} .. {:?}",
            event.cid, event.start_time, event.end_time
        );
    }
    Ok(())
}
