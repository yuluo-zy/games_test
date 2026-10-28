# 形状编辑“15叶”（M06）— 当前状态入口（2026-10-28）

**用户停止点**：原采样差异已确认，当前工作保存提交后停止；不继续M06实现。UI resize=0.5，timed/instant Morph replay=0.2，原生产默认正确。真实host非空编辑→提交、circle canonical wall/decorator Undo/Redo、utils177/177等已通过。

**最后未通过项**：圆形跨进程v5的circle/write（真实拖动、Undo/Redo、保存）通过；circle/restore在serialized-history JSON期望比较失败，circle/restore-applied未执行。已诊断26处仅f64 JSON解析/展示差异，f32 bits相同；尚未将测试期望按typed SavedMultiEdit规范化，未改生产serde。失败日志 `impl_circle_save_fresh_process_v5.log`。按停止指令保留现状，不能标15叶或M01整体验收完成。下文旧“继续推进”记录不覆盖本停止点。

按既定“15叶”规格分组列出具体函数；组号不是实际函数计数。下面表格优先于所有历史 checkpoint，不按旧百分比计算完成度。当前源码/静态原证/本地真实链验证/原侧执行和整模块验收分别记账。

分类口径：`B-MODULE-INVENTORY.csv` 将本批 ui_move_shapes、ui_edit_freehand_walls、circle/rectangle shape-edit 等叶子归 M06；`m01-scratch` 沿用历史名称，本表不代表 M01 的451组覆盖率，也不跨 M01/M06 重复计数。

| 规格组与具体实现 | 实现 | 真实 host 安装 | 正确性验证 / 未决 |
|---|---|---|---|
| §1 adjust_wall_height 三件套（UI/mode/history_record） | 已有真实输入、权威与历史链 | UI在原b组；其余已有owner | 既有真实undo/redo通过；本次重排host待运行 |
| §2 move_gizmo_and_trigger_vol | 实际box widget、轴/线factory及声音参数 | helper由§3/4调用 | 已有factory/animation测试；原侧全函数未执行 |
| §3 ui_move_shapes | drag/archive、ray/roof snap、border、release、typed pins | 原c[0]，decorator reader after publication | 已有真实picker/history测试；非空decorator组合链与host待验证 |
| §4 ui_move_freehand_walls | 真实picker/移动/原typed Wall pins/history | 原c[1]，共享move reader | 已有wall undo/redo；本批pins及host待验证 |
| §5 ui_edit_freehand_walls | 点widget、曲线更新、typed Wall pins、共享edit reader；原duration0已补 | 原c[2]，reader after publication | 非空decorator/history undo/redo跨crate3/3；点factory duration0→真实animation1/1；actual host非空编辑→reader→finish通过 |
| §6 ui_edit_circle / fullcircle helper | radius/archives/border/history、typed pins、互斥fullcircle3/5工厂和真实Time音频；Normal真实animated producer | 原a[3] | 最新5/5含真实picker→animated→arrow/line、canonical start/finish→wall+非空decorator Undo/Redo；circle跨进程保存恢复扩展由root进行 |
| §7 signifier_edit_circle_arrow | Named flat_arrow、真实factory/component | 原a[4] | 既有5 tests；本批§6真实producer→consumer通过 |
| §8 signifier_edit_circle_line | 真实Curve component、animated mesh及factory | 原a[5] | 既有factory/height测试；本批§6真实producer→consumer通过 |
| §9 ui_rotate_circle | 真实picker/history、two arrow factories、typed pins、ivy event | 原a[6]；ivy reader after publication | shape59/59含factory wake/polar通过；ivy last-reader/missing-wall2/2；真实host待验证 |
| §10 ui_edit_rectangle_dims / do_ui_edit_rectangle_dims | edge widget、真实Slider3d、Wip、typed pins | 原b两组，保留组间chain | 已有真实picker/history；非空decorator原侧差分及host待验证 |
| §11 ui_edit_rectangle_orientation / do_ui_edit_rectangle_rotate / box_trigger_volume | corner widget、原角度平滑与border循环、typed pins | 原b两组 | 既有原角度fragment和真实history；非空decorator及host待验证 |
| §12 signifier_edit_dims | 56B Curve2 component、真实factory与mesh | 原c内create_rectangle之前 | 既有2 tests；host待验证 |
| §13 edit_rectangle | 最后事件、权威curve/enclosure、两audio | 原c末段 | 既有flags/dt0/NaN测试；host与consumer组合待验证 |
| §14 finish_edit_rectangle | Wip snapshot、threshold/restore/history/notifications | 原c最后，decorator reader在history finish之前 | 既有真实history；原consumer set未证，当前顺序为必要工程约束 |

公共 pins consumer 已由 root 恢复真实 storage/cache/mesh/raycast 与 ECS 生命周期，原矩形交叠完整函数81个隔离输入已对照；最新本地3/3 native tests通过，含真实Rapier遮挡和8组原extent机器码输入。host在publication19后且Gameplay/已加载ClutterMeshes门内安装；host实际运行和Wall输入仍待验证。生产 host 不调用独立的 add_wall_cpu_systems，后者仍为现有CPU/test装配，无双跑安装。

构建与环境：host `cargo check -p tiny-glade --tests --locked --offline -j3` exit0（1m28s，impl_host_shape_check_fmod.log），使用现有真实导入库 `D:/game/tiny-glade/target/debug/build/country-core-02808306b06a09e4/out`；这是编译检查，不是FMOD运行验收。其后§6 canonical、ivy与point/pins指定批次均已通过。GPU画面/present/Release长稳按用户排除，不作为本轮阻断；不据局部测试称M01完整通过。

当前真实 host 验证：`shape_host_registration_tests` v5 exit0，原a/b/c完整树在Gameplay gate=true执行无操作帧，随后真实sphere widget→picker→press/held/release→权威曲线→publication19→非空decorator reader→history finish35，装饰(2,3,0)映射为(2.5,3,1)，ledger一项、pending清空（impl_host_shape_picker_v5.log）。测试明确供CPU terrain/path/depth/camera输入，无GPU/伪命中/伪资产成功。首轮缺资源、缺path heights、相机未对准实际terrain高程的fixture失败保留日志，未据此改游戏规则。该验证证明一个freehand编辑的真实host组合链，不替代每项原侧全函数差分。新增production帧前缀复用storage.mark_mutable，在所有编号set之前恢复history finish冻结后的写权限。

最终host同一链复验已通过（impl_host_shape_final.log，1m23s编译、0.07s执行）；既有CreateWall保存/undo cursor恢复/applied恢复三个fresh child已通过（impl_save_fresh_process_final.log）。circle resize与非空aux的跨进程扩展正在现入口实施/验证，不以CreateWall结果代替。rectangle reader旧未接近似helper及其自测已删除，真实archive/remap+原fragment4/4通过（impl_rectangle_reader_final.log）。

依赖回归：`cargo test -p utils --lib --locked --offline -j3` 最新177/177通过（impl_utils_full_final_v2.log）。首次176/1失败仅旧2026-10-13 equal4x6预期有误；原PE复跑旧100个输入99一致、此例原false，root已按原证修oracle、不改算法。公共pins最新含真实Wall事件/缺墙与遮挡3/3通过（impl_pins_wall_final.log）。

### 最新可复跑命令（工作目录 D:/game/tiny-glade）

以下 core 命令都显式启用 `native-physics`；pins 的 Rapier 遮挡测试执行真实 native 分支，不是无此 feature 时的恒 miss fallback。日志名与返回结果对应，不从日志没有打印的参数反推 feature。

| 精确命令 | 日志 | 结果 |
|---|---|---|
| `cargo test -p country-core --lib ui_edit_circle::tests --features native-physics --locked --offline -j 3` | impl_circle_canonical_final.log | 5 passed |
| `cargo test -p country-core --lib systems::ui_edit_freehand_walls --features native-physics --locked --offline -j 3` | impl_point_duration_final.log | 1 passed，factory wake→实际animation |
| `cargo test -p country-core --lib systems::ui_clutter::decoclutter_show_pins --features native-physics --locked --offline -j 3` | impl_pins_wall_final.log | 3 passed，含Rapier挡/不挡、真实Wall事件与缺墙 |
| `cargo test -p country-core --lib systems::ui_move --features native-physics --locked --offline -j 3` | impl_move_final.log | 8 passed |
| `cargo test -p country-core --lib systems::ivy::ivy_pruner --features native-physics --locked --offline -j 3` | impl_ivy_final.log | 2 passed |
| `cargo test -p country-core --lib systems::wall::shapes --features native-physics --locked --offline -j 3` | impl_shape_batch2.log / impl_shape_final_batch.log | 各59 passed；其后§6/point变更按上面受影响组重验 |
| `cargo test -p utils --lib --locked --offline -j 3` | impl_utils_full_final_v2.log | 177 passed |
| `cargo test -p system-decorator --lib decorator_on_rectangle_edited --locked --offline -j 3` | impl_rectangle_reader_final.log | 4 passed |
| `cargo test -p system-decorator --lib freehand --locked --offline -j 3` | impl_decorator_final.log | 3 passed，含非空decorator真实编辑/Undo/Redo |

host 命令前在同一 PowerShell 会话设置 `$env:FMOD_LIB_DIR='D:/game/tiny-glade/target/debug/build/country-core-02808306b06a09e4/out'`，复用已有真实导入库，不修改安装文件或 build.rs。`cargo test -p tiny-glade --lib shape_host_registration_tests --locked --offline -j 3` → impl_host_shape_final.log，1 passed；该package的既有依赖配置启用其native后端，未另传feature。`cargo test -p tiny-glade --test toolbar_module_chain history::real_history_buttons_roundtrip_disk_in_fresh_processes --locked --offline -j 3` → impl_save_fresh_process_final.log，旧wall场景三个fresh child通过；circle扩展后日志为impl_circle_save_fresh_process_v*.log，目前原replay0.2/UI0.5阶段期待正在按原证分开验证，旧wall结果不冒充circle持久化通过。

---

## 以下全部为历史记录（可能被当前表和后续原证更正覆盖）

# M01 shape-edit UX — implementation status (owner: impl agent, 2026-10-28)

Spec: `work/m01-scratch/shape-edit-spec.md`. Only READY sections hit.

## Command evidence
- `cargo test -p country-core --lib adjust_wall_height` → **55 passed** (`impl_adjust_wall_height.log`).
- `edit_arrow` → **5 passed**; `ui_move_shapes` → **2 passed**; `ui_edit_circle` → **1 passed**.
- `cargo check -p country-core --tests` → exit 0 for all of the above.

## sign-off (adjust trio)
- **§1.1 `ui_adjust_wall_height` = TESTED** (independent review): transmission/authority/history + height-signifier picture
  PASS; canonical `HistoryCommand::UNDO/REDO`; real replay. §1.2/§1.3 PASS. Trio TESTED, scope = in-source real CPU chain
  (not window/OS input, GPU picture, new-process save/restore).

## §3 `ui_move_shapes` — PARTIAL (case1+case2+case3 done; move_gizmo full body pending)
File `crates/country-core/src/systems/ui_move_shapes.rs`. 30-param PDB signature; `MoveShapeCache`/`MoveShapeEntry`
(map1/map3 + state/activation/shape_rec/variant/curve/committed); `OnShapeMoved` event; `0x142ab7514` dispatch.
- **case3 commit = IMPLEMENTED + unit-tested** (`case3_commit_updates_wall_and_records_undoable_history`):
  `update_wall_state_curve(curve)` → `CompoundEditBuilder::new_edit_sequence → next(ChangeSupports{wall,supports=1}) →
  build` → `History::add` (EditType::Wall, type 7) → emits `OnShapeMoved` (0x30 row) + `ShowPinnedDecoClutter` (0x18 row).
  **Corrected from StateChange → ChangeSupports** per analysis (variant 0x8000…000b); test asserts ChangeSupports. 5 tests.
- **case1 drag = IMPLEMENTED + unit-tested** (`case1_drag_writes_working_record_and_decorator_triples`):
  `archive_current_decorators_in_storages_with_resources(...,"move_shape")` → `decorator_cull_allow_wall` →
  `wall_decorators_mut(wall)` triples `{id, hi(u18)-cx, u24-cz}` → `ShapeParameters::center()` +
  `TerrainHeightsData::sample_at_world_xz` anchor → writes working record: `+0x00 cap/0, +0x08 ptr, +0x10 len,
  +0x18 Δscreen (ray∩y=anchor.y), +0x20..0x3c 28B SP copy, +0x3c anchor.y`; Vec triples owned on the entry.
  (6 tests in module.)
- **move_gizmo_and_trigger_vol**: `camera_rotate_pressed = actions.digital[2].phase < 3` implemented + tested;
  full 8-param body (Named `move_shape`/`move_shape_outline` axis records + activation `Points` + box_trigger_volume
  TriangleMesh widget + hover) NOT yet written. ActionSetStates(#15) slot 2 = RESOLVED.
- **case2 hover-snap = IMPLEMENTED + unit-tested**: added `spatial: Res<WallSpatialHash>` seam (CF: native
  `param_2[4]` ≈ WallSpatialHash; PublicWalls not extended); `SnapProposal`/`nearest_proposal` now `pub(crate)`;
  `hover_snap` = `nearest_proposal` → roof-ridge offset (tag2→-6.0 / tag1→-8.0) → `DebugSphere`+`DebugLine`
  signifiers (NO InnerWalls). Test `case2_hover_snap_finds_candidate_near_wall` builds a spatial hash and asserts
  a candidate. (5 tests pass in this module.)
- 5 unit tests pass. Status: **PARTIAL (case2 + case3 + digital[2]/move_gizmo TESTED-unit) — not module-TESTED, not scheduled.**

## §6 `ui_edit_circle` — PARTIAL (S0/S2-clamp/S3/S4/S5-threshold done; S1/S2-proj/S5-history pending)
File `.../circle/ui_edit_circle.rs`. 30-param PDB signature; `AnimatedCircleSignifierResource` (#[repr(C)] size 64);
`CircleGrabParams`/`CircleSignifierParams` Locals (empty, fields/default UNKNOWN); kept `show_full_circle_signifier`.
- **S0 gate** (mode>5 tail; only Editing(Wall/Roof) proceed) applied.
- **S2 clamp** `clamp_circle_radius` (0.9/8.0) + **S5 threshold** `circle_radius_changed` (|Δr|>0.01, NaN→changed) tested.
- **S3 fit-search** `GladeBorder::search_lerped_shape_that_fits_into_glade` (0x140ad2ce0) implemented + tested: lerp
  `lo→hi` over `t=step/20` (20 steps), returns largest shape inside the glade; Circle via `Circle2d::lerp`
  (native `FUN_140c98170`); Rectangle lerp deferred (§10-12). (2 glade_border tests)
- **S4 apply** `apply_circle_radius` (`Circle2d::new(...).as_curve2(0.2)` → `update_wall_state_curve` →
  `update_wall_state_enclosure` circle snapshot) tested.
- **PENDING (evidence)**: S1 wall source (`UiState`→wall: WallTriggerVolume has NO wall id; UiState has no wall
  accessor; WidgetId→entity→WallEntities mapping unresolved). S2 projection **AVAILABLE**
  (`utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray`) — needs S1 first. S5
  `CompoundEditBuilder/ConcurrentEdits` history wiring.
Status: **PARTIAL (S0/S2-clamp/S5-threshold/S4 unit-tested)** — not module-TESTED.

## §7 `signifier_edit_circle_arrow` — DONE (geometry per flat_arrow evidence; 5 tests)
System/Transform/layout/registration implemented+tested (5). Geometry now uses **`SignifierMesh::Named("flat_arrow")`**
(builder tag 0 → Named asset via `FUN_14092d570`; missing asset panics) + `vertex_color_shader`, record 0x30
`{1,1,Quat,Vec3}` carried by closure; name = `edit_circle_{WidgetId:?}_arrow` with WidgetId at resource `+0x00`.
Rectangle/other lever not applicable. Not yet scheduled (hook step).

## Remaining (await analysis)
- §3 case1 drag — RESOLVED to implement: step5 `param_6+0x00/+0x08/+0x10` = `{capacity, pointer, length}` Vec
  header (element 12B `{u32, f32, f32}`), NOT 6 f32; helpers (archive/cull/wall_decorators_mut) + projection
  formula resolved. Next implement.
- §6 — S1 needs the `edit_circle_wall_{WallId}` widget geometry/params (flow known: iterate PublicWalls → register
  widget → match UiState +0x18/+0x38/+0x28 → WallEntities→Query<&WallTriggerVolume>.is_valid); S5 history wiring
  (`CompoundEditBuilder/ConcurrentEdits`) pending. S2 projection available; S3 `search_lerped…` done.
- move_gizmo — needs full 8-param signature (only given: #5 WallId, #7 signifier_y, #8 stream; #1–4,#6 unspecified);
  body (`box_trigger_volume`/TriangleMesh widget/axis records `move_shape`/`move_shape_outline`/activation 0x28/
  tail anchor=xvy(center,terrain)/rotation=shape) otherwise specified.
- §2/§13 — BLOCKED on PENDING cache-type consumers/producers.
- §4/§5/§8–§12/§14 — analysis paused (scope shrink).
- Step 6/hook: after the 4 partials close, hook them into `install_toolbar_shape_dependencies` at a/c group relative
  positions (no full tree rebuild, no shell deletion) + one real schedule test.

## Checkpoint 2026-10-28（用户要求暂停并保存进度）

> 上面 "Remaining" 一节部分已被本检查点取代（case1 已完成；move_gizmo 8 参已确认）。

**已完成并签核**
- adjust 三件套（`ui_adjust_wall_height` + `_mode` + `history_record`）：**TESTED**（真实输入链、权威、History、真实 UNDO/REDO、空 signifier 画面）
- `signifier_edit_circle_arrow`：**PASS**（`Named("flat_arrow")`、WidgetId 名、组件注册；5 tests）
- `ui_move_shapes`：case1（drag，写 shape_rec / Vec 头 / 装饰三元组）、case2（hover-snap 接 `WallSpatialHash`）、case3（`ChangeSupports` + `History::add(7)`）均实现；6 tests
- `ui_edit_circle`：S0 门控、S2 clamp、S3 `GladeBorder::search_lerped_shape_that_fits_into_glade`、S4 应用、S5 阈值已实现并测试
- `move_gizmo_and_trigger_vol`：`digital[2].phase<3` 门控、smoothstep 已实现

**已知未闭合（暂停点）**
- §3 的 9 个 writer 中目前只有 `OnShapeMoved`、`ShowPinnedDecoClutter` 真正 emit，其余 7 个仅签名（审查线结论）
- move_gizmo 完整 body（8 参已确认；字面规格见 spec §2）
- §6 S1 widget 几何（已定：有效墙 `TriangleMesh(default_tris)`，否则 `Empty`）与 S2 Δr、S5 `MorphPremadeShape` 尚未接线
- 四项均未挂进 `install_toolbar_shape_dependencies`，尚无真实调度测试
- 真实窗口/OS 输入、GPU 画面、保存/新进程恢复、性能均未验收

**下一位接手入口**
- 规格：`shape-edit-spec.md`（§1.1/§2/§3/§6/§7）、`shape-edit-registration.md`
- 审查：`review/shape-edit-review.md`
- 证据方法：PDB `S_INLINESITE`（`llvm-pdbutil dump --symbols --modi=519`）定位内联读取；`flat_arrow`、`search_lerped` 已定位

## Logs
`impl_{adjust_wall_height,edit_arrow,ui_move_shapes,ui_edit_circle}.log`.

## 2026-10-28 resumed implementation (first batch, review pending)

- Reused the existing `systems/ui_move_shapes.rs`, `box_trigger_volume`,
  `WidgetParams`/`UiState`, and signifier factory/builder; no renderer changes.
- `move_gizmo_and_trigger_vol` now has the eight semantic arguments and returns
  the real widget context. It registers the native box mesh even while rotating
  the camera; digital slot 2 changes sound slots instead of suppressing the widget.
- Corrected the old spec's field names: public +0x30 is the source curve length,
  +0x280 is elevation. The helper reads the owned `PublicWallState` fields.
- Added activation-line Points and two Named-axis factories, original formatting
  names and hover scale from the analysis worker's B rodata/disassembly evidence.
  Original workers further require axis scale ×1.5, tag-controlled UiPass,
  duration 0 with default animation flags, and `dashed_line` plus the existing
  DashedLineLength component for the activation line. These are now implemented.
- `cargo check -p country-core --tests --locked --offline -j 3`: exit 0, 59.39s
  (`impl_move_gizmo_check.log`). Registration/box/sound-switch test: 1 passed,
  0 failed (`impl_move_gizmo_tests.log`), before the factory additions.
- First factory test failed (`impl_move_gizmo_factories.log`, 1 passed/1 failed):
  it checked target scale before the real animation stage. Original flags require
  initial scale 0. The corrected test explicitly verifies both initial state and
  real animation-stage target; 2 passed, 0 failed in `impl_move_gizmo_animation.log`.
- Fresh native evidence proves the supposed Local map1/map3 are PublicWalls
  records/roof associations. The production system now registers the selected
  public wall before examining drag phase (including an empty Local); anchor,
  orientation and signifier Y use the actual shape/terrain/elevation fields.
  Native dispatch is `dragged().wrapping_sub(2).min(3)`.
- The obsolete Local representation and actual grab/release transaction remain
  pending; no fake producer was added. The incorrectly attributed
  ChangeSupports commit/helper/test were removed: native ChangeSupports belongs
  to the separate support-toggle widget, not drag release. Do not claim undo.
- Latest real-system no-interaction verification is running in
  `impl_move_shapes_entry.log`; result pending. The host hook remains pending.

## 2026-10-28 circle implementation continuation (partial)

- `ui_edit_circle`: case1 now archives Wall+Terrain and allows wall decorators for culling; real picker -> ui_update JustPressed test checks archives and culling. Terrain-bucket fold is still pending.
- Held computes cursor projection, original RNE rate 10.0, clamp, border result, threshold and authority update; emits observed HideTrees and material audio. Real input Held test changes authority radius. ShowPinned and building audio remain TODO; no complete event-chain claim.
- `as_curve2` step corrected from old spec 0.2 to 0.5 (B original caller 0x14165f5ac, callee 0x140c97cf0 ABI); min/mid/max locally verify authority curve point count and coordinate bits against 0.5 sampling. Original side geometry differential remains pending.
- Release unchanged/NaN restores archived decorators and sends finished tree event. Actual input release test covers unchanged published radius and restoration; changed release/history/decorator position adjustment pending.
- Border fit search corrected to stop at 19/20, native 0x140ad2ce0. A fully-inside target regression passes. Circle threshold NaN now restores (native UCOMISS/JBE).
- Tests: `cargo test -p country-core --lib ui_edit_circle::tests --locked --offline -j 3` 5 passed after 0.5 correction; targeted real input test 1 passed after release addition; border search test 1 passed. No original full-system differential or module acceptance.
- Original RNE `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe` SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`: VA 0x1429345a8 bytes `00002041` = 10.0f. Static B patched evidence remains bound to `cf048e27...6323`.
- Follow-up: changed release now maps tag0/1 wall decorator positions onto the authoritative circle, marks all entries before tag dispatch, records MorphPremadeShape and emits Hint/Sheep/finished trees. A real UI-state -> held -> publication -> release schedule test passes; it manually starts History and does not verify finish/undo/redo. Missing grab panics; tag2 modification tracking checked. Earlier unchanged-release test passed, but that same test was later extended/replaced for changed release, so unchanged branch currently lacks a dedicated retained test.
- `ui_move_shapes`: corrected case1 position to Info+0x14/+0x1c, tag0/1 collected, tag2 skipped, tags3/4 panic. Removed false Local map gate, registered the four tuple groups as a Bevy system, and verified actual Schedule -> Started -> archive/cull using injected UiState. Helper tests cover all tag outcomes; 7 move tests passed before added panic cases, targeted helper including panic cases passed. Real picker input, held/release, decorator event consumer/history remain outstanding.
- Move release now sends typed `OnShapeMoved { offsets, wall, center }`, Sheep, `MorphPremadeShape`, and finished-tree event. Same actual Schedule Started/release test checks payload and pending history; its UiState and History start are injected, and no undo/redo or held movement is claimed.
- Existing `system-decorator::decorator_on_move_shape` now reads the core event; its plugin entry registers the event and consumer, with a real event/schedule/storage test. `cargo test -p system-decorator --lib decorator_on_move_shape --locked --offline -j 3`: 4 passed. Producer host registration and relative order remain unverified. Previous parallel `MoveShapeEvent` type removed; rg found no other callers.
- Native evidence now disproves move case2 `nearest_proposal`: it requires a downward physics ray and selected-wall exclusion followed by roof ridge snapping; existing collision exclude API is explicitly incomplete. Case2 remains unclosed and current behavior is not accepted.
- Native collision: B 0x1408ca9a0 wall predicate checks source tags 0/1/2/8 and wall id +8. `raycast_simple_excluding_wall` passes this predicate into the existing shared Rapier 17-slot loop; the old handle-typed compatibility method remains unchanged and must not be mistaken for wall exclusion. `cargo test -p country-core --lib finite_simple_ray_regressions --features native-physics --locked --offline -j 3`: 6 passed including same-slot farther wall and cross-slot default/filtered behavior. Move held has not yet been wired to this API.
- Move real picker/input -> start recording -> held authority update -> publication -> release -> finish recording -> canonical undo/redo now passes for a circle without associated roof, including circular-border clamp. Verifies wall center, not decorator replay; decorator event consumer is only separately tested. Rectangle boundary, real roof snap, ShowPinned and move audio remain incomplete. Move case2 no longer executes the disproved nearest-proposal approximation.
- Roof snapping depends on selected public +0x284 for ray height, but current `building_projection()` does not populate it. Source unresolved; +0x288 is `authority.arch_height`. Do not substitute zero or the adjacent elevation.
- Follow-up move: native-physics actual picker/input path now exercises selected-wall exclusion, circle and rectangle roof ridge, disable-snapping, vertical disjoint and NaN roof/ray behavior. First-entry sound and ridge line factories checked. The NaN ray outcome is the reconstructed Rapier backend, not original game execution. Candidate now derives from current published shape; both rectangle boundary branches and inside-only cursor audio are implemented but rectangle circle-border lacks direct native fixture and full UI test. `ShowPinned` remains TODO.
- `building_projection` +0x284 now publishes height bounded by source +0x65; original RNE SSE fixture compares ±qNaN/±sNaN by bits and local bounds tests pass. Source +0x65 maps to `source_flag_65` / clutter_version in the existing WallState reader.
- Rectangle edit consumer now consumes the last request and writes original material/building audio; local tests cover both flags, dt=0 and NaN. Existing dimension/orientation producers remain non-native input shells; their explicit audio fields are not proof of real UI wiring.
- Move real history start/finish and canonical undo/redo test validates wall center only. Separate system-decorator event test validates position write; combined consumer/history/undo path remains pending.

- §10: real edge picker/input feeds do_dims/Slider3d, edit_rectangle, finish and native history start/finish/process/replay; rectangle center and size undo/redo pass with empty decorator storage. ShowPinned, non-empty decorator replay, host scheduling and original system-level differential remain open. The decorator consumer now uses the 80-byte event and Wall/Terrain archival, but is not registered pending verification of its newly recovered remapping formula and scheduler order; system-decorator focused tests pass 6/6. §11 corner geometry has a finite published-path test only; its producer remains incomplete.
- §11: real corner producer and rotate transaction replace the synthetic orientation input. Native PE angle fragment 11 samples match by bits after normalizing each vector. Tests cover actual picker, press/held/release, authority update and real history undo/redo, and Wip-vs-event wall/corner. Current module tests 10/10; ShowPinned, non-empty decorator replay, original full-system differential and actual host schedule remain open. Smooth-helper edge cases are under original-instruction review.
- §12: replaced the four invented preview lines with the 56-byte Curve2 component, active edge factory (PE vtable 0x142aa4cb0), active Transform updates and animated curve mesh. Real Schedule/factory wake and range/degenerate failure tests pass 2/2; the specific 0x141466bb0 entry has static correspondence only, not original-side full-function differential, and host installation remains pending.
- §4: B-owned freehand move system now has actual picker/start/held/border/release and canonical wall history undo/redo test, with Wall/Terrain archive and terrain ID fold. ShowPinned Wall, cross-crate decorator replay and host schedule remain open. §5: freehand point picker/producer, real OnFreehandWallEdited consumer and point factory are implemented. A cross-crate real picker→published curve→nonempty decorator consumer→history wall/decorator undo/redo test passes, with frame-prefix mark_mutable. ShowPinned, original full-system execution, host ordering and point factory default animation duration remain unverified; factory is partial.
- §9: replaced invented RotateCircleCmd with real selected-circle UI/drag transaction, Wall-only decorator archive, 0.5 curve sampling, roof notification, real OnCircleRotated/ivy reader and Morph history. Real picker→history undo/redo and release sound test passes. CurrentAngleGizmo phase-specific target/smoothing and sound were corrected from PE; two arrow factories await exact name formatter, ShowPinned is pending shared owner, and full system native differential/host scheduling remain open.

## 2026-10-28 typed pin producers and host shape registration (review pending)

- Replaced pending ShowPinned sites in move premade/freehand/edit freehand/circle radius/circle rotation/rectangle dimensions/orientation with the shared typed event. Heights come from published elevation/max_y; premade move uses its candidate before border fitting, circle radius uses published circle with flag=false, rotation uses published circle with flag=true. Freehand uses Wall. Event order follows native static call sites.
- Circle rotation now writes both corner_arrow factories with the original shared name, Display f32 scale, explicit unmultiplied scale, flags 0x100, duration zero and tag-controlled UiPass. Extended its real picker/history test to wake both factories and inspect components; not yet executed in this batch.
- Host installs the original a/c nested chain_ignore_deferred and b parallel tuple groups separated by chain, set17 and existing gameplay gate. Removed standalone adjust/raise registration. Added real shape resource/event owners and the three decorator event consumers after publication19 and before history finish. The latter ordering is required reconstructed engineering ordering; original consumer set remains unproven.
- Production host does not call add_wall_cpu_systems; that independent CPU/test assembly remains unchanged. No renderer changes. Native tests/host compilation remain pending; this is implementation/registration work, not leaf or M01 acceptance.
