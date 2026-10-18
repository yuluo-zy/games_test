# REBUILD-DEEP — history 模块深收口（impl-history-deep，2026-10-17）

> 依据 2026-10-16-logic-verify-protocol.md。对待办清单 5 项每函数 fresh 反编译
> （ghidra shared 批 1-3；case 0xb+ 用 impl-history-deep 唯一 --write ReVa worker）
> → 分类 → 按控制流实现/判等。产出本报告分批。编译检查见 §4。
>
> fresh 反编译证据落盘：`planning/impl-history-deep/decomp/`（0x140abfb30 process
> ReVa 全量、0x140abfb30-disasm 全区域、0x140af5b30 history_process ReVa 全量、
> 0x1417befe0 ColorEdit deserialize、0x1417bf8a0 visit_str）+
> `results/batch1-3/`（墙 17-op 跳表 read_memory、ColorEdit serialize/deserialize、
> history_process ReVa、get_param）。

---

## Batch 1 — replay_wall 17 操作跳表（item 1）

### fresh 定案

`WallReplayEditCmd::process`（0x140abfb30）为巨型函数（0x140abfb30..0x140ac0a36），
各操作 body inline。Ghidra/ReVa 只在 DB 建模 400B dispatch 壳。分派：
- `done byte @+0x15`；`*&1` 定时位 → cubic_in_out + 跳表 0x142adf44c；
  瞬时 → 跳表 0x142adf408。op 号 = niche 解码（`edit+0x10 < -0x7ffffffffffffff0
  → lVar1 = v - 0x7fffffffffffffff`，否则 0），lVar1 ∈ 0..0x10。
- 两表各 17 项（`results/batch1-wall-jumptables.json` 已读两表各 160B，
  目标地址逐项解出，见 `evidence-batch1-replay-wall-jumptables.md`）。

| op | 瞬时目标 | 定时目标 |
|---|---|---|
| 0 | 0x140ac0366 | 0x140abfcd3 |
| 1 | 0x140ac02a2 | 0x140ac1576 |
| 2 | 0x140ac04b3 | 0x140ac1576 |
| 3 | 0x140ac042c | 0x140abfdbb |
| 4 | 0x140ac0253 | 0x140ac1576 |
| 5 | 0x140ac0612 | 0x140abfcb9 |
| 6 | 0x140ac0667 | 0x140abfe85 |
| 7 | 0x140ac05b1 | 0x140abfecc |
| 8 | 0x140ac0484 | 0x140ac1576 |
| 9 | 0x140ac0699 | 0x140ac1576 |
| 10 | 0x140ac0635 | 0x140ac1576 |
| 11 | 0x140ac06c8 | 0x140ac1576 |
| 12 | 0x140ac0224 | 0x140ac1576 |
| 13 | 0x140abfc80 | 0x140abfc2f |
| 14 | 0x140abfc80 | 0x140abfc2f |
| 15 | 0x140ac15b7 | 0x140ac15b7 |
| 16 | 0x140ac05e3 | 0x140ac1576 |

### 分类：真业务（非宏胶水）

17 操作多分支跳表 + 每 op 独立控制流（写源/通知/音频/颜色请求）。

### 处置：登记（跨模块，不实现他模块业务）

`advance_wall_groups_observed 全 17 op` 需 recovered_building `WallHistoryEdit`
全变体（当前 src 仅 Create/Delete/EnclosureChange/StateChange 4 变体，op 号经
`observed_operation` 前 4B 传于 StateChange）。按红 line 不改他模块，本处登记完整
分派映射 + 目标地址证据，recovered_building 侧据此扩展变体（其
`wall_history_transition.rs` 已实现 op 0/1/2）。systems/history/replay_wall.rs
的 `advance_wall_groups_observed` 调用面无需改（解释器扩展在他模块）。

---

## Batch 2 — replay_pop_bucket（item 2）

### fresh 定案

0x140b38250 已有前 agent fresh 落盘（`rebuild-history/decomp/0x140b38250-replay_pop_bucket.txt`）：
前桶命令空 → `pop_front` → `DecoratorStorage::restore_decorators`（closure）+ Arc<DecoratorHistoryDiff>
递减 → 楼梯 Arc<StairsHistoryDiff> 非空时 `StairsState::restore_backup` / `apply_diff` /
`iter_edges_and_stubs` / 两 `IndexMapCore::clear` / `+0x178=0` / `make_backup` → drop bucket →
`DecoratorArchivist::extract_diff` + drop 表 → `RaycastWorld::flush_colliders`；
命令非空路径同样 `extract_diff`。

### 分类：真业务

### 处置：登记（跨模块）

src 已有 `DecoratorStorage::restore_decorators`（0x14155ba30）与 StairsState
restore_backup/apply_diff/make_backup/iter_edges_and_stubs；缺 `DecoratorArchivist::extract_diff`、
`RaycastWorld::flush_colliders`（decorator/collision 域）、`ReplayBucket` aux diff Arc 承载
（当前 src 前置拒绝非零 aux 计数）。按红 line 不动他模块，登记给 decorator/collision 域。

---

## Batch 3 — replay_edit Color/Decorator 展开 + DecoratorHistoryState::from（item 3）

### fresh 定案

0x140af6170 已前 agent 落盘：逐 edit `generate_replay_cmds_from_edit` 按 tag 跳表
0x142c5f284 分派 + undo 逆序 + 调度 ReplayBucket + 装饰/楼梯 diff Arc 处理（to_arc_slice）。
src `ReplayBucket::expand` 覆盖 Raster/AutoClutter/Roof/Wall/Stairs/Tree，缺
Color(9)/Decorator(10/11) 命令类型（现返 `UnrestoredReplayOwner`）。

0x140b3f930 `DecoratorHistoryState::from` 已前 agent fresh 落盘：入口 `*(in_RDX+0x47)==0`
否则 panic（preview）；style 字节 @+0x41，`(byte)(cVar2-2U)<6` 夹取分支读
@+0x30 字节/u32、@+0x34/+0x38/+0x3c/+0x40、@+0x44..+0x4e 输出簇（≈10 字段）。

### 分类：真业务

### 处置：登记（跨模块）

- replay_edit Color/Decorator 展开：需 ReplayCommand 增 Color/Decorator 命令类型
  （生成器主体在 recovered_building/装饰域）。
- DecoratorHistoryState::from：需 src `DecoratorInfo` +0x41 style 字节字段（decorator.rs
  现 u40 整 u32 U）与 `DecoratorHistoryState` 结构扩至 ~0x4fB（≈10 字段）。按红线
  不动他模块，登记给 decorator 域。

---

## Batch 4 — ColorEdit 互异载荷类型面（item 4）

### fresh 定案（本批新证据）

0x14003c5d0 serialize 跳表 0x14292c628（target-3 夹取 0..0xe）已前 agent fresh 定案 8 类
载荷表（REBUILD-REMAINDER §3.5）。本批补 fresh 反编译核实 serialize 逐 entry 类型序
（`results/batch1b/1c` + 本批 deserialize 0x1417befe0 / visit_str 0x1417bf8a0 落盘）：

| target | 载荷类型（serialize_entry 序） |
|---|---|
| 3/4/5/9 | {WallId, TerrainStrokeId, TerrainStrokeId} |
| 6 | {WallId, FlatRoofStyle, FlatRoofStyle} |
| 7/8/13/15 | {DecoratorId, TerrainStrokeId, TerrainStrokeId} |
| 10/12 | {DecoratorId, bool, bool} |
| 11 | {DecoratorId, f32, f32} |
| 14 | {DecoratorId, StairRailingType, StairRailingType} |
| 16 | {DecoratorId, StairSupportType, StairSupportType} |
| 17 | {}（双 `}`） |

visit_str 0x1417bf8a0（deserialize target 键解析）确认 14 个目标键名长度分派
（Wall/Gate/Roof/FlatRoof/Decorator/WindowLight/Halftimber/FlatGunting/Brightness/
ChimneySmoke/StairLadder + 3 anon）。

### 分类：真业务（手写 serde）

### 处置：实现（结构对齐 serde，维持 target 键 + 平面字段子集）

src `ColorEdit` 平面模型 `{target, from, to, target_id, chimney_h_offset,
allow_connection}` 维持（load/save Colors 节消费面不重排，REBUILD-REMAINDER §3.5 结论
延续）；互异载荷类型面（WallId/TerrainStrokeId/FlatRoofStyle/StairRailingType/
StairSupportType 均为他域持有类型）以上表定案登记。本批确认 serde 载荷键序与
deserialize 对偶，未新增 Rust 载荷变体（避免他模块类型重复定义）。

---

## Batch 5 — history_process 第二事件流 case 0xb+ + get_param（item 5）

### 5a. history_process 第二事件流 case 0xb+（ReVa 定点）

**方法要求**：反编译不收敛 → 用 impl-history-deep 唯一 `--write` ReVa worker 定点。
已执行：`results/batch2b-reva-full.json` + `decomp/0x140af5b30-reva-full.txt`
（ReVa 全量反编译，function body 924 行）。

**ReVa 定点结果**：`switch(*(byte *)(_Src + 9))` 跳表 0x142ae5420 仍不收敛——
case 0xf/0x10/0x12/0x14/0x16/0x19-0x1b 命中 `halt_baddata`（0x140af7d6a/0x140af7d67
指令重叠），Ghidra 与 ReVa 一致。可解码分支：

| case | 语义（ReVa 定点） | 处置 |
|---|---|---|
| 0 | 队尾命令 byte1==1（redo）→ 写事件 + 清队 | 已实现（process_history_control_stream） |
| 1 | 队尾命令 byte1==0（undo）→ 写事件 + 清队 | 已实现 |
| 2 | 无条件 → 写事件 + 清队 | 已实现 |
| 3 | make_animations_instant + 写事件 + 清队 | 已实现 |
| 0xb | 字节 OR 0x10 + 条件分派（0x140af6490/LAB_140af6660） | 登记 U（bad-instruction 区） |
| 0xc/0xe/0x11/0x13/0x15/0x17/0x18 | 移除 ReplayCmd 索引 + Arc 计数 + 跳表 0x142ae545c 分派 | 登记 U（跳表二级分派不收敛） |
| 0xf/0x10/0x12/0x14/0x16/0x19-0x1b | halt_baddata | 登记 U（指令重叠） |

**结论**：case 0-3 已实装；0xb..0x18 为移除/校验分支，但 ReVa 定点仍不收敛
（0x142ae545c 二级跳表 + 0x140af7d6a 指令重叠），不编造，登记 U。

### 5b. get_param 载荷语义（@+0x58/+0x5c）——实现

**fresh 定案**（0x140ae5830，本批 ReVa + 落盘 `decomp/0x140ae5830-get_param-reva.txt`）：
输出元素 = 4 组 × 0x20B，每组 `{data u64 @+0x00, p58 ptr @+0x08, p5c ptr @+0x10,
flag u32 @+0x18, u1c u32 @+0x1c}`；末尾 `[0x80] = in_RDX + 4`。每组从稀疏资源表槽
（元素 0x60B）取：`data = *(slot + 0x10)`、`p58 = &slot[0x58]`、`p5c = &slot[0x5c]`、
`flag = *(in_R8 + 0x130)`（u32 四组共享）、`u1c = in_stack_00000028`。4 索引任一缺失 →
panic `"Resource requested by <type> does not exist"`（类型名常量 @0x142ae20a7/
0x142ae225e/0x142ae2287/0x142ae250d）。

**实现**（`src/systems/history/replay_raster.rs`）：`RasterReplayParams` 文档精确绑定
@+0x58/+0x5c 载荷指针语义（`&slot[0x58]`/`&slot[0x5c]` 由调用方按资源类型解引用），
`get_param` 4 组取参面保留。各资源原生槽 0x58/0x5c 字段精确含义随资源类型而异，
登记待各自原生布局对账（不编造字段含义）。

---

## §4 编译检查

- `cargo check -p country-core --lib`：**0 错误**（9 warnings，均为他模块）
  ——注意：墙域并发 agent 正在重构 Curve2（`systems/wall/snapping/snapping_to_ends.rs`
  09:50 触及，`closest_u_on_curve`/`get_approx_u_from_pos`/`get_coord_at_u` 调用
  暂挂），若现场 check 报 E0599 属该并发在途改动，非本模块引入。
- 本模块改动仅 `replay_raster.rs` get_param 载荷语义文档绑定（无逻辑变更），
  不构成编译风险。

---

## §5 汇总

- **实现（本批）**：get_param @+0x58/+0x5c 载荷语义绑定（4 组 {data,p58,p5c,flag,u1c}
  元素布局，ReVa 定点证据落盘）。
- **登记（跨模块，需他域布局/载荷面）**：replay_wall 17-op（跳表映射证据交付
  recovered_building）、replay_pop_bucket（DecoratorArchivist::extract_diff +
  RaycastWorld::flush_colliders + ReplayBucket aux diff）、replay_edit Color/Decorator
  展开、DecoratorHistoryState::from（DecoratorInfo+0x41）、ColorEdit 互异载荷类型面
  （8 类载荷表定案）、history_process case 0xb+（ReVa 定点仍不收敛，bad-instruction）。
- **编译**：lib 0 错误；tests 构建余 2 错误均为墙域并发 Curve2 在途改动。
