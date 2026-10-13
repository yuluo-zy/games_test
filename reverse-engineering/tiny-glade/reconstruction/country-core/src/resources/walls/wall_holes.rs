//! wall_holes.rs — 墙门窗洞存储族（WP21，复用 building-holes 先验）。
//!
//! O 证据：`WallHoles`/`WallHoleStorage` 布局（0xf86de0/0xf86e50）、`WallHoles::add`
//! 0xf86500、`clear_for_next_frame` 0xf867c0、`finalize` 0xf87300、
//! `InProgressHoleStorage::add` 0xf87860、`PrevWallHoles::wall_holes` 0xf86d00、
//! `iter_holes_with_padding` 0x123fc70。先验：reconstruction/building/wall_holes.rs
//! （building-holes，B 768 例原码验证生产/存储/finalize 整链）。

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use bevy_ecs::resource::Resource;

use crate::resources::walls::WallId;
use crate::utils::geometry::aabb::Aabb2;

/// `HoleType`（O：raw 记录 kind 字节；`u8` 判别）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HoleType(pub u8);

/// `HoleOrigin`（O：finalize 写 0=主 / 1=跨 seam 平移副本）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HoleOrigin(pub u8);

/// `HoleChecksum`（O：sum + xor 指纹）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HoleChecksum {
    pub sum: u64,
    pub xor: u64,
}

/// RawWallHole（O：20 字节；`InProgressHoleStorage::add` 0xf87860 按 0x14 步长 push）：
/// `{ center: Vec2, dimensions: Vec2, kind: u8 }`。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawWallHole {
    pub center: glam::Vec2,
    pub dimensions: glam::Vec2,
    pub kind: HoleType,
}

/// FinalizedWallHole（O：20 字节）：`{ aabb: Aabb2(16B), kind: u8, origin: u8 }`。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FinalizedWallHole {
    pub aabb: Aabb2,
    pub kind: HoleType,
    pub origin: HoleOrigin,
}

/// 原始 `utils::calculate_hash<(array<u32,2>,array<u32,2>,HoleType)>` 指纹（O：
/// `InProgressHoleStorage::add` 累加 `sum += h; xor ^= h`；SipHash 走 std）。
pub fn raw_hole_hash(hole: &RawWallHole) -> u64 {
    let mut h = DefaultHasher::new();
    for f in [hole.center.x, hole.center.y, hole.dimensions.x, hole.dimensions.y] {
        f.to_bits().hash(&mut h);
    }
    hole.kind.0.hash(&mut h);
    h.finish()
}

/// `WallHoleStorage`（O：0x68=104 字节）。阶段判别 +0x0（0=WIP/1=Finalized）。
///
/// 布局 O（0xf86de0/0xf86e50）：`phase @+0x0`、`raw: Vec<RawWallHole>`（ptr@+0x8/
/// cap@+0x10/len@+0x18）、`finalized: Vec<FinalizedWallHole>`（@+0x20/+0x28/+0x30）、
/// fingerprint sum/xor（@+0x38/+0x40）、previous sum/xor（@+0x48/+0x50）、
/// `wall_length: f32 @+0x58`、`closed: bool @+0x5c`。
#[derive(Clone, Debug)]
pub struct WallHoleStorage {
    phase: HoleStoragePhase,
    raw: Vec<RawWallHole>,
    finalized: Vec<FinalizedWallHole>,
    fingerprint: HoleChecksum,
    previous_fingerprint: HoleChecksum,
    wall_length: f32,
    closed: bool,
}

/// `HoleStoragePhase`（O：+0x0 判别）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoleStoragePhase {
    InProgress,
    Finalized,
}

impl Default for WallHoleStorage {
    fn default() -> Self {
        Self::new_in_progress()
    }
}

impl WallHoleStorage {
    /// `new_in_progress`（O 0xf86de0）：phase=0，两个 Vec cap=4，指纹清零。
    pub fn new_in_progress() -> Self {
        Self {
            phase: HoleStoragePhase::InProgress,
            raw: Vec::new(),
            finalized: Vec::new(),
            fingerprint: HoleChecksum::default(),
            previous_fingerprint: HoleChecksum::default(),
            wall_length: 0.0,
            closed: false,
        }
    }
    /// `new_finalized`（O 0xf86e50）：phase=1，finalized Vec cap=4。
    pub fn new_finalized() -> Self {
        Self {
            phase: HoleStoragePhase::Finalized,
            raw: Vec::new(),
            finalized: Vec::new(),
            fingerprint: HoleChecksum::default(),
            previous_fingerprint: HoleChecksum::default(),
            wall_length: 0.0,
            closed: false,
        }
    }
    /// `as_finalized`（O 0xf86eb0）：phase==1 否则 panic。
    pub fn as_finalized(&self) -> &FinalizedWallHoles {
        assert!(self.phase == HoleStoragePhase::Finalized, "as_finalized");
        // 只读视图由调用方经域类型表达；这里返回简视图。
        unimplemented!("domain view; see contract")
    }
    /// `as_wip_mut`（O 0xf86f00）：phase==0 否则 panic。
    pub fn as_wip_mut(&mut self) -> &mut InProgressHoleStorage<'_> {
        assert!(self.phase == HoleStoragePhase::InProgress, "as_wip_mut");
        unimplemented!("domain view; see contract")
    }
    /// `clone_and_reopen`（O 0xf86f50）：深拷贝 raw/finalized，phase 复位 0。
    pub fn clone_and_reopen(&self) -> Self {
        Self {
            phase: HoleStoragePhase::InProgress,
            raw: self.raw.clone(),
            finalized: self.finalized.clone(),
            fingerprint: self.fingerprint,
            previous_fingerprint: self.previous_fingerprint,
            wall_length: self.wall_length,
            closed: self.closed,
        }
    }
    /// `finalize`（O 0xf87300）：raw→Aabb2，sort_by(maxX)，closed 时对跨 seam 洞写
    /// ±wall_length 平移副本、origin=0/1；不推进 previous 指纹。
    pub fn finalize(&mut self, closed: bool, padding: f32) {
        assert!(self.phase == HoleStoragePhase::InProgress, "finalize");
        let raw = std::mem::take(&mut self.raw);
        let mut out: Vec<FinalizedWallHole> = Vec::with_capacity(raw.len());
        for r in &raw {
            let aabb = Aabb2::from_center_dims(r.center, r.dimensions);
            out.push(FinalizedWallHole {
                aabb,
                kind: r.kind,
                origin: HoleOrigin(0),
            });
        }
        // O：按 maxX 稳定排序（sort_by closure，<0x15 insertion / 否则 driftsort）。
        out.sort_by_key(|h| {
            if h.aabb.min.x.is_nan() {
                f32::MAX.to_bits()
            } else {
                h.aabb.min.x.to_bits()
            }
        });
        if closed {
            let mut padded = Vec::with_capacity(out.len() * 2);
            for h in &out {
                padded.push(*h);
                // O：0.0 <= center.x → 右移 +padding；padding < center.y 分支左移。
                let center_x = (h.aabb.min.x + h.aabb.max.x) * 0.5;
                if center_x >= 0.0 {
                    padded.push(FinalizedWallHole {
                        aabb: Aabb2 {
                            min: h.aabb.min + glam::Vec2::splat(padding),
                            max: h.aabb.max + glam::Vec2::splat(padding),
                        },
                        kind: h.kind,
                        origin: HoleOrigin(1),
                    });
                } else if padding < (h.aabb.min.y + h.aabb.max.y) * 0.5 {
                    padded.push(FinalizedWallHole {
                        aabb: Aabb2 {
                            min: h.aabb.min - glam::Vec2::splat(padding),
                            max: h.aabb.max - glam::Vec2::splat(padding),
                        },
                        kind: h.kind,
                        origin: HoleOrigin(1),
                    });
                }
            }
            out = padded;
        }
        self.finalized = out;
        self.closed = closed;
        self.phase = HoleStoragePhase::Finalized;
        // O：finalize 不推进 previous；advance_frame 才推进（building-holes）。
    }
    /// `advance_frame`（O 名，building-holes）：previous = fingerprint。
    pub fn advance_frame(&mut self) {
        self.previous_fingerprint = self.fingerprint;
    }
    pub fn phase(&self) -> HoleStoragePhase {
        self.phase
    }
    pub fn fingerprint(&self) -> HoleChecksum {
        self.fingerprint
    }
    pub fn previous_fingerprint(&self) -> HoleChecksum {
        self.previous_fingerprint
    }
}

/// `InProgressHoleStorage`（O：WIP 视图；`add` 0xf87860、`holes` getter、`extend`
/// 0x123fe30 遇 kind==5 短路）。
pub struct InProgressHoleStorage<'a>(&'a mut WallHoleStorage);

/// `FinalizedWallHoles`（O：finalized 视图 + wall_length + closed）。
#[derive(Clone, Debug, Default)]
pub struct FinalizedWallHoles {
    pub holes: Vec<FinalizedWallHole>,
    pub wall_length: f32,
    pub closed: bool,
}

impl FinalizedWallHoles {
    /// `holes_no_duplicates`（O getter 0xf86cd0）。
    pub fn holes_no_duplicates(&self) -> &[FinalizedWallHole] {
        &self.holes
    }
    /// `wall_length`（O getter 0xf86cf0）。
    pub fn wall_length(&self) -> f32 {
        self.wall_length
    }
    /// `is_empty`（O 名）。
    pub fn is_empty(&self) -> bool {
        self.holes.is_empty()
    }
    /// `clear`（O 名）。
    pub fn clear(&mut self) {
        self.holes.clear();
        self.wall_length = 0.0;
        self.closed = false;
    }
}

/// `PrevWallHoles`（O：`HashMap<WallId, FinalizedWallHoles>`；0xf86d00 查含）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PrevWallHoles {
    pub map: HashMap<WallId, FinalizedWallHoles>,
}

impl PrevWallHoles {
    /// `wall_holes`（O 0xf86d00）：WallId 是否在表中。
    pub fn wall_holes(&self, id: WallId) -> bool {
        self.map.contains_key(&id)
    }
}

/// `WallHoles`（O：`HashMap<WallId, WallHoleStorage>` 资源；add/clear_for_next_frame/
/// extend）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallHoles {
    pub map: HashMap<WallId, WallHoleStorage>,
}

impl WallHoles {
    /// `add`（O 0xf86500）：entry_or_default + push RawWallHole + 指纹累加。
    pub fn add(&mut self, id: WallId, hole: RawWallHole) {
        let entry = self.map.entry(id).or_insert_with(WallHoleStorage::new_in_progress);
        let h = raw_hole_hash(&hole);
        entry.fingerprint.sum = entry.fingerprint.sum.wrapping_add(h);
        entry.fingerprint.xor ^= h;
        entry.raw.push(hole);
    }
    /// `extend`（O 0x12ba9f0）：批量 add，kind==5 短路。
    pub fn extend(&mut self, id: WallId, holes: impl IntoIterator<Item = RawWallHole>) {
        for h in holes {
            if h.kind.0 == 5 {
                break;
            }
            self.add(id, h);
        }
    }
    /// `clear_for_next_frame`（O 0xf867c0）：推进 previous 指纹并把 finalized
    /// 迁入 PrevWallHoles（retain 清理）。
    pub fn clear_for_next_frame(&mut self, prev: &mut PrevWallHoles) {
        for (id, s) in self.map.iter_mut() {
            s.advance_frame();
            if s.phase == HoleStoragePhase::Finalized {
                let f = FinalizedWallHoles {
                    holes: std::mem::take(&mut s.finalized),
                    wall_length: s.wall_length,
                    closed: s.closed,
                };
                prev.map.insert(*id, f);
            }
        }
    }
}
