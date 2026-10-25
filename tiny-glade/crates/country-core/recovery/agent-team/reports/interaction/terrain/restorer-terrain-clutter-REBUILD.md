# 逐函数重建 — systems/terrain（restorer 本轮）

重建者: restorer-terrain-clutter
日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md + 2026-10-17-recovery-sop.md
证据（fresh，本轮重跑）: recovery/agent-team/work/restorer-terrain-clutter/jobs/
- `fresh12-result.json` + `decomp/`（12 函数 get_functions，shared 只读，fields=decompiled_code,callees）；
- `disasm-0/1.txt`（0x1409ea800 / 0x140a9fdc0 disassemble_function）；
- `readmem*-result.json`（画布名字串/常量/格式串 RNE 直读）；
- `closure-1409a83*/1409a86*/1409a88*/1409a8b*.txt`（focus_mode 闭包）。

## 分类结论

7 个目标函数全部为**真实业务函数**（重建），无宏胶水。判等函数
（delayed_terrain_changed_msg/on_terrain_modified/history_record_terrain/
TerrainHeightsData 采样族/insert_or_morph_last_point）未动（红线）。

## 函数表

| VA | 原函数 | 重建后 verdict | 本轮改动 / 关键判等 |
|---|---|---|---|
| 0x14094e000 | edit_terrain_stroke | **判等** | fresh 逐语句复核：两事件循环（0x30/0x28 stride 双缓冲 unread）、points[0] 双 panic、magnitude=`delta*sqrt(dz²+(dy²+dx²))/0.001`、apply_resolved 分派、stairs tag 1/2/3/4 偏移（+0x0c pos、+0x18/+0x1c 值、+0x20 幅度）、8B 事件→LOCK INC 顺序全部吻合；仅修正点集循环错误串（“set points of terrain stroke …” vs 第二循环 “edit a terrain stroke …”）。无 PlayTerrainSound 载荷事件。 |
| 0x140a03ba0 | terrain_stroke_changed | **判等** | unread 门 = 双缓冲任一非空（≡ `changed.read().last().is_some()`）；8B unit 事件（原生 WriteSettingsFileCmd，crate 以 RecalculateHeightMapCmd 承载）→ readback LOCK INC → StairsPlacementBucket tag=0（+8=0）。 |
| 0x140aff720 | terrain_hover_idx | **判等** | 开头无条件 `tag=0`；门 `disc<0xb ∧ (0x421>>(disc&31))&1`；id@+0x10；高度两分支（≤0：id!=-1 写；>0：hover&1 ∧ id!=-1 ∧ dist>0.5）；输出 {tag u64@0,id i64@8}。资源/事件适配面登记 I。 |
| 0x140a9fdc0 | ui_draw_terrain | **部分判等** | 按 disasm 重建：画布 1 = `height>0 ∧ ((tag&1)==0 ∨ pos.y≤0.5)` → `clearing_canvas_w_layer_and_sound("terrain_deny_draw",0x11,0)`；画布 2 **无条件** `"terrain_draw",0x0c,3`；`Circle2d::new(center,0.8)` + `GladeBorder::is_shape_inside`；非界内写 8B 事件，且 `widget.dragged()!=2` 再写一条；subtype=`dragged()`；u8 回绕状态机；subtype==3 写 drag 距离。**残留**：`&DAT_142adc498` 跳表目标 UNRECOVERED（state 1/2/3 语义按既有登记推断）。 |
| 0x140aef1f0 | ui_edit_terrain | **部分判等** | 逐笔画 widget（名字 `"terrain_edit_{}__canvas"`，RNE 直读两段字面）+ type 字节 `(hovered_id!=id)*3+4`、cursor.tag!=0 时固定 7；每 widget subtype=`dragged()` 的 switch（2 无动作/3 WindowFocus+UI 槽/4 8B 事件+stairs 桶两分支+累加器>0.3/5 TerrainEdit 哨兵 `0x8000000000000001` 且写后校验旧值 `…0002`/6 子跳表/8,0xc,0xf 响应表）；删除 16 位脏掩码自造近似。**残留**：subtype 6 的 `&DAT_142ae405c` 子跳表 fresh UNRECOVERED（旧 ReVa 名表仅登记）。 |
| 0x1409a4fb0 | ui_terrain_focus_mode | **部分判等（登记）** | 门 `!=5`；无笔画路径：raycast tag 0x11→f32::MAX 否则 3D 距离，条件 `hover_dist<ray_dist ∧ selection.tag==1 ∧ 上一聚焦值≠value` → WindowFocusEvent；有笔画路径：点集 min/max→中心、`scale=h*7+(1-h)*2.5`、4 个 signifier 名（vertex_color_shader / brush_arrow_flat_to_single_forward / brush_arrow_flat_to_w_flat_single_up / 动态名）+ closure$1 DrawTopology insert。**残留**：fresh 反编译被 120000 字符截断；CircularF32 焦点状态机/CameraMatrices::xz_projective_rotation/RequestAppModeChange 点击路径依赖宿主相机与实体机制，登记待接。 |
| 0x1409ea800 | terrain_rock_mask::sample_at_world_xz | **判等** | disasm 证实：DIVSS 130、MAXSS 0/MINSS 1、MULSS 512、CVTTSS2SI+饱和、`min 0x1ff`、`index=(zi<<9)|xi`、`(index>>5)<len` 否则 `panic_bounds_check`、`MOV EAX,[RCX+word*4]; BT EAX,xi; SETC` = **按位读取**。 |

## 编译/测试

- `cargo check -p country-core --lib`：**通过**（0 error；警告均来自 save/snapshot.rs 等他组文件）。
  期间 audio/save 域的并发 WIP 曾短暂阻断整库，已按 SOP 记录归属、未代改；其收敛后复跑通过。
- 新增单测（systems/terrain/tests.rs）：
  1. `ui_draw_canvas_gate_and_prev_wrap_match_fresh_state_machine` — 画布 1 gate、
     画布 2 无条件、`(prev-1)>1` u8 回绕（0→1、1→2、subtype 2→0）；
  2. `ui_edit_widget_type_byte_is_three_times_id_mismatch_plus_four` — 4/7 类型字节与别名；
  3. `rock_mask_reads_u32_word_bit_and_panics_out_of_bounds` — 位读取 + 越界 panic；
  4. `terrain_focus_mode_no_stroke_path_writes_window_focus_once` — raycast/门/事件。
- 实测：`cargo test -p country-core --lib systems::terrain::tests` → **5 passed / 0 failed**
  （4 新增 + 既有 1）。

## 登记（解锁条件）

1. ui_draw_terrain 末尾跳表 `&DAT_142adc498`：解锁 = PyGhidra 恢复 4 个目标函数体
   （或 ReVa High P-code 定点）。
2. ui_edit_terrain case 4 子跳表 `&DAT_142ae405c`：解锁 = 宿主 UI 参数名队列 +
   子跳表目标恢复。
3. ui_terrain_focus_mode 复杂体：解锁 = 完整反编译（>120 KB，分段/ReVa）+ 宿主相机/
   实体机制；当前按 closures 与截断段落地。
