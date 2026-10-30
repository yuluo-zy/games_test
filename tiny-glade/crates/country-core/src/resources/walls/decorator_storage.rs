//! B `resources/walls/decorator_storage.rs` — 装饰权威存储（原 114 函数：业务 60）。
//!
//! 证据：`RE-DECORATOR-STORAGE.md`（实体机制结论）+ 章节
//! `chapters/resources/walls/decorator_storage.rs.md`（布局 O 全表）+ `wall/decomp/*`。
//!
//! **实体机制（核心，O）**：DecoratorStorage 存纯数据（DecoratorInfo 72B 记录），
//! **不含 bevy Entity**；装饰实体由消费系统（ui_clutter / 渲染 / 音频）读
//! DecoratorInfo 后另行生成，实体↔装饰关联靠 DecoratorId 组件（RE §2）。
//!
//! 布局（O，章节 §算法级）：per_wall HashMap@+0x00（条目 0x98B={WallId u64,
//! Decorators 0x90B}）、removed/目的地注册表@+0x30、decorator_to_storage
//! 索引@+0x60（条目 24B={u32 id, pad, u64 判别, u64 payload}）、terrain 内联
//! @+0x90、stairs 内联@+0x120、immutable u8@+0x1b0。Rust std 容器与原 hashbrown
//! 布局不同，偏移仅作语义参考，非 ABI 契约。

use std::collections::{HashMap, HashSet};

use glam::Vec2;
use utils::curve::Curve2;

use super::WallId;
use super::decorator::{DecoratorId, DecoratorIdGen, DecoratorInfo, DeferredDecoratorOpEvents};
use crate::resources::history::decorator_edit::{
    send_state_diff_events, DecoratorBeforeAfterWithStorage, DecoratorHistoryState,
    DecoratorStateWithStorage, DecoratorStorageHistoryDiff,
};

/// B DecoratorStorageAddr：装饰目的存储地址（d2s 索引 24B 条目判别 +0/1/2 =
/// Wall/Terrain/Stair；payload = WallId）。repr u64 16B：{disc@0, payload@8}。
/// WallId 未实现 serde（walls/mod.rs），故本枚举不派生序列化；wire 格式 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DecoratorStorageAddr {
    /// disc 0：墙（payload = WallId）。
    Wall(WallId),
    /// disc 1：地形。
    Terrain,
    /// disc 2：楼梯。
    Stair,
}

impl Default for DecoratorStorageAddr {
    fn default() -> Self {
        DecoratorStorageAddr::Wall(WallId::default())
    }
}

impl DecoratorStorageAddr {
    /// 0x140a1c240（O）：disc==0 → WallId，否则 fmt panic。
    pub fn as_wall(self) -> WallId {
        match self {
            DecoratorStorageAddr::Wall(w) => w,
            _ => panic!("DecoratorStorageAddr::as_wall: not a wall storage"),
        }
    }

    /// 0x140a1c290（O）：disc==0 → Some(WallId)，否则 None。
    pub fn try_as_wall(self) -> Option<WallId> {
        match self {
            DecoratorStorageAddr::Wall(w) => Some(w),
            _ => None,
        }
    }
}

/// B DecoratorToStorageMap：d2s 索引（DecoratorId → 地址）。原布局
/// `Arc<HashMap>`（+0x0/+0x8 Arc 计数、+0x10 map 指针）——合并/分裂时 Arc 克隆
/// 共享；Rust 侧以 `Arc<HashMap>` 包装保持共享语义。
///
/// - `impl$8::remove` 0x140a1b440：存在性守卫，未命中 panic "d2s remove: no such {id}"
///   （fat 0x142ac8fb0）；删除在调用点执行。
/// - `impl$8::insert` 0x140a1b4e0：插入，冲突 panic "d2s insert: already exists; {id}"
///   （fat 0x142ac8fe8，Display 闭包 0x140a24330/0x140a24370）。
/// - `impl$8::update` 0x140a1b5a0：键值集合更新（修改追踪集合维护，I）。
#[derive(Clone, Debug, Default)]
pub struct DecoratorToStorageMap {
    /// 内部 HashMap（原为 Arc 共享数据）。
    map: HashMap<u32, DecoratorStorageAddr>,
}

impl DecoratorToStorageMap {
    /// 查询（lookup_decorator_info 首层，O）。
    pub fn get(&self, id: DecoratorId) -> Option<DecoratorStorageAddr> {
        self.map.get(&id.0).copied()
    }

    /// 0x140a1b440（O）：存在性守卫；未命中 panic。删除在调用点执行。
    pub fn assert_exists(&self, id: DecoratorId) {
        if !self.map.contains_key(&id.0) {
            panic!("d2s remove: no such {}", id);
        }
    }

    /// 0x140a1b4e0（O）：插入；冲突 panic。
    pub fn insert(&mut self, id: DecoratorId, addr: DecoratorStorageAddr) {
        if self.map.insert(id.0, addr).is_some() {
            panic!("d2s insert: already exists; {}", id);
        }
    }

    /// 0x140a1b5a0（I）：更新键值（返回旧值）。
    pub fn update(&mut self, id: DecoratorId, addr: DecoratorStorageAddr) -> Option<DecoratorStorageAddr> {
        self.map.insert(id.0, addr)
    }

    /// 直接删除（墙事件/删除族使用；未命中保留原 panic 语义可选）。
    pub fn remove(&mut self, id: DecoratorId) -> Option<DecoratorStorageAddr> {
        self.map.remove(&id.0)
    }

    /// 空判断（restore_snapshot 前置断言 `decorator_to_storage.map.is_empty()`）。
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// B DecoratorMaybeMut：装饰集合的共享/可变二象访问（迭代族使用）。
///
/// `get` 0x1400885d0（混名 WallId::get_inner）返回共享；`get_mut` 0x140a1b1e0
/// （helper([+8],[+0x10]) 后返回 [+0] 指针）返回可变。Mut 变体持 &mut；共享
/// 变体取可变即 panic（原不可行路径）。
#[derive(Debug)]
pub enum DecoratorMaybeMut<'a> {
    /// 共享访问。
    Ref(&'a Decorators),
    /// 可变访问。
    Mut(&'a mut Decorators),
}

impl<'a> DecoratorMaybeMut<'a> {
    /// 0x1400885d0（O）：返回共享引用。
    pub fn get(&self) -> &Decorators {
        match self {
            DecoratorMaybeMut::Ref(d) => d,
            DecoratorMaybeMut::Mut(d) => d,
        }
    }

    /// 0x140a1b1e0（O）：返回可变引用（消费自身，移出 &mut）；共享变体 panic
    /// （unreachable）。
    pub fn get_mut(self) -> &'a mut Decorators {
        match self {
            DecoratorMaybeMut::Ref(_) => unreachable!("DecoratorMaybeMut::get_mut on shared"),
            DecoratorMaybeMut::Mut(d) => d,
        }
    }
}

/// B Decorators：单墙/单目的地的装饰集合（0x90B）。
///
/// 布局（O）：+0x00 removed_ids 集合、+0x30 内嵌 map
/// `HashMap<u32 DecoratorId, DecoratorInfo 0x48B>`（条目 0x50B）、+0x60
/// modified_ids 集合。
#[derive(Clone, Debug, Default)]
pub struct Decorators {
    /// +0x00：已移除 DecoratorId 集合。
    pub removed_ids: HashSet<u32>,
    /// +0x30：内嵌 map（DecoratorId → DecoratorInfo）。
    pub decorators: HashMap<u32, DecoratorInfo>,
    /// +0x60：已修改 DecoratorId 集合。
    pub modified_ids: HashSet<u32>,
}

impl Decorators {
    /// 行 55（内联）：into_inner 访问器（I）。
    pub fn into_inner(self) -> HashMap<u32, DecoratorInfo> {
        self.decorators
    }

    /// 行 64（内联）：decorators 访问器（I）。
    pub fn decorators(&self) -> &HashMap<u32, DecoratorInfo> {
        &self.decorators
    }

    /// 0x140a1b030（O）：modified_ids 迭代。
    pub fn iter_modified_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.modified_ids.iter().copied()
    }

    /// 0x140a1b070（O）：modified（id, info）迭代（modified_ids → 内嵌 map 查值）。
    pub fn iter_modified(&self) -> impl Iterator<Item = (u32, &DecoratorInfo)> + '_ {
        self.modified_ids
            .iter()
            .filter_map(|id| self.decorators.get(id).map(|info| (*id, info)))
    }

    /// 0x140a1b0b0（O，闭包）：modified 可变迭代（避免借用冲突，原即闭包形）。
    pub fn iter_modified_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(u32, &mut DecoratorInfo),
    {
        let ids: Vec<u32> = self.modified_ids.iter().copied().collect();
        for id in ids {
            if let Some(info) = self.decorators.get_mut(&id) {
                f(id, info);
            }
        }
    }

    /// 0x140a1b100（O，与 iter_walls 同址）：removed_ids 迭代。
    pub fn iter_removed_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.removed_ids.iter().copied()
    }

    /// 0x140a1b140（O）：全部标记为 modified。
    pub fn mark_all_modified(&mut self) {
        self.modified_ids.extend(self.decorators.keys().copied());
    }

    /// 行 99（内联）+ 0x141257900（O）：清除修改追踪（modified 集 + 内嵌 map
    /// 一并清空，原 `Decorators::clear<DecoratorToStorageMap>`）。
    pub fn clear_modification_tracking(&mut self) {
        self.modified_ids.clear();
        self.decorators.clear();
    }

    /// 行 159（内联）+ 0x14155afe0（O）：插入并标记 modified。fresh 反编译：
    /// 先 `RawTable::remove_entry<(DecoratorId,())>`（从 removed_ids 移除——
    /// 重新放置即撤销 removed），再 `insert<DecoratorId,()>`（modified_ids 标记）
    /// + `insert<DecoratorId,DecoratorInfo>`（内嵌 map）。返回的是旧Option；
    /// 原+0x47的2是返回None niche，不是新输入记录的业务状态。
    pub fn insert(&mut self, id: DecoratorId, info: DecoratorInfo) -> Option<DecoratorInfo> {
        self.removed_ids.remove(&id.0);
        self.modified_ids.insert(id.0);
        self.decorators.insert(id.0, info)
    }

    /// 行 181（内联）：查询。
    pub fn get(&self, id: DecoratorId) -> Option<&DecoratorInfo> {
        self.decorators.get(&id.0)
    }

    /// 行 186（内联）：可变查询。
    pub fn get_mut(&mut self, id: DecoratorId) -> Option<&mut DecoratorInfo> {
        self.decorators.get_mut(&id.0)
    }

    /// 行 192（内联）：移除。
    pub fn remove(&mut self, id: DecoratorId) -> Option<DecoratorInfo> {
        self.decorators.remove(&id.0)
    }

    /// 行 217（内联，I）：maybe-mut 迭代。原语义 U（DecoratorMaybeMut 集合级
    /// 包装）；此处按内嵌 map 共享迭代提供（id, &DecoratorInfo）。
    pub fn iter_maybe_mut(&self) -> impl Iterator<Item = (u32, &DecoratorInfo)> + '_ {
        self.decorators.iter().map(|(id, info)| (*id, info))
    }

    /// 行 226（内联）：内嵌 map 可变迭代。
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut DecoratorInfo)> + '_ {
        self.decorators.iter_mut().map(|(id, info)| (*id, info))
    }

    /// 0x14155b170（O）：合并源集合并把目标/源的 d2s 索引重指向 `addr`。
    /// fresh 三循环（2026-10-17）：
    /// ① 迭代 source.removed_ids，目标内嵌 map 无此 id → 目标 modified_ids 传播；
    /// ② 迭代目标内嵌 map → 逐 id d2s remove（miss==3 panic）+ d2s insert(addr)；
    /// ③ 消费 source owned map；for匹配真实next Option，不复制niche业务门。
    ///    d2s remove→removed撤销→modified→map重插，再按旧Option注册/更新d2s。
    /// Rust 此前为纯 map 合并（冲突→removed_ids），**不含 d2s 重索引与 removed_ids 迁移**。
    pub fn merge_with(
        &mut self,
        source: Decorators,
        d2s: &mut DecoratorToStorageMap,
        addr: DecoratorStorageAddr,
    ) {
        for id in &source.removed_ids {
            if !self.decorators.contains_key(id) {
                self.modified_ids.insert(*id);
            }
        }
        for id in self.decorators.keys() {
            d2s.remove(DecoratorId(*id))
                .expect("assertion failed: d2s remove in merge_with: no such decorator");
            d2s.insert(DecoratorId(*id), addr);
        }
        for (id, info) in source.decorators {
            d2s.remove(DecoratorId(id))
                .expect("assertion failed: d2s remove in merge_with: no such decorator");
            let old = self.insert(DecoratorId(id), info);
            if old.is_none() {
                d2s.insert(DecoratorId(id), addr);
            } else {
                d2s.update(DecoratorId(id), addr);
            }
        }
    }

    /// 0x141256e40 / 0x1412573b0（O，两泛型实例化合并实现）：tracked_clone_from。
    /// fresh 反编译（2026-10-17）：
    /// - **快路径**：目标三集合全空 → 逐集合 `clone_from`（源 removed_ids/modified_ids/
    ///   内嵌 map），随后公共尾部逐源装饰 `d2s insert`。
    /// - **慢路径**：清 modified_ids → 当前内嵌 map 逐 id 标 removed（`insert<tuple<>>`）
    ///   → drain旧条目并逐项 `d2s remove` → 源 removed_ids
    ///   回填 → 源内嵌 map 重插（removed_ids remove + modified_ids insert）。
    /// - 公共尾部：逐源装饰 `d2s insert`（冲突 panic，常量 0x142c21300/0x142c21318）。
    /// Rust 此前仅 clone_from+clear+d2s.insert，**缺慢路径的 removed 标定与 d2s
    /// 旧条目移除**（REBUILD-REMAINDER §4.2 登记）——已重建。
    pub fn tracked_clone_from(
        &mut self,
        source: &Decorators,
        d2s: &mut DecoratorToStorageMap,
        addr: DecoratorStorageAddr,
    ) {
        if self.removed_ids.is_empty() && self.modified_ids.is_empty() && self.decorators.is_empty()
        {
            // 快路径（fresh 首块：`removed_ids 空 && modified_ids 空 && decorators 空`）。
            self.removed_ids.clone_from(&source.removed_ids);
            self.modified_ids.clone_from(&source.modified_ids);
            self.decorators.clone_from(&source.decorators);
        } else {
            // 慢路径（fresh else 块）。
            // ① 清 modified_ids（`in_RCX+0x78` len 置 0）。
            self.modified_ids.clear();
            // ② 当前内嵌 map 逐 id 标 removed（insert tuple<>）。
            for id in self.decorators.keys() {
                self.removed_ids.insert(*id);
            }
            // 原RawDrain保持分配；即使索引守卫panic，Drain析构仍清理剩余旧条目。
            for (id, _) in self.decorators.drain() {
                d2s.remove(DecoratorId(id))
                    .expect("assertion failed: d2s remove in tracked_clone_from: no such decorator");
            }
            // ④ 源 removed_ids 回填。
            self.removed_ids.extend(source.removed_ids.iter().copied());
            // ⑤ 源内嵌 map 重插（removed_ids remove + modified_ids insert）。
            for (id, info) in &source.decorators {
                self.decorators.insert(*id, *info);
                self.removed_ids.remove(id);
                self.modified_ids.insert(*id);
            }
        }
        // 公共尾部：逐源装饰 d2s insert（冲突 panic）。
        for id in source.decorators.keys() {
            d2s.insert(DecoratorId(*id), addr);
        }
    }
}

/// B DecoMoveToStorage：装饰迁移记录（0x140a1b430 = new，3 字段 {r8,r9,rdx}，
/// O 构造；字段名 I：split 迁移目标墙/u 区间/子侧）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoMoveToStorage {
    /// r8：目标墙。
    pub wall: WallId,
    /// r9：u 区间（I）。
    pub u: f32,
    /// rdx：子侧（I）。
    pub side: u8,
}

impl DecoMoveToStorage {
    /// 0x140a1b430（O）：3 字段构造。
    pub fn new(wall: WallId, u: f32, side: u8) -> Self {
        Self { wall, u, side }
    }
}

/// One storage bucket in a save snapshot. B capture maps every existing bucket,
/// including an empty bucket, to its actual ids and normalized history states.
/// Native mapper 0x141559890 sorts ids; 0x1412ff7b0 emits 0x58-byte items.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DecoratorAddrSnapshot {
    pub states: Vec<(DecoratorId, DecoratorHistoryState)>,
}

/// B save snapshot: 0x28-byte native outer rows are (storage address, bucket Vec).
/// The representation preserves absent versus present-but-empty terrain/stairs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DecoratorStorageSnapshot {
    pub states: Vec<(DecoratorStorageAddr, DecoratorAddrSnapshot)>,
}

impl DecoratorStorageSnapshot {
    /// Explicit canonical ordering for callers that request it. Capture itself
    /// retains the wall iterator order followed by terrain and stairs.
    pub fn sort(&mut self) {
        self.states.sort_by_key(|(addr, _)| match addr {
            DecoratorStorageAddr::Wall(wall) => (0, wall.0),
            DecoratorStorageAddr::Terrain => (1, 0),
            DecoratorStorageAddr::Stair => (2, 0),
        });
        for (_, bucket) in &mut self.states {
            bucket.states.sort_by_key(|(id, _)| *id);
        }
    }
}

/// 子侧状态字节公式（E2 取证定案，2026-10-16；fresh 0x140a1f590）：
/// `(b-2)<=2 ? b-1 : 0`（u8 回绕）。b∈{0,1}→侧0、b=2→侧1、b=3→侧2、b=4→侧3、
/// 其余→侧0。仅侧 0/1 有效；侧 2/3 在 on_wall_split 走 unreachable panic
/// （0x142ac8d80 "internal error: entered unreachable code"）。
#[inline]
fn side_from_state(b: u8) -> u8 {
    if 2 < b.wrapping_sub(2) {
        0
    } else {
        b.wrapping_sub(1)
    }
}

/// 该侧存储缺失哨兵：原生 `DecoratorStateWithStorage.storage_disc == 3`
/// （fresh `restore_decorators` 0x14155c27d / 0x14155c808 / 0x14155c814）。
const MISSING_STORAGE_DISC: u64 = 3;

/// `DecoratorStorageAddr` → 原生 `(disc, payload)` 两字。原生存放地址为
/// `{disc: u64 @+0x00, payload: u64 @+0x08}`（0=Wall/1=Terrain/2=Stair）；
/// Wall 的 payload = WallId，Terrain/Stair 的 payload 在原生 diff map 键中
/// 原样拷贝（语义不参与比较）。Rust 非墙变体无 payload，写 0（登记）。
#[inline]
fn storage_addr_parts(addr: DecoratorStorageAddr) -> (u64, u64) {
    match addr {
        DecoratorStorageAddr::Wall(wall) => (0, wall.0),
        DecoratorStorageAddr::Terrain => (1, 0),
        DecoratorStorageAddr::Stair => (2, 0),
    }
}

/// 装饰碰撞体移除端口（walls 侧对 collision 域的最小接口）。
///
/// 原生 `restore_decorators` 末段逐输出条目调用
/// `RaycastWorld::remove_collider<DecoratorId>` **两次**（0x14155cfb1 /
/// 0x14155cff6 / 0x14155d07f / 0x14155d0c4），分别作用于相邻的两张碰撞
/// map（`[archivist+0x00]` 与 `[archivist+0x30]`，即 collision_a / collision_b）。
/// 每次命中（返回句柄标志 != 0x11）后调用方对返回句柄内的
/// `Arc<RaycastMeshData>` 递减并在计数归零时 `drop_slow`。
///
/// Rust 侧 `collision::world::remove_collider` 在移除 map 条目后于内部
/// drop `ColliderMeshInstance`（含该 Arc），故“命中即递减”语义已包含在一次调用中；
/// 本端口把原生两次调用聚合成一次（顺序/two-map 语义不变，登记表面差异）。
pub trait DecoratorColliderRemover {
    /// 从全部装饰碰撞 map 移除该 id 的 collider（原生两次 remove_collider）。
    fn remove_decorator_collider(&mut self, id: DecoratorId);
}

/// 生产实现：collision 已落地的 `remove_collider` + 两张装饰碰撞 map。
///
/// 映射原生 `restore_decorators` 的实参：world = 原始第 5 参
/// （`in_stack_00000028` 解引用），collision_a/b = 第 6 参所指结构的
/// +0x00/+0x30 两张 map（`HashMap<DecoratorId, ColliderMeshInstance>`）。
#[cfg(feature = "native-physics")]
pub struct DecoratorColliderMaps<'a> {
    /// collision 子世界（`remove_collider` 的 `world` 实参）。
    pub world: &'a mut crate::systems::collision::world::RaycastWorld,
    /// collision_a（原生 archivist +0x00）。
    pub collision_a:
        &'a mut HashMap<DecoratorId, crate::systems::collision::world::ColliderMeshInstance>,
    /// collision_b（原生 archivist +0x30）。
    pub collision_b:
        &'a mut HashMap<DecoratorId, crate::systems::collision::world::ColliderMeshInstance>,
}

#[cfg(feature = "native-physics")]
impl DecoratorColliderRemover for DecoratorColliderMaps<'_> {
    fn remove_decorator_collider(&mut self, id: DecoratorId) {
        // 原生序：先 map1（+0x00）后 map2（+0x30），两次均命中才各递减一次 Arc。
        let _ = crate::systems::collision::world::remove_collider(
            self.world,
            self.collision_a,
            &id,
        );
        let _ = crate::systems::collision::world::remove_collider(
            self.world,
            self.collision_b,
            &id,
        );
    }
}

/// 缺失侧状态：`storage_disc = 3`，payload/state 原生未初始化（Rust 置 0）。
#[inline]
fn missing_state_with_storage() -> DecoratorStateWithStorage {
    DecoratorStateWithStorage {
        storage_disc: MISSING_STORAGE_DISC,
        storage_payload: 0,
        state: DecoratorHistoryState::default(),
    }
}

/// B DecoratorStorage：装饰权威存储资源（纯数据，无 bevy Entity）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct DecoratorStorage {
    /// +0x00：分墙桶 HashMap<WallId, Decorators>。
    pub per_wall: HashMap<WallId, Decorators>,
    /// +0x30：removed/目的地注册表（fresh 0x140a1f590 O：`HashMap<DecoratorStorageAddr,
    /// HashSet<DecoratorId>>`——每个目标地址待恢复的 removed id 集；on_wall_split
    /// 迁移经它登记，get_or_insert_default 新建桶时取走种子 removed_ids；
    /// restore_snapshot 前置断言 `removed.is_empty()`；iter_removed_storages 读键）。
    pub removed: HashMap<DecoratorStorageAddr, HashSet<u32>>,
    /// +0x60：d2s 索引（DecoratorId → 地址）。
    pub decorator_to_storage: DecoratorToStorageMap,
    /// +0x90：地形内联桶（懒初始化）。
    pub terrain: Option<Decorators>,
    /// +0x120：楼梯内联桶（懒初始化）。
    pub stairs: Option<Decorators>,
    /// +0x1b0：不可变标志（置位后拒绝写）。
    pub immutable: bool,
}

impl DecoratorStorage {
    /// 0x140a1c2c0（O）：目的 → 装饰集合。首段 per_wall 空门（+0x18==0 →
    /// None）；地形/楼梯分支以首 qword（+0x90/+0x120）判存在。
    pub fn get(&self, addr: DecoratorStorageAddr) -> Option<&Decorators> {
        match addr {
            DecoratorStorageAddr::Wall(wall) => self.per_wall.get(&wall),
            DecoratorStorageAddr::Terrain => self.terrain.as_ref(),
            DecoratorStorageAddr::Stair => self.stairs.as_ref(),
        }
    }

    /// 0x140a1cde0（O）：可变版；immutable 门控 panic "assertion failed:
    /// !self.immutable"（0x142ac9098）。
    pub fn get_mut(&mut self, addr: DecoratorStorageAddr) -> Option<&mut Decorators> {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        match addr {
            DecoratorStorageAddr::Wall(wall) => self.per_wall.get_mut(&wall),
            DecoratorStorageAddr::Terrain => self.terrain.as_mut(),
            DecoratorStorageAddr::Stair => self.stairs.as_mut(),
        }
    }

    /// 0x140a1c2b0（O）：清 immutable 标志（可写）。
    pub fn mark_mutable(&mut self) {
        self.immutable = false;
    }

    /// 行 542（内联）：置 immutable 标志（只读）。
    pub fn mark_immutable(&mut self) {
        self.immutable = true;
    }

    /// 0x140a1c3e0（O）：DecoratorId → 地址（d2s 索引查询）。
    pub fn lookup_storage(&self, id: DecoratorId) -> Option<DecoratorStorageAddr> {
        self.decorator_to_storage.get(id)
    }

    /// 0x140a1c510（O）：Option 版（与 lookup_storage 一致）。
    pub fn try_lookup_storage(&self, id: DecoratorId) -> Option<DecoratorStorageAddr> {
        self.lookup_storage(id)
    }

    /// 0x140a1c5f0（O）：DecoratorId → DecoratorInfo（双层：索引→桶→内嵌 map）。
    pub fn decorator_info(&self, id: DecoratorId) -> Option<&DecoratorInfo> {
        let addr = self.lookup_storage(id)?;
        self.get(addr)?.get(id)
    }

    /// 0x140a1c800（O）：可变版。
    pub fn decorator_info_mut(&mut self, id: DecoratorId) -> Option<&mut DecoratorInfo> {
        let addr = self.lookup_storage(id)?;
        self.get_mut(addr)?.get_mut(id)
    }

    /// 0x140a1c930（O）：双层查表（索引 24B 条目 → Decorators → 内嵌 map
    /// 0x50B 条目）。与 decorator_info 语义一致，独立反编译证实。
    pub fn lookup_decorator_info(&self, id: DecoratorId) -> Option<&DecoratorInfo> {
        self.decorator_info(id)
    }

    /// 0x140a1cbf0（I）：可变双层查表。
    pub fn lookup_decorator_info_mut(&mut self, id: DecoratorId) -> Option<&mut DecoratorInfo> {
        self.decorator_info_mut(id)
    }

    /// 0x140a1cf30（O，21KB）：取或插默认装饰集合。immutable 断言；墙走 per_wall
    /// 探针（0x98 步长），terrain/stair 走内联懒初始化（+0x30 注册表探测 + 写
    /// +0x90/+0x120）。**仅新建桶时取走 +0x30 注册表中该地址待恢复的 removed ids**
    /// 作为桶的 removed_ids（shared building modules restore_one 观察 + fresh
    /// 0x140a1f590 rustc_entry 结构；取/留语义 I 待取证）。
    pub fn get_or_insert_default(&mut self, addr: DecoratorStorageAddr) -> &mut Decorators {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        match addr {
            DecoratorStorageAddr::Wall(wall) => {
                if !self.per_wall.contains_key(&wall) {
                    let pending = self.removed.remove(&addr).unwrap_or_default();
                    self.per_wall.insert(
                        wall,
                        Decorators {
                            removed_ids: pending,
                            ..Decorators::default()
                        },
                    );
                }
                self.per_wall.get_mut(&wall).unwrap()
            }
            DecoratorStorageAddr::Terrain => {
                if self.terrain.is_none() {
                    let pending = self.removed.remove(&addr).unwrap_or_default();
                    self.terrain = Some(Decorators {
                        removed_ids: pending,
                        ..Decorators::default()
                    });
                }
                self.terrain.as_mut().unwrap()
            }
            DecoratorStorageAddr::Stair => {
                if self.stairs.is_none() {
                    let pending = self.removed.remove(&addr).unwrap_or_default();
                    self.stairs = Some(Decorators {
                        removed_ids: pending,
                        ..Decorators::default()
                    });
                }
                self.stairs.as_mut().unwrap()
            }
        }
    }

    /// 0x140b2c4b0 消费面：`get_or_insert_default(addr)` +
    /// `Decorators::tracked_clone_from`（0x141256e40/0x1412573b0）原子化，
    /// 同时把 d2s 索引重指向 `addr`。两次借用按字段拆分（per_wall/terrain/stairs
    /// 与 decorator_to_storage 互不相交）。
    pub fn tracked_clone_into(&mut self, addr: DecoratorStorageAddr, source: &Decorators) {
        self.get_or_insert_default(addr);
        let target = match addr {
            DecoratorStorageAddr::Wall(wall) => self.per_wall.get_mut(&wall).unwrap(),
            DecoratorStorageAddr::Terrain => self.terrain.as_mut().unwrap(),
            DecoratorStorageAddr::Stair => self.stairs.as_mut().unwrap(),
        };
        target.tracked_clone_from(source, &mut self.decorator_to_storage, addr);
    }

    /// 0x140a1dba0：移除桶并登记其原 removed 集合；随后逐活记录先登记
    /// ID 再删除 d2s，索引缺失时立即 panic，不处理剩余活记录。
    pub fn remove_storage(&mut self, addr: DecoratorStorageAddr) -> Option<Decorators> {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let removed = match addr {
            DecoratorStorageAddr::Wall(wall) => self.per_wall.remove(&wall),
            DecoratorStorageAddr::Terrain => self.terrain.take(),
            DecoratorStorageAddr::Stair => self.stairs.take(),
        };
        if let Some(decos) = &removed {
            let pending = self.removed.entry(addr).or_default();
            pending.extend(decos.removed_ids.iter().copied());
            for id in decos.decorators.keys() {
                pending.insert(*id);
                self.decorator_to_storage.remove(DecoratorId(*id))
                    .expect("assertion failed: self.map.remove(&id).is_some()");
            }
        }
        removed
    }

    /// 0x140a1e190（O）：墙装饰集合访问（DecoratorArchivist::all_storages 闭包）。
    pub fn wall_decorators(&self, wall: WallId) -> Option<&Decorators> {
        self.per_wall.get(&wall)
    }

    /// 0x140a1e300（O）：墙装饰 id 迭代（墙集合访问）。
    pub fn wall_decorator_ids(&self, wall: WallId) -> impl Iterator<Item = u32> + '_ {
        self.per_wall
            .get(&wall)
            .into_iter()
            .flat_map(|d| d.decorators.keys().copied())
    }

    /// 0x140a1e420（O）：墙装饰集合可变访问。
    pub fn wall_decorators_mut(&mut self, wall: WallId) -> Option<&mut Decorators> {
        self.per_wall.get_mut(&wall)
    }

    /// 0x140a1e280（O）：地形装饰迭代（{tag=1, ptr=self+0x90}）。
    pub fn iter_terrain_decorators(&self) -> impl Iterator<Item = &Decorators> + '_ {
        self.terrain.iter()
    }

    /// 0x140a1e2c0（O）：楼梯装饰迭代（{tag=1, ptr=self+0x120}）。
    pub fn iter_stair_decorators(&self) -> impl Iterator<Item = &Decorators> + '_ {
        self.stairs.iter()
    }

    /// 0x140a1b100（O，与 Decorators::iter_removed_ids 同址）：墙迭代。
    pub fn iter_walls(&self) -> impl Iterator<Item = (WallId, &Decorators)> + '_ {
        self.per_wall.iter().map(|(w, d)| (*w, d))
    }

    /// 0x140a1e540（O）：墙可变迭代。
    pub fn iter_walls_mut(&mut self) -> impl Iterator<Item = (WallId, &mut Decorators)> + '_ {
        self.per_wall.iter_mut().map(|(w, d)| (*w, d))
    }

    /// 0x140a1e5b0（O）：全存储迭代（per_wall 集合@+0x0）。
    pub fn iter(&self) -> impl Iterator<Item = (WallId, &Decorators)> + '_ {
        self.iter_walls()
    }

    /// 0x140a1e630（O）：全存储可变迭代。
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (WallId, &mut Decorators)> + '_ {
        self.iter_walls_mut()
    }

    /// 0x140a1e6e0（O）：removed 存储迭代（+0x30 注册表）。
    pub fn iter_removed_storages(&self) -> impl Iterator<Item = DecoratorStorageAddr> + '_ {
        self.removed.keys().copied()
    }

    /// 归档当前装饰（`archive_current_decorators_in_storage` 0x14123b5c0 的 O 读
    /// 路径）：逐墙拷贝 Decorators（0x90B memmove）+ 内嵌 map 条目到稳定快照。
    /// 供 decorator_archivist 归档消费；archivist 侧 merge_with（0x14155a950）
    /// 重插入接线属 shared building modules 归档类型边界（登记）。
    pub fn archive_current_decorators(
        &self,
    ) -> Vec<(DecoratorStorageAddr, Vec<(u32, DecoratorInfo)>)> {
        let mut out = Vec::new();
        for (wall, decorators) in &self.per_wall {
            let items = decorators
                .decorators
                .iter()
                .map(|(id, info)| (*id, *info))
                .collect();
            out.push((DecoratorStorageAddr::Wall(*wall), items));
        }
        if let Some(d) = &self.terrain {
            let items = d.decorators.iter().map(|(id, info)| (*id, *info)).collect();
            out.push((DecoratorStorageAddr::Terrain, items));
        }
        if let Some(d) = &self.stairs {
            let items = d.decorators.iter().map(|(id, info)| (*id, *info)).collect();
            out.push((DecoratorStorageAddr::Stair, items));
        }
        out
    }

    /// B 0x140a1e720 (791 B): end-of-frame tracking reset. Authoritative
    /// records, bucket owners and the decorator-to-storage index survive.
    /// The bucket's similarly named method is the separate destructive
    /// `Decorators::clear` adapter and must not be called from this consumer.
    pub fn clear_modification_tracking(&mut self) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        for bucket in self
            .per_wall
            .values_mut()
            .chain(self.terrain.iter_mut())
            .chain(self.stairs.iter_mut())
        {
            bucket.modified_ids.clear();
            bucket.removed_ids.clear();
        }
        self.removed.clear();
    }

    /// 0x140a1ea40（O，21KB）：墙合并迁移。fresh（2026-10-17）：双源 `remove_entry`
    /// 后，**仅当两源都存在才调 merge_with**（d2s 重索引内置于 merge_with）；单源路径
    /// **逐装饰 d2s remove（miss==3 panic）+ d2s insert(merged)**；随后
    /// `per_wall.insert(merged, 源)`。Rust 此前恒建 target=default、分离 d2s 时机，
    /// 且**重复 id 跨源时第二个源的 assert_exists 会 panic**（原 merge_with 对冲突
    /// 走 removed 去重，不 panic）。
    pub fn on_walls_merged(&mut self, a: WallId, b: WallId, merged: WallId) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let src_a = self.per_wall.remove(&a);
        let src_b = self.per_wall.remove(&b);
        let merged_addr = DecoratorStorageAddr::Wall(merged);
        let target = match (src_a, src_b) {
            (None, None) => return,
            (Some(d), None) | (None, Some(d)) => {
                Self::reindex_to_storage(self, &d, merged_addr);
                d
            }
            (Some(mut target), Some(source)) => {
                target.merge_with(source, &mut self.decorator_to_storage, merged_addr);
                target
            }
        };
        self.per_wall.insert(merged, target);
    }

    /// 单源合并路径（on_walls_merged 0x140a1ea40 内联）：逐装饰 d2s remove
    /// （miss==3 panic）+ d2s insert(merged)。源桶本身原样迁入合并墙。
    fn reindex_to_storage(
        storage: &mut DecoratorStorage,
        source: &Decorators,
        addr: DecoratorStorageAddr,
    ) {
        for id in source.decorators.keys() {
            storage
                .decorator_to_storage
                .remove(DecoratorId(*id))
                .expect("assertion failed: d2s remove in on_walls_merged: no such decorator");
            storage.decorator_to_storage.insert(DecoratorId(*id), addr);
        }
    }

    /// 0x140a1f550（I）：墙移除。per_wall 条目删除 + d2s 清理。
    pub fn on_wall_removed(&mut self, wall: WallId) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let removed = self.per_wall.remove(&wall);
        if let Some(decos) = &removed {
            for id in decos.decorators.keys() {
                self.decorator_to_storage.remove(DecoratorId(*id));
            }
        }
    }

    /// 0x140a1f590（O，46KB）：墙分裂迁移。fresh（2026-10-17）+ 反汇编定案：
    /// 参数 = (wall, children: &[(WallId, low, high)], curve: &Curve2)——子段含
    /// **u 区间 {low, high}**。消费owned map的真实next Option；d2s remove（miss==3
    /// panic）；侧判定 = **判别字节 DecoratorInfo+0x22**（= `dst_tag`，非 +0x47 state；
    /// 公式 `(b-2)<=2 ? b-1 : 0`，[`side_from_state`]）。
    /// - 侧 0：`Curve::get_approx_u_from_pos<Vec2>(point)` + clamp[0,1] +
    ///   {low,high} 对数组**包含搜索定子墙**（point = 高 32 位 dst_payload/u18）；
    ///   **命中 → 直接回插子墙桶**（`per_wall[child].insert`），未命中才落注册表。
    /// - 侧 1：`rustc_entry` + 注册表 insert（键 = **原墙地址** Wall(wall)）。
    /// - 侧 ≥2 → panic（unreachable ✓）。
    /// Rust 此前删除了侧 0 的 u 区间包含搜索与直接回插，把侧 0/侧 1 合并为同一
    /// removed-注册表路径（`children[side]` 直接下标替代了 u 区间归属判定）。
    pub fn on_wall_split(&mut self, wall: WallId, curve: &Curve2, children: &[(WallId, f32, f32)]) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let source = match self.per_wall.remove(&wall) {
            Some(d) => d,
            None => return,
        };
        // 原生首块：源已 removed_ids 并入 +0x30 注册表（原墙地址键）。
        if !source.removed_ids.is_empty() {
            self.removed
                .entry(DecoratorStorageAddr::Wall(wall))
                .or_default()
                .extend(source.removed_ids.iter().copied());
        }
        for (id, info) in source.decorators {
            // d2s 移除（fresh remove_entry miss==3 → panic）。
            self.decorator_to_storage
                .remove(DecoratorId(id))
                .expect("assertion failed: self.decorator_to_storage.map.remove(&decorator_id).is_some()");
            // 侧判定 = 判别字节 @+0x22（dst_tag；fresh 读 RBP+0x52）。
            let side = side_from_state(info.dst_tag());
            match side {
                0 => {
                    // u 区间包含搜索定子墙（fresh get_approx_u_from_pos + clamp[0,1]
                    // + {low,high} 对数组；命中直接回插子墙桶）。
                    let point = Vec2::new(
                        f32::from_bits((info.dst_payload >> 32) as u32),
                        f32::from_bits((info.u18 >> 32) as u32),
                    );
                    let u = curve.get_approx_u_from_pos(point);
                    let u = if 0.0 <= u { u } else { 0.0 };
                    let u = if u <= 1.0 { u } else { 1.0 };
                    if let Some((child, _low, _high)) = children
                        .iter()
                        .find(|(_, low, high)| *low <= u && u <= *high)
                    {
                        // 命中 → 直接拷回子墙桶（Decorators::insert<DecoMoveToStorage>
                        // 含内嵌 map + d2s 重注册）。
                        let child_decorators =
                            self.get_or_insert_default(DecoratorStorageAddr::Wall(*child));
                        let old = child_decorators.insert(DecoratorId(id), info);
                        if old.is_none() {
                            self.decorator_to_storage
                                .insert(DecoratorId(id), DecoratorStorageAddr::Wall(*child));
                        } else {
                            self.decorator_to_storage
                                .update(DecoratorId(id), DecoratorStorageAddr::Wall(*child));
                        }
                    } else {
                        // 未命中 → 落 +0x30 注册表（原墙地址键）。
                        self.removed
                            .entry(DecoratorStorageAddr::Wall(wall))
                            .or_default()
                            .insert(id);
                    }
                }
                1 => {
                    // 侧 1 → 注册表（原墙地址键）。
                    self.removed
                        .entry(DecoratorStorageAddr::Wall(wall))
                        .or_default()
                        .insert(id);
                }
                _ => unreachable!("internal error: entered unreachable code"),
            }
        }
    }

    /// 0x140a20670（O）：应用装饰移除。immutable 门控 → 目标桶（get_mut，缺失
    /// panic "assertion failed: self.map.remove(&id).is_some()" 0x142ac9068）→
    /// modified_ids 移除 + removed_ids 标记 + 内嵌 map 移除（未中 fmt panic
    /// "apply_decorator_remove: Trying to remove a decorator that doesn't exist: "
    /// 0x142ac9430）→ **d2s 移除（fresh remove_entry miss==3 → panic，行 53/66）**
    /// → 输出复刻移除的 DecoratorInfo。
    pub fn apply_decorator_remove(
        &mut self,
        addr: DecoratorStorageAddr,
        decorator_id: DecoratorId,
    ) -> DecoratorInfo {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let decorators = self
            .get_mut(addr)
            .unwrap_or_else(|| panic!("assertion failed: self.map.remove(&id).is_some()"));
        let info = decorators
            .decorators
            .remove(&decorator_id.0)
            .unwrap_or_else(|| {
                panic!(
                    "apply_decorator_remove: Trying to remove a decorator that doesn't exist: {}",
                    decorator_id
                )
            });
        decorators.modified_ids.remove(&decorator_id.0);
        decorators.removed_ids.insert(decorator_id.0);
        // fresh 0x140a20670：d2s 移除 miss==3 → panic（非忽略）。
        self.decorator_to_storage
            .remove(decorator_id)
            .expect("assertion failed: self.decorator_to_storage.map.remove(&id).is_some()");
        info
    }

    /// 0x140a20890（O，apply_decorator_op 变更分发）：变更应用。fresh 反编译
    /// （2026-10-17，disasm 0x140a20890）：
    /// - **目标特殊**（原生 op blob +0x10 == 3）：跳过 remove/reinsert，仅
    ///   `get_mut` + 内嵌 map 逐字段更新。
    /// - **正常路径**：`apply_decorator_remove`（移除旧条目，d2s 注销）+
    ///   `get_or_insert_default` + `Decorators::insert<DecoMoveToStorage>`（重插，
    ///   含 d2s 指向 addr）+ `get_mut` + `modified_ids.insert` + **内嵌 map
    ///   逐字段更新**（DecoratorExtraInfo/rank/type 判别跳表，非法 → panic）。
    /// Rust 此前仅 `get_mut → insert+modified` 简化（REBUILD-REMAINDER §4.1 登记）
    /// ——按 fresh 控制流重建。
    ///
    /// 注意：Rust `DecoratorStorageAddr` 判别 0/1/2（Wall/Terrain/Stair），
    /// 原生特殊目标判别 3 在 Rust 枚举无对应变体（仅正常路径可达）——登记判等-注意。
    pub fn apply_decorator_change(
        &mut self,
        addr: DecoratorStorageAddr,
        decorator_id: DecoratorId,
        info: &DecoratorInfo,
    ) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        // 正常路径：remove + get_or_insert_default + insert<DecoMoveToStorage>。
        self.apply_decorator_remove(addr, decorator_id);
        let decorators = self.get_or_insert_default(addr);
        decorators.insert(decorator_id, *info);
        self.decorator_to_storage.update(decorator_id, addr);
        // get_mut 目标桶（原生 extraout_RAX==0 → unwrap_failed panic）。
        let decorators = self.get_mut(addr).unwrap_or_else(|| {
            panic!("apply_decorator_change: target bucket missing");
        });
        decorators.modified_ids.insert(decorator_id.0);
        // 内嵌 map 逐字段更新（fresh 判别跳表 + panic）。
        if let Some(entry) = decorators.decorators.get_mut(&decorator_id.0) {
            Self::apply_decorator_change_field_update(entry, info);
        }
    }

    /// 内嵌 map 条目逐字段更新（fresh 0x140a20890 判别跳表）。
    /// 写面（条目 = 内嵌 map 值，offset 相对 DecoratorInfo 尾 +0x48）：
    /// decorator_id@+0x2c（[+0x48-0x1c]）、u30 低 dword@+0x30（[-0x18]）、
    /// u30 高 dword@+0x34（[-0x14]）、u40 字节1@+0x41（[-0x7]，判别）、
    /// rank@+0x44（[-0x4]）、u45@+0x45（[-0x3]）、ty@+0x46（[-0x2]）。
    /// 判别源（op blob → info）：`u30 低 32 位==1` → rank 判别 +
    /// **u30 低 dword = info.u30 高 dword**（MOVD [entry+0x30] 源 [RSI+0x54]）；
    /// `u38 & 1` → type 判别 + **u30 高 dword = info.u3c**（MOVD [entry+0x34]
    /// 源 [RSI+0x5c]）。判别非法 → panic（原生 DecoratorExtraInfo fmt panic）。
    ///
    /// 尾字段 +0x60..+0x65（rank 门/新 rank/id 门/id 增量/末判别/ty 值）为 op 专有，
    /// Rust DecoratorInfo 未承载（U），本实现以 u40/u45 近似取源。
    #[allow(clippy::cast_lossless)]
    fn apply_decorator_change_field_update(entry: &mut DecoratorInfo, info: &DecoratorInfo) {
        // 尾字段 blob[0x60]==1 → rank 覆写（U 近似：info.u40 低字节）。
        if info.u40 & 0xff == 1 {
            entry.rank = ((info.u40 >> 8) & 0xff) as u8;
        }
        // 尾字段 blob[0x65]!=0x40 → ty 覆写（U 近似：info.u45 != 0x40）。
        if info.u45 != 0x40 {
            entry.ty = info.u45;
        }
        // 尾字段 blob[0x62]==1 → decorator_id += signext(blob[0x63]) + u45 覆写
        // （demo ? 0 : 5；`_DEV_OVERRIDE_IS_DEMO_MODE` 0x143634d90 进程布尔，
        // 常规非 demo 运行取 5）。
        if (info.u40 >> 16) & 0xff == 1 {
            entry.decorator_id = entry
                .decorator_id
                .wrapping_add(((info.u40 >> 24) & 0xff) as i8 as u32);
            const DEV_OVERRIDE_IS_DEMO_MODE: bool = false;
            entry.u45 = if DEV_OVERRIDE_IS_DEMO_MODE { 0 } else { 5 };
        }
        // info.u30 低 32 位 == 1 → rank 判别：
        // `min((entry.u40 字节1 - 2) & 0xff, 6) | 4 != 6 → panic`；通过后
        // entry.u30 低 32 位 = info.u30 高 32 位（机器 MOVD [entry+0x30]，源 +0x54）。
        if info.u30 as u32 == 1 {
            let disc = (((entry.u40 >> 8) & 0xff) as u8).wrapping_sub(2).min(6) | 4;
            if disc != 6 {
                panic!("apply_decorator_change: invalid DecoratorExtraInfo rank");
            }
            entry.u30 = (entry.u30 & 0xffff_ffff_0000_0000) | u64::from(info.u30 >> 32);
        }
        // info.u38 & 1 → type 判别：`(entry.u40 字节1 | 0xf8) >= 0xfa → panic`；
        // 通过后 entry.u30 高 32 位 = info.u3c（机器 MOVD [entry+0x34]，源 +0x5c）。
        if info.u38 & 1 != 0 {
            let disc = ((entry.u40 >> 8) & 0xff) as u8 | 0xf8;
            if disc >= 0xfa {
                panic!("apply_decorator_change: invalid DecoratorExtraInfo type");
            }
            entry.u30 = (entry.u30 & 0xffff_ffff) | (u64::from(info.u3c) << 32);
        }
        // info.rank != 2 → 校验 entry.u40 字节1 == 5 后 u30 最低字节 = rank。
        if info.rank != 2 {
            if (entry.u40 >> 8) & 0xff != 5 {
                panic!("apply_decorator_change: invalid DecoratorExtraInfo rank");
            }
            entry.u30 = (entry.u30 & 0xffff_ffff_ff00_0000) | u64::from(info.rank);
        }
    }

    /// 原140a20c70丢弃inner旧Option；本现代兼容返回值保留新Info原字节。
    /// 桶先removed撤销/modified/map写入，再按真实旧Option更新唯一d2s。
    pub fn apply_decorator_place(
        &mut self,
        addr: DecoratorStorageAddr,
        decorator_id: DecoratorId,
        info: &DecoratorInfo,
    ) -> DecoratorInfo {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let old = self.get_or_insert_default(addr).insert(decorator_id, *info);
        if old.is_none() {
            self.decorator_to_storage.insert(decorator_id, addr);
        } else {
            self.decorator_to_storage.update(decorator_id, addr);
        }
        *info
    }

    /// B take_snapshot, inlined into capture_save_data 0x1417a0430. The actual
    /// collector 0x1412fbcd0 visits walls, then optional terrain and stair buckets;
    /// 0x141559890/0x1412ff7b0 collect and sort real (DecoratorId, HistoryState).
    /// Mutability and mutation-tracking sets do not restrict a read-only capture.
    pub fn take_snapshot(&self) -> DecoratorStorageSnapshot {
        let capture = |bucket: &Decorators| {
            let mut states: Vec<_> = bucket.decorators.iter().map(|(&id, info)| {
                (DecoratorId(id), DecoratorHistoryState::from_info(info))
            }).collect();
            states.sort_by_key(|(id, _)| *id);
            DecoratorAddrSnapshot { states }
        };
        let mut states: Vec<_> = self.iter_walls().map(|(wall, bucket)| {
            (DecoratorStorageAddr::Wall(wall), capture(bucket))
        }).collect();
        if let Some(bucket) = &self.terrain {
            states.push((DecoratorStorageAddr::Terrain, capture(bucket)));
        }
        if let Some(bucket) = &self.stairs {
            states.push((DecoratorStorageAddr::Stair, capture(bucket)));
        }
        DecoratorStorageSnapshot { states }
    }

    /// Compatibility entry point. A runtime loader must use
    /// `restore_snapshot_with_id_gen` with its authoritative generator.
    pub fn restore_snapshot(&mut self, snapshot: &DecoratorStorageSnapshot) {
        self.restore_snapshot_with_id_gen(snapshot, &mut DecoratorIdGen::default());
    }

    /// B restore_snapshot 0x140a20d70: empty mutable destination, create each
    /// captured bucket even when empty, reserve ids, restore live state and d2s.
    pub fn restore_snapshot_with_id_gen(
        &mut self,
        snapshot: &DecoratorStorageSnapshot,
        id_gen: &mut DecoratorIdGen,
    ) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        assert!(self.per_wall.is_empty(), "assertion failed: per_wall.is_empty()");
        assert!(self.terrain.is_none(), "assertion failed: terrain.is_none()");
        assert!(self.stairs.is_none(), "assertion failed: stairs.is_none()");
        assert!(self.removed.is_empty(), "assertion failed: removed.is_empty()");
        assert!(
            self.decorator_to_storage.is_empty(),
            "assertion failed: decorator_to_storage.map.is_empty()"
        );
        for (addr, snapshot) in &snapshot.states {
            // Original get_or_insert_default precedes the bucket's len check.
            self.get_or_insert_default(*addr);
            for (id, state) in &snapshot.states {
                id_gen.reserve(*id);
                let decorators = self.get_mut(*addr).expect("created bucket");
                decorators.removed_ids.remove(&id.0);
                decorators.modified_ids.insert(id.0);
                let old = decorators.decorators.insert(id.0, state.to_info());
                if old.is_none() {
                    self.decorator_to_storage.insert(*id, *addr);
                } else {
                    self.decorator_to_storage.update(*id, *addr)
                        .expect("d2s update: no such decorator");
                }
            }
        }
    }

    /// `0x14155ba30` `DecoratorStorage::restore_decorators<closure_env_0>`（fresh
    /// 2026-10-17 复核；Ghidra 函数界 4438B / PDB code_size 6407B，反编译证据在
    /// `planning/impl-walls-restore-decorators/decomp/14155ba30-fresh.txt`）。
    ///
    /// 实参（PDB `void()` 占位；由调用点 `replay_pop_bucket` 0x140b38385 逆推）：
    /// self、`&HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>`、
    /// `reverse: bool`、`&mut DecoratorIdGen`、`&mut RaycastWorld`、
    /// `&mut (collision_a, collision_b)`、事件闭包环境（= `DeferredDecoratorOpEvents`）。
    ///
    /// 逐分支（地址见 `decomp/14155ba30-disasm.txt`）：
    /// 1. `immutable` 断言 panic（0x14155ba2b）。
    /// 2. **移除侧扫描**（0x14155bb9d..0x14155c2ae）：`get_mut(addr)` 为 None 则跳
    ///    过该地址；逐 id：`modified_ids.remove` + `removed_ids.insert` +
    ///    `decorators.remove`；旧条目 None → 跳过；Some(info) → `d2s.remove(id)`
    ///    （缺失 panic "assertion failed: self.map.remove(&id).is_some()"，串
    ///    0x142ac9068）→ `DecoratorHistoryState::from(info)` →
    ///    输出 `BeforeAfter{before:{addr, state}, after:{disc:3}}`。
    /// 3. **新增侧扫描**（0x14155c2ae..0x14155cee0）：新增侧 map 空则跳过该地址；
    ///    `get_or_insert_default(addr)`；逐 id：`DecoratorIdGen::reserve(id)` →
    ///    输出条目 `after={addr, state}`（已有条目则覆盖 after，缺则新建且两侧
    ///    判别=3）→ `removed_ids.remove` + `modified_ids.insert` + 桶 map
    ///    insert(info) → 旧条目 None 用 `d2s.insert`（冲突 panic），Some 用
    ///    `d2s.update`（缺失 panic "d2s update: no such ..."）。
    ///    info 由 `DecoratorHistoryState::to_info()` 重建，状态字节按 fresh
    ///    0x14155c968 写 0。
    /// 4. **碰撞体移除**（0x14155cee0..0x14155d0fa）：逐输出条目，before 判别 3
    ///    跳过；after 判别 3，或两侧地址不等（disc 不同，或 disc==0 时 payload
    ///    不同）→ 原生连续两次 `remove_collider<DecoratorId>`（本实现经
    ///    [`DecoratorColliderRemover`] 聚合为一次）。
    /// 5. **事件**（0x14155d0fa）：`send_state_diff_events(&out, events)` 一次。
    ///
    /// `reverse`（原生 R8）选择侧：false → 移除侧=before、新增侧=after（正向
    /// redo）；true → 对调（undo）。分支选择为 O；语义命名 I。
    pub fn restore_decorators<C: DecoratorColliderRemover>(
        &mut self,
        diff: &HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>,
        reverse: bool,
        id_gen: &mut DecoratorIdGen,
        colliders: &mut C,
        events: &DeferredDecoratorOpEvents,
    ) {
        let out = self.restore_decorators_state_diff(diff, reverse, id_gen);
        // 4. 碰撞体移除（0x14155cee0..0x14155d0fa）。
        for (id, entry) in &out {
            if entry.before.storage_disc == MISSING_STORAGE_DISC {
                continue;
            }
            let remove = if entry.after.storage_disc == MISSING_STORAGE_DISC {
                true
            } else if entry.before.storage_disc != entry.after.storage_disc {
                true
            } else if entry.before.storage_disc == 0 {
                entry.before.storage_payload != entry.after.storage_payload
            } else {
                false
            };
            if remove {
                colliders.remove_decorator_collider(*id);
            }
        }
        // 5. 事件（0x14155d13f）。
        send_state_diff_events(&out, events);
    }

    /// `restore_decorators` 步骤 2/3：构建
    /// `HashMap<DecoratorId, DecoratorBeforeAfterWithStorage>`（原生栈上 0x30B map，
    /// 输出给 collision 段与 `send_state_diff_events`）。
    fn restore_decorators_state_diff(
        &mut self,
        diff: &HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>,
        reverse: bool,
        id_gen: &mut DecoratorIdGen,
    ) -> HashMap<DecoratorId, DecoratorBeforeAfterWithStorage> {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let mut out: HashMap<DecoratorId, DecoratorBeforeAfterWithStorage> = HashMap::new();

        // ---- 步骤 2：移除侧扫描 ----
        for (addr, d) in diff {
            let remove_side = if reverse {
                &d.after.states
            } else {
                &d.before.states
            };
            if self.get(*addr).is_none() {
                continue;
            }
            let (disc, payload) = storage_addr_parts(*addr);
            for id in remove_side.keys() {
                let old = {
                    let bucket = self.get_mut(*addr).expect("bucket checked above");
                    bucket.modified_ids.remove(&id.0);
                    bucket.removed_ids.insert(id.0);
                    bucket.decorators.remove(&id.0)
                };
                let Some(old_info) = old else {
                    continue;
                };
                // impl$10::remove：`self.map.remove(&id).is_some()` 断言。
                if self.decorator_to_storage.remove(*id).is_none() {
                    panic!("assertion failed: self.map.remove(&id).is_some()");
                }
                let state = DecoratorHistoryState::from(&old_info);
                out.insert(
                    *id,
                    DecoratorBeforeAfterWithStorage {
                        before: DecoratorStateWithStorage {
                            storage_disc: disc,
                            storage_payload: payload,
                            state,
                        },
                        after: missing_state_with_storage(),
                    },
                );
            }
        }

        // ---- 步骤 3：新增侧扫描 ----
        for (addr, d) in diff {
            let add_side = if reverse {
                &d.before.states
            } else {
                &d.after.states
            };
            if add_side.is_empty() {
                continue;
            }
            let (disc, payload) = storage_addr_parts(*addr);
            for (id, state) in add_side {
                id_gen.reserve(*id);
                let after = DecoratorStateWithStorage {
                    storage_disc: disc,
                    storage_payload: payload,
                    state: *state,
                };
                match out.entry(*id) {
                    std::collections::hash_map::Entry::Occupied(mut occupied) => {
                        occupied.get_mut().after = after;
                    }
                    std::collections::hash_map::Entry::Vacant(vacant) => {
                        vacant.insert(DecoratorBeforeAfterWithStorage {
                            before: missing_state_with_storage(),
                            after,
                        });
                    }
                }
                let mut info = state.to_info();
                // fresh 0x14155c968：重建的 DecoratorInfo 状态字节 = 0。
                info.state = 0;
                let old = {
                    let bucket = self.get_or_insert_default(*addr);
                    bucket.removed_ids.remove(&id.0);
                    bucket.modified_ids.insert(id.0);
                    bucket.decorators.insert(id.0, info)
                };
                if old.is_none() {
                    // Decorators::insert 的 None 分支（d2s insert，冲突 panic）。
                    self.decorator_to_storage.insert(*id, *addr);
                } else if self.decorator_to_storage.update(*id, *addr).is_none() {
                    // impl$8::update 的 unwrap_or_else panic（0x142ac9018）。
                    panic!(
                        "d2s update: no such {} in crates/country-core/src/resources/walls/decorator_storage.rs",
                        id
                    );
                }
            }
        }

        out
    }

    /// 应用单地址历史差分（restore 方向）：把该地址桶恢复到 before 状态。
    ///
    /// - after 有而 before 无的 id → 桶移除 + d2s 移除 + removed_ids 加入；
    /// - before 有（无论 after）的 id → 桶 insert(`DecoratorHistoryState::to_info`)
    ///   + removed_ids 移除 + d2s upsert。
    pub fn apply_storage_diff(
        &mut self,
        addr: DecoratorStorageAddr,
        diff: &crate::resources::history::decorator_edit::DecoratorStorageHistoryDiff,
    ) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let before_ids: HashSet<u32> = diff.before.states.keys().map(|d| d.0).collect();
        let after_ids: HashSet<u32> = diff.after.states.keys().map(|d| d.0).collect();
        let mut d2s_removals: Vec<u32> = Vec::new();
        let mut d2s_upserts: Vec<u32> = Vec::new();
        {
            let bucket = self.get_or_insert_default(addr);
            for id in after_ids.difference(&before_ids) {
                if bucket.decorators.remove(id).is_some() {
                    bucket.removed_ids.insert(*id);
                    d2s_removals.push(*id);
                }
            }
            for (id, state) in &diff.before.states {
                let id_u32 = id.0;
                bucket.decorators.insert(id_u32, state.to_info());
                bucket.removed_ids.remove(&id_u32);
                d2s_upserts.push(id_u32);
            }
        }
        for id in d2s_removals {
            self.decorator_to_storage.remove(DecoratorId(id));
        }
        for id in d2s_upserts {
            match self.decorator_to_storage.map.get_mut(&id) {
                Some(slot) => *slot = addr,
                None => {
                    self.decorator_to_storage.map.insert(id, addr);
                }
            }
        }
    }

    /// 批量应用多地址历史差分（`replay_pop_bucket` 的 `restore_decorators` 消费面）。
    pub fn restore_decorators_diff(
        &mut self,
        diffs: &std::collections::HashMap<
            DecoratorStorageAddr,
            crate::resources::history::decorator_edit::DecoratorStorageHistoryDiff,
        >,
    ) {
        for (addr, diff) in diffs {
            self.apply_storage_diff(*addr, diff);
        }
    }

    /// 0x14003bb80（O）：空函数（ret，no-op；PDB 与 decorator_subtype::queue
    /// 同址同名）。
    pub fn verify_integrity(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(id: u32) -> DecoratorInfo {
        DecoratorInfo {
            dst_wall: 7,
            decorator_id: id,
            ..DecoratorInfo::default()
        }
    }

    #[test]
    fn get_or_insert_default_creates_wall_terrain_stair_buckets() {
        let mut storage = DecoratorStorage::default();
        let wall_bucket = storage.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(1)));
        wall_bucket.insert(DecoratorId(10), info(10));
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(DecoratorId(20), info(20));
        storage
            .get_or_insert_default(DecoratorStorageAddr::Stair)
            .insert(DecoratorId(30), info(30));

        assert_eq!(storage.per_wall.len(), 1);
        assert!(storage.terrain.is_some());
        assert!(storage.stairs.is_some());
        assert_eq!(storage.get(DecoratorStorageAddr::Wall(WallId(1))).unwrap().decorators.len(), 1);
    }

    #[test]
    fn get_or_insert_default_seeds_removed_ids_from_registry() {
        let mut storage = DecoratorStorage::default();
        // 预置 +0x30 注册表：地形待恢复 removed id 7（模拟 on_wall_split 迁移）。
        storage
            .removed
            .insert(DecoratorStorageAddr::Terrain, HashSet::from([7]));
        let terrain = storage.get_or_insert_default(DecoratorStorageAddr::Terrain);
        assert!(terrain.removed_ids.contains(&7));
        // 注册表条目被取走（take 语义）。
        assert!(!storage.removed.contains_key(&DecoratorStorageAddr::Terrain));
        assert!(storage.iter_removed_storages().next().is_none());
    }

    #[test]
    fn apply_place_and_remove_roundtrip() {
        let mut storage = DecoratorStorage::default();
        let placed = storage.apply_decorator_place(
            DecoratorStorageAddr::Wall(WallId(3)),
            DecoratorId(5),
            &info(5),
        );
        assert_eq!(placed.decorator_id, 5);
        assert_eq!(storage.lookup_storage(DecoratorId(5)), Some(DecoratorStorageAddr::Wall(WallId(3))));
        assert_eq!(storage.decorator_info(DecoratorId(5)).unwrap().dst_wall, 7);

        let removed = storage.apply_decorator_remove(DecoratorStorageAddr::Wall(WallId(3)), DecoratorId(5));
        assert_eq!(removed.decorator_id, 5);
        assert_eq!(storage.decorator_info(DecoratorId(5)), None);
        // removed_ids 已标记。
        let bucket = storage.get(DecoratorStorageAddr::Wall(WallId(3))).unwrap();
        assert!(bucket.removed_ids.contains(&5));
    }

    #[test]
    fn on_walls_merged_migrates_and_updates_index() {
        let mut storage = DecoratorStorage::default();
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(1), &info(1));
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(2)), DecoratorId(2), &info(2));
        storage.on_walls_merged(WallId(1), WallId(2), WallId(9));

        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(1))).is_none());
        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(2))).is_none());
        let merged = storage.get(DecoratorStorageAddr::Wall(WallId(9))).unwrap();
        assert_eq!(merged.decorators.len(), 2);
        assert_eq!(
            storage.lookup_storage(DecoratorId(1)),
            Some(DecoratorStorageAddr::Wall(WallId(9)))
        );
    }

    #[test]
    fn side_from_state_formula() {
        // E2 公式 (b-2)<=2 ? b-1 : 0（u8 回绕）。
        assert_eq!(side_from_state(0), 0);
        assert_eq!(side_from_state(1), 0);
        assert_eq!(side_from_state(2), 1);
        assert_eq!(side_from_state(3), 2);
        assert_eq!(side_from_state(4), 3);
        assert_eq!(side_from_state(5), 0);
        assert_eq!(side_from_state(0xff), 0);
        assert_eq!(side_from_state(0x84), 0);
    }

    #[test]
    fn on_wall_split_side0_direct_insert_and_side1_registry() {
        let mut storage = DecoratorStorage::default();
        let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::new(10.0, 0.0)], false).unwrap();
        // decorator 1：判别字节 dst_tag=0（+0x22）→ 侧 0；point=(5,0) → u=0.5 → 命中子墙 11。
        let mut info1 = info(1);
        info1.dst_payload = (5.0f32.to_bits() as u64) << 32;
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(1), &info1);
        // decorator 2：判别字节 dst_tag=2（u20 高 16 位）→ 侧 1 → removed 注册表。
        let mut info2 = info(2);
        info2.u20 = 2 << 16;
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(2), &info2);
        storage.on_wall_split(
            WallId(1),
            &curve,
            &[(WallId(11), 0.4, 0.6), (WallId(12), 0.6, 1.0)],
        );

        // 原墙已删、d2s 已注销（fresh 0x140a1f590：remove_entry miss==3 → panic）。
        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(1))).is_none());
        // 侧 0 命中子墙 11：decorator 1 直接回插内嵌 map 并重注册 d2s → Wall(11)。
        assert_eq!(
            storage.lookup_storage(DecoratorId(1)),
            Some(DecoratorStorageAddr::Wall(WallId(11)))
        );
        let b11 = storage.get(DecoratorStorageAddr::Wall(WallId(11))).unwrap();
        assert!(b11.decorators.contains_key(&1));
        assert!(storage.removed.get(&DecoratorStorageAddr::Wall(WallId(11))).is_none());
        // 侧 1：decorator 2 落 removed 注册表（键 = 原墙地址），d2s 不重注册。
        assert_eq!(storage.lookup_storage(DecoratorId(2)), None);
        let reg = storage.removed.get(&DecoratorStorageAddr::Wall(WallId(1))).unwrap();
        assert!(reg.contains(&2));
        assert!(storage.iter_removed_storages().any(|a| a == DecoratorStorageAddr::Wall(WallId(1))));
    }

    #[test]
    fn on_wall_split_side0_miss_registers_removed() {
        let mut storage = DecoratorStorage::default();
        let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::new(10.0, 0.0)], false).unwrap();
        // point=(2,0) → u=0.2，不落在 [0.4,1.0] 区间 → 未命中 → 落 removed 注册表。
        let mut info = info(9);
        info.dst_payload = (2.0f32.to_bits() as u64) << 32;
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(9), &info);
        storage.on_wall_split(
            WallId(1),
            &curve,
            &[(WallId(11), 0.4, 0.6), (WallId(12), 0.6, 1.0)],
        );
        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(11))).is_none());
        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(12))).is_none());
        // 未命中：d2s 不重注册，仅落 removed 注册表。
        assert_eq!(storage.lookup_storage(DecoratorId(9)), None);
        let reg = storage.removed.get(&DecoratorStorageAddr::Wall(WallId(1))).unwrap();
        assert!(reg.contains(&9));
    }

    #[test]
    fn on_wall_split_transfers_all_legal_flags_without_repeating_the_iterator_niche() {
        let mut storage = DecoratorStorage::default();
        let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::new(10.0, 0.0)], false).unwrap();
        for (id, flag) in [(7, 0), (8, 1), (9, 0), (10, 1)] {
            let mut value = info(id); value.state = flag; value.u45 = 1 - flag;
            storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(id), &value);
        }
        storage.on_wall_split(WallId(1), &curve, &[(WallId(11), 0.0, 0.5), (WallId(12), 0.5, 1.0)]);
        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(1))).is_none());
        for (id, flag) in [(7, 0), (8, 1), (9, 0), (10, 1)] {
            assert_eq!(storage.lookup_storage(DecoratorId(id)), Some(DecoratorStorageAddr::Wall(WallId(11))));
            let value = storage.decorator_info(DecoratorId(id)).unwrap();
            assert_eq!((value.state, value.u45), (flag, 1 - flag));
        }
        assert!(storage.removed.is_empty());
    }

    #[test]
    fn on_wall_split_panics_unreachable_for_side_23() {
        let mut storage = DecoratorStorage::default();
        let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::new(10.0, 0.0)], false).unwrap();
        let mut info3 = info(3);
        info3.u20 = 3 << 16; // 判别字节 dst_tag=3 → 侧 2 → unreachable panic。
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(3), &info3);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            storage.on_wall_split(
                WallId(1),
                &curve,
                &[(WallId(11), 0.0, 0.5), (WallId(12), 0.5, 1.0)],
            );
        }))
        .is_err());
    }

    #[test]
    fn remove_storage_propagates_removed_and_live_ids_and_recreation_consumes_them() {
        let mut storage = DecoratorStorage::default();
        let addr = DecoratorStorageAddr::Wall(WallId(41));
        storage.apply_decorator_place(addr, DecoratorId(7), &info(7));
        storage.get_mut(addr).unwrap().removed_ids.insert(8);
        storage.remove_storage(addr).unwrap();
        assert_eq!(storage.lookup_storage(DecoratorId(7)), None);
        assert_eq!(storage.removed[&addr], HashSet::from([7, 8]));
        assert_eq!(storage.get_or_insert_default(addr).removed_ids, HashSet::from([7, 8]));
        assert!(!storage.removed.contains_key(&addr));
    }

    #[test]
    fn remove_storage_index_failure_preserves_only_visited_live_prefix() {
        let mut storage = DecoratorStorage::default();
        let addr = DecoratorStorageAddr::Wall(WallId(42));
        for id in [7, 8, 9] {
            storage.apply_decorator_place(addr, DecoratorId(id), &info(id));
        }
        storage.get_mut(addr).unwrap().removed_ids.insert(100);
        // Select the actual first iteration key, independent of HashMap seed.
        let first = *storage.get(addr).unwrap().decorators.keys().next().unwrap();
        storage.decorator_to_storage.remove(DecoratorId(first)).unwrap();
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            storage.remove_storage(addr);
        }));
        assert!(failed.is_err());
        assert!(storage.get(addr).is_none());
        assert_eq!(storage.removed[&addr], HashSet::from([100, first]));
        for id in [7, 8, 9].into_iter().filter(|id| *id != first) {
            assert_eq!(storage.lookup_storage(DecoratorId(id)), Some(addr));
        }
    }

    #[test]
    fn snapshot_collects_nonempty_buckets_and_retains_empty_owners() {
        let mut storage = DecoratorStorage::default();
        for (id, ty, style) in [(u32::MAX, 0x38, 6), (7, 0x3d, 5)] {
            let mut value = info(id);
            value.ty = ty;
            value.u40 = style << 8;
            storage.apply_decorator_place(
                DecoratorStorageAddr::Wall(WallId(4)), DecoratorId(id), &value,
            );
        }
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(8)));
        storage.get_or_insert_default(DecoratorStorageAddr::Terrain);
        storage.get_or_insert_default(DecoratorStorageAddr::Stair);
        storage.removed.insert(DecoratorStorageAddr::Wall(WallId(99)), HashSet::from([42]));
        storage.mark_immutable();
        let mut snapshot = storage.take_snapshot();
        snapshot.sort();
        assert_eq!(snapshot.states.len(), 4);
        assert_eq!(snapshot.states[0].0, DecoratorStorageAddr::Wall(WallId(4)));
        assert_eq!(snapshot.states[0].1.states.iter().map(|(id, _)| id.0).collect::<Vec<_>>(), vec![7, u32::MAX]);
        for (_, bucket) in &snapshot.states[1..] { assert!(bucket.states.is_empty()); }
        let source = storage.lookup_decorator_info(DecoratorId(7)).unwrap();
        assert_eq!(snapshot.states[0].1.states[0].1, DecoratorHistoryState::from_info(source));
        assert!(storage.immutable);
        assert!(storage.removed.contains_key(&DecoratorStorageAddr::Wall(WallId(99))));
    }

    #[test]
    fn snapshot_distinguishes_absent_and_empty_and_rejects_preview() {
        let storage = DecoratorStorage::default();
        assert!(storage.take_snapshot().states.is_empty());
        let mut preview = DecoratorStorage::default();
        let mut value = info(9);
        value.state = 1;
        preview.get_or_insert_default(DecoratorStorageAddr::Terrain).insert(DecoratorId(9), value);
        assert!(std::panic::catch_unwind(|| preview.take_snapshot()).is_err());
    }

    #[test]
    fn snapshot_restores_real_ids_buckets_and_generator_for_next_edit() {
        let mut source = DecoratorStorage::default();
        let mut value = info(90);
        value.ty = 0x38;
        value.u40 = 6 << 8;
        value.u30 = 0; // default window style must survive without inference.
        source.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(2)), DecoratorId(90), &value);
        source.get_or_insert_default(DecoratorStorageAddr::Terrain);
        source.get_or_insert_default(DecoratorStorageAddr::Stair);
        let snapshot = source.take_snapshot();
        let mut restored = DecoratorStorage::default();
        let mut generator = DecoratorIdGen::default();
        restored.restore_snapshot_with_id_gen(&snapshot, &mut generator);
        assert_eq!(generator.generate(), DecoratorId(91));
        assert_eq!(restored.lookup_storage(DecoratorId(90)), Some(DecoratorStorageAddr::Wall(WallId(2))));
        let actual = restored.lookup_decorator_info(DecoratorId(90)).unwrap();
        assert_eq!(actual.extra_style(), 6);
        assert_eq!(actual.state, 0);
        assert!(restored.terrain.as_ref().unwrap().decorators.is_empty());
        assert!(restored.stairs.as_ref().unwrap().decorators.is_empty());
        assert!(restored.get(DecoratorStorageAddr::Wall(WallId(2))).unwrap().modified_ids.contains(&90));
        let mut captured_again = restored.take_snapshot();
        let mut expected = snapshot;
        captured_again.sort(); expected.sort();
        assert_eq!(captured_again, expected);
    }

    #[test]
    fn storage_addr_accessors() {
        assert_eq!(
            DecoratorStorageAddr::Wall(WallId(4)).as_wall(),
            WallId(4)
        );
        assert_eq!(DecoratorStorageAddr::Terrain.try_as_wall(), None);
        assert_eq!(DecoratorStorageAddr::default(), DecoratorStorageAddr::Wall(WallId(0)));
    }

    #[test]
    fn immutable_blocks_mutation() {
        let mut storage = DecoratorStorage::default();
        storage.mark_immutable();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(1), &info(1));
        }))
        .is_err());
    }

    // ------------------------------------------------------------------
    // restore_decorators（0x14155ba30，fresh 2026-10-17）
    // ------------------------------------------------------------------

    use crate::resources::history::decorator_edit::{
        DecoratorDiffEvent, DecoratorStorageAddrHistoryState,
    };

    #[derive(Default)]
    struct MockColliderRemover {
        removed: Vec<DecoratorId>,
    }

    impl DecoratorColliderRemover for MockColliderRemover {
        fn remove_decorator_collider(&mut self, id: DecoratorId) {
            self.removed.push(id);
        }
    }

    fn addr_state(
        entries: &[(u32, &DecoratorInfo)],
    ) -> DecoratorStorageAddrHistoryState {
        let mut s = DecoratorStorageAddrHistoryState::default();
        for (id, info) in entries {
            s.states
                .insert(DecoratorId(*id), DecoratorHistoryState::from(*info));
        }
        s
    }

    fn diff_of(
        entries: &[(
            DecoratorStorageAddr,
            DecoratorStorageAddrHistoryState,
            DecoratorStorageAddrHistoryState,
        )],
    ) -> HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff> {
        entries
            .iter()
            .map(|(addr, before, after)| {
                (
                    *addr,
                    DecoratorStorageHistoryDiff {
                        before: before.clone(),
                        after: after.clone(),
                    },
                )
            })
            .collect()
    }

    fn drain_events(events: &DeferredDecoratorOpEvents) -> Vec<DecoratorDiffEvent> {
        let mut raw = Vec::new();
        events.unleash(&mut raw);
        raw.into_iter()
            .map(|r| DecoratorDiffEvent(r[..0x88].try_into().unwrap()))
            .collect()
    }

    /// 移除侧：modified_ids.remove + removed_ids.insert + d2s.remove + 移除事件；
    /// after 判别 3 → 碰撞体移除一次。
    #[test]
    fn restore_decorators_removes_d2s_bucket_and_marks_removed() {
        let addr = DecoratorStorageAddr::Wall(WallId(5));
        let mut storage = DecoratorStorage::default();
        let placed = info(1);
        storage.apply_decorator_place(addr, DecoratorId(1), &placed);

        let diff = diff_of(&[(
            addr,
            addr_state(&[(1, &placed)]),
            DecoratorStorageAddrHistoryState::default(),
        )]);
        let events = DeferredDecoratorOpEvents::default();
        let mut remover = MockColliderRemover::default();
        let mut id_gen = DecoratorIdGen::default();
        storage.restore_decorators(&diff, false, &mut id_gen, &mut remover, &events);

        let bucket = storage.get(addr).unwrap();
        assert!(!bucket.decorators.contains_key(&1));
        assert!(bucket.removed_ids.contains(&1));
        assert!(!bucket.modified_ids.contains(&1));
        assert_eq!(storage.lookup_storage(DecoratorId(1)), None);
        assert_eq!(remover.removed, vec![DecoratorId(1)]);

        let evs = drain_events(&events);
        assert_eq!(evs.len(), 1);
        assert_eq!(evs[0].kind(), 4, "移除事件种类 4");
        assert_eq!(evs[0].decorator_id(), 1);
    }

    /// 新增侧：id_gen.reserve + 桶插回（removed_ids.remove/modified_ids.insert）+
    /// d2s insert + 新增事件；before 判别 3 → 不做碰撞移除。
    #[test]
    fn restore_decorators_adds_bucket_and_inserts_d2s() {
        let addr = DecoratorStorageAddr::Wall(WallId(6));
        let source = info(2);
        let mut storage = DecoratorStorage::default();
        let diff = diff_of(&[(
            addr,
            DecoratorStorageAddrHistoryState::default(),
            addr_state(&[(2, &source)]),
        )]);
        let events = DeferredDecoratorOpEvents::default();
        let mut remover = MockColliderRemover::default();
        let mut id_gen = DecoratorIdGen::default();
        storage.restore_decorators(&diff, false, &mut id_gen, &mut remover, &events);

        let bucket = storage.get(addr).unwrap();
        assert!(bucket.decorators.contains_key(&2));
        assert!(bucket.modified_ids.contains(&2));
        assert!(!bucket.removed_ids.contains(&2));
        // fresh 0x14155c968：回填的 DecoratorInfo 状态字节 0。
        assert_eq!(bucket.decorators.get(&2).unwrap().state, 0);
        assert_eq!(storage.lookup_storage(DecoratorId(2)), Some(addr));
        assert!(remover.removed.is_empty());
        // reserve：next = max(next, id+1) → 下一次 generate 为 3。
        assert_eq!(id_gen.generate(), DecoratorId(3));

        let evs = drain_events(&events);
        assert_eq!(evs.len(), 1);
        assert_eq!(evs[0].kind(), 3, "新增事件种类 3");
        assert_eq!(evs[0].decorator_id(), 2);
    }

    /// DecoratorBeforeAfterWithStorage 组装：同 id 两侧都在 → before/after 各带
    /// 自己的存储槽与状态；同地址 → 不做碰撞移除；事件为变更事件。
    #[test]
    fn restore_decorators_assembles_before_after_with_storage() {
        let addr = DecoratorStorageAddr::Wall(WallId(5));
        let old_info = info(4);
        let new_info = DecoratorInfo {
            dst_wall: 9,
            decorator_id: 4,
            ..DecoratorInfo::default()
        };
        let mut storage = DecoratorStorage::default();
        storage.apply_decorator_place(addr, DecoratorId(4), &old_info);

        let diff = diff_of(&[(
            addr,
            addr_state(&[(4, &old_info)]),
            addr_state(&[(4, &new_info)]),
        )]);
        let mut id_gen = DecoratorIdGen::default();
        let out = storage.restore_decorators_state_diff(&diff, false, &mut id_gen);

        let entry = out.get(&DecoratorId(4)).expect("before/after entry");
        assert_eq!(entry.before.storage_disc, 0);
        assert_eq!(entry.before.storage_payload, 5);
        assert_eq!(entry.after.storage_disc, 0);
        assert_eq!(entry.after.storage_payload, 5);
        assert_eq!(entry.before.state, DecoratorHistoryState::from(&old_info));
        assert_eq!(entry.after.state, DecoratorHistoryState::from(&new_info));
        // 同地址：清除再回填后桶内为新状态。
        let bucket = storage.get(addr).unwrap();
        assert_eq!(bucket.decorators.get(&4).unwrap().dst_wall, 9);
    }

    /// 两侧地址不同（disc 0，payload 不同）→ 碰撞体移除一次；变更事件类型 0。
    #[test]
    fn restore_decorators_removes_collider_when_address_changes() {
        let wall5 = DecoratorStorageAddr::Wall(WallId(5));
        let wall6 = DecoratorStorageAddr::Wall(WallId(6));
        let placed = info(3);
        let mut storage = DecoratorStorage::default();
        storage.apply_decorator_place(wall5, DecoratorId(3), &placed);

        let diff = diff_of(&[
            (
                wall5,
                addr_state(&[(3, &placed)]),
                DecoratorStorageAddrHistoryState::default(),
            ),
            (
                wall6,
                DecoratorStorageAddrHistoryState::default(),
                addr_state(&[(3, &placed)]),
            ),
        ]);
        let events = DeferredDecoratorOpEvents::default();
        let mut remover = MockColliderRemover::default();
        let mut id_gen = DecoratorIdGen::default();
        storage.restore_decorators(&diff, false, &mut id_gen, &mut remover, &events);

        assert_eq!(remover.removed, vec![DecoratorId(3)]);
        assert_eq!(
            storage.lookup_storage(DecoratorId(3)),
            Some(wall6)
        );
        assert!(storage.get(wall6).unwrap().decorators.contains_key(&3));
        assert!(!storage.get(wall5).unwrap().decorators.contains_key(&3));

        let evs = drain_events(&events);
        assert_eq!(evs.len(), 1);
        assert_eq!(evs[0].kind(), 0, "变更事件种类 = before 判别");
        assert_eq!(evs[0].decorator_id(), 3);
    }

    /// reverse=true 时对调两侧：after 作为移除侧、before 作为新增侧。
    #[test]
    fn restore_decorators_reverse_swaps_sides() {
        let addr = DecoratorStorageAddr::Wall(WallId(7));
        let before_info = info(8);
        let after_info = DecoratorInfo {
            dst_wall: 11,
            decorator_id: 8,
            ..DecoratorInfo::default()
        };
        let mut storage = DecoratorStorage::default();
        storage.apply_decorator_place(addr, DecoratorId(8), &after_info);

        let diff = diff_of(&[(
            addr,
            addr_state(&[(8, &before_info)]),
            addr_state(&[(8, &after_info)]),
        )]);
        let mut id_gen = DecoratorIdGen::default();
        let out = storage.restore_decorators_state_diff(&diff, true, &mut id_gen);
        let entry = out.get(&DecoratorId(8)).unwrap();
        // 移除侧 = after（当前桶内 info），新增侧 = before。
        assert_eq!(entry.before.state, DecoratorHistoryState::from(&after_info));
        assert_eq!(entry.after.state, DecoratorHistoryState::from(&before_info));
        assert_eq!(storage.get(addr).unwrap().decorators.get(&8).unwrap().dst_wall, 7);
    }

    /// d2s remove 缺失 → panic（原生 impl$10::remove 断言）。
    #[test]
    fn restore_decorators_panics_when_d2s_entry_missing() {
        let addr = DecoratorStorageAddr::Wall(WallId(5));
        let placed = info(1);
        let mut storage = DecoratorStorage::default();
        storage.apply_decorator_place(addr, DecoratorId(1), &placed);
        // 制造不一致：桶里有、d2s 无。
        storage.decorator_to_storage.remove(DecoratorId(1));
        let diff = diff_of(&[(
            addr,
            addr_state(&[(1, &placed)]),
            DecoratorStorageAddrHistoryState::default(),
        )]);
        let events = DeferredDecoratorOpEvents::default();
        let mut remover = MockColliderRemover::default();
        let mut id_gen = DecoratorIdGen::default();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            storage.restore_decorators(&diff, false, &mut id_gen, &mut remover, &events);
        }))
        .is_err());
    }

    /// immutable 前置断言 panic（原生 0x14155ba2b）。
    #[test]
    fn restore_decorators_asserts_not_immutable() {
        let mut storage = DecoratorStorage::default();
        storage.mark_immutable();
        let diff: HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff> = HashMap::new();
        let events = DeferredDecoratorOpEvents::default();
        let mut remover = MockColliderRemover::default();
        let mut id_gen = DecoratorIdGen::default();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            storage.restore_decorators(&diff, false, &mut id_gen, &mut remover, &events);
        }))
        .is_err());
    }

    #[test]
    fn decorator_id_gen_reserve_semantics() {
        let mut id_gen = DecoratorIdGen::default();
        id_gen.reserve(DecoratorId(7));
        // next = max(0, 7+1) = 8。
        assert_eq!(id_gen.generate(), DecoratorId(8));
        id_gen.reserve(DecoratorId(2));
        // 不回退。
        assert_eq!(id_gen.generate(), DecoratorId(9));
    }

    #[test]
    fn place_uses_old_option_and_preserves_input_and_partial_mutation_on_index_conflict() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        for flag in [0, 1] {
            let id = DecoratorId(70 + u32::from(flag));
            let addr = DecoratorStorageAddr::Wall(WallId(7));
            let mut value = info(id.0); value.state = flag; value.u45 = 1 - flag;
            value.u30 = 0x1122_3344_5566_7788;
            let mut storage = DecoratorStorage::default();
            assert_eq!(storage.apply_decorator_place(addr, id, &value), value);
            let mut replacement = value; replacement.state = 1 - flag; replacement.u28 = 0x7fc1_2345;
            assert_eq!(storage.apply_decorator_place(addr, id, &replacement), replacement);
            assert_eq!(*storage.decorator_info(id).unwrap(), replacement);
            assert_eq!(storage.lookup_storage(id), Some(addr));

            let mut conflict = DecoratorStorage::default();
            conflict.decorator_to_storage.insert(id, DecoratorStorageAddr::Terrain);
            conflict.get_or_insert_default(addr).removed_ids.insert(id.0);
            assert!(catch_unwind(AssertUnwindSafe(|| conflict.apply_decorator_place(addr, id, &value))).is_err());
            // 原桶mutation先于d2s冲突；不能回滚或按新flag吞原失败。
            let bucket = conflict.get(addr).unwrap();
            assert_eq!(bucket.get(id), Some(&value));
            assert!(bucket.modified_ids.contains(&id.0));
            assert!(!bucket.removed_ids.contains(&id.0));
        }
    }

    #[test]
    fn owned_merge_keeps_target_allocation_and_reindexes_every_source_flag() {
        let mut storage = DecoratorStorage::default();
        let a = DecoratorStorageAddr::Wall(WallId(1));
        let b = DecoratorStorageAddr::Wall(WallId(2));
        let merged = DecoratorStorageAddr::Wall(WallId(3));
        storage.get_or_insert_default(a).decorators.reserve(32);
        for (id, flag, addr) in [(1, 0, a), (2, 1, a), (3, 0, b), (4, 1, b)] {
            let mut value = info(id); value.state = flag;
            storage.apply_decorator_place(addr, DecoratorId(id), &value);
        }
        let original_allocation = storage.get(a).unwrap().get(DecoratorId(1)).unwrap() as *const _;
        storage.on_walls_merged(WallId(1), WallId(2), WallId(3));
        let bucket = storage.get(merged).unwrap();
        assert_eq!(bucket.decorators.len(), 4);
        assert_eq!(bucket.get(DecoratorId(1)).unwrap() as *const _, original_allocation);
        for (id, flag) in [(1, 0), (2, 1), (3, 0), (4, 1)] {
            assert_eq!(storage.lookup_storage(DecoratorId(id)), Some(merged));
            assert_eq!(bucket.get(DecoratorId(id)).unwrap().state, flag);
        }
    }

    #[test]
    fn tracked_clone_uses_real_drain_on_success_and_on_required_index_failure() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let addr = DecoratorStorageAddr::Terrain;
        let mut source = Decorators::default();
        for (id, flag) in [(20, 0), (21, 1)] {
            let mut value = info(id); value.state = flag; source.insert(DecoratorId(id), value);
        }
        let mut target = Decorators::default(); target.decorators.reserve(32);
        let mut d2s = DecoratorToStorageMap::default();
        for id in [1, 2] { target.insert(DecoratorId(id), info(id)); d2s.insert(DecoratorId(id), addr); }
        let capacity = target.decorators.capacity();
        target.tracked_clone_from(&source, &mut d2s, addr);
        assert_eq!(target.decorators, source.decorators);
        assert!(target.removed_ids.contains(&1) && target.removed_ids.contains(&2));
        assert!(d2s.get(DecoratorId(1)).is_none() && d2s.get(DecoratorId(2)).is_none());
        assert_eq!(target.decorators.capacity(), capacity);
        for id in [20, 21] { assert_eq!(d2s.get(DecoratorId(id)), Some(addr)); }

        let mut broken = Decorators::default(); broken.decorators.reserve(32);
        for id in [7, 8, 9] { broken.insert(DecoratorId(id), info(id)); }
        let capacity = broken.decorators.capacity();
        assert!(catch_unwind(AssertUnwindSafe(|| broken.tracked_clone_from(&source, &mut DecoratorToStorageMap::default(), addr))).is_err());
        assert!(broken.decorators.is_empty());
        assert_eq!(broken.decorators.capacity(), capacity);
        assert!(broken.modified_ids.is_empty());
        assert!(broken.removed_ids.is_superset(&HashSet::from([7, 8, 9])));
        assert_eq!(source.decorators.len(), 2);
    }

}
