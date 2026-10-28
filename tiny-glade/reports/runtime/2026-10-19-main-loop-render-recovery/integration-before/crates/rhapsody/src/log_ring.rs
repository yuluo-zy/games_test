//! Process-wide diagnostic log ring.
//!
//! The ring is a fixed array of record slots. Writers reserve a slot with one
//! `fetch_add` and publish it through a per-slot sequence word (a seqlock), so
//! writers never block or allocate and a concurrent dump never returns a torn
//! record: slots that are being written, or that were overwritten while being
//! copied, are skipped. Records longer than [`MAX_RECORD_BYTES`] are truncated.
//! Nothing in this module panics; writes before [`init_logging`] are dropped
//! and dumps of an uninitialised ring are empty.
use std::io::Write;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering, fence};

/// Total payload capacity of the global ring in bytes.
pub const LOG_RING_BYTES: usize = 0x10000;
const WORDS_PER_SLOT: usize = 7;
/// Largest payload stored for one record; longer records are truncated.
pub const MAX_RECORD_BYTES: usize = WORDS_PER_SLOT * 8 - 1;

struct Slot {
    /// `0`: never written; odd: being written; even `2 * (index + 1)`: committed.
    sequence: AtomicU64,
    /// Little-endian payload words; the last byte holds the payload length.
    words: [AtomicU64; WORDS_PER_SLOT],
}
impl Slot {
    fn new() -> Self {
        Self {
            sequence: AtomicU64::new(0),
            words: std::array::from_fn(|_| AtomicU64::new(0)),
        }
    }
}

/// A bounded multi-writer record ring with torn-record detection.
pub struct RingBuffer {
    slots: Box<[Slot]>,
    next: AtomicU64,
}
impl RingBuffer {
    /// `capacity_bytes` is rounded down to whole slots (at least one slot).
    pub fn new(capacity_bytes: usize) -> Self {
        let count = (capacity_bytes / size_of::<Slot>()).max(1);
        Self {
            slots: (0..count).map(|_| Slot::new()).collect(),
            next: AtomicU64::new(0),
        }
    }
    /// Number of records retained before the oldest is overwritten.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    /// Total number of records ever written.
    pub fn written(&self) -> u64 {
        self.next.load(Ordering::Relaxed)
    }
    /// Append one record, truncating it to [`MAX_RECORD_BYTES`].
    pub fn write(&self, bytes: &[u8]) {
        let bytes = &bytes[..bytes.len().min(MAX_RECORD_BYTES)];
        let index = self.next.fetch_add(1, Ordering::Relaxed);
        let slot = &self.slots[(index % self.slots.len() as u64) as usize];
        // Writers lapping each other on one slot is harmless: a reader only
        // accepts a slot whose sequence is even and unchanged around its copy.
        slot.sequence
            .store(index.wrapping_mul(2).wrapping_add(1), Ordering::Relaxed);
        fence(Ordering::Release);
        let mut packed = [0u8; WORDS_PER_SLOT * 8];
        packed[..bytes.len()].copy_from_slice(bytes);
        packed[MAX_RECORD_BYTES] = bytes.len() as u8;
        for (word, chunk) in slot.words.iter().zip(packed.chunks_exact(8)) {
            let mut value = [0u8; 8];
            value.copy_from_slice(chunk);
            word.store(u64::from_le_bytes(value), Ordering::Relaxed);
        }
        slot.sequence
            .store(index.wrapping_add(1).wrapping_mul(2), Ordering::Release);
    }
    /// Committed records, oldest first. Torn or in-progress records are skipped.
    pub fn records(&self) -> Vec<Vec<u8>> {
        let mut records: Vec<(u64, Vec<u8>)> = Vec::with_capacity(self.slots.len());
        for slot in self.slots.iter() {
            let before = slot.sequence.load(Ordering::Acquire);
            if before == 0 || before % 2 == 1 {
                continue;
            }
            let mut packed = [0u8; WORDS_PER_SLOT * 8];
            for (word, chunk) in slot.words.iter().zip(packed.chunks_exact_mut(8)) {
                chunk.copy_from_slice(&word.load(Ordering::Relaxed).to_le_bytes());
            }
            fence(Ordering::Acquire);
            if slot.sequence.load(Ordering::Relaxed) != before {
                continue;
            }
            let len = usize::from(packed[MAX_RECORD_BYTES]).min(MAX_RECORD_BYTES);
            records.push((before / 2, packed[..len].to_vec()));
        }
        records.sort_unstable_by_key(|(order, _)| *order);
        records.into_iter().map(|(_, bytes)| bytes).collect()
    }
    /// Concatenated committed records, oldest first.
    pub fn dump(&self) -> Vec<u8> {
        self.records().concat()
    }
}

static RING: OnceLock<RingBuffer> = OnceLock::new();

/// Create the global ring. Repeated calls are no-ops.
pub fn init_logging() {
    RING.get_or_init(|| RingBuffer::new(LOG_RING_BYTES));
}

/// Append a record to the global ring; dropped while logging is uninitialised.
pub fn write(bytes: &[u8]) {
    if let Some(ring) = RING.get() {
        ring.write(bytes);
    }
}

/// `[u64 payload][kind]` record.
pub fn record_event(kind: u8, payload: u64) {
    let mut bytes = [0u8; 9];
    bytes[..8].copy_from_slice(&payload.to_le_bytes());
    bytes[8] = kind;
    write(&bytes);
}

/// `[name: 16 bytes, zero padded][u64 payload]?[kind]` record. The name is
/// truncated at a UTF-8 character boundary.
pub fn record_named(name: &str, payload: Option<u64>, kind: u8) {
    let mut end = name.len().min(16);
    while !name.is_char_boundary(end) {
        end -= 1;
    }
    let mut bytes = [0u8; 25];
    bytes[..end].copy_from_slice(&name.as_bytes()[..end]);
    let mut len = 16;
    if let Some(payload) = payload {
        bytes[16..24].copy_from_slice(&payload.to_le_bytes());
        len = 24;
    }
    bytes[len] = kind;
    write(&bytes[..=len]);
}

/// Committed records of the global ring, oldest first (empty if uninitialised).
pub fn dump_log_vec() -> Vec<u8> {
    RING.get().map(RingBuffer::dump).unwrap_or_default()
}

pub fn dump_log(out: &mut impl Write) -> std::io::Result<()> {
    out.write_all(&dump_log_vec())
}

pub fn print_log_dump() -> std::io::Result<()> {
    dump_log(&mut std::io::stderr().lock())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn records_are_ordered_truncated_and_never_panic() {
        let ring = RingBuffer::new(3 * size_of::<Slot>());
        assert_eq!(ring.capacity(), 3);
        assert!(ring.dump().is_empty());
        ring.write(b"abc");
        ring.write(b"");
        ring.write(&[7; 200]);
        assert_eq!(ring.records()[0], b"abc");
        assert_eq!(ring.records()[2].len(), MAX_RECORD_BYTES);
        ring.write(b"d");
        assert_eq!(
            ring.records(),
            vec![Vec::new(), vec![7; MAX_RECORD_BYTES], b"d".to_vec()]
        );
        assert_eq!(ring.written(), 4);
        assert_eq!(RingBuffer::new(0).capacity(), 1);
    }

    #[test]
    fn torn_slots_are_skipped() {
        let ring = RingBuffer::new(2 * size_of::<Slot>());
        ring.write(b"ok");
        ring.write(b"torn");
        // A writer that reserved slot 1 and has not committed yet.
        ring.slots[1].sequence.store(5, Ordering::Release);
        assert_eq!(ring.records(), vec![b"ok".to_vec()]);
    }

    #[test]
    fn concurrent_writers_produce_only_whole_records() {
        let ring = Arc::new(RingBuffer::new(64 * size_of::<Slot>()));
        let workers: Vec<_> = (0..4u8)
            .map(|id| {
                let ring = ring.clone();
                std::thread::spawn(move || {
                    for _ in 0..2000 {
                        ring.write(&[id; 32]);
                    }
                })
            })
            .collect();
        for _ in 0..50 {
            for record in ring.records() {
                assert_eq!(record.len(), 32);
                assert!(record.iter().all(|&b| b == record[0]));
            }
        }
        for worker in workers {
            worker.join().unwrap();
        }
        assert_eq!(ring.records().len(), 64);
    }

    #[test]
    fn global_ring_is_idempotent_and_formats_named_records() {
        write(b"may be dropped before init");
        init_logging();
        init_logging();
        record_named("abcdefghijklmn\u{4e2d}", Some(9), 3);
        record_event(0x1e, 7);
        let out = dump_log_vec();
        assert_eq!(&out[out.len() - 9..], &[7, 0, 0, 0, 0, 0, 0, 0, 0x1e]);
        let named = &out[out.len() - 34..out.len() - 9];
        assert_eq!(&named[..14], b"abcdefghijklmn");
        assert_eq!(
            &named[14..16],
            &[0, 0],
            "a split UTF-8 character is not stored"
        );
        assert_eq!(&named[16..24], &9u64.to_le_bytes());
        assert_eq!(named[24], 3);
    }
}
