# DOWNSTREAM-ANCHOR-MAIN — walls 41 项依赖的下游结构锚定与回接（主树）

日期：2026-10-17。工作树：`/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core`
（唯一改动/测试树）。方法：只以 fresh 反编译/反汇编为准；缺字段登记不编造；宏胶水不重建。

fresh 证据落盘：`recovery/agent-team/work/impl-downstream/decomp/`
（`batch1.result.json`、`b1/*.c`、`dis-*.txt`、`b3/*.c`、`disasm5.json`、
`dis-0x140ac1f20.txt`、`mem9/mem10` 常量/字符串）。

## 0. 结论一览

| 项 | 状态 |
|---|---|
| 1. `PreferredNodeAlignment` + `AlignKind`（24 B） | **closed**（类型落地 + 偏移/大小断言测试） |
| 2. `AutoClutterObject`（0x68 B，key@0x40）+ `wall_roof_hashmap` | **closed**（类型 + 关联函数 + panic 断言） |
| 3. `UiState` hover/click/交集/id/lock/map 字段与方法 | **closed**（+0x28 交集判别补字段，`is_active` 纠正） |
| 4. `UiWallHeight` 字段（0/8/9）回接 `ui_adjust_wall_height` | **closed（字段/公式层）**；widget/signifier 实体链仍登记 |
| 5. `MorphPremadeShape` serde schema → `ConcurrentEdits` → `WallHistoryCompoundEdit` | **closed** |
| 6. create_rectangle 第二条 Color history owner | **closed（事件 owner）**；`EditType::Color` 映射登记 |
| 7. edit_rectangle/ui_create_rectangle、6 个 UI 系统 | **seam/登记**（见 §7） |

## 1. AlignKind（24 B）与 PreferredNodeAlignment

**fresh 锚定**
- `AlignKind::position_offset` 0x140959f00（22 指令）：`tag [rcx+0]`、`flag [+1]`、
  `byte2 [+2]`、`Vec2 [+4]`、`Vec2 [+0xc]`；tag 0 需 `flag==0 || byte2==1`；
  tag 1 在 `flag!=0` 时加第二 Vec2 且 scale×0.5。
- `AlignKind::outgoing_dir` 0x140959f50（24 指令）：tag 0 → `(+4,+8)`，tag 1 →
  `(+0xc,+0x10)`，否则 panic。panic 字符串 @0x142ab4cf8 =
  `"AlignKind::None has no defined outgoing_dir"`；源码路径 @0x142ab4d38 =
  `crates/country-core/src/resources/stairs/stairs_assembly_params.rs`。
- 20 B 载荷 + 4 B 对齐 = **24 B**。
- 生产方（edit_rectangle 0x14097f620 / adjust_wall_height_impl 0x140ac1f20 /
  ui_create_rectangle 0x14155d410）写入的 0x28 桶记录被当作手写记录复用
  （`ui_edit_terrain::StairsPlacementEntry` 已记录同一模式），不能反过来改写
  AlignKind 布局。

**落地**：`src/resources/stairs/stairs_assembly_params.rs`
- 新增 `#[repr(C)] AlignKind`（tag/flag/byte_2/pad_0/first/second/pad_14），
  `NATIVE_SIZE = 0x18`，实现两个方法（panic 文案按 fresh 修正）。
- 新增 `PreferredNodeAlignment`（0x20）：
  `{q0@0,q1@8,q2@0x10,tag u32@0x18,byte_1c@0x1c,byte_1d@0x1d,pad[2]}`。
  证据：`edit_rectangle` 写 `04 00`；`adjust_wall_height_impl` 写
  `00 <bool>`；`grow_one` 符号的容器名称为
  `Vec<(DecoratorId, Vec<PreferredNodeAlignment>)>`。字段语义未证明，保持编号。
- `ObservedAlignKind::outgoing_dir` panic 文案改为 fresh 字符串。
- 导出：`resources/stairs/mod.rs`。

## 2. AutoClutterObject（0x68 B，key@+0x40）与 wall_roof_hashmap

**fresh 锚定**
- `manage_autoclutter_entities` 的单态 `from_iter` 0x142612aa0：
  `IMUL RAX,R15,0x68`（步长 0x68）与
  `MOV RDX,[RDI+RAX+0x40]`（map key = 对象 `+0x40` qword）。
- 0x140a94cd0 的重复插入检查 panic 字符串 @0x142ada570 =
  `"Only one roof should be associated with a wall id"`；
  replay 字符串 @0x142ada4c8 = `"Tried deleting roof {} during Replay"`。

**落地**
- `src/systems/ui_clutter/autoclutter_state.rs`：`#[repr(C)] AutoClutterObject`
  = `{payload [u8;0x40], key u64, tail [u8;0x20]}`，`NATIVE_SIZE=0x68`，
  `key()`；其余字节不编造。
- `src/systems/wall/wall_state/wall_roof_hashmap.rs`：新增
  `build_wall_roof_hashmap(objects) -> HashMap<u64, &AutoClutterObject>`，
  重复 key panic 与 fresh 同文案；dense/gen/replay 探针属宿主表，未伪造。

## 3. UiState（hover/click/交集/id/lock/map）

**fresh 锚定**
- `clicked` 0x14092e5f0（14 指令）：`byte[+0x18] >= 2`、`byte[+0x19] <= 1`、
  `byte[+0x19]&1`、`qword[+0x38] == ctx.id` → `SETZ AL`。
- `hovered` 0x14092e620：`byte[+0x18] == 1 && qword[+0x38] == id` → 返回
  `byte[+0x19]`。
- `is_active` 0x14092e680：`byte[+0x18] != 0 && qword[+0x38] == id &&
  dword[+0x28] != 2` → `SETNZ AL`。**旧实现错把 `native_tag != 2` 当交集判别**。
- `add` 0x1409236d0 / `add_with_id` 0x14092e3e0：锁字 `+0x60`、map `+0x68`，
  0xb0B WidgetParams 拷贝、Arc 0xc0 插表；`add` 用
  `utils::calculate_hash<(&str,&())>`（与现有 `DefaultHasher` str 哈希等价）。

**落地**：`src/systems/ui/resources/ui_state.rs`
- `UiState` 新增 `intersection_tag: u32`（native +0x28）及
  `intersection_tag()/set_intersection_tag()`；
- 新增 `hover_byte()/click_byte()/active_widget()`；
- `is_active` 改为读取 +0x28 交集判别；
- lock/map 偏移写入文档。

## 4. UiWallHeight（+0/+8/+9）回接

**fresh 锚定**
- signifier 0x140956cd0：`mode = byte[+9]`；`flag = byte[+8]`；
  `+0` = WallId（经 `resources::walls::impl$0::fmt`）；
  points 指针/长度 `+0x90/+0x98`，heights `+0xe0/+0xe8`；
  `min(points.len, heights.len)`。
- widget 0x1409566d0：`mode==0 → y+0.6`；否则 `stacked ? y-0.29 : y+0.0`；
  名称 `edit_height_{WallId}`。
- signifier mode==1 分支：名称 `wall_height_edit_{WallId}`，env 标量
  `flag==0 ? 0.2 : 0.0`；flag==0 时要求 hover/id/交集判别门。
- 非 mode==1：名称 `wall_height_edit_{WallId}{count}`，点 y+0.6。

**落地**：`src/systems/wall/adjust_wall_height.rs`
- 新增 `Resource UiWallHeight { wall(+0), flag(+8), mode(+9), points, heights }`
  与 `lifted_points/widget_name/signifier_name/signifier_env_scalar/
  signifier_guard_passes`；在 `register_wall_height_events` 注册资源。
- `resources/walls/mod.rs` 新增 `Display for WallId` = `"WallId({})"`
  （0x140b34ec0），供 signifier 名与 replay panic 使用。
- `ui_adjust_wall_height` 2647B 输入系统与 widget 实体链仍登记（见 §7）。

## 5. MorphPremadeShape（serde → ConcurrentEdits → Compound）

**fresh 锚定**（0x141464a60）
- 变体 5 载荷：`{wall u64 @+0x08, previous ShapeParameters @+0x10,
  next ShapeParameters @+0x2c}`；两个 shape 均为 28 B premade 记录
  （tag 0 = Circle / 1 = Rectangle）。
- 判定核（orientation ≤0.01° 且 dims ≤0.01 → 走
  `extract_and_restore_to_storage` 并清 state；否则
  `CompoundEditBuilder::next` → `from_iter<ConcurrentEdits>` → `History::add`
  + `WriteSettingsFileCmd` + `WindowFocusEvent`）。

**落地**
- `src/recovered_building/wall_history_edit.rs`：
  `MorphPremadeShapeEdit { wall_index, before, after }`
  （`Serialize/Deserialize`），变体载荷由 `UnresolvedWallEditPayload` 替换；
  `wall_id()` 现在返回 wall_index。
- `src/recovered_building/wall_rectangle_edit.rs`：决策核产出的 history 由
  `EnclosureChange` 改为 `MorphPremadeShape`，`before` = 状态记录 28 B，
  `after` = 当前 InnerWalls `source.shape`；经
  `WallCompoundHistoryEdit.groups`(=ConcurrentEdits) 送入
  `finish_edit_rectangle` → `RecordWallHistory`。
- `WriteSettingsFileCmd`/`WindowFocusEvent` 双写面仍登记（见 §7）。

## 6. create_rectangle 第二条 Color history owner

**fresh 锚定**（0x140a94617）
- gate = `GladeColorCustomization.gates != 0`（+0x38）。
- 0x30B 分配 = 0x10 Arc 头 + 32B 载荷
  `{+0x08 = 4, +0x10 = wall id, +0x18 = 0, +0x1c = gates}`。

**落地**
- `src/systems/wall/history_recording.rs`：
  `ColorHistoryRecord { kind, wall, from, to }`（`payload()` 保留 32B 偏移）
  + `RecordColorHistory` 事件 + `record_color_history` 消费者；事件在
  `register_wall_recording_events` 注册。
- `src/systems/wall/shapes/rectangle/create_rectangle.rs`：新增
  `EventWriter<RecordColorHistory>`，在 `colors.gates.0 != 0` 时发
  `{kind:4, wall:id, from:0, to:gates}`。
- `EditType::Color(Arc<ColorEdit>)` 映射仍登记：现有 serde `ColorEdit` 是
  另一套内存记录，直接套用会编造变体。

## 7. edit_rectangle / ui_create_rectangle / 6 个 UI 系统

**已落地 seam**
- `src/resources/walls/shape_alignment.rs`：
  - `AlignKindMapEntry`（0x28：`hash@0, key@8, value[24]@0x10`）与
    `WallShapeAlignmentWrites { preferred_node_alignments:
    Vec<PreferredNodeAlignment>, align_kind_entries: Vec<AlignKindMapEntry> }`，
    在 `register_shape_resources` 注册。
  - 记录值与记录布局按 fresh 锚定；生产方所需 dense InnerWalls 记录字
    (`+8/+0x28/+0x30`) 与 +0x40 decorator-id 计数器未由语义 `WallState`
    建模，故编辑器暂不写值（不编造）。

**仍登记（不可判等落地）**
| 系统 | 解锁条件 |
|---|---|
| `edit_rectangle` 0x14097f620 两条记录写面 | dense InnerWalls 0x40 记录字 + decorator-id 计数器落地后可写入 `WallShapeAlignmentWrites` |
| `ui_create_rectangle` 0x14155d410 | `wall_rectangle_canvas` 输入、`GladeBorder::is_shape_inside`、eye_right、矩形投影 `rectangle_dims_from_cursor_pos` 0x140c99520 |
| `ui_edit_rectangle_dims` 0x140ad6250 / `ui_edit_rectangle_rotate` 0x140ad8000 | Rapier `intersect_ray/is_hit`、widget/signifier 实体链 |
| `do_ui_edit_rectangle_dims` 0x140ad6e00 / `do_ui_edit_rectangle_rotate` 0x140ad8be0 | 事件跳表、`UiState80` 写面、快速 acos 门控 |
| `UiRectangleDimsEdit::signifier/widget` 0x14155eba0/0x14155fc70 | 宿主 + `ui_state::impl$15` Display + 实体链 |
| `UiRectangleOrientationEdit::signifier/widget` 0x14083da10/0x14083d3f0 | 同上 + `box_trigger_volume` 渲染宿主 |
| `ui_adjust_wall_height` 0x140b2fc00 / `ui_adjust_wall_height_mode` 0x140ac6d60 | `UiAdjustWallHeightMode` 0x30B 事件所有者、widget/实体链 |
| `UiWallHeight::widget/signifier` 0x1409566d0/0x140956cd0 + closure 0x1409576d0 | `UiState::add<String>` 参数块 0xb0（sounds/volume 来源）、AppendOnlyVec/实体链；业务门/名称/公式已落地 |
| `finish_edit_rectangle` 0x141464a60 剩余面 | DecoratorArchivist `extract_and_restore_to_storage`、0x10B `WindowFocusEvent` 载荷（wall id）、profiler buffer |
| `wall_roof_hashmap` 0x140a94cd0 剩余面 | dense/gen 探针、replay 门与 `WindowFocusEvent`；关联核已落地 |

## 8. 改动文件

- `src/resources/stairs/stairs_assembly_params.rs`（AlignKind/PreferredNodeAlignment + 测试）
- `src/resources/stairs/mod.rs`（导出）
- `src/systems/ui_clutter/autoclutter_state.rs`（AutoClutterObject + 测试）
- `src/systems/wall/wall_state/wall_roof_hashmap.rs`（关联核 + 测试）
- `src/systems/ui/resources/ui_state.rs`（+0x28 字段/方法/is_active + 测试）
- `src/systems/wall/adjust_wall_height.rs`（UiWallHeight + 测试 + 资源注册）
- `src/resources/walls/mod.rs`（shape_alignment 模块 + WallId Display）
- `src/resources/walls/shape_alignment.rs`（新文件：对齐记录宿主 + 测试）
- `src/systems/wall/shapes/mod.rs`（注册 WallShapeAlignmentWrites）
- `src/recovered_building/wall_history_edit.rs`（MorphPremadeShapeEdit + serde + 测试）
- `src/recovered_building/wall_rectangle_edit.rs`（决策核改产 MorphPremadeShape + 测试）
- `src/systems/wall/history_recording.rs`（RecordColorHistory/ColorHistoryRecord/消费者）
- `src/systems/wall/shapes/rectangle/create_rectangle.rs`（第二条 Color history 发射 + 测试）

未改数据库；只读 `--worker shared`；未触碰 `reconstruction/**` 或 `skeleton/B/**`。

## 9. 验证

- `cargo check -p country-core --lib`：**exit 0**（14 warnings，均为既有）。
- `cargo test -p country-core --lib`：**2068 passed / 0 failed**（本轮结束）。
  - 说明：任务给的基线是 2024。本轮开工时同一主树实测基线为
    **2047 passed / 1 failed**（失败为 `components::drawable::
    nani_bundle_layout_matches_write_components`，属他域并发 WIP）；该失败在
    收口前已被他组修复，故终值 2068/0。本组新增 20 条测试，未引入失败。
- 定向复核：`align_kind` 4、`preferred_node_alignment` 1、
  `auto_clutter_object` 1、`wall_roof_hashmap` 2、`is_active` 1、
  `widget_lift` 2、`morph_premade` 2、alignment/rectangle/color 相关 5 条全过。
