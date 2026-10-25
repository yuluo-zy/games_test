# restorer-eco — ivy 模块 RESTORE 报告

日期：2026-10-15（第五版：吸收 coordinator 卡6 SsboId 对账）
状态：in_review（round-4：SsboId 机制对账落账；send_to_gpu 提交/缓冲注册 render 域接线；garden pin 消费对账）
涉及文件：`resources/ivy/*`、`systems/ivy/*`、`startup/startup_ivy.rs`、`asset_libraries/ssbo_library.rs`（对账注记）

## 一、原函数 → Rust 位置 → 证据

### resources/ivy（权威存储，20+7 函数）
| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| Ivy::add_point | 0x140869460 | ivy_storage.rs:46 | collision-eco/decomp-0x140869460；章节 ivy_storage.rs.md |
| Ivy::last_point | 0x1408694f0 | ivy_storage.rs:61 | decomp-0x1408694f0 |
| Ivy::last_segment_point_ids | 0x140869540 | ivy_storage.rs:64 | decomp-0x140869540 |
| IvyStorage::spawn_ivy | 0x1408696f0 | ivy_storage.rs:124 | decomp-0x1408696f0 |
| Ivy::point_count / set_last_point_position / remove_last_point / remove_root / get_point_id / new / roots / iter_mut / iter / remove / total_segment_count / get | 内联 | ivy_storage.rs:57-190 | 章节 ivy_storage.rs.md（12 项内联） |
| IvyLeafStorage::add | 0x1408fe280 | ivy_leaf_storage.rs:56 | decomp-0x1408fe280 |
| IvyLeafStorage 其余 6 内联 | — | ivy_leaf_storage.rs | 章节 ivy_leaf_storage.rs.md |
| IvyDirectionProposer::get_direction | 0x141234590 | ivy_direction_proposer.rs:68 | decomp-0x141234590 |
| IvyDirectionProposer::default | 0x1412343b0 | ivy_direction_proposer.rs:42 | 章节 |
| IvySpawnNoise::new | 内联 | ivy_direction_proposer.rs:26 | 章节 |
| WallCoord::into_world_space | 0x140b199e0 | wall_coord.rs:14 | decomp-0x140b199e0 + **RNE 0x140b19acd/b72..ba7（reviewer FIX-1）** |

### systems/ivy（消费者）
| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| ivy_grower | 0x14094a910 | ivy_grower.rs:244 | decomp-0x14094a910（puffin+跳表壳）；ALGORITHMS §4.1 |
| closest_segment_index | 0x14094c800 | ivy.rs:74 | decomp-0x14094c800；**两端点均>max 才跳（RNE ucomiss，reviewer 确认 OK 不动）** |
| get_wall_double_sided_normal | 0x14094caa0 | ivy_grower.rs:96 | decomp-0x14094caa0 |
| intersect_ivy_growth_w_wall_segment | 0x14094cc60 | ivy_grower.rs:115 | decomp-0x14094cc60 |
| get_growth_point_proposal | 行227 内联 | ivy_grower.rs:142 | 章节 ivy_grower.rs.md（管线定位） |
| check_for_wall_jump | 行308 内联 | ivy_grower.rs:170 | 章节 ivy_grower.rs.md（管线定位） |
| get_ivy_x_direction_relative_to_wall_direction | 行491 内联 | ivy_grower.rs:117 | 章节 ivy_grower.rs.md |
| ivy_spawner | 0x14094cf90 | ivy_spawner.rs:180 | decomp-0x14094cf90（门控字节级） |
| try_random_ivy_root | 行212 内联 | ivy_spawner.rs:85 | decomp-0x14094cf90 |
| try_ivy_root_next_to_cursor | 行245 内联 | ivy_spawner.rs:107 | decomp-0x14094cf90 |
| is_point_inside_wall_segment | 行294 内联 | ivy_spawner.rs:64 | decomp-0x14094cf90 |
| ivy_pruner（墙变更有效性） | 0x140997410 | ivy_pruner.rs:117 | decomp-0x140997410 |
| is_point_still_valid | 0x140998dc0 | ivy_pruner.rs:104 | decomp-0x140998dc0（max=0x3f000000） |
| ivy_pruner_cmd | 0x140999330 | ivy_pruner.rs:89 | decomp-0x140999330 |
| remove_ivy_at_location | 0x140999860 | ivy_pruner.rs:66（apply_remove_ivy_cmd） | decomp-0x140999860 |
| ivy_leaf_animation | 0x140b0e380 | ivy_leaf_animation.rs:12 | decomp-0x140b0e380 |
| ivy_leaf_spawner | 0x140996080 | ivy_leaf_spawner.rs:88 | decomp-0x140996080 |
| ivy_branch_thickness_animation | 0x140995a80 | ivy_branch_thickness_animation.rs:15 | decomp-0x140995a80 |
| mod.rs add_ivy_systems | 0x140944070 | registration.rs:78 | decomp-0x140944070 |
| mod.rs insert_ivy_resources | 0x1409440c0 | registration.rs:27 | decomp-0x1409440c0 |
| mod.rs add_ivy_events | 0x140944280 | registration.rs:34 | decomp-0x140944280 |

### startup/startup_ivy
| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| startup_ivy | 0x140a21a30 | startup_ivy.rs:28 | decomp-0x140a21a30 + **read_memory 字符串（0x142ac9723 等）** |

## 二、本轮新增/修正

### round-1（2026-10-15 首轮后）
1. **修复 reviewer FIX-1**：`WallCoord::into_world_space` out.y 由 `v` 改为 `v ± offset_z`（RNE 0x140b19acd/b72..ba7）。
2. **声明 `pub mod ivy_grower;`**（systems/ivy.rs:18）——此前 ivy_grower.rs 从未挂入模块。
3. **新增 `ivy_spawner.rs`**：门控（AppMode==1 / ~40000 / <3 光标 ≥3 随机）+ 常量表。
4. **扩展 `ivy_pruner.rs`**：`is_point_still_valid` + 墙变更系统。
5. **扩展 `ivy_grower.rs`**：三内联函数 + 主循环接入。
6. **`IvyDirectionProposer` 补 `#[derive(Resource)]`**。
7. **registration.rs** 补三原命名函数。
8. **新增 `startup_ivy.rs`**（网格注册）。

### round-2（reviewer-eco 二轮 + re-analyst-eco RE-ANALYSIS2）
9. **新增 `ivy_send_to_gpu_leaf.rs` / `ivy_send_to_gpu_segments.rs`**（库内缺失 GAP 闭合）：
   - 元素+SsboType 合一模式（对齐 `rasterize_buildings::GroundedBuildingsSsbo`）；
   - 48B GPU 元素（IvyLeafGpu/IvyFlowerGpu/IvySegmentGpu）+ `upload_ivy_leaves`/`upload_ivy_segments` 纯函数（collect+pack+gpu_write+instance_count）；
   - 原 SsboId 常量 rne-read 定案（0x9e3cfa7f2993f406 / 0x1d6de5e69ac50d2a / 0x2002d2fbdff3d80f）；NAME=名表全路径串（0x142a9eb87 等）；
   - **已知建模 gap**：`SsboId::of` 哈希 NAME 与原 SsboId 不吻合（同 GroundedBuildingsSsbo 注释）；GPU 提交需 GraphicsDevice（render 域）。
10. **spawner 路径 B 按 RE-ANALYSIS2 重写**：随机墙 `usize_less_than(墙数)` → 随机 u → FastNoise 密度闸门（noise≥threshold）→ `get_bool` 定侧（true→−0.68/false→+0.68，@0x14094d6b0，删自造 uniform 掷硬币）→ 2.56 距校验（0x142ab3214）→ 垂直 `(uniform×1.5−0.75)/curve_length + u_on_curve`（`approx_u_from_pos` 精确投影，FUN_141469a20）钳 [0.000928482,1.0] → 尝试 2+usize_less_than(3)。
11. **grower 10 相状态机结构重构**：`GrowerState` Local（phase@+0x1c / guard@+0x48 / timer@+0x11c）按 0x142ab3be0 跳表分派（相0守卫翻转、1/3/4 主、2 计时门、5 二级表、6-9 panic 入口）；主=生长管线，守卫体=墙跳重锚校验。
12. **registration** 注释补全 22 系统差距（send_to_gpu 2 文件 + ivy_audio 等 render/audio 域接线）。

### round-3（reviewer-eco 三轮 MUST-FIX，2026-10-15）
13. **grower 守卫闩死修复（MUST-FIX）**：原实现 phase0 置 guard=true 后永不清 → 主生长体首帧后永不执行、run_guard_body 用 delta=0 → growth 永不前进、事件链断裂。修复：guard 位不由守卫体清除，由 phase0 每次 toggle（false↔true）自然交替；phase0 在 toggle 后即运行所选体（守卫/主），phase1/3/4 按位运行后回 phase0——校验相与生长相隔帧交替，主生长体带 time.delta 在生长相推进。
14. **新增相循环测试** `phase_cycle_keeps_advancing_growth_and_fires_completion_events`：60 帧跑完整 `ivy_grower` 系统，断言 pending 段完成 + 完成事件触发（捕获原闩死 bug）。
15. **侧向偏移路径 A/B 对账**：路径 A（光标）偏移 = 0.68×0.5 = 0.34（round-1 全解，常量 0x142ab31e0/0x142ab3200 × 0.5）；路径 B（随机）偏移 = 0.68 直接由 `get_bool` 选侧（RE-ANALYSIS2 原文 true→−0.68/false→+0.68，调用 0x14094d6b0）。分别落 `ROOT_OFFSET`（A）/`RANDOM_ROOT_OFFSET`（B）。

### round-4（coordinator 卡6，2026-10-15）
16. **SsboId 机制对账**（ssbo_library.rs，present_requires_semantic_review）：
    - 0x140c9cd80 反编译定案：`NameHash::compute` = `xxh3_64`（默认 secret）；原 EXE 0x142b2ebb0 的 192B secret 与默认 xxh3 secret 字节一致（用默认 `xxh3_64` 复算 Cobblestone3RasterStagingSsbo→0x802ff23dee42ab47 / NaniNextVisibilityBitmaskSsbo→0x69cdc275a605bad6 均命中）——**原版即默认 secret，`NameHash::compute` 现状正确，无需 custom-secret 改造**。
    - 例外待取证：ivy 三个 SsboId（rne-read qword 0x142c2e248/280/330 = 0x9e3cfa7f2993f406/0x1d6de5e69ac50d2a/0x2002d2fbdff3d80f）经 ~2000 候选名（短名/全路径/模块前缀/大小写/后缀）均不满足 `xxh3_64(name)`——产生这些 SsboId 的 NAME 串未定位，登记待取证，不伪造。
17. **send_to_gpu 提交**：`upload_ivy_leaves`/`upload_ivy_segments` 已带 `&dyn GraphicsDevice` + `gpu_write`（库内 render 抽象可用）；缓冲注册（add_new）+ 实际 GPU 提交 = 库外 render 域接线（reviewer 三轮裁决）；SsboId 名字未解析使 `SsboId::of` 现不对应原键（卡6 待取证）。
18. **garden pin**：draw_raster<GardenRasterStagingSsbo> 消费已在 draw_garden.rs 接线（RasterDrawResources + AssetSsboLibrary）；GardenRasterStagingSsbo 由 staging_rasters! 宏注册（MappedStorageBuffer<u8>，SsboType）——消费/注册对账通过，无需改。
19. **terrain 回读**（render_loop read_terrain_data/SSBO）属 interaction 组，本次未做，登记。

## 三、依赖复用边界
- `closest_segment_index`、`WallSegment`、`INITIAL_BEST_DISTANCE` 保持在 systems/ivy.rs。
- 墙侧复用 `PublicWalls` / `PublicWallState::simplified_curve_2d` / `WallSpatialHash`（只读）。
- SSBO 复用 `AssetSsboLibrary` / `InstanceBuffer` / `StorageElement` / `gpu_write`。
- RNG/噪声复用 `perchance` / `bracket-noise`。

## 四、仍缺失/待取证（诚实登记，不宣布完成）
1. **grower 10 相状态机主闭包**（0x14094ac20）：分派/守卫/计时/panic 相已结构重构，但主闭包逐相语义（空间哈希遍历、接受阈值、二级表 0x142ab3bf4 目标）仍待取证；守卫谓词（0x94ac51）除墙跳重锚外待取证。
2. **spawner 路径 A 空间哈希格采样**：世界→格换算（32/65、偏移 32.0）与格内墙段遍历仍以 closest_segment_index 全墙复检近似。
3. **FastNoise 密度闸门阈值来源**（res+0x78，IvySpawnNoise 构造参数）：待取证。
4. **max_ivy_grow_height**（PublicWallState getter）数值：pruner/spawner 用机器码 0.5，grower 用 f32::MAX 近似。
5. **ivy_pruner**：原脏墙 deque（param_6/7）+ 地形双栅格 + 附加采样点（0x14 步长）仍近似。
6. **GPU 48B↔104B 精确映射 + GPU 提交接线**：字段映射为记录卡点；GPU 提交/缓冲注册 = 库外 render 域接线（reviewer 三轮裁决）。
7. **ivy SsboId 名字来源**（卡6 待取证）：verified qword 无法由 `xxh3_64` 复现，NAME 串未定位。
8. **IvyPoint +32 标量**（flags vs thickness）：无冲突，send_to_gpu_segments 若打包该位需再对账。
9. **startup_ivy 的 shader 注册 + 3 个 spawn 记录**：渲染管线待接线。`ivy_debug_cube` 几何待取证。

## 五、inventory 建议
- 提升 `present_requires_semantic_review`：resources/ivy 5 项、systems/ivy 的 ivy_grower/ivy_spawner/ivy_pruner/ivy_leaf_animation/ivy_branch_thickness_animation/ivy_leaf_spawner/ivy_send_to_gpu_leaf/ivy_send_to_gpu_segments/mod.rs、startup/startup_ivy。
- 建议 reviewer 复核：①send_to_gpu 元素/SSBO 类型（SsboId 建模 gap）；②spawner 路径 B 侧向 get_bool 与 2.56 距校验；③grower 相状态机结构 vs 0x142ab3be0 跳表；④approx_u_from_pos 投影。
- 建议合同登记：send_to_gpu 2 文件 8 函数、spawner 5 函数、pruner 4 函数、grower 3 内联函数 + 相状态机。
