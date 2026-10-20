//! Shared archive owner; reuse the existing storage/restore implementation.
//!
//! 2026-10-17 archivist 迭代链 5 实现（impl-history-difftypes，fresh decomp
//! batch1 0x140b2cb30 / 0x140b2be70 / 0x1412b2a70 / 0x1412b3910 / 0x14123b5c0 /
//! 0x1412b4f10）：
//! - `extract_diff`（0x140b2cb30）：清碰撞 map → memmove before@+0xc0 / after@+0x150
//!   （0x90B Decorators，即 recovered 模型 special1/special2）→ 三路 chain
//!   （walls Drain + special1/2 IntoIter，均经 `extract_archived_decorators` 闭包
//!   映射为 `(DecoratorStorageAddr, Decorators)`）→ `from_iter<Addr,(AddrState,AddrState)>`
//!   → filter_map `extract_before_after_all` → `HashMap<Addr, DecoratorStorageHistoryDiff>`；
//!   后置断言 walls/before/after len==0。
//! - `extract_before_after_impl`（0x140b2be70）：单地址状态构建（桶 items 逐条目
//!   `DecoratorInfo → DecoratorHistoryState`，`impl$1::from` 0x140b3f930）。
//! - `archive_storage_decorators<&str/String>`（0x1412b2a70/0x1412b3910）：
//!   0=墙（已存在 return + clone 入 walls）/ 1=special1（before @+0xc0，已存在
//!   return）/ 2=special2（after @+0x150）。**collision collider/mesh 重建**（
//!   `RaycastWorld::get_collider/get_collider_mesh → HashMap insert (DecoratorId,
//!   (OffsetColliderTransform, RaycastMesh))`）为 collision 域，登记不实现。
//! - `archive_current_decorators_in_storage/_in_storages`（0x14123b5c0/0x1412b4f10）：
//!   经 `DecoratorStorage::get`(0x140a1c2c0) 逐地址归档。
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

/// B `resources::walls::decorator_archivist::DecoratorArchivalResources`
/// （`#[derive(SystemParam)]`，`archive_current_decorators_in_storages` 第 5 形参）：
/// `Res<RaycastWorld>` + `Res<DecoratorPhysicsColliders>`，仅供碰撞体载荷重建读取。
/// 重建本身属 collision 域（见 `archive_storage_decorators` 说明），本参数面只转交。
#[derive(bevy_ecs::system::SystemParam)]
pub struct DecoratorArchivalResources<'w> {
    pub raycast_world: bevy_ecs::system::Res<'w, crate::systems::collision::world::RaycastWorld>,
    #[cfg(feature = "native-physics")]
    pub colliders: bevy_ecs::system::Res<
        'w,
        crate::resources::walls::decorator_collision::DecoratorPhysicsColliders,
    >,
}

/// Original archive collision payload. These are owned collision objects, not
/// addresses copied from a native allocation.
#[cfg(feature = "native-physics")]
pub type LiveColliderPayload = (
    crate::systems::collision::world::OffsetColliderTransform,
    crate::systems::collision::world::RaycastMesh,
);
#[cfg(feature = "native-physics")]
pub type LiveDecoratorArchivist = DecoratorArchivist<LiveColliderPayload>;

/// The CPU-only build shares the same archive and culling owner. Its collision
/// payload is unavailable until native-physics is enabled; UI culling changes
/// still use this owner rather than a second flag in the UI resource.
#[cfg(not(feature = "native-physics"))]
pub type LiveColliderPayload = ();
#[cfg(not(feature = "native-physics"))]
pub type LiveDecoratorArchivist = DecoratorArchivist<LiveColliderPayload>;

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

    /// 0x140b2bc00（重新核对 2026-10-20，`&mut self`，无返回值）：参数为
    /// `(archivist, &wall, &DecoratorStorage.walls)`。在**存储**的墙表里探针该墙（miss 则什么都
    /// 不做），命中则 `Map::fold`（`fold_impl` 0x140b76260 → `HashMap::insert` 0x1402c9200）
    /// 把该墙桶里的全部装饰 id 插入 `archivist+0x30`（`culling.allowed_ids`）。
    /// 调用者：decorator_on_rectangle_edited / merge_walls ×2 / ui_adjust_wall_height_mode /
    /// ui_edit_roof_ridge_dir。此前误写成读 archivist 自身 `walls` 的谓词，调用方各自
    /// 复刻了真实语义，现统一回这里。
    pub fn decorator_cull_allow_wall(&mut self, wall: WallId, storage: &LiveDecoratorStorage) {
        if let Some(decorators) = storage.get(DecoratorStorageAddr::Wall(wall)) {
            self.culling
                .allowed_ids
                .extend(decorators.decorators().keys().copied());
        }
    }

    /// 0x140b2bd30（重新核对 2026-10-20，`&mut self`，无返回值；仅被
    /// `auto_allow_decorator_culling` 0x14080d3d0 尾跳调用）：`Map::fold` 单态
    /// 0x141293280 含三处 `HashMap::insert`，即 walls + special1 + special2 三条链的
    /// 全部归档装饰 id 插入 `culling.allowed_ids`。此前误写成“stairs 桶谓词”。
    pub fn decorator_cull_allow(&mut self) {
        let ids: Vec<u32> = self.all_decorators().map(|(id, _)| *id).collect();
        self.culling.allowed_ids.extend(ids);
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

    /// B `archive_current_decorators_in_storage` 0x14123b5c0：经
    /// `DecoratorStorage::get`(0x140a1c2c0) 逐地址 → 归档入 archivist。
    /// special1/2 分别承载 Terrain/Stair 桶（判别 1/2 与 `DecoratorStorageAddr`
    /// disc 1/2 对应）。
    pub fn archive_current_decorators_in_storage(&mut self, storage: &LiveDecoratorStorage) {
        for (wall, decos) in storage.iter_walls() {
            self.archive_storage_decorators(0, decos, Some(wall));
        }
        if let Some(decos) = storage.terrain.as_ref() {
            self.archive_storage_decorators(1, decos, None);
        }
        if let Some(decos) = storage.stairs.as_ref() {
            self.archive_storage_decorators(2, decos, None);
        }
    }

    /// B `archive_current_decorators_in_storages` 0x1412b4f10：多存储批量归档。
    pub fn archive_current_decorators_in_storages(&mut self, storages: &[LiveDecoratorStorage]) {
        for storage in storages {
            self.archive_current_decorators_in_storage(storage);
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
    fn archive_current_decorators_in_storage_archives_wall_and_specials() {
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
        archivist.archive_current_decorators_in_storage(&storage);
        assert!(archivist.walls.contains_key(&RecoveredWallId(3)));
        assert!(archivist.special1.is_some());
        // 再归档：已存在 → return。
        archivist.archive_current_decorators_in_storage(&storage);
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
    fn decorator_cull_allow_wall_inserts_storage_bucket_ids_and_allow_all_inserts_archived() {
        let mut archivist = empty_archivist();
        // 原版放行读的是存储里的墙桶，不是 archivist 自己的 walls。
        let mut storage = LiveDecoratorStorage::default();
        storage
            .per_wall
            .insert(WallId(1), live_bucket(10, 0));
        archivist.decorator_cull_allow_wall(WallId(2), &storage);
        assert!(archivist.culling.allowed_ids.is_empty());
        archivist.decorator_cull_allow_wall(WallId(1), &storage);
        assert!(archivist.culling.allowed_ids.contains(&10));
        assert!(!archivist.is_decorator_cull_allowed(11));
        archivist
            .walls
            .insert(RecoveredWallId(1), tracked_bucket(11));
        archivist.special2 = Some(tracked_bucket(20));
        archivist.decorator_cull_allow();
        for id in [10, 11, 20] {
            assert!(archivist.is_decorator_cull_allowed(id));
        }
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

pub mod changes;
