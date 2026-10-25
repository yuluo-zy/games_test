# ERASER — walls 域 eraser finish（0x14092f990）重建（restorer-eraser，2026-10-17）

> restorer：restorer-eraser。协议：`planning/2026-10-16-logic-verify-protocol.md`。
> 方法：**只以 fresh 反编译为准**。本轮 fresh 拉取 7 函数（0x14092f990 / 0x14092ea50 /
> 0x14092e8e0 / 0x14092eff0 / 0x14092f280 / 0x14092f630 / 0x14092e6e0）落盘
> `planning/impl-walls-return/decomp/restorer-eraser-fresh-result.json`（ghidra `--worker shared`
> 只读；body 与既有 `14092f990-finish.txt` 等逐字节判等，复用成立）。
> history 侧 0xf8B 布局 + 17 变体判别表 + ConcurrentEdits 语义以
> `reports/state/history/restorer-walledit-WALLEDIT.md` 定案，本任务只做 walls 域装配。

## 改动文件（walls 域）

- `src/systems/wall/eraser/archivist.rs`：**重建**（3-map + 5 事件方法 + finish 主体 +
  WallHistoryEdits 5 桶 + 7 测试）。`cargo check -p country-core --lib` 0 错误；
  lib 全量 1857 测试 PASS（新增 7）；`--lib wall` 93 PASS。
- `src/systems/wall/eraser/wall_eraser.rs`：**未改**（`cache_if_original_wall` 签名保持
  `(wall, &snapshot, range)`，调用面兼容）。
- `recovered_building/wall_history_edit.rs` 消费方：**勿动**（红线遵守）。

## 函数表（fresh 逐函数 verdict）

| VA | 函数 | verdict | 分歧点 / 重建依据 |
|---|---|---|---|
| 0x14092e6e0 | `new` | **判等-结构**（3 map 布局锚定） | 三 map 各 0x30B（`{ctrl, mask, X, items, k0, k1}`），map1 @+0x00 / map2 @+0x30 / map3 @+0x60；独立 RandomState；`&anon_...` 哨兵 + 0。Rust `Default` 以 3 空 HashMap 承载 |
| 0x14092e8e0 | `record_rewiring_event` | **判等** | map1 `HashMap::insert<OriginalWallId, RewireEnclosedWall>` + 旧值 drop。Rust `map1.insert(wall, target)` |
| 0x14092ea50 | `cache_if_original_wall` | **判等-结构**（载荷内层登记） | map1 已存在 → return；map3 已存在 → panic；克隆源 → `EraserWallState{id, 0x3f80000000000000}` 插 map2 → map1 插 `(id, id)`。Rust：`contains_key` 门 + `EraserWallState{original, range}` 插 map2 + map1 自指插 |
| 0x14092eff0 | `shrink_wall` | **判等-结构**（ChildWall 记录形态登记） | map1 探针（key==id）→ map3 探针（key==map1 值）→ 逐 child 找 id → `ChildWall::slice(id,t1,t2)` 写回 a/b；未命中 expect/unwrap_failed。Rust：`map1.get(id)` → `map3.get_mut(target)` → `children[..].slice` |
| 0x14092f280 | `add_split_event` | **判等-结构** | 同寻址 + `swap_remove` 被拆 child + reserve + `in_R9` 段逐段 `slice` push + 每段 `insert<GlobalId,GlobalId>`（map1 插新段）。Rust：`ChildWalls::remove` + `extend` + map1 自指插 |
| 0x14092f630 | `add_fully_erased_event` | **判等-结构** | 同寻址 + `swap_remove`；空桶重写 0x8000000000000000（+0x70）+ 被移除 child 值（+0x78）+ 0（+0x80）。Rust：`fully_erased = true` 为判别最近拟（编码登记缺类型） |
| 0x14092f990 | `finish`（11509B） | **重建**（结构装配 + 3 登记缺类型） | 见 §A；与 history 5 桶判别表对齐，非简化冒顶 |

## A. finish（0x14092f990）— 逐分支重建

1. **drain map2**（EraserWallState，0xd8B 载荷）：RawTable 迭代逐 entry → 0xf8B
   `WallHistoryEdit` 变体 0 桶 local_5b8（op 槽 @+0xe8 = **2**，start/end 未显式写 =
   don't-care）。Rust `erase_original_state_change`：`StateChange{wall, previous:
   EraserWallState.original, next: 切片后快照, observed_operation:{2,0,0}}`。
2. **drain map3**（ChildWalls，0x90B 条目）每 entry 按判别字段两分支：
   - **shrink/split**：初始区间 `0.5 ± 0.5·(0.2/height)`（高度夹 0.2、[0,1] 断言 +
     NaN panic）→ `Curve::crashy_slice<Vec2,2D>` → 变体 0 桶 local_570 + 变体 2 桶
     local_558；child `sort_by`（smallsort 1<len<0x15 / driftsort 大）后逐段中点
     `fVar66 = cur[+8]·0.5 + prev[+0xc]·0.5`（[0,1] 断言 + NaN panic）→ 末位 end=1.0
     或 `next[+8]·0.5 + cur[+0xc]·0.5` → `end <= start` 断言 → `crashy_slice` →
     `inverse_lerp`×2（NAN/INF panic）→ clamp → 装配变体 0。
   - **全擦除**：`len < 2 || height <= 0` → panic；child Vec 排序；逐段改写变体 1 桶
     local_588 + 变体 2 桶 local_5a0；原生同分支 remap 循环（LAB_140930c32）还产变体
     0 桶 local_570（StateChange）——本重建一并装配。
3. **Sip13 inner-walls 探针**（`core::hash::sip::impl$5::write<Sip13Rounds>` 展开 +
   -0x78 stride 表探针，命中装配 0x70B ChildWall 改写记录）——**缺类型登记**（需
   inner-walls map 类型 + SipHash13 常量，非 walls 域现有类型），本重建不实现。
4. **末段**：5 桶 → 逐条 `Arc<WallHistoryEdit>`（0x108 = 0x10 Arc 头 + 0xf8 载荷）→
   `from_iter<ConcurrentEdits>` → `History::add(tag 0x7)` → `RawTable::clear ×2`（map1 +
   map2）。Rust：`WallHistoryEdits.compile_history_edit` → `WallCompoundHistoryEdit{groups:
   [bucket_5b8, bucket_570, bucket_558, bucket_588, bucket_5a0]}`（`ConcurrentEdits =
   Vec<WallHistoryEdit>` 已由 history 侧建模；Arc 层 = `RecordWallHistory(Arc<..>)`）；
   finish 清 map1 + map2。

## 落盘的 walls 域类型（缺类型补锚）

- `OriginalWallId(pub u64)` / `RewireEnclosedWall(pub u64)`：map1 键/值 == map3 键
  （u64 newtype；Ghidra 折叠为 `GlobalId`）。
- `EraserWallState {wall, original: WallAuthoritySnapshot, range: ErasedRange}`：原生
  0xd8B 载荷。**内层字段映射登记缺类型**（finish 把 0xd8B 逐偏移读入记录载荷
  +0x08..+0xe0，各 qword 语义未逐字段取证；本重建以快照 + 区间为结构最近拟）。
- `ChildWalls {wall, original, children: Vec<ChildWall>, fully_erased}`：原生 0x80B 载荷
  （`Vec<ChildWall>` @+0x78/+0x80/+0x88 + 墙态字段）。`remove`（swap_remove）/`extend`/
  `is_empty` 按原 inline（inventory line 73/82/98）。**`fully_erased` 判别字段编码
  登记缺类型**（原生空桶重写 0x8000000000000000）。

## 缺类型登记（不编造）

| 缺类型 / 缺面 | 说明 |
|---|---|
| EraserWallState / ChildWalls 内层字段（载荷 +0x08..+0xe0 逐偏移语义） | fresh 仅确认记录装配偏移；字段语义待布局取证 |
| ChildWalls +0x70 判别字段编码（shrink/split vs 全擦除） | 本重建以 `fully_erased: bool` 为判别最近拟 |
| finish Sip13 inner-walls 探针 + 0x70B 输出记录 | 需 inner-walls map 类型 + SipHash13；非 walls 域 |
| ChildWall 在 archivist 内的记录形态（shrink_wall `puVar6[2]/[3]` 改写面） | 以 `utils::child_wall::ChildWall` + `slice(id,t1,t2)` 为锚定形态 |
| roof 删除路径 | 原生 finish 本体不含 roof；roof 删除在擦除流他处。本系统保留既有 walls 域 `RecordRoofHistory` 输出，登记待 roof 域锚定 |

## 自检

- `cargo check -p country-core --lib`：0 错误（仅既有 non_snake_case 等警告）。
- `cargo test -p country-core --lib`：1857 passed，0 failed。
- `cargo test -p country-core --lib archivist`：9 passed（7 新增 + 2 既有 decorator）。
- `cargo test -p country-core --lib wall`：93 passed。
- 未改他模块业务；共用只读 `--worker shared`，本任务无需写库。
