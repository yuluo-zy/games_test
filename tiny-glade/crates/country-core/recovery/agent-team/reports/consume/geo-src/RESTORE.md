# RESTORE — consume/geo-src（restorer-consume-geo-src，2026-10-17）

> 角色：restorer-consume-geo-src。目标：消费 `unlock/geo-src-raycast` 落库的
> `geo_src_to_edit_subject` / `_w_deco_clutter` 与 `legacy_raycast` 命中码，实现
> - `crates/systems/onboarding`：`hint_focused_edit_hint` 0x14227dd60、
>   `hint_clutter_focused_edit_hint` 0x141c3db80；
> - `crates/systems/tree-tool`：`ui_place_tree` 0x142269170。
>
> **三态：完成。判等 0 / 重建 3 / 未实现 0；登记 I/U 8（不阻塞）。**
> reviewer-consumers 复核后的 FIX-1/2/4/5/6/8 已修（FIX-7 登记 U，FIX-9 他路修）。
> 全部 fresh 反编译，未采信此前报告；证据可重放（`evidence/`）。

## 0. 身份与工具

- B patched EXE `tiny-glade.exe`，base `0x140000000`，131517 函数；
  sha256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`
  （`evidence/result-fresh.json` `get_metadata`）。
- ghidra `--worker shared`（只读）+ radare2 `--worker restorer-consume-geo-src`
  （原始指令/跳表字节）。未写任何数据库。
- 关键新证据：
  - `result-fresh.json`：0x14227dd60 / 0x141c3db80 / 0x142269170 / 0x142269970
    完整 decompiled_code + callees。
  - `result-r2.json`：三函数逐指令反汇编 + 跳表字节。
  - `result-callers.json`：PDB 还原的**完整 SystemParam 签名**（见下）。
  - `result-helpers.json`：`EventWriter::write<DisplayHintRequest>` 0x14227ba70、
    `PublicWalls::get` 0x140a91ef0、`make_focus_or_camera_binding_hint` 0x141c3de00
    的 fresh 反编译。
  - `result-uistate.json` / `result-byte.json`：UiState 访问器 + 0x140002450 原始字节
    （`48 8d 41 18 c3` = `lea rax,[rcx+0x18]; ret`，即 UiState+0x18 字节门）。
  - `jump-tables-decoded.txt`：0x1430386a0 六项跳表逐项解码。

### PDB 全签名（callers 中的 FunctionSystem 泛型参数）

```text
hint_focused_edit_hint:
  Res<AppMode>, Res<PublicWalls>, Query<(&Roof,&RoofTriggerVolume)>,
  ResMut<FocusedEditHintConditions>,
  EventReader<NotifyHintSystemShapeEditWasPerformed>,
  EventWriter<DisplayHintRequest>, Res<RaycastWorld>, Res<CameraCursorRay>,
  Res<GameProfile>

hint_clutter_focused_edit_hint:
  Res<AppMode>, ResMut<FocusedClutterHintConditions>,
  EventWriter<DisplayHintRequest>,
  EventReader<NotifyHintSystemClutterWasMoved>,
  Res<DecoratorStorage>, Res<RaycastWorld>, Res<CameraCursorRay>, Res<UiState>,
  Res<GameProfile>

place_tree (对照，未改业务):
  ResMut<TreeStorage>, EventReader<PlaceTreeCmd>, EventWriter<InsertTreeCmd>,
  ResMut<History>, Res<ActiveSession>
```

## 1. `hint_focused_edit_hint` 0x14227dd60（重建）

跳表 **0x1430386a0，6 项**（i=0..5，base+4i+int32），解码见
`evidence/jump-tables-decoded.txt`：

| subject（EditSubject） | target | 行为 |
|---|---|---|
| 0 None / 3 DecoClutter / 4 StairPoint / 5 Terrain | 0x14227e113 | epilogue，无提示 |
| 1 Wall | 0x14227e09f | `PublicWalls::get(handle@+8)`；`[rax+0x258] >= 2` → return；否则落公共写面 |
| 2 Roof | 0x14227e0c1 | 公共写面 |

逐语句（fresh）与 Rust body 对照：

1. `cmp byte [rax+8],0; jne epilogue` → `if state.hint_done { return; }`（hint_done@+8）。
2. EventReader 剩余计数（Bevy `ManualEventReader::read` 的 saturating-sub 两缓冲公式）
   >0 → **游标一次性置 event_count（耗尽全部未读）**；`count++`（`inc qword [rax]`）；
   `setae byte [rax+9]` = `4 < count_new`
   → `state.count.wrapping_add(1); state.state = u8::from(4 < state.count)`。
   Rust 用 `shape_edit_events.read().count() > 0`（FIX-4；`.next()` 每帧只消费 1 个，
   多事件帧会把剩余事件留到后续帧再 +1，原生不会）。
3. Query 遍历（`Q+0x100` 数、`Q+0xf8` 索引、`Q+0x108` dense/sparse 两套 stride）
   末尾 `test rbx,rbx; je epilogue` → 非空门。
4. `cmp byte [rax+9],0; je epilogue`；`*in_RCX==0` → `state.state != 0`、
   `mode.native_tag() == 0`。
5. `legacy_raycast`（`cmp dword [rsp+0xa0],2; je epilogue`，+0x30 槽）→
   `legacy_raycast_miss`（native 判据 `toi.to_bits()==2`）。
6. `geo_src_to_edit_subject(&hit.handle[24B])`；按判别跳表。
7. Wall：`PublicWalls::get(...).unwrap()`（0x14227e0b6 `je` → `option::unwrap_failed`
   0x1428d9310；Location 0x143038670 = `hint_focused_edit.rs:133:47`）；
   **`get` 返回 bucket+8 = value 起点**（0x140a91fdb `lea rax,[rcx+rbx]` +
   `add rax,-0x2a0`；key @bucket+0、count @value+0x10、length @value+0x30，
   与 Rust `validate_public_wall_read` 一致）。故 `[rax+0x258]` = value+0x258 =
   `WallShapeSnapshot::tag()`（0 Circle/1 Rectangle/2 Freehand）；`>= 2` → return。
8. 公共写面 0x14227e0c1：`make_focus_or_camera_binding_hint(profile)`；
   `movups [rsp+0x70], subject16B`；`mov word [rsp+0x80], 0x0101`（placement+0x10）；
   `mov word [rsp+0xb8], 0x0100`（+0x48 flags=0、+0x49 id=1 FocusedEdit）；
   `EventWriter::write<DisplayHintRequest>`（0x14227ba70，步长 0x58）。

Rust：`hint_focused_edit_hint` + `write_focused_edit_hint`（新）。
写面以新 `HintPlacement::Subject { subject, word_0x10: 0x0101 }` 表达。

## 2. `hint_clutter_focused_edit_hint` 0x141c3db80（重建）

逐语句（fresh）：

1. `cmp byte [rbx],0; jne epilogue` → `state.hint_done`（@+0）。
2. EventReader<ClutterWasMoved> 剩余计数 >0 → **游标一次性置 event_count（耗尽）** +
   `mov byte [rbx+1],1` → `state.state = 1`（@+1；FIX-5，`.read().count() > 0`）。
3. `lea rax,[rcx+0x18]; call 0x140002450; cmp byte [rax],2; je epilogue`：
   0x140002450 原始字节 = `48 8d 41 18 c3`（`lea rax,[rcx+0x18]; ret`），
   `rcx=[Res<UiState>]` → 门 = **UiState+0x18 == 2 → return**
   （同字节即 UiInteraction 的 native_tag；`ui.interaction().native_tag == 2`）。
4. `cmp byte [rbx+1],0; je epilogue`；`AppMode @+0 != 0 → return`。
5. `RaycastWorld::raycast` 0x1408cc870：`mask=0x100`（stack 实参）、
   `max_toi=f32::MAX`（0x142925980）；`cmp dword [rbp+0x64],2; je epilogue`
   （结果 +0x34 判别 2 = miss → return）。
6. `geo_src_to_edit_subject_w_deco_clutter(handle, &DecoratorStorage)`；
   `cmp dword [rbp-0x30],3; jne epilogue` → 仅 `DecoClutter` 继续。
7. `make_focus_or_camera_binding_hint(profile)`；
   placement = `{dword 3, decorator_id@+4, qword(uninit)@+8, word 0x0100@+0x10}`；
   `mov word [rbp+0x28],0x0100` → flags=0、id=1（FocusedEdit）；
   `EventWriter::write` 内联（grow_one + 0x58B 拷贝）。

Rust：`hint_clutter_focused_edit_hint`（新）；placement 用
`HintPlacement::Subject { subject, word_0x10: 0x0100 }`。

## 3. `ui_place_tree` 0x142269170（重建）

fresh 控制流：

1. 门禁 `*(int*)*in_RCX == 0x10`（AppMode::Tree）且 `DEV_OVERRIDE_IS_DEMO_MODE`
   byte == 0（既有实现已判等）。
2. `WidgetParams::new/occlude_by_world/sounds` + `UiState::add("tree_canvas")`
   （既有近似，见既有登记）。
3. **新增门**：`RaycastWorld::legacy_raycast`（0x1422692b5）→
   `cmp dword [rbp+0xf8],2; je 0x1422692f6`（+0x30 命中码 == 2 → **跳过** geo_src）；
   `lea rcx,[rbp+0x178]; lea rdx,[rbp-0x50]; call geo_src_to_edit_subject`
   → `cmp dword [rbp+0x178],0; jne 0x1422695f6`（subject != None → **return**）。
   即：命中且非 `EditSubject::None` → 早退；miss 或 None → 继续放置。
4. `CursorTerrainRaycast::position()` → `GladeBorder::is_pos_inside(Vec2(x,z))`
   （既有实现判等）。
5. 界外：`DisplayOutOfBorder` 无条件；`clicked` → `FeedbackInputOutOfBorder`。
6. 界内：`is_active` → signifier 写；`clicked` → 二次 `position()` → 水下
   `(y/-2.5)*-1.2 + (1-y/-2.5)*0.0` → `PlaceTreeCmd{position}`。

Rust：新增 `legacy_raycast_miss` + geo_src 门（位于 UiState::add 之后、
position 之前，与原顺序一致）；其余沿用既有实现。

## 4. 验证

workspace 根 `/Users/yuluo/Desktop/game/games_test/tiny-glade`：

| 命令 | 结果 |
|---|---|
| `cargo check -p system-onboarding -p system-tree` | **0 error**（已存在 warning 与本轮无关） |
| `cargo check -p system-onboarding -p system-tree --features country-core/native-physics` | **0 error** |
| `cargo test -p system-onboarding -p system-tree` | **36 passed / 0 failed**（onboarding 29 + 2，tree 5） |

改动文件：
- `crates/systems/onboarding/src/hint_focused_edit.rs`
- `crates/systems/onboarding/src/hint_clutter_focus_edit.rs`
- `crates/systems/onboarding/src/hint_orchestrator.rs`（新增 `HintPlacement::Subject`；
  `HintGraphicParams` 改为原生 32B 语义 `{Vec<图形名>, f32@+0x18, f32@+0x1c}`）
- `crates/systems/onboarding/src/hint_display_system.rs`（FIX-1/2）
- `crates/systems/tree-tool/src/ui_place_tree.rs`

新增回归测试：
- `hint_focused_edit::tests::shape_edit_events_are_exhausted_once_per_frame`（FIX-4：
  一帧 2 事件 → count 只 +1；下一帧不增长）。
- `hint_clutter_focus_edit::tests::make_focus_steam_query_and_double_icon_payload`
  （FIX-6/8：Steam=0x0101；单 Graphic = 双图标 + 0.7/100.0）。
- `hint_display_system::tests::position_bounds_takes_max_screen_x_corner_with_its_y`
  与更新的 cursor/clamp 期望（FIX-1/2）。

## 5. 登记（I/U，不阻塞）

1. **I（padding）**：原版 `DisplayHintRequest` 的 placement +0x12..+0x18、
   display 末尾 padding、+0x4a..+0x50 为栈上未初始化字节（无消费者读取）。
   Rust 以语义结构承载，未复刻未初始化字节。
2. **U（单图标 Graphic 载荷）**：`make_focus` 的 32B 双图标载荷已按 fresh 反汇编判等
   （FIX-6/8：Steam=0x0101、`[hint/mouse, hint/rmb|hint/mmb]` + 0.7/100.0）。
   camera/stairs 等**单图标**写面的 +0x18/+0x1c 未逐面取证 → 置 0（U）。
3. **U（profile+0x1c6 鼠标键）**：非 1/2 分支原生 `MOVZX R9D,word [profile+0x1c6]`；
   country-core `GameProfile` 无该字段 → 暂以 `MouseButton::Left` 占位（FIX-7 登记 U，
   解锁=core 补 +0x1c6 u16）。
4. **I（query 门等价）**：原版查询遍历后取**末实体** item 指针做 `!=0`；
   本实现判 `query.iter().next().is_some()`。对合法 Query 两者恒同（非空 ↔ 指针非零）。
5. **I（shell seam）**：非 `native-physics` 的 `legacy_raycast`/`raycast` 壳返回
   `miss()` 且不写 +0x30 位型；`legacy_raycast_miss` 以 `|| hit.is_miss()`
   归一。native 下两条件恒等价（legacy miss 同时写 +0x30=2 与 +0x34=2）。
6. **I（placement 变体语义）**：native 判别 1/2/3 的 placement 变体名与
   +0x10 u16（0x0101/0x0100）语义未取证；按原字节值结构化建模
   （`HintPlacement::Subject`），不臆造布尔含义。
7. **I（UiState 投影）**：`UiState+0x18 == 2` 门用 `ui.interaction().native_tag`
   表达（既有 UiState 语义投影，native 字节为 +0x18）。
8. **U（ui_place_tree 其余面）**：widget/signifier 写面为用户此前登记的近似
   （SignifierFactory/format 串面），本轮未改、不声称判等。

## 6. reviewer-consumers FIX 回执（2026-10-17 晚）

| FIX | 处理 |
|---|---|
| FIX-1 | **已修**：`hint_position_update` 边距 `dims.y`（height/1000×20,30）；cursor 期望 `(14,20)`、clamp 期望 `(-103,-203)`。 |
| FIX-2 | **已修**：case 8 四角投影后取**屏幕 x 最大**角（`new.x > acc.x` 严格大于、并列保留先者）并携带其 y；新增回归测试 `position_bounds_takes_max_screen_x_corner_with_its_y`。 |
| FIX-3 | 维持未实现登记：7/8 的放置 Y 仍缺 `PreviewCircle+0x20`/`PreviewRectangle+0x28` 字段（core 缺口）。 |
| FIX-4 | **已修**：`hint_focused_edit_hint` 改 `.read().count() > 0`（一次性耗尽）；新增测试 `shape_edit_events_are_exhausted_once_per_frame`。 |
| FIX-5 | **已修**：`hint_clutter_focused_edit_hint` 同步 `.read().count() > 0`（幂等）。 |
| FIX-6 | **已修**：`STEAM_FOCUS_ACTION = 1` → `packed_set_plus_one(1) = 0x0101`；测试断言。 |
| FIX-7 | **登记 U**：`profile+0x1c6` 鼠标键字段 core 未落库，暂以 `MouseButton::Left` 占位。 |
| FIX-8 | **已修**：`HintGraphicParams` 改为原生 32B 语义 `{Vec<图形名>, f32@+0x18, f32@+0x1c}`；`make_focus` **单次** `alternate` 载荷 `[hint/mouse, hint/rmb|hint/mmb]` + 0.7 + 100.0，不再拆两个元素。 |
| FIX-9 | **他路已修**（debugline 线处理 `closure$1` 的 `DashedLineLengthView` 跨模块类型）；本线不再重复改动。 |

## 7. 未在本任务范围（已解锁但不改）

`hint_roof_shape` 0x142284ed0 的 `geo_src_seam()` 现可直接换成真实
`geo_src_to_edit_subject == 2`（0x142285133）；属另一条任务线，本轮未触碰。

## 8. 三态回执

**完成**：三个消费者全部 fresh 反编译并落库；`geo_src*` + legacy 命中码 +
原跳表/门控逐语句对齐。**判等 0 / 重建 3 / 未实现 0**；登记 I/U 8。
reviewer-consumers 的 FIX-1/2/4/5/6/8 已修，FIX-7 登记 U，FIX-9 由他路修。
无阻塞。
