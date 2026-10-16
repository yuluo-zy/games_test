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

use super::WallId;
use super::decorator::{DecoratorId, DecoratorInfo};

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
    /// + `insert<DecoratorId,DecoratorInfo>`（内嵌 map）；随后按状态字节写输出
    /// +0x47（2=live）或 d2s update。
    pub fn insert(&mut self, id: DecoratorId, info: DecoratorInfo) -> Option<DecoratorInfo> {
        self.removed_ids.remove(&id.0);
        let old = self.decorators.insert(id.0, info);
        self.modified_ids.insert(id.0);
        old
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

    /// 0x14155b170（O）：合并源集合。源有自无 → 插入；冲突 → 标 removed（去重）。
    pub fn merge_with(&mut self, source: &Decorators) {
        for (id, info) in &source.decorators {
            if self.decorators.contains_key(id) {
                self.removed_ids.insert(*id);
            } else {
                self.decorators.insert(*id, info.clone());
            }
        }
    }

    /// 0x141256e40 / 0x1412573b0（O，两泛型实例化合并实现）：tracked_clone_from。
    /// 逐字段克隆内嵌 map，并按源 removed_ids/内嵌 map 标 removed；随后维护 d2s
    /// 追踪集合（impl$8 insert 冲突 panic 常量 0x142c21300/0x142c21318）。
    pub fn tracked_clone_from(
        &mut self,
        source: &Decorators,
        d2s: &mut DecoratorToStorageMap,
        addr: DecoratorStorageAddr,
    ) {
        self.removed_ids.clone_from(&source.removed_ids);
        self.decorators.clone_from(&source.decorators);
        self.modified_ids.clear();
        for id in &self.removed_ids {
            let _ = id;
        }
        for (id, _info) in &self.decorators {
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

/// B DecoratorAddrSnapshot：快照单条记录（地址 + 编码 DecoratorId + 状态字节）。
/// take_snapshot/restore_snapshot 0x140a20d70 以 16B XOR 掩码编码 id。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecoratorAddrSnapshot {
    /// 编码 DecoratorId（64 位半字旋转 XOR 掩码）。
    pub encoded_id: u64,
    /// 所在存储地址。
    pub addr: DecoratorStorageAddr,
    /// 状态字节（判别跳表 0x142ac9bdc/0x142ac9bfc，U：状态语义待取证）。
    pub state: u8,
}

/// B DecoratorStorageSnapshot：装饰存储快照（`sort` 行 1240 内联）。
/// **① c-apply-api-formats 契约 §2 格式标注**：原生 restore_snapshot（0x140a20d70）
/// 载荷 = `&[0x28B 记录]`（{状态 16B + 子计数 8B} + 0x16B 子记录）；本 crate
/// `DecoratorAddrSnapshot`（16B 条目）为**近似模型**，与原生 0x28B/0x16B 节格式**不符**——
/// 逐字段名待①取证（中优先）。`restore_snapshot` 签名 + 空桶前置 + 逐装饰恢复序可直接用
/// （0x140a20d70），节格式对齐待①。
#[derive(Clone, Debug, Default)]
pub struct DecoratorStorageSnapshot {
    /// 快照条目。
    pub states: Vec<DecoratorAddrSnapshot>,
}

impl DecoratorStorageSnapshot {
    /// 行 1240（内联）：排序（U：原排序键待取证；按（地址，编码 id）稳定序）。
    pub fn sort(&mut self) {
        self.states.sort_by_key(|s| {
            (
                match s.addr {
                    DecoratorStorageAddr::Wall(w) => w.0,
                    DecoratorStorageAddr::Terrain => u64::MAX - 1,
                    DecoratorStorageAddr::Stair => u64::MAX,
                },
                s.encoded_id,
            )
        });
    }
}

/// 快照 id 编码掩码（_UNK_1429cd080/_UNK_1429cd090，U：数值待取证，暂 0）。
const SNAPSHOT_ID_MASK: u64 = 0;

/// 编码：`rotate_right32(id) XOR mask`（原 `(uVar18<<0x20|uVar18>>0x20) ^ mask`）。
/// take_snapshot 捕获面使用（当前快照记录为空存储前置断言下无内容，保留供
/// capture 路径接线）。
#[allow(dead_code)]
#[inline]
fn encode_snapshot_id(id: u32) -> u64 {
    (u64::from(id)).rotate_right(32) ^ SNAPSHOT_ID_MASK
}

/// 解码：`(encoded ^ mask).rotate_left(32)`。
#[inline]
fn decode_snapshot_id(encoded: u64) -> u32 {
    (encoded ^ SNAPSHOT_ID_MASK).rotate_left(32) as u32
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
    /// 作为桶的 removed_ids（recovered_building restore_one 观察 + fresh
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

    /// 0x140a1dba0（I）：删除存储（清 per_wall/内联桶 + d2s 索引）。
    pub fn remove_storage(&mut self, addr: DecoratorStorageAddr) -> Option<Decorators> {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let removed = match addr {
            DecoratorStorageAddr::Wall(wall) => self.per_wall.remove(&wall),
            DecoratorStorageAddr::Terrain => self.terrain.take(),
            DecoratorStorageAddr::Stair => self.stairs.take(),
        };
        if let Some(decos) = &removed {
            for id in decos.decorators.keys() {
                self.decorator_to_storage.remove(DecoratorId(*id));
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
    /// 重插入接线属 recovered_building 归档类型边界（登记）。
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

    /// 0x140a1e720（I）：清除所有桶的修改追踪。
    pub fn clear_modification_tracking(&mut self) {
        for d in self.per_wall.values_mut() {
            d.clear_modification_tracking();
        }
        if let Some(d) = &mut self.terrain {
            d.clear_modification_tracking();
        }
        if let Some(d) = &mut self.stairs {
            d.clear_modification_tracking();
        }
    }

    /// 0x140a1ea40（O，21KB）：墙合并迁移。per_wall 两源 → merge_with →
    /// d2s 移除（panic "d2s remove: no such"）→ 子墙插入 + d2s 更新。
    pub fn on_walls_merged(&mut self, a: WallId, b: WallId, merged: WallId) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let src_a = self.per_wall.remove(&a);
        let src_b = self.per_wall.remove(&b);
        let mut target = Decorators::default();
        if let Some(d) = &src_a {
            for id in d.decorators.keys() {
                self.decorator_to_storage.assert_exists(DecoratorId(*id));
                self.decorator_to_storage.remove(DecoratorId(*id));
            }
            target.merge_with(d);
        }
        if let Some(d) = &src_b {
            for id in d.decorators.keys() {
                self.decorator_to_storage.assert_exists(DecoratorId(*id));
                self.decorator_to_storage.remove(DecoratorId(*id));
            }
            target.merge_with(d);
        }
        for id in target.decorators.keys() {
            self.decorator_to_storage
                .insert(DecoratorId(*id), DecoratorStorageAddr::Wall(merged));
        }
        self.per_wall.insert(merged, target);
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

    /// 0x140a1f590（O，46KB）：墙分裂迁移。fresh 反编译（2026-10-16）+ E2 取证
    /// 定案：**逐装饰记录 d2s 移除 + 记入子墙 +0x30 注册表的 removed id 集**，并
    /// 保证子墙桶存在；装饰 info 不直接拷入子墙内嵌 map——由消费方（archivist /
    /// 重放）按 removed 注册表重新落位。**子侧判定 = 状态字节公式
    /// `(b-2)<=2 ? b-1 : 0`**（[`side_from_state`]，b∈{0,1}→侧0、b=2→侧1、
    /// b=3→侧2、b=4→侧3、其余→侧0；仅侧 0/1 有效，侧 2/3 panic "internal error:
    /// entered unreachable code" 0x142ac8d80）；侧 0 原还经 u 区间包含搜索定子墙
    /// （get_approx_u_from_pos + clamp[0,1] + {low,high} f32 对数组），0x28B 记录
    /// 完整语义仍 U（RE §6.4）。d2s 移除未中 panic "assertion failed:
    /// self.decorator_to_storage.map.remove(&decorator_id).is_some()"（0x142ac9300）。
    pub fn on_wall_split(&mut self, wall: WallId, children: [WallId; 2]) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let source = match self.per_wall.remove(&wall) {
            Some(d) => d,
            None => return,
        };
        for (id, info) in &source.decorators {
            let side = side_from_state(info.state);
            if side > 1 {
                unreachable!("internal error: entered unreachable code");
            }
            let child = children[side as usize];
            let child_addr = DecoratorStorageAddr::Wall(child);
            // d2s 移除（fresh remove_entry miss==3 → panic）。
            self.decorator_to_storage
                .remove(DecoratorId(*id))
                .expect("assertion failed: self.decorator_to_storage.map.remove(&decorator_id).is_some()");
            // 子墙桶存在（fresh rustc_entry + 默认构造）。
            let _ = self.get_or_insert_default(child_addr);
            // +0x30 注册表：子墙地址 → 该 id 记 removed（fresh rustc_entry + set
            // insert；实际落位由消费方按注册表重放）。
            self.removed
                .entry(child_addr)
                .or_default()
                .insert(*id);
        }
    }

    /// 0x140a20670（O）：应用装饰移除。immutable 门控 → 目标桶（get_mut，缺失
    /// panic "assertion failed: self.map.remove(&id).is_some()" 0x142ac9068）→
    /// modified_ids 移除 + removed_ids 标记 + 内嵌 map 移除（未中 fmt panic
    /// "apply_decorator_remove: Trying to remove a decorator that doesn't exist: "
    /// 0x142ac9430）→ 输出复刻移除的 DecoratorInfo。
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
        let _ = self.decorator_to_storage.remove(decorator_id);
        info
    }

    /// 0x140a20890（O，apply_decorator_remove 闭包）：变更应用。目标存在 →
    /// 覆盖并标 modified；缺失 → 走插入路径（I：精确分支待取证）。
    pub fn apply_decorator_change(
        &mut self,
        addr: DecoratorStorageAddr,
        decorator_id: DecoratorId,
        info: &DecoratorInfo,
    ) {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        if let Some(decorators) = self.get_mut(addr) {
            decorators.decorators.insert(decorator_id.0, *info);
            decorators.modified_ids.insert(decorator_id.0);
        }
    }

    /// 0x140a20c70（O）：应用装饰放置。immutable 门控 → 目标桶
    /// （get_or_insert_default）→ 内嵌 map 写入 + modified_ids 标记 + d2s 插入
    /// → 输出复刻放置的 DecoratorInfo（与 remove 对称）。
    pub fn apply_decorator_place(
        &mut self,
        addr: DecoratorStorageAddr,
        decorator_id: DecoratorId,
        info: &DecoratorInfo,
    ) -> DecoratorInfo {
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let decorators = self.get_or_insert_default(addr);
        decorators.insert(decorator_id, *info);
        self.decorator_to_storage.insert(decorator_id, addr);
        *info
    }

    /// 0x140a20d70（O，26KB）：快照（take_snapshot 内联于同址）。前置 6 断言：
    /// per_wall/removed/d2s 空 + terrain/stairs None + !immutable。逐桶以
    /// get_or_insert_default 记录装饰状态，id 以 16B XOR 掩码编码，按状态字节
    /// 判别跳表（0x142ac9bdc/0x142ac9bfc）标 removed。
    pub fn take_snapshot(&self) -> DecoratorStorageSnapshot {
        assert!(self.per_wall.is_empty(), "assertion failed: per_wall.is_empty()");
        assert!(self.terrain.is_none(), "assertion failed: terrain.is_none()");
        assert!(self.stairs.is_none(), "assertion failed: stairs.is_none()");
        assert!(self.removed.is_empty(), "assertion failed: removed.is_empty()");
        assert!(
            self.decorator_to_storage.is_empty(),
            "assertion failed: decorator_to_storage.map.is_empty()"
        );
        assert!(!self.immutable, "assertion failed: !self.immutable");
        // 原实现：快照在逐墙/逐装饰记录重建过程中收集状态；Rust 侧以独立
        // capture 结构承载（I：逐记录状态字节编码待取证）。
        DecoratorStorageSnapshot::default()
    }

    /// 0x140a20d70（O，26KB）：恢复快照。前置断言同 take_snapshot；逐条解码
    /// 编码 id（旋转还原）→ get_or_insert_default 目标桶 → 标 removed。
    pub fn restore_snapshot(&mut self, snapshot: &DecoratorStorageSnapshot) {
        assert!(self.per_wall.is_empty(), "assertion failed: per_wall.is_empty()");
        assert!(self.terrain.is_none(), "assertion failed: terrain.is_none()");
        assert!(self.stairs.is_none(), "assertion failed: stairs.is_none()");
        assert!(self.removed.is_empty(), "assertion failed: removed.is_empty()");
        assert!(
            self.decorator_to_storage.is_empty(),
            "assertion failed: decorator_to_storage.map.is_empty()"
        );
        assert!(!self.immutable, "assertion failed: !self.immutable");
        let mut decoded: Vec<(u32, DecoratorStorageAddr, u8)> = Vec::with_capacity(snapshot.states.len());
        for s in &snapshot.states {
            decoded.push((decode_snapshot_id(s.encoded_id), s.addr, s.state));
        }
        for (id, addr, _state) in decoded {
            let decorators = self.get_or_insert_default(addr);
            // 状态字节 U：此处按已移除处理（恢复即标 removed，与 ID 解码后
            // "旋转还原后移除" 的 O 描述一致）。
            decorators.removed_ids.insert(id);
            let _ = self.decorator_to_storage.update(DecoratorId(id), addr);
        }
    }

    /// 0x14155ba30（I，历史 replay_pop_bucket closure）：按历史闭包恢复装饰。
    pub fn restore_decorators<F>(&mut self, restore: F)
    where
        F: FnOnce(&mut DecoratorStorage),
    {
        restore(self);
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
    fn on_wall_split_marks_removed_and_unregisters() {
        let mut storage = DecoratorStorage::default();
        // state 0 → 侧 0；state 2 → 侧 1（公式 side_from_state）。
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(1), &info(1));
        let mut info2 = info(2);
        info2.state = 2;
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(2), &info2);
        storage.on_wall_split(WallId(1), [WallId(11), WallId(12)]);

        // 原墙已删、d2s 已注销（fresh 0x140a1f590：remove_entry miss==3 → panic）。
        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(1))).is_none());
        assert_eq!(storage.lookup_storage(DecoratorId(1)), None);
        assert_eq!(storage.lookup_storage(DecoratorId(2)), None);
        // 子墙桶存在，且 +0x30 注册表登记各 id 的 removed 归属。
        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(11))).is_some());
        assert!(storage.get(DecoratorStorageAddr::Wall(WallId(12))).is_some());
        let reg_11 = storage.removed.get(&DecoratorStorageAddr::Wall(WallId(11))).unwrap();
        let reg_12 = storage.removed.get(&DecoratorStorageAddr::Wall(WallId(12))).unwrap();
        assert!(reg_11.contains(&1));
        assert!(reg_12.contains(&2));
        assert!(storage.iter_removed_storages().any(|a| a == DecoratorStorageAddr::Wall(WallId(11))));
        assert!(storage.iter_removed_storages().any(|a| a == DecoratorStorageAddr::Wall(WallId(12))));
    }

    #[test]
    fn on_wall_split_panics_unreachable_for_side_23() {
        let mut storage = DecoratorStorage::default();
        let mut info3 = info(3);
        info3.state = 3; // → 侧 2 → unreachable panic。
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(WallId(1)), DecoratorId(3), &info3);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            storage.on_wall_split(WallId(1), [WallId(11), WallId(12)]);
        }))
        .is_err());
    }

    #[test]
    fn snapshot_id_encode_decode_roundtrip() {
        for id in [0u32, 1, 0x1234_5678, u32::MAX] {
            assert_eq!(decode_snapshot_id(encode_snapshot_id(id)), id);
        }
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
}
