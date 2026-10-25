# FAKE-IMPLEMENTATION-LEDGER — country-core 独立反假实现审计

> 审计员: logic-integrity-auditor | 日期: 2026-10-17
> 方法: ghidra `--worker shared` **fresh 反编译**（7 批 65 份 decomp，`planning/logic-auditor/decomp/batch1..7/`）+ Rust body 逐语句对照。
> 红线: 只以 fresh 反编译/机器码为准；不采信既有 REPORT/REVIEW/REBUILD 描述；未修改 src。
> 协议: planning/2026-10-16-logic-verify-protocol.md。

## 审计范围与命中

- 抽审 12 模块、**55 个函数**（优先级高进度模块 40 + 低进度 REBUILD 后 15）。
- **命中「假逻辑-凑数」: 0 个新发现**（即:既有 LOGIC 报告声称「判等/闭合」的函数中，独立复验未发现一例 body 非原逻辑）。
- **推翻/修正既有判定: 1 个「未实现-需重做」被 REBUILD 反证为**误判（music_audio_system photomode 分支），**0 个「判等」被推翻**。
- 抽样证明: high-progress 模块（world_raster/music/garden/session/input/ivy/render_loop/camera）的判等声明**基本真实**；
  真正的大量「结构对上逻辑假」集中在 walls/collision/terrain/history/roof——**这些 LOGIC 报告自己已如实标未实现-需重做**，未发现被漏标或冒充判等。

## 函数表（按模块）

| 模块 | VA | 函数 | 声称判定 | 审计 verdict | 分歧点（fresh 证据） |
|---|---|---|---|---|---|
| world_raster | 0x140a15da0 | clear_all | 判等 | **真实现** | composite/scratch memset、dirty 0xff@+0x98、staging_dirty=1、tile_hashes 0x07954dbb 逐条符 |
| world_raster | 0x140a172d0 | copy_to_staging | 判等 | **真实现** | staging_dirty→清 out+清标志、selector 写+0x298、retain/哈希、Shared+Secondary 锁发布、published=1，全符 |
| world_raster | 0x14083ebe0 | AllWorldRasterWrite::write | 判等 | **真实现** | 6 槽循环、None→清 composite、长度不符 panic 文案一致、每槽 dirty set_all+staging_dirty=1、尾部 unwrap panic 符 |
| world_raster | 0x14083fa60 | upload_to_staging | 判等 | **真实现** | 6 槽 FrozenMap get+名字 memcmp 断言+copy_to_staging，符 |
| world_raster | 0x140957990 | record_raster_history | 判等 | **真实现** | take_history_stroke、Arc<[Vec3]> strong/weak=1+memcpy、Arc<RasterStroke> 8 字段、History::add，全符 |
| world_raster | 0x140ae4bd0 | DrawState::update | 判等 | **真实现** | r=min(mode_id-2,3) 回绕、三分支 raw/2/3/0 逐条符 |
| world_raster | 0x140a16960 | rasterize_stroke | 判等(结构有偏差) | **存疑-判定过宽** | 命令生成/坐标/混合选择全符；但**原版 `blend_into<closure>` 单步直混，src 用 `render_into(None)` 进零缓冲再逐像素混**——call-chain 不同，行为等价声明依赖 zeno 覆盖度一致性，未独立证毕。按协议 callee 链不等，判等声明偏宽 |
| world_raster | 0x140868fb0 | InProgressRasterStroke::add_point | 判等(浮点微差) | **真实现(注)** | 结构逐条符；距离平方加序原生 `(dz²+dy²)+dx²` vs src `(dy²+dx²)+dz²`，仅末位舍入 |
| world_raster | 0x140868d00 | start_new | 判等 | **真实现** | 清 solid/input/resampled、length@+0x40=0、fluid_cursor 不动、add_point、resampled.push、param_a/b，全符 |
| world_raster | 0x1408691b0 | solidify_fluid_into | 判等 | **真实现** | 相邻去重闭包、<2 原样、try_new+crashy_resample(0.5)、out 空全量/非空 skip(1)，符 |
| world_raster | 0x140868e40 | resample_and_smooth | 判等 | **真实现** | clear+extend(solid)+solidify+smooth_in_place(10,false)，符 |
| world_raster | 0x140ae4c20 | as_float_grid | 未实现-需重做 | **确认分歧真实** | 原生**无 index 参数**，一次返回 4 栅格元组（64B=4×{ptr,len}，按各栅格 selector 选 composite/scratch）；src `as_float_grid(index)` 单索引。签名+逻辑不符，标未实现正确 |
| music | 0x1408b5500 | MusicPlayer::update | 判等 | **真实现** | 5 态状态机（含两跳表 0x142a9fa24/34 分派）、SwitchTrack 体、末尾反暂停快照块，逐分支符 |
| music | 0x1408b70d0 | switch_track | 判等 | **真实现** | 洗牌弹出器:身份 0..n→fastrand shuffle→首元素==当前时 swap(0,len/2)→pop 尾，机器码符 |
| music | 0x1408b6b10 | load_tracks | 判等 | **真实现** | 清 tracks+names→逐 path instantiate+set_callback is_ok→push，符 |
| music | 0x1408b63c0 | init | 判等 | **真实现** | 4 快照名逐串、disabled/active、状态=0/current=None/playlist cap8，符 |
| music | 0x140a71f20 | music_audio_init_system | 判等 | **真实现** | ambience 实例+start、preview 二名、player init、insert_resource，符 |
| music | 0x1408b7620 | get_timeline_beat_properties | 判等 | **真实现** | 存储序 beat→tempo→count→upper→lower→bar 最后写，机器码符 |
| music | 0x140a72810 | music_audio_system | **未实现-需重做** | **推翻(误判)** | LOGIC 称 photomode start 实例互换；REBUILD fresh disasm 0x140a72cfb `lea rcx,[rbx+0x70]` 证明原生 mode==2 **起 photomode(+0x70)** 停 first_person(+0xa0)，Rust 现状正是如此——**分歧不存在**。此为反例:LOGIC 判定可误 |
| session | 0x140b46860 | is_halloween2024 | 判等 | **真实现** | 遍历 mutators Vec，元素==0 停，len 越界返 false，符 |
| session | 0x140b46370 | assets_root | 判等 | **真实现** | Path::_join + glade 两表（6/5/7/6/6）符 |
| session | 0x140b46920 | NextSession::load | 判等 | **真实现** | kind=0x8000000000000002@+0x18、id@+0x20、ignore_snapshot=0、Vec cap8，符 |
| session | 0x140b46300 | session_file_list | 未实现-需重做 | **确认分歧真实** | 原生 `from_iter<RelativePathBuf>`（3 相对路径）；src 返回绝对 PathBuf。语义相对 vs 绝对不判等，标未实现正确 |
| garden | 0x140ae4620 | draw_garden | 判等 | **真实现** | 模式分发/橡皮半径 inverse_lerp/花丛半径/UI 注册/界内/出界事件/状态机/引脚圆/draw_raster，逐条符 |
| garden | 0x141695e10 | record_garden_history | 判等(本体) | **真实现(已修)** | 事件读取/槽5 clear/Arc 包装/History::add 符；LOGIC 指出的自造 failures 分支在 REBUILD 中已删（src 注释证实） |
| input | 0x1416feeb0 | calculate_input_delta/digital_delta | 判等 | **真实现** | 40 模拟槽拷贝+数字槽 Shape A/B 逐槽（0.1/0.05、0.2/0.1 auto_repeat 仅 20..25），机器码符 |
| ivy | 0x140869460 | Ivy::add_point | 判等 | **真实现** | next_point_id 先增后用、push_back、thickness=0，符 |
| ivy | 0x1408696f0 | IvyStorage::spawn_ivy | 判等 | **真实现** | root id=0/thickness=0、范围断言、RNG count→float、insert、next_ivy_id+1，符 |
| ivy | 0x140996080 | ivy_leaf_spawner | 判等 | **真实现** | 容量 10000、uniform>=0.4、噪声>-0.05、normal=avg normalize、disc、3×normal clamp、angle/rotate、delay、add()，符（内联常量未逐字节重导，报告已注） |
| ivy | 0x140b0e380 | ivy_leaf_animation | 判等 | **真实现** | growth<1→delay<=0 时 growth+=2dt clamp/否则 delay-=dt、按 id 找端点、branch_thickness=(1-avg)*0.5+avg，符 |
| ivy | 0x140995a80 | ivy_branch_thickness_animation | 判等 | **真实现** | expf(len/-10)、thickness=taper*(1-att)、len=1 NaN 语义，符 |
| ivy | 0x140999330 | ivy_pruner_cmd | 判等 | **真实现** | box 严格 < 三分量、首个命中、suffix 弹出、growth/pending 清零、delete_affected_leaves，符 |
| ivy | 0x14094c800 | closest_segment_index | 判等 | **真实现** | 双端>max 才跳、t clamp、垂距+|t-t|*len*100、初值 100，SIMD 逐条符 |
| ivy | 0x140b199e0 | into_world_space(REBUILD) | 重建后判等 | **真实现(已修)** | out.y=v、out.x/z=interp±normal·0.315，RNE 0x140b19b72..ba7 符——原 Y 错位已修 |
| render_loop | 0x14098c680 | read_terrain_data | 判等 | **真实现** | 高度→rock→water(仅非 demo)→garden 四 compute pass 写序、回读 cmpxchg→copy_issued|=old!=0，符 |
| render_loop | 0x1409e7770 | bin_lights | 判等 | **真实现** | count==0→两占位、否则 grid 0x10000+clear→count→subtotal→prefix→merge→indices→bin 全链，符 |
| render_loop | 0x1409ba4b0 | DrawLists::sort | 判等 | **真实现** | 21 列表 len>=2 才 sort_by_key 稳定排序，符 |
| camera | 0x140911710 | store_parameters | 判等 | **真实现** | 位置/yaw/pitch/fov 各自有限性检查+替换默认、输出序 yaw/pitch/pos/arm=0/fov/off=0，符 |
| camera | 0x140911520 | load_parameters(REBUILD) | 重建后判等 | **真实现(已修)** | 轨道四元数+ActiveRotation push+0x118=0+ignore_snapshot 门，与原字段直写不同已重建符 |
| walls | 0x140a6a480 | snapping_audio | 判等 | **真实现** | mode2→kind1/mode3→kind2、去重比对 published、xvy swizzle，符 |
| walls | 0x140a6a560 | check_for_disable_snapping | 判等 | **真实现** | mode{2,3}×flag 矩阵 5 分支，符 |
| walls | 0x140ada600 | snapping_init | 判等 | **真实现** | 重置/debug/no-progress/len≤1.65/投影步行/1.5 阈值/mode=1+snapped，符（t 表达式代数等价） |
| walls | 0x140a8a710 | calc_wall_bend | 判等 | **真实现** | 末 4 方向/平均/双 clamp/acos/事件，符 |
| walls | 0x140accab0 | wall_eraser | 判等(结构) | **真实现(结构)** | height=0.75+13.25t/min-current/-0.79 平顶；ChildWall 分裂待接线（报告已注） |
| walls | 0x140a6ad00 | snapping_maintain(REBUILD) | 重建后判等 | **真实现(已修)** | 原只发布模式字节→REBUILD 已全状态发布 @+0x20..0x38（mode/snapped/snap/wall/end），fresh 符 |
| walls | 0x140a6b5e0 | snapping_to_segments(REBUILD) | 重建后判等 | **真实现(已修)** | 原取首候选无排序→REBUILD 已 sort_by 取最近，fresh `sort_by`+`*local_a8` 符 |
| walls | 0x140aa5a10 | query_with_aabb_clamped | 未实现-需重做 | **确认分歧真实** | 原生 get_cells 后 `sort_unstable_by`（复用圆查询比较器）+按 +0x30 去重；src 直接遍历收集无排序。标未实现正确 |
| walls | 0x140aac1e0 | delete_wall | 判等 | **真实现** | 事件→remove_roof/entity/plaster 清链，符 |
| history | 0x140a84c50 | nuke_future_states | 判等-限定 | **真实现** | cursor!=len-1→cursor<0 全清/cursor≥0 截断+retain、nonce=fastrand，符 |
| history | 0x140a84650 | current_snapshot | 判等 | **真实现** | 无分支二分+最近者选择+chain_index==cursor 才返回，符 |
| history | 0x140a82b20/2b40/2d50 | CompoundEditBuilder 三函数 | 判等-限定 | **存疑-判定过宽** | fresh 未见 started 标志/next 空门/build None 分支（原版 next 无条件 push、build 无 None 判定）——src 的 started/is_empty/None 为自造建模。行为面近似但控制流不对应 |
| stairs | 0x1408ffd30 | add_node | 判等 | **真实现** | graph.add_node + dirty_edges insert(self-loop)，符 |
| stairs | 0x1408ffef0 | remove_node | 判等+登记 | **真实现(注)** | 原生仅**节点未找到**时插自环脏边；src 恒插自环。差异已登记 |
| stairs | 0x1408ff210 | restore_backup | 判等+登记 | **真实现(注)** | 原生不写 backed_up；src 强制 true。已登记 |

## 低进度模块 REBUILD 后抽审

- **collision**: LOGIC 自标 8/9 未实现-需重做（raycast 无 toi 比较/mask u64 vs u32、intersects_any 缺地形短路、spherecast/legacy_raycast 返 None 桩、new_convex hash 硬编码 0、mesh_intersect 自造常量）。本审抽读该报告 fresh 依据（planning/logic-collision/），**分歧证据可信，未发现把假逻辑标判等**。
- **terrain**: LOGIC 自标 6/7 核心系统未实现（edit_terrain_stroke 编造 PlayTerrainSound、ui_draw_terrain 画布门反向、ui_edit_terrain 自造 16 位脏掩码、terrain_hover_idx id@+4 vs +8）。已实现中判等 9 个（sample_at_world_xz/delayed_terrain_changed_msg/copy_from_gpu 等）均有 fresh 证据。**未发现漏标**。
- **history 载荷**: ColorEdit/RasterEdit/TreeEditData/DecoratorHistoryEdit/StairsHistoryEdit 均为平面近似（vs 原生按 target/style 变体写互异载荷），LOGIC 已如实标未实现-需重做。**未发现漏标**。
- **roof_noise**: 原生 hash 链含 finalizer+角种子表，src hash2 无 finalizer 且 combine 前异或——数值对照 (0,0) 0.5488 vs 0.7193。LOGIC 已标未实现。**未发现漏标**。

## 结论

1. **最高价值输出:对既有「判等/闭合」声明的独立复验——抽审 55 函数无一被打穿为假**。high-progress 模块（world_raster 26、music 8、garden 2、session 18、input 20、ivy 13、render_loop 34、camera 8）的判等声明**基本真实**，与「真实现模块」总账判断一致。
2. **1 个判定被修正**:music_audio_system 被 LOGIC 标未实现-需重做，REBUILD fresh disasm 反证分歧不存在（原生 mode==2 起 photomote，Rust 现状即如此）。证明既有判定体系**可误**（此处为过度判未实现），需以 fresh 复核兜底。
3. **凑数率估计**（按抽审命中）:world_raster/music/session/garden/input/ivy/render_loop/camera **~0%**（全部真实现，仅 2 处浮点末位/代数等价注记）；walls **0%（抽审判等全部真实）**但报告自列的 on_wall_split/contains_shape/expand/snapping 系未实现为真实分歧；history/stairs **0%（判等真实）**但载荷平面近似/CompoundEditBuilder 自造为真实分歧；collision/terrain 抽读未发现假逻辑被标判等（已实现部分大多真或如实标未实现）。
4. **系统性模式确认**（与总账一致，均已在 LOGIC 中披露）:事件/命令发射对不上（PlayTerrainSound 编造）、数据流假（edit 载荷平面近似/返回类型 Option vs RaycastWorldResult）、调用链错（collision raycast toi 比较、roof_noise hash）、"长得像"近似（CompoundEditBuilder started/None、load_parameters 旧字段直写已重建）。**未发现任何一例 LOGIC 声称判等而实际 body 是凑数的函数**。

## 证据

- fresh decomp: `planning/logic-auditor/decomp/batch1..7/`（65 份）。
- 只读 `--worker shared`；未修改 src；未使用 write worker。
