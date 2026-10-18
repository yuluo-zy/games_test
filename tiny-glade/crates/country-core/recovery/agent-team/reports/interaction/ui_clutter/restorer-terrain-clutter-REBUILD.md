# 逐函数重建 — systems/ui_clutter（restorer 本轮）

重建者: restorer-terrain-clutter
日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md + 2026-10-17-recovery-sop.md
证据（fresh，本轮重跑）: recovery/agent-team/work/restorer-terrain-clutter/jobs/
`fresh12-result.json` + `decomp/`（5 个 ui_clutter 函数，shared 只读）。

## 分类结论

5 个目标函数全部为**真实业务函数**（重建）。`history_record_auto_clutter`
（判等）未动（红线）；`register_required_components` / `clone_behavior` 为 bevy 胶水。

## 函数表

| VA | 原函数 | 重建后 verdict | 本轮改动 / 关键判等 |
|---|---|---|---|
| 0x140976ca0 | handle_rmb_autoclutter | **判等（适配面登记）** | 旋转 = atan2f 链 + fmod + 半角 sin/cos（fresh 复核，identity 四元数 → π/2 已入测）；入口 `*(int*)(in_RDX+0x18)!=0` 门 → [`RmbAutoClutterDirect`] 直达、跳过 probe；probe 未中 `option::unwrap_failed` panic；0x48B 事件 tag=2@0/subtype=3@+0x29/rotation@+0x2c/type@+0x3c/entity@+0x40；**found 路径直接 `History::start_input_recording` + `History::add(tag 0xd)`（无条件）**，删除经 defered 事件的间接链。 |
| 0x14099ca60 | ui_convert_autoclutter_to_decoclutter | **部分判等（登记）** | **函数内维护 `HashMap<DecoratorId,Vec<DisplayChildAutoClutterInfo>>`**（`maintain`：已有键 push/新键新建 Vec）；父类型 `<3` 门；DisplayChild 0x60 字段（mesh 4 qword + transform 6 qword + +0x50）；**删除编造的 InstaCreateDeco/InstaCreateStairPoint 事件**；尾部模式门 `disc==0 ∨ Eraser(Bulldoze)` 按 fresh `disc-6<0xf→disc-5` 实现。**残留**：0xa0B/0x60/0x30 宿主记录表以 [`AutoClutterChildTable`] 适配；尾部 raycast→DecoratorStorage→UiState::add→0x60 类型 3..5 子跳表登记。 |
| 0x1409a1f50 | debug_spawn_all_clutter | **判等（entity 计数 I）** | 两循环 `History::start_input_recording`；螺旋 `r=sqrt(n)`、`θ=n*2.3998277`、cos/sin、类型表 `AVAILABLE_DECOCLUTTER[n]`、地形高度采样；**螺旋 rotation@+0x2c=0**；**单条位置/旋转来自事件载荷（uStack_9a 两段 8B 写覆盖 +0x2a..+0x39），不采地形高度**（新增 `DebugSpawnClutterCmd.rotation`）。entity 计数原生=Events 内部 +0x40，crate 独立计数器登记 I。 |
| 0x1412357c0 | decoclutter_show_pins | **部分判等（登记）** | 类型门 `d=[0xe]-0x38; 7<d ∨ d==6`；几何 `expand(0.5³)→set_orientation_xz→translate→intersects`；触发判别 kind 1/0/2 的三层 intersects/contains_point/半径门；`[+0x20/+0x28/+0x30]` 标志去重；双 Vec（pinned/hits）+ 0x18 stride 动画累加（0.3 / −0.2，clamp[0,1]，>0.75 翻转周期）。**残留**：GlobalId 哈希探测（kind 2）结果由宿主提供；SparseInstanceBuffer ChunkId 写口、spawn/despawn(DecoClutterLock,Ui2dWsExtent,Ui2dImage) 依赖宿主 ECS，登记。 |
| 0x1409a92c0 | ui_focus_decoclutter | **部分判等（登记）** | 门 `disc!=3 → return`；d2s 探测→`DecoratorStorage::get`→null/`[+0x48]==0` 早退；**新增类型门 `info.ty`：`d=ty-0x38; d<8 ∧ d!=6 → return`**；多路事件按 `is_active`/phase 门控：click phase==3 → RequestAppModeChange + PlayDecoratorSoundCmd，rotate active → RequestRotateDecorator，move active → RequestMoveClutter，SignifierFactory→UiAppendSignifier。**删除编造的单个 DecoClutterFocused 事件**。**残留**：几何（Rectangle3d/clutter_mesh_ty_data/DebugRectangle3d::show）、相机 xz_projective_rotation、多 widget 名队列由宿主提供（[`DecoClutterFocusUi`] 适配，I）。 |

## 编译/测试

- `cargo check -p country-core --lib`：**通过**（0 error）。audio/save 域并发 WIP 曾短暂阻断整库，
  已按 SOP 记录归属、未代改；收敛后复跑通过。
- 新增单测（systems/ui_clutter/tests.rs）：
  1. `rmb_direct_entry_adds_history_unconditionally_and_rotates`（直达门→History tag 0xd→
     旋转 π/2）；
  2. `ui_convert_maintains_parent_hashmap_for_type_below_three`（<3 入 map、>=3 跳过）；
  3. `debug_spawn_single_uses_payload_rotation_and_batch_uses_zero`（单条 rotation=1.5/
     位置来自载荷；批量 42 条 rotation=0）；
  4. `show_pins_applies_type_gate_geometry_and_dedup`（0x40 命中、0x38 跳过、toggle）；
  5. `focus_decoclutter_gates_disc_three_and_decorator_type`（disc=3 + ty 门 + 多事件）；
  另有 `decoclutter_show_pins::tests` 2 条（类型门 + 几何序列）。
- 实测：`cargo test -p country-core --lib systems::ui_clutter` → **9 passed / 0 failed**
  （7 新增 + 既有 2）。

## 登记（解锁条件）

1. 0xa0B clutter 记录 / 0x60 父记录 / 0x30 变换表布局：解锁 = 宿主实体表类型合同。
2. decoclutter_show_pins 的 GlobalId 哈希探测与 SparseInstanceBuffer/实体 spawn：
   解锁 = render sparse buffer 写口 + ECS Commands 接线。
3. ui_focus_decoclutter 几何/相机与 UiState 多 widget 名队列：解锁 = 宿主 UI 名队列 +
   相机矩阵资源；载荷语义（RequestRotate/Move amount）待取证。
