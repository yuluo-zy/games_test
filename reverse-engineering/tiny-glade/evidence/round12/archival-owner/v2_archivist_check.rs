//! B address-scoped decorator archival and restoration owners.
//! The native resource adapter retains both actual collision channels and mesh
//! Arcs. extract_diff_against_storage follows the original live after-state
//! and semantic equality algorithm; the older no-storage API remains a legacy
//! approximation for callers not yet migrated and is not an equivalence gate.
pub use crate::resources::walls::decorator_archivist::changes::DecoratorArchivist;
pub use crate::resources::walls::decorator_archivist::changes::DecoratorStorage;
pub use crate::resources::walls::decorator_archivist::changes::ObservedDecoratorInfo;
pub use crate::resources::walls::decorator_archivist::changes::TrackedDecorators;
pub use crate::resources::walls::decorator_archivist::changes::restore_archived_decorators;
use std::collections::{HashMap, HashSet};

use crate::resources::history::decorator_edit::{
    DecoratorHistoryState, DecoratorStorageAddrHistoryState, DecoratorStorageHistoryDiff,
};
use crate::resources::walls::WallId;
use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use crate::resources::walls::decorator_storage::{
    DecoratorStorage as LiveDecoratorStorage, DecoratorStorageAddr,
};
use crate::resources::walls::publication_fields::WallId as RecoveredWallId;

/// Original archive collision payload. These are owned collision objects, not
/// addresses copied from a native allocation.
#[cfg(feature = "native-physics")]
pub type LiveDecoratorArchivist = DecoratorArchivist<(
    crate::systems::collision::world::OffsetColliderTransform,
    crate::systems::collision::world::RaycastMesh,
)>;

/// The CPU-only build shares the same archive and culling owner. Its collision
/// payload is unavailable until native-physics is enabled; UI culling changes
/// still use this owner rather than a second flag in the UI resource.
#[cfg(not(feature = "native-physics"))]
pub type LiveDecoratorArchivist = DecoratorArchivist<()>;

impl<C: Send + Sync + 'static> Default for DecoratorArchivist<C> {
    /// B default @140341370, installed by DecoratorPlugin @140343132/13d:
    /// empty tracking/collision maps and allow_any=false @1403414f8.
    fn default() -> Self {
        Self {
            walls: HashMap::new(),
            special1: None,
            special2: None,
            collision_a: HashMap::new(),
            collision_b: HashMap::new(),
            culling: DecoratorCullPolicy {
                allowed_ids: HashSet::new(),
                allow_any: false,
            },
        }
    }
}

/// B +0x30 is the permitted decorator-ID set; +0x1e0 permits all IDs.
/// Explicit state is required until original startup construction is connected.
#[derive(Clone, Debug)]
pub struct DecoratorCullPolicy {
    pub allowed_ids: HashSet<u32>,
    pub allow_any: bool,
}

impl<C: Send + Sync + 'static> DecoratorArchivist<C> {
    /// B140b2cb30 uses each archived address as its before state and reads its
    /// after state from live storage. The bool reaches extract_before_after_impl
    /// as allow_preview (fold140b2ab29/35); strict missing-before is false.
    /// This concrete API avoids the legacy no-storage approximation below.
    pub fn extract_diff_against_storage(
        &mut self,
        storage: &LiveDecoratorStorage,
        allow_preview: bool,
    ) -> HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff> {
        self.collision_a.clear();
        self.collision_b.clear();
        let walls = std::mem::take(&mut self.walls);
        let terrain = self.special1.take();
        let stairs = self.special2.take();
        let addresses = walls
            .into_iter()
            .map(|(wall, bucket)| (DecoratorStorageAddr::Wall(WallId(wall.0)), bucket))
            .chain(
                terrain
                    .into_iter()
                    .map(|bucket| (DecoratorStorageAddr::Terrain, bucket)),
            )
            .chain(
                stairs
                    .into_iter()
                    .map(|bucket| (DecoratorStorageAddr::Stair, bucket)),
            );
        let mut out = HashMap::new();
        for (address, bucket) in addresses {
            let mut before: HashMap<_, _> = bucket
                .items
                .iter()
                .map(|(id, info)| {
                    (
                        DecoratorId(*id),
                        DecoratorHistoryState::from(&observed_to_info(info)),
                    )
                })
                .collect();
            let mut after = HashMap::new();
            if let Some(live) = storage.get(address) {
                for (id, info) in &live.decorators {
                    if !allow_preview && info.state != 0 {
                        panic!(
                            "Found a preview decorator while trying to extract history diff. {:?}: {:?}",
                            DecoratorId(*id),
                            info
                        );
                    }
                    after.insert(DecoratorId(*id), DecoratorHistoryState::from(info));
                }
            }
            // Original1415237c0 removes matching IDs from both snapshots using
            // the game's semantic state comparison (float ordered equality).
            let equal: Vec<_> = before
                .iter()
                .filter_map(|(id, b)| {
                    after
                        .get(id)
                        .filter(|a| archived_history_equal(b, a))
                        .map(|_| *id)
                })
                .collect();
            for id in equal {
                before.remove(&id);
                after.remove(&id);
            }
            if !before.is_empty() || !after.is_empty() {
                out.insert(
                    address,
                    DecoratorStorageHistoryDiff {
                        // The original AddrHistoryState is one48B map. Legacy
                        // projection removed-sets have no native producer here.
                        before: DecoratorStorageAddrHistoryState {
                            states: before,
                            removed: HashSet::new(),
                        },
                        after: DecoratorStorageAddrHistoryState {
                            states: after,
                            removed: HashSet::new(),
                        },
                    },
                );
            }
        }
        assert!(self.walls.is_empty());
        assert!(self.special1.is_none());
        assert!(self.special2.is_none());
        self.culling.allowed_ids.clear();
        self.culling.allow_any = false;
        out
    }

    /// B 0x140b2bbf0 changes this flag only, preserving the ID set.
    pub fn decorator_cull_allow_any(&mut self) {
        self.culling.allow_any = true;
    }

    /// B 0x140b2bd90: flag short-circuit, otherwise ID membership.
    pub fn is_decorator_cull_allowed(&self, id: u32) -> bool {
        self.culling.allow_any || self.culling.allowed_ids.contains(&id)
    }

    /// 0x140b2bb00（fresh 反编译 2026-10-17）：walls map 迭代 + special1/special2
    /// 两条 Option 链的三元 `Chained` 迭代器（返回各桶内嵌 map 的 (id, info)）。
    pub fn all_decorators(&self) -> impl Iterator<Item = (&u32, &ObservedDecoratorInfo)> + '_ {
        self.walls
            .values()
            .flat_map(|decorators| decorators.items.iter())
            .chain(
                self.special1
                    .iter()
                    .flat_map(|decorators| decorators.items.iter()),
            )
            .chain(
                self.special2
                    .iter()
                    .flat_map(|decorators| decorators.items.iter()),
            )
    }

    /// 0x140b2bb80（fresh 反编译 2026-10-17）：walls + 可选 special1/2 的存储地址
    /// 迭代器。原生以 niche 编码 `(p==0)*2+1` / `(q==0)+2` 携带地址判别；Rust 以
    /// `DecoratorStorageAddr` 显式判别表达同一三元链。
    pub fn all_storages(
        &self,
    ) -> impl Iterator<Item = (DecoratorStorageAddr, &TrackedDecorators)> + '_ {
        self.walls
            .iter()
            .map(|(wall, decorators)| (DecoratorStorageAddr::Wall(WallId(wall.0)), decorators))
            .chain(
                self.special1
                    .iter()
                    .map(|decorators| (DecoratorStorageAddr::Terrain, decorators)),
            )
            .chain(
                self.special2
                    .iter()
                    .map(|decorators| (DecoratorStorageAddr::Stair, decorators)),
            )
    }

    /// Original140b2bc00 mutates the permitted-ID set using this live wall's
    /// keys. The fold's accumulator is (), not the result of an `any` query.
    pub fn decorator_cull_allow_wall(&mut self, wall: WallId, storage: &LiveDecoratorStorage) {
        if let Some(decorators) = storage.get(DecoratorStorageAddr::Wall(wall)) {
            // Original140b2bc00 calls the unit-valued HashSet::extend fold
            //140b76260 over the live bucket's keys, targeting self+0x30.
            self.culling
                .allowed_ids
                .extend(decorators.decorators.keys().copied());
        }
    }

    /// Original140b2bd30 extends the same set with live stair-bucket keys.
    pub fn decorator_cull_allow_stairs(&mut self, storage: &LiveDecoratorStorage) {
        if let Some(decorators) = storage.get(DecoratorStorageAddr::Stair) {
            self.culling
                .allowed_ids
                .extend(decorators.decorators.keys().copied());
        }
    }
}

/// `ObservedDecoratorInfo`（recovered 归档侧原始 0x48B 字节）→ 权威 `DecoratorInfo`
/// （repr(C) 0x48B）。布局断言：`size_of::<DecoratorInfo>() == 72`。
fn observed_to_info(o: &ObservedDecoratorInfo) -> DecoratorInfo {
    assert_eq!(size_of::<DecoratorInfo>(), o.0.len());
    unsafe { std::ptr::read_unaligned(o.0.as_ptr().cast::<DecoratorInfo>()) }
}

/// 权威 `DecoratorInfo` → `ObservedDecoratorInfo`（归档字节）。
fn info_to_observed(info: &DecoratorInfo) -> ObservedDecoratorInfo {
    let mut bytes = [0u8; 72];
    assert_eq!(size_of::<DecoratorInfo>(), bytes.len());
    unsafe {
        std::ptr::copy_nonoverlapping(
            info as *const DecoratorInfo as *const u8,
            bytes.as_mut_ptr(),
            bytes.len(),
        );
    }
    ObservedDecoratorInfo(bytes)
}

/// 权威 `Decorators`（decorator_storage 桶）→ recovered 归档 `TrackedDecorators`。
fn decos_to_tracked(
    decos: &crate::resources::walls::decorator_storage::Decorators,
) -> TrackedDecorators {
    TrackedDecorators {
        removed: decos.removed_ids.clone(),
        items: decos
            .decorators
            .iter()
            .map(|(id, info)| (*id, info_to_observed(info)))
            .collect(),
        added: decos.modified_ids.clone(),
    }
}

/// recovered 归档 `TrackedDecorators` → 权威 `Decorators`
/// （`extract_and_restore_to_storage` 0x140b2c4b0 的 `tracked_clone_from` 源）。
fn tracked_to_decos(
    tracked: &TrackedDecorators,
) -> crate::resources::walls::decorator_storage::Decorators {
    crate::resources::walls::decorator_storage::Decorators {
        removed_ids: tracked.removed.clone(),
        decorators: tracked
            .items
            .iter()
            .map(|(id, info)| (*id, observed_to_info(info)))
            .collect(),
        modified_ids: tracked.added.clone(),
    }
}

/// 单 `TrackedDecorators` 桶 → `DecoratorStorageAddrHistoryState`
/// （`extract_before_after_impl` 0x140b2be70 的 Rust 对应：桶 items 逐条目
/// `DecoratorInfo → DecoratorHistoryState`；removed 面为恢复语义所需）。
///
/// fresh 0x140b2be70 另含 **preview 断言**：桶内任一条目
/// `DecoratorInfo.state`(+0x47，disasm `CMP byte [entry+0x4f],0`) 非 0 →
/// `panic_fmt("Found a preview decorator while trying to extract history diff. ")`
/// （消息 + DecoratorId/DecoratorInfo 两参数；0x142aeb4f8）。
fn tracked_to_addr_state(tracked: &TrackedDecorators) -> DecoratorStorageAddrHistoryState {
    let mut states = HashMap::new();
    for (id, info) in &tracked.items {
        // entry = {DecoratorId@0, DecoratorInfo@8..0x50}；entry+0x4f = info+0x47。
        if info.0[0x47] != 0 {
            panic!(
                "Found a preview decorator while trying to extract history diff. {:?}: {:?}",
                DecoratorId(*id),
                observed_to_info(info)
            );
        }
        states.insert(
            DecoratorId(*id),
            DecoratorHistoryState::from(&observed_to_info(info)),
        );
    }
    DecoratorStorageAddrHistoryState {
        states,
        removed: tracked.removed.iter().copied().map(DecoratorId).collect(),
    }
}

/// `extract_before_after_all` 闭包（内联，fresh 不收敛）的 Rust 重建：
/// 对单地址当前桶，before/after 桶（special1/special2）逐 id 覆盖当前状态；
/// 未在 before/after 的 id 保持当前状态。**登记 U**：原生闭包体为内联未单解，
/// 覆盖语义按恢复推断，非判等。
fn extract_before_after_pair(
    before: &Option<TrackedDecorators>,
    after: &Option<TrackedDecorators>,
    cur: &TrackedDecorators,
) -> (
    DecoratorStorageAddrHistoryState,
    DecoratorStorageAddrHistoryState,
) {
    let mut before_states = HashMap::new();
    let mut after_states = HashMap::new();
    for (id, info) in &cur.items {
        let cur_info = observed_to_info(info);
        let b = before
            .as_ref()
            .and_then(|b| b.items.get(id))
            .map(observed_to_info)
            .unwrap_or(cur_info);
        let a = after
            .as_ref()
            .and_then(|a| a.items.get(id))
            .map(observed_to_info)
            .unwrap_or(cur_info);
        before_states.insert(DecoratorId(*id), DecoratorHistoryState::from(&b));
        after_states.insert(DecoratorId(*id), DecoratorHistoryState::from(&a));
    }
    let mut before_removed: HashSet<DecoratorId> =
        cur.removed.iter().copied().map(DecoratorId).collect();
    let mut after_removed: HashSet<DecoratorId> =
        cur.removed.iter().copied().map(DecoratorId).collect();
    if let Some(b) = before.as_ref() {
        before_removed.extend(b.removed.iter().copied().map(DecoratorId));
    }
    if let Some(a) = after.as_ref() {
        after_removed.extend(a.removed.iter().copied().map(DecoratorId));
    }
    (
        DecoratorStorageAddrHistoryState {
            states: before_states,
            removed: before_removed,
        },
        DecoratorStorageAddrHistoryState {
            states: after_states,
            removed: after_removed,
        },
    )
}

/// Original141523a60..141523eef. Destination variants compare only their
/// active payload; all subsequent common fields are still compared. Inactive
/// bytes and padding must not create edits. NaN differs, signed zero is equal.
fn archived_history_equal(before: &DecoratorHistoryState, after: &DecoratorHistoryState) -> bool {
    let b = before.to_bytes();
    let a = after.to_bytes();
    let read32 = |s: &[u8; 0x50], p| u32::from_le_bytes(s[p..p + 4].try_into().unwrap());
    let feq = |p| f32::from_bits(read32(&b, p)) == f32::from_bits(read32(&a, p));
    let sel_b = if b[0x22].wrapping_sub(2) < 3 {
        b[0x22] - 1
    } else {
        0
    };
    let sel_a = if a[0x22] >= 2 { a[0x22] - 1 } else { 0 };
    if sel_b != sel_a {
        return false;
    }
    let destination_equal = match sel_b {
        1 | 2 => [0, 4, 8].into_iter().all(feq),
        3 => b[..8] == a[..8] && [8, 12, 16].into_iter().all(feq),
        _ => {
            [0, 4, 16, 20, 24, 28].into_iter().all(feq)
                && b[8..16] == a[8..16]
                && b[0x20..0x23] == a[0x20..0x23]
        }
    };
    if !destination_equal {
        return false;
    }
    // Native normalizes the decorator type to0..7 / fallback6, and when
    // fallback6 it additionally compares the full byte. This is exact identity.
    b[0x4a] == a[0x4a]
        && [0x48, 0x49, 0x4b, 0x4c, 0x4d, 0x4e]
            .into_iter()
            .all(|p| b[p] == a[p])
        && [0x28, 0x2c, 0x34, 0x40, 0x44]
            .into_iter()
            .all(|p| read32(&b, p) == read32(&a, p))
        && [0x30, 0x38, 0x3c].into_iter().all(feq)
}

impl<C: Send + Sync + 'static> DecoratorArchivist<C> {
    /// B `extract_diff` 0x140b2cb30：消费归档内容，产出
    /// `HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>`。
    ///
    /// fresh 结构：清 collision_a/b → memmove before@+0xc0 / after@+0x150（= recovered
    /// special1/special2）→ 三路 chain（walls Drain + special1/2 IntoIter）→
    /// `from_iter<Addr,(AddrState,AddrState)>` → filter_map → 输出 diff map；
    /// 后置断言 walls/before/after len==0 panic。collision 重建为 collision 域（登记）。
    ///
    /// before/after 配对按 [`extract_before_after_pair`]（登记 U，非判等）。
    pub fn extract_diff(&mut self) -> HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff> {
        self.collision_a.clear();
        self.collision_b.clear();
        let walls = std::mem::take(&mut self.walls);
        let before = self.special1.take();
        let after = self.special2.take();
        let mut out = HashMap::new();
        for (wall, tracked) in walls {
            let addr = DecoratorStorageAddr::Wall(WallId(wall.0));
            let (b, a) = extract_before_after_pair(&before, &after, &tracked);
            out.insert(
                addr,
                DecoratorStorageHistoryDiff {
                    before: b,
                    after: a,
                },
            );
        }
        // fresh：三路 chain 中 special1/special2 条目也经 extract_before_after_all 产出
        // diff（Terrain / Stair 地址）。此前 Rust 只遍历 walls。
        if let Some(tracked) = &before {
            let (b, a) = extract_before_after_pair(&before, &after, tracked);
            out.insert(
                DecoratorStorageAddr::Terrain,
                DecoratorStorageHistoryDiff {
                    before: b,
                    after: a,
                },
            );
        }
        if let Some(tracked) = &after {
            let (b, a) = extract_before_after_pair(&before, &after, tracked);
            out.insert(
                DecoratorStorageAddr::Stair,
                DecoratorStorageHistoryDiff {
                    before: b,
                    after: a,
                },
            );
        }
        // 后置断言（fresh：in_RDX[3]=walls len、in_RDX[0x18]=before、in_RDX[0x2a]=after）。
        assert!(self.walls.is_empty(), "assertion failed: walls.is_empty()");
        assert!(
            self.special1.is_none(),
            "assertion failed: special1.is_none()"
        );
        assert!(
            self.special2.is_none(),
            "assertion failed: special2.is_none()"
        );
        out
    }

    /// B `extract_and_restore_to_storage` 0x140b2c4b0（fresh 反编译 2026-10-17）：
    /// 1. 清 collision_a/collision_b 两个 map；
    /// 2. memcpy + 置空 special1（+0xc0）与 special2（+0x150）；
    /// 3. 三路 chain（walls Drain + special1 IntoIter + special2 IntoIter），
    ///    逐 `(地址, 桶)`：`DecoratorStorage::get_or_insert_default` +
    ///    `Decorators::tracked_clone_from<DecoMoveToStorage>`。
    /// 归档侧三表被 drain/take 清空。
    pub fn extract_and_restore_to_storage(&mut self, storage: &mut LiveDecoratorStorage) {
        self.collision_a.clear();
        self.collision_b.clear();
        let walls = std::mem::take(&mut self.walls);
        let special1 = self.special1.take();
        let special2 = self.special2.take();
        for (wall, tracked) in walls {
            let addr = DecoratorStorageAddr::Wall(WallId(wall.0));
            storage.tracked_clone_into(addr, &tracked_to_decos(&tracked));
        }
        if let Some(tracked) = special1 {
            storage.tracked_clone_into(DecoratorStorageAddr::Terrain, &tracked_to_decos(&tracked));
        }
        if let Some(tracked) = special2 {
            storage.tracked_clone_into(DecoratorStorageAddr::Stair, &tracked_to_decos(&tracked));
        }
    }

    /// B `extract_before_after_impl` 0x140b2be70：单地址状态提取
    /// （桶 → `DecoratorStorageAddrHistoryState`）。
    pub fn extract_before_after(
        &self,
        addr: DecoratorStorageAddr,
    ) -> Option<DecoratorStorageAddrHistoryState> {
        match addr {
            DecoratorStorageAddr::Wall(wall) => self
                .walls
                .get(&RecoveredWallId(wall.0))
                .map(tracked_to_addr_state),
            DecoratorStorageAddr::Terrain => self.special1.as_ref().map(tracked_to_addr_state),
            DecoratorStorageAddr::Stair => self.special2.as_ref().map(tracked_to_addr_state),
        }
    }

    /// B `archive_storage_decorators` 0x1412b2a70/0x1412b3910（in-domain 核）：
    /// 按存储判别 0=墙 / 1=special1（before @+0xc0）/ 2=special2（after @+0x150）。
    ///
    /// - 墙：源桶已存在（walls 探针）则 return；否则 clone 入 archivist.walls。
    /// - special1/2：对应区已存在（首 qword 非 0）则 return；否则克隆入。
    ///
    /// 原生还做 collision collider/mesh 重建（get_collider/get_collider_mesh →
    /// `HashMap insert (DecoratorId,(OffsetColliderTransform,RaycastMesh))`）——
    /// 属 collision 域（RaycastMesh/OffsetColliderTransform），登记不实现。
    pub fn archive_storage_decorators(
        &mut self,
        selector: u8,
        source: &crate::resources::walls::decorator_storage::Decorators,
        wall: Option<WallId>,
    ) {
        match selector {
            0 => {
                let wall =
                    wall.expect("archive_storage_decorators: wall selector requires wall id");
                let rw = RecoveredWallId(wall.0);
                if self.walls.contains_key(&rw) {
                    return;
                }
                self.walls.insert(rw, decos_to_tracked(source));
            }
            1 => {
                if self.special1.is_some() {
                    return;
                }
                self.special1 = Some(decos_to_tracked(source));
            }
            2 => {
                if self.special2.is_some() {
                    return;
                }
                self.special2 = Some(decos_to_tracked(source));
            }
            _ => unreachable!("archive_storage_decorators: invalid selector"),
        }
    }
}

/// Original SystemParam carries these two canonical shared owners. It never
/// clones RaycastWorld or builds a second collision world for editing.
#[cfg(feature = "native-physics")]
#[derive(bevy_ecs::system::SystemParam)]
pub struct DecoratorArchivalResources<'w> {
    pub world: bevy_ecs::prelude::Res<'w, crate::systems::collision::world::RaycastWorld>,
    pub colliders: bevy_ecs::prelude::Res<
        'w,
        crate::resources::walls::decorator_collision::DecoratorPhysicsColliders,
    >,
}

#[cfg(feature = "native-physics")]
impl LiveDecoratorArchivist {
    /// B1421a0470: archive exactly the requested address, including both
    /// collider channels and their owned meshes. An absent live bucket is
    /// archived as an empty before-state; it is not an ignored request.
    pub fn archive_current_decorators_at_address(
        &mut self,
        address: DecoratorStorageAddr,
        storage: &LiveDecoratorStorage,
        resources: &DecoratorArchivalResources<'_>,
        reason: &str,
    ) {
        self.archive_current_decorators_with_collision(
            address,
            storage,
            &resources.world,
            &resources.colliders,
            reason,
        );
    }

    fn archive_current_decorators_with_collision(
        &mut self,
        address: DecoratorStorageAddr,
        storage: &LiveDecoratorStorage,
        world: &crate::systems::collision::world::RaycastWorld,
        colliders: &crate::resources::walls::decorator_collision::DecoratorPhysicsColliders,
        _reason: &str,
    ) {
        let already_archived = match address {
            DecoratorStorageAddr::Wall(wall) => self.walls.contains_key(&RecoveredWallId(wall.0)),
            DecoratorStorageAddr::Terrain => self.special1.is_some(),
            DecoratorStorageAddr::Stair => self.special2.is_some(),
        };
        if already_archived {
            return;
        }
        // The full original's0622..0c4c cold path supplies empty Decorators
        // when get(address) is None. The reason participates only in the
        // temporary diagnostic string; it is not a persisted archive field.
        let source = storage.get(address).cloned().unwrap_or_default();
        let (selector, wall) = match address {
            DecoratorStorageAddr::Wall(wall) => (0, Some(wall)),
            DecoratorStorageAddr::Terrain => (1, None),
            DecoratorStorageAddr::Stair => (2, None),
        };
        self.archive_storage_decorators(selector, &source, wall);
        // The two original loops visit all current keys, first channel then
        // second. Missing collision entries skip; stale actual handles panic.
        for (live, archived) in [
            (&colliders.first, &mut self.collision_a),
            (&colliders.second, &mut self.collision_b),
        ] {
            for &id in source.decorators.keys() {
                let Some(instance) = live.get(&DecoratorId(id)) else {
                    continue;
                };
                if instance.slot == crate::systems::collision::EMPTY_COLLIDER_HANDLE_TAG {
                    continue;
                }
                let physical = instance.handle();
                let collider = world.get_collider(physical);
                let mesh = world
                    .get_collider_mesh(collider.generation_low as u32, collider.generation)
                    .clone();
                let transform = world.into_transformed_shape(physical);
                archived.insert(id, (transform, mesh));
            }
        }
    }

    /// Original plural entry receives addresses into one live storage owner.
    pub fn archive_current_decorators_in_storages(
        &mut self,
        addresses: impl IntoIterator<Item = DecoratorStorageAddr>,
        storage: &LiveDecoratorStorage,
        resources: &DecoratorArchivalResources<'_>,
        reason: &str,
    ) {
        for address in addresses {
            self.archive_current_decorators_at_address(address, storage, resources, reason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::decorator_storage::Decorators as LiveDecorators;

    fn live_bucket(id: u32, style: u8) -> LiveDecorators {
        let mut b = LiveDecorators::default();
        b.insert(
            DecoratorId(id),
            DecoratorInfo {
                dst_wall: id as u64,
                decorator_id: id,
                u40: ((style as u32) << 8) | 0xaa,
                ..DecoratorInfo::default()
            },
        );
        b.removed_ids.insert(999);
        b
    }

    #[test]
    fn extract_diff_drains_and_produces_per_wall_diff() {
        let mut archivist = DecoratorArchivist::<()> {
            walls: HashMap::new(),
            special1: None,
            special2: None,
            collision_a: HashMap::new(),
            collision_b: HashMap::new(),
            culling: DecoratorCullPolicy {
                allowed_ids: HashSet::new(),
                allow_any: false,
            },
        };
        archivist
            .walls
            .insert(RecoveredWallId(7), decos_to_tracked(&live_bucket(1, 7)));
        let out = archivist.extract_diff();
        let diff = out.get(&DecoratorStorageAddr::Wall(WallId(7))).unwrap();
        assert_eq!(diff.before.states.len(), 1);
        assert_eq!(diff.before.removed.len(), 1);
        assert!(diff.before.removed.contains(&DecoratorId(999)));
        // 归档已清空。
        assert!(archivist.walls.is_empty());
        assert!(archivist.special1.is_none());
        assert!(archivist.special2.is_none());
    }

    #[test]
    fn archive_tracking_preserves_the_first_wall_and_specials_snapshots() {
        let mut storage = LiveDecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(WallId(3)))
            .insert(
                DecoratorId(10),
                DecoratorInfo {
                    dst_wall: 3,
                    decorator_id: 10,
                    ..DecoratorInfo::default()
                },
            );
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(
                DecoratorId(11),
                DecoratorInfo {
                    dst_wall: 0,
                    decorator_id: 11,
                    ..DecoratorInfo::default()
                },
            );
        let mut archivist = DecoratorArchivist::<()> {
            walls: HashMap::new(),
            special1: None,
            special2: None,
            collision_a: HashMap::new(),
            collision_b: HashMap::new(),
            culling: DecoratorCullPolicy {
                allowed_ids: HashSet::new(),
                allow_any: false,
            },
        };
        archivist.archive_storage_decorators(
            0,
            storage.get(DecoratorStorageAddr::Wall(WallId(3))).unwrap(),
            Some(WallId(3)),
        );
        archivist.archive_storage_decorators(
            1,
            storage.get(DecoratorStorageAddr::Terrain).unwrap(),
            None,
        );
        assert!(archivist.walls.contains_key(&RecoveredWallId(3)));
        assert!(archivist.special1.is_some());
        // 再归档：已存在 → return。
        archivist.archive_storage_decorators(
            1,
            storage.get(DecoratorStorageAddr::Terrain).unwrap(),
            None,
        );
        assert!(archivist.special1.is_some());
    }

    fn empty_archivist() -> DecoratorArchivist<()> {
        DecoratorArchivist::<()> {
            walls: HashMap::new(),
            special1: None,
            special2: None,
            collision_a: HashMap::new(),
            collision_b: HashMap::new(),
            culling: DecoratorCullPolicy {
                allowed_ids: HashSet::new(),
                allow_any: false,
            },
        }
    }

    fn tracked_bucket(id: u32) -> TrackedDecorators {
        decos_to_tracked(&live_bucket(id, 0))
    }

    #[test]
    fn all_iterators_chain_walls_and_specials() {
        let mut archivist = empty_archivist();
        archivist
            .walls
            .insert(RecoveredWallId(1), tracked_bucket(10));
        archivist.special1 = Some(tracked_bucket(11));
        archivist.special2 = Some(tracked_bucket(12));
        let mut ids: Vec<u32> = archivist.all_decorators().map(|(id, _)| *id).collect();
        ids.sort_unstable();
        assert_eq!(ids, vec![10, 11, 12]);
        assert_eq!(archivist.all_storages().count(), 3);
    }

    #[test]
    fn decorator_cull_allow_wall_and_stairs_fold_buckets() {
        let mut archivist = empty_archivist();
        let mut storage = LiveDecoratorStorage::default();
        storage.per_wall.insert(WallId(1), live_bucket(10, 0));
        storage.stairs = Some(live_bucket(20, 0));
        archivist
            .walls
            .insert(RecoveredWallId(1), tracked_bucket(10));
        archivist.special2 = Some(tracked_bucket(20));
        archivist.decorator_cull_allow_wall(WallId(1), &storage);
        assert!(archivist.is_decorator_cull_allowed(10));
        assert!(!archivist.is_decorator_cull_allowed(20));
        archivist.decorator_cull_allow_stairs(&storage);
        assert!(archivist.is_decorator_cull_allowed(20));
        archivist.decorator_cull_allow_wall(WallId(2), &storage);
        assert_eq!(archivist.culling.allowed_ids.len(), 2);
    }

    #[cfg(feature = "native-physics")]
    fn add_live_collider(
        world: &mut crate::systems::collision::world::RaycastWorld,
        map: &mut HashMap<DecoratorId, crate::systems::collision::world::ColliderMeshInstance>,
        id: u32,
        source: crate::systems::collision::world::RaycastGeoSource,
        position: glam::Vec3,
    ) -> std::sync::Arc<crate::systems::collision::world::RaycastMeshData> {
        use crate::systems::collision::world::{RaycastMeshData, upsert_collider};
        let mesh = std::sync::Arc::new(RaycastMeshData::new_cuboid(
            glam::Vec3::splat(0.5),
            glam::UVec3::splat(id),
        ));
        upsert_collider(
            world,
            map,
            DecoratorId(id),
            source,
            mesh.clone(),
            position,
            glam::Quat::IDENTITY,
        );
        mesh
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn address_archive_keeps_both_real_collision_channels_and_meshes_after_live_deletion() {
        use crate::resources::walls::decorator_collision::DecoratorPhysicsColliders;
        use crate::systems::collision::world::{RaycastGeoSource, RaycastWorld, remove_collider};
        let mut world = RaycastWorld::new();
        let mut physics = DecoratorPhysicsColliders::default();
        let first = add_live_collider(
            &mut world,
            &mut physics.first,
            10,
            RaycastGeoSource::Slot0,
            glam::Vec3::new(1.0, 2.0, 3.0),
        );
        let second = add_live_collider(
            &mut world,
            &mut physics.second,
            11,
            RaycastGeoSource::Slot1,
            glam::Vec3::new(-2.0, 4.0, 1.0),
        );
        let unrelated = add_live_collider(
            &mut world,
            &mut physics.first,
            20,
            RaycastGeoSource::Slot0,
            glam::Vec3::ZERO,
        );
        let mut storage = LiveDecoratorStorage::default();
        let mut selected = live_bucket(10, 0);
        selected.insert(
            DecoratorId(11),
            DecoratorInfo {
                decorator_id: 11,
                ..Default::default()
            },
        );
        selected.insert(
            DecoratorId(12),
            DecoratorInfo {
                decorator_id: 12,
                ..Default::default()
            },
        );
        storage.per_wall.insert(WallId(3), selected);
        storage.per_wall.insert(WallId(4), live_bucket(20, 0));
        storage.terrain = Some(live_bucket(30, 0));
        let mut archive = LiveDecoratorArchivist::default();
        archive.archive_current_decorators_with_collision(
            DecoratorStorageAddr::Wall(WallId(3)),
            &storage,
            &world,
            &physics,
            "roof_edit",
        );
        assert_eq!(archive.walls.len(), 1);
        assert!(archive.special1.is_none() && archive.special2.is_none());
        assert_eq!(archive.walls[&RecoveredWallId(3)].items.len(), 3);
        assert_eq!(
            (archive.collision_a.len(), archive.collision_b.len()),
            (1, 1)
        );
        assert!(std::sync::Arc::ptr_eq(
            &archive.collision_a[&10].1.data,
            &first
        ));
        assert!(std::sync::Arc::ptr_eq(
            &archive.collision_b[&11].1.data,
            &second
        ));
        assert_eq!(
            archive.collision_a[&10].0.position,
            glam::Vec3::new(1.0, 2.0, 3.0)
        );
        assert_eq!(
            archive.collision_b[&11].0.position,
            glam::Vec3::new(-2.0, 4.0, 1.0)
        );
        // First archival owns the before-state. Later live edits do not replace it.
        storage
            .per_wall
            .get_mut(&WallId(3))
            .unwrap()
            .decorators
            .remove(&10);
        archive.archive_current_decorators_with_collision(
            DecoratorStorageAddr::Wall(WallId(3)),
            &storage,
            &world,
            &physics,
            "roof_edit_again",
        );
        assert!(archive.walls[&RecoveredWallId(3)].items.contains_key(&10));
        let weak = std::sync::Arc::downgrade(&first);
        remove_collider(&mut world, &mut physics.first, &DecoratorId(10)).unwrap();
        drop(first);
        assert!(
            weak.upgrade().is_some(),
            "archival collision owns the deleted live mesh"
        );
        assert!(!archive.collision_a.contains_key(&20));
        archive.archive_current_decorators_with_collision(
            DecoratorStorageAddr::Wall(WallId(99)),
            &storage,
            &world,
            &physics,
            "absent before-state",
        );
        assert!(archive.walls[&RecoveredWallId(99)].items.is_empty());
        drop(archive);
        assert!(weak.upgrade().is_none());
        drop((second, unrelated));
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn invalid_actual_collider_generation_panics_after_the_before_state_is_archived() {
        use crate::resources::walls::decorator_collision::DecoratorPhysicsColliders;
        use crate::systems::collision::world::{RaycastGeoSource, RaycastWorld};
        let mut world = RaycastWorld::new();
        let mut physics = DecoratorPhysicsColliders::default();
        let _mesh = add_live_collider(
            &mut world,
            &mut physics.first,
            10,
            RaycastGeoSource::Slot0,
            glam::Vec3::ZERO,
        );
        let instance = physics.first.get_mut(&DecoratorId(10)).unwrap();
        let (index, generation) = instance.collider.into_raw_parts();
        instance.collider =
            rapier3d::geometry::ColliderHandle::from_raw_parts(index, generation.wrapping_add(1));
        let mut storage = LiveDecoratorStorage::default();
        storage.per_wall.insert(WallId(3), live_bucket(10, 0));
        let mut archive = LiveDecoratorArchivist::default();
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            archive.archive_current_decorators_with_collision(
                DecoratorStorageAddr::Wall(WallId(3)),
                &storage,
                &world,
                &physics,
                "roof_edit",
            );
        }));
        assert!(failed.is_err());
        assert!(archive.walls[&RecoveredWallId(3)].items.contains_key(&10));
        assert!(archive.collision_a.is_empty());
    }

    #[test]
    fn extract_and_restore_to_storage_moves_buckets_and_clears_archive() {
        let mut archivist = empty_archivist();
        archivist
            .walls
            .insert(RecoveredWallId(3), tracked_bucket(10));
        archivist.special1 = Some(tracked_bucket(11));
        archivist.special2 = Some(tracked_bucket(12));
        let mut storage = LiveDecoratorStorage::default();
        archivist.extract_and_restore_to_storage(&mut storage);
        assert!(
            storage
                .get(DecoratorStorageAddr::Wall(WallId(3)))
                .unwrap()
                .decorators
                .contains_key(&10)
        );
        assert!(
            storage
                .get(DecoratorStorageAddr::Terrain)
                .unwrap()
                .decorators
                .contains_key(&11)
        );
        assert!(
            storage
                .get(DecoratorStorageAddr::Stair)
                .unwrap()
                .decorators
                .contains_key(&12)
        );
        assert_eq!(
            storage.decorator_to_storage.get(DecoratorId(10)),
            Some(DecoratorStorageAddr::Wall(WallId(3)))
        );
        assert!(archivist.walls.is_empty());
        assert!(archivist.special1.is_none());
        assert!(archivist.special2.is_none());
    }

    #[test]
    #[should_panic(expected = "Found a preview decorator")]
    fn extract_before_after_rejects_preview_decorator_state() {
        let mut bucket = live_bucket(10, 0);
        bucket.decorators.get_mut(&10).unwrap().state = 1;
        let mut archivist = empty_archivist();
        archivist
            .walls
            .insert(RecoveredWallId(3), decos_to_tracked(&bucket));
        let _ = archivist.extract_before_after(DecoratorStorageAddr::Wall(WallId(3)));
    }

    #[test]
    fn diff_uses_each_live_address_prunes_unchanged_and_resets_culling() {
        let mut archive = DecoratorArchivist::<()>::default();
        let make = |id| DecoratorInfo {
            decorator_id: id,
            ..Default::default()
        };
        let first = make(1);
        let second = make(2);
        let third = make(3);
        archive.walls.insert(
            RecoveredWallId(3),
            TrackedDecorators {
                items: [
                    (1, info_to_observed(&first)),
                    (2, info_to_observed(&second)),
                ]
                .into(),
                ..Default::default()
            },
        );
        archive.special1 = Some(TrackedDecorators {
            items: [(3, info_to_observed(&third))].into(),
            ..Default::default()
        });
        archive.culling.allow_any = true;
        archive.culling.allowed_ids.insert(99);
        let mut storage = LiveDecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(WallId(3)))
            .insert(DecoratorId(1), first);
        // ID2 is removed, ID4 is new, terrain ID3 is unrelated and unchanged.
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(WallId(3)))
            .insert(DecoratorId(4), make(4));
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(DecoratorId(3), third);
        let out = archive.extract_diff_against_storage(&storage, false);
        assert_eq!(out.len(), 1);
        let wall = &out[&DecoratorStorageAddr::Wall(WallId(3))];
        assert_eq!(
            wall.before.states.keys().copied().collect::<HashSet<_>>(),
            HashSet::from([DecoratorId(2)])
        );
        assert_eq!(
            wall.after.states.keys().copied().collect::<HashSet<_>>(),
            HashSet::from([DecoratorId(4)])
        );
        assert!(archive.walls.is_empty() && archive.special1.is_none());
        assert!(!archive.culling.allow_any && archive.culling.allowed_ids.is_empty());
    }

    #[test]
    fn diff_preview_validation_runs_on_live_after_and_flag_allows_it() {
        let mut archive = DecoratorArchivist::<()>::default();
        archive.special1 = Some(TrackedDecorators::default());
        let mut storage = LiveDecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(
                DecoratorId(8),
                DecoratorInfo {
                    decorator_id: 8,
                    state: 1,
                    ..Default::default()
                },
            );
        let mut strict = DecoratorArchivist::<()>::default();
        strict.special1 = Some(TrackedDecorators::default());
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || strict.extract_diff_against_storage(&storage, false)
            ))
            .is_err()
        );
        assert!(strict.special1.is_none());
        assert_eq!(
            archive.extract_diff_against_storage(&storage, true).len(),
            1
        );
    }

    #[test]
    fn history_equality_keeps_nan_and_respects_active_destination_and_common_fields() {
        let zero = DecoratorHistoryState::default();
        let mut signed = zero;
        signed.dst_wall = u64::from((-0.0f32).to_bits());
        assert!(archived_history_equal(&zero, &signed));
        let mut n = zero;
        n.dst_wall = u64::from(f32::NAN.to_bits());
        assert!(!archived_history_equal(&n, &n));
        for tag in [2, 3, 4] {
            let mut b = zero;
            b.u20 = tag << 16;
            let mut inactive = b;
            inactive.u18 = 99 << 56;
            assert!(archived_history_equal(&b, &inactive));
            inactive.u4c = 1;
            assert!(!archived_history_equal(&b, &inactive));
        }
    }

    #[test]
    fn extract_diff_also_emits_terrain_and_stair_entries() {
        let mut archivist = empty_archivist();
        archivist
            .walls
            .insert(RecoveredWallId(3), tracked_bucket(10));
        archivist.special1 = Some(tracked_bucket(11));
        archivist.special2 = Some(tracked_bucket(12));
        let out = archivist.extract_diff();
        assert!(out.contains_key(&DecoratorStorageAddr::Wall(WallId(3))));
        assert!(out.contains_key(&DecoratorStorageAddr::Terrain));
        assert!(out.contains_key(&DecoratorStorageAddr::Stair));
        assert!(archivist.walls.is_empty());
        assert!(archivist.special1.is_none());
        assert!(archivist.special2.is_none());
    }
}

#[path="D:/game/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist/changes.rs"] pub mod changes;
