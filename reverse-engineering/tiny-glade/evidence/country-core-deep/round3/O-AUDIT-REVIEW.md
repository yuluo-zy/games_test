# O 结论抽查复查（round3，2026-10-15）

任务来源：MODULE_GAP_ANALYSIS §4 —— 新章节 1413 条 O 未做过同类抽查；旧审计（round2 样点）仅 56% 正确率。
本轮对错误率最高的域（collision-eco 80%、ui-input 79%、audio-misc 59%）+ 渲染顶层 render/** 做固定种子抽查。

## 方法与范围

- **O 行清单来源**：`review-audit/extract_claims.py`（collision-eco/ui-input/audio-misc，与既有 claims-*.tsv 同一逻辑）+ round3 新增 render 变体
  （`chapters/render/**`、`chapters/systems/render_loop/**`、`chapters/asset_libraries/**`，650 行）。
  注意：AUDIT JSON 的 stats O 数（collision-eco 219 / ui-input 178 / audio-misc 27）与本清单数（171/157/164）不一致，
  因 AUDIT 采用不同章节集与胶水过滤口径；本抽查直接面向章节表行，以章节为准。
- **抽样**：seed=42，按地址排序后随机抽，每域 10 条，共 40 条。清单落盘 `round3/O-sample-42.json`。
- **验证链**：章节行 → `<域>/decomp/decomp-<va>.txt`（或 round2/render-loop/round3 MCP 反编译）→
  `rne-read.py` 原字节/常量/字符串 → 必要时 `tools/country-core-audit/cca.load_procs` PDB 过程表、`dis.py` 反汇编、
  MCP（tg_rev1）补反编译 / get_xrefs_to。
- **分级**：一致 / 不一致（写清代码写 X 证据为 Y）/ 证据不足。

## 逐条结果（40 条）

### collision-eco（10 条）

| # | 章节文件 | 函数 | VA | 判定 | 证据说明 |
|---|---|---|---|---|---|
| 1 | chapters/systems/paths/draw_path.rs.md | `_::impl$0::apply` | 0x14003bb80 | **不一致（地址错位）** | 代码写"draw_path SystemParam 胶水"；证据为 NotFound 错误文件。机器码=1B `ret` no-op；PDB 精确符号=`bevy_app::plugin::Plugin::finish<MainCameraPlugin>`（唯一）。已修正 |
| 2 | chapters/resources/stairs/structural_graph.rs.md | `add_edge_by_collider` | 0x14086a260 | 一致 | 反编译=petgraph::GraphMap::add_edge(0x14093add0)+IndexMap::get_mut(0x1416284b0, key 三元组排序交换)+entry 写 0；未命中 panic 串 "assertion failed: self.graph.remove_node(collider…)"@0x142a96be8 |
| 3 | chapters/resources/clutter_meshes.rs.md | `ClutterMeshTy::from` | 0x1408b83d0 | 一致（举例名修正） | 68KB 反编译串表确认按路径匹配（clutter/anvil、door_bell、door_krans、stall_bread、rack_brooms、flowerbox、birdnest、firewood、cart_w_pumpkins 等）+ "internal error: entered unreachable code"。但原举例 shovel/bucket/fallen_leaves/bench 未在串表（pumpkin2 亦无，仅 crates_w_pumpkins_regular）。已修正举例清单 |
| 4 | chapters/systems/ivy/ivy_send_to_gpu_leaf.rs.md | `ivy_send_to_gpu_leaf` | 0x1408c1580 | 一致 | callee=IvyLeafSsbo/IvyFlowerSsbo from_iter + FrozenMap::get<SsboId> + rhapsody GraphicsDevice::update_buffer_bytes = 叶子/花 SSBO 上传 |
| 5 | chapters/systems/collision/world.rs.md | `legacy_raycast_w_params` | 0x1408cc770 | 一致 | 反编译=调 Rapier::raycast(0x1408cdc60) 拷贝结果；判别==2 时写 kind=2。"同上带参数"成立 |
| 6 | chapters/systems/collision/world.rs.md | `RaycastGeoSource::subworld` | 0x1408cea20 | 一致 | 机器码 `movzx eax,byte[rcx]; lea rcx,[0x142aa4415]; movzx eax,byte[rax+rcx]; ret` = disc→槽表查表；常量地址精确吻合。注：decomp 文件实为 NotFound，结论来自手解，证据列引用错误文件（已可机器码证实） |
| 7 | chapters/systems/collision/world.rs.md | `RaycastMeshData::new_indexed` | 0x1408cee40 | 一致（注记） | callee=utils::calculate_hash×3（顶点/索引去重哈希）+ Vec<Vec3>::from_iter(normals_without_indices 闭包)+多 Vec 构造；"parry trimesh" 部分无 parry 符号 callee（推断/数据成员），核心"去重哈希"机器证实 |
| 8 | chapters/systems/collision/world.rs.md | `RaycastWorld::on_level_reload` | 0x1408d8710 | 一致 | callee=rapier RigidBodySet/Collider Arena/QueryPipeline/IslandManager/RapierSubWorld/MeshInstanceSources/RaycastMesh slotmap 全 drop_in_place + RapierSubWorld default + Arena::with_capacity = 清空重建 |
| 9 | chapters/systems/paths/path_brush_preview_size.rs.md | `impl$0::register_required_components` | 0x140b1dd70 | 一致（胶水注记） | 身份 PDB 精确；callee=bevy_ecs enforce_no_required_components_recursion + QueuedRegistration::register + HashMap<TypeId,ComponentId>::insert = Bevy 注册胶水，按项目规则不计业务（O 仅证实身份） |
| 10 | chapters/systems/collision/visualize_colliders.rs.md | `visualize_colliders` | 0x141326370 | 一致（注记） | 身份精确；callee=visualize_shape(0x141326930)+puffin scope+str rfind；"线框"为推断表述 |

### ui-input（10 条）

| # | 章节文件 | 函数 | VA | 判定 | 证据说明 |
|---|---|---|---|---|---|
| 11 | chapters/systems/ui/resources/slider3d.rs.md | `_::impl$0::apply` | 0x14003bb80 | **不一致（地址错位）** | 同 #1：实为 Plugin::finish<MainCameraPlugin> 1B ret。已修正 |
| 12 | chapters/systems/ui/resources/slider3d.rs.md | `_::impl$0::new_archetype` | 0x14003bb80 | **不一致（地址错位）** | 同上。已修正 |
| 13 | chapters/systems/ui/resources/slider3d.rs.md | `_::impl$0::queue` | 0x14003bb80 | **不一致（地址错位）** | 同上。已修正 |
| 14 | chapters/systems/ui/signifiers/mod.rs.md | `SignifierEntityBuilder::rotation` | 0x14089dc30 | **不一致（局部）** | 核心行为一致（wp15 同址分析+机器码：写 16B f32 + memcpy 0x90）；但"13 callers"未证实——round3 get_xrefs_to=仅 1 条 DATA 引用、0 call。已修正 |
| 15 | chapters/systems/ui/resources/ui_state.rs.md | `ActiveWidget::is_dragging` | 0x14092e6a0 | **不一致（结论过时）** | 行写"折叠单行（NotFound）"；机器码 `cmp byte[rcx],2; sete al; ret` = 判别==2，与函数名/PDB 精确一致，可证实。已修正 |
| 16 | chapters/resources/gui.rs.md | `PrevEscMenuState::get` | 0x14093fbb0 | 一致 | 机器码 `xor eax,eax; cmp dword[rcx],0xb; cmovne rax,rcx; ret` = 状态==0xb→None 否则 Some(&self)；0xb 哨兵成立 |
| 17 | chapters/systems/mode_manager.rs.md | `mode_manager` | 0x140a86b60 | 一致 | 反编译：hash('esc_menu_canvas'@0x142ad8501,len 0xf) 过滤（+0x28!=2 时 return）+ check_if_can_switch×2(0x140afa8a0/0x140afadb0) + 跳表 0x142ad890c 按 +0x80 索引（"WARNING: jumptable"） |
| 18 | chapters/systems/mode_manager.rs.md | `check_if_can_switch<env$7>` | 0x140afadb0 | 一致 | 反编译：`char[+0x18]==2` 分支→循环取模(+1 % len)；否则越界 panic_bounds_check；选定后写回 byte[+0x18]=2 并输出条目 |
| 19 | chapters/systems/backend/input/calculate_input_delta.rs.md | `calculate_input_delta` | 0x1416feeb0 | 一致（注记） | wp20 同址反编译：kind2 → value+0.1、≤0.05 夹取 0.05、delta=value-prev；kind1 → value+prev；kind0 → 输出 prev 值（delta 0/状态 3）；三通道 +0x30/0x31/0x32。注："参数 @+0xa0 共 10 qword"未逐项核实（拷贝区为 +0x58..+0x80 6 qword） |
| 20 | chapters/systems/ui/resources/widget_params.rs.md | `WidgetParams::signifier<closure$2>` | 0x1422bcc20 | 一致 | 身份精确；callee=SignifierFactory::new(0x14089d8b0)；"同上（closure$2）"成立 |

### audio-misc（10 条）

| # | 章节文件 | 函数 | VA | 判定 | 证据说明 |
|---|---|---|---|---|---|
| 21 | chapters/systems/steam/mod.rs.md | `steam::_::impl$0::apply` | 0x14003bb80 | **不一致（地址错位）** | 同 #1：实为 Plugin::finish<MainCameraPlugin> 1B ret。已修正 |
| 22 | chapters/resources/feedback_report_resources.rs.md | `feedback_report_texture_receive` | 0x140909ab0 | 一致 | 反编译：事件接收/拷贝 + Arc drop（LOCK/dec/UNLOCK+drop_slow）+ 状态字节@+0x3e 的 `-7;<2` 判定（=7/8 转换）+ `==8` 分支 + parking_lot Mutex |
| 23 | chapters/frozen_map.rs.md | `FrozenMap::insert` | 0x140969bf0 | **不一致（行为）** | 代码写"键已存在→不替换，返回已有值引用"；MCP 反编译（tg_rev1）：键已存在→**丢弃新值 Box**（vtable drop+free）返回 1、表不变；键缺失→写入 key+value、len+1 返回 0。"不替换"部分一致，"返回已有值引用"与机器码不符（返回仅标量 1；xrefs 仅 1 DATA）。已修正 |
| 24 | chapters/systems/audio/path_audio.rs.md | `path_audio_init_system` | 0x1409d08d0 | 一致 | callee=AudioCore::instantiate_loop + Commands::insert_resource<PathAudioState>；串表=event:/SFX/CobblestoneDraw、Path Draw@0x142ac1660、Erase Draw@0x142ac1690、Water/PathWaterDraw、Water/PathWaterErase |
| 25 | chapters/resources/audio_resources.rs.md | `Bank::load_sample_data` | 0x140a53870 | 一致 | callee=libfmod::Bank::load_sample_data(0x141435750)+path fmt；串 "Loading sample data of FMOD bank "@0x142ad2618 |
| 26 | chapters/systems/audio/replay_audio.rs.md | `replay_audio_system` | 0x140ad5800 | 一致 | 反编译 line115 `==0xd`（AppMode==0xd）+ kind==1→EventInstance::start(0x1414372d0)+串 "event:/UI/DenyInput"@0x142ae1778（undoreplay/replay 变体） |
| 27 | chapters/resources/grading_lut.rs.md | `_::impl$0::serialize<serde_json>` | 0x140af00e0 | 一致 | callee 全为 serde_json::value::ser::serialize_field<bool/String/f32>+end = 纯 serde 胶水（按规则不计业务） |
| 28 | chapters/systems/audio/game_loading_audio.rs.md | `init_game_loading_audio` | 0x140b1ae70 | 一致（MCP 补证） | 原行无证据。MCP 反编译：instantiate_audio(0x140a54440, "snapshot:/Loading"@0x142ae94a8, len 0x11)+libfmod::EventInstance::start+Vec 输出。已补证据 |
| 29 | chapters/resources/shader_refactoring_state.rs.md | `ShaderRefactoringState::disabled` | 0x140b1f210 | 一致 | 机器码 `cmp dword[rcx],0; sete al; ret` = [+0]==0（disabled），与行述精确吻合 |
| 30 | chapters/systems/audio/wall_audio_orchestrator.rs.md | `wall_audio_orchestrator` | 0x141234bf0 | 一致 | callee=hash_one<GlobalId>（hashbrown 查 RequestWallMaterialAudio）+MaterialStats::max_pillar_movement / which_material_changed_the_most+EventWriter::write<PlayWallAudio>+ArrayVec<WallSound,4>::push |

### render（10 条）

| # | 章节文件 | 函数 | VA | 判定 | 证据说明 |
|---|---|---|---|---|---|
| 31 | chapters/systems/render_loop/session_selection/mod.rs.md | `render_session_selection_previews` | 0x1408f6c00 | 一致 | 串表：两 assertion 原文（"backed_range.len() <= THUMBNAIL_POOL_SIZE.min(save_list.sessions.len())"@0x142aa8b69、"idx < self.texture_pool.len()"@0x142aa8d15）+painterly 缩略图 shader 串；callee=save_folder_path+Path::_join+image Reader/decode/to_rgba8+GraphicsDevice::update_texture+DrawBuilder::new/draw_no_vao+RasterPass::render |
| 32 | chapters/systems/render_loop/draw_passes/draw_shaders/stars.rs.md | `startup_stars` | 0x140903ca0 | 一致 | 串表 "star_shader"@0x142aaa046 + "/star.raster.hlsl" + "star/star.raster.hlsl"（三名称全中）；callee=load_mesh_into_library+load_shader_into_library+spawn_empty |
| 33 | chapters/render/pass_builder/draw_builder.rs.md | `DrawBuilder::bind_external_storage_buffer` | 0x140906400 | 一致 | 反汇编：插入记录 {handle, 0, u64::MAX(-1)}@0x1409066a4..b2 + rhapsody get_vk_buffer(0x140bef470) miss→0x140906783 unwrap_failed(0x1428d9310)+ud2；hashbrown SIMD 探测 |
| 34 | chapters/render/pass_builder/compute_pass_builder.rs.md | `ComputePass::bind_external_raw` | 0x14096e540 | 一致 | 反汇编：4×16B（64B=8×u64）原始参数打包 + 记录 0x48B（64B+8B 额外字段）插入 + RawVec::grow_one<indexmap::Bucket<…>> |
| 35 | chapters/render/nani/instance_data.rs.md | `CurvedHalfTimberInstanceData::new_straight` | 0x14098f970 | 一致 | 反汇编精确：+0x20 = caps*2+(has_cap?2:1)（cmp/cmove）；+0x24 = 0x3dc28f5c00000001（movabs）；+0x2c = 0x100000000（movups 双 8B，rdata 0x142ab9500）——三条常量全部命中 |
| 36 | chapters/systems/render_loop/compute_passes/roof_tiles_gravity.rs.md | `roof_tiles_gravity_compute` | 0x140a29b10 | 一致 | 串 "Roof tiles/roof_tiles_gravity.cs.hlsl"@0x142acacc0 + "/roof_tiles_gravity.cs.hlsl"；callee=AssetSsboLibrary FrozenMap::get + QueryState::new/update<RoofTiles…> + ComputePass::new/read_storage_buffer<RoofTileSsbo>/dispatch_with + UniformBinder::write_uniform |
| 37 | chapters/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs.md | `draw_instanced_butterfly_shader` | 0x1415547e0 | 一致 | callee=bind_texture_to_draw_builder×2 + read_storage_buffer<draw 资源> + DrawBuilder::draw_instanced_w_gl_draw_flag |
| 38 | chapters/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs.md | `draw_wren_shader` | 0x141555ec0 | 一致 | callee=nani_draw(0x140904ae0)（nani_wren 目标成立） |
| 39 | chapters/render/nani/gpu_world.rs.md | `impl$14::type_name<SolidAlbedoMapEmissiveInstanceData>` | 0x141572250 | 一致 | 机器码=lea 串@0x142a9d8f9（"country_core::render::nani::instance_data…"）+mov edx,0x4d(77)+ret；函数体 13B（"13B 返回字符串"成立） |
| 40 | chapters/render/nani/gpu_world.rs.md | `impl$14::gpu_flush<GothicWindowBricksInstanceData>` | 0x141572400 | 一致 | 机器码=`add rcx,0x18; jmp ShadowedBuffer::gpu_flush<WrenInstanceData>`(0x141692e70)——9B thunk（"9B 装载"成立） |

## 统计

| 域 | 抽查 | 一致 | 不一致 | 证据不足 |
|---|---|---|---|---|
| collision-eco | 10 | 9 | 1 | 0 |
| ui-input | 10 | 5 | 5 | 0 |
| audio-misc | 10 | 8 | 2 | 0 |
| render | 10 | 10 | 0 | 0 |
| **合计** | **40** | **32** | **8** | **0** |

一致率 80%（32/40）。较旧审计 56% 显著提高，但 8/40 仍存在；其中 5 条集中在同一地址簇（0x14003bb80），
实质独立错误点 5 处（FrozenMap::insert 行为、rotation callers 数、is_dragging 结论过时、ClutterMeshTy 举例名、
0x14003bb80 簇身份错位）。

## 错误模式归类（系统性）

1. **ICF/胶水地址错位（0x14003bb80 簇，9 行×3 域，系统性）**：draw_path/slider3d/steam 三章把各自的
   `impl$0::apply/new_archetype/queue` 全贴到 0x14003bb80，证据均为 NotFound 错误文件却标 O。
   PDB 过程表该址唯一符号=`bevy_app::plugin::Plugin::finish<MainCameraPlugin>`（size 1），机器码=1B `ret`。
   低地址 0x14003xxx 区是 Bevy 泛型/系统包装折叠区，作者以"疑折叠"猜测充当 O 证据 → 需"PDB 精确符号 + 机器码"双证才能标 O。
2. **NotFound 证据当作 O**（0x14092e6a0、0x1408cea20、0x14003bb80×9）：decomp 落盘为 `{"error":"NotFound"}` 仍被引用为证据。
   其中 0x14092e6a0 / 0x1408cea20 结论本身正确（本次已用机器码证实并修正证据）；0x14003bb80 簇结论错误。
3. **依赖库契约推断被机器码推翻**（FrozenMap::insert）：reconstruction 注释"返回已有值引用"与机器码不符
   （实为丢弃新值、返回标量 1）——依赖实例的行为也要机器码确认，不能只凭库语义注释。
4. **未证实数量断言**（rotation "13 callers"：DB xrefs 仅 1 DATA/0 call）。
5. **举例清单不实**（ClutterMeshTy 举例 7 项中 4 项不在串表）。

## 就地修正清单（14 行 / 8 文件，均注明"round3 复查修正"）

| 文件 | 行 | 修正 |
|---|---|---|
| chapters/systems/paths/draw_path.rs.md | L12 apply / L15 new_archetype / L16 queue | 0x14003bb80→Plugin::finish<MainCameraPlugin> 1B ret；原身份未证实 |
| chapters/systems/ui/resources/slider3d.rs.md | L11 apply / L14 new_archetype / L15 queue | 同上 |
| chapters/systems/steam/mod.rs.md | L19 apply / L21 new_archetype / L22 queue | 同上 |
| chapters/frozen_map.rs.md | L11 | 返回"已有值引用"→丢弃新值返回标志 1；补 MCP 证据 |
| chapters/systems/ui/signifiers/mod.rs.md | L22 rotation | "13 callers"→未证实（xrefs 1 DATA/0 call）；保留核心行为 |
| chapters/systems/ui/resources/ui_state.rs.md | L32 is_dragging | NotFound→机器码 cmp byte[rcx],2 = 判别==2 |
| chapters/resources/clutter_meshes.rs.md | L20 | 举例名改为串表实存路径 |
| chapters/systems/audio/game_loading_audio.rs.md | L11 | 空证据→补 MCP 反编译证据（instantiate_audio "snapshot:/Loading"+start） |

另注：`reconstruction/country-core/src/frozen_map.rs` 注释第 5-6 行"返回已有值的引用"与本轮机器码证据不符，
属 reconstruction 源码，按任务约束未改动，留给集成 owner 裁决。

## 佐证文件

- `round3/O-sample-42.json`（抽样清单）
- `round3/oaudit-job.json` / `oaudit-out.json`（MCP 反编译 0x140969bf0 / 0x140b1ae70，tg_rev1）
- `round3/oaudit-xrefs.json` / `oaudit-xrefs-out.json`（0x140969bf0 xrefs）
- `round3/oaudit-xrefs2.json` / `oaudit-xrefs2-out.json`（0x14089dc30 xrefs）
- `round3/claims-render.tsv`（render 域 O 清单，extract 变体生成）
