# O 结论抽查复查 · 第二轮（round3 round2，2026-10-15）

> 任务来源：第一轮（seed 42）只抽查 40 条 / 1923 条 O 结论，一致率 80%。本轮按域比例补查
> 225 条（?? 顶层 70 含 10 条非 render 分层补抽；terrain-water 因第一轮未覆盖且 coverage 最低补抽 10 条），
> 固定种子 20261015，排除第一轮 O-sample-42.json 全部已查地址。本轮同时复核并纠正第一轮
> "0x14003bb80 簇身份错位"结论（关键方法发现 #2 二阶纠错）。

## 方法与范围

- **O 行清单**：章节表行（逆向结论列以 O 或 **O** 开头；**O** 行 claim 取作用列）。域归属用
  scripts/chapter-audit.py 的 DOMAINS 映射（与 AUDIT stats 口径一致）。
- **抽样**：seed=20261015，每域均匀不放回抽；?? 域 60 条均匀抽 + 10 条非 render 顶层子域分层补抽
  （components/startup/utils/geometry/顶层——第一轮未覆盖；render/** 占 ?? 池约 66%，均匀抽会淹没它们）；
  terrain-water 补抽 10 条（seed 20261015 续抽，排除 round1+round2 已查地址）。
  清单落盘 `round3/O-sample-round2.json`（215 条）+ `round3/O-sample-round2-terrain-supp.json`（10 条）。
- **验证链**：章节行 → 域 decomp/ 既有反编译（先排除 `{"error":...}` 坏文件）→ `rne-read.py` 原字节/常量/串手解
  → PDB 身份（CU 过程表 + **S_PUB32 public-symbols.json 双查**）→ MCP 补反编译（dbid tg_oa2w1..w9）。
- **分级**：一致 / 不一致（代码写 X 证据为 Y）/ 证据不足。9 个 worker 分片 + 主 shard 交叉验证。

## 本轮关键方法发现（先读）

1. **S_PUB32 是 ICF 折叠区的权威符号表**：CU 过程表（load_procs）每地址只留 1 个代表符号，用它断言
   "该址唯一符号"会系统性误判。实测折叠区符号数：0x140002370=1757、0x14003bb80=437、0x14067dd80=141、
   0x140949170=4、0x14092e6a0=2、0x14092e6c0=2。行内 fn 与 pdb_exact 不同时，必须查 S_PUB32 该地址
   是否含行内 fn 符号；有且机器码吻合 → 一致（ICF 折叠注记）。
2. **第一轮 9 行 0x14003bb80 修正本身被推翻（二阶纠错）**：round1 判"该址唯一符号=Plugin::finish；
   原身份未证实"。S_PUB32 实测该址含 draw_path PathRastersResources / slider3d Slider3d /
   steam SteamOverlayOpenResources 的 SystemParam apply/new_archetype/queue 精确符号 →
   原"SystemParam 胶水"身份成立（机器码 1B ret = 无状态胶水 no-op）。9 行已回改（就地修正清单）。
3. **wp15/wp20 证据目录在 reconstruction/country-core-render/ 下，不在 evidence/**：多处章节 evid
   引用 wp15/analysis/xxx.c、wp20/analysis/xxx.txt 在 evidence 下找不到；实际文件在 reconstruction 子目录。
   本轮 2 条证据不足因此补证为一致（0x14097d280；0x14089d9b0 的行为部分）。
4. **"折叠（NotFound）"滥用**：decomp 落盘为 NotFound 不代表函数不存在——0x14092df10（26B）、
   0x1409a90c0（70B）、0x140aabdc0（5B）等均为真实小函数（Ghidra 未建函数），被章节误标"折叠单行"。
5. **反编译截断伪影**：Ghidra 跳表恢复失败会产生截断文本（0x141444c70 "316B 跳板"实为 10.4KB 函数），
   不能按截断长度断言函数大小；跳表基址必须按 lea disp 化验算。
6. **数量断言（"N callers"）无 xrefs 证据不成立**（延续 round1 rotation 规则）：signifiers 一章
   9 处 callers 数量断言中本轮抽到的 "35 callers"（0x14089d9b0）无 xrefs 支撑，已删除。
## 逐条结果（225 条）

### ??（70 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| components·mod.rs | clone_behavior | 0x140002370 | ✓ 一致 |
| asset_libraries·mesh_library.rs | transient_meshes | 0x140002450 | ✓ 一致 |
| render·render_config.rs | Error> | 0x14003ae40 | ✓ 一致 |
| utils·mod.rs | get_last_event<OsCursorCaptureCmd> | 0x1402b4aa0 | ✓ 一致 |
| render·pass_builder·compute_pass_build | Cobblestone1RasterStagingSsbo> | 0x140588820 | ✓ 一致 |
| components·drawable.rs | register_required_components | 0x14082cad0 | ✓ 一致 |
| components·drawable.rs | register_required_components | 0x14082d1c0 | ✓ 一致 |
| components·drawable.rs | register_required_components | 0x140830250 | ✓ 一致 |
| components·mod.rs | register_required_components | 0x140850aa0 | ✓ 一致 |
| components·mod.rs | register_required_components | 0x140855030 | ✓ 一致 |
| game_key_bindings.rs | assign_mouse_button | 0x14089b280 | ✓ 一致 |
| game_key_bindings.rs | assign_key | 0x14089b2a0 | ✓ 一致 |
| game_key_bindings.rs | keycode_to_str | 0x14089d490 | ✓ 一致 |
| asset_libraries·configwatch.rs | watch | 0x1408ba290 | ✓ 一致 |
| render·pass_builder·draw_stream.rs | to_texture_binding_key | 0x1408eefd0 | ✓ 一致 |
| render·pass_builder·draw_builder.rs | multi_draw_indexed_indirect | 0x140907990 | ✓ 一致 |
| render·pass_builder·draw_builder.rs | draw_indexed_indirect_raw | 0x140907cc0 | ✓ 一致 |
| game_config.rs | upgrade | 0x14090df90 | ✗ 不一致 |
| game_profile.rs | profile_file_path | 0x140916440 | ✓ 一致 |
| game_profile.rs | save_game_profile | 0x140917230 | ✓ 一致 |
| asset_libraries·vao_library.rs | new | 0x140924030 | ✓ 一致 |
| asset_libraries·mesh_library.rs | add | 0x14092cc60 | ✓ 一致 |
| asset_libraries·mesh_library.rs | get | 0x14092d2e0 | ✓ 一致 |
| asset_libraries·shader_library.rs | new | 0x140936c60 | ✓ 一致 |
| asset_libraries·shader_library.rs | debug_raster_get_name | 0x140937cb0 | ✓ 一致 |
| asset_libraries·color_library.rs | update_color_library | 0x14093b530 | ✓ 一致 |
| asset_libraries·texture_library.rs | create_name_mapping | 0x14093c720 | ✓ 一致 |
| asset_libraries·texture_library.rs | handle_to_bindless_index | 0x14093d310 | ✓ 一致 |
| asset_libraries·texture_library.rs | image_name_to_index | 0x14093d690 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | NaniNextVisibilityBitmaskSsbo> | 0x140970b20 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | ShapeStreamSsbo> | 0x140971020 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | DofAutofocusSsbo> | 0x140972220 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | NaniFilteredInstanceAllocatorSsbo> | 0x140973c20 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | GrassBladeDataSsbo> | 0x140973d20 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | AtomicCounterBuffer> | 0x140974020 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | DrawIndirectBuffer> | 0x140974d20 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | Flower03Ssbo> | 0x140975020 | ✓ 一致 |
| render·raster_single.rs | closure_env$0> | 0x1409d00a0 | ✓ 一致 |
| render·mesh.rs | append | 0x140a08440 | ✓ 一致 |
| render·pass_builder·pass_resource_acce | dims | 0x140a59860 | ✓ 一致 |
| game_profile.rs | serialize<ron> | 0x140a658a0 | ✓ 一致 |
| utils·dirty_tiles.rs | iter_world_space_rect_cells<intersects closure | 0x140adec50 | ✓ 一致 |
| render·view_constants.rs | build | 0x140b18f60 | ✓ 一致 |
| startup·load_mesh_atlases.rs | load_mesh_atlases | 0x14129f400 | ✓ 一致 |
| render·nani·gpu_world.rs | SolidVertexColorWindowInstanceData> | 0x14156e880 | ✓ 一致 |
| render·nani·gpu_world.rs | VelocityOnlyInstanceData> | 0x14156f290 | ✓ 一致 |
| render·nani·gpu_world.rs | SolidVertexColorWindowGlassInstanceData> | 0x14156f950 | ✓ 一致 |
| render·nani·gpu_world.rs | SolidAlbedoMapInstanceData> | 0x1415722c0 | ✓ 一致 |
| render·nani·gpu_world.rs | SolidVertexColorBricksInstanceData> | 0x141572340 | ✓ 一致 |
| render·nani·gpu_world.rs | WrenInstanceData> | 0x141576500 | ✓ 一致 |
| render·nani·gpu_world.rs | BalconyDoorInstanceData> | 0x1415780c0 | ✓ 一致 |
| render·nani·gpu_world.rs | SolidVertexColorBricksInstanceData> | 0x141579590 | ✓ 一致 |
| render·nani·gpu_world.rs | PlantInstanceData> | 0x14157aa60 | ✓ 一致 |
| render·pass_builder·compute_pass_build | AsComputeShaderSource> > > | 0x141600570 | ✓ 一致 |
| render·pass_builder·compute_pass_build | AtomicCounterBuffer> | 0x1416056e0 | ✓ 一致 |
| render·pass_builder·compute_pass_build | NaniDrawListLengthSsbo> | 0x141606900 | ✓ 一致 |
| render·pass_builder·compute_pass_build | FireflyVisibilitySsbo> | 0x141607040 | ✓ 一致 |
| render·pass_builder·compute_pass_build | BrickDrawListSsbo> | 0x141607cf0 | ✓ 一致 |
| render·pass_builder·compute_pass_build | MouseTerrainRaycastSsbo> | 0x1416093d0 | ✓ 一致 |
| render·pass_builder·compute_pass_build | BushCpuSsbo> | 0x1416095a0 | ✓ 一致 |
| render·pass_builder·compute_pass_build | closure_env$1> | 0x14160a2e0 | ✓ 一致 |
| render·pass_builder·compute_pass_build | DofAutofocusSsbo> | 0x14160ddc0 | ✓ 一致 |
| render·pass_builder·shader_uri.rs | as_compute_pipeline | 0x1416247a0 | ✓ 一致 |
| render·pass_builder·shader_uri.rs | as_compute_pipeline | 0x141624c40 | ✓ 一致 |
| render·pass_builder·shader_uri.rs | source_hash | 0x1416250c0 | ✓ 一致 |
| render·pass_builder·shader_uri.rs | as_compute_pipeline | 0x1416250f0 | ✓ 一致 |
| render·pass_builder·draw_builder.rs | StairsTransformSsbo> | 0x14162cb90 | ✓ 一致 |
| render·pass_builder·draw_builder.rs | ShadowMaskMeanSsbo> | 0x14162f890 | ✓ 一致 |
| render·pass_builder·draw_builder.rs | GrassBladeDataSsbo> | 0x141630790 | ✓ 一致 |
| render·nani·systems.rs | nani_update_solid_albedo_map_emissive | 0x142072b50 | ✓ 一致 |

- **0x14090df90** `upgrade`（game_config.rs）：代码写 f32 比较 [rcx+0x84] vs 常量 0x14202018e（0.5?），证据为：ucomiss 目标实为 0x142925904（f32=1.0f），第二常量 0x142aab384（f32=0.125f）；0x14202018e 处字节为 69c2d000（f32=0x00d0c269，非 0.5），地址错。其余字节断言成立：test byte[rcx+0x88],1（行内写 cmp byte,1，指令编码为 test&1，语义对 0/1 bool 等价）；je+1; ret 形态吻合（7401c3）；cmp dword[rcx+0x80],0 与 mov dword[rcx+0x80],0 吻合；后续 mov dword[rcx+0x88],1 + movss[rcx+0x8c]=xmm0（回填的是旧值 [0x84] 而非新常量）。修复建议：常量地址改为 0x142925904（1.0f）与 0x142aab384（0.125f），指令记法改 test byte,1，新常量回填改述为 0x8c 复制旧值、0x88 置版本标志。函数总长 80B 与 PDB size 吻合。

### wall（30 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| resources·walls·decorator.rs | events | 0x14067dd80 | ✓ 一致 |
| resources·walls·decorator.rs | is_initial | 0x140833230 | ✓ 一致 |
| resources·walls·decorator.rs | dst_as_wall | 0x140833260 | ✓ 一致 |
| resources·walls·decorator.rs | dst_as_wall_mut | 0x1408332c0 | ✓ 一致 |
| resources·walls·decorator.rs | as_wall_mut | 0x140833540 | ✓ 一致 |
| resources·walls·decorator.rs | from | 0x140839550 | ✓ 一致 |
| resources·walls·decorator.rs | from | 0x140839560 | ✗ 不一致 |
| resources·walls·inner_walls.rs | remove_all | 0x14085a730 | ✓ 一致 |
| systems·wall·merge_walls.rs | merge_walls | 0x140999920 | ✓ 一致 |
| resources·walls·decorator_storage.rs | new | 0x140a1b430 | ✓ 一致 |
| resources·walls·decorator_storage.rs | as_wall | 0x140a1c240 | ✓ 一致 |
| resources·walls·decorator_storage.rs | get_or_insert_default | 0x140a1cf30 | ✓ 一致 |
| resources·walls·decorator_storage.rs | iter_terrain_decorators | 0x140a1e280 | ✓ 一致 |
| resources·walls·material_stats.rs | add_gate_length | 0x140a884f0 | ✓ 一致 |
| systems·wall·calc_wall_bend.rs | calc_wall_bend | 0x140a8a710 | ✓ 一致 |
| systems·wall·wall_state·wall_spatial_h | add_segment | 0x140aa5dd0 | ✓ 一致 |
| systems·wall·wall_state·wall_spatial_h | bounds_ws | 0x140aa6600 | ✓ 一致 |
| systems·wall·adjust_wall_height·ui_adj | history_record_adjust_wall_height | 0x140ac79e0 | ✓ 一致 |
| systems·wall·shapes·rectangle·ui_edit_ | ui_edit_rectangle_dims | 0x140ad6250 | ✓ 一致 |
| systems·wall·shapes·rectangle·ui_edit_ | ui_edit_rectangle_rotate | 0x140ad8000 | ✓ 一致 |
| resources·walls·wall_attachment.rs | is_attached | 0x140b256b0 | ✓ 一致 |
| resources·walls·decorator_subtype.rs | insert_collision_and_interaction_as_same | 0x140b25ef0 | ✓ 一致 |
| resources·walls·public_wall_state.rs | u_space | 0x140b2a770 | ✓ 一致 |
| resources·walls·decorator_archivist.rs | decorator_cull_allow_wall | 0x140b2bc00 | ✓ 一致 |
| resources·walls·decorator_outline.rs | display | 0x140b33b00 | ✓ 一致 |
| resources·walls·mod.rs | split_height_into_skirts | 0x140b34f80 | ✓ 一致 |
| resources·walls·wall_holes.rs | wall_holes | 0x140f86d00 | ✓ 一致 |
| resources·walls·wall_holes.rs | finalize | 0x140f87300 | ✓ 一致 |
| systems·wall·valid_enclosure·mod.rs | get_pos_at_u | 0x1411ff620 | ✓ 一致 |
| resources·walls·wall_path_segment.rs | pt_count | 0x141234970 | ✓ 一致 |

- **0x140839560** `from`（resources/walls/decorator.rs）：代码写 DecoratorExtraInfo From 变体 8（写判别字节 +0x11=8），机器码为：mov rax,rcx; mov ecx,[rdx+0x10]; mov [rax+0x10],ecx; movups xmm0,[rdx]; movups [rax],xmm0; ret（16B，与 PDB size 16 吻合）——只复制 payload {16B@+0, u32@+0x10}，全程没有任何 +0x11 字节写入，与 0x140839550（写 7）形成明确对照。fix 建议：该行作用改为 From<StairDecoratorInfo>：payload 复制 {16B@+0, u32@+0x10}，无判别字节写入（或判别由调用方/内存预置），不应断言写 8。

### collision-eco（15 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| resources·stairs·stairs_state.rs | CompactFormatter> > > | 0x1402e0630 | ✓ 一致 |
| resources·clutter_meshes.rs | register_required_components | 0x1408b9170 | ✓ 一致 |
| systems·collision·world.rs | legacy_raycast | 0x1408cc680 | ✓ 一致 |
| systems·collision·world.rs | rapier_hit_to_raycast_world_result | 0x1408ce4a0 | ✓ 一致 |
| resources·stairs·stairs_state.rs | add_edge | 0x1408ffd70 | ✓ 一致 |
| resources·stairs·stairs_state.rs | remove_node | 0x1408ffef0 | ✓ 一致 |
| systems·ivy·mod.rs | add_ivy_systems | 0x140944070 | ✓ 一致 |
| systems·ivy·ivy_spawner.rs | ivy_spawner | 0x14094cf90 | ✓ 一致 |
| systems·ivy·ivy_leaf_spawner.rs | ivy_leaf_spawner | 0x140996080 | ✓ 一致 |
| resources·ivy·wall_coord.rs | into_world_space | 0x140b199e0 | ✓ 一致 |
| systems·collision·world.rs | to_offset_collider_builder | 0x140b26bd0 | ✓ 一致 |
| resources·critters.rs | register_required_components | 0x140f88c80 | ✓ 一致 |
| systems·collision·world.rs | IsoTransform> | 0x141296510 | ✓ 一致 |
| systems·collision·world.rs | closure_env$1> | 0x14129b780 | ✓ 一致 |
| resources·critters·sheep.rs | StrRead> > > | 0x1413ff8a0 | ✓ 一致 |

### ui-input（15 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| resources·gui.rs | first | 0x140002370 | ✓ 一致 |
| input·input_stream.rs | serialize<Compact> | 0x1402e0060 | ✓ 一致 |
| systems·ui·signifiers·mod.rs | new | 0x14089d9b0 | ✗ 不一致 |
| systems·ui·signifiers·mod.rs | register_required_components | 0x1408a15f0 | ✓ 一致 |
| systems·ui·resources·ui_state.rs | into_drag_state | 0x14092df10 | ✗ 不一致 |
| resources·gui.rs | as_string | 0x140940170 | ✓ 一致 |
| input·input_stream.rs | on_history_sync | 0x1409494c0 | ✓ 一致 |
| systems·ui_move_shapes.rs | move_gizmo_and_trigger_vol | 0x14097d280 | ✓ 一致 |
| systems·ui·resources·widget_params.rs | on_drag_release | 0x1409a90c0 | ✗ 不一致 |
| systems·render_loop·draw_passes·ui_2d_ | mei_pass | 0x1409c6700 | ✓ 一致 |
| resources·toolbar.rs | new | 0x140af8c50 | △ 证据不足 |
| systems·mod_key.rs | update_mod_key_state | 0x140af9c00 | ✓ 一致 |
| systems·mode_manager.rs | closure_env$13> | 0x140afa060 | ✓ 一致 |
| resources·photo_mode.rs | build | 0x140b59640 | ✓ 一致 |
| systems·backend·input·translate_input. | translate_input | 0x1417cd0a0 | ✓ 一致 |

- **0x14089d9b0** `new`（systems/ui/signifiers/mod.rs）：身份吻合（PDB exact = SignifierEntityBuilder::new，size 274；S_PUB32 off 0x89c9b0 同址）。但：① 唯一引用证据 wp15/analysis/b6_widgets/new_0x14089d9b0.c 不存在——wp15 目录整体缺失（ls/find 均无结果）；② claim『35 callers』数量断言无任何 xrefs 证据支撑（与第一轮 rotation 行同模式）；③ 无 decomp/机器码证据。fix：删除『35 callers』断言或补 xrefs 计数证据；补该址反编译后按机器码核行为。

- **0x14092df10** `into_drag_state`（systems/ui/resources/ui_state.rs）：身份正确（PDB exact = slider3d::impl$1::into_drag_state，size 26 == 机器码 26B；S_PUB32 off 0x92cf10 同址），但行内『折叠单行（NotFound）』断言被证伪：该址是真实 26B 函数，非折叠占位——机器码：mov rdx,[rcx]; mov al,2; cmp byte[rdx+0x18],2; jb ret; mov r8,[rdx+0x38]; cmp r8,[rcx+8]; jne ret; movzx eax,byte[rdx+0x19]; ret（默认返回 2，[rdx+0x18]>=2 且 [rdx+0x38]==[rcx+8] 时返回 byte@0x19）。decomp NotFound 为 DB 工件被当证据；『语义=DragState::is_started』未证实（判别值映射需 DragState 枚举证据）。fix：删除『折叠（NotFound）』表述，按上述机器码重写行为描述，语义待判别值证据。

- **0x1409a90c0** `on_drag_release`（systems/ui/resources/widget_params.rs）：身份正确（PDB exact = UiSounds::on_drag_release，size 70 == 机器码 70B；S_PUB32 off 0x9a80c0 同址），但『折叠（NotFound）』断言被证伪：真实 70B 函数——机器码为 UiSounds 结构复制/重排：槽 0..0x50 复制，[rax+0x30]=[rdx+0x0]（重复）、[rax+0x40]=[r8+0x10]（替换）、0x48/0x50 直拷。『槽 6-8』与复制触及 0x30..0x48 部分吻合，但需按真实复制语义重写；decomp NotFound 为 DB 工件被当证据。fix：删除『折叠』表述，按机器码描述结构复制语义。

- **0x140af8c50** `new`（resources/toolbar.rs）：身份吻合（PDB exact = Toolbar::new，size 3058；S_PUB32 off 0xaf7c50 同址）。但 claim 自带『待核 I』注记（O(部分)）：行为断言（alloc 0x30x8B 元素表、与 0x140af9000 的 11 类 0x10B 表为不同单态/内联）无任何 decomp/机器码证据；且 0x140af9000 位于 0x140af8c50+0x1b0（同一函数体内），『不同单态』说法可疑，两地址关系需反汇编裁决。O(部分) 不应作完整 O。fix：补该址反汇编后核，未核前降级为非 O。

### terrain-water（25 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| systems·world_raster.rs | apply | 0x14003bb80 | ✓ 一致 |
| resources·terrain_editing.rs | CompactFormatter> > > | 0x1400e7480 | ✓ 一致 |
| systems·render_loop·compute_passes·upl | …upload_raster_staging_to_texture<PathRaster> | 0x1403d8bd0 | ✓ 一致 |
| resources·world_raster.rs | would_write_succeed | 0x14083f410 | ✓ 一致 |
| resources·history·terrain_edit.rs | reapply | 0x1408592d0 | ✓ 一致 |
| resources·active_raster_stroke.rs | resample_and_smooth | 0x140868e40 | ✓ 一致 |
| resources·active_raster_stroke.rs | solidify_fluid_into | 0x1408691b0 | ✓ 一致 |
| resources·terrain_editing.rs | PrettyFormatter> | 0x1408e5bc0 | ✓ 一致 |
| resources·terrain_editing.rs | new_stroke | 0x140924a10 | ✓ 一致 |
| startup·startup_terrain.rs | grid_gen_ccw | 0x140928350 | ✓ 一致 |
| systems·render_loop·compute_passes·mou | mouse_read_terrain_stroke_index | 0x1409768b0 | ✓ 一致 |
| systems·render_loop·compute_passes·rea | read_terrain_data | 0x14098c680 | ✓ 一致 |
| resources·terrain_height.rs | TerrainHeightSsbo> | 0x1409e9640 | ✗ 不一致 |
| systems·audio·water_ambience_audio.rs | water_ambience_audio_system | 0x1409ecea0 | ✓ 一致 |
| resources·brush_sizes.rs | set_path | 0x1409f1870 | ✓ 一致 |
| resources·brush_sizes.rs | reset_water | 0x1409f1890 | ✓ 一致 |
| resources·world_raster.rs | commit_scratch_to_composite<512> | 0x140a16580 | ✓ 一致 |
| resources·world_raster.rs | closure_env$0> > | 0x140a16960 | ✓ 一致 |
| systems·audio·garden_audio.rs | garden_audio_system | 0x140a89480 | ✓ 一致 |
| resources·terrain_stroke.rs | insert_or_morph_last_point | 0x140a97880 | ✓ 一致 |
| systems·terrain·delayed_terrain_change | on_terrain_modified | 0x140aabdc0 | ✓ 一致 |
| systems·world_raster.rs | Cobblestone3Raster> > | 0x140ab6de0 | ✓ 一致 |
| systems·world_raster.rs | closure_env$2> | 0x140aba770 | ✓ 一致 |
| systems·terrain·deferred_terrain_histo | history_record_terrain | 0x140ac9770 | ✓ 一致 |
| systems·terrain·ui_edit_terrain.rs | ui_edit_terrain | 0x140aef1f0 | ✓ 一致 |

- **0x1409e9640** `TerrainHeightSsbo>`（resources/terrain_height.rs）：行为主体成立但 debug 门方向反（claim 与机器码相反）：代码为 if(*0x14362c308 == 0) { x=u*512 截断 clamp[0,511]；index=(y<<9|x)+0x201 } else { idx=(int)(u*512-0.5)+1 clamp[0,511]；index=(y<<9|x) 无 0x201 }——即 +0x201 偏移在 debug 全局字节==0 时走，而 claim 写 "≠0 走 +0x201 偏移视觉网格变体"。其余逐项吻合：(x/130+0.5)（130.0f@0x1429a3c70、0.5f@0x1429258e0）、×512（0x1429a3c80/84/b0）、-0.5+1（0x14295a0e8=-0.5）、clamp[0,511]、u16 读取（*2）。fix 建议：该行改为 "debug 全局字节 *0x14362c308==0 时走 +0x201 分支，≠0 时走无偏移的 u*512-0.5+1 分支"。

### history（15 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| resources·history·path_edit.rs | CompactFormatter> > > | 0x1404896f0 | ✓ 一致 |
| resources·history·replay_cmd_buffer.rs | len | 0x140949170 | ✓ 一致 |
| systems·history·replay_wall.rs | replay_wall | 0x140951b80 | ✓ 一致 |
| systems·history·replay_wall.rs | _deprecated_morph_curve | 0x140952570 | ✓ 一致 |
| systems·history·replay_mode.rs | replay_mode | 0x140a7c550 | ✓ 一致 |
| resources·history·mod.rs | insert_starting_edit | 0x140a844f0 | ✓ 一致 |
| resources·history·mod.rs | iter_resolved_snapshots | 0x140a84a40 | ✓ 一致 |
| systems·history·history_process.rs | interrupt | 0x140af5b20 | ✓ 一致 |
| systems·history·history_process.rs | replay_edit | 0x140af6170 | ✗ 不一致 |
| systems·history·replay_stairs.rs | replay_stairs | 0x140af75b0 | ✓ 一致 |
| resources·history·mod.rs | iter | 0x140b31cf0 | ✓ 一致 |
| systems·history·replay_pop_bucket.rs | replay_pop_bucket | 0x140b38250 | ✓ 一致 |
| systems·history·history_process.rs | closure_env$0> | 0x141444c70 | ✗ 不一致 |
| systems·history·replay_path.rs | generate_path_replay_cmds | 0x1416286d0 | ✗ 不一致 |
| resources·history·tree_edit.rs | Serializer> | 0x1417c2770 | ✓ 一致 |

- **0x140af6170** `replay_edit`（systems/history/history_process.rs）：身份成立（PDB exact=replay_edit，5173B，本址自有 decomp-0x140af6170.txt 19544B 真实存在），但行内有两处错：(a) evid 错引 decomp-0x140af5b30.txt——那是相邻函数 history_process(0x140af5b30) 的反编译，非本址证据，正确应为 decomp-0x140af6170.txt+DIGEST §3.4；(b) 作用列为 history_process 描述的复制粘贴（undo/redo 游标+第二事件流+log 非本函数行为）；本函数实际为 MultiEdit→ReplayCmd 桶展开（decomp 调用 FUN_14143fb50=generate_replay_cmds_from_edit，24B edit 步长，DIGEST §3.4）。fix：作用按 §3.4 改写，evid 改 decomp-0x140af6170.txt。

- **0x141444c70** `closure_env$0>`（systems/history/history_process.rs）：身份 PDB 吻合（generate_replay_cmds_from_edit<water closure_env$0>，size 10371），分派行为方向正确（按 *param_1 索引跳表间接跳转，各分支含浮点常量 0.1f/0.2f、标志字节、Arc 计数、串引用=各领域命令生成），但两处细节与机器码不符：(a) 316B 跳板错误——316B 只是 Ghidra 跳表恢复失败的截断 decomp 文本；实际函数约 10.4KB（机器码连续运行至 PDB end 0x141446cf3 附近才 ret+nop 填充）。(b) 跳表基址错——本单态实算为 0x142c5f2fc（lea rdx,[rip+0x181A659] 于 0x141444c9c），非行内/文件作用声称的 0x142c5f248（该基址属 0x14143d2c0 单态；四单态各有独立跳表 0x142c5f248/0x284/0x2c0/0x2fc 间距 60B，非 ICF 合并）。fix：作用改写为约 10.4KB 大型按 tag 分派生成核心（本单态跳表 0x142c5f2fc），DIGEST §3.5 同步修正。

- **0x1416286d0** `generate_path_replay_cmds`（systems/history/replay_path.rs）：身份 PDB 吻合（generate_path_replay_cmds，26B，机器码恰 26B）；niche 过滤形态正确（mov rax,[rcx]; dec; cmp 4; jae 放行 1..=4，0=niche，0..3 变体可作等价描述），但尾调目标地址错：e9 rel32（-0x1E6309）程序化验证目标=0x1414423e0，非行内/文件作用声称的 0x1414433xx；0x1414423e0 恰为本 chunk 第 13 行 generate_replay_cmds_from_edit<generate_path_replay_cmds::closure_env$0> 单态（其自身跳表基址 0x142c5f2c0）。注：evid decomp 为 NotFound（tg_a1），PDB+机器码裁决。fix：尾调目标改 0x1414423e0。

### render-loop（15 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| systems·render_loop·compute_passes·gra | is_exclusive<…FunctionSystem…> | 0x140002370 | ✓ 一致 |
| systems·render_loop·vermeer.rs | startup_vermeer | 0x1408aa720 | ✓ 一致 |
| systems·render_loop·menu_background.rs | render_menu_bg | 0x1408c9a30 | ✓ 一致 |
| systems·render_loop·draw_passes·final_ | resolve_to_swapchain | 0x14090d850 | ✓ 一致 |
| systems·render_loop·compute_passes·pre | inclusive_prefix_scan_u32_1m | 0x14098bf60 | ✓ 一致 |
| systems·render_loop·draw_passes·transp | post_taa_transparency_pass | 0x1409c40b0 | ✓ 一致 |
| systems·render_loop·draw_passes·wirefr | wireframe_pass | 0x1409ccb20 | ✓ 一致 |
| systems·render_loop·draw_passes·object | object_grass_push_pass | 0x1409daaa0 | ✓ 一致 |
| systems·render_loop·compute_passes·rep | calculate_reprojection_map | 0x140a0d190 | ✓ 一致 |
| systems·render_loop·compute_passes·ras | rasterize_buildings | 0x140a32770 | ✓ 一致 |
| systems·render_loop·compute_passes·app | apply_screen_shadows | 0x140a39e70 | ✓ 一致 |
| systems·render_loop·compute_passes·taa | calculate_tgsr | 0x140a3ba60 | ✓ 一致 |
| systems·render_loop·compute_passes·dof | calc_dof | 0x140a5a570 | ✓ 一致 |
| systems·render_loop·compute_passes·gra | register_required_components | 0x140a66970 | ✓ 一致 |
| systems·render_loop·draw_passes·draw_s | leaf_particle_draw | 0x140a8a090 | ✓ 一致 |

### session-save（10 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| systems·prefab.rs | update_prefabs | 0x140841d60 | ✓ 一致 |
| systems·prefab.rs | register_required_components | 0x1408446b0 | ✓ 一致 |
| resources·session.rs | is_flowery | 0x14092e6a0 | ✓ 一致 |
| resources·session.rs | is_autumn | 0x14092e6c0 | ✓ 一致 |
| systems·save·save_game.rs | add_history_edit_to_save_file | 0x140a439b0 | ✓ 一致 |
| systems·save·load_completion_checker.r | deny_load | 0x140a9fa70 | ✓ 一致 |
| resources·session.rs | save_folder_path | 0x140b46000 | ✓ 一致 |
| resources·prefab.rs | PrefabName> > | 0x141552d70 | ✓ 一致 |
| systems·save·save_thread.rs | flush_deferred_and_sync | 0x14171f7f0 | ✗ 不一致 |
| systems·save·save_thread.rs | snapshot_gc_thread | 0x14172b280 | ✓ 一致 |

- **0x14171f7f0** `flush_deferred_and_sync`（systems/save/save_thread.rs）：身份 PDB 吻合（CurrentSaveFile::flush_deferred_and_sync，1934B；decomp 4213B 真实，含 TLS 守卫+断言串 truncate<=history_edit_offsets.len()+vtable 间接调用=落盘结构），但错误日志 Error flushing save file on game thread disconnect. 不属本函数：该串（VA 0x142ccaa28，rne-read 确认在二进制 .rdata）的指针槽 0x142ccaa60 仅被 save_thread 主循环 0x141720680 引用（0x1417212c0 处 48 8b rip-rel load，全 .text 唯一一处）；本函数 decomp 无该串引用（其串为 UNK_142cca30f/311/478/4ab 断言族 + thread local + puffin）。fix：错误日志串归属注记为 save_thread(0x141720680) 调用侧（游戏线程断开时落盘结构成立，仅串归属错）。

### roofs-rt（10 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| resources·render·mod.rs | from_cols | 0x14089d500 | ✓ 一致 |
| resources·render·mod.rs | add_y | 0x14089d550 | ✓ 一致 |
| resources·roofs·roof_ridge.rs | inverse_ridge_01_param | 0x1408e30f0 | ✓ 一致 |
| resources·roofs·roof_shape.rs | roof_uv_at_ws_pos | 0x1408e4040 | ✓ 一致 |
| resources·roofs·roof_trigger_volume.rs | set_triangles | 0x1408e45b0 | ✓ 一致 |
| resources·roofs·roof_trigger_volume.rs | set_normals | 0x1408e45f0 | ✓ 一致 |
| resources·mod.rs | insert_resources | 0x140940d80 | ✓ 一致 |
| resources·tree_storage.rs | get | 0x140a38460 | ✓ 一致 |
| resources·tree_storage.rs | remove | 0x140a385a0 | ✓ 一致 |
| resources·render·mesh_atlas_library.rs | from | 0x140ae6570 | ✓ 一致 |

### camera-time（10 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| systems·camera_rig·butterfly.rs | constrain_to_be_above_terrain | 0x14003bb80 | ✓ 一致 |
| systems·camera_rig·free.rs | constrain_to_be_above_terrain | 0x14003bb80 | ✓ 一致 |
| resources·time_of_day.rs | Serializer> | 0x1400802c0 | ✓ 一致 |
| systems·despawn.rs | despawn_glade_entities | 0x1408b3510 | ✓ 一致 |
| systems·main_camera.rs | main_camera_update | 0x140912c20 | ✓ 一致 |
| systems·camera_rig·normal.rs | translate_screen_space | 0x140949e70 | ✓ 一致 |
| systems·camera_rig·walk.rs | rotate_yaw_pitch | 0x140993750 | ✓ 一致 |
| systems·camera_rig·free.rs | update`（free/walk 共享） | 0x140993d30 | ✓ 一致 |
| systems·camera_rig·butterfly.rs | update | 0x140a0d090 | ✓ 一致 |
| systems·time_of_day.rs | time_of_day | 0x140a73460 | ✓ 一致 |

### audio-misc（5 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| startup·debug.rs | debug | 0x1408efd40 | ✓ 一致 |
| utils·debug_viz.rs | radius | 0x140b024e0 | ✓ 一致 |
| utils·debug_viz.rs | dashed_line | 0x140b02ea0 | ✓ 一致 |
| utils·debug_viz.rs | show | 0x140b04640 | ✓ 一致 |
| name_hash.rs | eq | 0x140c9cd70 | ✓ 一致 |

### backend-misc（5 条）

| 文件 | 函数 | VA | 判定 |
|---|---|---|---|
| systems·brush_preview.rs | brush_preview | 0x140865050 | ✓ 一致 |
| systems·backend·flush_screen_log.rs | log_warning_to_screen | 0x1408c1310 | ✓ 一致 |
| systems·mod.rs | is_gameplay_active | 0x140a90e00 | ✓ 一致 |
| systems·mod.rs | dyn_hash | 0x140a90e80 | ✓ 一致 |
| systems·backend·texture_watch.rs | texture_watch | 0x140aab370 | ✓ 一致 |


## 统计

| 域 | 抽查 | 一致 | 不一致 | 证据不足 |
|---|---|---|---|---|
| ?? | 70 | 69 | 1 | 0 |
| wall | 30 | 29 | 1 | 0 |
| collision-eco | 15 | 15 | 0 | 0 |
| ui-input | 15 | 11 | 3 | 1 |
| terrain-water | 25 | 24 | 1 | 0 |
| history | 15 | 12 | 3 | 0 |
| render-loop | 15 | 15 | 0 | 0 |
| session-save | 10 | 9 | 1 | 0 |
| roofs-rt | 10 | 10 | 0 | 0 |
| camera-time | 10 | 10 | 0 | 0 |
| audio-misc | 5 | 5 | 0 | 0 |
| backend-misc | 5 | 5 | 0 | 0 |
| **合计** | **225** | **214** | **10** | **1** |

一致率 **95.1%**（214/225），较第一轮 80% 显著提高；仍存
10 条不一致与 1 条证据不足（其中 0x140af8c50 为 O(部分) 主动降级）。


## 错误模式归类（10 条不一致 + 1 条证据不足，另 1 条补证为一致）

1. **常量/地址错**（0x14090df90 GameConfig::upgrade）：f32 比较常量地址错——claim 写 0x14202018e（0.5?），
   objdump 证实为 1.0@0x142925904 与 0.125@0x142aab384；首指令为 `test byte[+0x88],1`（非 cmp）。
2. **行为断言被机器码推翻**（0x140839560 impl$97::from）：claim"写判别字节 +0x11=8"，机器码 16B 仅拷贝
   payload {16B@+0, u32@+0x10} 无判别写入（对照 impl$96 0x140839550 确有 `c6 41 11 07`）。
3. **"折叠（NotFound）"证伪**（0x14092df10 into_drag_state、0x1409a90c0 on_drag_release）：decomp NotFound
   被当"折叠"证据；实为 Ghidra 未建函数的真实小函数（26B/70B），机器码为条件函数/结构复制。
4. **数量断言无证据**（0x14089d9b0 "35 callers"）：延续 round1 rotation 模式；行为（默认值序列）经
   wp15 反编译证实，数量断言删除。
5. **evid 错引相邻函数**（0x140af6170 replay_edit）：evid 引用 decomp-0x140af5b30.txt（相邻函数），
   本址自有 decomp 存在；作用列是复制粘贴的描述。
6. **截断反编译误当函数大小**（0x141444c70）："316B 跳板"实为 Ghidra 跳表恢复失败截断，函数约 10.4KB；
   跳表基址 0x142c5f2fc（非 0x142c5f248，该址属另一单态）。
7. **尾调目标错**（0x1416286d0）：行内 0x1414433xx，e9 rel32 化验算=0x1414423e0。
8. **串归属错**（0x14171f7f0 flush_deferred_and_sync）：错误日志串的 xref 在 save_thread 主循环
   0x141720680，不在本函数。
9. **方向/条件反**（0x1409e9640 sample_at_world_xz）：debug 门 `*0x14362c308 == 0` 才走 +0x201 分支，
   claim 写"≠0"。
10. **O(部分) 不当完整 O**（0x140af8c50 Toolbar::new）：行为断言未核，0x140af9000 实为函数体内 +0x1b0
    （非独立函数）；行内自带"待核 I"注记，本轮维持证据不足/降级。
11. **证据路径漂移**（0x14097d280）：wp20 证据在 reconstruction/country-core-render/ 下，worker 在
    evidence 下找不到标证据不足；补证后一致。

## 就地修正清单（round3 round2 复查修正，合计 ~80 行）

| 文件 | 行/条目 | 修正 |
|---|---|---|
| systems/paths/draw_path.rs.md | L12/15/16（3 行） | 0x14003bb80 回改：S_PUB32 含 PathRastersResources SystemParam apply/new_archetype/queue → 身份成立（round1 "唯一符号"结论作废） |
| systems/ui/resources/slider3d.rs.md | L11/14/15（3 行） | 同上（Slider3d SystemParam） |
| systems/steam/mod.rs.md | L19/21/22（3 行） | 同上（SteamOverlayOpenResources SystemParam） |
| game_config.rs.md | L12（1 行） | 常量地址 0x14202018e→1.0@0x142925904/0.125@0x142aab384；test byte |
| resources/walls/decorator.rs.md | L47（1 行） | impl$97::from 判别写入证伪 → 16B payload 拷贝 |
| resources/walls/inner_walls.rs.md | L18（1 行） | max_id 误标 → remove_all |
| resources/walls/decorator_archivist.rs.md | L26（1 行） | cull_allow_any 误标 → cull_allow_wall |
| systems/ui/resources/ui_state.rs.md | L27/28（2 行） | "折叠（NotFound）"证伪 → 26B 条件函数；语义未证实注记 |
| systems/ui/resources/widget_params.rs.md | L25（1 行） | "折叠"证伪 → 70B 结构复制 |
| systems/ui_move_shapes.rs.md | L12（1 行） | 证据路径补证（reconstruction/wp20） |
| resources/toolbar.rs.md | L11（1 行） | 0x140af9000=体内+0x1b0；O(部分) 降级注记 |
| systems/ui/signifiers/mod.rs.md | L18（1 行） | "35 callers" 删除（无 xrefs）；行为经 wp15 证实 |
| systems/history/history_process.rs.md | L11-14+L17（5 行） | 316B 跳板→10.4KB；跳表基址按单态；replay_edit evid 纠错 |
| systems/history/replay_path.rs.md | L3+L11（2 行） | 尾调目标 0x1414433xx→0x1414423e0 |
| systems/save/save_thread.rs.md | L14（1 行） | 错误日志串归属改主循环 0x141720680 |
| resources/history/mod.rs.md | L109（1 行） | filter_map 谓词不内联进 16B 迭代器构造 |
| render/pass_builder/draw_builder.rs.md | 43 行 evid | 0x14162c290（函数址）→ "WallColorIds" 类型名串段 @0x14293ff6b |
| render/pass_builder/shader_uri.rs.md | 4 行 guess | source_hash 元 &i32→&self |
| render/mesh.rs.md | L50（1 行） | append 尺寸 170B→570B（PDB） |
| render/render_config.rs.md | L13（1 行） | "字符串枚举"→"字段名匹配"（DisplayConfig） |
| render/raster_single.rs.md | L12（1 行） | 补 MCP 反编译证据（oa2-out/w2-decomp-1409d00a0.txt） |
| resources/terrain_height.rs.md | L11（1 行） | debug 门方向 ≠0→==0 |

## 佐证文件

- `round3/O-sample-round2.json`（215 条抽样清单）、`round3/O-sample-round2-terrain-supp.json`（10 条）
- `round3/oa2-pdb-identity.json`（225 址 PDB 身份：CU 精确符号/owner/S_PUB32）
- `round3/oa2-merged-verdicts.json`（逐条判定合并）
- `round3/oa2-chunks/*.json`、`round3/oa2-results/w1..w9.json`（9 worker 验证明细）
- `round3/oa2-out/`（MCP 补反编译）、`round3/oa2-decomp-index.json`（decomp 存在性索引）
- `round3/dis_game_config.py`、`tools/country-core-audit/dis.py`（objdump 反汇编复核）
