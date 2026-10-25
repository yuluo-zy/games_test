# 未探究/未调查清单（5 组 review 合并）

来源：round3/REVIEW-UNEXPLORED/group{A,B,C,D,E}-*.md。分类：可再挖（有机器码路径）/ 卡点（本质 U）/ 裁决项（跨域或依赖边界）。

## A history/session/camera

- **无结论行检查**：SOURCE_MAP.json 三域 business 函数（带 VA 且 kind=business）已全部在章节有
- **主要系统性发现**：三域共有 **25 行 O 结论引用的 decomp 文件是 NotFound 错误文件**（`{"error":...}`），
- camera-time 早期版本（本审查提取器首版）曾把 focus/lens/eye_forward 三行漏解析，系行内函数名带
- SOURCE_MAP business 中 `MainCamera::eye_forward`（0x140911df0）与 DIGEST 用名 `eye_position`
- 三域章节的"内联/无独立符号"列表里标"待推断"的项（如 load_game 的 apply_parsed_game_data /
- **camera-time（12）**：0x14003bb80×4（butterfly 3 + free 1）、0x140994310（walk rotation，round2 已 CLOSED）、
- **session-save（7）**：0x140ae8320×2（is_corrupted/is_winter 折叠）、0x140b468c0（is_xmas2024）、
- **history（6）**：0x140a84290/0x140a842a0（is_*_diff_empty）、0x140a84c40（debug_set_history_start，3B 手解）、

## B wall/collision

- **11 行 I 且引用 decomp 全为 NotFound 错误文件**（推断 + 坏证据，最弱）：
- `impl$8::update` 0x140a1b5a0：I 但证据列空白（NO-REF）——结论来自 round4 手解，引用需补。
- 另有 I 行描述错位（章节质量问题）：decorator_storage 章节 verify_integrity 0x14003bb80 行描述写
- U=1：`impl$41::into_transformed_shape` 0x14155d340 —— **有有效 decomp（引用文件 OK）**，行注
- I=3：collision::world::impl$11::from 0x1408cc630、stairs_state::impl$1::from 0x1408fe5c0、
- 提示：into_transformed_shape 族共 4+ 地址（0x14155d340/0x1408d98c0/0x1408d9960/0x140b26d20），
- wall：SOURCE_MAP business=442，章节行覆盖 433+（精确比对 9 行未匹配，其中 8 行是
- collision-eco：business=213，章节覆盖 212（**1 真缺：`path_brush_preview_size::path_brush_preview_size`
- 命名冲突另记：**material_stats 0x140a887d0**——decomp 文件头/DIGEST 称 `arches_length_delta`，
- 章节"内联/无独立符号"清单中真实业务谓词仍未推断（不计行数但影响语义）：
- wall/decomp：405 文件，303 OK / **100 NotFound 错误**（24.7%）/ 2 DecompFailed
- collision-eco/decomp：202 文件，168 OK / **30 NotFound 错误**（14.9%）/ 4 DecompFailed
- 章节行层面：O 行 469 行中 **137 行（29%）仅引用 NotFound 错误文件**；I 行 83 行中 11 行同

## C terrain/ui/audio

- 日期：2026-10-15；依据：三域 DIGEST/REVIEW/CLOSURE + round3 oa2（w6/w9）+ round3 O-AUDIT-REVIEW（seed=42 40 条）+ SOURCE_MAP 交叉 + 本组独立 10 行抽查。
- 判定记号：O=机器码证实 / I=有界推断 / U=未恢复；**已修**=章节/source 已按抽查修正，**未修**=抽查指出的瑕疵仍在。
- 未修改任何源码/章节（仅本报告）。
- 三域 SOURCE_MAP 业务函数（kind=business 且带 VA）共 **575** 条，**全部**被章节文本地址级覆盖；**0 条** business 函数缺 VA；0 条完全无章节提及。
- 仅 **6 行**只有地址提及、无名称级结论：`input/action_set.rs`、`input/input_stream.rs` 的 serialize/deserialize/visit_str 各 3 条——全部是 serde derive 胶水（按项目规则复用 derive、不计独立业务）。
- **分类口径问题**（非缺口）：这 6 条在 SOURCE_MAP 中仍标为 kind="business"，与 AGENTS.md「serde visitor 直接复用 derive 不计业务」规则不符，建议 SOURCE_MAP 改标 glue（如 round3 v2 表对 terrain serde 单态的做法）。
- 结论质量分层：多数行有"地址+断言+证据文件"；少量行仅"地址+一句话"（如 debug_viz 两行"原字节短函数"、rocky_terrain load）——抽查证实方向正确但证据链薄，属补强级非错误级。
- w6 两行"证据不足"（SignifierEntityBuilder::new、move_gizmo_and_trigger_vol）的**依据（wp15/wp20 目录缺失）是错的**——两目录及所引文件均实存；对应章节已另补证并修正，建议 oa2 汇总时作废该两行原始判定。
- 0x14003bb80 簇为三域系统性错误源（9 行×3 域，O-AUDIT 已修）；本组复核 world_raster.rs.md 该址 3 行（apply/new_archetype/queue）本就正确。
- terrain-water 域 O 抽查覆盖率最低（oa2 仅 brush_preview 1 行 + 本组 8 行），建议下轮补抽（候选：rasterize_stroke 三 blend 闭包、copy_to_staging、subscribe、mesh_follow_terrain、draw_garden）。

## D render

- render_loop DIGEST 口径：78 文件 / 159 业务 / 48 内联 / 121 胶水；decomp 156 个（146 OK + 10 NotFound 手解）；章节占位符清零。抽查复核：**成立**（见 §5、§7）。
- roofs-rt DIGEST 口径：20 章全填充，decomp 63/89 批处理 + 26 手解 + 12 既有 .c；"所有业务函数至少 I 级，无臆造字段名"。抽查复核：**成立**（3 个 I 全列出，见 §1.2）。
- backend-misc DIGEST：16 文件 21 业务 + 5 助手地址。复核：与既有关联记录一致，无新 I/U 引入。
- RENDER_GAP §10 闭合记录 9 项 O / 2 卡点——卡点复查见 §2。
- `gpu_generate_draw_lists::IndirectArgsType::buffer_offset` — O 名、语义 I。
- `compute_path_masks` 运行条件 A∥B∥C∥E非空∥D 中 E 的事件驱动语义（I）。
- `prefix_scan` 复用链 `/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl`（I）。
- `session_selection` 三层索引中 render_session_selection_previews 与 wp5 无冲突（未列 I，但部分路径依赖 painterly 缩略图语义，I）。
- `RasterPipelineDesc` +0x18 字段语义（feature/binding 选择子）——U。
- `FrameConstants` 全 UBO 字段列表（main_view/sun_view 成员名 O）——U。
- `ComputeShaderProgram` 字段级布局细节——U。
- `AvailableGraphicsDevice` 160 B 记录字段布局（reconstruction `_raw:[u64;20]` typed gap）——U。
- `nani::draw_ext::nani_draw`：External subset 的 draw_count 来源；Owned 的实例间接缓冲 RG 句柄——U。
- `nani::buffers`：`GpuInstance` 字段（B_n_world U 清单）；`NaniDrawListParams::{list_draw_merge_window_range, list_draw_merge_window_subtotal_range}` 语义——I/U。
- `nani::systems`：`nani_update_*` 各 kind 载荷字段已有 20 系统 O，残留 `Roof/Floor 双缺 unwrap` 等边界语义已 O；`InstanceHandle::subset`（nani.rs.md）——待推断残留。
- `render_config::VideoSettings::upgrade`（render_config.rs.md 行 20）——待推断残留。
- `dirty_index_range::DirtyIndexRange::to_range`（dirty_index_range.rs.md 行 12）——待推断残留。
- `nani.rs.md` `InstanceHandle::subset` / `impl$3::component_ids`——待推断残留。
- SsboId `0xF78C9C2EB9E1F6FB` O（`add_new<compute_garden::Flower03Ssbo>` 0x1412c4480 立即数）；NAME 穷举（全二进制串 / 1459 SPIR-V 标识符 / 12558 HLSL `__alias_*` / ~1.2 万花名组合 / 类型名串）无命中；工具自检 6 例全中。
- 同态 U：`IvyFlowerSsbo`=0x1d6de5e69ac50d2a、`LilypadFlowerSsbo`=0xAE37CBCAFBE88DA4（add_new 立即数 O）。
- reconstruction `compute_garden.rs` 仍以占位 `Flower03Ssbo`（xxh3=0x5f3a187f0dd4b356≠键，**已确认错误**）并自注 U。
- 建议：以 SsboId 常量锚定占位（安全），NAME 保持 U 登记；可选低价值方向：w2/w4 已验证的 `get_rg_buffer` 槽名表（0x142c8151d 系）再扫一轮——若花族 ssbo 的 NAME 实为槽名则已在搜，否保持卡点。
- 0..0x18 全映射表 O（尺寸/格式/用法链/key）；11/25 名字 xxh3 复算命中（brdf_lut/terrain_lighting/ShadowMap/water_mask/garden_mask/rocky_terrain/ObjectGrassPush/…）；0x19..0x1f 死变体 O。
- 名字为编译期常量、二进制无串、候选空间（SPIR-V + 二进制串 + 结构化候选）穷尽 → 仍 U。
- **可推进方向（新）**：映射 preset 索引 → 消费方 pass。`From<TexturePreset>` 调用点（各 compute/draw pass 的 TextureDesc::from(preset=N)）已可定位；preset N 对应 pass 内 shader 资源名/槽名已知族（reconstructed HLSL 字典 95% 命中）——用"preset 槽在哪个 pass 以哪个 name 绑定"交叉推 14 个未知名（如 preset 0xb/0xc/0xd 的 storage+sampled 512² 常被 water/GI 链使用）。该方向未在 RENDER-U-CLOSURE 记录中执行，属**未探究**。
- SOURCE_MAP-render.json：render/** **782 业务函数**。
- 章节表行地址覆盖：598/782（**76%**）。未逐地址覆盖的 **184**：
- `render/ssbo.rs` 113 —— **族级归并**（6 族，代表地址 + 布局表；ssbo.rs.md "113 业务 → 6 族"）。族内各实例（如 `new_storage_buffer` 38 实例）未逐实例反编译，靠同族形态推断——**结论级 I 可接受**，但每个实例的 T 类型→地址配对只在代表行登记。
- `render/shadowed_buffer.rs` 39 —— 族级归并（22 类型 new+gpu_flush），同前。
- `render/sparse_instance_buffer.rs` 31 —— 族级归并（4 类型族），且章内自注"每类型族内 hide/show/take_out_waste/gpu_flush/push/clear/to_draw 精确 VA 配对未逐条反编译，按同类型簇地址连续推断（**I**）"。
- `render/mesh.rs` 1 —— `impl$7::from` 已并入"from（Vec 属性 From impls）"合并行（0x140a0db00/0x140a0db90），非缺口。
- 文件级覆盖 **37/37 全有章节**（无缺章文件）。
- **〔未探究〕族级归并行的逐实例地址配对**是 render 域最大的"无结论"面：782 业务里 184 行（24%）只有代表地址，实例级 VA 未独立核验（ICF 折叠、同型异参 callee 等以 w1 的"read/write_access_buffer 53/37 实例 ICF 描述不精确"为证，见 §5 末）。建议：对 6 族各抽 1-2 个非代表实例做 rne-read 首字节 + procs.tsv 身份核验，闭合"族归并"风险。
- **身份与机器码层面全部通过**（这些 O 行虽由远端会话按 B_* 审计文档 + wp1/wp3/wp4 合同填写，但地址→符号→机器码三者一致，抽样未发现错误结论）。
- **证据链层面存在系统性缺口**：render 顶层 37 章 498 个唯一地址中 **490 个无任何 decomp-*.txt 落盘**（仅 8 个有，且其中 0x140002370/0x14003bb80 是跨域 NotFound 副本/已知 ICF）。对比 render-loop 域 132 地址仅 1 个缺 decomp（0x140a66970，grass::impl$4::register_required_components，且该族 round3 有 rrc 兄弟文件）。→ render 顶层的 O 结论依赖 B 阶段审计文档/合同，非独立 decomp 链；**若需交付级证据，须为这 490 个地址补 decomp（或至少每个归并族抽代表实例落盘）**。
- **已知措辞瑕疵与本章节冲突**：w1 已核 "read/write_access_buffer '53/37 实例 ICF' 描述不精确（每实例 per-T callee 不同，非折叠）"，而 render 顶层 `pass_resource_access.rs.md` 的 `read_access_buffer` 行仍写"同一泛型（53 实例，ICF）"——**章节措辞待修正**（同族实例是独立函数，非 ICF 折叠）。
- `raster_single.rs.md` 0x1409d00a0 行的证据引 `round3 oa2-out/w2-decomp-1409d00a0.txt`——**该文件不存在**（oa2-out 目录不存在；w2 的核验在 MCP 会话 tg_oa2w2 内完成未落盘）。结论经 w2 核验成立，但证据引用需补落盘或改引。
- 7 章文件头"文件作用：**待推断**"占位残留（§1.3）。
- **内容质量分层**：nani/{gpu_world,systems,draw_ext,buffers}、pass_builder/{compute_pass_builder,pass_resource_access,shader_uri}、mesh.rs、shader.rs、ssbo.rs 等为高信息量章（含布局表/族归并表/纠错）。fsr_plugin.rs.md 质量好（0x14003bb80 ICF 空桩判定与 round2 复核一致）。
- **模板残留**：7 章文件头"待推断"（§1.3）；`dirty_index_range/run*`、`render_config::VideoSettings::upgrade`、`nani.rs` 若干行仍"待推断"。
- **证据链**：见 §5 —— decomp 落盘率 8/498（1.6%），与 render-loop 131/132 形成鲜明对比。
- **归置**：`cull_shaders::impl$18::as_compute_pipeline` 行放在 `pass_builder/shader_uri.rs.md` 而非 nani/cull_shaders 章（URI 消费者归置，可接受但注意）。
- **抽查未发现错误结论**（15/15 身份全中），但**不可独立复验**是最大短板。

## E 六新模块

- **texture_watch 消费端**：见清单 #3。w9 证据已存在但 ALGORITHMS.md 未引 → 陈旧。
- **load_json_as_mesh 实现位置**：见清单 #2。utils §1 全流程已闭合 → 陈旧。
- **PlacementDataSsbo JSON schema**：见清单 #4，真 U。
- **FrozenMap::insert 集成裁决**：`reconstruction/country-core/src/frozen_map.rs` 现实现 `insert → Result<(), &V>`（键存在返回既有值引用、不替换、不 drop 新值）。与机器码 §0.2（键存在 → 表内 drop 新值 Box + 返回 Err(1)，调用方再查表）**行为等价但结构不同**（drop 时机/调用方是否重查）。集成裁决：确认重构方 Err 携带既有值引用的替代方案成立；借用字节 @+8 的 try_borrow().unwrap() panic 语义（get_mut/remove 冲突 panic）重构方未实现，需补或登记降级。
- 其余 U：transient_meshes 元素类型（清单 #5）；compute shader RasterPipelineDesc 第二字段 Option<String>（I）。
- **float_grid NaN 边界修复**：见清单 #12，共享侧待修。
- **共享映射 16 项去重状态**：见 §5。确认等价=index_condenser/NameHash/float_grid(差异NaN)/Transform/curve/geometry 面/rdp/swizzlers/comparative_space；game-own=load_json/dirty_tiles 位图/debug_viz/geo 网格/EventRegistry/AudioLoop/ChildWall/project_cursor_onto_ray。注意 MODULE-DEEP-DIG 称「16 项」但 ALGORITHMS §4 表 17 行（10 共享 + 7 自有）——计数口径待统一。
- **CENTERED_PATH 写入者**：已闭合（RENDER-U-CLOSURE），ALGORITHMS.md 陈旧（清单 #1）。
- **无大 U** 复核成立。计数修正已落实：spawn_brush_previews 24 次载入（章节 ✓）、debug 35 次（章节 ✓「21x 名式+14x 文件式」）、load_shaders ~13（章节 ✓）、"0xe"=路径长度、id 真名、主题小写（ALGORITHMS §6 全部已在章节体现）。
- 残余 U：28 系统逐名（#11）、图集 parts key 语义（#10）。
- **MainActionSet 双形态字段序**：登记正确；reconstruction 未按双形态建模（#6）。
- **InputFrame serde 偏移冲突**：登记正确（#7）。
- **prev_normalized_cursor_pos 26 字符**：结论正确；reconstruction 无该字段（#8）。
- **InputStream deserialize visit_str 0x1413fe800**：算法未深挖（#9）。
- 额外复核：reconstruct 0x140948727 跳表坏数据、try_load 写入序 §6.5 均已按机器码登记。
- **组件→required 映射表**：213 个 register 全扫，无 required；表 44（mod.rs）+12（drawable）+1（transient）=57 行，头写 45 有 off-by-one（#16）。TypeId 抽查（w2/w3/w6）确认 impl$9=NaniInstanceDesc、impl$12=NaniActiveInstance、impl$27=AlphaMap、impl$52=UiDrawOrder、impl$82=WallFlagCloth，映射可靠。
- **cube/plane From 判定**：均判定 From 并给出全算法（§6，0x140a05530/0x140a06370）。**sphere try_from 0x140a06b00 有章节全算法但 ALGORITHMS.md §6 未收**（仅「共同点」一句）——汇总层缺口（见 §3）。
- **clone_behavior 归类**：17 Custom + 共享 Default 已登记；reconstruction 全组件 derive(Clone) 标注与二进制不符已登记。
- 行含地址但**缺 `**O**/**I**/**U** 标记**：0 条。
- 「作用」列仅「猜测」未证实：0 条（全部「猜测：旧文案；证实：新结论」双写）。
- 证据列为空/「—」：0 条。
- **结论分布**：O 295 / I 1（唯一 I = `ProportionalScreenSpace::transform_screen_space` 0x140ac8450，helper FUN_140c906d0 常量未解析——章节与 ALGORITHMS §6 一致登记）。
- **章节行 vs SOURCE_MAP 业务函数覆盖**：292/634 逐名命中；未命中 342 = ssbo 334 模板族（章节 12 族行含计数，334=78 as_any+78 as_any_mut+4 buffer+4 handle+73 add_new+9 add_new_with_data+5 get_mut+81 get_rg_buffer+1 BufferState::new+1 export_from_rg 完全覆盖）+ instanced_wall 4（#14 + 2 个 register 胶水）+ sphere 1（new）+ dirty_tiles 3（iter_world_space_rect_cells 闭包实例，章节有汇总行）。
- **110 个文件存在且非 NotFound**（分布于 asset-libraries/decomp、startup/decomp、input/decomp、expansion-b1/decomp、expansion-b2/decomp、backend-misc/decomp、ui-input/decomp 等目录）。
- **3 个行引用了 NotFound 错误文件（tg_b2）作唯一证据**：
- **/tmp 依赖**：utils `Mesh::set_attribute` decomp 仅存 `/tmp/out-tgm3/decomp-0x140a35240.txt`（清单 #15）。
- `w9_audio_backend`：`startup::debug` 0x1408efd40（9 路径串逐字节 ✓）、`backend::texture_watch` 0x140aab370（=清单 #3 消费端证据）、`DebugSphere::radius` 0x140b024e0（31B，**builder 拷贝形态非 movss 直写**，重建签名需复核）、`DebugRectangle3d::dashed_line` 0x140b02ea0（38B 整结构拷贝）、`DebugRectangle2d::show` 0x140b04640（945B decomp 有效）。
- `w3_top_nonrender`：`update_color_library`、`get_last_event`、`load_mesh_atlases`、components register 胶水（impl$9/impl$12/impl$27/impl$52/impl$82 身份）。
- `w6_ui_camera`：`input_stream::serialize` 0x1402e0060、`translate_input` 0x1417cd0a0、signifiers register 胶水。
- `w7_history_save`：`update_prefabs`（触 audio_trigger_library 路径）、`PrefabInstanceData` register、`GladeType::is_flowery` ICF 同址。
- `w1_pass_builder`：`ComputePass::new`（仅 shader_library get_compute_by_name 消费链触点，非本模块）。
