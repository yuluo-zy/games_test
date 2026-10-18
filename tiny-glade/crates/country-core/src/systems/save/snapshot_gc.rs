//! B 0x141286730 selective reference reader and 0x14172b280 GC worker.
use crossbeam_channel::Sender;
use hashbrown::HashSet;
use serde::{
    Deserialize,
    de::{DeserializeSeed, IgnoredAny, MapAccess, Visitor},
};
use std::{
    fmt, fs,
    io::{BufReader, Read},
    path::PathBuf,
    time::Instant,
};

/// Serialized reference, not a second in-memory history or snapshot owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub struct SaveFileSnapshot {
    pub snapshot_id: u64,
    pub chain_index: i32,
}

/// The native utils::json path visitor stops at its first matching key. Both
/// 0x1413e4830 and 0x1413e4990 drop the surrounding parser error, retaining a
/// successfully captured value. In particular, it never reads edit_chain.
pub fn parse_save_file_snapshots(reader: impl Read) -> Vec<SaveFileSnapshot> {
    let mut decoder = serde_json::Deserializer::from_reader(reader);
    let mut result = Vec::new();
    let _ = serde::Deserializer::deserialize_map(&mut decoder, FindHistory(&mut result));
    result
}

struct FindHistory<'a>(&'a mut Vec<SaveFileSnapshot>);
impl<'de> Visitor<'de> for FindHistory<'_> {
    type Value = ();
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a save object containing History")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        while let Some(key) = map.next_key::<String>()? {
            if key == "History" {
                *self.0 = map.next_value_seed(HistorySnapshots)?;
                return Ok(());
            }
            map.next_value::<IgnoredAny>()?;
        }
        Ok(())
    }
}
struct HistorySnapshots;
impl<'de> DeserializeSeed<'de> for HistorySnapshots {
    type Value = Vec<SaveFileSnapshot>;
    fn deserialize<D: serde::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        let mut result = Vec::new();
        let _ = decoder.deserialize_map(FindSnapshots(&mut result));
        Ok(result)
    }
}
struct FindSnapshots<'a>(&'a mut Vec<SaveFileSnapshot>);
impl<'de> Visitor<'de> for FindSnapshots<'_> {
    type Value = ();
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a history object containing snapshots")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        while let Some(key) = map.next_key::<String>()? {
            if key == "snapshots" {
                *self.0 = map.next_value()?;
                return Ok(());
            }
            map.next_value::<IgnoredAny>()?;
        }
        Ok(())
    }
}

/// Read the locked backup files, releasing each through the original channel
/// protocol, then delete unreferenced snapshot files. Dropping the receiver
/// cancels at the next release and skips the deletion phase.
pub fn snapshot_gc_thread(
    backup_paths: Vec<PathBuf>,
    snapshot_paths: Vec<PathBuf>,
    mut referenced: HashSet<u64>,
    processed_backups: Sender<PathBuf>,
) {
    let start = Instant::now();
    for path in backup_paths {
        match fs::File::open(&path) {
            Ok(file) => referenced.extend(
                parse_save_file_snapshots(BufReader::with_capacity(0x2000, file))
                    .into_iter()
                    .map(|reference| reference.snapshot_id),
            ),
            Err(error) => log::warn!("Error opening backup file {}: {error}", path.display()),
        }
        if processed_backups.send(path).is_err() {
            log::info!(
                "Save file changed before snapshot GC could finish. Shutting down the GC thread"
            );
            return;
        }
    }
    for path in snapshot_paths {
        if path.extension().and_then(|extension| extension.to_str()) != Some("snapshot") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        // Original invalid numeric names also reach deletion (0x14172bb94).
        if stem
            .parse::<u64>()
            .ok()
            .is_some_and(|id| referenced.contains(&id))
        {
            continue;
        }
        log::debug!("Deleting unreferenced snapshot {}", path.display());
        if let Err(error) = fs::remove_file(&path) {
            log::warn!(
                "Error deleting unreferenced snapshot {}: {error}",
                path.display()
            );
        }
    }
    log::debug!("Snapshot GC finished in {:?}", start.elapsed());
}
