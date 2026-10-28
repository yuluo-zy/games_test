# Decorator / clutter 增量审计 2026-10-21 15:08

审计基线：父任务已同步 main 1f73dbc73，上一轮 f72938c81。只读当前生产源码及已提交日志；未执行 Cargo、测试、Git懒取或新逆向。

## 结论

本次是真正的算法恢复增量，但完整视觉/编辑主链仍未安装。不能继续用旧stair4/roof63 cache None结论；也不能把大量candidate路径和反编译材料算作生产落地。

| 模块 | 上次估计 | 当前建议 | 说明 |
|---|---:|---:|---|
| decorator总体 | 40–55% | 45–60% | cache关键大分支、Gothic主producer落地；UI/完整set25/unload仍未闭合 |
| window装饰 | 50–65% | 60–75% | Gothic主算法+hat实际合入；canonical ECS三源仍候选，宿主未挂 |
| stairs业务/视觉 | 30–45% | 35–50% | stair cache+canonical结构图已实现；main装配/扶手/graph编辑及lifecycle候选未落 |
| clutter/autoclutter | 45–60% | 45–60% | bush readback排序/哈希/缺buffer语义修复是正确性增量，主要业务缺口未变 |
| ivy | 45–60% | 45–60% | 无实质增量，仍缺主App增长/生成/GPU链安装 |
| door装饰 | 30–45% | 30–45% | 本轮生产无闭合，builder/附加记录未消费缺口仍在 |

均为无完整分母的保守业务恢复区间，中低置信度；不是可玩率，也不能平均为整项目进度。

## 本次已实际进入生产

1. `tiny-glade/crates/systems/decorator/src/cache_decorator_transforms.rs`：原九参真实使用stairs_requests与stairs；:88楼梯桶处理与即时modified；:125 first directed outgoing；:213 roof63调整出方向；:244平屋顶63真实Extra高度；:259地形63高度/方向；:316 compute_stair_transform查同顺序请求，不命中原负零/-100/零缩放sentinel；曲线/切线/插值、kind2台阶量化、kind3梯子端点调整/量化与横向宽度保留原required读顺序。旧None分支已消失。:590起有真实World tests覆盖先后序/错参/浮点边界。这里只读确认实现及既有日志，不作新的等价证明。
2. `.../decorator_visual/gothic_wall_window.rs:310`：四参10658B真实主producer；:153 get_gothic_hat_bones用真实墙曲线/坐标，不再以旧简式替代；完整帽/玻璃/框、deform bytes、color、3洞表、collision/interaction Arc、dormer-hole，required查表失败保留之前输出。注意:62的旧calc_gothic_half_dormer_profile_ws仍是简式；当前主producer未调用该helper，不能仅因旧helper存在判定新主算法仍简化，也不能宣称该独立helper完成。
3. `country-core/src/resources/stairs/structural_graph.rs:89`：一图六表；七原业务入口upsert_span(:122)、upsert_support_source(:147)、add_edge_by_span(:154)、add_edge_by_collider(:170)、remove(:196)、disconnect(:215)、clear(:111)。span明确含采样u32，永久/临时连接合并，替换/移除副作用顺序恢复。卸载integration fixture适配该canonicalAPI，生产完整卸载body无新增。
4. `systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs:63`：原raw u32词典排序（signed zero/NaN bits）；:71低32(hash XOR empty_hash)，空格初始0；:91真实SSBO缺注册提前返回、计数bounds失败顺序；真正mapped readback仍由原owner提供，无新假资源。不能把CPU映射fixture算GPU验收。

## 当前仍然开放，且未因研究目录新增关闭

- `tiny-glade/crates/tiny-glade/src/run_game.rs:409`仍仅`build_startup_resources`；`decorator_plugin.rs:71`候选视觉注册仅定义，未发现main调用；完整build定义及register_ui_systems仍未安装。该helper旧stair-cache阻断注释已过期，但完整generator/validator/flush/assembly顺序本身仍缺，不应直接挂旧helper冒充完整链。
- ui_move_decorator.rs:62 gate后只空转，:89楼梯放置候选恒None；ui_place_decorator.rs:48仅init/id，不是完整选点→Storage→历史→画面。
- decorator_visual/stairs.rs:338楼梯visual generator空；:781 maintain_stair_graph空；stairs_railings_assemble.rs:20起扶手四函数空。两个62KB楼梯装配main、Arch160/Bracket168载荷、Info63真实producer仍未完成。
- decorator_plugin.rs:49原完整卸载仍todo。lifecycle现有限owner reset未变；楼梯typed lifecycle六源、support-split3075B、必要mesh helper均在候选目录，未进入生产stairs_assemble.rs。报告详见 TG-347c9d16cc440d611d/.../STOP_HANDOFF_20261021.md:9、:15。
- Clutter region种类近似、candidate过滤/排序猜测、manager非Window selector/source explicit pending未变。
- Ivy主App仅owner/startup安装，缺增长/生成/剪枝/完整GPU上传链仍未变。

## 既有实际验证与边界

本次没有运行任何测试。已提交postpush日志显示：cache阶段core2843/1ignored、decorator392/2ignored、utils167通过；后Gothic+结构图阶段core2853/1ignored；Gothic首次400通过/2失败（旧fixture）有保留，最终tg-gothic-fixture-decorator-cpu-v3-20261021.log:1044为402通过/0失败/2ignored。STOP-AND-PUSH-20261021.md最新权威说明这些是合并前冻结身份，不能称当前merge后完整验收。

该快照同时明确原资产30帧在sampler V3通过、GameLoop验收13world present通过；完整进程入口最后loading4470/gameplay0，render_frame:303 device_ready失败。因此既不能继续说“从未GPU跑过”，也不能说已能正常进入可编辑游戏。
