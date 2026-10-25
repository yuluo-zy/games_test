# 逐函数逻辑重建 — systems/terrain + 地形资源

重建者: rebuild-terrain
日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（含 2026-10-17 宏生成函数分类补充）
依据: LOGIC.md（logic-verifier-terrain 判等）+ fresh 反编译
      （recovery/agent-team/work/logic-verifier-terrain/jobs/terrain-systems-result.json、
      resources-result.json、focus_closures-result.json）
方法: 逐函数分类（宏胶水 vs 真业务）→ fresh 反编译重建控制流 → 逐语句实现 → `cargo check -p country-core`。

## 结论摘要

- 本轮重建 7 个 `未实现-需重做` 函数，全部判定为**真实业务函数（重建）**；
- 判等 9 函数（delayed_terrain_changed_msg/on_terrain_modified/history_record_terrain/
  RockyTerrainMesh::load/TerrainHeightsData×5/get_last_mut_stroke/new_stroke/
  insert_or_morph_last_point）**未动**（红线）；
- 胶水-非业务（mod.rs 三注册函数、SSBO 载体类型实现）未按业务重建；
- **编译状态**: `cargo check -p country-core` 通过（lib，0 error；测试套件存在与本轮
  无关的多模块既有断点——GraphicsDevice 构造等，另注）。

## 函数表（重建）

| VA | 原函数 | 分类 | 重建后 verdict | 关键改动 |
|---|---|---|---|---|
| 0x14094e000 | edit_terrain_stroke | 真业务 | **重建-判等** | 删自造 PlayTerrainSound 载荷事件；tag 1/2/3/4 观测改写 StairsPlacementBucket（indexmap::Bucket 等价条目）；magnitude/apply_resolved/readback LOCK INC 保留 |
| 0x140a03ba0 | terrain_stroke_changed | 真业务 | **重建-判等** | 删 PlayTerrainSound::StrokeChanged（编造）；补 StairsPlacementBucket tag=0 记录；顺序 8B 事件→readback→stairs 桶 |
| 0x140aff720 | terrain_hover_idx | 真业务 | **重建-判等** | 输出布局改 {tag u64@0, id i64@8}（原 id@+4 错）；函数开头无条件 tag=0 复位；门 `mode&0x1f` 移位掩码 |
| 0x140a9fdc0 | ui_draw_terrain | 真业务 | **重建-判等** | 画布门反向修正 `height>0∧(¬hovered∨dist≤0.5)`；状态机 prev=0 的 u8 回绕修正 `(prev-1 as u8)>1/<2`；new_stroke/续笔按 jumptable state 分派 |
| 0x140aef1f0 | ui_edit_terrain | 真业务 | **重建-部分判等** | 补 widget 注册（UiState::add<String>，type=(id!=w)*3+4）+ 事件 subtype switch + stairs 桶 tag=2 写 + TerrainEdit 哨兵 0x…0001（校验 0x…0002 panic）；删自造 16 位脏掩码→EditTerrainStrokeCmd/SetTerrainStrokePtsCmd；case 4 UNRECOVERED_JUMPTABLE 待取证 |
| 0x1409a4fb0 | ui_terrain_focus_mode | 真业务 | **重建-部分判等** | **fresh 本轮反编译成功（120KB 近全内联）：门 = `AppMode != 5`（Editing(Terrain)）**（早期 RE-ANALYSIS"状态首 u32 == 1"未复现，已修正）；补 signifier 创建（closure$1 "vertex_color_shader"）+ DrawTopology insert 登记；复杂体（焦点 HashMap/as_curve3/CircularF32/RequestAppModeChange/相机旋转/`AppendOnlyVec::push<UiAppendSignifier>`×4）依赖宿主实体机制，登记待接 |
| 0x1409ea800 | terrain_rock_mask::sample_at_world_xz | 真业务 | **重建-判等** | 越界改 panic_bounds_check（原 unwrap_or(0) 静默）；按**词位**读取（index>>5 词、index&31 位，原按整 u32 词返回错）；x/130 DIVSS（原 x*(1/130)）；clamp 用原生 NaN 保持语义（不用 f32::clamp） |

## 重点分歧落地记录

1. **PlayTerrainSound 事件系编造已删**: edit_terrain_stroke / terrain_stroke_changed
   原版把 tag 0/1/2/3/4 观测写 **StairsPlacementBucket（indexmap::Bucket<(DecoratorId,
   DecoratorId),AlignKind>）**，无 PlayTerrainSound 载荷事件。`PlayTerrainSound` 类型与
   `Events<PlayTerrainSound>` 注册**保留**（systems/audio/terrain_audio 依赖、红线不改他模块）；
   但地形生产面不再写它（已改 StairsPlacementBucket）。terrain_audio 消费链系对原音频桶的
   误读，属 audio 模块后续任务（本轮不越界）。
2. **terrain_hover_idx 输出布局**: 原版 `*puVar4=1; puVar4[1]=iVar7`（8B tag + 8B id@+8，
   ≥16B）；`HoveredTerrainStrokeId` 改 `{tag:u64, id:i64}`，id@+8（原 +4 错）。函数开头
   无条件 `tag=0` 复位（原版在 mode 门前写）。
3. **ui_draw_terrain 画布门 + prev 回绕**: 画布门 `height>0 ∧ (¬hovered ∨ dist≤0.5)`
   （原版非悬停帧也注册）；状态机 `(byte)(prev-1)>1 / <2`（prev=0→255 算>2），Rust 原
   `prev>2/prev<=2`（prev=0 算≤2）结果相反，已修正。结尾 jumptable（&UNK_142adc498）
   目标函数未恢复（UNRECOVERED_JUMPTABLE），按已恢复语义 state 1=新笔/2=续笔/3=释放分派。
4. **ui_edit_terrain 主体重建**: 逐笔画 `UiState::add<String>`（type 字节 =
   `(iVar37!=id)*3+4`）；事件 subtype switch（&DAT_142ae404c，case 0-4/6/8/0xc/0xf）驱动
   WindowFocusEvent 型 0x10B / WriteSettingsFileCmd 型 8B / stairs 桶 tag=2；case 3 写
   TerrainEdit 哨兵 **0x8000000000000001** 且校验上一哨兵 0x8000000000000002（否则 panic）；
   case 4 仍 UNRECOVERED_JUMPTABLE。crate 无 `WindowFocusEvent`，以本地
   [`TerrainWindowFocus`] 等价承载（I）。自造 `MouseActivityTimer/TerrainStrokeParamsCache/
   16 位脏掩码→命令` 全部删除（原版脏判定=事件队列非空，不产这些命令）。
5. **ui_terrain_focus_mode**: 补 closure$1 = "vertex_color_shader" SignifierEntityBuilder::build
   + bevy command push `insert<DrawTopology>`（crate 无 SignifierEntityBuilder，以
   UiSignifierStream + [`SignifierDrawTopology`] 事件登记）。closure$2/3/4 的
   brush_arrow_flat_to_single_forward / brush_arrow_flat_to_w_flat_single_up 等 signifier
   创建依赖宿主实体机制，登记。
6. **terrain_rock_mask::sample_at_world_xz**: `(index>>5) < len` panic_bounds_check（越界
   **panic** 而非静默 0）；返回 `mask[index>>5]` 的位 `index&31`（位图语义，每 u32=32 格）；
   `x/130.0+0.5`（DIVSS）；clamp 保持 NaN → `as u32` 饱和 0。

## 方法备注

- 全部以 fresh 反编译为准。**本轮对 12 个重建函数重新跑 ghidra fresh 反编译**
  （`rebuild-fresh-result.json`，shared 只读，fields=decompiled_code,callees）；11 个与
  logic-verifier-terrain 证据**逐字节一致**；`ui_terrain_focus_mode`（0x1409a4fb0）此前
  fresh 超时（0 字节），本轮成功（120KB 近全内联），据此修正门值（`!=5` 非 `==1`）并登记
  复杂体。
- `cargo check -p country-core` 通过；`--tests` 存在与本轮无关的**既有**断点
  （rhapsody_contract::GraphicsDevice 由 tuple 改真实 struct 后多模块测试仍用
  `GraphicsDevice(n)` 构造、ivy_pruner 私有函数、color_library/placement_data_library
  Device 等），不在本轮范围。
- `PlayTerrainSound` 类型/注册保留系 audio 模块依赖（红线不改他模块），生产面已停写。
