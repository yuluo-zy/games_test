# WALLEDIT — WallHistoryEdit 0xf8B 真布局 + ConcurrentEdits 语义（restorer-walledit，2026-10-17）

> restorer: restorer-walledit。依据 2026-10-16-logic-verify-protocol.md。
> 目的：解锁 walls 域 eraser finish（0x14092f990）5 桶→History::add 链。
> 方法：只以 fresh 反编译为准（本任务 fresh 拉取 0x140a84ec0/0x140a82b20/40/d50
> 落盘 `impl-history-walledit/decomp/*.txt`；交叉复核 eraser finish
> `impl-walls-return/decomp/14092f990-finish.txt`、ev_new_wall
> `verifier-history/decomp/0x140a83300-ev_new_wall.txt`、new_height_changed
> `verifier-history/decomp/0x140a83fb0-new_height_changed.txt`）。
> 红线遵守：只以 fresh 反编译为准；缺类型登记不编造；不改 walls/collision 模块。

---

## 1. `WallHistoryEdit` 0xf8B 真布局（fresh 定案）

**记录**：0xf8B（`grow_one<WallHistoryEdit>` 步长 0xf8、`next` memcpy 0xf8、
`Arc<WallHistoryEdit>` 分配 0x108 = 0x10 Arc 头 + 0xf8 载荷）。

**判别**：判别字段在 **T+0x00（u64）**。变体 0 用**非 niche 值**（eraser finish 的
bucket0 直接把 OriginalWallId 写入判别槽）；变体 1..16 用 niche 值
`0x8000000000000000..0x800000000000000f`。

**判别解码**（`History::add` 0x140a84ec0 与 `WallReplayEditCmd::process`
0x140abfb30 同式，均 fresh）：

```text
lVar7 = disc - 0x7fffffffffffffff            ; 无符号回绕
if (disc 有符号) > -0x7ffffffffffffff1       ; = disc > 0x800000000000000f(signed)
        → variant = 0                         ; 非 niche（wall id 等数据）
else    → variant = lVar7 ∈ [1, 16]           ; niche 区
```

### 1.1 变体判别表（T+0x00 值）

| 变体 | 操作（WallHistoryOperationTag） | 判别值（T+0x00） | 载荷（fresh 确认） |
|---|---|---|---|
| 0 | WallChanged | 非 niche（= wall id，eraser bucket0） | `u32 @T+0xe8`（op 号）、`f32 @T+0xec`（start）、`f32 @T+0xf0`（end）；其余 = previous/next authority 快照等 |
| 1 | CreateWall | `0x8000000000000000` | `Option<source> @T+0x08`（None → add panic）、wall id、color、restored height/elevation |
| 2 | DeleteWall | `0x8000000000000001` | `Option<source> @T+0x08`（None → add panic）、wall id、color |
| 3 | AdjustHeight | `0x8000000000000002` | `wall_id u64 @T+0x08`、`from f32 @T+0x10`、`to f32 @T+0x14`（`write_observed_arc_fields`） |
| 4 | ChangeIsEnclosedState | `0x8000000000000003` | previous/next shape（**旧注释 0x8000000000000004 差一错，fresh 修正为 3**） |
| 5 | MorphPremadeShape | `0x8000000000000004` | **缺类型**（未恢复） |
| 6 | ChangeElevation | `0x8000000000000005` | **缺类型**（未恢复） |
| 7 | ChangeArchHeight | `0x8000000000000006` | **缺类型**（未恢复） |
| 8 | ChangeSupports | `0x8000000000000007` | **缺类型**（未恢复） |
| 9 | ChangeStairsType | `0x8000000000000008` | **缺类型**（未恢复） |
| 10 | ChangePlasterPeelAmount | `0x8000000000000009` | **缺类型**（未恢复） |
| 11 | ChangeFreehandHeightType | `0x800000000000000a` | **缺类型**（未恢复） |
| 12 | ChangeArchType | `0x800000000000000b` | **缺类型**（未恢复） |
| 13 | MorphFreehandWall | `0x800000000000000c` | **缺类型**（未恢复） |
| 14 | DeprecatedMorphOriginal | `0x800000000000000d` | **缺类型**（未恢复） |
| 15 | Unresolved15 | `0x800000000000000e` | **缺类型**（未恢复） |
| 16 | ChangeSkirt | `0x800000000000000f` | **缺类型**（未恢复） |

### 1.2 已知载荷偏移细表

- `Option<source> @T+0x08`：变体 1/2（`History::add` `SBORROW8(0, v)` 即 v>0 才 panic
  的逆映射：None（0）→ 违规）。`None is not allowed in CreateWall/DeleteWall`
  panic 文案 @0x142ad81b8/@0x142ad8200。
- 变体 0 slice 校验：`*(u32 @T+0xe8) < 2` 门内，`f32 @T+0xf0 < f32 @T+0xec || ==`
  → panic `"Slice start_u must be smaller than end_u..."`（0x142ad8170）。
- 变体 3：`wall_id @T+0x08`、`from @T+0x10`、`to @T+0x14`（`new_height_changed`
  0x140a83fb0 `memcpy(&local_168, 0x108)` 头 3 qword = Arc 头 ×2 + 判别，随后
  `write_observed_arc_fields` 逐字节：`[16..24]=0x8000000000000002`、`[24..32]=wall_id`、
  `[32..36]=from`、`[36..40]=to`）。
- 变体 0 判别 = wall id（eraser bucket0 `local_2f8 = lVar7` = map2 key = OriginalWallId）。

### 1.3 eraser finish 5 桶→判别映射（0x14092f990，walls 侧消费面）

| 桶（local_*） | 来源 | 判别值 | 变体 |
|---|---|---|---|
| local_5b8 | drain map2 EraserWallState（0xd8B 载荷） | OriginalWallId（非 niche） | 0 WallChanged |
| local_570 | map3 shrink/split 第一段 + Sip13 inner-walls 探针改写 | ChildWall @+8 值（非 niche） | 0 WallChanged |
| local_558 | map3 shrink/split 第二段 | `0x8000000000000001` | 2 DeleteWall |
| local_5a0 | map3 全擦除分支 | `0x8000000000000001` | 2 DeleteWall |
| local_588 | map3 全擦除分支 | `0x8000000000000000` | 1 CreateWall |

末段：5 桶 → 逐条 `Arc<WallHistoryEdit>`（0x108）→
`from_iter<ConcurrentEdits, Map<IntoIter<Vec<Arc<WallHistoryEdit>>>, impl_0::new::closure_env_0>>`
→ `History::add`（tag 0x7）→ `RawTable::clear ×2`（map1 RewireEnclosedWall + map2
EraserWallState）。另有一 0x70B 记录（`grow_one<CommandBuffer, Vec<PassPgoDuration>>`）
为 inner-walls 探针的**非 history 输出面**，walls 域登记。

---

## 2. `ConcurrentEdits` 容器语义（fresh 定案）

- **类型**：原生 `MultiEdit.records = Vec<ConcurrentEdits>`（`History::add` 逐元素步长
  0x10 = `{ptr, len}` → `ConcurrentEdits = Box<[Arc<WallHistoryEdit>]>`）。构建经
  `from_iter<ConcurrentEdits, Map<IntoIter<Vec<Arc<WallHistoryEdit>>>, impl_0::new::closure_env_0>>`
  （`build` 0x140a82d50 / eraser finish / ev_new_wall 三处同签名；closure 以 `.into()`
  把组 `Vec<Arc<WallHistoryEdit>>` 收为 boxed slice）。
- **语义**：**单个并发编辑组** = 同一历史索引处同时发生的一组编辑；`Vec<ConcurrentEdits>`
  = 组序列，**插入序保留**（不按 id/时间重排）。组内编辑共享 `Arc`（同一编辑可出现在
  多个并发组）。erafer finish 的 5 桶 → 5 组（空桶保留为空组）。
- **消费**：`History::add` 逐组、逐 Arc 校验（§1.2）；replay 侧 `expand_wall_compound`
  逐组推进，相邻组 wall id 集不相交才同批（`is_disjoint` 守卫）。

**Rust 建模**：`pub type ConcurrentEdits = Vec<WallHistoryEdit>`（等价 `groups:
Vec<Vec<WallHistoryEdit>>`，省去组内 Arc 共享；消费方逐组迭代语义不变）。

---

## 3. Rust 重建（`src/recovered_building/wall_history_edit.rs`）

- `WallHistoryEdit` 由 4 变体近似改为 **17 变体**（§1.1 表）。判别按 fresh：
  - `StateChange` = 原生变体 0 WallChanged（disc = wall id）。
  - `Create` = 变体 1（`0x8000000000000000`）、`Delete` = 变体 2（`0x8000000000000001`）。
  - **`AdjustHeight { wall, from, to }`** = 变体 3（`0x8000000000000002`），新增实载荷。
  - `EnclosureChange` = 变体 4（`0x8000000000000003`）——**旧注释 0x8000000000000004
    差一错已修正**。
  - 变体 5..16 = `MorphPremadeShape(UnresolvedWallEditPayload)` 等 13 项，以单元标记
    显式登记缺类型（不编造字段）。
- 新增 `native_discriminant()` / `operation_tag()`（`History::add` 同式解码）/
  `wall_id() -> Option<WallId>` 方法。
- `WallHistoryOperationTag` 补 `native_discriminant()` + `from_variant_number()`。
- `ConcurrentEdits` 类型别名 + 语义注释；`WallCompoundHistoryEdit.groups:
  Vec<ConcurrentEdits>`。
- `apply_instant_wall_edit`：新增变体返 `WallReplaySourceError::UnrestoredOperation(u64)`
  （新增错误变体）；`AdjustHeight` 即时面由 `WallHeightReplayRequest` 权威消费，不经
  authority 快照写入。

**消费方更新（非 walls/collision 域）**：
- `recovered_building/wall_history_replay.rs` `wall_id()` → `edit.wall_id().unwrap_or(...)`。
- `systems/history/replay_wall.rs` `command_wall()` → 同上。
- `systems/save/load_game.rs` `wall_id_of()` → `edit.wall_id().map(...)`。
- `resources/history/mod.rs` `validate_for_recording` → 新增变体补 `{}` 分支（原生 add
  只校验变体 0/1/2）。

**新增测试（`wall_history_edit.rs::tests`，4 项全 PASS）**：
- `arc_allocation_holds_header_plus_f8_payload`（≤0x108 上限）。
- `niche_discriminants_match_fresh`（含 EnclosureChange=3 修正）。
- `history_add_discriminant_decode_matches_fresh`（含 0x800000000000000f→16）。
- `concurrent_edits_groups_preserve_insertion_order`。

**编译/测试**：`cargo check -p country-core --lib` 0 错误；lib 全量 1850 测试 PASS
（history 59、wall 86、decorator 等未回归）。

---

## 4. 缺类型登记（不编造）

| 缺类型 / 缺面 | 说明 | 归属域 |
|---|---|---|
| 变体 5..16 载荷（MorphPremadeShape..ChangeSkirt） | fresh 仅确认判别值 + 操作号；字段语义待布局取证 | history（本模块登记） |
| eraser 变体 0（WallChanged）载荷 = EraserWallState（0xd8B） | 原生 wall 原始快照 + 擦除范围；walls 域类型，history 型无法直接承载 | walls（`impl-walls-return` 登记） |
| 变体 0 的 `Option<source> @T+0x08` 面 | 原生为 walls authority 快照 Option；src `StateChange/Create/Delete` 以 `source_66` Option 为结构最近拟（replay_wall `MissingSource66` 同缺口） | walls |
| eraser 5 桶的具体 ChildWall 字段→判别映射 | local_570 的判别 = ChildWall @+8 值，字段语义未解 | walls（`impl-walls-return`） |
| 0x70B inner-walls 探针输出记录 | `grow_one<CommandBuffer, Vec<PassPgoDuration>>`，非 history 面 | walls/collision |

---

## 5. `History::add`（0x140a84ec0）tag-7 校验承接确认

LOGIC.md §1.1 登记缺的 tag-7 Wall 复合编辑校验**已由本产出承接**（`mod.rs
validate_for_recording`，此前已存在，本产出保持并更新变体面）：

- **变体 0 slice 校验**（`Slice start_u...`）：src `StateChange.observed_operation`
  = `{op_tag u32, start f32, end f32}` ↔ 原生 `@T+0xe8 u32` / `@T+0xec` / `@T+0xf0`。
  逐字段对账 ✓（`op_tag < 2` ↔ `u32[+0xe8] < 2`；`start >= end` ↔ `f32[+0xf0] <= f32[+0xec]`）。
- **变体 1/2 source 校验**（`None is not allowed in CreateWall/DeleteWall`）：src 以
  `source_66: Option<u8>` 为结构最近拟；原生 `Option<source> @T+0x08` 为 walls authority
  快照 Option——**字段面差一**，登记 §4。
- 变体 3/5..16 无入史断言（原生 add 不校验）——src 补 `{}` 分支一致。

---

## 6. 汇总

- **布局定案**：`WallHistoryEdit` 0xf8B / niche 判别表（17 变体，EnclosureChange 判别
  修正为 3）+ 变体 0 slice 三字段偏移 / 变体 1/2 Option source / 变体 3 height 三字段。
- **ConcurrentEdits 语义定案**：单个并发编辑组 = `Box<[Arc<WallHistoryEdit>]>`，
  `Vec<ConcurrentEdits>` 组序列插入序保留；Rust `type ConcurrentEdits = Vec<WallHistoryEdit>`。
- **Rust 落地**：wall_history_edit.rs 17 变体 + 判别方法 + ConcurrentEdits + 4 测试；
  4 个非 walls/collision 消费方适配；lib 全量 1850 测试 PASS。
- **登记缺类型**：变体 5..16 载荷、eraser WallChanged 的 EraserWallState 载荷、
  Option<source> 快照面、0x70B 探针输出（均 walls/collision 域或待取证，不编造）。
- **解锁状态**：walls eraser finish 所需的历史侧依赖（0xf8B 布局 + 判别表 +
  ConcurrentEdits 语义 + 5 桶→add 链结构）已闭合；erafer 各桶的 walls 域载荷类型与
  字段级装配仍归 walls restorer（`impl-walls-return`）。
