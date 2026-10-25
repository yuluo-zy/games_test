# 建筑与地形业务只读审计（2026-10-21）

基线 main f72938c81；只读源码和现存取证/验证记录，未运行 Cargo、测试或新逆向。以下百分比是完整业务范围的保守工程估计区间，不是函数数量或测试通过率；缺少穷举分母，置信度中/低。原作等价的交互、历史、视觉宿主验收均未完成。

| 模块 | 估计恢复范围 | 置信度 | 已有 | 最大缺口 |
|---|---:|---|---|---|
| 自由墙绘制/编辑 | 60–70% | 中 | main 实挂 set19 两条原链：draw/continue/snapping/finish/merge/height/publication/delete/spatial；set34 record，set40 replay，set44 flush | 路径与墙真实交点采样是已挂空操作；完整复杂墙体侧面/角件/裙边、支撑与灰泥未闭；整个编辑→保存→重启原作对照未验 |
| wall-constructor（含 gates/fences） | 50–60% | 中 | set19 calculate/init、set22 split/arches/lintels、set28 gates/fences/main bricks、set44真实SSBO刷新；process_segment/score已复用core活路径 | 39.6KB sample_wall_path_intersections 无输出；platform/floors/elevation/plaster/curved-half/玩家stairs等大量明确 todo，不可由legacy整体注册入口挂载 |
| 公共 building/structure/规则形状 | 45–60% | 中低 | raw/public权威、曲线/路径准备、屋顶关联、墙砖与洞口、真实 collision/RT owners 有实现 | 规则形状UX完整8系统等未挂；rectangle输入仍代理且投影注释为结构近似；支撑/楼板等视觉生产者缺失 |
| 屋顶 | 70–80% | 中 | 原 RoofSystems 主插件已挂；UI17/18，生命周期20/21，瓦片/底面/尖顶29，历史34/40，GPU44；瓦片真实Sparse writer；trigger完整12048B重建带native fixtures | 部分UI体积/WidgetParams为零替身（ridge_length）；若干样式/关联辅助挂载点OPEN；任意建筑样式下完整编辑→历史→显示对照未验 |
| 建筑玩家楼梯/平台/扶手 | 30–45% | 中 | 图节点/锚点/样条→装配requests、六资源插值、结构凸包小球分支、history stairs已接 | 主体网格/扶手/梯支撑/订阅/完整cache未闭，视觉宿主明确未挂；wall-constructor平台stairs、玩家stairs架构/柱仍todo。门口短台阶已恢复叶子不能外推整个玩家楼梯 |
| 地形编辑 | 50–65% | 中 | 原 TerrainSystems set15 编辑链+set34record；set40 replay和render高度生产；真实height owners | hover/readback/focus/widget等适配输入在main无真实writer；MouseActivityTimer和TerrainStrokeParamsCache仍未还原，完整原操作生命周期与存档/GPU交互验收未闭 |
| 路径/鹅卵石 | 40–55% | 中 | 四raster资源、真实draw/record独立实现、通用raster replay与staging宿主已挂 | main只初始化PathsPlugin资源/少量事件，未找到draw_path/record_path_history实际宿主调用；不能算用户可完整绘制/撤销链 |
| 花园 | 40–55% | 中 | draw/record独立CPU实现、GardenRaster owner、通用raster replay已挂 | 原GardenPlugin实际draw/record未挂；pin专用替身尚未统一ShowPinnedDecoClutter；完整编辑及GPU视觉闭环未验 |
| 水 | 65–75% | 中 | main WaterPlugin 实挂draw16/record34/audio-grid36，真实WaterRaster、共用raster replay/staging；音频真实raycast+FloatGrid和生命周期有现存验证 | WaterPinnedClutterCircle替身事件未统一到真实pins消费者；whole edit→history→water render→save/reload验收缺；native全局门禁当前constant adapter |

## 主要源码锚点

- `tiny-glade/crates/tiny-glade/src/run_game.rs:98` 原插件总装，terrain/roof实际调用；`:400` 装配；`:409` WaterPlugin；`:965` WallSystems。
- `tiny-glade/crates/tiny-glade/src/app_systems.rs:17` 明确未挂系统说明；`:43` set19原绘制链；`:63` 高度/发布/分段链；`:103` set28；`:126` set34；`:145` set44。注释中的缺实现表部分过时，按下方实际源码区分。
- `tiny-glade/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs:14` 函数只有get_mut和读取curve.length，交点采样未恢复。
- `tiny-glade/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs:54` process_segment直接复用core；`:133` score同实现。新fe772f消除重复todo，但不是整个墙链完成。
- `tiny-glade/reports/runtime/2026-10-19-main-loop-render-recovery/PLUGIN-WIRING-2026-10-20.md:93` 6e5c补录的最新附录C，明确挂载顺序及sample空操作。legacy `add_wall_constructor_systems` 顺序错误且含todo，当前不用是正确选择。
- `tiny-glade/crates/tiny-glade/src/startup_construction_runtime.rs:44` roof辅助注册点OPEN；`:60` 适配输入不是原恢复闭环；`:74` 地形适配无host writer。
- `tiny-glade/crates/systems/roof/src/lib.rs:155` RoofSystems原App注册；`:199` 起UI原阶段；`visual/assemble_roof_tiles.rs:13` 真实写入；`roof_trigger_volume.rs:319` 原fixtures对比测试源码（本审计未运行）。
- `tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_length.rs:118` 零WidgetVolumeShape替身仍存在。
- `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs.rs:108` 节点derive已实现；`stairs_assemble.rs:30` determine_ladder_support仍结构U/返回0；`stairs_railings_assemble.rs:21` assemble_railings空体。
- `tiny-glade/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs:41` todo；`construct_floors.rs:63` todo；`construct_elevation_supports/mod.rs:124`等todo；`plaster_systems.rs:76`等todo。
- `tiny-glade/crates/country-core/src/systems/terrain/mod.rs:198` 两个原owner缺失；`:214` 原插件实际链。
- `tiny-glade/crates/country-core/src/systems/paths.rs:283` draw；`:532` record；`:581` 仅World/Schedule独立build，main未调用。`systems/garden/mod.rs:81` 独立CPU注册，main未调用。
- `tiny-glade/crates/tiny-glade/src/history_runtime.rs:187` 通用replay_rasters实际已挂；run_game旧注释“replay_path/water/garden尚无系统”已过时，不能据旧注释判断回放完全缺失。
- `tiny-glade/crates/systems/water/src/lib.rs:137` 原WaterPlugin注册，`:160` pin事件替身缺口。

## 现存验证边界

`tiny-glade/reports/runtime/2026-10-19-main-loop-render-recovery/latest-audit-fa53d49a-20261020/CURRENT-INTEGRATION.md` 最新收口指出：完整楼梯requests+插值和十源validator已接入，但完整cache/stair mesh与视觉宿主仍未挂；core2838/decorator378/utils160通过是既有快照，不能充当上述模块功能分母。主App240帧零参数校验失败只证明已安装系统资源满足，不证明未挂业务。真实world单帧与30帧图像也不等同完整业务编辑验收。最新wall调度测试仅证明系统各一次及相对顺序，未独立断言非空mask产出。

先前观察 `history_runtime.rs` 头部孤立括号属于工具输出缺行引起误判；已用绝对路径复读排除，正式源码合法，不列作阻断。
