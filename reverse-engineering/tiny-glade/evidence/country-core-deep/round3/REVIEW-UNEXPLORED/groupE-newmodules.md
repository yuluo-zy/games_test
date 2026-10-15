# 组 E 六新深挖模块 review：未探究清单（round3，2026-10-15）

范围：asset_libraries + utils + startup + input + components + geometry（`chapters/{asset_libraries,utils,startup,input,components,geometry}/**`、各模块 `ALGORITHMS.md`、`round3/oa2-results/*`）。
结论分级沿用 O/I/U。**不改任何源码/章节**；本清单供集成 owner 裁决。

---

## 1. 未探究项总清单

| # | 未探究项 | 类别 | 缺口 | 建议 |
|---|---|---|---|---|
| 1 | `utils/ALGORITHMS.md` §3「CENTERED_PATH 写入者仍未找到」 | **陈旧引用（跨模块矛盾）** | `round3/RENDER-U-CLOSURE.md`（10:21）已机器证实写入者 = `load_game` 0x14146dc82-0x92 `setge [rax]`（字段 @+0xc0 >= 0x1a）；`chapters/utils/float_grid.rs.md` 已采纳，但 utils ALGORITHMS.md（10:12）仍写「RENDER_GAP §5.2 U 项维持」——与自身章节矛盾 | ALGORITHMS.md §3 改引 RENDER-U-CLOSURE，写入者闭合；`sampling_fix` 接线点=load_game |
| 2 | asset_libraries `load_json_as_mesh` 实现位置 | **陈旧引用** | utils 深挖已全流程闭合（`utils/ALGORITHMS.md` §1：0x140b0eeb0/0x140b10ed0/0x140b11f50/0x140b0fe60 + `read_attribute_from_json` 0x14083e210 + `set_attribute` 0x140a35240），但 asset_libraries `ALGORITHMS.md` §3/§13 与 `chapters/asset_libraries/mesh_library.rs.md` 纠错节仍列「U：load_json_as_mesh 真实实现（B_x_out 遗留）」 | 资产库侧更新引用到 utils 章节/ALGORITHMS；U 移除 |
| 3 | texture_watch 消费端（backend 系统） | **陈旧引用 / 部分闭合** | oa2 w9（`round3/oa2-results/w9_audio_backend.json` /9）已证 `systems::backend::texture_watch::texture_watch` = **0x140aab370**（PDB 2640B，decomp 23774B 有效；锁 Mutex@+0x10、日志「Texture watch: detected changes: 」0x142add810/「Error processing textures: 」0x142add878、FUN_140b73a50（u16 位图载体）×2、FUN_140917860、FUN_140be24b0、func_0x14003bb80 no-op）。asset_libraries ALGORITHMS.md §9/§13 仍写「U」。**残余 U**：FUN_140b73a50 身份与到 texture_library 的重载调用链未追踪 | 消费端地址/流程引 w9 证据；FUN_140b73a50 归入后续 worker |
| 4 | `PlacementDataSsbo` JSON schema | **真 U（未探究）** | `load<PathBuf>` 0x1409365d0 五步已算法化，但 JSON 字段（u32/f32/i32/[f32;3]/[f32;2] + pack_2x16f_uint 候选）无任何证据；无新反编译 | 单独工作包；找 placement JSON 资源（assets 侧）或 serde visitor 反编译 |
| 5 | `transient_meshes` 元素类型 + 布局不一致 | **未连接（跨模块矛盾）** | mesh_library 章 `transient_meshes` = `Arc<Mutex<Vec<_>>>` @+0x18（ArcInner 0x30：强/弱@0、futex@0x10、poison@0x11、Vec@0x18）元素类型 U；components §4 `TransientMeshRegistry` = `Mutex<Vec<u64>>` 24B（state@0、poison@1、Vec{cap@8,ptr@0x10,len@0x18}）——**两布局不同**（Mutex 类型/尺寸、Arc 包装），疑同一队列两种读取面或两个独立结构 | 裁决：drain_meshes_to_delete 0x1412e9e70 锁的 Vec 到底挂在哪个字段；mesh_library 元素类型按 u64=mesh_handle 试填 |
| 6 | MainActionSet 双形态字段序 | **待裁决（未落实）** | input ALGORITHMS §6.1：运行时 0x170B digital@0x00..0x140 在前 vs serde 0x58B analog@0x00..0x30 在前，字段序相反。`reconstruction/country-core/src/input/action_set.rs` 现有 `pub struct MainActionSet{analog, digital}`（analog 在前）+ 注释「Field layout U」——**与运行时形态不符**，单结构无法同时表示两形态 | 按 ALGORITHMS §6.1 建模双形态（运行时 digital-first 0x170B + serde 专用 0x58B），或拆两个类型 |
| 7 | InputFrame 两视图偏移冲突 | **待裁决** | serde `translation_state`@0x00 vs 回放读取状态块 @0x68..0xc0（→state+0x58..0xa8）冲突已登记 §6.2；无集成裁决 | 集成时以 serde 键序为帧内布局，回放偏移单独建模 |
| 8 | `prev_normalized_cursor_pos` 26 字符 | **结论已定、落码未验** | ALGORITHMS §6.3：串表 0x142c5b3e1 起 26 字节，非 30 字符。reconstruction 目前**无该字段**（`input_stream.rs` 无 prev_normalized 名），无从核对 30 字符写法 | 恢复落码时按 26 字符；无需改现存代码 |
| 9 | InputStream serde `deserialize::visit_str` 0x1413fe800 | **算法未深挖（仅结论级）** | 章节行仅「JSON 变体名匹配」结论；action_set 的 visit_str 0x1416f92d0 有全算法（长度分派/SIMD 窗/unknown 0x2e 哨兵），InputStream 侧（sync_points/frames/mode 写入序）无对应 | 补反编译 + 长度分派/键窗分析；与 try_load 写入序 §6.5 一并定 |
| 10 | 图集 parts key 常量语义（9/24/19/26/21） | **真 U** | startup ALGORITHMS §6.7：stone_floor_=9、timber_=24、plaster=19、stairs_floor=26、smoke=21，key 语义未定 | 疑固定数组索引/哈希；由 mesh_atlas 消费端索引模式裁决 |
| 11 | startup 28-系统逐名 | **真 U（Bevy 泛型 tuple）** | ALGORITHMS §1.1：28 系统 tuple 不暴露名字；PDB 仅证 Audio 组 + create_preset_textures | 低优先；由各单资产生产者系统行名反推（本章节已覆盖 16 个） |
| 12 | `FloatGrid` NaN 边界（共享侧） | **待修复（未落实）** | 共享 `shared-foundation/crates/utils/src/float_grid.rs:8` 注释自认「B's max/min converts NaN to zero; clamp preserves NaN」但**代码未修**（仍 `v.max(0.0).min(1.0)` 保 NaN） | 去重落地前按 B 显式 NaN→0 |
| 13 | `cache_prev_object_transforms` 参数面/语义 | **I 级推断** | components-geometry ALGORITHMS §3：`+0x14` u32 kind、`+0x110/+0x118` 两帧索引、velocity_only/wren 消费均为 I | 由 SparseInstanceBuffer 写路径/消费方反查参数面 |
| 14 | InstancedWall `new` 0x140858800 / `impl$1::from` 0x140858790 | **无行无算法** | 两业务函数无章节行、无 ALGORITHMS 描述（new=构造；impl$1::from=From 转换，语义未知） | 短函数，rne-read 手解即闭合 |
| 15 | utils `Mesh::set_attribute` decomp 证据持久性 | **证据落盘风险** | decomp 仅存 `/tmp/out-tgm3/decomp-0x140a35240.txt`（12.6KB，现存在）——/tmp 非仓库持久路径；load_json 章引用此文件 | 拷贝到 `utils/decomp/` 或复用 asset-libraries/decomp |
| 16 | 组件计数「45 mod.rs」 | **计数偏差** | components-geometry ALGORITHMS §1 头「45+12+1」；实际 mod.rs 表 44 行、SOURCE_MAP mod.rs register 函数 44（+9 clone_behavior +1 serialize=54） | 头改 44+12+1=57；或在表内补缺的 1 个组件行 |

---

## 2. 六模块逐项卡点复查

### 2.1 asset_libraries（12 章，387 业务函数）
- **texture_watch 消费端**：见清单 #3。w9 证据已存在但 ALGORITHMS.md 未引 → 陈旧。
- **load_json_as_mesh 实现位置**：见清单 #2。utils §1 全流程已闭合 → 陈旧。
- **PlacementDataSsbo JSON schema**：见清单 #4，真 U。
- **FrozenMap::insert 集成裁决**：`reconstruction/country-core/src/frozen_map.rs` 现实现 `insert → Result<(), &V>`（键存在返回既有值引用、不替换、不 drop 新值）。与机器码 §0.2（键存在 → 表内 drop 新值 Box + 返回 Err(1)，调用方再查表）**行为等价但结构不同**（drop 时机/调用方是否重查）。集成裁决：确认重构方 Err 携带既有值引用的替代方案成立；借用字节 @+8 的 try_borrow().unwrap() panic 语义（get_mut/remove 冲突 panic）重构方未实现，需补或登记降级。
- 其余 U：transient_meshes 元素类型（清单 #5）；compute shader RasterPipelineDesc 第二字段 Option<String>（I）。

### 2.2 utils（8 章 + float_grid）
- **float_grid NaN 边界修复**：见清单 #12，共享侧待修。
- **共享映射 16 项去重状态**：见 §5。确认等价=index_condenser/NameHash/float_grid(差异NaN)/Transform/curve/geometry 面/rdp/swizzlers/comparative_space；game-own=load_json/dirty_tiles 位图/debug_viz/geo 网格/EventRegistry/AudioLoop/ChildWall/project_cursor_onto_ray。注意 MODULE-DEEP-DIG 称「16 项」但 ALGORITHMS §4 表 17 行（10 共享 + 7 自有）——计数口径待统一。
- **CENTERED_PATH 写入者**：已闭合（RENDER-U-CLOSURE），ALGORITHMS.md 陈旧（清单 #1）。

### 2.3 startup（35 章）
- **无大 U** 复核成立。计数修正已落实：spawn_brush_previews 24 次载入（章节 ✓）、debug 35 次（章节 ✓「21x 名式+14x 文件式」）、load_shaders ~13（章节 ✓）、"0xe"=路径长度、id 真名、主题小写（ALGORITHMS §6 全部已在章节体现）。
- 残余 U：28 系统逐名（#11）、图集 parts key 语义（#10）。

### 2.4 input（4 章）
- **MainActionSet 双形态字段序**：登记正确；reconstruction 未按双形态建模（#6）。
- **InputFrame serde 偏移冲突**：登记正确（#7）。
- **prev_normalized_cursor_pos 26 字符**：结论正确；reconstruction 无该字段（#8）。
- **InputStream deserialize visit_str 0x1413fe800**：算法未深挖（#9）。
- 额外复核：reconstruct 0x140948727 跳表坏数据、try_load 写入序 §6.5 均已按机器码登记。

### 2.5 components + geometry（3+4 章，agent 进行中）
- **组件→required 映射表**：213 个 register 全扫，无 required；表 44（mod.rs）+12（drawable）+1（transient）=57 行，头写 45 有 off-by-one（#16）。TypeId 抽查（w2/w3/w6）确认 impl$9=NaniInstanceDesc、impl$12=NaniActiveInstance、impl$27=AlphaMap、impl$52=UiDrawOrder、impl$82=WallFlagCloth，映射可靠。
- **cube/plane From 判定**：均判定 From 并给出全算法（§6，0x140a05530/0x140a06370）。**sphere try_from 0x140a06b00 有章节全算法但 ALGORITHMS.md §6 未收**（仅「共同点」一句）——汇总层缺口（见 §3）。
- **clone_behavior 归类**：17 Custom + 共享 Default 已登记；reconstruction 全组件 derive(Clone) 标注与二进制不符已登记。

---

## 3. 算法未深挖清单（仅结论级、无算法级描述）

对照 SOURCE_MAP 六模块业务函数（634，其中 ssbo 334=12 模板族已按族归并）vs 各 ALGORITHMS.md（地址级搜索，见 `round3/REVIEW-UNEXPLORED/_addr_scan.txt`）：

**startup（16 个单资产生产者系统）**——章节有「实现方式（算法级）」短节（载入计数/记录数级），ALGORITHMS.md 仅 §1.1 一句带过，无逐资产流程：
`debug` 0x1408efd40、`init_color_library` 0x14082c000、`load_audio` 0x140b0e0f0、`load_ui_shaders` 0x140aaaf80、`load_shaders` 0x140aab0c0、`spawn_human_ref` 0x14089d5a0、`startup_blit` 0x140901430、`startup_critters` 0x1409d49a0、`startup_deferred_raster` 0x140a97a30、`startup_garden_brush` 0x1415f87c0、`startup_grass` 0x140a21620、`startup_ivy` 0x140a21a30、`startup_roof` 0x1414547d0、`startup_sky_and_sun` 0x140864430、`glade_startup_walls` 0x140a92670、`startup_water` 0x1408f1a20。
（注：`load_glade_audio` 0x1408be360 / `load_glade_grading` 0x1408bf9c0 / `filter_mesh_by_vertex_color` 0x140a08680+0x140a08d30 已在 ALGORITHMS §2.6/§2.3 覆盖，不计。）

**utils**：`debug_viz` 14 个短函数（DebugSphere::color/solid/show、DebugRectangle3d::color/line_width/dashed_line/show、DebugRectangle2d::show、DebugLine::color/set_ui_pass/dashed/line_width/show）——章节「核心算法」有 show() 族四步流，ALGORITHMS.md §4 仅「游戏自有」标注；`geo::update_curve_mesh` ×3 闭包实例（0x14157f130/0x14157eda0/0x14157ef70，章节核心算法有流、ALGORITHMS 无）；`utils/mod.rs get_last_event ×3 + drain_events`（0x1402b4910/0x1402b4aa0/0x1402b4c40/0x1402b47f0，章节有 EventRegistry 算法）；`AudioCore::instantiate_loop` 0x140a59220。

**input**：`InputStream deserialize visit_str` 0x1413fe800（#9，真缺口）。

**components+geometry**：`drawable impl$2::build` 0x14082bd70（章节结论级：tail-call 0x140b6633f 委托，可接受）；`geometry/sphere::try_from` 0x140a06b00（章节全算法、ALGORITHMS.md 未收）；`InstancedWall::new` 0x140858800 + `impl$1::from` 0x140858790（无行无算法，#14）；`components/mod.rs serialize` 0x14036a4f0（derive 胶水，章节已归）。

**asset_libraries**：无（12 模板族归并 + 各库行级全 O；短函数均有机器码证据）。

---

## 4. 无结论行检查

对六模块全部章节表（`_rows_evid.json`，281 行带结论）扫描：
- 行含地址但**缺 `**O**/**I**/**U** 标记**：0 条。
- 「作用」列仅「猜测」未证实：0 条（全部「猜测：旧文案；证实：新结论」双写）。
- 证据列为空/「—」：0 条。
- **结论分布**：O 295 / I 1（唯一 I = `ProportionalScreenSpace::transform_screen_space` 0x140ac8450，helper FUN_140c906d0 常量未解析——章节与 ALGORITHMS §6 一致登记）。
- **章节行 vs SOURCE_MAP 业务函数覆盖**：292/634 逐名命中；未命中 342 = ssbo 334 模板族（章节 12 族行含计数，334=78 as_any+78 as_any_mut+4 buffer+4 handle+73 add_new+9 add_new_with_data+5 get_mut+81 get_rg_buffer+1 BufferState::new+1 export_from_rg 完全覆盖）+ instanced_wall 4（#14 + 2 个 register 胶水）+ sphere 1（new）+ dirty_tiles 3（iter_world_space_rect_cells 闭包实例，章节有汇总行）。

---

## 5. 证据链抽查（10 条 + 全量）

对六模块章节全部 **113 个 decomp-*.txt 引用**做存在性+内容抽查（比随机 10 条更强）：
- **110 个文件存在且非 NotFound**（分布于 asset-libraries/decomp、startup/decomp、input/decomp、expansion-b1/decomp、expansion-b2/decomp、backend-misc/decomp、ui-input/decomp 等目录）。
- **3 个行引用了 NotFound 错误文件（tg_b2）作唯一证据**：
  1. `chapters/startup/clearing.rs.md` `clearing::…::deserialize::visit_str` 0x140a14620 — 证据列 `decomp-0x140a14620.txt` = `{"error":"No function at 0x140A14620","database":"tg_b2"}`；O 结论仅靠坏文件。
  2. `chapters/startup/init_photomode_state.rs.md` `on_enable_photomode` 0x140870570 — 证据列同样坏文件；**算法节已注明「机器码手解为准（rne-read --hex 0x140870570）」**，证据列未同步更正（引用卫生）。
  3. `chapters/startup/mod.rs.md` `impl$1::build` 0x140b1ac90 — 证据列坏文件；作用列有机器码（tail-call），证据列未更正。
  建议：三行证据列改引 rne-read/有效文件；clearing visit_str 需补真实证据（serde 胶水，业务值低但证据断链）。
- **/tmp 依赖**：utils `Mesh::set_attribute` decomp 仅存 `/tmp/out-tgm3/decomp-0x140a35240.txt`（清单 #15）。

**oa2 抽查覆盖对照**（`round3/oa2-results/`）——六模块有多条被覆核：
- `w9_audio_backend`：`startup::debug` 0x1408efd40（9 路径串逐字节 ✓）、`backend::texture_watch` 0x140aab370（=清单 #3 消费端证据）、`DebugSphere::radius` 0x140b024e0（31B，**builder 拷贝形态非 movss 直写**，重建签名需复核）、`DebugRectangle3d::dashed_line` 0x140b02ea0（38B 整结构拷贝）、`DebugRectangle2d::show` 0x140b04640（945B decomp 有效）。
- `w3_top_nonrender`：`update_color_library`、`get_last_event`、`load_mesh_atlases`、components register 胶水（impl$9/impl$12/impl$27/impl$52/impl$82 身份）。
- `w6_ui_camera`：`input_stream::serialize` 0x1402e0060、`translate_input` 0x1417cd0a0、signifiers register 胶水。
- `w7_history_save`：`update_prefabs`（触 audio_trigger_library 路径）、`PrefabInstanceData` register、`GladeType::is_flowery` ICF 同址。
- `w1_pass_builder`：`ComputePass::new`（仅 shader_library get_compute_by_name 消费链触点，非本模块）。

---

## 6. 共享库去重对照（utils ↔ shared-foundation/crates/utils）

| country-core | shared utils | 状态 |
|---|---|---|
| `utils/load_json.rs` | 无 | 游戏自有（68 调用方）；不参与去重 |
| `utils/dirty_tiles.rs` 位图本体 | tabloid 协议（crates/tabloid）+ 镜像测试 | 位图自有；State/StateDelta/Publisher 复用已证语义 |
| `utils/debug_viz.rs` | 无 | 游戏自有命令队列 |
| `utils/geo.rs` 网格构建 | geometry/curve 数学面 | 构建自有；底层类型共享等价 |
| `utils/mod.rs` EventRegistry | 无 | 游戏自有（TypeId→Events HashMap，非 Bevy Events） |
| `utils/audio_utils.rs` AudioLoop | 无（kira 复用） | 游戏自有 |
| `utils/project_cursor_onto_ray.rs` | camera.rs 不重叠 | 游戏自有 |
| `utils/child_wall.rs` | 无 | 游戏自有 |
| `utils/float_grid.rs` | `float_grid.rs` | **等价恢复**；差异=NaN→0 未修（清单 #12）；`sampling_fix`=CENTERED_PATH（写入者已闭合） |
| `utils/index_condenser.rs` | `index_condenser.rs` | **确认等价**（共享版同址 0x140a19f50 恢复；swap_remove+搬移索引一致） |
| `src/name_hash.rs`（xxh3_64） | `name_hash.rs` | **确认一致**（seed 0 默认 secret） |
| `utils/components/transform.rs` | `components/transform.rs` | 布局一致；**共享版 look_at/abs_diff_eq 已实现（0x140c9ce00/0x140c9d0d0），country-core 同名方法仍 todo!——去重以 shared 版为准** |
| `utils/curve.rs` | `curve/` | 等价恢复面；country-core 侧多为 wp12 facade/typed gap |
| `utils/geometry.rs` | `geometry/` | 等价恢复面（字段布局有证据） |
| `utils/rdp.rs`/`swizzlers.rs`/`comparative_space.rs` | 同名 | 等价恢复面（RENDER_GAP §7.1 方向成立） |

**裁决清单**：可直接去重 = index_condenser、NameHash、Transform（以 shared 实现为权威）；可去重但需先修 = float_grid（NaN）；等价面待接线 = curve/geometry/rdp/swizzlers/comparative_space（country-core 侧多为 typed gap，落地时以 shared 为源）。口径注：MODULE-DEEP-DIG「16 项」vs ALGORITHMS §4 表 17 行（10 共享 + 7 自有），计数需统一。

---

## 7. 优先级建议

1. **陈旧引用清理（1-3, 15, 16）**：utils/asset_libraries ALGORITHMS.md 三处 U 已闭合未更新；三行 NotFound 证据列；/tmp decomp 落盘；45→44 计数。全部为文档级修正，成本低。
2. **真 U 定向（4, 9, 10, 14）**：PlacementDataSsbo schema、InputStream visit_str、atlas key 语义、InstancedWall new/from——各 1-2 个反编译即可闭合。
3. **裁决项（5, 6, 7）**：transient_meshes 布局、MainActionSet 双形态（reconstruction 现单结构 analog-first 与运行时 digital-first 不符）、InputFrame 偏移——需集成 owner 定。
