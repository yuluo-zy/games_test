# wall-constructor 还原报告（restorer-interaction-wallconstructor）

日期：2026-10-17。范围：`crates/systems/wall-constructor`（package `system-wall-constructor`）。
标准：逐函数 fresh 反编译 → 重建原函数完整控制流图 → Rust body 逐语句判等；近似体一律不通过。

## 本次成果

- **重建（2）**：`utils::perturb_row_ends`（0x141240d30）、`utils::split_horizontally`（0x141240ec0）。
  两函数均以原发行 RNE 机器码逐指令重建（radare2 raw disasm + Ghidra/ReVa 定点反编译 +
  常量表读取），写入 `src/utils/mod.rs`，`cargo check -p system-wall-constructor` 通过。
- **判等（已恢复复用，3 组）**：`resolve_hole_overlap`（0x1412411c0）、`trim_rows`/
  `above_to_below`/`below_to_above`（0x14123eef0/0x14123fab0/0x14123fb70）、
  `split_into_random_rows`（0x14123e110）——直接复用 country-core
  `recovered_building::wall_constructor`，G04 冻结证据（320/200/160 例逐字节）成立。
- **卡点攻克（1）**：`find_stone_column_start_y`（0x141250720）Ghidra jumptable 截断已用
  radare2 解码 `.rdata` 跳表（0x142c20af8）与控制流，见下。
- **未实现（约 60）**：清单见下，逐项列分歧点。crate 内现有大量"按证据标注的近似体"
  （`circle_categorize`、`add_wall_corners`/`add_skirts`/`stairs_signifiers_new`、ECS 外壳、
  支撑梁 4 函数、construct_platform_stairs 等）经 fresh 反编译确认**不是**原二进制真实逻辑，
  全部降级为未实现登记，不得以近似通过。

## 复核（reviewer FIX 5 项）处理记录

对照 REVIEW.md 逐项处理：
1. **find_stone_column_start_y 跳表**：以 .rdata 字节（0x142c20af8..0x142c20b3c）逐项重算
   并 ASCII 列交叉核对，更正为 续={0x00,0x06,0x07,0x08,0x09,0x0A,0x0B,0x0E}、
   常量={0x02→0x92d、0x0D→0x937}、返回0={0x01}、panic={0x03,0x04,0x05,0x0C,0x0F,0x10}。
   本会话字节证据与 reviewer 列出的具体 code 集（续含 0x12/0x15、常量含 0x14）不一致；
   0x12 在 jumptable 之外（idx18=float 数据），不可能为合法跳转目标，故以字节为准。
2. **支撑梁 4 函数 + construct_platform_stairs**：确认 crate 版本为推零壳/近似，全部降级
   ❌ 未实现（propose cost=len*(‖Δdir‖+.5)/flags[0xf]/3D 方向、place_support_beams 双
   Grower、add_diagonal_support 轴参数、emit_support_beams rdp、platform_stairs 链均未实现）。
3. **DoorMeshInProgress::add_quad**：crate 无符号；country-core 4 数组 ≠ 原 3 数组，
   改标"G06 复用覆盖"（覆盖不完全，差异登记待门族验收）。
4. **debug_force_update_arches**：统一为 1B ret 存根判等（B 中无真实业务），lib.rs 注释已改。
5. **resolve_hole_overlap case0 横切 split_x**：fresh 反编译（0x1412411c0 line 110-111/135）
   显示原版横切**以 split_x==0 为门**（cStack_193=MyBrick@0x25），country-core `b.split_x==0`
   与之匹配，reviewer 所述分歧未获字节证据确认；仍登记待补横切路径定向冻结用例。

## 逐函数清单（按 ANALYSIS §7/§12 全符号）

标注：✅判等 ｜ 🟡重建（fresh 反编译/机器码差分对应，逐语句待复核）｜ 🔵glue-legitimate ｜ ❌未实现(分歧点)

### 主干
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `WallConstructor::from_curve` | 0x140769760 | ❌ | crate 仅有 `from_curve_resolved` 安全闭包；README G04 明确 row→proposal 中段（相邻行匹配/横向 partition/noise/seed 来源）未恢复，不称完整 |
| `construct_walls` | 0x140757800 | ❌ | 外壳参数面与实调 add_wall_corners/stairs_signifiers 属实，但从 curve 到 96B 砖的装配链未逐语句对应；closure$1..7 未定点 |
| `init_wall_height_and_arch_segment_height` | 0x14078b550 | ❌ | 两对 {count,height} 权威语义、-0x48 计数与 core wall_height_authority 对齐未闭合 |
| `debug_force_update_arches` | 0x14003bb80 | ✅ | B 中该符号为 1B `ret` 存根；crate 空函数体与之判等 |
| `gpu_clear_wall_gate_pieces` | 0x140773c00 | ❌ | SSBO 直写体待 render 边界接线（后置） |
| `gpu_flush_plank_buffers` | 0x140774010 | ❌ | 同上，后置 |

### ArchWalker
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `ArchWalker::new` | 0x14074c2d0 | 🟡 | 16B 结构布局确认 {+0x00 u32、+0x04 f32、+0x08 brick_len、+0x0c=0.1}；但参数名/顺序未定（真实 [0]=in_R8D、[1]=in_XMM3、[2]=in_XMM1），crate `new(kind,extra,brick_len)` 映射待与调用点核对 |
| `ArchWalker::walk<arches closure$3>` | 0x141232bb0 | ❌ | **crate `walk()` 为近似体**：真实循环推进 `travelled=(travelled−step).max(0)`、u 经 wall_space subtract+project_clamp 周期投影、while `step<travelled+dist`、尾砖 `step*0.1<fVar5`；crate 用 `travelled=step*emitted` + normalize 直插值、while `step>=travelled+seg*t`、尾砖 `step*0.1<total−travelled`。fresh decomp /tmp/wc-dec/0x141232bb0.c（171 行） |
| `ArchWalker::walk<lintels closure_env$0>` | 0x141233380 | ❌ | 同上分歧；64B WoodenPlankSsbo 端点 -0.225Y 细节未逐语句 |
| `ArchWalker::walk<brackets closure$7>` | 0x141231ea0 | ❌ | 同上分歧 |
| `ArchWalker::walk<arches_n_walls closure$6<false>>` | 0x14048ab50 | ❌ | 同上分歧 |

### 分段
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `calculate_path_wall_segmentation` | 0x14074f310 | ❌ | crate 外壳接入 country-core wall_segmentation 冻结链，但 `process_segment`/`add_splits_in_empty_range`/`score_split_candidate` 为近似体（见下三行），非逐语句 |
| `calc_path_wall_segmentation` | 0x14074f7a0 | ❌ | 同上；prepared/path/shape/special 装配未逐语句 |
| `process_segment` | 0x140750420 | ❌ | crate 版本仅按 flag≠0 推 [0,0,1,0.5]，真实逻辑含 score_split_candidate>0.4 选最高|1 + add_splits 锚点 + 首尾坐标驱动长度 |
| `add_splits_in_empty_range` | 0x140751240 | ❌ | crate 版本 `ceil(len/1.5)` 近似；真实为 floor(len/span)、count==1 且 len<1.5span→0、fraction>0.5 选下槽、flags bit8 |
| `score_split_candidate` | 0x140751610 | ❌ | crate 版本 `(mid-0.5).abs()` 近似；真实为左右 2 单位弦 + 投影钳制 + `(dist_XZ+chordY-currY)*0.5 -0.4/clamp0` |
| `calculate_wall_path_segments_impl` | 0x14122ff90 | 🟡 | 136B→104B materializer，country-core 冻结链复用 |
| `sample_wall_path_intersections` | 0x14074c720 | ❌ | 四 512² 栅格/衰减/阈值 crossing 细节未逐语句 |
| `split_wall_path_segment_into_platform_stairs` | 0x140751960 | ❌ | threshold_range 阈值/闭开、closure7/8、与 split_platform_wall_masks 关系未闭合 |

### 拱 / 过梁
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `construct_arches` | 0x14075f680 | ❌ | 外壳接入 wall_arches 冻结链；shape2 分支未冻结；prepared 源（path-prep）以简化曲线代（I 级登记） |
| `create_stone_arch_profile` | 0x1407645f0 | ❌ | crate 版本 trim/重采样近似；真实含 regular cubic 中心加权/`clamp(h/Pub284)*.5`/lower=max(highest-3.5,ground)-1-groundY 钳0/min 全局拱高、freeform trim .1875/cap .95/slice |
| `ArchFunction::remap_t` | 0x140765300 | ❌ | crate 版本 `1-(1-u)^2` 近似；真实为 `if(*rcx!=0){ inverse_lerp() }`，枚举取值未定 |
| `construct_lintels` | 0x14076d9f0 | ❌ | 外壳接入 lintel_support_instances；prepare_lintel_support/third 回写/accumulate 未逐语句 |
| `place_support_beams<closure_env$2>` | 0x141242530 | ❌ | **crate 版本为零壳**（遍历 proposals 推 [0;64]）；真实双 Grower 0/end 向中 propose、cost 小者 consume、len≥1.0 发射/≤3.15/join、分开各 len*0.85 对角、join 且 total≥1.4175 双侧 0.5 均未实现 |
| `add_diagonal_support` | 0x141243740 | ❌ | **crate 版本为零壳**（推 [0;64]）；真实 extend_xz/锚高 sample_height−0.2/方向点差(Δy−0.2)/轴 side×0.2/dir×(len−0.2)/third×0.2 未实现 |
| `SupportBeamBuilder::emit_support_beams` | 0x141243c20 | ❌ | **crate 版本为近似壳**（walker.walk 直推 [0;64]）；真实 rdp_support_indices→索引对→walk_lintel 未实现 |
| `SupportBeamGrower::propose` | 0x141fe09c0 | ❌ | **crate 版本为近似体**（仅装配 proposal 结构、curvature_cost=0）；真实 cost=len*(‖Δdir‖+.5)、flags[next]&0xf 停、已有方向投影、3D 方向均未实现 |
| `SupportBeamGrower::new` | 内联 | 🔵 | 内联胶水（Grower::init 复用） |

### 立面支撑 / place_pillars
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `detect_overhang_support_segments` | 0x1407867d0 | ❌ | fVar4 判据、ElevationSupports 字段、DirtyTiles AABB 过滤未闭合 |
| `overhang_subscribe_to_tabloids` | 0x140786440 | 🔵 | 8×RaycastWorld+1×tabloid 纯订阅胶水 |
| `construct_circle_supports` | 0x140788160 | ❌ | 外壳 is_circle 分流；bottom_layer/indexmap bucket/circle_pillars 装配未逐语句 |
| `construct_rectangle_columns` | 0x140788a80 | ❌ | 同上 |
| `construct_rectangle_brackets` | 0x1407896b0 | ❌ | 墙高(0x2a8-0x1c)≥1.6 判据以外装配未逐语句 |
| `circle_categorize` | 0x14124af60 | ❌ | **crate 现有版本为近似体**（threshold 0.6s+0.3(1-s) 系推断）；真实含 ceil(2r) 网格双层扫、Circle2d::is_point_inside、内部逐 0x50B 墙 get_wall_shape+GladeBorder::is_pos_inside，输出副作用未定 |
| `construct_circle_bottom_layer` | 0x14124b2c0 | ❌ | **crate 版本用 TAU 均分近似**；真实为 halftimber 早退 + count=ceil(perimeter/0.69) + random_splits_into + 逐对 get_pos_at_u |
| `add_bottom_brick` | 0x141fe1de0 | ❌ | **crate 版本只写 position**；真实含 fastrand f32 抖动 + push + dirty |
| `construct_circle_pillars` | 0x14124ba30 | ❌ | 0x298 守卫/count=round(inverse_lerp→t*7+(1-t)*4)/terrain+0.05≤墙高/raycast/smoothstep/-0.4 外移/rng 后 halftimber 二路未逐语句 |
| `construct_turret` | 0x14124cc30 | ❌ | 层数/收分率未定 |
| `construct_rect_bottom_layer` | 0x14124eb80 | ❌ | halftimber→四角 plank vs 5 段边/count=ceil(len/0.69)/jitter=0.32/len 未逐语句 |
| `construct_rectangle_pillars` | 0x14124fcb0 | ❌ | 4 角+3 raycast 净空≥2.3/smoothstep/二路未逐语句 |
| `find_stone_column_start_y` | 0x141250720 | ❌ | **jumptable 已攻克并复核**：0x142c20af8 跳表 17 项（0x00-0x10）逐字节重算（.rdata 0x142c20af8..0x142c20b3c，ASCII 列交叉核对；0x11 在 switch 前单独判 hit）——续(0x14125082e)={0x00,0x06,0x07,0x08,0x09,0x0A,0x0B,0x0E}；常量={0x02→0x92d(byte[0x91]&1?−0.2:−1.0)、0x0D→0x937(−0.2)}；返回0(0x941)={0x01}；panic(0x94f)={0x03,0x04,0x05,0x0C,0x0F,0x10}；≥0x12 越界。命中路径 4 点 min+clamp 1000（0x447a0000）。未实现残留：7 参 ABI、helper 0x141246fb0/0x1408ccc50/0x140c9cde0 语义、0.7 缩放细节 |
| `construct_rectangle_large_brackets` | 0x141253fc0 | ❌ | 托架几何参数未定 |
| `construct_rectangle_small_brackets` | 0x141250da0 | ❌ | 同上 |
| `find_overhang_support_segments` | 0x141248620 | ❌ | 输出结构/阈值/多 closure 未定 |
| `OverhangSupportFinderCache::new` | 0x141248300 | ❌ | 两 Vec 装配；消费方未定 |
| `maintain_stone_column_colliders` | 0x14078a6e0 | ❌ | 依赖 core rapier 后端未闭合 |
| `construct_wooden_pillar_w_stone_base` | 0x141fe2150 | ❌ | 高度钳1.1/len>1.25 石基/0.3 斜撑/0x18B 记录二路未逐语句 |
| `construct_brick_columns` | 0x141fe3240 | ❌ | FrozenMap(0x3a)→writer→0x40B 表未逐语句 |
| `construct_crossbrick_pillar` | 0x142418f50 | ❌ | count=ceil((top-bottom)/(size*0.4))/random_splits/四边+xz_rotation_bad 未逐语句 |
| `place_pillars` | 0x140753670 | ❌ | 0x88B 提案字段语义/LoopRange+CurveSlice 采样/assemble_stone_pillar 分流未逐语句 |
| `PillarProposals::add_proposal` | 0x1407534d0 | ❌ | **crate 现有 `add_proposal(i32,f32)` 近似**；真实为三向量（proposal 0x1cB/scores/keys）+ 同 key NaN-aware min 替换末提案 |
| `assemble_stone_pillar` | 0x14120d670 | ❌ | **crate 现有版本为近似体**；真实含 1.26 分段/0.56 层/random_splits/0.42/0.7 jitter/2.1 盖砖/closure$4 发 96B |
| `place_pillars::closure$4` | 0x14120d340 | ❌ | Affine3Packed+push+计数+dirty，未在 crate 实现 |

### 门 / 栅栏 / 地板 / 楼梯
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `construct_gates`（systems.rs） | 0x140756370 | ❌ | 外壳 G06 冻结接线；is_sewer_gate/construct_sewer_bars 分支未实现 |
| `update_gates_hashmap` | 0x140755690 | ❌ | 六组合 despawn 生命周期未逐语句 |
| `construct_wooden_gate_doors` | 0x141210110 | ❌ | crate 无实现；curve 切片/两叶/内部柱/header inset 未恢复（country-core gates.rs 有候选需接线） |
| `DoorMeshInProgress::add_quad` | 0x141214ed0 | 🔵 | **G06 复用覆盖**：crate 无独立符号；country-core recovered_building::gates 有 add_quad（6 顶点 0,1,2,0,2,3 并行写），但**原实现为 3 数组**（positions/colors/uvs，stride 0xc/0xc/8）而 country-core 为 4 数组——覆盖不完全，差异登记待门族验收 |
| `construct_fences` | 0x140765830 | ❌ | 外壳；RNG seed=WallId inner+begin+end wrap64 之外装配未逐语句 |
| `emit_fences` | 0x1412388f0 | ❌ | 最大非正[begin,end)/墙高/Aabb2/首尾桩跳过未逐语句 |
| `process_segment`（fences） | 0x141238d80 | ❌ | perchance 派生/spacing=smoothstep(.7)/crashy_resample/64B plank+post(-0.1Y) 未逐语句 |
| `split_curve_based_on_angle` | 0x141fdf9e0 | ❌ | 40° 阈值/水平 dot/重复点断言未逐语句 |
| `construct_floors` | 0x1407661e0 | ❌ | Floor 实体 HashMap diff/log2f(h*3) 缩放/StoneFloorInstanceData 未逐语句 |
| `construct_platform_stairs` | 0x1407744d0 | ❌ | **crate 版本为零壳**（遍历 storage 不产输出）；真实 80B 装饰源→blockers/cells/碰撞/96B 砖/holes/stats 链（country-core wall_platform_stairs 冻结链）未接线 |
| `construct_playermade_stairs_arches_n_walls<true>` | 0x140496800 | ❌ | <true>/<false> 翻转、closure 绑定、ArchWalker+from_curve+raycast+StairsPillarProposals 装配未逐语句 |
| `preprocess_curve` | 0x140780e20 | ❌ | slice_into/try_new_from_points/密度 min(2/len,.45)/try_resample 未逐语句 |
| `construct_playermade_stair_brackets` | 0x1407834f0 | ❌ | Rng::f32×18+DecoratorExtraInfo::try_as_stair+random_orthogonal_quat 未逐语句 |
| `playermade_stair_floating_bricks` | 0x14077a2f0 | ❌ | MagicSupports::clear/Rng×15/EasingFunction::apply 未逐语句 |
| `construct_playermade_stair_pillars_visuals` | 0x14077d520 | ❌ | StairsState::graph/StairSegment/Rectangle2d/WallConstructor::from_curve+random_splits 未逐语句 |
| `process_playermade_stair_pillars_proposals` | 宿主 0x140178a70 | ❌ | **已知卡点**：宿主=bevy impl$3::call_mut 系统包装；6 参 SystemParam 已定，内层业务块（提案→raycast→碰撞注册→rt 事件）未定点 |

### 半木 / 抹灰
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `curved_halftimber_maintain_entities` | 0x1407706d0 | ❌ | 实例字段语义、@0xa0 字节枚举、HashMap<WallId,Vec<Entity>> 生命周期未逐语句 |
| `wooden_floor_borders` | 0x141244310 | ❌ | 边框宽高常量、new_curved_xz/new_straight、矩形 fmodf 周期未逐语句 |
| `create_plaster` | 0x14075d460 | ❌ | 判重/mesh 实体表/generate_plaster 接线未逐语句 |
| `maintain_plaster` | 0x14075b810 | ❌ | 淡出曲线/时长、deferred buffer 释放未逐语句 |
| `split_wall_into_sides` | 0x14075dc60 | ❌ | WallSide 24B/0.1m 格/psubusw 00020002 下限/roof_flag 枚举未逐语句 |
| `generate_plaster_mesh_and_entities` | 0x1412269b0 | ❌ | mesh 拓扑/UV/buffer 布局/mirror_holes_as_needed 未逐语句 |

### UI 控制
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `ui_arch_controls` | 0x140766e70 | ❌ | 控件表 SIMD 掩码遍历/剖面三角链/UiState+WidgetParams 未逐语句 |
| `do_ui_arch_controls` | 0x140767b70 | ❌ | PublicWalls×5/is_active/CompoundEditBuilder/History 未逐语句 |
| `ui_lintel_controls` | 0x140771a60 | ❌ | 同上 + ProportionalScreenSpace 定位 |

### utils
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `perturb_row_ends` | 0x141240d30 | ✅（本次重建） | RNE 逐指令重建：前向 min(center, 1.0−0.5*(rows[next]+rows[next−1]))×1.99(0x3ffeb852)；后向 min(0.5*(rows[0]+rows[1]), 1.0−center)×1.99；NaN 感知 min(amplitude)·magnitude+原值 |
| `split_horizontally` | 0x141240ec0 | ✅（本次重建） | RNE 逐指令重建：t=split×0.4(0x3ecccccc)+0.3(0x3e99999a)；new=lerp(min,max,t)；lower aabb={min_x,min_y,max_x,new_y} deformation={min,new} seed；upper aabb={min_x,new_y,max_x,max_y} deformation={new,max} seed+1 |
| `resolve_hole_overlap` | 0x1412411c0 | ✅ | 复用 country-core（G04 320 例冻结）。**复核 note**：reviewer 提出 case0 横切分支"原版不查 split_x"——本会话 fresh 反编译（/tmp/wc-dec/0x1412411c0.c line 110-111/135）显示原版横切**确实**以 `cStack_193==0`（=MyBrick@0x25 split_x）为门，country-core `b.split_x==0` 与之匹配；CONDITION_A=`(max_x<=hole.max_x)||(hole.min_x<=min_x)` 非→横切与 country-core `a[2]>hole[2]&&hole[0]>a[0]` 等价。分歧未确认；仍登记待补横切路径定向冻结用例（320 例未覆盖 split_x!=0 横切场景） |
| `check_for_hole_overlap` | 内联 | 🔵 | inlinee 71192，宿主 resolve_hole_overlap |
| `trim_rows` | 0x14123eef0 | ✅ | 复用 country-core（G04 200 例冻结） |
| `above_to_below` | 0x14123fab0 | ✅ | 复用 country-core（同链） |
| `below_to_above` | 0x14123fb70 | ✅ | 复用 country-core（同链） |
| `split_into_random_rows` | 0x14123e110 | ✅ | 复用 country-core（G04 160 例冻结） |
| `stairs_signifiers_new` | 0x141223d10 | ❌ | **crate 现有版本为近似体**（只写首点）；真实含非矩形/矩形两路、terrain 采样 y-terrain<0.5 且 max_y-y≥3.0 判段、SubtractiveBoolean1D 减 hole、ceil(段长*h/0.69)+random_splits 踏板、2 阶循环、max_y−0.79−0.1−0.3 顶高 |
| `add_wall_corners` | 0x141215540 | ❌ | **crate 现有版本为近似体**（首尾两点）；真实含角检测（-0.1736482/0.17364822 阈值即 ±10°）、Triangle 链、随机砖宽 0.92±0.16/0.76、aabb 合并、resolve_hole_overlap、96B 发射 |
| `add_wall_corners::closure$1` | 0x141217a20 | ❌ | 同上未实现 |
| `add_skirts` | 0x1412221f0 | ❌ | **crate 现有版本为近似体**（沿曲线均分）；真实含 max_y/is_rectangle 分支、split_height_into_skirts、ceilf((u1-u0)*h/0.69)+random_splits_into、resolve_hole_overlap、WallSpaceCoord to_world_space、裙阈值 wall_y+3.5 |
| `half_timbering` | 0x141218e30 | ❌ | **已知卡点**：Ghidra/ReVa 空反编译（void() 占位 + ReVa "Unable to find unique hash for varnode"）；radare2 pdg 亦 "No function at this offset"。需唯一 worker 修签名后定点重取 |
| `roof_outline` | 0x141217be0 | ❌ | 非矩形返回/锁 SSBO BufferState/4 轮矩形边采样/roundf 木板数(≥2)/0.4 档/0.25 档系数未逐语句 |
| `overlapping_windows` | 0x1412218a0 | ❌ | **crate 现有版本为近似体**（纯 AABB 相交）；真实含 DecoratorStorage SIMD 掩码遍历、subtype byte 过滤、±0.038/±0.14、ResourceAccessInfo stride 0x14 收集、中心/侧向/扩展三测试 |
| `get_window_aabb` | 0x141221da0 | ❌ | **crate 现有版本为近似体**（from_center_dims）；真实含门类 dst_as_wall+byte+0x46=='9'→gothic 剖面曲线末点算 [x-x0,y0+y,y0,..+0.093]、普通 bbx_ls |
| `check_for_overlap_with_all_windows` | 0x141221f70 | ❌ | get_window_aabb+Aabb2::contains 中心/侧向/扩展三测试未实现 |
| `add_underwindow_pattern` | 0x141fd1c30 | ❌ | 样式检查 (rcx+0x11)&0x1e!=0x16/窗高差∈(0.5,2.0)/7 定位点/fmodf/WallSpaceCoord/圆角≤0.078/0.15 宽未实现 |
| `sine_wave`（BrickRowSine::new/sine_wave_deform/random_f） | 内联 from_curve | ❌ | **已知卡点**：module 401 inlinee 24627/24641/24629，宿主 from_curve（source_line 12/45/82）；无独立编译实例，需从 from_curve 体内按行号提取，未完成 |
| `wrap_around` | 内联 | ❌ | 未实现 |

### 注册 / GPU 胶水
| 函数 | 地址 | 判定 | 说明/分歧点 |
|---|---|---|---|
| `add_wall_constructor_systems` | 注册 | 🔵 | Plugin 注册胶水（原 impl$N 注册顺序见 lib.rs） |
| `WallConstructor::normalize_holes` 等 from_curve 内部 helper | 内联 | 🔵 | country-core recovered_building 已恢复链，复用不重建 |

## Fresh 反编译证据索引（本次会话落盘）

- `/tmp/wc-dec/0x141240d30.c`（perturb_row_ends，Ghidra）+ RNE disasm 0x141240d30..0x141240eb4
- `/tmp/wc-dec/0x141240ec0.c`（split_horizontally，Ghidra）+ RNE disasm 0x141240ec0..0x141241117
- `/tmp/wc-dec/0x141223d10.c`（stairs_signifiers_new，Ghidra 704 行）
- `/tmp/wc-dec/0x141215540.c`（add_wall_corners，Ghidra 976 行）+ closure$1 0x141217a20
- `/tmp/wc-dec/0x1412221f0.c`（add_skirts）
- `/tmp/wc-dec/0x1412218a0.c`（overlapping_windows）、`0x141221da0.c`（get_window_aabb）、
  `0x141221f70.c`（check_for_overlap）、`0x141fd1c30.c`（add_underwindow_pattern）、
  `0x141217be0.c`（roof_outline）
- `/tmp/wc-dec/0x141250720.c`（find_stone_column_start_y，jumptable 截断）+ RNE disasm
  0x141250720..0x141250967 + 跳表 0x142c20af8 96B
- `/tmp/wc-dec/0x14124fcb0.c`（construct_rectangle_pillars）、`0x14124eb80.c`
  （construct_rect_bottom_layer）、`0x14124ba30.c`（construct_circle_pillars）、
  `0x14124cc30.c`（construct_turret）
- 复用既有证据：evidence/building-constructor（G04 冻结）、building-arches/
  building-structures/building-segmentation/building-fences/building-gates/
  building-platform-stairs

## 结论

本轮交付 2 个重建函数（perturb_row_ends/split_horizontally）+ 3 组判等复用 +
1 个卡点攻克（find_stone_column_start_y 跳表解码）。复核发现 crate 既有
`ArchWalker::walk` 与真实逻辑存在结构性分歧（travelled 推进/周期 u/尾砖条件），
已从"重建待复核"降级为未实现并登记分歧点；crate 内现有近似体
（circle_categorize/add_wall_corners/add_skirts/stairs_signifiers_new/
overlapping_windows/get_window_aabb/check_for_overlap/PillarProposals::
add_proposal/assemble_stone_pillar）已逐一改为 `todo!("未实现: …")` 并更新文档，
不得以近似通过。其余约 55 个业务函数如实登记为未实现（含其分歧点）。
`cargo check -p system-wall-constructor` 通过（未被其他组阻断）。
