//! 装饰编辑的共享归档 owner，沿用 B 的桶、碰撞快照和恢复生命周期。
//!
//! - B @1413e77f0：按 Wall/Terrain/Stair 地址保留首次快照，缺桶也归档为空。
//! - B @1412b2a70 / @1412b3910：数据桶归档后，依次读取两张活碰撞表，
//!   保存活 collider 的变换与网格 Arc；快照不依赖随后可能卸载的句柄。
//! - B @140b2cb30：按地址与当前存储配对，移除原语义相等的条目后产出历史差分，
//!   同时释放碰撞快照并重置剔除许可。+0xc0/+0x150 是 Terrain/Stair 桶，
//!   不能误解成全局 before/after。
//! - B @140b2c4b0：取消编辑时 drain/take 归档桶，经权威存储的 tracked_clone_into
//!   恢复数据和全局装饰索引；碰撞载荷随归档卸载释放。
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
/// 活碰撞体与网格从这两个权威资源读取；资源本身不被归档修改。
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
    utils::iso_transform::IsoTransform,
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

/// B @1415237c0 内联 PartialEq：目的变体只比较有效字段；浮点用 IEEE
/// 相等比较（NaN 不相等、正负零相等），不能对重建载荷的 u32 位模式派生判等。
fn history_states_equal(a: &DecoratorHistoryState, b: &DecoratorHistoryState) -> bool {
    let float_eq = |x: u32, y: u32| f32::from_bits(x) == f32::from_bits(y);
    let low = |x: u64| x as u32;
    let high = |x: u64| (x >> 32) as u32;
    let a_tag = ((a.u20 >> 16) & 0xff) as u8;
    let b_tag = ((b.u20 >> 16) & 0xff) as u8;
    let variant = |tag: u8| if (2..=4).contains(&tag) { tag - 1 } else { 0 };
    if variant(a_tag) != variant(b_tag) {
        return false;
    }
    let same_destination = match a_tag {
        2 | 3 => {
            float_eq(low(a.dst_wall), low(b.dst_wall))
                && float_eq(high(a.dst_wall), high(b.dst_wall))
                && float_eq(low(a.dst_secondary), low(b.dst_secondary))
        }
        4 => {
            a.dst_wall == b.dst_wall
                && float_eq(low(a.dst_secondary), low(b.dst_secondary))
                && float_eq(high(a.dst_secondary), high(b.dst_secondary))
                && float_eq(low(a.dst_payload), low(b.dst_payload))
        }
        _ => {
            a_tag == b_tag
                && (a.u20 as u16) == (b.u20 as u16)
                && a.dst_secondary == b.dst_secondary
                && float_eq(low(a.dst_wall), low(b.dst_wall))
                && float_eq(high(a.dst_wall), high(b.dst_wall))
                && float_eq(low(a.dst_payload), low(b.dst_payload))
                && float_eq(high(a.dst_payload), high(b.dst_payload))
                && float_eq(low(a.u18), low(b.u18))
                && float_eq(high(a.u18), high(b.u18))
        }
    };
    same_destination
        && a.ty == b.ty
        && a.rank == b.rank
        && a.u49 == b.u49
        && a.u28 == b.u28
        && float_eq(a.u30, b.u30)
        && a.u34 == b.u34
        && float_eq(a.u38, b.u38)
        && float_eq(a.u3c, b.u3c)
        && a.u40 == b.u40
        && a.u44 == b.u44
        && a.u4b == b.u4b
        && a.u4c == b.u4c
        && a.u4d == b.u4d
        && a.style == b.style
}

impl<C: Send + Sync + 'static> DecoratorArchivist<C> {
    /// B @140b2cb30：每个归档地址的原状态与当前权威存储配对；相同条目从两侧
    /// 同时删除，剩余条目才进入历史。Terrain/Stair 是独立地址，并非 before/after。
    /// @1409dd921 记录传 false；@140b382ce 回放清理传 true，跳过附加预览诊断。
    pub fn extract_diff(
        &mut self,
        storage: &LiveDecoratorStorage,
        skip_preview_validation: bool,
    ) -> HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff> {
        self.collision_a.clear();
        self.collision_b.clear();
        let walls = std::mem::take(&mut self.walls);
        let terrain = self.special1.take();
        let stair = self.special2.take();
        let archived = walls
            .into_iter()
            .map(|(wall, bucket)| (DecoratorStorageAddr::Wall(WallId(wall.0)), bucket))
            .chain(
                terrain
                    .into_iter()
                    .map(|bucket| (DecoratorStorageAddr::Terrain, bucket)),
            )
            .chain(
                stair
                    .into_iter()
                    .map(|bucket| (DecoratorStorageAddr::Stair, bucket)),
            );
        let mut out = HashMap::new();
        for (address, bucket) in archived {
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
            if let Some(current) = storage.get(address) {
                for (id, info) in current.decorators() {
                    if !skip_preview_validation && info.state != 0 {
                        panic!(
                            "Found a preview decorator while trying to extract history diff. {:?}: {:?}",
                            DecoratorId(*id),
                            info
                        );
                    }
                    after.insert(DecoratorId(*id), DecoratorHistoryState::from(info));
                }
            }
            // 原 @1415237c0 消费 before 迭代器，同时从 before/after 移除相等项。
            before.retain(|id, old| {
                let unchanged = after
                    .get(id)
                    .is_some_and(|new| history_states_equal(old, new));
                if unchanged {
                    after.remove(id);
                }
                !unchanged
            });
            if !before.is_empty() || !after.is_empty() {
                out.insert(
                    address,
                    DecoratorStorageHistoryDiff {
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
        // B @140b2cc8d..ccd2：一次提取结束本轮允许剔除的装饰集合和全允许标志。
        self.culling.allowed_ids.clear();
        self.culling.allow_any = false;
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
    /// 此兼容入口只接收数据桶；实际编辑必须使用带真实资源的归档入口，
    /// 才能同时保存 collider 位姿和网格。
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

/// 原 B `archive_storage_decorators` 的碰撞载荷适配。原生先复制活 collider 的
/// 28B 平移/四元数，再克隆完整 RaycastMesh；不保存句柄，也不额外克隆 shape Arc。
#[cfg(feature = "native-physics")]
fn archive_live_collider_map(
    target: &mut HashMap<u32, LiveColliderPayload>,
    decorators: &crate::resources::walls::decorator_storage::Decorators,
    colliders: &HashMap<DecoratorId, crate::systems::collision::world::ColliderMeshInstance>,
    world: &crate::systems::collision::world::RaycastWorld,
) {
    for id in decorators.decorators().keys() {
        let Some(instance) = colliders.get(&DecoratorId(*id)) else {
            continue;
        };
        // B @1412b30b0 / @1412b33c0：网格索引来自活 collider.user_data，
        // 不能以装饰表可能陈旧的 registry_index/generation 代替。
        let collider = world.get_collider(instance.handle());
        assert_ne!(
            collider.generation, 0,
            "archived collider has no mesh generation"
        );
        let mesh = world.get_collider_mesh(collider.generation_low as u32, collider.generation);
        let transform = utils::iso_transform::IsoTransform::from_translation_rotation(
            collider.position,
            collider.rotation,
        );
        // 原 insert 会替换重复 id 并释放旧网格 Arc，而不是保留先到的碰撞载荷。
        target.insert(*id, (transform, mesh.clone()));
    }
}

impl LiveDecoratorArchivist {
    /// B @1413e77f0 的地址切片单态适配：逐地址归档原始桶，再读取两张碰撞表。
    /// 缺失的桶也归档为空桶，才能撤销本轮新创建的存储；同地址只归档一次。
    /// `source` 是原版调用来源诊断标签，不进入持久化/历史载荷。
    pub fn archive_current_decorators_in_storages_with_resources(
        &mut self,
        addresses: &[DecoratorStorageAddr],
        storage: &LiveDecoratorStorage,
        resources: &DecoratorArchivalResources<'_>,
        source: &str,
    ) {
        for &address in addresses {
            self.archive_current_decorators_at_with_resources(address, storage, resources, source);
        }
    }

    /// 单地址入口保持同一 archivist owner，供其它编辑系统逐步迁移。
    pub fn archive_current_decorators_at_with_resources(
        &mut self,
        address: DecoratorStorageAddr,
        storage: &LiveDecoratorStorage,
        resources: &DecoratorArchivalResources<'_>,
        source: &str,
    ) {
        self.archive_current_decorators_at_with_owners(address, storage,
            &resources.raycast_world,
            #[cfg(feature = "native-physics")]
            &resources.colliders,
            source);
    }

    /// 原维护楼梯图已经持有ResMut<RaycastWorld>，只读归档借用同一owner。
    /// 不另建SystemParam、克隆碰撞世界或遗漏原两张碰撞表；原入口也委派此实现。
    pub fn archive_current_decorators_at_with_owners(
        &mut self,
        address: DecoratorStorageAddr,
        storage: &LiveDecoratorStorage,
        raycast_world: &crate::systems::collision::world::RaycastWorld,
        #[cfg(feature = "native-physics")]
        colliders: &crate::resources::walls::decorator_collision::DecoratorPhysicsColliders,
        _source: &str,
    ) {
        let exists = match address {
            DecoratorStorageAddr::Wall(wall) => self.walls.contains_key(&RecoveredWallId(wall.0)),
            DecoratorStorageAddr::Terrain => self.special1.is_some(),
            DecoratorStorageAddr::Stair => self.special2.is_some(),
        };
        if exists {
            return;
        }
        let empty = crate::resources::walls::decorator_storage::Decorators::default();
        let decorators = storage.get(address).unwrap_or(&empty);
        let (selector, wall) = match address {
            DecoratorStorageAddr::Wall(wall) => (0, Some(wall)),
            DecoratorStorageAddr::Terrain => (1, None),
            DecoratorStorageAddr::Stair => (2, None),
        };
        self.archive_storage_decorators(selector, decorators, wall);
        #[cfg(feature = "native-physics")]
        {
            // 原执行序为整桶 first，再整桶 second，保持两类碰撞形状彼此独立。
            archive_live_collider_map(
                &mut self.collision_a,
                decorators,
                &colliders.first,
                raycast_world,
            );
            archive_live_collider_map(
                &mut self.collision_b,
                decorators,
                &colliders.second,
                raycast_world,
            );
        }
        #[cfg(not(feature = "native-physics"))]
        let _ = raycast_world;
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
        let out = archivist.extract_diff(&LiveDecoratorStorage::default(), false);
        let diff = out.get(&DecoratorStorageAddr::Wall(WallId(7))).unwrap();
        assert_eq!(diff.before.states.len(), 1);
        assert!(diff.before.removed.is_empty());
        assert!(diff.after.states.is_empty());
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
        storage.per_wall.insert(WallId(1), live_bucket(10, 0));
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
        let out = archivist.extract_diff(&LiveDecoratorStorage::default(), false);
        assert!(out.contains_key(&DecoratorStorageAddr::Wall(WallId(3))));
        assert!(out.contains_key(&DecoratorStorageAddr::Terrain));
        assert!(out.contains_key(&DecoratorStorageAddr::Stair));
        assert!(archivist.walls.is_empty());
        assert!(archivist.special1.is_none());
        assert!(archivist.special2.is_none());
    }

    #[test]
    fn extract_diff_compares_each_current_address_filters_unchanged_and_replays_both_directions() {
        use crate::resources::walls::decorator::{DecoratorIdGen, DeferredDecoratorOpEvents};
        use crate::resources::walls::decorator_storage::DecoratorColliderRemover;
        struct NoPhysics;
        impl DecoratorColliderRemover for NoPhysics {
            fn remove_decorator_collider(&mut self, _: DecoratorId) {}
        }
        let wall = DecoratorStorageAddr::Wall(WallId(5));
        let terrain = DecoratorStorageAddr::Terrain;
        let stair = DecoratorStorageAddr::Stair;
        let mut storage = LiveDecoratorStorage::default();
        for (address, id) in [(wall, 10), (terrain, 20), (stair, 30)] {
            storage.apply_decorator_place(
                address,
                DecoratorId(id),
                &DecoratorInfo {
                    decorator_id: id,
                    ..Default::default()
                },
            );
        }
        let mut archive = empty_archivist();
        archive.archive_current_decorators_in_storage(&storage);
        archive.decorator_cull_allow_any();
        archive.culling.allowed_ids.insert(10);
        storage
            .get_or_insert_default(wall)
            .decorators
            .get_mut(&10)
            .unwrap()
            .rank = 2;
        // 原移除/加入依赖全局d2s索引，使用已有权威入口更新。
        storage.apply_decorator_remove(terrain, DecoratorId(20));
        storage.apply_decorator_place(
            terrain,
            DecoratorId(21),
            &DecoratorInfo {
                decorator_id: 21,
                ..Default::default()
            },
        );
        let diff = archive.extract_diff(&storage, false);
        assert_eq!(diff.len(), 2);
        assert!(!diff.contains_key(&stair));
        assert_eq!(diff[&wall].before.states[&DecoratorId(10)].rank, 0);
        assert_eq!(diff[&wall].after.states[&DecoratorId(10)].rank, 2);
        assert!(diff[&terrain].before.states.contains_key(&DecoratorId(20)));
        assert!(diff[&terrain].after.states.contains_key(&DecoratorId(21)));
        assert!(archive.culling.allowed_ids.is_empty());
        assert!(!archive.culling.allow_any);
        let mut ids = DecoratorIdGen::default();
        let events = DeferredDecoratorOpEvents::default();
        storage.restore_decorators(&diff, true, &mut ids, &mut NoPhysics, &events);
        assert_eq!(storage.get(wall).unwrap().decorators()[&10].rank, 0);
        assert!(storage.get(terrain).unwrap().decorators().contains_key(&20));
        assert!(!storage.get(terrain).unwrap().decorators().contains_key(&21));
        storage.restore_decorators(&diff, false, &mut ids, &mut NoPhysics, &events);
        assert_eq!(storage.get(wall).unwrap().decorators()[&10].rank, 2);
        assert!(!storage.get(terrain).unwrap().decorators().contains_key(&20));
        assert!(storage.get(terrain).unwrap().decorators().contains_key(&21));
    }

    #[test]
    fn history_equality_uses_variant_payload_and_native_float_semantics() {
        let mut a = DecoratorHistoryState::default();
        let mut b = a;
        b.dst_wall = 0x8000_0000;
        assert!(history_states_equal(&a, &b));
        a.dst_wall = 0x7fc0_0000;
        b.dst_wall = a.dst_wall;
        assert!(!history_states_equal(&a, &b));
        a = DecoratorHistoryState {
            u20: 3 << 16,
            ..Default::default()
        };
        b = a;
        b.dst_payload = u64::MAX;
        b.u18 = u64::MAX;
        b.u24 = u32::MAX;
        assert!(history_states_equal(&a, &b));
        b.u28 = 1;
        assert!(!history_states_equal(&a, &b));
    }

    #[test]
    fn resource_archive_records_missing_buckets_and_preserves_first_snapshot() {
        use bevy_ecs::system::SystemState;
        let mut world = bevy_ecs::world::World::new();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        let mut params = SystemState::<DecoratorArchivalResources>::new(&mut world);
        let mut archivist = LiveDecoratorArchivist::default();
        let mut storage = LiveDecoratorStorage::default();
        let addresses = [
            DecoratorStorageAddr::Wall(WallId(51)),
            DecoratorStorageAddr::Terrain,
            DecoratorStorageAddr::Stair,
        ];
        archivist.archive_current_decorators_in_storages_with_resources(
            &addresses,
            &storage,
            &params.get(&world),
            "change_elevation",
        );
        for (index, address) in addresses.into_iter().enumerate() {
            storage
                .get_or_insert_default(address)
                .insert(DecoratorId(90 + index as u32), DecoratorInfo::default());
        }
        for (index, address) in addresses.into_iter().enumerate() {
            storage
                .decorator_to_storage
                .insert(DecoratorId(90 + index as u32), address);
        }
        archivist.archive_current_decorators_in_storages_with_resources(
            &addresses,
            &storage,
            &params.get(&world),
            "change_elevation",
        );
        assert_eq!(archivist.all_storages().count(), 3);
        assert_eq!(archivist.all_decorators().count(), 0);
        archivist.extract_and_restore_to_storage(&mut storage);
        assert_eq!(archivist.all_storages().count(), 0);
        for address in addresses {
            assert!(storage.get(address).unwrap().decorators().is_empty());
        }
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn resource_archive_owns_both_live_meshes_across_disposal_and_releases_on_restore() {
        use crate::resources::walls::decorator_collision::DecoratorPhysicsColliders;
        use crate::systems::collision::world::{
            RaycastGeoSource, RaycastMeshData, RaycastWorld, upsert_collider,
        };
        use bevy_ecs::system::SystemState;
        use glam::{Quat, Vec3};
        use std::sync::Arc;
        let id = DecoratorId(71);
        let address = DecoratorStorageAddr::Wall(WallId(8));
        let mut storage = LiveDecoratorStorage::default();
        storage.get_or_insert_default(address).insert(
            id,
            DecoratorInfo {
                decorator_id: id.0,
                dst_wall: 8,
                u40: 3 << 8,
                ..Default::default()
            },
        );
        storage.decorator_to_storage.insert(id, address);
        let mut raycast = RaycastWorld::new();
        let mut maps = DecoratorPhysicsColliders::default();
        let data_a = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::Y));
        let data_b = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::X));
        let weak_a = Arc::downgrade(&data_a);
        let weak_b = Arc::downgrade(&data_b);
        let rotation = Quat::from_rotation_z(0.37);
        upsert_collider(
            &mut raycast,
            &mut maps.first,
            id,
            RaycastGeoSource::decorator(id, address),
            data_a.clone(),
            Vec3::new(3.0, 4.0, 5.0),
            rotation,
        );
        upsert_collider(
            &mut raycast,
            &mut maps.second,
            id,
            RaycastGeoSource::decorator_interaction(id, address, 0),
            data_b.clone(),
            Vec3::new(-2.0, 1.0, 8.0),
            Quat::IDENTITY,
        );
        let pose_a = raycast.get_collider(maps.first[&id].handle());
        let pose_b = raycast.get_collider(maps.second[&id].handle());
        // 表中注册回执故意陈旧；B 依然以活 collider.user_data 查询网格。
        maps.first.get_mut(&id).unwrap().registry_index = u32::MAX;
        maps.first.get_mut(&id).unwrap().generation = u32::MAX;
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(raycast);
        world.insert_resource(maps);
        let mut params = SystemState::<DecoratorArchivalResources>::new(&mut world);
        let mut archivist = LiveDecoratorArchivist::default();
        archivist.archive_current_decorators_in_storages_with_resources(
            &[address],
            &storage,
            &params.get(&world),
            "change_elevation",
        );
        let (transform_a, mesh_a) = &archivist.collision_a[&id.0];
        let (transform_b, mesh_b) = &archivist.collision_b[&id.0];
        assert_eq!(transform_a.translation, pose_a.position);
        assert_eq!(transform_a.rotation, pose_a.rotation);
        assert_eq!(transform_b.translation, pose_b.position);
        assert_eq!(transform_b.rotation, pose_b.rotation);
        assert!(Arc::ptr_eq(&mesh_a.data, &data_a));
        assert!(Arc::ptr_eq(&mesh_b.data, &data_b));
        assert_ne!(mesh_a.source, mesh_b.source);
        drop(data_a);
        drop(data_b);
        // 销毁整个活世界后归档仍持有原网格；恢复只移回数据，碰撞快照卸载释放。
        drop(params);
        drop(world);
        assert!(weak_a.upgrade().is_some());
        assert!(weak_b.upgrade().is_some());
        storage
            .get_or_insert_default(address)
            .decorators
            .get_mut(&id.0)
            .unwrap()
            .u40 = 9 << 8;
        archivist.extract_and_restore_to_storage(&mut storage);
        assert_eq!(
            storage.get(address).unwrap().decorators()[&id.0].extra_style(),
            3
        );
        assert!(archivist.collision_a.is_empty());
        assert!(archivist.collision_b.is_empty());
        assert!(weak_a.upgrade().is_none());
        assert!(weak_b.upgrade().is_none());
    }

    #[cfg(feature = "native-physics")]
    #[test]
    #[should_panic(expected = "get_collider: collider handle invalid")]
    fn resource_archive_rejects_stale_live_handle_instead_of_silent_missing_mesh() {
        use crate::systems::collision::world::{
            RaycastGeoSource, RaycastMeshData, RaycastWorld, upsert_collider,
        };
        use glam::{Quat, Vec3};
        use std::sync::Arc;
        let id = DecoratorId(70);
        let mut colliders = HashMap::new();
        let mut world = RaycastWorld::new();
        upsert_collider(
            &mut world,
            &mut colliders,
            id,
            RaycastGeoSource::decorator(id, DecoratorStorageAddr::Terrain),
            Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
            Vec3::ZERO,
            Quat::IDENTITY,
        );
        world.on_level_reload();
        archive_live_collider_map(
            &mut HashMap::new(),
            &live_bucket(id.0, 0),
            &colliders,
            &world,
        );
    }
}

pub mod changes;
