# 批次 3 业务逻辑复核：systems/wall-constructor

> 复核对象：`upper-modules-plan/systems/wall-constructor/ANALYSIS.md`（A 主干/分段/拱/立面支撑 + B 门/栅栏/地板/楼梯/半木/utils）
> 对照骨架：`skeleton/B/crates/systems/wall-constructor/`（PDB 符号 + 编译实例 RVA）
> 证据源：B patched EXE（sha256 cf048e…323），Ghidra 共享只读反编译（本次复核 spot-check 8 函数，未写库）
> 日期：2026-10-15

---

## 1. 复核范围与结论摘要

对 `system_wall_constructor::` 全部 **105 条 compiled_procedure（发射）符号**（剔除 lib.rs 3 条重复记录与 2 条同 RVA 复用后 ≈100 个唯一发射过程）逐条与 ANALYSIS.md 比对。结论：

- **覆盖**：除 2 个 `ArchWalker::walk` 单态外，全部发射函数在 ANALYSIS 中均有条目/地址且一一对应；**ANALYSIS 标注的每个地址都与骨架「编译实例 RVAs」精确一致**（抽查 8 函数反编译全部对上）。
- **可复用标注**：A/B 两部分的 O 标注引用的 reconstruction/building/* 文件与 evidence/building-* 目录**全部存在**，且 `propose` cost 公式、`walk_profile` count/尾补公式、`construct_regular_wall_arches`/`construct_freehand_wall_arches`、`prepare_wall_path`、`wall_path_segmentation` 等关键函数签名与 ANALYSIS 描述逐字吻合。O 标注可信。
- **已知三卡点**：全部经复核确认属实（详见 §5）。
- **硬伤**：无。存在 1 处发射函数遗漏（两 walk 单态只被 A 未知#8 交叉指认、B 部未实际展开）与若干内联体未点名（属「未发射」，按口径不计遗漏，但建议登记）。详见 §2/§4。

整体评价：A/B 两子范围分析质量高、地址自洽、与既有恢复证据一致，可作为 wall-constructor 族实现的权威基线；少量收尾项见 §6。

---

## 2. 覆盖核对表

口径：只列 `compiled_procedure`（发射）符号；闭包计入但标注；`inlinee_declaration`（未发射，内联进宿主）单独列于表末备注。✓=有条目且地址对应；△=仅交叉指认未展开；「未发射」=仅 inlinee 声明无独立 RVA，不计遗漏。

### A 主干/分段/拱/过梁/立面支撑

| 骨架符号 | RVA | ANALYSIS | 状态 |
|---|---|---|:--:|
| ArchWalker::new | 0x74c2d0 | A1.3 | ✓ |
| ArchWalker::walk<arches closure_env$3> | 0x1232bb0 | A1.4 | ✓ |
| ArchWalker::walk<lintel emit_support_beams closure_env$0<closure_env$2>> | 0x1233380 | A1.5 | ✓ |
| **ArchWalker::walk<brackets closure_env$7>** | **0x1231ea0** | 仅 A未知#8「属子范围B」；B5.3 未展开 | **△ 遗漏** |
| **ArchWalker::walk<arches_n_walls closure_env$6<false>>** | **0x48ab50** | 仅 A未知#8；B5.1 未展开 | **△ 遗漏** |
| construct_arches | 0x75f680 | A3.1 | ✓ |
| construct_arches::closure$3（拱砖发射） | 0x1231af0 | A1.6 | ✓ |
| ArchFunction::remap_t | 0x765300 | A3.3 | ✓ |
| create_stone_arch_profile | 0x7645f0 | A3.2 | ✓ |
| construct_circle_bottom_layer | 0x124b2c0 | A4.7 | ✓ |
| add_bottom_brick<circle> / <rect>（同 RVA） | 0x1fe1de0 | A4.8 | ✓ |
| circle_categorize | 0x124af60 | A4.6 | ✓ |
| construct_circle_pillars | 0x124ba30 | A4.9 | ✓ |
| construct_turret | 0x124cc30 | A4.10 | ✓ |
| find_overhang_support_segments | 0x1248620 | A4.16 | ✓ |
| OverhangSupportFinderCache::new | 0x1248300 | A4.17 | ✓ |
| maintain_stone_column_colliders | 0x78a6e0 | A4.18 | ✓ |
| overhang_subscribe_to_tabloids | 0x786440 | A4.2 | ✓ |
| detect_overhang_support_segments | 0x7867d0 | A4.1 | ✓ |
| detect_overhang…closure$2 | 0x787d80 | A4.1 | ✓ |
| construct_circle_supports | 0x788160 | A4.3 | ✓ |
| construct_rectangle_columns | 0x788a80 | A4.4 | ✓ |
| construct_rectangle_brackets | 0x7896b0 | A4.5 | ✓ |
| construct_rect_bottom_layer | 0x124eb80 | A4.11 | ✓ |
| construct_rectangle_large_brackets | 0x1253fc0 | A4.12 | ✓ |
| construct_rectangle_pillars | 0x124fcb0 | A4.13 | ✓ |
| find_stone_column_start_y | 0x1250720 | A4.14（卡点） | ✓ |
| construct_rectangle_small_brackets | 0x1250da0 | A4.15 | ✓ |
| construct_crossbrick_pillar | 0x2418f50 | A4.21 | ✓ |
| construct_wooden_pillar_w_stone_base | 0x1fe2150 | A4.19 | ✓ |
| construct_brick_columns | 0x1fe3240 | A4.20 | ✓ |
| construct_lintels | 0x76d9f0 | A3.4（O/I） | ✓ |
| add_diagonal_support | 0x1243740 | A3.6 | ✓ |
| place_support_beams<closure_env$2> | 0x1242530 | A3.5 | ✓ |
| SupportBeamBuilder::emit_support_beams | 0x1243c20 | A3.7 | ✓ |
| SupportBeamGrower::propose | 0x1fe09c0 | A3.9 | ✓ |
| construct_platform_stairs | 0x7744d0 | B4 | ✓ |
| construct_walls | 0x757800 | A1.2 | ✓ |
| init_wall_height_and_arch_segment_height | 0x78b550 | A1.7 | ✓ |
| debug_force_update_arches | 0x3bb80 | A1.8（卡点，已验证 1B ret） | ✓ |
| gpu_clear_wall_gate_pieces | 0x773c00 | A1.9 | ✓ |
| gpu_flush_plank_buffers | 0x774010 | A1.10 | ✓ |
| assemble_stone_pillar | 0x120d670 | A4.24 | ✓ |
| PillarProposals::add_proposal | 0x7534d0 | A4.23 | ✓ |
| place_pillars | 0x753670 | A4.22 | ✓ |
| place_pillars::closure$4 | 0x120d340 | A4.25 | ✓ |
| calc_path_wall_segmentation | 0x74f7a0 | A2.2 | ✓ |
| calculate_path_wall_segmentation | 0x74f310 | A2.1 | ✓ |
| process_segment（分段） | 0x750420 | A2.3 | ✓ |
| add_splits_in_empty_range | 0x751240 | A2.4 | ✓ |
| score_split_candidate | 0x751610 | A2.5 | ✓ |
| sample_wall_path_intersections | 0x74c720 | A2.7 | ✓ |
| split_wall_path_segment_into_platform_stairs | 0x751960 | A2.8 | ✓ |
| calculate_wall_path_segments_impl | 0x122ff90 | A2.6 | ✓ |
| WallConstructor::from_curve | 0x769760 | A1.1 | ✓ |

### B 门/栅栏/地板/楼梯/半木/抹灰/UI/utils

| 骨架符号 | RVA | ANALYSIS | 状态 |
|---|---|---|:--:|
| construct_wooden_gate_doors | 0x1210110 | B1.1 | ✓ |
| DoorMeshInProgress::add_quad | 0x1214ed0 | B1.2 | ✓ |
| update_gates_hashmap | 0x755690 | B1.3 | ✓ |
| construct_gates | 0x756370 | B1.3 | ✓ |
| construct_fences | 0x765830 | B2 | ✓ |
| emit_fences<closure_env$0> | 0x12388f0 | B2 | ✓ |
| process_segment<closure_env$0>（栅栏） | 0x1238d80 | B2 | ✓ |
| split_curve_based_on_angle | 0x1fdf9e0 | B2 | ✓ |
| construct_floors | 0x7661e0 | B3 | ✓ |
| construct_playermade_stair_brackets | 0x7834f0 | B5.3 | ✓ |
| playermade_stair_floating_bricks | 0x77a2f0 | B5.4 | ✓ |
| construct_playermade_stairs_arches_n_walls<true> | 0x496800 | B5.1 | ✓ |
| preprocess_curve | 0x780e20 | B5.2 | ✓ |
| preprocess_curve::closure$0 | 0x782f50 | B5.2 | ✓ |
| arches_n_walls::closure$6<true> | 0x48b320 | B5.1 | ✓ |
| construct_playermade_stair_pillars_visuals | 0x77d520 | B5.5 | ✓ |
| curved_halftimber_maintain_entities | 0x7706d0 | B6.1 | ✓ |
| wooden_floor_borders | 0x1244310 | B6.2 | ✓ |
| maintain_plaster | 0x75b810 | B7.2 | ✓ |
| create_plaster | 0x75d460 | B7.1 | ✓ |
| generate_plaster_mesh_and_entities | 0x12269b0 | B7.4 | ✓ |
| split_wall_into_sides | 0x75dc60 | B7.3 | ✓ |
| do_ui_arch_controls | 0x767b70 | B8 | ✓ |
| ui_arch_controls | 0x766e70 | B8 | ✓ |
| do_ui_arch_controls::closure$1 | 0x7690d0 | B8 | ✓ |
| ui_lintel_controls | 0x771a60 | B8 | ✓ |
| ui_lintel_controls::closure$1（与上共用 RVA） | 0x7690d0 | B8 | ✓ |
| add_underwindow_pattern | 0x1fd1c30 | B9.6 | ✓ |
| half_timbering | 0x1218e30 | B9.1（卡点） | ✓ |
| overlapping_windows | 0x12218a0 | B9.3 | ✓ |
| roof_outline | 0x1217be0 | B9.2 | ✓ |
| get_window_aabb | 0x1221da0 | B9.4 | ✓ |
| check_for_overlap_with_all_windows | 0x1221f70 | B9.5 | ✓ |
| perturb_row_ends | 0x1240d30 | B9.13 | ✓ |
| split_horizontally | 0x1240ec0 | B9.14 | ✓ |
| resolve_hole_overlap | 0x12411c0 | B9.9 | ✓ |
| split_into_random_rows | 0x123e110 | B9.12 | ✓ |
| stairs_signifiers_new | 0x1223d10 | B9.16 | ✓ |
| trim_rows | 0x123eef0 | B9.10 | ✓ |
| above_to_below | 0x123fab0 | B9.11 | ✓ |
| below_to_above | 0x123fb70 | B9.11 | ✓ |
| add_wall_corners | 0x1215540 | B9.7 | ✓ |
| add_wall_corners::closure$1 | 0x1217a20 | B9.7 | ✓ |
| add_skirts | 0x12221f0 | B9.8 | ✓ |

### 非业务/派生（不比对，共 ~15 条）
`impl$4::default`、`impl$0::default`（fences/floors/magic_support）、`impl$1::default`（gates/ui_arch）、`impl$0::default/impl$2::default/impl$7::default/impl$8::clone`（lib）、`impl$1::clone/impl$2::clone`（place_pillars）、`impl$2::register_required_components`、`impl$0::drop`（plaster）、`impl$0::clone`（segmentation）、`impl$5::default`（wall_constructor）。均按 derive/bevy 胶水复用，不计业务。

### 未发射内联体（inlinee_declaration，不计遗漏，按建议登记）
已被 ANALYSIS 点名解释：normalize_holes / select_native_row_holes / split_plain_row_horizontal / consume_wall_proposal / emit_resolved_wall_brick（A1.1）、add_split（A2.4）、crashy_resample / place_fence_piece 语义（B2 覆盖）、process_loop_range_into_wallpath_segments / update_phys / add_colision_geo（B4）、construct_pillar_stub（B5.5）、maintain_deferred_plaster_buffer_disposal / mirror_holes_as_needed / WallSide::new / sd_box（B7）、check_for_hole_overlap（B9.9）、find_corners / check_angle（B9.7）、split_height_into_skirts（B9.8）、prepare_lintel_support / lintel_support_instances / accumulate_lintel_third_heights / rdp_support_indices（A3.4）、BrickRowSine::new / sine_wave_deform / random_f（B9.15）。

**未点名但属业务的内联体**（宿主已分析、内联逻辑未单独登记，建议后续补齐）：
`emit_segment_bricks`(inst=8)、`curve_to_arch_ws`(arch_walker)、`create_wall`(construct_walls, 行424)、`create_stone_arch_profile_impl`(profile, 行62)、`SupportBeamGrower::accept`(行532)、`wooden_lintel_profile`(profile, 行12)、`generate_plaster_mesh`(行316)、`vec_push_within_capacity_assert`(行972)、`emit_pillar_at_point`(place_pillars, 行276)、`PillarProposals::empty`(行82)、门族 `door_mesh/door_mesh_gap/door_bottom_curve/offset_door_bottom_curve/segment_length/place_brick_steps/place_ladder/DoorMeshInProgress::into_mesh`、栅栏 `place_fence_piece`、玩家楼梯 `init_magic_supports`(行246)/`animate_brick`(行226)、half_timber `wrap_around`(行1872)、分段采样 `threshold_curve_path_mask`/`add_point_with_mask_to_output`/`arch_function`、`calculate_wall_path_segments`(wall_path_segment 行23)、`trim_rows::impl$0::above`。

---

## 3. 逻辑总清单

抽查 8 函数反编译并与 reconstruction 实现交叉验证，逐条结论：

1. **debug_force_update_arches @0x14003bb80**：size=1，反编译为 `void return;`，PDB 名实为 `finish<country_core::systems::main_camera::MainCameraPlugin>`。A1.8「1B ret 存根」**属实**。语义/是否注册仍未知（合理）。
2. **half_timbering @0x141218e30**：size=27945，`decompile_failed=true`，错误「Unable to find unique hash for varnode」，签名 void() 占位。B9.1 卡点描述**属实**。
3. **find_stone_column_start_y @0x141250720**：反编译「WARNING: Control flow encountered bad instruction data」+ jumptable 截断，switch case 0x12..0x16→halt_baddata。A4.14「jumptable 坏数据截断（case 0x12..0x16）」**逐字属实**。
4. **walk<arches> @0x141232bb0**：`count = max(1, floor(total/step+0.5))`（`total=Σ‖Δ‖`、`step=[RCX+8]`），与 A1.4 公式一致；reconstruction `walk_profile` 同款（step 位 0x3f0f5c29≈0.56 与 A1.3 吻合）。
5. **walk<brackets> @0x141231ea0（3330B）与 walk<arches_n_walls<false>> @0x14048ab50（1999B）**：同为 walk 骨架（首部 total 累加+count 公式与 1.4 逐字同构），仅 closure 不同——确认二者是真发射函数，仅被 A未知#8 交叉指认而未展开（见 §4）。
6. **construct_arches::closure$3 @0x141231af0**：`Affine3Packed::from_mat3_translation`+`push<BrickTransformSsbo>`+计数++dirty，与 A1.6 一致。
7. **create_stone_arch_profile @0x1407645f0**：首判 `(*in_stack_00000028&1)==0` 走 regular 分支、读 style@+600/e0/e8，与 A3.2「regular vs freehand 双分支」一致。
8. **place_support_beams / propose**：reconstruction `propose` 的 `cost=len>=1 ? total*(‖prev_dir-dir‖+0.5) : 0`、`flags&0xf→stopped`、`total>0x40499999(3.15)→None`，与 A3.5/A3.9 公式**完全一致**（含 3.15 上限）。

**逻辑一致性整体判断**：A 部 1.1 描述的 construct_walls 调用链（InstancedWall new/update、WallHoleStorage finalize/new_finalized/clone_and_reopen、WallTriggerVolume::recompute、Roof::is_flat/is_gable/gable_sides、PublicWalls::get/roof_entity、half_timbering/stairs_signifiers_new/add_wall_corners/add_skirts）与骨架依赖注解逐条对应。仅一处小缺口：A1.2 列 construct_walls 后段调用为「half_timbering/add_wall_corners/add_skirts/stairs_signifiers_new」，**漏了 roof_outline**（骨架证明 construct_walls 亦直接调 roof_outline，见依赖注解行 300-303）。B9.2 已单列 roof_outline，非矛盾，补一笔即可。

---

## 4. 冲突与需裁定项

1. **两 walk 单态遗漏（A vs B 跨部分）**：`walk<brackets::closure_env$7>`(0x1231ea0) 与 `walk<arches_n_walls::closure_env$6<false>>`(0x48ab50) 是发射函数，A未知#8 声明「属子范围 B」但 B 部 5.1/5.3 从未展开。**裁定方向**：不改代码，仅在 B5.1/B5.3 各补一条子条目，标注「同 A1.4 行走骨架、closure 不同」，并把 `<true>`/`<false>` 翻转（fn<true> → closure$6<true>@0x48b320 → 其 walk env 为 closure_env$6<false>@0x48ab50）纳入 B5.1 的未知项解释。B5.1「<true> 开关语义未知」可借此收窄。
2. **平台楼梯双路径关系（A2.8 vs B4）**：`split_wall_path_segment_into_platform_stairs`(0x140751960, A2.8, I) 与 `construct_platform_stairs`(0x1407744d0, B4, O) 名称相近、都产平台楼梯，且 reconstruction 已有 `split_platform_wall_masks`(wall_platform_segmentation.rs)。A2.8 未知#4 已把「与 recovered split_platform_wall_masks（mask 级）关系」列为未知。**裁定方向**：保持 A2.8=I 合理；待定点 threshold_range 阈值/闭开后，若确认 0x140751960 与 split_platform_wall_masks 同源，则 A2.8 可升 O。不做代码改动。
3. **init_wall_height @0x14078b550 是否可升 O**：reconstruction/building/wall_height_authority.rs 已含 `assign_resolved_height`/`write_resolved_source_height`，A1.7 现标 I、未知#1 问「与 core wall_height_authority 对齐」。**裁定方向**：建议定点时把 wall_height_authority.rs 列为候选复用对象，确认「两对 {count,height}」即顶/底 authority 后可升 O。
4. **A1.2 漏 roof_outline**（见 §3）——补依赖清单，非冲突。
5. **construct_lintels 标 O/I 混注**（A3.4）：O 指内部链、I 指 ECS shell，语义清晰但与其他条目单一标注不同。**裁定方向**：接受现有标注，建议在 §5 总览统一注明 O/I 混注含义，避免下游误读为「部分实现」。

---

## 5. 遗留 unknown 汇总

已确认三卡点（复核属实）：
1. **half_timbering @0x141218e30**：Ghidra/ReVa 空反编译，控制流未取（size 27945B，须修 void() 签名后定点）。
2. **sine_wave.rs**：BrickRowSine::new(24627)/sine_wave_deform(24641)/random_f(24629) 为 inlinee，宿主未定位（候选 construct_walls/arches 砖行块）。
3. **find_stone_column_start_y @0x141250720**：jumptable 坏数据截断（case 0x12..0x16），须 reva 定点。

其余按 A§7(9 项)+B§12(12 项) 原样保留，复核未发现「把未知当确定」的过度断言——所有存疑处均以「候选/推断/须定点」措辞（例：A1.7「候选 WallHeightAuthority 顶/底」、B9.1「推断为半木梁布局主函数」、B9.15「候选宿主」、A1.8「真实语义/是否注册」）。补充两处可收窄项：
- B5.1 `<true>` 开关 + closure 编号绑定：借助 walk env `<false>` 与 closure$6<true> 的关系可部分解释（§4.1）。
- A2.8 threshold_range 阈值/闭开：与 `split_platform_wall_masks` 对表（§4.2）。

---

## 6. 整体评价与建议

**评价**：本模块分析在发射函数覆盖（100/102 唯一发射过程）、地址准确性（抽查 8 全对）、可复用标注可信度（与 reconstruction 实现逐字互证）、未知清单克制度（无过度断言）四个维度均达标，可作为实现权威基线。A/B 合并版与两份原始分件内容一致（合并版为超集）。

**建议**（不改代码，收尾项）：
1. 补两条 walk 单态子条目 + `<true>/<false>` 翻转说明（§4.1）。
2. 登记 §2 表末 25 个未点名业务内联体，避免后续实现时被当作「无此函数」漏掉（尤其 create_wall、emit_segment_bricks、wooden_lintel_profile、SupportBeamGrower::accept、generate_plaster_mesh、wrap_around、init_magic_supports、animate_brick、门族 door_mesh 等 9 族）。
3. A1.2 依赖清单补 roof_outline；TASKS.md UM-WALL-25（crossbrick 内联宿主）已被 A4.21 定位为 0x142418f50，同步 TASKS 状态。
4. half_timbering / find_stone_column_start_y / sine_wave 三卡点建议作为墙族定点优先项排入后续批次。
