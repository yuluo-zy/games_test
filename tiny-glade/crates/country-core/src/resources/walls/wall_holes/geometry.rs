//! Game WallHoleStorage lifecycle and geometry policy. std owns hashing/sort/Vec.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawWallHole {
    pub center: [f32; 2],
    pub dimensions: [f32; 2],
    /// Original HoleType byte. Producers identify its variants separately.
    pub kind: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FinalizedWallHole {
    /// [minimum x, minimum y, maximum x, maximum y].
    pub aabb: [f32; 4],
    pub kind: u8,
    /// Original origin discriminants: 0 primary, 1 shifted wrap duplicate.
    pub origin: u8,
}
#[derive(Clone, Debug)]
pub struct PreviousWallHoles {
    pub holes: Vec<FinalizedWallHole>,
    pub wall_length: f32,
    pub closed: bool,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HoleFingerprint {
    pub sum: u64,
    pub xor: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoleStoragePhase {
    InProgress,
    Finalized,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoleStorageError {
    AlreadyFinalized,
    NotFinalized,
    InvalidDimensions,
    NonOrderableBounds,
}

/// Original private calculate_hash schema; SipHash implementation stays in std.
/// Arrays are hashed with their original u64 length prefixes; floats use raw bits.
pub fn raw_hole_hash(hole: RawWallHole) -> u64 {
    let mut hash = DefaultHasher::new();
    for vector in [hole.center, hole.dimensions] {
        hash.write(&2u64.to_le_bytes());
        hash.write(&vector[0].to_bits().to_le_bytes());
        hash.write(&vector[1].to_bits().to_le_bytes());
    }
    hash.write(&(hole.kind as u64).to_le_bytes());
    hash.finish()
}
fn add(a: f32, b: f32) -> f32 {
    let r = a + b;
    if r.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        r
    }
}
fn subtract(a: f32, b: f32) -> f32 {
    let r = a - b;
    if r.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        r
    }
}

#[derive(Clone, Debug)]
pub struct WallHoleStorage {
    phase: HoleStoragePhase,
    raw: Vec<RawWallHole>,
    finalized: Vec<FinalizedWallHole>,
    fingerprint: HoleFingerprint,
    previous_fingerprint: HoleFingerprint,
    wall_length: f32,
    closed: bool,
}
impl Default for WallHoleStorage {
    fn default() -> Self {
        Self::new_in_progress()
    }
}
impl WallHoleStorage {
    pub fn new_in_progress() -> Self {
        Self {
            phase: HoleStoragePhase::InProgress,
            raw: Vec::new(),
            finalized: Vec::new(),
            fingerprint: HoleFingerprint::default(),
            previous_fingerprint: HoleFingerprint::default(),
            wall_length: 0.0,
            closed: false,
        }
    }
    pub fn phase(&self) -> HoleStoragePhase {
        self.phase
    }
    pub fn fingerprint(&self) -> HoleFingerprint {
        self.fingerprint
    }
    pub fn previous_fingerprint(&self) -> HoleFingerprint {
        self.previous_fingerprint
    }
    pub fn raw_holes(&self) -> &[RawWallHole] {
        &self.raw
    }
    pub fn finalized_holes(&self) -> Result<&[FinalizedWallHole], HoleStorageError> {
        if self.phase != HoleStoragePhase::Finalized {
            return Err(HoleStorageError::NotFinalized);
        }
        Ok(&self.finalized)
    }
    pub fn wall_length(&self) -> Result<f32, HoleStorageError> {
        if self.phase != HoleStoragePhase::Finalized {
            return Err(HoleStorageError::NotFinalized);
        }
        Ok(self.wall_length)
    }
    pub fn closed(&self) -> bool {
        self.closed
    }
    /// Original clones both source and cached geometry from either enum variant,
    /// keeps both fingerprints and cached parameters, and reopens the enum.
    pub fn clone_and_reopen(&self) -> Self {
        let mut copy = self.clone();
        copy.phase = HoleStoragePhase::InProgress;
        copy
    }
    /// Original outer WallHoles::clear_for_next_frame resolved row transition.
    /// A finalized row snapshots its geometry, keeps the cache and advances the
    /// old current fingerprint into previous. An unfinished row clears both.
    pub fn advance_frame(&mut self) -> Option<PreviousWallHoles> {
        let previous = if self.phase == HoleStoragePhase::Finalized {
            self.previous_fingerprint = self.fingerprint;
            Some(PreviousWallHoles {
                holes: self.finalized.clone(),
                wall_length: self.wall_length,
                closed: self.closed,
            })
        } else {
            self.previous_fingerprint = HoleFingerprint::default();
            self.finalized.clear();
            None
        };
        self.raw.clear();
        self.fingerprint = HoleFingerprint::default();
        self.phase = HoleStoragePhase::InProgress;
        previous
    }
    pub fn add(&mut self, hole: RawWallHole) -> Result<(), HoleStorageError> {
        if self.phase != HoleStoragePhase::InProgress {
            return Err(HoleStorageError::AlreadyFinalized);
        }
        let hash = raw_hole_hash(hole);
        self.fingerprint.sum = self.fingerprint.sum.wrapping_add(hash);
        self.fingerprint.xor ^= hash;
        self.raw.push(hole);
        Ok(())
    }
    /// Returns true when geometry is rebuilt. Identical sum/xor fingerprints
    /// reuse the original cached geometry AND its old length/closed parameters.
    pub fn finalize(&mut self, wall_length: f32, closed: bool) -> Result<bool, HoleStorageError> {
        if self.phase == HoleStoragePhase::Finalized {
            return Err(HoleStorageError::AlreadyFinalized);
        }
        if self.fingerprint == self.previous_fingerprint {
            self.phase = HoleStoragePhase::Finalized;
            return Ok(false);
        }
        let mut finalized = Vec::with_capacity(self.raw.len());
        for hole in &self.raw {
            let [dx, dy] = hole.dimensions;
            let length = (dy * dy + dx * dx).sqrt();
            if !(length >= 0.0) {
                return Err(HoleStorageError::InvalidDimensions);
            }
            let hx = dx * 0.5;
            let hy = dy * 0.5;
            let aabb = [
                subtract(hole.center[0], hx),
                subtract(hole.center[1], hy),
                add(hole.center[0], hx),
                add(hole.center[1], hy),
            ];
            finalized.push(FinalizedWallHole {
                aabb,
                kind: hole.kind,
                origin: 0,
            });
            if closed {
                // Ordered SSE branches: NaN minimum does not select the left
                // wrap. At most one duplicate is produced, even a huge hole.
                let move_right = aabb[0] < 0.0;
                if move_right || aabb[2] > wall_length {
                    let mut aabb = aabb;
                    if move_right {
                        aabb[0] = add(aabb[0], wall_length);
                        aabb[2] = add(aabb[2], wall_length);
                    } else {
                        aabb[0] = subtract(aabb[0], wall_length);
                        aabb[2] = subtract(aabb[2], wall_length);
                    }
                    finalized.push(FinalizedWallHole {
                        aabb,
                        kind: hole.kind,
                        origin: 1,
                    });
                }
            }
        }
        // Original stable comparison uses max_x, accepts equal signed zeros,
        // and panics on an unordered comparison; it never merges records.
        let mut failed = false;
        finalized.sort_by(|a, b| {
            a.aabb[2].partial_cmp(&b.aabb[2]).unwrap_or_else(|| {
                failed = true;
                std::cmp::Ordering::Equal
            })
        });
        if failed {
            return Err(HoleStorageError::NonOrderableBounds);
        }
        // Fingerprint advancement is an outer per-frame operation; finalize
        // preserves both pairs instead of copying current into previous.
        self.finalized = finalized;
        self.wall_length = wall_length;
        self.closed = closed;
        self.phase = HoleStoragePhase::Finalized;
        Ok(true)
    }
}
