//! Process-wide diagnostic log ring.
//!
//! The ring is a fixed array of record slots. Writers reserve a slot with one
//! `fetch_add`, claim exclusive write access with one per-slot compare_exchange,
//! then publish through that slot's sequence word. Busy, superseded or exhausted
//! reservations are dropped without waiting, retrying or allocating; see
//! [`RingBuffer::dropped`]. A concurrent dump only returns whole records: slots
//! that are being written, or that changed while being copied, are skipped. Records longer than [`MAX_RECORD_BYTES`] are truncated.
//! Nothing in this module panics; writes before [`init_logging`] are dropped
//! and dumps of an uninitialised ring are empty.
use std::io::Write;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering, fence};

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
    dropped: AtomicU64,
    exhausted: AtomicBool,
}
impl RingBuffer {
    /// `capacity_bytes` is rounded down to whole slots (at least one slot).
    pub fn new(capacity_bytes: usize) -> Self {
        let count = (capacity_bytes / size_of::<Slot>()).max(1);
        Self {
            slots: (0..count).map(|_| Slot::new()).collect(),
            next: AtomicU64::new(0),
            dropped: AtomicU64::new(0),
            exhausted: AtomicBool::new(false),
        }
    }
    /// Number of records retained before the oldest is overwritten.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    /// Number of reservation attempts, including in-flight and dropped writes.
    /// This preserves the original counter; it wraps after `u64::MAX` attempts.
    pub fn written(&self) -> u64 {
        self.next.load(Ordering::Relaxed)
    }
    /// Number of rejected busy, stale, contended or exhausted reservations.
    /// A concurrent snapshot need not include attempts still in flight.
    pub fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }
    /// Best-effort append, truncating to [`MAX_RECORD_BYTES`]. A reservation
    /// loses immediately if its slot is busy, superseded, or claimed by another
    /// writer. `written()` counts the attempt; `dropped()` counts the rejection.
    pub fn write(&self, bytes: &[u8]) {
        self.write_with(bytes, |_| {}, |_| {});
    }
    /// Private scheduling hooks let tests stop the actual production protocol
    /// after reservation or payload stores. No-op closures have no dynamic
    /// dispatch and disappear from the optimized production instantiation.
    #[inline]
    fn write_with(
        &self,
        bytes: &[u8],
        reserved: impl FnOnce(u64),
        mut stored_word: impl FnMut(usize),
    ) -> bool {
        let bytes = &bytes[..bytes.len().min(MAX_RECORD_BYTES)];
        let index = self.next.fetch_add(1, Ordering::Relaxed);
        reserved(index);
        // Never reuse a publication sequence, even on counter exhaustion.
        // A repeated even sequence could otherwise hide an intervening writer.
        let committed = index.checked_add(1).and_then(|n| n.checked_mul(2));
        let Some(committed) = committed.filter(|_| !self.exhausted.load(Ordering::Acquire)) else {
            self.exhausted.store(true, Ordering::Release);
            self.dropped.fetch_add(1, Ordering::Relaxed);
            return false;
        };
        let slot = &self.slots[(index % self.slots.len() as u64) as usize];
        let observed = slot.sequence.load(Ordering::Relaxed);
        // One successful CAS owns all seven payload words until publication.
        // The order check also rejects an old reservation delayed until after
        // a newer writer committed to this same slot.
        if observed % 2 == 1
            || observed >= committed
            || slot
                .sequence
                .compare_exchange(observed, committed - 1, Ordering::AcqRel, Ordering::Relaxed)
                .is_err()
        {
            self.dropped.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        // Paired with the reader's Acquire fence through any payload word it
        // reads: observing this writer's data also exposes its odd sequence.
        fence(Ordering::Release);
        let mut packed = [0u8; WORDS_PER_SLOT * 8];
        packed[..bytes.len()].copy_from_slice(bytes);
        packed[MAX_RECORD_BYTES] = bytes.len() as u8;
        for (i, (word, chunk)) in slot.words.iter().zip(packed.chunks_exact(8)).enumerate() {
            let mut value = [0u8; 8];
            value.copy_from_slice(chunk);
            word.store(u64::from_le_bytes(value), Ordering::Relaxed);
            stored_word(i);
        }
        slot.sequence.store(committed, Ordering::Release);
        true
    }
    /// Committed records in reservation order. This is a best-effort collection,
    /// not an atomic snapshot of every slot; changed/in-progress slots are skipped.
    pub fn records(&self) -> Vec<Vec<u8>> {
        self.records_with(|_, _| {})
    }
    #[inline]
    fn records_with(&self, mut copied_word: impl FnMut(usize, usize)) -> Vec<Vec<u8>> {
        let mut records: Vec<(u64, Vec<u8>)> = Vec::with_capacity(self.slots.len());
        for (slot_index, slot) in self.slots.iter().enumerate() {
            let before = slot.sequence.load(Ordering::Acquire);
            if before == 0 || before % 2 == 1 {
                continue;
            }
            let mut packed = [0u8; WORDS_PER_SLOT * 8];
            for (i, (word, chunk)) in slot
                .words
                .iter()
                .zip(packed.chunks_exact_mut(8))
                .enumerate()
            {
                chunk.copy_from_slice(&word.load(Ordering::Relaxed).to_le_bytes());
                copied_word(slot_index, i);
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
    fn overlapping_writer_is_dropped_before_touching_payload() {
        use std::sync::mpsc;
        let ring = Arc::new(RingBuffer::new(size_of::<Slot>()));
        let (entered_tx, entered_rx) = mpsc::sync_channel(0);
        let (resume_tx, resume_rx) = mpsc::sync_channel(0);
        let writer = {
            let ring = ring.clone();
            std::thread::spawn(move || {
                ring.write_with(
                    &[0xa5; MAX_RECORD_BYTES],
                    |_| {},
                    |i| {
                        if i == 0 {
                            entered_tx.send(()).unwrap();
                            resume_rx.recv().unwrap();
                        }
                    },
                )
            })
        };
        entered_rx.recv().unwrap();
        // A has written only word0 and still owns this single slot. A second
        // writer must return without altering any payload word or sequence.
        ring.write(&[0x5a; MAX_RECORD_BYTES]);
        let during = ring.records();
        let counts = (ring.written(), ring.dropped());
        resume_tx.send(()).unwrap();
        assert!(writer.join().unwrap());
        assert!(during.is_empty());
        assert_eq!(counts, (2, 1));
        assert_eq!(ring.records(), vec![vec![0xa5; MAX_RECORD_BYTES]]);
    }

    #[test]
    fn delayed_reservation_cannot_replace_a_newer_record() {
        use std::sync::mpsc;
        let ring = Arc::new(RingBuffer::new(size_of::<Slot>()));
        let (entered_tx, entered_rx) = mpsc::sync_channel(0);
        let (resume_tx, resume_rx) = mpsc::sync_channel(0);
        let older = {
            let ring = ring.clone();
            std::thread::spawn(move || {
                ring.write_with(
                    b"older",
                    |_| {
                        entered_tx.send(()).unwrap();
                        resume_rx.recv().unwrap();
                    },
                    |_| {},
                )
            })
        };
        entered_rx.recv().unwrap();
        ring.write(b"newer");
        resume_tx.send(()).unwrap();
        assert!(!older.join().unwrap());
        assert_eq!(ring.written(), 2);
        assert_eq!(ring.dropped(), 1);
        assert_eq!(ring.records(), vec![b"newer".to_vec()]);
    }

    #[test]
    fn a_slot_overwritten_mid_copy_is_skipped() {
        use std::sync::mpsc;
        let ring = Arc::new(RingBuffer::new(size_of::<Slot>()));
        ring.write(&[0xa5; MAX_RECORD_BYTES]);
        let (entered_tx, entered_rx) = mpsc::sync_channel(0);
        let (resume_tx, resume_rx) = mpsc::sync_channel(0);
        let reader = {
            let ring = ring.clone();
            std::thread::spawn(move || {
                ring.records_with(|_, word| {
                    if word == 0 {
                        entered_tx.send(()).unwrap();
                        resume_rx.recv().unwrap();
                    }
                })
            })
        };
        entered_rx.recv().unwrap();
        ring.write(&[0x5a; MAX_RECORD_BYTES]);
        resume_tx.send(()).unwrap();
        assert!(reader.join().unwrap().is_empty());
        assert_eq!(ring.records(), vec![vec![0x5a; MAX_RECORD_BYTES]]);
        assert_eq!((ring.written(), ring.dropped()), (2, 0));
    }

    #[test]
    fn exhausted_tokens_never_reuse_a_publication_sequence() {
        let ring = RingBuffer::new(size_of::<Slot>());
        ring.next.store(u64::MAX / 2 - 1, Ordering::Relaxed);
        ring.write(b"last representable");
        let committed = ring.slots[0].sequence.load(Ordering::Relaxed);
        assert_eq!(committed, u64::MAX - 1);
        ring.write(b"not representable");
        assert_eq!(ring.dropped(), 1);
        assert_eq!(ring.slots[0].sequence.load(Ordering::Relaxed), committed);
        // Simulate eventual attempt-counter wrap: the terminal state must
        // still refuse publication, rather than reusing an early even stamp.
        ring.next.store(0, Ordering::Relaxed);
        ring.write(b"wrapped");
        assert_eq!(ring.dropped(), 2);
        assert_eq!(ring.records(), vec![b"last representable".to_vec()]);
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
