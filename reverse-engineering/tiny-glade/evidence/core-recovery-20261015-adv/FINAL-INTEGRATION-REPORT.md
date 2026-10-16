# country-core 七轮推进整合报告（2026-10-16）

> 依据：`PROGRESS.md` 台账、`round4/{contracts,deliveries,REVIEW.md}`、`round5/`、`round6/`、`round7/`、
> `COORDINATION.md`、各组 `*-DELIVERY.md`。目标 B 版 tiny-glade core 业务一致性，全部逻辑落
> `D:/game/tiny-glade/crates/country-core/`。规则：编译通过≠完成；逐项分离真实接线/原侧验证/仍缺项；
> 无证据标待取证不编造。整体为**功能闭包估计 ≈83%**，非完成率。

---

## 一、推进历程

| 轮 | 结构 | 内容 | 结果 |
|---|---|---|---|
| 第三轮 | 3组×2人 + 中央 | 9 优先模块：核对用户批改(f510c7e7)→关闭 DELTA 缺口；D 组 rhapsody 接入 | 9/9 复核通过（无水分） |
| 第四轮 | 三人团队①RE②实现③review | 逆向关闭剩余缺口（T1 garden_audio/T2 调度器/T3 待取证项） | 抓出 VERTICAL_MIN 编造、EditType 双错；T1/T2/T3 复核通过 |
| 第五轮 | 续跑三人团队 | 三个集中块（A 调度器整合/B Sheep 写盘链/C 快照应用） | A/B 完全闭合、C 结构+调用链闭合 |
| 第六轮 | 续跑（③ 用户停） | 字段级取证→实现闭合（Walls 0x78B/GameKeyBindings/sheep/audio G6/Decorators 0x28B/AsyncSaveCmd） | 批次1+2 复核通过 |
| 第七轮 | 续跑（父直接复核） | 主流程三链（save/load、ECS 调度器、session 切换）+ 真实样本反推 serde | 链2/链3 通过；链1 收敛到真实证据边界 |

跨轮身份：用户重构区（rhapsody/roof/wall 含 snapping/signifier）任何 agent 全程零修改。

---

## 二、模块状态（功能闭包估计）

| 模块 | 进度 | 关键已闭合 | 主要剩余 |
|---|---|---|---|
| ivy | 78% | grower/spawner/pruner/G6 洞约束/flag_48/send_to_gpu CPU | 22 系统原序、count_range RNG 界、send_to_gpu seed/wall_normal、GPU |
| garden | 81% | draw/record/130.0门(T1)/P2 定论/生产注册 | GPU scatter/pin renderer 后置 |
| sheep | 70% | component/being_petted(0.01 纠偏)/布局/query owner/save 写盘链 | spawn/movement 状态机、X4 羊 Query、4 宿主组件、picking |
| music | 85% | 五状态 player/load_glade_audio/GladeAudio 72B/native.rs/audio 全域入生产 | theme 旗标取证、FMOD 真机后置 |
| input | 82% | translate/delta/cursor/mod_key+producer/record_replay/GameKeyBindings 直读 | Steam 全链、UI 历史生产者、表2 键码 |
| terrain | 85% | draw/edit/hover/ui_draw 状态机/replay_terrain/生产注册 | state3、28 序、GPU |
| save | 87% | save_thread/load_game/节拍器备份/Sheep 节写盘/WallState 解码 | GameSaveData 240B 二进制、Decorators 0x28B 打包、Async 全链 |
| session | 85% | SessionMeta/NextSession/load_game_session/事件生产者 | create 载荷字段、消费端完整接线 |
| ui_clutter | 78% | rmb 还原/ui_convert/pins 56B/事件闭环/生产注册 | RMB 生产者(输入域)、Suppression(GPU)、0x58B |
| rhapsody 接入 | 70% | 8 文件真实接线/ExecutionChain 宿主/5 桩处理 | GPU 真机、类型缝、bindless、shader_variants |
| 调度器+startup | 73% | register_core_startup 装配/孤儿系统入生产/28-系统逐名 | 28-tuple 槽位 fn_ptr、Schedule.run 调用点、宿主 GameLoop 驱动 |

**整体 core ≈83%**；GPU/FMOD/render 真机按项目边界后置不计分。

---

## 三、顶层目录

```
resources    ≈83%  类型/状态层最扎实；剩字段名取证
systems      ≈80%  行为层全入生产；剩 wall/render 域(用户重构)+字段级
startup      ≈71%  装配层；剩宿主 GameLoop 执行桥+28 序取证
```

---

## 四、抓出并修正的真水分（质检成效）

| 项 | 发现轮 | 纠偏 |
|---|---|---|
| VERTICAL_MIN=0.000928482 编造值（全 EXE 0 次出现） | 4 | → 0.0@0x142960690 |
| EditType Tree/Terrain 双错（Tree=0/Terrain=15） | 4 | → Terrain=0/Tree=12（visit_str 权威表，真实存档终验） |
| sheep being_petted 阈值 0x292f（地址算术错） | 6 | → 0.01@0x142960674 |
| AsyncSaveCmd 判别值编造 | 6 | → 3..11 对齐原生 |
| ivy spawner 参数三处（count_range/scalar_range/source_60） | 3 | → 0x14/按 path/墙 id |
| garden 130.0 门资源身份 | 4/5 | → CursorTerrainRaycast 32B + WaterRaster |
| Terrain 整节 deflate 旧理解 | 7 | → strokes JSON 明文，flate2 仅栅格尾（真实样本） |

---

## 五、剩余待取证（诚实登记，均标地址）

1. **链1 内存二进制**：Decorators 0x28B/0x16B 打包、WallState 0x70B 曲线区（可变 2..104 点样本受限）、GameSaveData 240B 二进制（拆 0x140a3f470）
2. **sheep**：spawn/movement 状态机、X4 羊 Query（0x140a3f470 sheep 捕获段）、4 宿主组件本体
3. **ui_clutter**：AutoClutterRecord 0x58B 完整布局、0x60B U 区间
4. **ivy**：count_range RNG 界、send_to_gpu seed/wall_normal、22 系统原序
5. **input**：mod_key 表2 键码、GameKeyBindings value 动作 id、Steam 全链、UI 历史生产者
6. **调度器**：28-tuple 槽位 fn_ptr、GameLoop::run 内 Schedule.run 调用点、纯 Commands 身份
7. **会话/写盘**：create 96B+32B+24B 载荷字段、5 域目标资源、on_event 应用对象补全
8. **深层**：camera_rig/weather/time_of_day 深层、6 会话消费系统完整接线
9. **后置**：GPU 真机、live FMOD、suppression、shader 变体/bindless

---

## 六、跨组仲裁（累计 15+ 条，PROGRESS.md 仲裁表）

典型裁定：sheep save 节接口路径（B 供接口 C 接线定序）、systems/debug/sheep_controls（批准）、
cursor_terrain_raycast 32B 扩展（①契约驱动）、EditType 修正（权威表驱动）、save 域采集器+Apply 桥、
G6 音频注册（裁定 add_audio_systems 注册点）。用户三重构区（rhapsody/roof/wall）全程零修改。

---

## 七、证据索引

- 台账：`evidence/core-recovery-20261015-adv/PROGRESS.md`
- 协调：`COORDINATION.md`；模块核对：`A-vegetation/ B-creatures-audio/ C-world-services/` 各 `*-DELTA/DELIVERY.md`
- 逆向契约：`round4/contracts/`(10) `round5/contracts/`(8) `round6/contracts/`(9) `round7/contracts/`(7+)
- 实现交付：`round4/deliveries/ round5/deliveries/ round6/deliveries/ round7/deliveries/`
- 审查：`round4/REVIEW.md round5/REVIEW.md round6/REVIEW.md`
- rhapsody 接入：`D-rhapsody/{WIRING,DELIVERY}.md`
- 真实样本证据：`round7/contracts/r7-chain1-field-CONTRACT.md`、`r7-chain1-writeback-CONTRACT.md`（35+ 发行 snapshot 反推 serde）

---

## 八、结论

country-core 主流程（启动→会话切换→读档恢复→系统运行→存档循环）与 9 优先模块 + rhapsody 接入 +
调度器整合已推进到**真实证据可及的极限**：核心业务真实实现、真实消费者、真实生产注册、整 crate 编译
零错误、真实发行样本反推的 serde schema 落地。剩余全部为诚实登记的字段级取证（部分样本受限）与
GPU/FMOD 后置项。整体功能闭包 ≈83%。

---

## 九、全模块覆盖审计（补充——9 优先模块之外的覆盖）

七轮深推覆盖 9 优先模块 + rhapsody + 调度器；以下为 **country-core 全部其余模块**的覆盖分类
（深推 = 七轮有实质工作；部分 = 有实现但未深度闭合；未深推 = 仅骨架/待取证；重构域 = 用户三区）。

| 模块 | 分类 | 现状 |
|---|---|---|
| **墙系统 wall**（resources/walls + systems/wall + shapes/snapping/signifiers） | 重构域+部分 | inner_wall_state/public_walls CPU 发布/wall_highlight 已真；完整 PublicWalls/snapping/shapes 在你重构收敛后 |
| **屋顶 roof**（resources/roofs + systems/roof） | 重构域+部分 | Roof 88B/roof_tiles/roof_shape 谓词族已真；重构迁移中 |
| **相机 camera_rig** | 未深推 | systems/camera_rig + render 相机资源，深层补证待做 |
| **天气/时间 weather_system/time_of_day** | 未深推 | time_of_day 已注册+run 条件；深层补证待做 |
| **楼梯 stairs / 碰撞 collision** | 部分 | stairs 权威图/快照/差分、collision 查询策略已部分；未深度闭合 |
| **水 water/water_mask/water_exclude_clutter_mask** | 部分 | water 音频已入 G6；水栅格/遮罩/渲染消费部分 |
| **mode_manager/工具栏 toolbar** | 部分 | AppMode 判别/工具栏转换已真；PhotoMode 映射/完整状态机待证 |
| **GUI 全链**（gui/photo_mode/ui_settings/menu/load·save） | 部分 | EscMenuState 拥有状态/菜单推进/缩略图已真；PhotomodeScreen/DebugGui/日志 sink 待 |
| **asset_libraries**（12 文件：color/shader/ssbo/texture/vao/mesh/configwatch 等） | 部分(渲染面) | texture_library 已接 rhapsody；其余宿主/D 组装配面 |
| **prefab/preplaced_autoclutter/paths/brush_preview** | 部分 | prefab RON 解析、autoclutter 预置部分；消费链待 |
| **components/geometry/input 工具层**（drawable/registry/trees/cube/instanced_wall/action_set/input_stream） | 部分 | drawable 已接 nani、action_set 已真；其余部分/复用 |
| **recovered_building/recovered_systems** | 历史保留 | 旧墙/历史实现，作为 evidence 保留，非当前主线 |
| **utils/ 其余**（child_wall/geo/load_json/dirty_tiles/debug_viz/index_condenser 等） | 共享复用 | 已接入共享 utils 或 core 内实现 |
| **startup 其余** | 已覆盖 | 全部 startup_* 已真（见 §二） |
| **render_loop / render 消费** | 后置 | GPU 真机、完整 pass 执行后置 |

**结论补充**：9 优先模块之外，最值得注意的是**墙系统（你重构域）与屋顶（重构域）**——它们是除 9 模块外最大的游戏系统，重构收敛后需按同样口径闭合；其次 camera_rig/weather/time_of_day 深层、GUI Photomode/DebugGui、水系统消费链是下一批可推项。这些均已登记为后续工作，报告中此前未逐条列出，现补充。

