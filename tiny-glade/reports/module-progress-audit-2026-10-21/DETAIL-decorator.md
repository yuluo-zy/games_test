# 2026-10-21 Decorator / clutter / ivy 只读审计

基线：main f72938c81（父任务已同步）。本次只读代码和已提交报告，不运行测试、不重新逆向。百分比为“原业务恢复且可接入”的保守区间，无完整逐函数分母，置信度中低；不能换算成可玩率，也不与旧27%机械相加。

| 模块 | 估计恢复 | 主宿主现状 | 最大缺口 |
|---|---:|---|---|
| decorator 总体 | 40–55% | 资源/真实loader/独立窗灯和事件已接；完整UI与set25视觉未挂 | move/place主body、真实控件命中、完整cache、原注册顺序、卸载与多帧历史验收 |
| clutter/autoclutter | 45–60% | ClutterPlugin真实Startup及Update/set23、36已接 | region种类/排序/过滤仍猜测；manager只接受Window0/1/2，其余nonempty显式pending；原形状缝合/城垛算法尚有adapter |
| ivy | 45–60% | startup_ivy与owner已接，主App未找到grow/spawn/prune/update/upload调用 | 原22系统完整安装、增长/出生常量/墙语义、叶/花/branch上传字段与真实设备闭链 |
| window装饰与玻璃/灯 | 50–65% | 窗灯已真实set36安装；完整window generators所在set25未装 | 农舍墙窗/角窗较完整，但哥特墙窗world-space仍简式，完整动态几何、generator→glass/Nani宿主联合验收 |
| door装饰 | 30–45% | 资产/蓝图与door stairs叶函数有实现，generator未挂 | 两种阳台门builder创建后丢弃，农舍附加clutter记录丢弃，几何/参数/实际visual构造未全闭合 |
| stairs业务及视觉 | 30–45% | StairsState/requests资源存在，完整producer/interpolation无实际宿主调用 | stair-attached cache/roof63、graph维护、实际stair visuals/railings/装配/碰撞注册回调、UI选点编辑及host specialization |

重要纠正：最新08ece后 validator已不是缺实现。blueprint.rs:814的完整14参数validate_blueprints已恢复，开头正确drain tracking，拒绝后backup.state=2及mute真实事件；CURRENT-INTEGRATION.md:92记录完整11类原因及反馈顺序。其尚未装入主宿主。完整双楼梯requests和六资源interpolation也已恢复，不能沿用10/20旧报告说完全没有producer。cache却仍不完整：参数_stairs_requests/_stairs未使用，roof63返回None，dst4最后None。

直接源码证据（所有路径以D:/game/为前缀）：
- tiny-glade/crates/tiny-glade/src/run_game.rs:405，仅build_startup_resources；:889 stairs请求owner；:900窗灯set36；:916 unleash set25。
- tiny-glade/crates/tiny-glade/src/decorator_plugin.rs:51完整unload todo；:71候选visual安装函数只有定义，未发现调用。build:30定义也未被main调用。
- tiny-glade/crates/systems/decorator/src/cache_decorator_transforms.rs:32九参；:39、:41 stairs参数未用；:196 roof63 None；:260 dst4 None。
- tiny-glade/crates/systems/decorator/src/blueprint.rs:814 validator真实实现。
- tiny-glade/crates/systems/decorator/src/ui_systems/ui_move_decorator.rs:62 gated后空body；:87 find_stair_placement_candidate恒None。
- tiny-glade/crates/systems/decorator/src/ui_systems/ui_place_decorator.rs:49主体只init/id生成；:93开始真实cursor图标factory。
- tiny-glade/crates/systems/decorator/src/decorator_visual/stairs.rs:338 stair visual空；:349完整请求producer；:781 graph维护空。
- tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:37 support恒0；:51订阅空；:61create/remove碰撞空。
- tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs:20 assemble_railings及后续几何空。
- tiny-glade/crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs:162真实七参generator；:195 glass WindowLight key与蓝图visual、墙洞、lintel等；cottage_corner_window.rs:83真实生成。
- tiny-glade/crates/systems/decorator/src/decorator_visual/gothic_wall_window.rs:97 world-space明确简式未恢复。
- tiny-glade/crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs:107 builder未消费；cottage_balcony_door.rs:178起builder与附加record丢弃。
- tiny-glade/crates/systems/clutter/src/lib.rs:95真实ClutterPlugin；run_game.rs:109实际安装。
- tiny-glade/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs:171种类猜测；process_autoclutter_candidates.rs:88/95过滤/排序猜测；manage_autoclutter_entities.rs:23–29非Window合同explicit pending。
- tiny-glade/crates/tiny-glade/src/gameplay_startup.rs:369 ivy owner，:374实际startup；country-core/src/systems/ivy/registration.rs:83原22系统只表达7CPU，GPU/音频待接；main/run_game未找到add_ivy_systems调用。
- country-core/src/systems/ivy/ivy_spawner.rs:77出生上界40待证；ivy_send_to_gpu_leaf.rs:89等scale/arch语义待证；ivy_send_to_gpu_segments.rs:22/33字段待证。

既有验证（本次未重跑）：latest-audit-fa53d49a-20261020/FINAL-CORE-ROUND-RESULT-20261021.json记录 core 2838pass/1ignored，decorator378pass/2ignored，utils160pass；原安装full/demo资产2/2（四表52/34键）；默认与native-audio check通过。只能证明已有CPU/类型边界，不是完整视觉宿主/GPU/游戏验收；旧CENTRAL-REPORT.md的364与FMOD/CR阻断已过期。
