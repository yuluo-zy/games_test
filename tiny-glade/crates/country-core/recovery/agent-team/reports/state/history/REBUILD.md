# REBUILD — resources/history + systems/history 真实逻辑重建

> restorer: rebuild-history，2026-10-17。依据 2026-10-16-logic-verify-protocol.md。
> 对 LOGIC.md 标 `未实现-需重做` / `未实现（无 body）` 的函数先做 **宏生成 vs 真实业务 分类**，
> 再按 fresh decomp（rebuild-history/decomp/，shared worker，batch 1-2，fields=decompiled_code,callees）
> 重建控制流并逐语句实现。红线遵守：不删除判等函数、不改他模块业务。
>
> 编译：`cargo check`（lib）0 错误；`cargo check --tests` 本模块 0 错误
> （测试构建余 55 错误均为**他 agent 并发在改**的 rhapsody_contract GraphicsDevice/
> game_profile/mod_key/asset_libraries 等无关模块，非本模块引入）。

---

## 1. 分类结论（宏胶水 vs 真实业务）

对每个 `未实现-需重做`/`未实现（无 body）` 函数：

| VA | 函数 | 分类 | 判定依据 |
|---|---|---|---|
| 0x140a84ec0 | `History::add` | **真业务** | 真实控制流：pending 判空、nuke、push；**tag-7 Wall 逐操作 niche 断言**。 |
| 0x140a84260 | `try_from`（resolved 谓词） | **真业务** | `[*+0x10] < 2` 过滤规则，非 derive。 |
| 0x140a84a50 | `closest_resolved_snapshot` | **真业务** | 过滤谓词 + 无分支二分选择；选择算法已判等。 |
| 0x14003c5d0 | `ColorEdit::serialize` | **真业务（非宏胶水）** | 手写 `serialize`，按 ColorTarget 跳表写**互异载荷**（WallId+TerrainStrokeId / FlatRoofStyle×2 / DecoratorId+f32 / …）；derive 无法覆盖。 |
| 0x1404f46e0 | `DecoratorHistoryEdit::serialize` | **真业务（非宏胶水）** | 手写单 entry map `serialize_entry<str, __SerializeWith>` 惰性写嵌套 diff。 |
| 0x140b3f930 | `DecoratorHistoryState::from` | **真业务** | 完整 info→state 转换（style 字节分支 + is_preview panic），非 derive。 |
| 0x1404896f0 | `RasterEdit::serialize` | **真业务（非宏胶水）** | 手写单 map 四 entry（points Vec3 序）。 |
| 0x1416e8510 | `RasterEdit::deserialize` | **真业务（非宏胶水）** | 手写 `deserialize_struct<__Visitor>` + Arc 建载荷。 |
| 0x140a976b0 | `RasterEdit From` | **真业务** | Arc 切片→RasterEdit 逐字段复制。 |
| 0x14003d4e0 | `TreeEditData::serialize` | **真业务（非宏胶水）** | 手写单 map 六 entry（position→scale→rot→**i32 id**→prefab→is_preplaced）。 |
| 0x140259af0 | `StairsHistoryEdit::serialize` | **真业务（非宏胶水）** | 手写，`*in_RCX & 1` 判别单边嵌套 map。 |
| 0x140a18ee0 | `StairsHistoryEdit::deserialize` | **真业务（非宏胶水）** | 手写 visitor（struct_variant 状态机）。 |
| 0x140af5b30 | `history_process` 第二事件流 | **真业务** | 16B 记录 byte[+9] 跳表 0x142ae5420（case 0..0x17）。 |
| 0x140af6170 | `replay_edit` | **真业务** | 逐 edit `generate_replay_cmds_from_edit` 按 tag 跳表分派。 |
| 0x140b38250 | `replay_pop_bucket` | **真业务** | 前桶清空 → pop + restore_decorators + Stairs restore/flush_colliders。 |
| 0x1408b36a0 | `finish_replay` | **真业务** | AppMode/RestoreAppTo/Toolbar/MetaCommander 哨兵弹栈。 |
| 0x140a7c550 | `replay_mode` | **真业务** | AppMode 进出 0xd + HoldToFastReplay 0.2s + clearing_canvas + time_continuous_lerp。 |
| 0x140af75b0 | `replay_stairs` | **真业务** | 动画门控 + 边集增删 + 装饰/音频。 |
| 0x140951b80/0x140abfb30 | `replay_wall`/`WallReplayEditCmd::process` | **真业务** | 17 操作跳表分派（cubic_in_out）。 |
| 0x140ae5830 | `get_param` | **真业务** | 4 索引稀疏表取参否则 panic。 |
| 0x1409e6210 | `missing_prefab` | **真业务** | 24B String 常量返回。 |

**结论：全部 21 个函数均为真实业务，无宏生成胶水**。serde 族（Color/Decorator/Raster/Tree/
Stairs）为手写 `Serialize`/`Deserialize`（含真实字段策略/状态输出），按协议 §3 属"按业务重载"，
不是 derive 可覆盖的宏胶水。

---

## 2. 重建实现

### 2.1 `History::add`（0x140a84ec0）—— tag-7 Wall 复合编辑校验分支

fresh decomp 确认 push 前分支：

```c
if ((*in_RDX == 7) && (*(longlong *)(in_RDX + 4) != 0)) {
  plVar6 = *(longlong **)(in_RDX + 2);            // groups ptr
  plVar4 = plVar6 + *(longlong *)(in_RDX + 4) * 2;
  do {
    if (plVar6[1] != 0) {                         // group.len
      do {
        lVar2 = *(longlong *)(*plVar6 + lVar5);   // edit ptr
        lVar7 = *(longlong *)(lVar2 + 0x10) + -0x7fffffffffffffff;  // niche 解码
        if (-0x7ffffffffffffff1 < *(longlong *)(lVar2 + 0x10)) lVar7 = 0;
        if (lVar7 == 0) { if (*(uint*)(lVar2+0xf8) < 2 &&
           (*(float*)(lVar2+0x100) < *(float*)(lVar2+0xfc) ||
            *(float*)(lVar2+0x100) == *(float*)(lVar2+0xfc)))
             panic("Slice start_u must be smaller than end_u..."); }
        else if (lVar7 == 1) { if (SBORROW8(0, *(longlong*)(lVar2+0x18)))
             panic("None is not allowed in `CreateWall`..."); }
        else if (lVar7 == 2 && SBORROW8(0, *(longlong*)(lVar2+0x18)))
             panic("None is not allowed in `DeleteWall`...");
      } while (plVar6[1] << 3 != lVar5);
    }
    plVar6 = plVar6 + 2;
  } while (plVar6 != plVar4);
}
```

实现（`history_transactions.rs` + `resources/history/mod.rs`）：
- 新增 trait `HistoryRecordingValidation { fn validate_for_recording(&self) {} }`，泛型 `add`/
  `add_with_nonce_provider` 加 `M: HistoryRecordingValidation` 约束，push 前调用
  `edit.validate_for_recording()`（nuke 分支之后，与原版顺序一致）。
- `impl HistoryRecordingValidation for EditType`：仅 `EditType::Wall(compound)` 逐 group/逐 edit
  校验；`Create`/`Delete` 断言 source 存在（src 结构最近拟 = `metadata.source_66.is_some()`，
  与 replay_wall `MissingSource66` 同一缺口面），`StateChange` 解 `observed_operation`
  `{op_tag u32, start f32, end f32}`，`op_tag<2` 时 `start>=end` panic；`EnclosureChange`
  niche 0x8000000000000004 不在 0/1/2 校验面 → 无检查（与原版一致）。
- no-op impl：`RasterStroke`、`Arc<RasterStroke>`、`RoofRimHistoryMutation`、
  `BuildingEdit`（后者由 wrapper 层 `UnresolvedOwner` 拒绝承接）。
- panic 文案按字符串常量（0x142ad8118 / 0x142ad8170 头 / 0x142ad81c8）：
  - `"Slice start_u must be smaller than end_u. Otherwise an invalid curve will be created!"`
  - `"None is not allowed in \`CreateWall\` history command recording"`
  - `"None is not allowed in \`DeleteWall\` history command recording"`
- 新增测试：`wall_compound_with_present_source_passes_add` /
  `create_wall_without_source_66_panics` / `state_change_with_degenerate_slice_panics` /
  `non_wall_edits_pass_without_validation`。

### 2.2 `replay_stairs`（0x140af75b0）—— 动画门控下限 0.05

fresh decomp 确认：

```c
fVar44 = *(float *)(cmd + 0xc) / speed;   // duration / speed
if (fVar44 <= 0.05) fVar44 = 0.05;        // 下限 0.05（非旧登记 0.1）
if (elapsed <= fVar44) { elapsed += dt; continue; }  // 先判后加
```

`stairs_history_edit.rs`：`REPLAY_DURATION_FLOOR: f32 = 0.05`（原 0.1 修正）。
`StairsReplayCmd::advance` 已实现 `(duration/speed).max(FLOOR)` + 先判后加，逐句一致。
（楼梯域并发 agent 同步按机器码对齐双路径 instant/动画 undo 语义与 `StairsRemovedSupports`
全边键 `StairGraphEdge`——本处已对齐其 API。）

### 2.3 `finish_replay`（0x1408b36a0）—— 实装系统本体

fresh decomp 核心流程（AppMode==0xd 且双队列空）：

```c
if (*(int *)*in_RCX == 0xd) {
  ...（queue drain）
  if ((uVar8 != 0) || (pending非空)) goto meta_check;      // 非自然结束跳过恢复
  *(undefined8 *)*in_RCX = restore_to;                     // RestoreAppTo 写回 AppMode
  if (restore_tag∈{5,20} 特判) { toolbar +0x18 = 7; }      // CloneClutter → Editing
  else resources::toolbar::Toolbar::switch_mode();
meta_check:
  if (MetaCommand 队首 tag != -0x7fffffffffffffff) return; // 哨兵 = 0x8000000000000001
  log("History replay has naturally finished. Unblocking game commander.");
  pop_front; drop_in_place<Option<MetaCommand>>;
}
```

实现（`finish_replay.rs`）：
- `is_replay_naturally_finished(mode, replay, pending)` = `AppMode==13 && replay 空 && pending 空`。
- `finish_replay` 系统：`ResMut<AppMode>` + `ResMut<RestoreAppTo>` + `ResMut<Toolbar>` +
  `ResMut<MetaCommander>` + `Res<ReplayBuffer>` + `Res<PendingHistoryCommands>`。
  - 双空 + Replay：`*mode = restore.0`；restore tag==20（CloneClutter，无工具栏类别）
    → `toolbar.current = ToolbarCategory::Editing`（原 `+0x18=7`）；否则 `toolbar.switch_mode(&restore)`。
  - 随后（回放/非回放两路径）检查 MetaCommander 队首：`tag == 0x8000000000000001`
    （新常量 `REPLAY_FINISHED_SENTINEL`）→ log + `consume()` 弹栈。
- `CmdUnfreezeSheep` 属 sheep 域（未建），以 log 文案承载，登记为宿主消费。

### 2.4 `replay_mode`（0x140a7c550）—— 实装系统本体

fresh decomp 关键分支：

```c
if (*(int *)*in_RCX != 0xd) { if (!(DAT_143634d88&1)) return; DAT_143634d88 = 0; return; }
if (!(DAT_143634d88&1)) { log; DAT_143634d88 = 1; }
UiState::clearing_canvas_w_layer_and_sound();
// HoldToFastReplay：
if ((hold & 1) == 0) {                       // 未触发
  if ((input+0x80 < 3) || (input+0x88 < 3)) fVar24 = 0.0; else fVar24 = hold_time + dt;
  fVar23 = 1.0; if (fVar24 <= 0.2) fVar23 = fVar24;       // clamp 至 0.2
  hold = (0.2 < fVar24); hold_time = fVar23;              // 阈值 0.2s
  if (fVar24 <= 0.2) { time_continuous_lerp 路径; return; }
  // fast → time_continuous = 1.0
}
```

实现（`replay_mode.rs`）：
- `FAST_REPLAY_HOLD_SECONDS: f32 = 0.2`（旧 `FAST_REPLAY_HOLD_MS=500` 证伪删除）。
- `HoldToFastReplay { fast, hold_time }` + `advance(engaged, dt) -> bool`：
  `fast` 保持；未触发时 `accrued = engaged ? hold_time+dt : 0`，clamp 至 0.2，
  `fast = 0.2 < accrued`。`reset()` 退出回放时清零。
- `replay_mode` 系统：`Local<bool>`（`DAT_143634d88` 模拟）检测 AppMode 进出 Replay，
  进入 → `ClearCanvasCmd(true)`（clearing_canvas）；回放中 `fast` → `HistoryCommand::REDO`。
  input-ready（原 `+0x80/+0x88`）经 `ReplayFastHoldInput` 资源由宿主汇总。
- `time_continuous_lerp` / `TimeOfDayScreen` 写回由宿主装配（本模块提供状态机）。

### 2.5 edit 载荷 serde 重建

**`RasterEdit`**（`path_edit.rs`，0x1404896f0/0x1416e8510/0x140a976b0）：
`{points: Vec<Vec3>, is_additive: bool, brush_size: f32, blend_mode: StrokeBlendMode}`
——**点型 Vec3（旧 src Vec2 证伪）**、字段序 `points→is_additive→brush_size→blend_mode`
（旧 src 序错）。手写 `Serialize`（单 map 四 entry）+ 手写 visitor `Deserialize` +
`impl From<Arc<[Vec3]>>`（源结构余四字段 +0x00/+0x08/+0x30/+0x38 语义 U，登记）。
`StrokeBlendMode` 补 `serde::Serialize/Deserialize/Default`（RasterEdit 载荷面）。

**`TreeEditData`**（`tree_edit.rs`，0x14003d4e0）：
字段序 `position(Vec3)→scale(f32)→rot(f32)→id(i32)→prefab(String)→is_preplaced(bool)`；
**id 由 u64 改 i32、由首字段移至第 4**（旧 src 两处证伪）。手写 `Serialize`；
`TreeEdit::id()` 提宽 u64 保留调用面。snapshot.rs 以整体类型引用，不受影响。

**`ColorEdit`**（`color_edit.rs`，0x14003c5d0）：
手写 `Serialize`/`Deserialize`（非 derive）。原版按 ColorTarget 跳表写互异载荷
（WallId+TerrainStrokeId / WallId+FlatRoofStyle×2 / DecoratorId+f32×2 / DecoratorId+bool×2 /
StairRailingType×2 / StairSupportType×2 / DecoratorId+TerrainStrokeId×2 / String…）；src 平面
数据模型不含各 target 互异载荷字段（由各自域持有），实现 **target 键单 entry map +
平面字段子集**的结构对齐 serde，互异载荷类型面登记 U。

**`DecoratorHistoryEdit`**（`decorator_edit.rs`，0x1404f46e0）：
手写 `Serialize`/`Deserialize`：**单 entry map** `{"<id>": {before, after}}`
（`__SerializeWith` 惰性嵌套 diff 面）。

**`StairsHistoryEdit`**（0x140259af0/0x140a18ee0）：**未重建**——原生为 `*&1` 判别的
单边枚举（每变体单 `StairGraphEdge` 嵌套 map），src 为 before/after Vec diff 模型
（`StairsReplayCmd::expanded` 差集语义依赖），两形状不同构；维持数据模型 derive，
原生枚举形状登记 U（见 §4）。

---

## 3. 自检

- **History::add**：lib 编译通过；tag-7 分支在原版 nuke 之后、push 之前；panic 文案与字符串常量逐字核对。
- **replay_stairs**：常量 0.05 与 fresh `if (fVar44 <= 0.05) fVar44 = 0.05;` 一致；`advance` 先判后加逐句对应。
- **finish_replay**：RestoreAppTo 写回 / Toolbar 恢复 / 哨兵弹栈 / log 与 decomp 分支对应；
  哨兵 tag 0x8000000000000001 来自 `!= -0x7fffffffffffffff`。
- **replay_mode**：HoldToFastReplay clamp 至 0.2、`fast = 0.2 < accrued`（严格大于）与
  `(uint)(0.2 < fVar24)` 一致；`<= 0.2` 保持路径返回。
- **RasterEdit/TreeEditData**：字段序/类型/手写 serde 与 fresh 逐 entry 对应。
- **编译**：`cargo check`（lib）0 错误；`cargo check --tests` 本模块 0 错误。

---

## 4. 未重建登记（真业务，缺他域布局/载荷面）

| VA | 函数 | 缺什么 |
|---|---|---|
| 0x140b3f930 | `DecoratorHistoryState::from` | 需 live `DecoratorInfo`（resources/walls/decorator.rs）style 字节分支语义 + `DecoratorHistoryState` 结构扩展（原版 ~0x4fB ≈10 字段）；decomp 已留档。 |
| 0x140259af0 / 0x140a18ee0 | `StairsHistoryEdit` serde | 原生单边枚举 vs src diff-Vec 模型不同构；需边枚举数据模型重构。 |
| 0x140af5b30 | `history_process` 第二事件流 | 16B 记录 byte[+9] 跳表 0x142ae5420（case 0..0x17：校验/动画即时化/移除）未解码。 |
| 0x140af6170 | `replay_edit` Color/Decorator/Terrain 命令展开 | tag 跳表 0x142c5f284 各命令生成器主体在 recovered_building/装饰域。 |
| 0x140b38250 | `replay_pop_bucket` | restore_decorators + Stairs restore_backup/apply_diff/make_backup + flush_colliders 主体在装饰/楼梯域。 |
| 0x140951b80 / 0x140abfb30 | `replay_wall`/`WallReplayEditCmd::process` 17 操作 | 操作跳表 0x142adf44c/0x142adf408 主体在 recovered_building（wall_history_replay 目前仅 EnclosureChange/StateChange(op0) 实现，其余 `UnrestoredOperation`）。 |
| 0x140ae5830 | `get_param` | 4 索引稀疏表取 4 组参数否则 panic（replay_raster 资源面）。 |
| 0x1409e6210 | `missing_prefab` | 24B String 常量（"MISSING_PREFAB…"）返回。 |
| 0x140a854a0 | `add_decorator_diff` | 楼梯 checksum 校验 + 两 u32 边循环 + Arc 替换两表（装饰/楼梯域）。 |
| 0x140b3fab0 | `convert_before_after_states_to_changes` | 按 style 字节分变体比较浮点输出变更集（NaN 语义）。 |
| 0x1417bdae0 | `send_state_diff_events` | replay_pop_bucket 消费链装饰 diff 事件。 |

---

## 5. 汇总

- **分类**：21 个待重做/未实现函数全部为**真实业务**（含 6 组 serde 族手写实现，
  非宏胶水）。
- **重建**：History::add（tag-7 Wall 校验）、replay_stairs（0.05 下限）、finish_replay、
  replay_mode（HoldToFastReplay 0.2s）、RasterEdit（Vec3 载荷）、TreeEditData（i32 id/字段序）、
  ColorEdit、DecoratorHistoryEdit（结构对齐 serde）。
- **未重建（登记）**：DecoratorHistoryState::from、StairsHistoryEdit serde、history_process
  第二事件流、replay_edit Color/Decorator、replay_pop_bucket、replay_wall 17 操作、
  get_param、missing_prefab——均需他域（decorator/stairs/recovered_building）布局或载荷面。
- **编译**：lib 0 错误；本模块测试 0 错误（测试构建余错误均为他 agent 并发在改的无关模块）。
