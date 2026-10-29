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
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering, fence};

/// Total payload capacity of the global ring in bytes.
pub const LOG_RING_BYTES: usize = 0x10000;
const WORDS_PER_SLOT: usize = 32;
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

/// Capture an already formatted log record without a heap allocation. Long
/// records are truncated at a UTF-8 boundary and always retain a final newline.
pub fn record_log(arguments: std::fmt::Arguments<'_>) {
    use std::fmt::Write as _;
    let Some(ring) = RING.get() else { return; };
    struct Record { bytes: [u8; MAX_RECORD_BYTES], len: usize }
    impl std::fmt::Write for Record {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            let mut count = text.len().min(MAX_RECORD_BYTES - 1 - self.len);
            while !text.is_char_boundary(count) { count -= 1; }
            self.bytes[self.len..self.len + count].copy_from_slice(&text.as_bytes()[..count]);
            self.len += count;
            Ok(())
        }
    }
    let mut record = Record { bytes: [0; MAX_RECORD_BYTES], len: 0 };
    let _ = record.write_fmt(arguments);
    record.bytes[record.len] = b'\n';
    ring.write(&record.bytes[..=record.len]);
}

/// Committed records of the global ring, oldest first (empty if uninitialised).
pub fn dump_log_vec() -> Vec<u8> {
    RING.get().map(RingBuffer::dump).unwrap_or_default()
}

#[cfg(test)]
mod tests;
