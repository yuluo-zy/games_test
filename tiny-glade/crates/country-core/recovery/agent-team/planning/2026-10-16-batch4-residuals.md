# 第四批残余缺口规划(2026-10-16,coordinator)— 完整 core 行为验收

> 背景:第三批卡A-H 已开工(eco/audio、interaction/{wall,collision,camera_rig,time_weather,ui_mode,
> render_bridge,terrain},state/{history,roofs,stairs,startup} 等 RESTORE 已落盘)。本文件整合
> DISPATCH 与各 RESTORE 登记的**残余缺口**,按依赖排序给专项卡与组/角色,供主会话派发。
> 原则:机械缺口不伪造、跨域接线标注宿主确权、证据项交 re-analyst 定点、完整验收按达成清单核。

## 一、大块机械缺口(需专项卡)

| 卡 | 缺口 | 证据/来源 | 组/角色 | 依赖 | 验收门槛 |
|---|---|---|---|---|---|
| R1 | `collision/world.rs`(170 函数,rapier3d 胶水):17 SubworldIndex 槽 ray/sphere/shape 查询、dirty 瓦片生命周期 flush/publish/QueryPipeline::update、collider upsert/remove 泛型族、parry 网格构造 | interaction/collision/RESTORE.md(已登记 GAP;库内仅 collision.rs 常量锚定) | interaction/restorer | rapier3d(native-physics 特征,已入 Cargo) | RaycastWorld::raycast/spherecast/intersects_any/legacy 与碰撞组/墙/屋顶/楼梯几何衔接闭合;不把"编译过"当完成 |
| R2 | `walls/cached_decorator_transforms.rs`(B 骨架 9.7MB,无 src,仅章节 60 行 + inventory 条目,11 函数) | eco/walls/REVIEW-A1.md、RESTORE-DECORATOR.md §3 登记 | eco/restorer(装饰器续) | DecoratorStorage(已闭合,卡A1) | 缓存变换权威与 DecoratorInfo 消费链闭合;实体↔变换关联取证后实现 |
| R3 | `history/wall_compound_edit.rs` 完整族(79 函数):serde/new/clone/fmt + 复合编辑闭包族 | interaction/wall/RESTORE.md、state/history/RESTORE.md(现仅 WallCompoundHistoryEdit 覆盖核心) | state/restorer | History 权威链(已闭合) | 完整 79 函数族逐函数对齐,MultiEdit 跳表 0x142aee31c 衔接 |
| R4 | `wall/valid_enclosure/mod.rs`(45 函数):墙围合判定资源/检查 | interaction/wall/RESTORE.md(登记 GAP,crate 无对应文件) | interaction/restorer | PublicWalls/墙编辑(已闭合) | 围合判定资源 + 检查链闭合 |
| R5 | `wall/snapping/*`(init/maintain/to_ends/audio)、`wall/eraser/*`(wall_eraser/ui_bulldozer/archivist/animate_wall_bulldoze)、`wall/shapes/*`(calc_shape_height/circle/rectangle/shape_placement) | interaction/wall/RESTORE.md(登记缺) | interaction/restorer | 墙编辑/形状/吸附(卡A2 部分) | 吸附/推土/形状编辑可见链闭合 |
| R6 | `roofs/roof_ridge.rs`(8/7 业务)、`roof_visual_shape.rs`(6)、`roof_tiles/dormer_tiles/roof_buffers` | state/roofs/RESTORE.md(登记 MISSING) | state/restorer | Roof 权威(已闭合);roof_edit serde 对齐(卡C 残留) | ridge 几何/视觉形状/瓦资源闭合 |
| R7 | `stairs/{visual,stairs_collider,stairs_removed_supports,structural_graph}`(11/1/9/8)+ `stairs_walls.rs`(5) | state/stairs/RESTORE.md(登记 MISSING;视觉/碰撞/音频衔接缺口) | state/restorer | StairsState 权威图(已闭合) | 楼梯视觉网格/碰撞/移除支撑/墙-楼梯衔接闭合 |
| R8 | startup 缺文件批量(27):clearing/debug/panic_reporter_startup/init_color_library/load_audio/load_glade_grading/load_glade_tod_and_weather/load_mesh_atlases/load_prefabs/load_shaders/spawn_*/startup_* 等注册胶水 | state/startup/RESTORE.md(多数为域系统注册胶水,随域闭合;纯注册可随后补) | state/restorer | 各域(卡A-F/卡R) | 纯注册文件补齐 + 不注册空系统;与 add_game_startup_systems 并入 |

## 二、跨域接线(依赖 render/host)

| 卡 | 缺口 | 证据/来源 | 组/角色 | 依赖 | 验收门槛 |
|---|---|---|---|---|---|
| X1 | 实体生成(nani 域):nani_update_preplaced_flower 等实体 spawn | interaction/render_bridge/RESTORE.md(登记) | **宿主**(render/nani 域) | 卡A1 decorator | 装饰实体 spawn/实体↔DecoratorId 关联闭合;需主会话确权 |
| X2 | 宿主 run_game 顶层渲染执行序:draw_shaders/compute_passes 执行序 | interaction/render_bridge/RESTORE.md(库内仅合同,登记) | **宿主**(tiny-glade) | 卡H/render 域 | 顶层渲染循环执行序接线闭合;需主会话确权 |
| X3 | 全局调度顺序:add_game_startup_systems(0x140941110)28 系统 tuple 挂载 | state/startup/RESTORE.md(合同已立,register_core_startup 已建) | state + **宿主** | 卡A-G/卡R 各域系统 | 28 系统 tuple 按原序挂载,不注册空系统;宿主 run_game 调用 |
| X4 | save_game 本体 ECS 组装(0x140a3f470):完整资源组装系统(ECS 参数面 + AsyncSaveCmd 发送) | state/save/REVIEW4.md(登记仍阻塞) | state/restorer | 卡0 域类型、卡1 | 6 段 SystemParam + 备份节拍 + 捕获/发送全链闭合 |
| X5 | capture_game_snapshot 域 Query(0x1417a1d00,184KB)+ init_game_state_from_snapshot(0x1417a9c50)应用 | state/save/REVIEW2.md/RESTORE.md(全 Query 串化依赖域 Query) | state/restorer | 各域 Query 类型(卡A-F) | 快照捕获/恢复域 Query 全链闭合 |
| X6 | commander_update case 2/3/7/8/0xc 载荷:case2/3=profiler_sinks(puffin 数据发送)、7/8=Handled 占位、0xc=GPU/内存报告(0x68B 结构+Arc+hashbrown insert) | state/session/RE-ANALYSIS.md、REVIEW5.md(登记 PARTIAL) | state/restorer | meta_commander(已建)+卡2 | case 2/3/7/8/0xc 载荷与 sinks 消费闭合 |
| X7 | run_game_thread 3 命令载荷(rbx+0xb00,&[MetaCommand;3]):逐 cmd tag/载荷 | state/session/RE-SESSION-SWITCH.md、state/startup/RESTORE.md(需 ReVa p-code 定点) | state/re-analyst + restorer | X6/meta_commander | 3 条 MetaCommand 逐 cmd 原字节闭合;会话切换链收口 |

## 三、待取证证据项(派 re-analyst,并行)

| 卡 | 证据项 | 证据/来源 | 组/角色 | 服务 |
|---|---|---|---|---|
| E1 | EditType 40 变体逐 tag 名映射(Debug fmt 跳表 0x142ad88d0 vs MultiEdit deserialize 0x142aee31c;现 14 变体保留号待定) | state/history/RESTORE.md、REVIEW6.md §5.1 | state/re-analyst | 卡R3/MultiEdit 对齐 |
| E2 | DecoratorInfo 72B 完整字段表(+0x00..0x20 与 +0x24..0x44 逐字段语义;**+0x45 字节** rank_ty 区;ExtraInfo 联合载荷) | eco/walls/RESTORE-DECORATOR.md §6.1 | eco/re-analyst | 卡A1/R2 |
| E3 | sheep kind4 loop 事件名(0x1409d61b0 帧尾 loop event):"event:/SFX/Sheep/Baah" 候选未对跳表证实 | eco/audio/RESTORE-BATCH3.md §三 | eco/re-analyst | 卡F/audio |
| E4 | ivy/garden 实体→SSBO 48B 字段:实例布局 48B 逐字段映射(ivy GPU 48B↔104B CPU 叶) | interaction/render_bridge/RESTORE.md、eco/ivy/RESTORE.md | eco/re-analyst | 卡H/卡6 |
| E5 | ivy 三键 NAME 串:0x9e3cfa7f2993f406/0x1d6de5e69ac50d2a/0x2002d2fbdff3d80f 经 ~2000 候选均不满足 xxh3_64;NAME 未定位 + NameHash secret 裁决(0x140c9cd80 是否传 0x142b2ebb0 secret) | eco/ivy/RESTORE.md、verify_ssbo_id.py、scratch_ssbo.rs | eco/re-analyst | SsboId::of 对账(utils 库内修) |
| E6 | 噪声阈值 res+0x78:FastNoise 密度闸门阈值来源(IvySpawnNoise 构造参数) | eco/ivy/RESTORE.md §四 | eco/re-analyst | 卡A ivy grower |
| E7 | terrain EditCommand 字段序(原事件 {falloff,height,position,id} id@+0x1c vs tg_terrain) | interaction/terrain/RESTORE.md(登记待取证) | interaction/re-analyst | 卡B/terrain |

## 四、验收层面达成清单(完整 core 行为验收)

完整 core 行为验收 = 下列全部闭合,任一未闭合不得宣布"完整 core"或模块 100%:

1. **全局调度**:X3(add_game_startup_systems 28 系统原序)+ 宿主 run_game register_core_startup
   调用;lib.rs 模块声明齐全、无空系统注册。
2. **存档往返**:X4(save_game 本体 ECS)+ X5(capture/restore 域 Query)+ 卡1(域组装)+ 卡2
   (load_game/会话切换链)+ 卡0(域类型)。"保存→文件→读档→恢复"行为等价验证。
3. **渲染消费桥**:卡H(ivy/garden/terrain GPU 提交)+ X1(实体生成)+ X2(宿主 run_game 执行序)+
   render_loop→render 桥;GPU 消费链真实执行,非空壳。
4. **证据完备**:E1-E7 全部定点闭合(fresh 反编译 + rne-read),卡R 各机械缺口逐函数闭合;
   NameHash secret 裁决落地 utils 库内修。
5. **宿主确权**:X1/X2/X3 与 Steam 全链、DecoratorStorage 实体(system_clutter)、save/load 经
   run_game 均需主会话确权范围;确权前不伪造 GPU/调度/加载成功。

## 依赖顺序

```
[并行证据] E1-E7(re-analyst)→ 卡R3/R2/R6/R7/R5 与 X4/X5/X6 对齐
[机械缺口] R4/R5(wall,交互)→R6/R7(roofs/stairs,state)→R1(collision,依赖几何)→R8(startup,依赖各域)
[跨域] X5(capture Query,依赖域 Query)→X4(save_game ECS)→X6/X7(commander/run_game,依赖卡2)
[宿主] X1/X2/X3 + 卡H(需确权;依赖 R 卡与渲染域)
```

**优先建议**:E5(NameHash secret)与 E1(EditType)先出证据(影响 utils 库内修与 MultiEdit 对齐);
R2(cached_decorator_transforms 9.7MB 骨架)与 R1(collision 170 函数)体量大,宜各派一名 restorer
专职;宿主项 X1-X3 与卡H 需主会话确权后再派。
