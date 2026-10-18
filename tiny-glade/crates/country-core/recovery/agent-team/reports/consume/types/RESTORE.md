# restorer-consume-types — RESTORE 报告

日期：2026-10-17。范围：`system-tree`（tree-tool）、`system-roof`、`system-decorator`。
方法：每函数 **fresh** 反编译 + 反汇编（Ghidra `--worker shared` 只读；
`work/consume-types/decomp/*_fresh.{c,asm}`、`results/fresh1|asm1|consts|msg*.json`），
重建真实控制流后与 Rust body 逐语句对照。

## 一、任务函数 verdict

| 原函数 | VA | Rust 位置 | verdict | 关键证据 |
|---|---|---|---|---|
| process_tree_cmds（Insert 分支） | 0x142267430 | `tree-tool/src/process_tree_cmds.rs` | **重建**（可表达语句判等；G4/C2 已修，余 T2/T4） | fresh asm 142267a29-142268201 + 常量直读 |
| replay_tree（Insert/undo） | 0x141bc34f0 | `tree-tool/src/replay_tree.rs` | **判等**（C1：Move-undo 旧位已实现；T2 载荷登记） | fresh decomp 行 90-219 |
| ui_move_tree（剩余） | 0x14226b1a0 | `tree-tool/src/ui_move_tree.rs` | **重建**（G5/G6/C3/C4 已修；候选源 gap 9） | fresh asm 14226b8c1 = `dragged` |
| gpu_flush_buffers | 0x140de50a0 | `roof/src/lib.rs` | **判等** | coregap-1 decomp + fresh asm |
| maintain_dormer_tiles（每瓦 GPU 退役） | 0x1407ce920 | `decorator/src/decorator_visual/cottage_dormers/maintain_dormer_tiles_draw.rs` | **判等**（G3 单槽 store 已修） | fresh asm 1407cea19-1407cea58 + 被调符号解析 |

A-toolbar/B-ecstate 复核后计数：**判等 3 / 重建 2 / 整函数未实现 0**
（余 T2/T4 登记缺口 + ui_move 候选源代理）。

## 二、逐函数说明与判等依据

### 1. process_tree_cmds Insert 分支（0x142267430，重建）

fresh 事件布局（元素 base；Bevy 分叉 `EventInstance` 头 8B，故相对事件 base）：

| 偏移 | 字段 | 依据 |
|---|---|---|
| +0x00/+0x08 | name String ptr/len | asm 142267ad2/142267b44 读 `[R12+8]/[R12+0x10]` |
| +0x10/+0x14/+0x18 | position x/y/z | asm 142267cdf-142267d49 |
| +0x1c | scale | asm 142267a64、142267be8 |
| +0x20 | rotation | asm 142267ce6 / 142267dba |
| +0x24 | variant | asm 142267dc8 |
| +0x28 | flags | asm 142267c11、142267de2 |
| +0x30 | id | asm 142267c0c、142267df1 |
| +0x38 | height | place_tree 写侧 asm 142269dcf |

已判等语句：
- 名称类别门 5 串（常量直读 0x143034d30/d50/d60/f28/f4f）：
  `clearing/naked_tree/{large, naked1, naked2, naked1_no_mushrooms,
  naked2_no_mushrooms}`。
- `deny_load`（0x140a9fa70）门；`scale' = max(scale,0.7)` 仅 `naked && deny_load`
  （MAXSS 常量 0x1429cd030=0.7；asm 142267be3-142267c08）。
- α = `inverse_lerp(0.5,0.9,scale)*0.75 + (1-r)*0.55`（常量 0x142925870=0.5、
  0x142a7c88c=0.9、0x1429edb90=0.75、0x142a877a0=0.55）。
- `spawn_empty` + 4 条 insert：TreeComponent / TreeAnimation{α} /
  TreeMoveBend{x,z 打包} / TreeAmbience{x, y+scale*10, z}
  （常量 0x142a95db0={0,10}）。
- `MISSING_TREE_PREFAB_NAME`(24B) → `random_tree_prefab`；否则事件名。
- `TreeStorage::insert`（0x140a38560）：Some(previous) → 日志
  `"Tree {} already exists. Replacing it with a new tree"` + `DespawnPrefabCmd`。
- 水面 `0.0 < bilinear_sample` → `PlayTreeSoundCmd` kind=0、selector=in_water、
  extra=naked&&(deny||corrupted)、value=r（槽 stride 0x20 逐偏移对应）。

未实现-登记（分歧）：
- **T2**：共享 `InsertTreeCmd` 仅 `{id,position,scale,prefab_name}`；真实 0x48
  事件另含 rotation/variant/flags/height → 4 处写面不可表达。
- **T4**：`is_corrupted` 宿主资源容器未取证（以 false）；`random_tree_prefab`
  的 drag_started 资源面同缺。**G4 已修**：selector 原生 = 元素+0x38 qword
  （事件 id@+0x30）低字节（asm 142268146 `MOV RCX,[RDI]`，RDI=LEA[R12+0x38]），
  Rust 现写 `(cmd.id & 1)`；原「selector 不可表达」理由撤销。
- **T5（已闭合，C2）**：`SpawnPrefabCmd` 0x48 载荷已落 core（prefab/entity/
  position/scale/rotation/variant/1.0/flags[16]/mode），Insert 分支逐字段写入
  （rotation/variant 因 T2 置 0；mode = naked&&deny ? 3 : 1，drag/flags 门登记）。

### 2. replay_tree（0x141bc34f0，重建）

fresh decomp 行 90-242 逐分支复核：
- tag==8；disc 2=Delete / 1=Move / 0=Insert；仅 `elapsed==0.0 && !NAN` 重发
  （Rust `==0.0` 对 NaN 为假，等价）。
- Delete 非 undo → DeleteTreeCmd(id)；Delete undo / Insert 非 undo → InsertTreeCmd
  （name/position/scale/id，可表达字段全写）；Move 非 undo → MoveTreeCmd(id,index,
  新位置)；Place undo → DeleteTreeCmd。
- flags bit0==0 → 立即移除；bit0==1 → `delay=delay/speed`、下钳 0.05、
  `delay<elapsed` 移除、`elapsed += GameTime.delta`；索引逆序移除。
- **Move undo（C1 已闭合）**：原生用 `edit+0x14/+0x1c` 旧位重发 MoveTreeCmd；
  `TreeEditData.previous_position`（运行时字段，不进存档 6-entry 序列化）承载，
  replay 按 undo 选旧/新位。
- **T2 登记**：Insert 重建缺 height/variant/rotation/flags。

### 3. ui_move_tree（0x14226b1a0，重建）

- 门禁 `mode ∈ {Tree(0x10), Editing(None)(0)} && !DEMO`（行 101-110）。
  **G5 已修**：补 `!DEMO`（0x14362c098 覆盖字节，复用
  `ui_place_tree::DEV_OVERRIDE_IS_DEMO_MODE`）。
- 光标 y<0 水下重映射（行 288-292：`(y/-2.5)*-1.2`）——已补。
- 候选循环：`format!("move_tree_{id}")` widget（格式串 0x143035501 直读 =
  `move_tree_` + 0x1430354a0 源文件路径 piece）+ `UiState::add`。
- **状态机判等**：decomp 标 `slider3d::impl$1::into_drag_state`，fresh asm
  14226b8c1 实调 **0x14092df10 = `UiWidgetContext::dragged`**；`AL-2` 后
  `>2 → 3`：0（空闲 2）= 无动作、1/2/3 = case1/2/3，**state≥5 全部钳到 case3**
  （wave4 G6）。
- case1 记录树原位置 + `树位置 − 光标` 偏移；case2/3 目标 = 光标 + 偏移，
  `y=max(h,0)`；界内 MoveTreeCmd（槽 0x20 全载荷）。
- case2 界外 → **双事件** DisplayOutOfBorder + FeedbackInputOutOfBorder
  （行 318-341）。
- case3 界内 → MoveTreeCmd + History TreeEdit Move。
- **G6 已修**：删除原生不存在的 `cache.dragging` id 门与 case3 清除；Local 仅
  旧位置+偏移。
- 登记：候选源 `Decorators::iter_removed_ids` 仍以 `storage.iter()` 代理（分歧 9，
  状态机按 widget id 门控，非活动候选无动作）；**T3 已闭合（C3）**：History Move
  载荷 `previous_position = cache.start`（旧位）+ 新位；**C4**：MoveTreeCmd 载荷
  改为原生 0x18 `{index@+0, position@+8}`（去掉误记的 EventInstance id）。
  UiSounds 槽/2.5-1.0 widget 偏移面未承载。

### 4. gpu_flush_buffers（0x140de50a0，判等）

fresh 控制流全部可观测：`AssetSsboLibrary::get_mut<RoofTileSsbo>()` → None →
`Option::unwrap` panic → `GraphicsDeviceResource::deref`（&dyn GraphicsDevice）→
`SparseInstanceBuffer<RoofTileSsbo>::gpu_flush` → ResMut 变更 tick（Bevy
`deref_mut` 自动）。Rust 5 行照落；`RoofTileSsbo: SparseSsbo`（CHUNK_COUNT
0x2000）已具备。调度注册点维持原状（原函数属渲染 flush 面，不在 CPU 链）。

### 5. maintain_dormer_tiles（0x1407ce920，判等）

fresh asm 每瓦四步（1407cea19-1407cea58）：
1. `*cursor += 1`；2. `MOV [R13],EBP`——**id 单槽 store**（R13=state+0x10，
   循环内不变、不递增、无容量检查；wave4 G3，Rust 已由 `Vec::push` 改为
   `*id_slot = value`）；
3. `hash_one<RandomState,&GlobalId>`（**0x1405b0d70**，输入 = 瓦元素+8）；
4. `RawTable::remove_entry<u64,V>`（**0x140053320**）；
5. 返回值非空 → `drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>`
   （**0x141259000**）。
Rust：`DormerTilesDrawState.retire: hashbrown::HashMap<u64,
SparseInstanceBufferAllocation<RoofTileSsbo>>`，每瓦 `remove(+8 GlobalId)` 后
作用域 drop。

**纠错**：wave3 曾断言 0x1405b0d70/0x140053320 为“虚构变换写 helper”；本会话
`get_functions` 签名证明其为 hash_one/remove_entry，0x141259000 为分配 drop。
PDB 表值的 `Box<dyn FnOnce(*mut c_void,bool)>` 与 drop 符号不一致（ICF 别名），
本实现以 drop 符号锚定的分配类型承载。

## 三、并发类型重构适配（编译面）

core 于 21:45 由 `ui-restorer-seams` 统一 `PrefabInstanceData` 为 0x58 语义体
（无 position/scale/rotation），并新增 `PlayTreeSoundCmd` selector/extra/value。
为使 `cargo check` 通过，本模块内做等价适配：
- tree-tool：位置读写改用 `country_core::utils::components::transform::Transform`
  （`process_tree_cmds` Move/Delete、`recalc_*`、`tree_move_bend`、`ui_delete_tree`）。
- tree-tool：`PlayTreeSoundCmd` 补 selector/extra/value 三字段。
- tree-tool/decorator/roof：`WidgetParams::new(volume, extra_78)` 两参签名适配。
- decorator：删除 core 已证伪的 `InstaCreateStairPoint`/`InstaCreateDeco` 参数
  （fresh 无此事件）；移除 `DecoratorOpEventKind::Place/Remove/Change` 旧近似桥；
  `DecoratorHistoryEdit` 改为 diffs map 模型后，48B 运行时载荷映射标未实现（返回空 edit）。

## 四、wave4 复核响应（G2-G6）

| FIX | 处置 |
|---|---|
| G2 replay Move-undo | 当时降级并标未实现；**后由 C1 闭合**（`TreeEditData.previous_position` + 旧位事件），verdict 回判等。 |
| G3 dormer id 写面 | `Vec::push` → `*id_slot = value` 单槽 store（R13 循环内不变）；`ids` 字段改为 `id_slot: *mut u32`（null 安全 seam）。 |
| G4 Insert selector | `random_tree_prefab(0,…)` → `(cmd.id & 1) as u8`（元素+0x38=id 低位）；T4 selector 理由撤销，仅 `is_corrupted`/drag_started 保留。 |
| G5 ui_move_tree DEMO 门 | 补 `if DEV_OVERRIDE_IS_DEMO_MODE { return; }`（0x14362c098）。 |
| G6 ui_move_tree 状态机 | 删除 `cache.dragging` id 门/case3 清除；`AL-2` 后 `>2 → 3` 复刻 state≥5 全部钳到 case3。 |

gpu_flush_buffers 判等维持（复核通过）。maintain_dormer_tiles G3 修正后维持判等。

## 四之二、A-toolbar / B-ecstate 复核响应（C1-C4）

| FIX | 处置 |
|---|---|
| C1 replay Move-undo 旧位 | core `TreeEditData` 增运行时字段 `previous_position`（`#[serde(default)]`，**不进入** 0x14003d4e0 的 6-entry 手写序列化）；replay Move 分支按 `undo` 选旧/新位发 MoveTreeCmd。verdict 回**判等**（T2 Insert 载荷仍登记）。 |
| C2 SpawnPrefabCmd 载荷（T5） | core `SpawnPrefabCmd` 由 `_gap` 扩为 0x48 载荷（prefab Arc / entity / pos / scale / rotation / variant / 1.0 / flags[16] / mode）；process Insert 逐字段写入；forest 调用点改 Default 并登记映射 U。 |
| C3 ui_move History 旧位 | `TreeEditData.previous_position = cache.start`（旧位）+ 新位；T3 闭合。 |
| C4 MoveTreeCmd 布局 | core 去多余 `id`：payload 0x18 `{index u64@+0, position Vec3@+8}`；replay/ui_move/process 三处同步。 |

MoveTreeCmd 生产/消费同结构自洽，且字段数/顺序与原生 0x18 载荷一致。
其余登记维持：T2（InsertTreeCmd rotation/variant/flags/height）、T4（is_corrupted/
drag_started 资源）、ui_move 候选源代理（gap 9）、WidgetParams sounds/volume。

## 五、编译

`cargo check -p system-tree -p system-roof -p system-decorator`：**通过**
（0 error；告警为此前既有：tree/roof/decorator 零新增语义告警）。

## 六、证据索引

- `work/consume-types/results/fresh1.json`（6 函数 fresh 反编译 + 5 函数反汇编 + 常量）
- `work/consume-types/results/asm1.json`（逐函数反汇编）
- `work/consume-types/results/callees.json` / `dormer_callees.json`（0x1405b0d70 /
  0x140053320 / 0x141259000 等符号解析）
- `work/consume-types/results/consts.json` / `msg*.json`（0.5/0.9/0.7/0.75/0.55、
  {0,10}、名称表与日志串直读）
- 复核证据：`work/reviewer-wave4-geometry/evidence/`（reviewer 独立 fresh）+
  `reports/wave4/reviews/geometry/REVIEW.md`。
- 全部 Ghidra 调用 `--worker shared` 只读，无写库。
