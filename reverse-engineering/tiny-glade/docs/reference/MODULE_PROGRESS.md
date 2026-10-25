# Tiny Glade 源码模块进度表

> 新鲜度（2026-10-24）：本表最后更新于 2026-10-13，当前状态以 [STATUS.md](../../../../tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md) 为准；本表用于模块/文件映射与证据定位，不作为完成率。

更新：2026-10-13T19:46:14.306122+08:00。以原作PDB的crate、目录、文件为主键；B是主要恢复基线，C适配单独验收。

这是后续逆向推进的统一入口。历史里程碑和批次报告保留为证据，不再作为任务划分或完成率依据。

共享基础优先按[业务功能与当前源码对照](../reference/FOUNDATION_BUSINESS_COMPARISON.md)推进：库内逻辑缺失、原验证缺口和库外接线分别列出。[原函数接口库存](../current/INTERFACE_PROGRESS.md)用于定位和映射；不要求原函数同名，等价组合实现也登记为已有代码，宏/getter不单列功能任务。

用户重要指示：不要求源码级别1:1复刻。先分析真实业务逻辑，自主实现，再与原作实际规则对照、记录差异、修复并复验；源码写法与原Rust ABI不作为必须相同的目标。[业务对照流程](../../evidence/shared-foundation-2026-10-13/BEHAVIOR_WORKFLOW.md)。

包级上下游与层次：[交互式包引用图](../../package-dependencies.html)。图使用B原锁依赖，模块表更新后运行 `python3 scripts/build-package-graph.py` 刷新图中状态。

实现、接线、原码验证、GPU验证分别登记；候选函数和适配器不等于作者原函数。完整验收只能由人工明确登记版本、范围和报告，刷新不会自动升格。本次刷新只核对清单、文件和显式路径哈希，不运行游戏测试。

## 维护方式

1. 先打开目标模块的明细，选择实际生产者→消费者闭包；不按孤立函数或新增candidate crate派工。
2. 更新 [人工范围与映射](../../tracking/module-progress-overrides.json) 中的文件、函数、缺口、接线和证据；精确文件映射放入file_bindings，逐函数合同/证明放入function_bindings。原文件/原名未知时留空，不靠词法扫描补造。
3. 运行 `python3 scripts/update-module-progress.py` 生成总览、完整文件台账和模块明细；再运行 `python3 scripts/update-module-progress.py --check` 检查输入和清单。
4. 源码、依赖或验收输入变化时，仅重验受影响范围；保留旧报告，不改旧hash冒认当前通过。已匹配的显式路径hash也不证明报告绑定了全部输入。

状态：部分分析/草稿/候选/接线仅描述已列范围；未逐项评估不代表从未研究；“记录有交付，本机缺失”不计入本机实现。Rust函数定义扫描不提供原名/ABI/逐函数验收。

## 模块总览

| 原源码模块 | 状态 | B原接口：已观察/已分析/未触及 | 原文件 A/B/C | 本机恢复文件引用数 | 已恢复范围 | 接线与验证 | 下一闭包 |
|---|---|---|---|---|---|---|---|
| [crates/asset-pipe/src](../../tracking/module-progress/crates__asset-pipe__src.md) | 部分候选实现 | 14/13/1 | 2/2/5 | 2 | B两原文件的图片处理、mip/padding、DDS/数组、原压缩preset、版本缓存与输出；已接原easy-parallel。 | 两资产包14测试、严格Clippy通过；OETF4364原码零差异；完整原图流水线差分未完成。 | 按接口覆盖表补未触及接口及全链原侧对照 |
| [crates/asset/src](../../tracking/module-progress/crates__asset__src.md) | 部分候选实现 | 18/16/2 | 8/9/13 | 11 | B九原文件的资源扫描、prefab继承、纹理交换格式；texture.rs是合成glue，不计C恢复。 | 388原纹理字节回写一致，25prefab/28依赖通过；候选合同与原全链差分分开。 | 按接口覆盖表补未触及接口及真实消费者 |
| [crates/clutter-icon-pipe/src](../../tracking/module-progress/crates__clutter-icon-pipe__src.md) | 未逐项评估 | 未盘点 | 0/0/1 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/clutter-pack-pipe/src](../../tracking/module-progress/crates__clutter-pack-pipe__src.md) | 未逐项评估 | 未盘点 | 0/0/2 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/conservative-line-raster/src](../../tracking/module-progress/crates__conservative-line-raster__src.md) | 依赖/归属边界 | 未盘点 | 1/1/1 | 0 | 先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。 | 依赖实现不计游戏逆向覆盖。 | 来源/必要分叉差异待核实，公共实现复用 |
| [crates/country-core/src](../../tracking/module-progress/crates__country-core__src.md) | 部分候选实现 | 未盘点 | 6/6/6 | 1 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/asset_libraries](../../tracking/module-progress/crates__country-core__src__asset_libraries.md) | 部分接线 | 未盘点 | 9/12/11 | 2 | 实际decorator collision JSON读取与mesh/shader library生产者边界已有局部恢复/分析。 | 18键资产放置等限定World链；ShaderLibrary只读分析非完整Rust恢复。 | 完整asset/pass关联、缓存淘汰和资源所有权 |
| [crates/country-core/src/components](../../tracking/module-progress/crates__country-core__src__components.md) | 未逐项评估 | 未盘点 | 3/3/3 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/geometry](../../tracking/module-progress/crates__country-core__src__geometry.md) | 未逐项评估 | 未盘点 | 4/4/4 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/input](../../tracking/module-progress/crates__country-core__src__input.md) | 部分候选实现 | 未盘点 | 3/4/4 | 1 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/render](../../tracking/module-progress/crates__country-core__src__render.md) | 记录有交付，本机缺失 | 未盘点 | 34/39/28 | 43 | pass_builder、SSBO/Shadowed/SparseBuffer、Mesh/Shader、nani GPU-world/atlas/实例与21 ECS系统有交付记录。 | 项目记忆报告offline check/clippy通过；本机源缺失，不计本机已复核实现。 | 找回原交付与合同，核对身份/源码hash，再接pass到rhapsody真实Command与managed资源 |
| [crates/country-core/src/resources](../../tracking/module-progress/crates__country-core__src__resources.md) | 部分候选实现 | 未盘点 | 56/60/66 | 7 | 地形、水、树存储、critters/window lights等跨系统资源有局部字段证据；不是整个resources目录恢复。 | 水模块候选9测试；其他资源主要分析或缺失交付记录。 | 每个资源精确字段/原类型和生产者仍待对齐 |
| [crates/country-core/src/resources/critters](../../tracking/module-progress/crates__country-core__src__resources__critters.md) | 未逐项评估 | 未盘点 | 2/2/2 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/resources/history](../../tracking/module-progress/crates__country-core__src__resources__history.md) | 部分接线 | 未盘点 | 10/12/13 | 9 | A/B/C历史格式解析、Wall/Roof事务和游戏命令字段；其他命令保留未知值。 | 历史静态样本与Rust解析有证明，建筑事务限定链通过；不等于完整存档。 | Path/Garden/Color/Stairs等完整owner和schema消费者 |
| [crates/country-core/src/resources/ivy](../../tracking/module-progress/crates__country-core__src__resources__ivy.md) | 未逐项评估 | 未盘点 | 4/4/4 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/resources/render](../../tracking/module-progress/crates__country-core__src__resources__render.md) | 未逐项评估 | 未盘点 | 3/3/2 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/resources/roofs](../../tracking/module-progress/crates__country-core__src__resources__roofs.md) | 部分接线 | 未盘点 | 11/11/11 | 7 | Roof88、参数/形状、曲面/Curve2、脊线、方向与Flat/Rim部分消费者已恢复。 | 限定原码报告：surface23810、ridge14776；参数与状态有独立对照。不是全部11文件验收。 | roof_noise/buffers/trigger_volume/tile_sim等全部字段与消费者未闭合 |
| [crates/country-core/src/resources/rt](../../tracking/module-progress/crates__country-core__src__resources__rt.md) | 未逐项评估 | 未盘点 | 3/3/3 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/resources/session](../../tracking/module-progress/crates__country-core__src__resources__session.md) | 未逐项评估 | 未盘点 | 0/0/1 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/resources/stairs](../../tracking/module-progress/crates__country-core__src__resources__stairs.md) | 未逐项评估 | 未盘点 | 0/7/7 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/resources/walls](../../tracking/module-progress/crates__country-core__src__resources__walls.md) | 部分接线 | 未盘点 | 22/25/32 | 17 | Public/InnerWall投影、开口存储、subtype、装饰字段与部分附着候选。 | 高度、hole、subtype有原码局部证明；Info和部分storage为新增候选，不能整体通过。 | 未知published字段及PublicWall+296生产者 |
| [crates/country-core/src/startup](../../tracking/module-progress/crates__country-core__src__startup.md) | 未逐项评估 | 未盘点 | 37/35/43 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems](../../tracking/module-progress/crates__country-core__src__systems.md) | 部分候选实现 | 未盘点 | 29/29/37 | 1 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/audio](../../tracking/module-progress/crates__country-core__src__systems__audio.md) | 部分候选实现 | 未盘点 | 34/34/35 | 11 | 16事件注册；wall/building/stairs/music/menu/sheep/clutter/water局部事件映射。 | clutter+audio候选check/build/test5/5；非FMOD运行或原作完整音频等价。 | FMOD资源类型/实例和真实后端接线 |
| [crates/country-core/src/systems/backend](../../tracking/module-progress/crates__country-core__src__systems__backend.md) | 未逐项评估 | 未盘点 | 16/19/19 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/camera_rig](../../tracking/module-progress/crates__country-core__src__systems__camera_rig.md) | 部分草稿 | 未盘点 | 5/5/5 | 7 | normal/free/walk/butterfly/camera_body候选源码；不是完整MainCamera。 | 构造器native返回已观察；Rust完整差分/系统未验收。 | 完整input/mode transition和MainCamera编排 |
| [crates/country-core/src/systems/collision](../../tracking/module-progress/crates__country-core__src__systems__collision.md) | 未逐项评估 | 未盘点 | 3/3/3 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/debug](../../tracking/module-progress/crates__country-core__src__systems__debug.md) | 未逐项评估 | 未盘点 | 3/3/3 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/garden](../../tracking/module-progress/crates__country-core__src__systems__garden.md) | 未逐项评估 | 未盘点 | 2/2/5 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/grass2](../../tracking/module-progress/crates__country-core__src__systems__grass2.md) | 未逐项评估 | 未盘点 | 0/0/1 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/history](../../tracking/module-progress/crates__country-core__src__systems__history.md) | 部分接线 | 未盘点 | 13/15/16 | 9 | Wall/Roof外层bucket与同Wall组barrier、计时和离散回放；terrain/tree/water/auto-clutter局部链。 | 建筑冻结链通过；terrain Edit512原作↔Rust；其他新回放候选范围独立。 | 全部命令owner统一调度、失败展开、snapshot联接 |
| [crates/country-core/src/systems/ivy](../../tracking/module-progress/crates__country-core__src__systems__ivy.md) | 未逐项评估 | 未盘点 | 9/9/9 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/paths](../../tracking/module-progress/crates__country-core__src__systems__paths.md) | 未逐项评估 | 未盘点 | 4/4/4 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/render_loop](../../tracking/module-progress/crates__country-core__src__systems__render_loop.md) | 记录有交付，本机缺失 | 未盘点 | 78/81/63 | 84 | render_world帧顺序、54pass调用、8条件分支及CPU组合有分析/交付记录。 | B原二进制+404shader静态分析；没有整帧GPU验收。 | CPU交付源码本机缺失 |
| [crates/country-core/src/systems/save](../../tracking/module-progress/crates__country-core__src__systems__save.md) | 未逐项评估 | 未盘点 | 5/11/12 | 0 | 仅建立原文件清单，未登记完整恢复。 | 未建立模块验收。 | save_game/load_game/snapshot/snapshot_gc完整状态、格式、后台消费者与错误恢复 |
| [crates/country-core/src/systems/steam](../../tracking/module-progress/crates__country-core__src__systems__steam.md) | 未逐项评估 | 未盘点 | 2/2/7 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/terrain](../../tracking/module-progress/crates__country-core__src__systems__terrain.md) | 部分候选实现 | 未盘点 | 7/9/10 | 5 | Stroke schema、点列/ID、已解析编辑与Edit回放；重算/读回事件链已分析。 | M1报告Edit512原作↔Rust；Delta512是早期原片段↔公式，不能升格完整Rust入口。 | New/Move全入口与UI、pending history、真实ECS |
| [crates/country-core/src/systems/ui](../../tracking/module-progress/crates__country-core__src__systems__ui.md) | 部分候选实现 | 未盘点 | 10/11/13 | 1 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/ui_clutter](../../tracking/module-progress/crates__country-core__src__systems__ui_clutter.md) | 未逐项评估 | 未盘点 | 0/7/7 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/country-core/src/systems/wall](../../tracking/module-progress/crates__country-core__src__systems__wall.md) | 部分接线 | 未盘点 | 56/57/59 | 20 | raw Curve2→terrain/raster/prepared→cache/publication→Roof/openings/brick消费者；创建、高度、删除/bulldoze及部分矩形编辑。 | 阶段4冻结54测试/67限定证明；新增并行批不继承冻结验收。 | 完整UI placement/startup、snapping/merge/splitter未恢复 |
| [crates/country-core/src/utils](../../tracking/module-progress/crates__country-core__src__utils.md) | 部分候选实现 | 未盘点 | 8/8/9 | 1 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/crash-reporter-gui/src](../../tracking/module-progress/crates__crash-reporter-gui__src.md) | 部分候选实现 | 未盘点 | 1/1/1 | 2 | B诊断优先级、前8帧匹配、复制提示、一次callback/thread与Tick完成联接，内嵌完整性报告。 | 共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。 | 原panic报告producer/callback payload ABI与发送入口；完整原GUI样式/clipboard OS实效/真实崩溃端到端；A/C未适配 |
| [crates/feedback-reporter/src](../../tracking/module-progress/crates__feedback-reporter__src.md) | 部分候选实现 | 未盘点 | 1/1/1 | 1 | B等待/替换/601计数、选定文件ZIP、benchmark、multipart/HTTP失败规则与Bevy Update事件插件。 | 2026-10-14 库内 fresh 反编译复核（B）：build@0x1406901f0/report_player_feedback@0x1406903a0/zip_dir@0x141002650（内联 send_report/prepare/zip_benchmark）逐项对照，601/等待分支、multipart顺序/MIME、HTTP规则、obfstr endpoint 字节核验；修复 zip_dir 缺失跳过（原 stat 后仅 NotFound 跳过、其它 metadata 错误落 is_file panic）。隔离验证 workspace 9 测试/clippy -D warnings 通过。库外 producer 接线与 A/C 仍留。 | 公开字段与两ready路径语义原名/ABI；完整原report/zip入口oracle；country-core/UI实际反馈producer和A/C适配 |
| [crates/frozen/src](../../tracking/module-progress/crates__frozen__src.md) | 依赖/归属边界 | 未盘点 | 1/1/1 | 0 | 先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。 | 依赖实现不计游戏逆向覆盖。 | 来源/必要分叉差异待核实，公共实现复用 |
| [crates/geforcenow/src](../../tracking/module-progress/crates__geforcenow__src.md) | 部分候选实现 | 未盘点 | 0/1/1 | 1 | B固定DLL/三导出全部加载后initialize(0)，成功query/shutdown/unload；失败不吞为false。 | 共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。 | 真实GFN SDK未加载/云服务未运行；实际游戏启动/路径/配置消费者；原错误枚举标签/Display与A/C适配 |
| [crates/gpu-allocator/src](../../tracking/module-progress/crates__gpu-allocator__src.md) | 依赖/归属边界 | 未盘点 | 12/12/0 | 0 | 先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。 | 依赖实现不计游戏逆向覆盖。 | 来源/必要分叉差异待核实，公共实现复用 |
| [crates/lang-lib/src](../../tracking/module-progress/crates__lang-lib__src.md) | 部分候选实现 | 11/11/0 | 2/2/2 | 2 | 语言加载、回退、热切换、缺失key和请求locale持久化；修正非Unicode环境分支。 | 2单元测试、5隔离进程、15发行locale通过；原8入口静态对照、15调用边界，另3内联接口单列。 | 补内联接口合同并接入真实消费者 |
| [crates/leekspin/src](../../tracking/module-progress/crates__leekspin__src.md) | 依赖/归属边界 | 未盘点 | 0/2/0 | 0 | 先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。 | 依赖实现不计游戏逆向覆盖。 | 来源/必要分叉差异待核实，公共实现复用 |
| [crates/motomoto/src](../../tracking/module-progress/crates__motomoto__src.md) | 部分候选实现 | 未盘点 | 1/1/1 | 2 | B chunk tag/Writer/ChunkWriter/Reader/Iterator业务链可调用；库IO/LE编码复用byteorder与std。 | 既有6候选tests/173原Reader案例；独立review新增20限定原Cursor Writer/Drop+pending error与3原Reader错误后恢复组均匹配，候选四故障/unwind/零复制验证。无确认代码缺陷或性能回退，完整save consumer/泛型IO仍未验收。 | 在真实save consumer恢复schema/压缩/失败事务，按需核实泛型IO合同；不重复未变孤立案例 |
| [crates/paths/src](../../tracking/module-progress/crates__paths__src.md) | 部分候选实现 | 10/10/0 | 1/1/1 | 1 | Windows路径选择、账号目录和顺序初始化/错误传播；10个原接口映射。 | 原锁候选合同验证；静态原代码规则核对，完整消费者和原入口运行差分尚未完成。 | 对接原初始化消费者，补原入口行为对照 |
| [crates/rhapsody/crates](../../tracking/module-progress/crates__rhapsody__crates.md) | 功能作用已评估；逐文件恢复未评估 | 未盘点 | 3/7/0 | 0 | 对渲染帧路径非阻塞：3 辅助（aftermath-rs/dxgi-enumerate-gpus/gpu-profiler）+ vk-sync-fork 可复用公开基座 + shader-pipeline 已恢复 | 未建立模块/文件级验收；候选交付不等于原作等价；真实挂账仅 vk-sync-fork 分叉补丁待核实 | 不启动逐文件逆向；仅具体消费链要求时再评估 |
| [crates/rhapsody/crates/shader-pipeline/src](../../tracking/module-progress/crates__rhapsody__crates__shader-pipeline__src.md) | 部分接线 | 未盘点 | 5/5/0 | 6 | B原compiled容器完整schema、反射/类型、文件键与binding hash已恢复；5个原PDB文件映射接入现有rhapsody。 续轮已接独立反射、后处理与fresh容器producer。 | B404原文件/638模块完整typed decode/encode；独立再反射404完整metadata零差异；新编译404包中210metadata与原一致，191依赖集合不同。wire再生成27exact/377map顺序变化，不计字节全等。 | 补原macro manifest/metadata语义残差与加载缓存错误，接country-core资产/pass |
| [crates/rhapsody/src](../../tracking/module-progress/crates__rhapsody__src.md) | 部分候选实现 | 未盘点 | 0/2/0 | 2 | crate根与leek入口包装已实现部分候选。 | 候选check/test/clippy；不能从根文件存在判断完整crate完成。 | 全部原root/reexports/features/原API闭合 |
| [crates/rhapsody/src/raw](../../tracking/module-progress/crates__rhapsody__src__raw.md) | 部分接线 | 未盘点 | 28/31/0 | 35 | 设备、buffer/image/view/sampler、descriptor/pipeline/attachments、command/submit/WSI/诊断/格式候选可运行。 2026-10-13：真实compiled asset直接stored reflection生产布局/compute，删除catalog offset生产fixture。 | 10/12候选统一64测试/13GPU探针；10/13补接口69测试和严格Clippy、未重跑GPU。报告当前性逐项检查。 | get_setup_cb_submit_info/with_setup_cb、init_texture_data/update_texture、fullscreen exclusive |
| [crates/rhapsody/src/raw/managed](../../tracking/module-progress/crates__rhapsody__src__raw__managed.md) | 部分接线 | 未盘点 | 9/9/0 | 12 | handle/ref/generation/checksum、资源持有/CPU退役/GPU保护与managed资源包装。 | 限定原生命周期合同和候选测试/GPU链；新增接口不是原完整ABI恢复。 | 原全域producer/序号桶、真实游戏持有关系与性能 |
| [crates/rhapsody/src/rg](../../tracking/module-progress/crates__rhapsody__src__rg.md) | 部分接线 | 未盘点 | 14/15/0 | 27 | graph/compiled/registry/pool、访问分析、barrier/record、executor/temporal/import-export/pass API。 | 原CPU合同/叶函数限定差分；候选串行session/temporal GPU已运行；全图与原整帧未验收。 | 将游戏pass的registry与正在录制的Command接入首个真实shader callback，完成同一提交/回收 |
| [crates/shader-variants/src](../../tracking/module-progress/crates__shader-variants__src.md) | 部分接线 | 未盘点 | 1/1/0 | 1 | B原raster variant函数clone base后追加SHADER_VARIANT_DEPTH_ONLY=0/1；完整原compute宏manifest未恢复。 | 234原raster资产macro/key证据；新fill/egui实际编译与GPU布局通过；不代表全部变体或shader算法等价。 | 从原pass/variant生产者恢复compute宏tuple；禁止未知空宏包冒充原canonical key |
| [crates/systems/clutter/src](../../tracking/module-progress/crates__systems__clutter__src.md) | 部分候选实现 | 未盘点 | 20/20/20 | 9 | autoclutter区域/筛选/放置/实体管理、merlons/stitches、auto-clutter历史草稿接可编译crate。 | clutter+audio check/build/test5/5，使用glue占位；未通过原作完整行为验证。 | BrickTransformSsbo/StairsFloatingBrickParams字段与真实资源producer |
| [crates/systems/color/src](../../tracking/module-progress/crates__systems__color__src.md) | 未逐项评估 | 未盘点 | 17/17/28 | 0 | 索引/零散候选；完整系统范围尚未核对。 | 未登记完整模块验收。 | 完整文件/函数、状态/资源和生产者消费者尚未恢复 |
| [crates/systems/critters/src](../../tracking/module-progress/crates__systems__critters__src.md) | 部分候选实现 | 未盘点 | 24/24/24 | 10 | duck/shared/sheep terrain/wren boids/fish/butterfly候选；firefly和sheep pet独立局部。 | 新harness记录22测试；firefly原码报告有失败，不记全通过；其他物种新源码未等价验收。 | CritterLocomotion物种映射、全startup/delete/history/建筑反应 |
| [crates/systems/decorator/src](../../tracking/module-progress/crates__systems__decorator__src.md) | 部分接线 | 未盘点 | 64/74/86 | 9 | 墙面四族asset/subtype/position→raw holes→wall brick；Info/storage与部分stairs候选。 | placement1668/subtype1326等限定报告；新增Info/storage不冒称当前整批通过。 | 全部decorator_visual家族：dormer/chimney/corner/flag/clothline/lantern等 |
| [crates/systems/forest/src](../../tracking/module-progress/crates__systems__forest__src.md) | 未逐项评估 | 未盘点 | 3/3/3 | 0 | 索引/零散候选；完整系统范围尚未核对。 | 未登记完整模块验收。 | 完整文件/函数、状态/资源和生产者消费者尚未恢复 |
| [crates/systems/glade-border/src](../../tracking/module-progress/crates__systems__glade-border__src.md) | 未逐项评估 | 未盘点 | 3/3/3 | 0 | 索引/零散候选；完整系统范围尚未核对。 | 未登记完整模块验收。 | 完整文件/函数、状态/资源和生产者消费者尚未恢复 |
| [crates/systems/onboarding/src](../../tracking/module-progress/crates__systems__onboarding__src.md) | 未逐项评估 | 未盘点 | 17/20/29 | 0 | 索引/零散候选；完整系统范围尚未核对。 | 未登记完整模块验收。 | 完整文件/函数、状态/资源和生产者消费者尚未恢复 |
| [crates/systems/roof/src](../../tracking/module-progress/crates__systems__roof__src.md) | 部分接线 | 未盘点 | 26/26/24 | 22 | create/edit/delete/spatial/replay和类型/动画已接World；CPU铺瓦完整核心。 | 矩形33输入19024记录/圆形37输入11467记录历史报告；编辑/生命周期/World链各自范围独立。 | roof_trigger_volume实际几何、place_spires/place_support_beams、snow与瓦片音频模拟完整消费者 |
| [crates/systems/timberframe/src](../../tracking/module-progress/crates__systems__timberframe__src.md) | 未逐项评估 | 未盘点 | 4/4/4 | 0 | 索引/零散候选；完整系统范围尚未核对。 | 未登记完整模块验收。 | 完整文件/函数、状态/资源和生产者消费者尚未恢复 |
| [crates/systems/toolbar/src](../../tracking/module-progress/crates__systems__toolbar__src.md) | 部分候选实现 | 未盘点 | 37/38/49 | 1 | 索引/零散候选；完整系统范围尚未核对。 | 未登记完整模块验收。 | 完整文件/函数、状态/资源和生产者消费者尚未恢复 |
| [crates/systems/tree-tool/src](../../tracking/module-progress/crates__systems__tree-tool__src.md) | 部分草稿 | 未盘点 | 11/11/11 | 6 | 六个文件的命令/删除/回放/地形跟随/弯曲/预置树恢复草稿。 | reva/RNE反汇编交叉核对与语义草稿；未登记完整编译/原码差分/系统验收。 | PlaceTreeCmd载荷和TreeEdit跨系统判别值 |
| [crates/systems/wall-constructor/src](../../tracking/module-progress/crates__systems__wall-constructor__src.md) | 部分接线 | 未盘点 | 10/10/12 | 5 | construct_walls/WallConstructor与noise/seed/authority walker→96B砖输出已恢复并接World。 | 普通85/其他304限定入口及连续World链；冻结后新增源码不继承旧报告。 | 所有结构family和完整原资源消费者 |
| [crates/systems/wall-constructor/src/construct_arches](../../tracking/module-progress/crates__systems__wall-constructor__src__construct_arches.md) | 部分接线 | 未盘点 | 2/2/3 | 2 | construct_arches/profile；regular/freeform arch输出、final arch/third和120B ConstructGateCmd | 仅所列局部/连续链报告；不继承其他family验收。 | 更多支撑族、门命令最终consumer |
| [crates/systems/wall-constructor/src/construct_elevation_supports](../../tracking/module-progress/crates__systems__wall-constructor__src__construct_elevation_supports.md) | 部分分析 | 未盘点 | 13/13/13 | 0 | construct_brick_columns定位/局部helper候选 | 仅所列局部/连续链报告；不继承其他family验收。 | 圆/矩形柱、托架、turret、碰撞完整family |
| [crates/systems/wall-constructor/src/construct_fences](../../tracking/module-progress/crates__systems__wall-constructor__src__construct_fences.md) | 部分候选实现 | 未盘点 | 2/2/2 | 1 | construct_fences/process_segment部分候选 | 仅所列局部/连续链报告；不继承其他family验收。 | 完整floor/fence消费链及新代码验收 |
| [crates/systems/wall-constructor/src/construct_gates](../../tracking/module-progress/crates__systems__wall-constructor__src__construct_gates.md) | 部分候选实现 | 未盘点 | 2/2/2 | 1 | Gate120→CPU木门/支撑/post fallback、候选mesh/entity/Nani生命周期 | 仅所列局部/连续链报告；不继承其他family验收。 | 新增整批未统一验收；原MeshLib/ShaderLib/SSBO所有权 |
| [crates/systems/wall-constructor/src/construct_lintels](../../tracking/module-progress/crates__systems__wall-constructor__src__construct_lintels.md) | 部分接线 | 未盘点 | 3/3/3 | 1 | construct_lintels/place_support_beams限定CPU链 | 仅所列局部/连续链报告；不继承其他family验收。 | 完整梁/悬挑/资源生命周期 |
| [crates/systems/wall-constructor/src/construct_platform_stairs](../../tracking/module-progress/crates__systems__wall-constructor__src__construct_platform_stairs.md) | 部分接线 | 未盘点 | 1/1/6 | 4 | 80B装饰源→cells/碰撞/96B记录/holes/stats | 仅所列局部/连续链报告；不继承其他family验收。 | 扩大consumer输入边界；新Rapier接线整批未验收 |
| [crates/systems/wall-constructor/src/construct_playermade_stairs](../../tracking/module-progress/crates__systems__wall-constructor__src__construct_playermade_stairs.md) | 未逐项评估 | 未盘点 | 0/5/6 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/systems/wall-constructor/src/construct_wooden_walls](../../tracking/module-progress/crates__systems__wall-constructor__src__construct_wooden_walls.md) | 未逐项评估 | 未盘点 | 0/0/4 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/systems/wall-constructor/src/curved_halftimber](../../tracking/module-progress/crates__systems__wall-constructor__src__curved_halftimber.md) | 未逐项评估 | 未盘点 | 0/2/2 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/systems/wall-constructor/src/segmentation](../../tracking/module-progress/crates__systems__wall-constructor__src__segmentation.md) | 部分接线 | 未盘点 | 4/4/4 | 3 | calculate_wall_path_segmentation；sample_wall_path_intersections；split_wall_path_segment_into_platform_stairs | 仅所列局部/连续链报告；不继承其他family验收。 | 完整路径mask/material-stat已验限定范围；更多形状/消费者 |
| [crates/systems/wall-constructor/src/utils](../../tracking/module-progress/crates__systems__wall-constructor__src__utils.md) | 部分候选实现 | 未盘点 | 9/9/9 | 1 | resolve_hole_overlap、trim_rows、split_into_random_rows限定策略 | 仅所列局部/连续链报告；不继承其他family验收。 | wall_corners/wall_skirts、half_timber与全部helper来源归属 |
| [crates/systems/water/src](../../tracking/module-progress/crates__systems__water__src.md) | 部分候选实现 | 未盘点 | 4/4/4 | 2 | WaterRaster CPU状态、world_raster/replay/上传调用、water_audio_grid。 | 五模块README记录候选9测试；没有完整水域原作运行等价。 | zeno闭包内部、mask SSBO字段和完整draw/history producer |
| [crates/tabloid/src](../../tracking/module-progress/crates__tabloid__src.md) | 部分候选实现 | 3/3/0 | 1/1/1 | 1 | 发布、订阅、共享增量批次及reader生命周期；三主要接口有业务恢复。 | 3个原锁合同测试、严格Clippy；Default/Drop/Deref内联接口单列复核。 | 补内联接口对照并对接真实生产者 |
| [crates/tg-gpu-selector/src](../../tracking/module-progress/crates__tg-gpu-selector__src.md) | 部分候选实现 | 未盘点 | 3/3/3 | 5 | B游戏私有设备选择/driver建议/graphics错误策略及Vulkan allowlist六项workaround状态，未重建枚举/驱动库。 | 共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。 | rhapsody真实AvailableGraphicsDevice/driver_support生产者；opaque allowlist u32字段语义和实际device创建消费；原GUI像素与A/C适配 |
| [crates/tg-iced/src](../../tracking/module-progress/crates__tg-iced__src.md) | 部分候选实现 | 未盘点 | 3/3/4 | 5 | B手写对话框选项/结果与文本入口；真实原iced/winit/cosmic-text后端可运行，窗口接线为重建适配。 | 共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。 | 原GUI默认尺寸/调度频率/完整选项与精确widget样式；真实游戏启动消费者；A/C未适配 |
| [crates/tg-integrity-report/src](../../tracking/module-progress/crates__tg-integrity-report__src.md) | 部分候选实现 | 未盘点 | 2/2/2 | 3 | B报告三列表保持顺序，all-empty归一，单文件删除与状态/建议/UI联接。 | 共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。 | 原verify入口至实际启动报告消费者；完整原GUI样式/平台失败边界与A/C适配 |
| [crates/tg-mods/src](../../tracking/module-progress/crates__tg-mods__src.md) | 未逐项评估 | 未盘点 | 0/0/5 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |
| [crates/tg-package-integrity/src](../../tracking/module-progress/crates__tg-package-integrity__src.md) | 部分候选实现 | 未盘点 | 1/1/1 | 2 | B manifest原bytes校验→schema/version→逐文件missing/corrupt→递归extra→聚合候选链，原依赖复用。 | 既有4候选tests/真实manifest/disk检查；独立review原x64 FNV/数组Hash叶4096+128组合gate零差异，跨批/Unicode/junction探针通过。无确认业务bug/造轮子；64批线程创建与全文件读取成本已测，未证明原性能回退。完整verify/罕见schema/report consumer未验收。 | 接真实本地report消费者，再定点核实罕见schema与release性能；哈希/线程池继续复用依赖 |
| [crates/tg-text/src](../../tracking/module-progress/crates__tg-text__src.md) | 部分候选实现 | 未盘点 | 3/3/3 | 8 | B FontTable与ShapeStream/RawShapeStream CPU配置→lookup/scaled、builder→resolver→原u32 words子链；lib.rs仅模块glue。 | 既有6tests/51原append+21原metrics；独立review未发现CPU代码/性能/造轮子缺陷。原PDB String::deserialize→RON deserialize_string链纠正旧同址标签误读；当前仅注释/README修正，旧证明hash保留。候选请求/接受集合/panic/分配复用探针完成；完整text/glyph/GPU与原serde未验。 | 沿真实TextFragment consumers恢复attrs/metrics/prepare并复用原cosmic-text/etagere |
| [crates/theme-defs/src](../../tracking/module-progress/crates__theme-defs__src.md) | 部分候选实现 | 0/0/0 | 1/1/1 | 1 | B活动/错误传输合同完整静态盘点；作者精确未发射名称/Display保留未知。 | 2026-10-14实际三库26离线测试/严格Clippy/fmt通过；4个clone/eq内联合同，std/ArcSwap胶水排除。 | 原完整入口与库外plugin/内容消费验收；A/C另验。 |
| [crates/theme-fetcher/src](../../tracking/module-progress/crates__theme-fetcher__src.md) | 部分候选实现 | 未盘点 | 2/2/2 | 2 | B库内HTTP/CAS/ZIP/活动发布静态补全，含真实处理顺序/时钟/mtime/清单写入与21处原日志。 | 2026-10-14实际三库26离线测试/严格all-targets Clippy/fmt通过；[当前报告](../../evidence/theme-static-2026-10-14/REPORT.md)。无服务请求。 | 完整原入口oracle和库外主题plugin/内容接线；A/C另验。 |
| [crates/theme-server-proto/src](../../tracking/module-progress/crates__theme-server-proto__src.md) | 部分候选实现 | 未盘点 | 1/1/1 | 1 | B ActiveEvent/EventsResponse字段、serde对象/序列/required/duplicate/unknown合同完整静态复核。 | 2026-10-14实际三库26离线测试/严格Clippy/fmt通过；7public均visitor/Drop胶水，无独立手写校验漏项。 | 原完整服务兼容验收与库外主题plugin；未发射API/A/C未覆盖。 |
| [crates/tiny-glade/src](../../tracking/module-progress/crates__tiny-glade__src.md) | 未逐项评估 | 未盘点 | 40/45/58 | 0 | 仅登记原源码结构；局部调用或适配不等于本crate实现。 | 未登记完整crate验证；源码索引不算恢复。 | 逐文件/函数恢复范围、类型和实际生产消费链尚未建立 |
| [crates/turbosloth/src](../../tracking/module-progress/crates__turbosloth__src.md) | 依赖/归属边界 | 未盘点 | 0/0/7 | 0 | 先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。 | 依赖实现不计游戏逆向覆盖。 | 来源/必要分叉差异待核实，公共实现复用 |
| [crates/usdl-sys/src](../../tracking/module-progress/crates__usdl-sys__src.md) | 依赖/归属边界 | 未盘点 | 0/0/9 | 0 | 先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。 | 依赖实现不计游戏逆向覆盖。 | 来源/必要分叉差异待核实，公共实现复用 |
| [crates/usdl/src](../../tracking/module-progress/crates__usdl__src.md) | 依赖/归属边界 | 未盘点 | 0/0/7 | 0 | 先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。 | 依赖实现不计游戏逆向覆盖。 | 来源/必要分叉差异待核实，公共实现复用 |
| [crates/utils/src](../../tracking/module-progress/crates__utils__src.md) | 部分候选实现 | 135/99/36 | 20/23/23 | 34 | 22顶层文件候选：camera/RDP/Spline/Timer/区间等已对照并修复；原PDB内联接口仍有遗漏。 | 全utils37测试、strict Clippy、12候选JSON场景通过；新增916原码记录、继承367记录；不等于全部原接口验收。 | 按原函数接口库存优先补未触及工作 |
| [crates/utils/src/components](../../tracking/module-progress/crates__utils__src__components.md) | 部分候选实现 | 11/2/9 | 1/1/1 | 2 | B原接口台账：11项已观察，2项已有部分业务分析，9项未触及；具体状态见逐接口证据。 | 原指令对照覆盖指定实例与输入；静态候选、归属待定项及原消费者链分别记录，不能据此验收整个目录。 | 按原接口缺口补充业务分析、复用原依赖实现并对照修正。 |
| [crates/utils/src/curve](../../tracking/module-progress/crates__utils__src__curve.md) | 部分候选实现 | 115/29/86 | 4/4/4 | 6 | B原接口台账：115项已观察，29项已有部分业务分析，86项未触及；具体状态见逐接口证据。 | 原指令对照覆盖指定实例与输入；静态候选、归属待定项及原消费者链分别记录，不能据此验收整个目录。 | 按原接口缺口补充业务分析、复用原依赖实现并对照修正。 |
| [crates/utils/src/geometry](../../tracking/module-progress/crates__utils__src__geometry.md) | 部分候选实现 | 151/97/54 | 13/14/15 | 22 | B原接口台账：151项已观察，97项已有部分业务分析，54项未触及；具体状态见逐接口证据。 | 原指令对照覆盖指定实例与输入；静态候选、归属待定项及原消费者链分别记录，不能据此验收整个目录。 | 按原接口缺口补充业务分析、复用原依赖实现并对照修正。 |
| [shader-compiler](../../tracking/module-progress/shader-compiler.md) | 部分接线 | 未盘点 | 0/0/0 | 5 | B包名已知，原源码目录未在runtime PDB定位；原638请求、include/chunk/宏拼接、实际DXC、精确public反射和后处理/打包策略接入现有工程。 | 404新包/638stage实际构建有效；393官方1.9、11官方preview1.10；新metadata210/404与原一致。最终10测试及严格all-targets Clippy、新fill真实GPU读回通过；不代表原shader全等。 | 恢复原compute macro/provider/config，修metadata与算法残差；精确验证所需compiler fork差异 |
| [target](../../tracking/module-progress/target.md) | 构建元数据 | 未盘点 | 0/0/8 | 0 | 仅有源码清单或候选交付；完整模块行为尚未评估 | 未建立模块/文件级验收；候选交付不等于原作等价 | 先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向 |

文件数量只是库存，不用于计算完成率；同一恢复文件可服务多个原模块，引用数不可相加。

## HLSL源码层

独立HLSL源码层：404/404原bin有对应重建main；260main/共享include及大算法族已重建。对应覆盖不等于算法验收。

本轮实际DXC构建404包/638stage全部生成：393官方1.9+11官方preview1.10；新完整metadata210/404、stage公共接口302/638、cap617/638、非debugopcode histogram100/638与原一致。原metadata再反射404零差异，不等于源码shader算法等价。

剩余：170compute原macro tuple未独证，其中38非零key未知；234raster tuple有原depth-only=0/1证据；新metadata194包/公开stage反射336项不同；191依赖集合不同，需逐项原消费者核实；原DXC/fork部署身份未知，provider/error/cache/temp行为尚有候选；Shader源码→游戏pass→pipeline/descriptor→真实整帧/画面对照

[Shader逐文件、入口、函数与验证报告](../../tracking/module-progress/shaders.md)。公开算法包含文件只登记依赖边界，不计私有恢复。

## 证据当前性

| 报告 | 显式路径哈希检查 | 报告存在 |
|---|---|---|
| [evidence/roof-surface/verification.json](../../evidence/roof-surface/verification.json) | explicit_path_hashes_matched | True |
| [evidence/roof-ridge/verification.json](../../evidence/roof-ridge/verification.json) | not_recorded | True |
| [evidence/multi-version/B-parameters-verification.json](../../evidence/multi-version/B-parameters-verification.json) | historical_inputs_changed | True |
| [evidence/scope-priorities/wall-rules-verification.json](../../evidence/scope-priorities/wall-rules-verification.json) | explicit_path_hashes_matched | True |
| [evidence/building-holes/storage-B-verification.json](../../evidence/building-holes/storage-B-verification.json) | historical_inputs_changed | True |
| [evidence/building-placement/B-subtype-verification.json](../../evidence/building-placement/B-subtype-verification.json) | historical_inputs_changed | True |
| [evidence/building-decorator-info/findings.md](../../evidence/building-decorator-info/findings.md) | not_recorded | True |
| [evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md](../../evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md) | not_recorded | True |
| [evidence/building-integrate/stage-reviews/stage4-frozen/summary.json](../../evidence/building-integrate/stage-reviews/stage4-frozen/summary.json) | historical_inputs_changed | True |
| [evidence/building-wall-history/findings.md](../../evidence/building-wall-history/findings.md) | not_recorded | True |
| [evidence/multi-version/history-rust-verification.json](../../evidence/multi-version/history-rust-verification.json) | historical_inputs_changed | True |
| [evidence/building-history/B-transaction-verification.json](../../evidence/building-history/B-transaction-verification.json) | historical_inputs_changed | True |
| [evidence/building-history-owners/B-owner-verification.json](../../evidence/building-history-owners/B-owner-verification.json) | historical_inputs_changed | True |
| [evidence/multi-version/B-replay-verification.json](../../evidence/multi-version/B-replay-verification.json) | historical_inputs_changed | True |
| [evidence/terrain-editing/milestones/M1/status.md](../../evidence/terrain-editing/milestones/M1/status.md) | not_recorded | True |
| [evidence/multi-baseline/README.md](../../evidence/multi-baseline/README.md) | not_recorded | True |
| [evidence/terrain-editing/second-batch-findings.md](../../evidence/terrain-editing/second-batch-findings.md) | not_recorded | True |
| [evidence/terrain-editing/B-delta-edit-verification.json](../../evidence/terrain-editing/B-delta-edit-verification.json) | not_recorded | True |
| [tracking/parallel-modules-stopped-snapshot.json](../../tracking/parallel-modules-stopped-snapshot.json) | historical_inputs_changed | True |
| [evidence/parallel-camera/B-constructor-native.json](../../evidence/parallel-camera/B-constructor-native.json) | not_recorded | True |
| [evidence/multi-baseline/compile-clutter-audio.json](../../evidence/multi-baseline/compile-clutter-audio.json) | not_recorded | True |
| [evidence/multi-baseline/audio-events-analysis.md](../../evidence/multi-baseline/audio-events-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/audio-extra-analysis.md](../../evidence/multi-baseline/audio-extra-analysis.md) | not_recorded | True |
| PROJECT_MEMORY.md | not_recorded | True |
| [reconstruction/country-core-render/_integration/INTEGRATION_REPORT.md](../../reconstruction/country-core-render/_integration/INTEGRATION_REPORT.md) | not_recorded | True |
| [evidence/render-passes/INDEX.md](../../evidence/render-passes/INDEX.md) | not_recorded | True |
| [evidence/building-integrate/findings.md](../../evidence/building-integrate/findings.md) | not_recorded | True |
| [evidence/render-recovery/consumer-chain-review-2026-10-12/README.md](../../evidence/render-recovery/consumer-chain-review-2026-10-12/README.md) | not_recorded | True |
| [evidence/roof-pipeline-verification.json](../../evidence/roof-pipeline-verification.json) | historical_inputs_changed | True |
| [evidence/roof-tiles/circular-verification.json](../../evidence/roof-tiles/circular-verification.json) | partly_unresolved | True |
| [evidence/source-edits/verification.json](../../evidence/source-edits/verification.json) | historical_inputs_changed | True |
| [evidence/building-constructor/B-wall-constructor-verification.json](../../evidence/building-constructor/B-wall-constructor-verification.json) | historical_inputs_changed | True |
| [evidence/building-production-authority/findings.md](../../evidence/building-production-authority/findings.md) | not_recorded | True |
| [evidence/building-segmentation/findings.md](../../evidence/building-segmentation/findings.md) | not_recorded | True |
| [evidence/building-arches-freehand/findings.md](../../evidence/building-arches-freehand/findings.md) | not_recorded | True |
| [evidence/building-structures/findings.md](../../evidence/building-structures/findings.md) | not_recorded | True |
| [evidence/building-platform-stairs/B-rapier-backend-verification.json](../../evidence/building-platform-stairs/B-rapier-backend-verification.json) | historical_inputs_changed | True |
| [evidence/building-gates/findings.md](../../evidence/building-gates/findings.md) | not_recorded | True |
| [evidence/building-fences/findings.md](../../evidence/building-fences/findings.md) | not_recorded | True |
| [evidence/building-placement/B-placement-verification.json](../../evidence/building-placement/B-placement-verification.json) | historical_inputs_changed | True |
| [evidence/multi-baseline/tree-process-cmds-analysis.md](../../evidence/multi-baseline/tree-process-cmds-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/tree-tool-rest-analysis.md](../../evidence/multi-baseline/tree-tool-rest-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/tree-tool-extra-analysis.md](../../evidence/multi-baseline/tree-tool-extra-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/water-cpu-analysis.md](../../evidence/multi-baseline/water-cpu-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/water-extra-analysis.md](../../evidence/multi-baseline/water-extra-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/critters-behavior-analysis.md](../../evidence/multi-baseline/critters-behavior-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/critters-extra-analysis.md](../../evidence/multi-baseline/critters-extra-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/clutter-placement-analysis.md](../../evidence/multi-baseline/clutter-placement-analysis.md) | not_recorded | True |
| [evidence/multi-baseline/clutter-extra-analysis.md](../../evidence/multi-baseline/clutter-extra-analysis.md) | not_recorded | True |
| [reconstruction/rhapsody-rebuild/MODULE_STATUS.md](../../reconstruction/rhapsody-rebuild/MODULE_STATUS.md) | not_recorded | True |
| [reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json](../../reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json) | historical_inputs_changed | True |
| [reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json](../../reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json) | historical_inputs_changed | True |
| [reconstruction/rhapsody-rebuild/evidence/raw-resource-implementation.json](../../reconstruction/rhapsody-rebuild/evidence/raw-resource-implementation.json) | historical_inputs_changed | True |
| [reconstruction/rhapsody-rebuild/evidence/shader-production-validation.json](../../reconstruction/rhapsody-rebuild/evidence/shader-production-validation.json) | historical_inputs_changed | True |
| [reconstruction/rhapsody-rebuild/evidence/swapchain-functional-validation.json](../../reconstruction/rhapsody-rebuild/evidence/swapchain-functional-validation.json) | historical_inputs_changed | True |
| [evidence/shader-toolchain-2026-10-13/integration.json](../../evidence/shader-toolchain-2026-10-13/integration.json) | historical_inputs_changed | True |
| [evidence/shader-toolchain-2026-10-13/runtime/runtime-contract.json](../../evidence/shader-toolchain-2026-10-13/runtime/runtime-contract.json) | partly_unresolved | True |
| [reconstruction/rhapsody-rebuild/evidence/managed-implementation.json](../../reconstruction/rhapsody-rebuild/evidence/managed-implementation.json) | historical_inputs_changed | True |
| [reconstruction/rhapsody-rebuild/evidence/pass-bindings-functional-validation.json](../../reconstruction/rhapsody-rebuild/evidence/pass-bindings-functional-validation.json) | historical_inputs_changed | True |
| [evidence/render-recovery/verification.json](../../evidence/render-recovery/verification.json) | explicit_path_hashes_matched | True |
| [evidence/render-recovery/consumer-chain-review-2026-10-12/context-entry-validation.json](../../evidence/render-recovery/consumer-chain-review-2026-10-12/context-entry-validation.json) | explicit_path_hashes_matched | True |
| [reconstruction/rhapsody-rebuild/evidence/queue-graph-functional-validation.json](../../reconstruction/rhapsody-rebuild/evidence/queue-graph-functional-validation.json) | historical_inputs_changed | True |
| [reconstruction/rhapsody-rebuild/evidence/temporal-functional-validation.json](../../reconstruction/rhapsody-rebuild/evidence/temporal-functional-validation.json) | historical_inputs_changed | True |
| [evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json) | historical_inputs_changed | True |
| [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/utils-tabloid/native-manifest.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/native-manifest.json) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/integration/dependency-validation.json](../../evidence/shared-foundation-2026-10-13/integration/dependency-validation.json) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/PARALLEL_PLAN.md](../../evidence/shared-foundation-2026-10-13/PARALLEL_PLAN.md) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/utils-base/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-base/delivery.json) | partly_unresolved | True |
| [evidence/shared-foundation-2026-10-13/utils-base/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-base/CONTRACT.md) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/utils-base/verification.json](../../evidence/shared-foundation-2026-10-13/utils-base/verification.json) | partly_unresolved | True |
| [evidence/shared-foundation-review-2026-10-13/REVIEW.md](../../evidence/shared-foundation-review-2026-10-13/REVIEW.md) | not_recorded | True |
| [tracking/shared-foundation-code-review.json](../../tracking/shared-foundation-code-review.json) | historical_inputs_changed | True |
| [evidence/shared-foundation-review-2026-10-13/integration-validation.json](../../evidence/shared-foundation-review-2026-10-13/integration-validation.json) | historical_inputs_changed | True |
| [FOUNDATION_BUSINESS_COMPARISON.md](../reference/FOUNDATION_BUSINESS_COMPARISON.md) | not_recorded | True |
| [tracking/shared-foundation-business-comparison.json](../../tracking/shared-foundation-business-comparison.json) | historical_inputs_changed | True |
| [evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json) | explicit_path_hashes_matched | True |
| [tracking/shared-foundation-interfaces.json](../../tracking/shared-foundation-interfaces.json) | historical_inputs_changed | True |
| [evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/coverage-delta.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/coverage-delta.json) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json) | explicit_path_hashes_matched | True |
| [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/assets-small-first/cargo-test.log](../../evidence/shared-foundation-2026-10-13/assets-small-first/cargo-test.log) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/assets-small-first/clippy.log](../../evidence/shared-foundation-2026-10-13/assets-small-first/clippy.log) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/assets-small-first/gamma-verification.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/gamma-verification.json) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/assets-small-first/bounded-report.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/bounded-report.json) | partly_unresolved | True |
| [evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json) | explicit_path_hashes_matched | True |
| [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) | explicit_path_hashes_matched | True |
| [evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md](../../evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/utils-tabloid/tabloid-test.log](../../evidence/shared-foundation-2026-10-13/utils-tabloid/tabloid-test.log) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/utils-tabloid/tabloid-clippy.log](../../evidence/shared-foundation-2026-10-13/utils-tabloid/tabloid-clippy.log) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/lang-small/inventory.json](../../evidence/shared-foundation-2026-10-13/lang-small/inventory.json) | explicit_path_hashes_matched | True |
| [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json) | historical_inputs_changed | True |
| [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) | partly_unresolved | True |
| [evidence/shared-foundation-2026-10-13/lang-small/locale-policy.log](../../evidence/shared-foundation-2026-10-13/lang-small/locale-policy.log) | not_recorded | True |
| [evidence/shared-foundation-2026-10-13/lang-small/original-assets.log](../../evidence/shared-foundation-2026-10-13/lang-small/original-assets.log) | not_recorded | True |
| [evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md) | not_recorded | True |
| [evidence/small-libs-batch-01/tg-text/native-oracle.json](../../evidence/small-libs-batch-01/tg-text/native-oracle.json) | not_recorded | True |
| [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) | explicit_path_hashes_matched | True |
| [evidence/small-libs-batch-01/integration/dependency-validation.json](../../evidence/small-libs-batch-01/integration/dependency-validation.json) | not_recorded | True |
| [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json) | historical_inputs_changed | True |
| [evidence/small-libs-batch-01/tg-text/interface-map.json](../../evidence/small-libs-batch-01/tg-text/interface-map.json) | not_recorded | True |
| [evidence/small-libs-review-2026-10-13/tg-text/REVIEW.md](../../evidence/small-libs-review-2026-10-13/tg-text/REVIEW.md) | not_recorded | True |
| [evidence/small-libs-review-2026-10-13/tg-text/findings.json](../../evidence/small-libs-review-2026-10-13/tg-text/findings.json) | not_recorded | True |
| [evidence/small-libs-review-2026-10-13/integration/summary.json](../../evidence/small-libs-review-2026-10-13/integration/summary.json) | historical_inputs_changed | True |
| [evidence/small-libs-review-2026-10-13/integration/ERRATA.md](../../evidence/small-libs-review-2026-10-13/integration/ERRATA.md) | not_recorded | True |
| [DEPENDENCY_FILTERING.md](../current/DEPENDENCY_FILTERING.md) | not_recorded | True |
| [evidence/shader-build-2026-10-13/pipeline/validation.json](../../evidence/shader-build-2026-10-13/pipeline/validation.json) | historical_inputs_changed | True |
| [evidence/shader-build-2026-10-13/provenance/package-classification.json](../../evidence/shader-build-2026-10-13/provenance/package-classification.json) | partly_unresolved | True |
| [evidence/shader-build-2026-10-13/dxc/merged-audit.json](../../evidence/shader-build-2026-10-13/dxc/merged-audit.json) | explicit_path_hashes_matched | True |
| [evidence/shader-toolchain-2026-10-13/pipeline/validation.json](../../evidence/shader-toolchain-2026-10-13/pipeline/validation.json) | historical_inputs_changed | True |
| [evidence/shader-toolchain-2026-10-13/pipeline/delivery.json](../../evidence/shader-toolchain-2026-10-13/pipeline/delivery.json) | historical_inputs_changed | True |
| [evidence/shader-build-2026-10-13/integration.json](../../evidence/shader-build-2026-10-13/integration.json) | historical_inputs_changed | True |
| [evidence/shader-build-2026-10-13/dxc/validation.json](../../evidence/shader-build-2026-10-13/dxc/validation.json) | historical_inputs_changed | True |
| [evidence/shader-build-2026-10-13/dxc/delivery.json](../../evidence/shader-build-2026-10-13/dxc/delivery.json) | historical_inputs_changed | True |
| [evidence/shader-build-2026-10-13/provenance/PROVENANCE.md](../../evidence/shader-build-2026-10-13/provenance/PROVENANCE.md) | not_recorded | True |
| [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json) | not_recorded | True |
| [evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md) | not_recorded | True |
| [evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md](../../evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md) | not_recorded | True |
| [evidence/small-libs-review-2026-10-13/motomoto/findings.json](../../evidence/small-libs-review-2026-10-13/motomoto/findings.json) | not_recorded | True |
| [evidence/small-libs-batch-01/tg-package-integrity/delivery.json](../../evidence/small-libs-batch-01/tg-package-integrity/delivery.json) | partly_unresolved | True |
| [evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md) | not_recorded | True |
| [evidence/small-libs-batch-01/tg-package-integrity/DIFFERENCES.md](../../evidence/small-libs-batch-01/tg-package-integrity/DIFFERENCES.md) | not_recorded | True |
| [evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md](../../evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md) | not_recorded | True |
| [evidence/small-libs-review-2026-10-13/tg-package-integrity/findings.json](../../evidence/small-libs-review-2026-10-13/tg-package-integrity/findings.json) | not_recorded | True |
| [evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json) | not_recorded | True |
| [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) | not_recorded | True |
| [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json) | not_recorded | True |
| [evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json) | explicit_path_hashes_matched | True |
| [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json) | explicit_path_hashes_matched | True |
| [evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json) | not_recorded | True |
| [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) | explicit_path_hashes_matched | True |
| [evidence/roof-tiles/dependency-boundary.json](../../evidence/roof-tiles/dependency-boundary.json) | not_recorded | True |
| [evidence/shader-build-2026-10-13/pipeline/delivery.json](../../evidence/shader-build-2026-10-13/pipeline/delivery.json) | not_recorded | True |
| [evidence/shader-build-2026-10-13/provenance/versioned-pdb-source-boundaries.json](../../evidence/shader-build-2026-10-13/provenance/versioned-pdb-source-boundaries.json) | not_recorded | True |
| [evidence/roof-tiles/source-map.json](../../evidence/roof-tiles/source-map.json) | not_recorded | True |
| [evidence/native/roof-state-verification.json](../../evidence/native/roof-state-verification.json) | not_recorded | True |
| [evidence/native/window-aspect-verification.json](../../evidence/native/window-aspect-verification.json) | not_recorded | True |
| [evidence/render-recovery/findings.md](../../evidence/render-recovery/findings.md) | not_recorded | True |
| [evidence/render-passes/integration/wave2-final/completion.json](../../evidence/render-passes/integration/wave2-final/completion.json) | not_recorded | True |
| [evidence/render-passes/RECOVERY_TRACKER.md](../../evidence/render-passes/RECOVERY_TRACKER.md) | not_recorded | True |
| [evidence/shader-build-2026-10-13/SUMMARY.md](../../evidence/shader-build-2026-10-13/SUMMARY.md) | not_recorded | True |

## 待核对与本机缺失

- crates/feedback-reporter/src：missing_recorded_restoration，`reconstruction/shared-foundation/crates/feedback-reporter/examples/*.rs`。
- crates/theme-fetcher/src：missing_recorded_restoration，`reconstruction/shared-foundation/crates/theme-fetcher/examples/*.rs`。
- crates/theme-server-proto/src：missing_recorded_restoration，`reconstruction/shared-foundation/crates/theme-server-proto/examples/*.rs`。
- crates/geforcenow/src：missing_recorded_restoration，`reconstruction/shared-foundation/crates/geforcenow/examples/*.rs`。

机器可读全集：[module-progress.json](../../tracking/module-progress.json)。原路径来源：[version-map.json](../../tracking/version-map.json)。
