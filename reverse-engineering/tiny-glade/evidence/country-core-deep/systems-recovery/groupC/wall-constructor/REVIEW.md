# 组 C reviewer — wall-constructor 审查（reviewer-C / dbid tg_vC1）

> 日期：2026-10-16。对象：system-wall-constructor crate 复写审查。
> **交付状态：restorer-C 尚未交付实现**。`tiny-glade/crates/systems/wall-constructor/` 不存在（多次复核）；
> restorer-C 今日 10:24–10:27 完成 97 业务中 ~90 个的 decomp 批（groupC/wall-constructor/decomp/），target-status.json
> 记录了每个函数的证据归属。本文件交付"逐函数核对矩阵 + 关键数值清单 + consumer 契约"，供交付后按表复验；
> 当前判定 **in_progress（restorer）/ 待交付后 review**。结论同步 main。

## 0. 审查依据

- 清单：`SOURCE_MAP-systems.json`（wall-constructor 业务 97 项，本文件已提取）。
- 数值权威：`evidence/building-arches|building-structures|building-constructor|building-fences|building-gates|
  building-segmentation|building-platform-stairs|building-door-stairs|building-freeform-cache|building-production/*/findings.md`
  与 `B-*-verification.json`（原码差分报告，多数为 G05/G06 冻结结果）。
- 机器码：groupC/wall-constructor/decomp/*（fresh）+ building-*/B-core、B-*-native.asm。
- consumer 契约：`reconstruction/country-core/src/resources/walls/**`（PublicWalls/PublicWallState 0x2a0/arches/plaster/
  wooden_gates/curved_halftimber/elevation_support_stats/wall_path_segment 等已恢复权威资源）。

## 1. 缺失 decomp 清单（交付前补齐，restorer-C）

SOURCE_MAP 业务 VA 中 groupC decomp 尚无的 6 项（另有 1 项 rhapsody 依赖）：

| VA | 函数 | 既有证据 | 处置 |
|---|---|---|---|
| 0x141233380 | ArchWalker::walk<emit_support_beams closure> | building-structures/B-141233380-native.asm | 有原 asm，可不重跑 decomp；落盘时以 asm 核对 |
| 0x14048ab50 | ArchWalker::walk<playermade stairs closure<false>> | 无（playermade 域） | 需补 decomp/asm |
| 0x1412269b0 | generate_plaster_mesh_and_entities<Map<Filter…>> | 无 | 需补（plaster 主 mesh 生产，97 业务中体量较大者） |
| 0x141fe1de0 | add_bottom_brick（circle/rect 共用同一体） | building-*?（未定位） | 与 0x14124b2c0/0x14124eb80 同域，补反编译 |
| 0x142418f50 | construct_crossbrick_pillar | 无 | 需补（elevation 域） |
| 0x141fdca60 | rhapsody create_buffer（依赖胶水） | — | 按 DEPENDENCY_FILTERING 筛为 rhapsody 依赖，不计业务 |

## 2. 逐函数核对矩阵（97 业务；交付后按此表逐项填判定）

分组按 DISPATCH 的 13 子模块。**数值 = 必须与实现逐位一致的原常量/公式**（源：building-* findings 冻结报告 + fresh decomp）。

### 2.1 主墙构造（wall_constructor.rs / construct_walls.rs / place_pillars.rs）
| 函数 | VA | 必须匹配的关键数值 | 证据 |
|---|---|---|---|
| WallConstructor::from_curve | 0x140769760 | from_curve 栈参数面（RCX holes Vec、RDX Curve2、R8D material、R9 arch 数组 + stack +20..+A8 共 16 项）；输出 96B BrickTransformSsbo（48 affine+16 deformation+12 surface+4 seed+4 material+8 wall_distance+4 flags）；Curve2 cap-first 布局；FinalizedWallHoles 20B 单洞；MyBrick 40B；WallRow 32B | building-constructor/README.md（G04 冻结 3516/3516） |
| construct_walls | 0x140757800 | 调 add_wall_corners 0x141215540 / add_skirts 0x1412221f0 / stairs_signifiers 0x141223d10（**不** CALL arches/lintels）；A8 参数实传 0 | building-structures/findings.md |
| place_pillars / PillarProposals::add_proposal | 0x140753670 / 0x1407534d0 | proposals 装配 + 96B 记录（closure 0x14120d670） | building-*（无专门目录，fresh decomp） |
| split_into_random_rows | 0x14123e110 | 0.495 上限、0.21/span jitter、0.046/height 去近边；fastrand 1.8.0 复用 | building-constructor（160 原指令输入通过） |
| trim_rows / above_to_below / below_to_above | 0x14123eef0 / 0x14123fab0 / 0x14123fb70 | split_heightmap_by_threshold 0x141fe0750 阈值穿越/横向量化/窄条删除/扩宽；32B 行输出 | building-constructor（200 例通过） |
| resolve_hole_overlap | 0x1412411c0 | 中心拆分递归/角分类/seed/deformation/双 split flag/deque 序；38B 记录 | building-constructor（320 场景） |
| perturb_row_ends / split_horizontally | 0x141240d30 / 0x141240ec0 | 行端扰动、横向 split 数值 | building-constructor 域 |
| add_wall_corners / add_skirts / stairs_signifiers_new | 0x141215540 / 0x1412221f0 / 0x141223d10 | construct_walls 实调；角/裙边/阶梯指示符几何 | building-structures findings（G05 caller 链） |

### 2.2 拱（construct_arches / arch_walker / profile）
| 函数 | VA | 必须匹配的关键数值 | 证据 |
|---|---|---|---|
| construct_arches | 0x14075f680 | 自身消费墙事件（**非** construct_walls 内部调用）；136B 处理掩码输入；shape_tag 0/1；Pub284 constrained_extra、Pub294 原始字节、Pub28c 源 Curve2 长度；has_roof 关系；Pub294!=0 时选跨段最高 (height+groundY)；第三数组跨段 maxnum 累计；闭合轮廓 first→last 复制（无 −0.1125） | building-arches/findings.md（G05 冻结 130 例/4159 条 96B） |
| create_stone_arch_profile | 0x1407645f0 | weight=clamp(oldHeight/Pub284)·0.5；**立方在乘 weight 之前**；target=max(highestTotal−3.5, selectedGroundY)−1−groundY 下限 0；shape0/1 blend 上限=全局 arch 高−groundY | building-arches findings |
| ArchWalker::walk | 0x141232bb0 | step=bits3f0f5c29(≈0.56)、tail ratio 0.1；WallSpace/U 插值；distance pair=sourceU×Pub28c 且 endU+1 seam 解包；矩阵 tangent×length、up×bits3ed70a3e、side×0.7、段中心 | building-arches findings |
| ArchFunction::remap_t | 0x140765300 | profile remap | fresh decomp |
| shape2 专用分支（未冻结） | caller 0x14076179b..2821 | 端点 trim 0.1875（上限 0.95/双端 0.475=bits3ef33333）；inverse_lerp(segLen, lowBound2.0=native14292e6f0, highBound1.0)；15 次 0.5 Jacobi 平滑；ClutterVersion::is_initial(Pub295==0) 且 Pub288+.92>Pub284 | building-arches findings "Next shape2 gaps"（**restorer 不得把 shape2 当已完成**） |

### 2.3 檐梁（construct_lintels / place_support_beams）
| 函数 | VA | 必须匹配的关键数值 | 证据 |
|---|---|---|---|
| construct_lintels | 0x14076d9f0 | 消费 136B 掩码→14122ff90 materializer→104B (0x68) 段；third 写回=max(selGroundY−clipGroundY−.2,0)+max(Pub284+.5,Pub288)，随后 +bitsbde66667(−0.112500004) 按 LoopRange 映射 prepared index 与旧 third maxnum（oldNaN→new）；special 源 raw+54 夹 .3..2.5（有效下限 .8） | building-structures findings（178 例/3104 条 64B 冻结） |
| place_support_beams | 0x141242530 | 双向 greedy grower；合并长度 1.0/3.15；成本=total·(Δlen+.5)，左右同价取右；prefix/suffix 处理；flags 低 4bit anchors | building-structures findings |
| SupportBeamGrower::propose | 0x141fe09c0 | 463B；proposal 结构 index(0)/XZlen(8)/归一方向(12)/曲率成本(24)/point(28)/tag(40) | building-structures findings |
| add_diagonal_support | 0x141243740 | Curves extend_left/right、arch 高度采样、48B packer 0x14089d560、metadata 16B→**64B Beam ssbo** | building-structures findings |
| SupportBeamBuilder::emit_support_beams | 0x141243c20 | 私有 RDP 141fe0b90（endpoint-distance=1、epsilon=.2、short-left）；ArchWalker step=bits40499999(≈3.15)、tail .1；plank 输出在 height−.225，x 厚 .225、z 厚 bits3eaccccc；斜撑预算每侧累计长·.85、合并 whole·.5；水平扩展 length+.112500004+.1875 | building-structures findings（**勿借 .3 等其他参数**） |

### 2.4 栅栏（construct_fences / process_segment）
| 函数 | VA | 必须匹配的关键数值 | 证据 |
|---|---|---|---|
| construct_fences | 0x140765830 | hot caller 0x140765ddc 对 Pub296==1 调 emit_fences | building-fences findings |
| emit_fences | 0x1412388f0 | 首柱跳过 iff begin!=0、末柱 iff prepared_count−1>end；RNG seed=WallId inner+begin+end wrapping64；返回相邻柱对数累计 fence_segments_count/seed | building-fences findings |
| process_segment | 0x141238d80 | Perchance 0.4.0 状态；间距 smoothstep(clamp((extent−1)/5))、.7→bits3fce147a；柱 −0.1Y 偏移；64B plank 输出 | building-fences findings |
| split_curve_based_on_angle | 0x141fdf9e0 | 40 度 bits3f32b8c2、水平 dot、XYZ 重复断言 | building-fences findings |
| construct_floors | 0x1407661e0 | 墙内地板（与屋顶地板路径分开） | building-fences findings（下一 owner） |

### 2.5 门（construct_gates）
| 函数 | VA | 必须匹配的关键数值 | 证据 |
|---|---|---|---|
| systems::construct_gates | 0x140756370 | 9 参数 SystemParam 顺序：EventReader<ConstructGateCmd>, ResMut<Gates>, Commands, ResMut<AssetMeshLibrary>, Res<AssetShaderLibrary>, Res<PublicWalls>, Res<TerrainHeightsData>, Res<AssetSsboLibrary>, Res<UiSignifierStream>；120B 事件+8B EventId=128B | building-gates findings（G06） |
| update_gates_hashmap | 0x140755690 | 6 组合（changed/create/switch rebuild + wall/roof removal）despawn 顺序；重建保留空条目 | building-gates findings（B-gate-lifecycle 冻结） |
| construct_wooden_gate_doors | 0x141210110 | 门宽 **严格 >2.7** 双门扇（间距 min(.05/全长,.1)），==2.7 单门；header inset bits3f547ae2；footer 在 mesh offset 之后重载端点；右轴归一=除 length 非倒数乘；136B spawn payload；seed=inner·ordinal+leaf ordinal u32；Shader 键 solid_vertex_color_wooden_gate；负 Y 比例选择 + ratio>.5 内柱 | building-gates findings（28 原返回/1886 条 96B/46 mesh/822 顶点冻结） |

### 2.6 平台楼梯 / 玩家楼梯 / 分段
| 函数 | VA | 关键数值 | 证据 |
|---|---|---|---|
| calculate_path_wall_segmentation 族 | 0x14074f310/7a0/50420/51240/51610 | 分段/评分候选/add_splits_in_empty_range/score_split_candidate | building-segmentation（B-process/B-core + 平台链验证） |
| sample_wall_path_intersections | 0x14074c720 | 路径交叉采样、arch_function 内联 | building-production-authority/B-intersections |
| split_wall_path_segment_into_platform_stairs | 0x140751960 | 平台楼梯段拆分 | building-segmentation/B-stairs + B-platform-* 系列验证 |
| calculate_wall_path_segments_impl | 0x14122ff90 | **104B (0x68) MaterializedWallSegment** 生产（98 例/304 段冻结）；endpoint clip=(next−prev)·t+prev、cap=bitsbf75c290 | building-structures findings |
| construct_platform_stairs | 0x1407744d0 | 平台楼梯构造 + 碰撞几何（rapier 边界按 building-platform-stairs 结论） | building-platform-stairs（B-dirty-tiles/B-collision 冻结）+ segmentation |
| construct_playermade_stairs_arches_n_walls<true>/<false> | 0x140496800 / 0x14048ab50 | 玩家楼梯拱/墙双模板 | building-production findings（B-variant/B-other-kind）+ fresh decomp |
| preprocess_curve | 0x140780e20 | 曲线预处理 | building-production |
| brackets / magic_support / pillars_* | 0x1407834f0 / 0x14077a2f0 / 0x14077d520 | 支架/浮动砖/柱视觉（animate_brick 内联） | building-production + fresh decomp |

### 2.7 高程支撑（construct_elevation_supports）
| 函数 | VA | 证据 |
|---|---|---|
| circle_bottom_layer / circle_categorize / circle_pillars / circle_turret | 0x14124b2c0 / 0x14124af60 / 0x14124ba30 / 0x14124cc30 | fresh decomp（无 building-* 冻结目录） |
| rectangle_bottom_layer / rectangle_pillars / find_stone_column_start_y / rectangle_small_brackets / rectangle_large_brackets | 0x14124eb80 / 0x14124fcb0 / 0x141250720 / 0x141250da0 / 0x141253fc0 | fresh decomp |
| find_overhang_support_segments / OverhangSupportFinderCache::new | 0x141248620 / 0x141248300 | fresh decomp（43KB 大函数） |
| maintain_stone_column_colliders | 0x14078a6e0 | fresh decomp |
| construct_wooden_pillar_w_stone_base / construct_brick_columns / construct_crossbrick_pillar | 0x141fe2150 / 0x141fe3240 / 0x142418f50 | fresh decomp（crossbrick 缺 decomp，见 §1） |
| overhang_subscribe_to_tabloids / detect_overhang_support_segments / construct_circle_supports / construct_rectangle_columns / construct_rectangle_brackets | 0x140786440 / 0x1407867d0 / 0x140788160 / 0x140788a80 / 0x1407896b0 | fresh decomp |

### 2.8 半木构 / 曲面半木 / 石膏 / 杂项
| 函数 | VA | 证据 |
|---|---|---|
| half_timbering / roof_outline / overlapping_windows / get_window_aabb / check_for_overlap_with_all_windows / add_underwindow_pattern | 0x141218e30 / 0x141217be0 / 0x1412218a0 / 0x141221da0 / 0x141221f70 / 0x141fd1c30 | fresh decomp（building-* 无冻结目录——**半木构是本轮证据最薄子模块，需逐函数原码差分**） |
| curved_halftimber_maintain_entities / wooden_floor_borders | 0x1407706d0 / 0x141244310 | fresh decomp |
| maintain_plaster / create_plaster / split_wall_into_sides / register_required_components / generate_plaster_mesh_and_entities | 0x14075b810 / 0x14075d460 / 0x14075dc60 / 0x14075e9e0 / 0x1412269b0 | fresh decomp（generate_plaster_mesh 缺 decomp，见 §1） |
| init_wall_height_and_arch_segment_height | 0x14078b550 | fresh decomp；与 core WallState 高度规则衔接（对照 reconstruction/wall/height_rules.rs：clamp 下限 1.6/0.7/0.3/0.5、上限 special 2.5 否则 14.0、flat_roof_y=max_y−0.79） |
| ui_arch_controls / do_ui_arch_controls / ui_lintel_controls | 0x140766e70 / 0x140767b70 / 0x140771a60 | fresh decomp（UI 域，与 toolbar 交互后置） |
| debug_force_update_arches / gpu_clear_wall_gate_pieces / gpu_flush_plank_buffers | 0x14003bb80 / 0x140773c00 / 0x140774010 | building-runtime/B-focused（debug）+ fresh decomp |

## 3. consumer 契约 vs core WallState（restorer 必须按此接线，不得自造签名）

- `PublicWalls`（core）：get/wall_ids/wall_entity/**roof_entity**/contains——edit_roof 0x1421ab7c0 已证 `PublicWalls::roof_entity()` 消费；arches/lintels/gates 的 has_roof/Pub28c/Pub284/Pub288/Pub294/Pub295/Pub29a 均为 PublicWallState(+0x2a0 raw 投影) 字段。
- 数据面尺寸（原字节绑定，restorer 不可改）：136B WallPathSegmentationMask（storage 记录）→ 104B MaterializedWallSegment（materializer 输出）→ 96B WallBrickInstance/BrickTransformSsbo（主墙/拱实例）→ 64B WoodenPlankSsbo/Beam ssbo（檐梁/栅栏）→ ConstructGateCmd **120B**（+8B Bevy EventId = 128B 元素）→ RoofEditEvent 64B（roof 域）。WallRow 32B、MyBrick 40B、FinalizedWallHoles 20B。
- **construct_walls 的调用关系铁律**：不 CALL construct_arches/construct_lintels（二者是独立墙事件消费者）；construct_walls 实调 add_wall_corners/add_skirts/stairs_signifiers。restorer 若按"从 construct_walls 内部调用"组织代码 = 机器码不符（building-structures findings 首条）。
- SystemParam 面：gates 9 参数顺序见 §2.5；arches/lintels 外层 EventReader+HashMap+GetMut 是真实 Bevy 接线（G05 报告明确"外层 ECS 未在原码证明中执行"——**验收边界**：restorer 交付 ECS 层后 reviewer 按 fresh decomp 逐参数面核对）。

## 4. 偷懒/缺失检测清单（SOURCE_MAP vs 交付）

1. 97 业务函数必须**逐一对齐**；比对方式 = groupC decomp 命名 vs crate 内符号（可脚本化：`_groupC_inventory` 风格清单待 restorer 交付后生成）。
2. 以下"既有恢复"禁止当作已闭合而跳过原码核对：reconstruction/wall/height_rules.rs（仅 clamp 层，init_wall_height_and_arch_segment_height 0x14078b550 的 arch_segment_height 初始化必须另核对）；reconstruction/building/{fences,gates,wall-structures 等 probe 模块}——这些是 **reconstruction 工作区的既有 probe**，不是 tiny-glade/crates/systems 交付；复用其数值可以，但函数签名/参数面必须以 fresh decomp 为准。
3. 重点水分区：**半木构（2.8）与高程支撑（2.7）**无 building-* 冻结差分——不允许只照 fresh decomp 翻译一遍就标 O；至少对 half_timbering 141218e30、construct_rectangle_pillars 14124fcb0、circle_turret 14124cc30 做原码差分（可仿 G05 的 verify_*.py 模式）。
4. construct_arches 的 **shape2 分支（§2.2 末行）与 C 版 wooden_arch_frame** 是明确未冻结缺口，交付时必须在 REVIEW 续写节登记"未覆盖/边界"，不得并入"已闭合"。
5. rhapsody 依赖（0x141fdca60 create_buffer 等）按 DEPENDENCY_FILTERING 筛除；gpu_* 三函数（§2.8 末行）为 SSBO 直写，接 render 边界后置，但函数体仍需恢复（机器码 0x140773c00/0x140774010 已反编译）。

## 5. 判定汇总（wall-constructor）

- **当前状态：in_progress（restorer-C 已产出 decomp 批与 target-status，实现未落盘）→ 审查挂起，待交付。**
- 已固化的审查基准：上述 97 项矩阵 + 数值清单（G05/G06 冻结报告数值为准）+ 缺失 decomp 6 项（§1）。
- 交付后 reviewer 续写：逐函数判定表、BLOCKER/MAJOR/MINOR 问题清单、与 roof 的 64B RoofEditEvent/88B 参数面跨 crate 一致性核对。
- 对 main 的即时要求：**督促 restorer-C 优先补齐 §1 的 6 个缺失 decomp，并在交付时附"SOURCE_MAP 97 项对齐表"**。

---

## 6. 复验轮 2（2026-10-16，restorer-C 交付后）— 判定：**不合格（fix_pending）**

交付位置：`D:/game/tiny-glade/crates/systems/wall-constructor/`（55 rs / 1554 行）。编译零错误 ✓，但 **97 业务中绝大多数为桩/壳**，机器码体量（8–70KB/函数）与实现行数（均值 ~16 行）严重不匹配。本审查者逐函数分类（脚本 + 人工读全部关键文件）：

### 6.1 分类统计（97 业务）
| 类别 | 数量 | 说明 |
|---|---|---|
| REAL（实质实现） | ~5 | create_stone_arch_profile（0x1407645f0）、circle_categorize（0x14124af60，coordinator 示范 ✓）、arch_walker::walk/new、calculate_wall_path_segmentation 主入口（消费事件→冻结链→掩码写回） |
| 冻结链 re-export（合法复用） | ~10 | split_into_random_rows/trim_rows/above_to_below/below_to_above/resolve_hole_overlap/perturb_row_ends/split_horizontally（country-core recovered_building::wall_constructor）——**但 ECS 侧不消费**（见 6.2） |
| STUB（空体/let _ 丢弃） | **29** | half_timber 族 6、plaster 族 5、elevation mod 5、ui_arch/ui_lintel 3、segmentation process_segment/add_splits_in_empty_range/score_split_candidate 3、gpu_* 3、wall_corners/wall_skirts/stairs_signifiers 3、playermade pillars_visual 1、find_stone_column_start_y 1 |
| THIN 壳（输出丢弃/几何近伪） | **35** | gates 2、lintels 1（`let _ = (projection, mask, ssbo)` 全空）、fences 2、floors 1、elevation 单函数 9、playermade 5、curved_halftimber 2、from_curve 壳、place_pillars 2、其余 |
| 已接线但输出丢弃 | 3 | construct_walls（`let _records` + helpers 传空切片）、construct_arches（`let _ = construct_regular_wall_arches`，96B 输出与 gate_commands 全丢）、construct_gates（伪造门宽检查 + spawn 空实体） |

### 6.2 关键问题（FIX 回 restorer-C，按严重度）
1. **W-B1【BLOCKER】冻结链在场不接线**：country-core `recovered_building` 已有 `wall_structures::construct_lintel_supports`（703 行）、`fences::construct_fences_cpu`（353 行）、`gates::construct_wooden_gate_doors`（538 行）、`wall_arches::construct_regular_wall_arches`（227 行，被调但输出被 `let _` 丢弃）、`wall_platform_*` 族——本 crate 的系统（construct_lintels/construct_fences/construct_gates/construct_arches/construct_platform_stairs）**全部未消费这些冻结链**。G05/G06 冻结验收（178 例/3104 条 64B、28 原返回/1886 条 96B、130 例/4159 条）因此在本 crate 内零体现。
2. **W-B2【BLOCKER】construct_arches 输出全丢**：`let _ = construct_regular_wall_arches(input, &mut arch, &mut third);` —— WallArchesOutput（ordered 96B records + ConstructGateCmd120 + segment_count）未写 SSBO、未发 EventWriter<ConstructGateCmd>、third 数组未写回墙缓存、闭合轮廓 first→last 复制未做（G05 caller 语义）。shape2/MaterialStats 直写分支为占位（登记为未覆盖 ✓，但不允许以"已恢复"表述）。
3. **W-B3【BLOCKER】half_timber 族 6 函数全桩**（0x141218e30/141217be0/1412218a0/141221da0/141221f70/141fd1c30）：26.7KB asm 手解证据在 `groupC/wall-constructor/decomp/half_timbering-asm.txt`，实现为空体/恒 false/恒 0。签名面也与机器码不符（原 11 参数）。
4. **W-B4【BLOCKER】plaster 族 5 函数全桩**（0x14075b810/75d460/75dc60/75e9e0/1412269b0）：maintain/create/split/generate/register 全空；generate_plaster_mesh_and_entities 71.6KB decomp。
5. **W-B5【BLOCKER】elevation supports 大面积壳**：construct_circle_supports/construct_rectangle_columns/construct_rectangle_brackets/detect_overhang_support_segments/overhang_subscribe_to_tabloids 空桩；circle_pillars（0x14124ba30，1016 行 decomp）实现为**均匀圆环布点**（几乎必然与机器码不符）；find_overhang_support_segments（43KB decomp）2 行；rectangle_large_brackets（56KB）6 行。circle_categorize 是正确的示范——**其余 9 个须按同一标准补**。
6. **W-M1【MAJOR】gates 伪实现**：construct_gates 用 `curve.length <= 2.7 continue` 冒充门宽判断（机器码：段几何宽>2.7 双门扇、==2.7 单门，间距 min(.05/全长,.1)）；spawn 空实体而非 136B WoodenGate payload；frozen chain 未调。update_gates_hashmap 无 despawn/6 组合语义（G06 冻结契约未实现）。
7. **W-M2【MAJOR】fences 记录残缺**：emit_fences/process_fence_segment 计算 rng_seed/spacing 后 `let _` 丢弃；64B plank 记录仅写 position（0..12B），affine/lean/rail/seed/source/WallId 缺；fence_segments_count/seed 未累计。
8. **W-M3【MAJOR】from_curve_resolved 返回空 Vec**：rows/trimmed/normalized 全算后丢弃；`consume_wall_proposal`/`construct_resolved_plain_wall_proposal`（README 声明的安全闭包）未调。construct_walls 侧 `let _records` 同样丢弃。
9. **W-M4【MAJOR】segmentation 叶函数空桩**：process_segment（0x140750420）/add_splits_in_empty_range（0x140751240）/score_split_candidate（0x140751610）空；sample_wall_path_intersections 5 行壳。
10. **W-M5【MAJOR】curved_halftimber / init_wall_height / place_pillars / playermade 壳**：wooden_floor_borders 6 行、curved_halftimber_maintain_entities 9 行、init_wall_height_and_arch_segment_height 13 行（机器码 0x14078b550 高度/拱段初始化未核对）、place_pillars 24 行（0x140753670 51KB）、playermade brackets/pillars_visual/magic_support 2–5 行。

### 6.3 判定
- **wall-constructor 复验：不合格（fix_pending）**。"97/97 交付 + 编译零错误"仅为 crate 骨架成立；按 SOURCE_MAP 逐函数标准，实质业务恢复约 5/97，桩 29、壳 35、未接线冻结链 ~10。**不得进入 review_passed**。
- 修复路径（建议 restorer-C 轮 2 优先级）：① 把 construct_lintels/construct_fences/construct_gates/construct_arches/construct_platform_stairs 接到 country-core 冻结链并真实写出输出（SSBO/事件）；② half_timber 按 half_timbering-asm.txt 手解补全（或明确降级登记为"未恢复"而非桩）；③ plaster 5 函数；④ elevation 9 函数按 circle_categorize 标准；⑤ segmentation 3 叶 + 采样；⑥ corners/skirts/signifiers/place_pillars/playermade/curved_halftimber。
- 已正确登记的边界（保留）：shape2 分支 + C wooden_arch_frame（construct_arches 注释）、gpu_* 三函数渲染边界、spires/SSBO 上传、gate spawn payload 渲染侧。
- 复验后 reviewer 逐函数续填判定表；当前全部为 PENDING（桩/壳/未核对）。

---

## 7. 终验轮 3（2026-10-16，restorer-C 轮 2 修复后）— 判定：**继续 fix_pending（3 个子系统可先行验收）**

cargo check -p system-wall-constructor 零错误 ✓（9 warning，本审查者实跑）。RESTORE.md 轮 2 节逐项核实：

| 项 | 状态 | 核实结果 |
|---|---|---|
| W-B1 lintels | ✓ 已接线 | construct_lintel_supports 实调（WallSegmentSource 由掩码+prepared 数组装配），64B plank → output.plank_instances；`closed:false` 登记待 range 接线 |
| W-B1 fences | ✓ 已接线 | construct_fences_cpu 实调，64B 落盘；统计（posts/counts/seeds）计算后丢弃（MaterialStats 边界，登记可接受）；split_curve_based_on_angle 委托冻结 split_fence_angles |
| W-B1 gates | **✗ 未完成** | construct_gates 仍 `curve.length <= 2.7 continue` 伪门宽判断 + spawn_empty；**construct_wooden_gate_doors（country-core gates.rs:538，G06 冻结）仍未调用**；`let _ = (payload_bytes, meshes, terrain, ssbo, output)`。payload 仅解前 16B。轮 2 声称"gates → 门实体"名不副实 |
| W-B2 arches 输出 | ✓ 已落盘 | 96B brick_instances → WallConstructorOutput ✓；ConstructGateCmd120 事件写出 ✓（payload 仅前 16B wall+ordinal，104B profile 投影字节登记 I 级）；third 数组仍无 caller 侧累计/闭合复制（登记） |
| W-B3 half_timber | ⚠️ 部分 | **前导结构经本审查者 asm 复核成立**：half_timbering-asm.txt 证实 call 0x140b2a600（try_get_wall_shape，空→return）、call 0x1408e33e0（gable_sides）、`[rbp+0x20]=0x3d` + `cmp [rdi+0x18],0x3d`（61 字符名长校验）✓；overlapping_windows/get_window_aabb/check_for_overlap 为朴素 AABB（I 级未核对 8KB decomp）；**中段 plank 坐标 ~0x6860B 显式未恢复登记** ✓（诚实） |
| W-B4 plaster | ⚠️ 结构级 | 实体生命周期（Create/Delete/Recalc 命令）真实；split_wall_into_sides 3 行猜测体（未核对 23KB decomp）；generate_plaster_mesh_and_entities 仅返回计数（71.6KB 顶点坐标显式未恢复登记）；register_required_components 空 |
| W-B5 circle_pillars | **✗ 数值未证实** | 结构（halftimbered 门控 + contains_point）在；但**环形步进（τ/count 均匀分割）与 0.2 内缩偏移在 decomp 无依据**：本审查者 grep 0x14124ba30 decomp 无 0x3f99999a(1.2)/0x3e4ccccd(0.2)；注释引用的 `_UNK_142a7cd48` rne-read 实测 = **0x40e00000 = 7.0**（非 1.2）。机器码步进为砖长驱动（含 7.0 常量），均匀分割实现大概率不符——**须按 circle_categorize 三方核对标准重做或继续登记为未恢复** |
| from_curve | 登记合规 | row→proposal 中段显式未恢复（README 边界一致） |
| gpu×3/debug | 登记合规 | 壳登记 + "不称已恢复" 注释 ✓ |

**结论**：轮 2 实质进步（lintels/fences/arches 三个子系统的冻链接线 + 输出落盘真实，可先行验收）；但 **W-B1 gates 未完成、W-B5 circle_pillars 步进数值未证实（且常量引用错误）、half_timber 中段/plaster 网格/segmentation 叶/corners/skirts/signifiers/playermade 仍为未恢复或未核对**。按 DISPATCH 验收门槛（"review_passed 前不算完成；不得把编译通过/局部测试当整模块完成"），wall-constructor **不得有条件通过**，维持 fix_pending。建议：gates 接入 construct_wooden_gate_doors；circle_pillars 按 0x14124ba30 三方核对（纠正 _UNK_142a7cd48 引用）；其余未恢复项保持显式登记直至逐项补完。

---

## 8. 终验轮 4（2026-10-16，restorer-C 轮 3 修复后）— **继续 fix_pending（4 条主链已闭环，可先行子系统验收）**

逐项核实：
- **gates W-B1 完成 ✓**：construct_gates 现调 `construct_gate_cpu`（G06 冻结链：negative-Y ratio 选择 + 内柱），96B 支撑砖 → WallConstructorOutput；门扇 136B spawn payload 按 G06 findings 布局实写（scale/top_positions@80/WallId@104/endpoint_dirs@112/seed@128/public_29a@132；mesh/shader 句柄为渲染边界登记）；ConstructGateCmd payload 全解（wall@0/ordinal@8/point_count@16/段端点@20..120）。段源（简化曲线代 prepared）为 I 级登记。
- **circle_pillars W-B5 重做 ✓（机器码一致）**：本审查者复核——count=round(lerp(4,7,clamp01(ratio)))，`_UNK_142a7cd48`=7.0f、`_UNK_142934694`=4.0f 与本人 rne-read 实测一致；t=i/count 角向步进；terrain+max(0,ratio)<=wall_height(+0x280) 门控；wall+0x298 第二变体登记；start_y + smoothstep((h−y+0.8)/5→·1.5) 高度族。旧均匀分割实现已弃（注释如实记录 reviewer 裁决）。剩余细节（shape_point_at 语义/偏移族）为登记复核项。
- **半木构/plaster/segmentation 叶/corners/skirts/signifiers/playermade/platform**：保持结构级+显式未恢复登记（合规）。
- **空体标注 ✓**：自动化扫描 0 个无标注空 pub fn；8 处文件带"未恢复（壳登记）"标注。

**结论**：四条主链（arches/lintels/fences/gates）冻链接线+输出落盘全部闭环，可先行子系统验收；但按 DISPATCH 门槛（"不得把编译通过/局部测试当整模块完成"）与 13 子模块验收面（阶梯/半木构/石膏/角裙边/指示符等仍为未恢复或未核对），wall-constructor **整体维持 fix_pending**。后续轮次按剩余清单推进：half_timber 中段、plaster 网格坐标、elevation 其余 8 函数（circle_categorize/circle_pillars 为标准）、segmentation 3 叶、corners/skirts/signifiers、playermade/platform stairs、from_curve row→proposal 桥。
