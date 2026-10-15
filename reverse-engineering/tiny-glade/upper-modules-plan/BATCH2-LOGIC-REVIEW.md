# 批次 2 业务逻辑复核（critters / clutter / color / onboarding）

> 复核 agent 产出，2026-10-14。复核对象：批次 2 四份模块内部分析报告（`upper-modules-plan/systems/{critters,clutter,color,onboarding}/ANALYSIS.md`）。只复核模块内部实现，不展开 caller/callee 与跨 crate 接线。
> 方法：对照 `skeleton/B/crates/systems/<crate>/src/*.rs` 的 compiled_procedure 发射清单逐条比对；用 ghidra MCP（`--worker shared`，只读）对可疑/关键结论抽查反编译（update_firefly_goals、smooth_critter_velocities、random_vel_change、frame_update、tutorial_completion、hints_startup、reset_hints、hint_camera_progress_reset、exponential_back_off、color_menu、show_color_options、add_autoclutter_on_windows、update_butterfly_audio、duck_splash_particle、move_ducks_along_trajectories）。
> 标注沿用报告：**O**=观察 / **I**=推断 / **U**=未知。

## 1. 复核范围与结论摘要

- **覆盖核对**：四模块的 **全部 B 发射函数（compiled_procedure，本 crate 命名空间）均被各自 ANALYSIS.md 覆盖**，未发现业务函数遗漏，也未发现"多余"条目（报告中每条目都能对到发射清单的 RVA）。未被单独列举的只有派生胶水（`register_required_components`、`SystemParam::get_param`、`fmt`/`clone` derive），按项目约束不计独立业务，属合理省略；另有 color 两个发射闭包（`ui_color_wall_and_roof::closure$2`、`ui_color_gates::closure$1`）未逐条单独文档化，属轻微缺口。
- **逻辑清晰度**：四报告整体极高（逐函数输入→处理→输出、常量、布局均到字节级）。发现 **2 处确定的事实错误** 与 **1 处内部自相矛盾**（见 §4 冲突与需裁定项）：
  1. **critters firefly 惯性 bias**：报告（含 extra-analysis「校正」）称无目标应为 `-1.0(0xbf000000)`。实测 `0xbf000000 = -0.5`，「校正」方向反了，草稿 `-0.5` 才是对的。
  2. **onboarding reset_hints 阈值**：报告称写 `0.33/0.33/60`（0x3eaaaaab）。实测写 `0x3f333333/0x3f333333/0x42700000 = 0.7/0.7/60`，与 hints_startup 相同——并非第三套值。
  3. **onboarding §0.2 提示 id 11/12 归属互换**：报告称 id 11=CameraZoom、12=CameraRotate；实测（hints_startup 的进度写入 + hint_camera_progress_reset 的 id→轴分派）为 id 11=CameraRotate、12=CameraZoom。报告自己的 §7.2/§8.1 也是按 0xb=rotate、0xc=zoom 用的，与 §0.2 自相矛盾。
- **算法标注**：全部函数算法已标注，且均能对应到已知算法名（Reynolds Boids、EMA 指数平滑、hashbrown SwissTable、driftsort、指数退避、径向菜单布局等）。有现成实现（`reconstruction/critters/`、core `recovered_*`、`utils`）处均正确标注「复用/候选」而非重新推导。
- **unknown 合理性**：unknown 清单普遍合理且克制（标注 O/I/U）。确定错误处恰好表现为**过度断言**（把错值当 O 写出），而非把 unknown 当结论。

## 2. 各模块覆盖核对表

### 2.1 critters（54 业务 / 1 胶水；发射 55 条 ns 记录 + 2 非 ns）

发射清单来源 `skeleton/B/crates/systems/critters/src/*.rs`（55 条 ns compiled_procedure；另有 lib.rs 内 `core::str::pattern::TwoWaySearcher` 非 ns）。ANALYSIS.md 逐条覆盖。

| B 发射函数（VA） | 报告覆盖 | 逻辑清楚 | 算法标注 | unknown 合理 | 备注 |
|---|---|---|---|---|---|
| Boid::random_vel_change (0x1dc2170) | ✓ | ✓（timer/dir/旋转全到字节） | ✓ 锥采样+quat arc+时间连续 lerp | ✓ | Boid 布局注释自相矛盾（见 §4-F） |
| Boid::frame_update<fish> (0x1dc2740) | ✓ | ✓ | ✓ Reynolds Boids 三规则+速度约束+反射 | ✓ | 同向门描述略粗（见 §4-G） |
| Boid::frame_update<wren> (0x1dc3510) | ✓ | ✓ | 同上+射线可行 | ✓ | |
| BoidFlock::remove<fish>/<wren> (0x1dc2340/0x1dc2540) | ✓ | ✓ | ✓ swap_remove 批删 | ✓ | |
| critters_avoid_collisions (0x23179a0) | ✓ | ✓ | ✓ 线段相交+circle+spherecast+空间哈希 | ✓ | |
| smooth_critter_velocities (0x2318c10) | ✓ | ✓（抽查确认 powf+weight 写回+Multiplier=Query+0x14） | ✓ 指数衰减 EMA | ✓ | NaN 符号位为既有差分失败项（§4-B） |
| move_critters (0x2318fa0) | ✓ | ✓ | ✓ dt 积分+nlerp 朝向+球/墙/水避让 | ✓ | |
| plan_duck_trajectories (0x230ff00) | ✓ | ✓ | ✓ 黄金螺旋+射线寻路+计划修剪 | ✓ | |
| move_ducks_along_trajectories (0x2312200) | ✓ | ✓（抽查确认大函数结构） | ✓ 曲线参数化+Frenet 帧 | ✓ | |
| update_duck_goals (0x2313b30) | ✓ | ✓ | ✓ 计时状态机+盘面采样+有效性校验 | ✓ | |
| set_duck_velocities_towards_goals (0x2314470) | ✓ | ✓ | ✓ 归一化+冻结门 | ✓ | |
| update_duck_ripples (0x2314b70) | ✓ | ✓ | ✓ 指数衰减+SSBO 上传 | ✓ | |
| DuckSpawnLocations::new (0xf8ee90) | ✓ | ✓ | ✓ 10×10 均匀网格 | ✓ | |
| spawn_ducks (0xf8f210) | ✓ | ✓ | ✓ 冷却计时+上限+黄金螺旋 | ✓ | |
| despawn_ducks (0xf8f8b0) | ✓ | ✓ | ✓ 水面越界→离场计划 | ✓ | |
| duck_splash_particle (0x2322650) | ✓ | ✓（抽查确认事件驱动+spawn 组件） | ✓ 事件驱动粒子 | ✓ | |
| ducks_react_to_being_inside_buildings (0x231b140) | ✓ | ✓ | ✓ 包络判定+音效 | ✓ | |
| FishSpawnLocations::new (0xf909f0) | ✓ | ✓ | ✓ 25×25 网格 | ✓ | |
| despawn_fish (0xf90c10) | ✓ | ✓ | ✓ 资源清理 | ✓ | |
| fish (0xf90cd0) | ✓ | ✓ | ✓ boids+出生/清理+避让+逃避 | ✓ | |
| update_butterfly_states (0x1d84d40) | ✓ | ✓ | ✓ 跳表 FSM+距离门控+相位计时 | ✓ | |
| update_butterfly_goals (0x1d86060) | ✓ | ✓ | ✓ 球面采样+状态偏置+半径校验 | ✓ | |
| set_butterfly_velocities_towards_goals (0x1d867b0) | ✓ | ✓ | ✓ 归一化+跳表动画分派 | ✓ | 查询含 Sheep 疑误解析（未知合理） |
| update_butterfly_audio (0x1d86eb0) | ✓ | ✓（抽查确认逐蝶 set_position） | ✓ | ✓ | |
| butterflies_send_to_gpu (0x1d87140) | ✓ | ✓ | ✓ SSBO cpu_write | ✓ | |
| fireflies_send_to_gpu (0x2316760) | ✓ | ✓ | ✓ SSBO cpu_write | ✓ | |
| update_firefly_goals (0x2316940) | ✓（逻辑对） | bias 常量值**错** | ✓ 球面采样+惯性 bias | 该项应归 unknown | **§4-A：-1.0 应为 -0.5** |
| set_firefly_velocities_towards_goals (0x2317040) | ✓ | ✓ | ✓ 归一化+海拔下限 | ✓ | |
| sim_wren_boids (0x2326a80) | ✓ | ✓ | ✓ boids+出生/清理+树摇爆发 | ✓ | |
| sim_wren_boids::closure$3 (0x1dc42f0) | ✓ | ✓ | ✓ RAY_DIRS 射线谓词 | ✓ | |
| update_decorator_bird_spots (0x231c7d0) | ✓ | ✓ | ✓ HashMap 栖点生命周期 | ✓ | |
| update_roof_bird_spots (0x231d6c0) | ✓ | ✓ | ✓ 屋顶几何取样+random_splits | ✓ | |
| update_wrens (0x231e390) | ✓ | ✓ | ✓ 状态跳表+放置矩阵+调试可视化 | ✓ | |
| update_sheep_goals (0x2315050) | ✓ | ✓ | ✓ 跟随状态机+盘面漫游 | ✓ | |
| set_sheep_velocities_towards_goals (0x2315e40) | ✓ | ✓ | ✓ 归一化+pet 门控 | ✓ | |
| update_sheep_audio (0x2316490) | ✓ | ✓ | ✓ | ✓ | |
| sheep_walk_on_terrain (0x231bd80) | ✓ | ✓ | ✓ 弹簧-阻尼跟随状态机 | ✓ | 复用 reconstruction/critters 标注正确 |
| sheep_unstuck (0x23246b0) | ✓ | ✓ | ✓ 卡住检测（射线/掩码/水格） | ✓ | 解卡公式 unknown 合理 |
| react_to_being_inside_buildings (0x231f610) | ✓ | ✓ | ✓ 包络判定+emoji 生成 | ✓ | |
| sheep_reaction_emoji (0x231ff70) | ✓ | ✓ | ✓ 缓动+世界→屏幕投影 | ✓ | |
| sheep_floaties (0x2320cf0) | ✓ | ✓ | ✓ 附件+入水音效 | ✓ | |
| sheep_umbrellas (0x2321980) | ✓ | ✓ | ✓ 附件+雨天 | ✓ | |
| sheep_walk_animation (0x231a630) | ✓ | ✓ | ✓ 动画跳表 | ✓ | |
| sheep_pet_animation (0x231ac20) | ✓ | ✓（候选实现对照五相位） | ✓ 五相位 FSM | ✓ | |
| sheep_input_record_replay (0x231b720) | ✓ | ✓ | ✓ 输入帧读写 | ✓ | |
| ui_pet_sheep (0x2324f80)+closure$0/$1 (0x2326170/0x2326340) | ✓ | ✓ | ✓ UI 命中 | ✓ | |
| ui_pet_ducks (0x2322e30)+closure$0/$1 (0x2324290/0x2324460) | ✓ | ✓ | ✓ UI 命中 | ✓ | |
| system_critters::impl$0::build (0x67d440) | ✓ | ✓ | ✓ 插件装配 | ✓ | |
| sheep::impl$0::build (0x67d810) | ✓ | ✓ | ✓ 插件装配 | ✓ | |
| sheep_react…::impl$0::register_required_components (0x2320480) | 未单列 | 胶水 | — | — | 派生胶水，不计业务（合理省略） |

### 2.2 clutter（36 业务 / 2 胶水；发射 38 条 ns + 5 非 ns serde/btree/TwoWaySearcher）

| B 发射函数（VA） | 报告覆盖 | 逻辑清楚 | 算法标注 | unknown 合理 | 备注 |
|---|---|---|---|---|---|
| AutoClutterState::add_region (0xee3fd0) | ✓ | ✓ | ✓ hashbrown SwissTable+AABB→lazy grid | ✓ | |
| AutoClutterState::mark_dirty (0xee48c0) | ✓ | ✓ | ✓ 脏格→region 反查+哨兵 | ✓ | |
| AutoClutterState::remove_region_from_lazy_grid (0xee4a70) | ✓ | ✓ | ✓ AABB→网格移除 | ✓ | |
| AutoClutterState::flush_candidates_pending_removal (0xee4bf0) | ✓ | ✓ | ✓ 降序 swap_remove+driftsort | ✓ | |
| populate_autoclutter_regions (0x22d6ed0) | ✓ | ✓（44KB 碎片化已注明） | ✓ lazy grid/水位采样/季节门控/retain | ✓ | 内联闭包未逐行还原已如实登记 |
| autoclutter_subscribe_to_tabloids (0x22d6930) | ✓ | ✓ | ✓ 发布-订阅 | ✓ | |
| process_autoclutter_candidates (0x22ce260) | ✓ | ✓ | ✓ enumerate+稳定排序+贪心矩形重叠剔除 | ✓ | |
| manage_autoclutter_entities (0x22d04e0) | ✓ | ✓ | ✓ 集差生命周期 | ✓ | |
| read_bush_positions_from_gpu (0xeebaa0) | ✓ | ✓ | ✓ GPU 回读+cell 哈希变更检测+发布 | ✓ | |
| load_clutter_meshes (0x22c0760)+closure$0 (0x22c1cc0) | ✓ | ✓ | ✓ 批量资产加载 | ✓ | 约 50 个网格名 unknown 合理 |
| autoclutter_animation (0x22ccef0) | ✓ | ✓ | ✓ 缓动+四元数摆动 | ✓ | |
| add_autoclutter_around_gates (0x22cc390) | ✓ | ✓ | ✓ 墙端点/法向→区域 | ✓ | |
| add_autoclutter_around_windows (0x22cefe0) | ✓ | ✓ | ✓ 曲线延伸+3 类区域 | ✓ | |
| add_autoclutter_on_windows (0x22cd7d0) | ✓（radare2 兜底） | ✓ | ✓ 跳表分派+统一旋转/投影 | ✓ | **§4-C：Ghidra 反编译失败已确认** |
| add_birdnests (0x22c9c70)+closure$0 (0x22ca8c0) | ✓ | ✓ | ✓ 脏格遍历+去重哈希集 | ✓ | |
| add_lantern_water_setdresssing (0x22d2fa0) | ✓ | ✓ | ✓ 圆采样+抖动+水位过滤 | ✓ | |
| add_preplaced_autoclutter (0x1ce5900) | ✓ | ✓ | ✓ 变更检测+预置区域 | ✓ | |
| add_tree_setdressing (0x22d3800) | ✓ | ✓ | ✓ 同 lantern | ✓ | |
| dirty_walls (0x1cd16b0) | ✓ | ✓ | ✓ 事件链+哈希集合并 | ✓ | 草案正确 |
| detect_inter_roof_merlons (0x1ce3a70) | ✓ | ✓ | ✓ 形状相交+平顶门控 | ✓ | |
| construct_inter_roof_merlons (0x22d4870) | ✓ | ✓ | ✓ 沿曲线等距 0.46 切分+random_splits+抖动 | ✓ | |
| request_inter_roof_merlon_audio (0x22d4230) | ✓ | ✓ | ✓ 音频命令入队 | ✓ | |
| detect_intra_shape_corners (0x1cd1a90)+closure$4 (0xedbb60) | ✓ | ✓ | ✓ 墙对形状相交+角点/开洞 | ✓ | |
| add_hole_at_shape_intersection (0x1cd3ce0) | ✓ | ✓ | ✓ 世界↔墙空间+投影钳制 | ✓ | `*0.0` 表达式已如实登记 |
| intersect_shapes<merlons/corners> (0xedd250/0xedbf60) | ✓ | ✓ | ✓ 形状相交 | ✓ | |
| spawn_stitches (0x1ce65b0) | ✓ | ✓ | ✓ 随机切分+抖动 | ✓ | |
| spawn_stitch_bricks (0x1ce7190) | ✓ | ✓ | ✓ 实例缓冲烘焙 | ✓ | |
| impl$0::build (0x652050) | ✓ | ✓ | ✓ 插件装配 | ✓ | |
| impl$10::register_required_components (0xee4ee0) | ✓（§4 提） | 胶水 | — | — | |
| detect_intra…::_::impl$0::get_param (0x1ce3430) | 未单列 | 派生 SystemParam 胶水 | — | — | 合理省略 |

### 2.3 color（30 业务 / 1 胶水；发射 31 条 ns + 2 非 ns）

| B 发射函数（VA） | 报告覆盖 | 逻辑清楚 | 算法标注 | unknown 合理 | 备注 |
|---|---|---|---|---|---|
| init_color_ids_buffer (0x2290750) | ✓ | ✓ | ✓ 资源/SSBO 初始化 | ✓ | SSBO 名=类型全名（memcmp 长度证据链，O 成立） |
| set_wall_color_id (0x2290820) | ✓ | ✓ | ✓ 索引写 Vec+整段上传+halftimber AABB 重算 | ✓ | |
| write_roof_color_id_to_gpu (0x2298520) | ✓ | ✓ | ✓ 增量上传 | ✓ | |
| set_decorator_color_id (0x22af0c0) | ✓ | ✓ | ✓ 子类型分派+楼梯级联 | ✓ | |
| set_gate_color_id (0x2299840) | ✓ | ✓ | ✓ 同 wall（tag 7） | ✓ | |
| set_halftimber_color_id (0x2299250) | ✓ | ✓ | ✓ 同 wall（tag 12） | ✓ | |
| set_flag_connection_and_chimney_smoke (0x2298970) | ✓ | ✓ | ✓ 两阶段字段写+命令分发 | ✓ | |
| set_window_light (0x2298260) | ✓ | ✓ | ✓ 窗光写+动画 tag | ✓ | |
| set_roof_customization (0x2299dd0) | ✓ | ✓ | ✓ 变更检测+稀疏位集定位+历史 | ✓ | |
| color_menu (0x2291230) | 部分 | **内部逐行=U（如实）** | 标注缺失（无法标） | ✓ | **§4-D：三路工具失败已确认**；20KB 中枢，建议后续攻破 |
| color_button_animations (0x22979b0) | ✓ | ✓ | ✓ UI 缓动缩放/位移 | ✓ | |
| delete_color_icon_entities (0x2297ff0) | ✓ | ✓ | ✓ 进度≤0 销毁 | ✓ | |
| show_color_options (0x229a7f0) | 部分 | **内部逐行=U（如实）** | 标注缺失 | ✓ | **§4-D**；closure$0 (0x22acce0) 已发射但未展开（合理） |
| hide_color_menu (0x22adcf0) | ✓ | ✓ | ✓ 标记+销毁+复位命令 | ✓ | |
| autoclose_color_menu (0x2298d30) | ✓ | ✓ | ✓ 远距/点击关闭 | ✓ | |
| ColorRadialMenu::create (0x25f27c0) | ✓ | ✓ | ✓ 径向菜单均匀分布（cos/sin+AngleRange） | ✓ | |
| IconInfo::new_separator (0x25f2680) | ✓ | ✓ | ✓ 构造器 | ✓ | |
| IconInfo::new_flat_roof_rim_ty (0x25f26f0) | ✓ | ✓ | ✓ 构造器 | ✓ | |
| ui_color_wall_and_roof (0x1c9ab70)+closure$0 (0xed2580) | ✓ | ✓ | ✓ 射线拾取+UI→命令 | ✓ | closure$2 (0xed2660) 发射但未单列（轻微） |
| color_cursor_icon (0x1c9b510) | ✓ | ✓ | ✓ 跟随鼠标图标 | ✓ | |
| ui_color_gates (0x22afe70)+closure$1 (0xed2660)+closure$2 (0x22b0ba0) | ✓（closure$2 单列） | ✓ | ✓ 三角化 mesh 拾取 | ✓ | closure$1 未单列（轻微） |
| ui_color_windows_and_flags (0x22b0e00) | ✓ | ✓ | ✓ 射线拾取+楼梯图遍历 | ✓ | |
| replay_color (0x1c7f270) | ✓ | ✓ | ✓ 帧回放调度+桶移除+drop 分派 | ✓ | |
| add_color (0x651870) | ✓ | ✓ | ✓ 插件注册 | ✓ | |
| radial_menu::impl$4::register_required_components (0x25f3d60) | 未单列 | 胶水 | — | — | 合理省略 |

### 2.4 onboarding（53 业务 / 13 胶水；发射 66 条 ns）

| B 发射函数（VA） | 报告覆盖 | 逻辑清楚 | 算法标注 | unknown 合理 | 备注 |
|---|---|---|---|---|---|
| hint_orchestrator (0x227fc30) | ✓ | ✓ | ✓ 收集→稳定排序→实体增删状态机 | ✓ | |
| alternate/adjacent/together ×5+（0x1c4e4f0…0x1c4f230） | ✓ | ✓ | ✓ 列表合并（tag 0xca/cb/cc） | ✓ | |
| filter_to_mode_impl (0x22808d0) | ✓ | ✓ | ✓ 按设备模式递归裁剪显示树 | ✓ | |
| resolve_steam_impl (0x227e5e0) | ✓ | ✓ | ✓ 递归把 Steam 字形替换为绑定 | ✓ | |
| cleanup_impl (0x227f4a0) | ✓ | ✓ | ✓ 递归树规范化+压缩 | ✓ | |
| hint_animation_system (0x1c4f400) | ✓ | ✓ | ✓ 归一化进度+缓动 | ✓ | |
| hint_deletion_cooldown_system (0x1c4f790) | ✓ | ✓ | ✓ 倒计时→标记 | ✓ | |
| hint_audio (0x1c4fb00) | ✓ | ✓ | ✓ 动画边界音效 | ✓ | |
| hint_deletion_system (0x1c4fea0) | ✓ | ✓ | ✓ 动画归零销毁 | ✓ | |
| hint_highlight_shape (0x1c50120) | ✓ | ✓ | ✓ 动画>0.9 高亮命令 | ✓ | |
| hint_position_update (0x1c50510) | ✓ | ✓ | ✓ 多锚点屏幕投影+夹取+插值 | ✓ | |
| hint_draw (0x1c51a60)+closure$2 (0x228c260) | ✓ | ✓ | ✓ mei IMGUI 分槽渲染 | ✓ | |
| render_hints (0x1c52700)+closure$1/$2/$4 | ✓ | ✓ | ✓ 判别渲染+列表递归+精灵帧 | ✓ | |
| progress_bar_update_system (0x1c530f0) | ✓ | ✓ | ✓ 分段量化/滞回 | ✓ | |
| exponential_back_off (0x2284800) | ✓ | ✓（抽查确认 clamp(c*1.5,720,10800)） | ✓ 指数退避 | ✓ | |
| hints_startup (0x227b450) | ✓ | ✓（抽查确认 12 资源+0.7/0.7/60） | ✓ 启动注册 | 阈值三处不一致项需修正 | **§4-E** |
| reset_hints (0x1c1c8d0) | ✓（值有错） | 逻辑对，阈值**错** | ✓ 两分支复位 | 该项应归 unknown | **§4-E：0.33 实为 0.7** |
| tutorial_completion (0x227c230) | ✓ | ✓（抽查确认 id→偏移 18 路 switch） | ✓ 完成标志→落盘 | ✓ | §0.2 偏移表对，名称归属有误（§4-E2） |
| camera_pan_hint/rotate_hint/zoom_hint (0x227d000/0x227d2f0/0x227d680) | ✓ | ✓ | ✓ BindingHintDisplay 组合构造 | ✓ | |
| hint_camera_enable_condition (0x227c780) | ✓ | ✓ | ✓ 阈值计数器状态机 | ✓ | |
| hint_camera_display_condition (0x227c910) | ✓ | ✓ | ✓ 组合条件+duration 判定 | ✓ | |
| hint_camera_progress_update (0x227bd30) | ✓ | ✓ | ✓ 事件累积+边沿完成 | ✓ | |
| hint_camera_progress_reset (0x227c070) | ✓ | ✓（抽查确认 1.4/1.2/120） | ✓ id 变化→归零+换阈值 | ✓ | |
| hint_focused_edit_condition (0x227da70) | ✓ | ✓ | ✓ 完成标志/工具判定 | ✓ | |
| hint_focused_edit_hint (0x227dd60) | ✓ | ✓ | ✓ 射线+跳表生成提示 | ✓ | |
| make_focus_or_camera_binding_hint (0x1c3de00) | ✓ | ✓ | ✓ 组合构造 | ✓ | |
| hint_clutter_focused_edit_condition (0x1c3d890) | ✓ | ✓ | ✓ 工具判定 | ✓ | |
| hint_clutter_focused_edit_hint (0x1c3db80) | ✓ | ✓ | ✓ 射线+id3 请求 | ✓ | |
| hint_roof_shape (0x2284ed0) | ✓ | ✓ | ✓ 条件链+raycast+is_flat 门控 | ✓ | |
| make_lbm_drag_binding_hint (0x2285270) | ✓ | ✓ | ✓ together/alternate 组合 | ✓ | |
| hint_stairs (0x2285a20) | ✓ | ✓ | ✓ 多阶段状态机 | ✓ | |
| hint_terrain_layering (0x2283cc0) | ✓ | ✓ | ✓ 工具/事件条件 | ✓ | |
| hint_terrain_focus (0x2283980) | ✓ | ✓ | ✓ | ✓ | |
| hint_terrain_steepness (0x2283640) | ✓ | ✓ | ✓ | ✓ | |
| hint_undo (0x2284220) | ✓ | ✓ | ✓ 数字动作+计时 | ✓ | |
| hint_unlink_windows (0x227d760) | ✓ | ✓ | ✓ | ✓ | |
| hint_deselect_tool (0x22853c0) | ✓ | ✓ | ✓ | ✓ | |
| hint_disable_snapping (0x227e140) | ✓ | ✓ | ✓ | ✓ | |
| prompt_rollercoaster (0x227bb70) | ✓ | ✓ | ✓ | ✓ | |
| add_onboarding (0x650d50) | ✓ | ✓ | ✓ 插件注册 21 事件 | ✓ | |
| impl$1/3/4/5/7/13/21/23/25/27/31::register_required_components | 未单列 | 胶水×11 | — | — | 合理省略 |
| impl$14::fmt (0x22810c0) / impl$30::clone (0x2282cb0) | 未单列 | Debug/Clone derive | — | — | 合理省略 |

## 3. 逻辑总清单（本批次全部业务函数：逻辑一句话 + 实现方式 + 对应算法）

### 3.1 critters（54 条）
1. `Boid::random_vel_change` — 计时到点换锥向+重设 timer，方向时间连续插值→quat arc 旋转速度 | 手写 SIMD 四元数 | 锥均匀采样+arc 旋转+时间连续 lerp
2. `Boid::frame_update<fish/wren>` — 邻居三规则积分+加速度/速度约束+NaN 归零；fish 判水反射、wren 射线可行 | SIMD 向量化 | Reynolds Boids+smoothstep 分离核
3. `BoidFlock::remove<fish/wren>` — 索引倒序 swap_remove 双槽+存活 despawn | Vec swap_remove | O(1) 批删
4. `critters_avoid_collisions` — 仅 active 目标：方向归一化+圆/线段几何+墙空间哈希+水格+spherecast；球距<0.1 命中→DespawnDuckCmd | 栅格/空间哈希 | 线段相交+spherecast+双线性采样
5. `smooth_critter_velocities` — v*retention+dir*(1-retention)，retention=powf(per-second,dt)；写 Multiplier | 3 轴 8B 打包 | 指数衰减 EMA（一阶低通）
6. `move_critters` — 位移=vel*dt*multiplier+避让；朝向 nlerp+归一化；写停靠点 | SIMD divps/sqrtps | dt 欧拉积分+四元数 nlerp
7. `plan_duck_trajectories` — dev 热键：golden_spiral 3 候选+圆偏移构轨迹；热键 B 射线寻路 spawn 鸭群+计划修剪 | in_place_collect | 黄金螺旋+射线寻路
8. `move_ducks_along_trajectories` — 进度+=dir*12*dt/段长；Frenet 帧姿态+音频+状态迁移 | truncf 段索引 | 曲线参数化+弧长推进
9. `update_duck_goals` — 停留-换目标状态机：timer 双半；盘面采样新位+glade/半径/水格校验 | 2×uniform_f32 | 圆盘采样+计时状态机
10. `set_duck_velocities_towards_goals` — DuckState==2 且 active→归一化方向+冻结门 | 直写 | 方向归一化
11. `update_duck_ripples` — 最多 4 水面鸭波纹指数衰减+SSBO 上传 | from_iter+FilterMap | 指数衰减+GPU 上传
12. `DuckSpawnLocations::new` — 10×10 网格候选（52 范围/10 档 z） | 手写 | 均匀网格
13. `spawn_ducks` — 30s 冷却+上限 15：候选筛选（已用）+水/地形过滤+3 点黄金螺旋轨迹 | sorted_by | 冷却计时+黄金螺旋
14. `despawn_ducks` — 水面越界检测→离场轨迹计划 | 多采样求和 | 水面越界检测
15. `duck_splash_particle` — 事件→取网格/着色器句柄+spawn 水花+旧粒子 despawn | 事件驱动 | 粒子生成/销毁
16. `ducks_react_to_being_inside_buildings` — 墙/屋顶包络判定+入内音效 | core 查询 | 包络判定
17. `FishSpawnLocations::new` — 25×25 网格 | 手写 | 均匀网格
18. `despawn_fish` — 重置 FishBoids+重建 FishSpawnLocations | 资源清理 | —
19. `fish` — 上限 200/每帧 20 出生（水面过滤）；boids 更新+墙推开+光标/相机逃避 | switch 逃避半径 | boids+逃避
20. `update_butterfly_states` — 距离门控后跳表状态机（dwell/相位） | 跳表 | FSM
21. `update_butterfly_goals` — 球面采样+状态偏置（±y）+半径向内校验 | 加权 normalize | 球面采样+偏置
22. `set_butterfly_velocities_towards_goals` — 归一化×1.77 写 target+multiplier 累加+动画跳表 | 跳表 | 归一化×标度
23. `update_butterfly_audio` — 逐蝶 set_position | 直写 | —
24. `butterflies_send_to_gpu` — 实例缓冲 from_iter→SSBO cpu_write | FrozenMap+memcmp | GPU 上传
25. `fireflies_send_to_gpu` — 同蝴蝶 | 同上 | GPU 上传
26. `update_firefly_goals` — timer 刷新+球面采样×惯性 bias+z 额外加权+半径校验 | **bias=±0.5（错标 -1.0）** | 球面采样+惯性 bias
27. `set_firefly_velocities_towards_goals` — 归一化写 target+multiplier=1.0+海拔下限 0.25 | 直写 | 归一化+海拔修正
28. `sim_wren_boids` — 顶层开关；出生上限 80（x±65/y1..10）+保留半径 91.9/高 30+树摇爆发+逐帧 boids | closure_3 RAY_DIRS | boids+出生/清理+爆发
29. `sim_wren_boids::closure$3` — 32 向射线可行谓词 | RAY_DIRS 表 | 射线可行性
30. `update_decorator_bird_spots` — HashMap 栖点增删（gen_id+create_spots+despawn 失效） | hashbrown | 生命周期管理
31. `update_roof_bird_spots` — 屋顶几何（脊线/山墙/平顶）取样+random_splits+noise 偏移布置栖点 | recycle+gen_id | 屋顶取样+随机切分
32. `update_wrens` — 放置矩阵 to_affine3a+状态跳表行为+调试可视化+音频位置 | 跳表 | FSM
33. `update_sheep_goals` — 跟随（相机 mode3 或光标）+圆盘漫游+计时器+共享目标刷新 | 2×uniform_f32 | 跟随状态机+盘面采样
34. `set_sheep_velocities_towards_goals` — pet 门控+归一化写 target+multiplier=1.0 | 直写 | 归一化
35. `update_sheep_audio` — 逐羊 set_position | 直写 | —
36. `sheep_walk_on_terrain` — 地形采样+状态机（地面/过渡/行走/浮空）弹簧-阻尼竖向跟随 | 双缓冲水格 | 弹簧-阻尼+状态机
37. `sheep_unstuck` — spherecast/掩码/水格判定卡住+解卡 | 阈值组合 | 卡住检测
38. `react_to_being_inside_buildings` — 包络判定+随机 emoji 生成 | init_rng+表 | 包络+随机索引
39. `sheep_reaction_emoji` — 缓动弹出/淡出+世界→屏幕定位+结束 despawn | EasingFunction | 缓动动画+投影
40. `sheep_floaties` — 水面判定→spawn 漂浮物+入水音效 | AudioCore | 附件生成
41. `sheep_umbrellas` — 雨天判定→spawn 雨伞+音效+开合动画 | EasingFunction | 附件生成
42. `sheep_walk_animation` — pet 门控+跳表动画 | 跳表 | 动画 FSM
43. `sheep_pet_animation` — 五相位动画（idle→启动→保持→淡出→完成） | 候选实现对照 | 五相位 FSM
44. `sheep_input_record_replay` — 变换序列化到输入帧 | InputStream 读写 | 录制/回放
45. `ui_pet_sheep` — 相机门+WidgetParams+命中判定+距离门 | UiWidgetContext | UI 命中
46. `ui_pet_ducks` — 同构 | 同上 | UI 命中
47. `system_critters::impl$0::build` — 资源/事件/系统注册（含 OnExit 清理） | Bevy 装配 | —
48. `sheep::impl$0::build` — 羊事件/系统注册 | Bevy 装配 | —

### 3.2 clutter（36 条）
1. `AutoClutterState::add_region` — key hashbrown 探测：命中等价/不等价（换区域+入 pending）/未命中（写条目） | reserve_rehash | SwissTable+空间哈希
2. `mark_dirty` — 脏格→region key 反查→候选入 pending+0xffff 哨兵 | 反查表 | 脏格→region 反查
3. `remove_region_from_lazy_grid` — AABB 网格枚举删除 | Rectangle2d::aabb | 网格枚举
4. `flush_candidates_pending_removal` — 降序 swap_remove+被换入候选 key 修正条目索引 | driftsort | 索引批量删除
5. `populate_autoclutter_regions` — 删过期+flush+脏格合入+filter_clutter_object+按种类分派产出候选 | 44KB 碎片化 | lazy grid+水位采样+retain
6. `autoclutter_subscribe_to_tabloids` — 13 订阅句柄保存 | Publisher::subscribe | 发布-订阅
7. `process_autoclutter_candidates` — enumerate+filter_map→稳定排序→贪心矩形重叠剔除→复制写输出 | driftsort | 贪心重叠剔除
8. `manage_autoclutter_entities` — 集差 spawn/despawn/update+朝向/抖动 | NameHash+init_rng | 集差生命周期
9. `read_bush_positions_from_gpu` — 回读 92B 记录+空间网格（cell 哈希去重）+每 cell 哈希变更检测→发布脏格 | ipnsort | GPU 回读+哈希变更检测
10. `load_clutter_meshes` — ~50 closure_0 逐个加载+区间收集+insert_resource | from_iter | 批量资产加载
11. `autoclutter_animation` — 缓动+sin/cos 四元数摆±60/50°+完成 despawn | EasingFunction | 时间动画+四元数
12. `add_autoclutter_around_gates` — 墙端点+法向偏移 2.0 两放置 gate_right/gate_left→add_region | calculate_hash | 宿主几何→区域
13. `add_autoclutter_around_windows` — 曲线延伸+3 处 add_region | CurveSlice | 曲线延伸落位
14. `add_autoclutter_on_windows` — 跳表 6 分支+统一旋转/投影+直接写 type5 输出候选 | radare2 | 跳表分派
15. `add_birdnests` — 脏格遍历+去重哈希集+每格 1 鸟巢候选 type4 | cell_from_world_space | 脏格遍历
16. `add_lantern_water_setdresssing` — 装饰物圆周内层 5 次 jitter+水位过滤+type5 候选 | fmodf 圆周 jitter | 圆采样+水位过滤
17. `add_preplaced_autoclutter` — 长度差集变更检测→预置区域生成 | Local 标志 | 变更检测
18. `add_tree_setdressing` — 每树内层 12 次，同 lantern | 同上 | 圆采样+水位过滤
19. `dirty_walls` — 三事件链→HashSet<WallId> | set::extend | 增量脏集
20. `detect_inter_roof_merlons` — 脏屋顶平顶门控+形状相交生成断口+WallHoles | intersect_shapes | 形状相交
21. `construct_inter_roof_merlons` — 沿曲线等距 0.46 切分+random_splits_into+抖动+一城垛砖 | SparseInstanceBufferWriter | 曲线等距切分
22. `request_inter_roof_merlon_audio` — 状态 0xd 早退+光标+indexmap 入队 | spec_extend | 音频命令入队
23. `detect_intra_shape_corners` — 脏墙对形状相交+角点高偏移-0.67+WallHoles | intersect_shapes | 形状包含/相交
24. `add_hole_at_shape_intersection` — 世界↔墙空间变换+投影钳制+写洞记录 | project_clamp | 坐标变换
25-26. `intersect_shapes<…>` — 形状相交核心（两实例） | 0xedd250/0xedbf60 | 形状相交
27. `spawn_stitches` — 短段缝合：truncate(min(1000,2|长|))+random_splits+抖动+grow_one | 0x40 参数 | 随机切分+抖动
28. `spawn_stitch_bricks` — 清空+遍历把 StairsFloatingBrickParams 烘焙进 BrickTransformSsbo | push | 实例缓冲烘焙
29. `impl$0::build` — 系统/资源/事件注册 | Bevy 装配 | —

### 3.3 color（30 条）
1. `init_color_ids_buffer` — 注册 4 SSBO+3 颜色资源 | AssetSsboLibrary::add_new | —
2. `set_wall_color_id` — WallColorIds[idx]=id+整段上传；halftimber 墙装饰器 AABB 中心重算+tag6 命令 | SIMD minps/maxps | 索引写+上传
3. `write_roof_color_id_to_gpu` — 逐元素写 GPU SSBO 增量上传 | 两路查询 | 增量上传
4. `set_decorator_color_id` — 5 层循环+子类型分派（楼梯级联/灯笼/默认）+动画 tag 命令 | DecoratorExtraInfo+0x46 | 子类型分派
5. `set_gate_color_id` — 同 wall（tag7） | — | 同上
6. `set_halftimber_color_id` — 同 wall（tag12） | — | 同上
7. `set_flag_connection_and_chimney_smoke` — flag 连接值+chimney 烟雾值+tag9/15 命令 | as_flag/as_chimney | 字段写+命令
8. `set_window_light` — 窗光写+动画 tag 集合 {8,10,0xd} | as_window | 字段写+命令
9. `set_roof_customization` — 屋顶组件变更检测（样式/rim/颜色）+历史+tag11 | 稀疏位集定位 | 变更检测+历史
10. `color_menu` — 径向菜单逐图标交互→命令分发中枢 | **内部逻辑=U** | —（待攻破）
11. `color_button_animations` — 删除反向/出现正向缓动+缩放+位移 | EasingFunction | UI 缓动
12. `delete_color_icon_entities` — 进度≤0 despawn | +0x18+idx*0x24 | 动画结束销毁
13. `show_color_options` — 读 ShowColorMenuCmd→构造图标→ColorRadialMenu::create | **内部逻辑=U** | —（待攻破）
14. `hide_color_menu` — 标记销毁全部图标+每类写复位命令 | 子类型 switch | 收尾命令
15. `autoclose_color_menu` — kind12 或图标距摄像机>10 关闭 | 尺寸系数 2.0/1.2 | 自动关闭
16. `ColorRadialMenu::create` — N 图标角范围均匀分布（cos/sin 极坐标）+tag3..14 分派+选中态 | AngleRange | 径向菜单布局
17-18. `IconInfo::new_separator/new_flat_roof_rim_ty` — 纯构造器 | — | —
19. `ui_color_wall_and_roof` — 模式 12+射线拾取+slider+命令 | 命中类型分派 | UI 交互
20. `color_cursor_icon` — 跟随鼠标图标实体 | SignifierEntityBuilder | —
21. `ui_color_gates` — 稀疏位集→三角化 mesh→控件+命令（tag5） | fold Triangle 列表 | 三角化拾取
22. `ui_color_gates::closure$2` — 拷贝 slice+DrawTopology 实体 | — | —
23. `ui_color_windows_and_flags` — 射线拾取+楼梯图遍历+点击开对应类别菜单 | StairsState graph | UI 交互
24. `replay_color` — DecoratorHistoryEdit 负载→sub 分派到 14 事件写者+桶移除 | 反转 Vec::remove | 帧回放调度
25. `add_color` — 15 事件+Startup/Update 系统注册 | — | —

### 3.4 onboarding（53 条）
1. `hint_orchestrator` — 请求收集排序→同 id 迁移/类型变更/冷却→spawn 提示组件 | driftsort | 实体增删状态机
2-6. `alternate/adjacent/together<…>` — 同族列表 push 否则新建 2 元素列表 | grow_one | 显示树组合
7. `filter_to_mode_impl` — 手柄/键鼠递归裁剪显示树 | 递归 | 模式过滤
8. `resolve_steam_impl` — 递归替换 Steam 抽象字形，空集退化 Empty | in_place_collect | 递归替换
9. `cleanup_impl` — 递归清理压缩，剩 1 提升单元素 | 递归 | 树规范化
10. `hint_animation_system` — 进度+=dt/0.3*dir clamp+缓动 | EasingFunction | 归一化进度
11. `hint_deletion_cooldown_system` — 冷却递减→标记 | — | 倒计时状态机
12. `hint_audio` — 动画值 1.0/0.0→PlayUiSound tag21/20 | — | 边界音效
13. `hint_deletion_system` — 动画归零 despawn | — | 销毁
14. `hint_highlight_shape` — 动画>0.9 且类型∈{1,2,6,7,8,9}→屋顶高亮 | roof_entity | 高亮命令
15. `hint_position_update` — 光标/世界点/包围盒/屏中心 多锚点+夹取+插值 | time_continuous_lerp | 多锚点投影
16. `hint_draw`+closure$2 — mei 分槽渲染绑定/文本/进度条/交互 | with_slot | IMGUI 绘制
17. `render_hints`+closures — 判别渲染（图/键/鼠/Steam/4 键/列表递归） | floorf 帧索引 | 显示树渲染
18. `progress_bar_update_system` — 进度量化 0..3 档+档位变化音效 | 阈值 0.4/0.99 | 分段量化/滞回
19. `exponential_back_off` — 双计时器/冷却，newCooldown=clamp(old*1.5,720,10800)，到点清标志+落盘 | 3 reader | 指数退避
20. `hints_startup` — 插入 12 条件资源，初值来自完成标志+阈值 0.7/0.7/60 | 标志读取 | 启动注册
21. `reset_hints` — 事件==0→全完成置 1/状态 2；否则→条件资源复位+标志清零+阈值**0.7/0.7/60（报告错写 0.33）** | 两分支 | 复位状态机
22. `tutorial_completion` — 完成事件 id→18 路 switch 置偏移+落盘 | SaveGameProfileCmd | id↔标志映射
23-25. `camera_pan/rotate/zoom_hint` — 按 profile 模式构造 Graphic/键组合 | alternate/adjacent/together | 组合构造
26. `hint_camera_enable_condition` — 鼠标/相机事件计数器阈值 100/10/0.1/1.5→state=1 | — | 计数器状态机
27. `hint_camera_display_condition` — 三完成组合→state=2+构造 3 提示按 duration 判定 | AppMode/StructureType | 条件组合
28. `hint_camera_progress_update` — pan+=dt/阈、rotate+=事件值/阈、zoom+=dt/阈；跨 1.0→完成事件 | 边沿判定 | 事件累积
29. `hint_camera_progress_reset` — 提示 id 变化→归零+换阈值（1.4/1.2/120） | Local 上一帧 | id 变化复位
30. `hint_focused_edit_condition` — 完成标志/工具∈{1,2}/焦点工具判定 | — | 条件判定
31. `hint_focused_edit_hint` — count>4 门控+射线+跳表生成提示 | geo_src_to_edit_subject | 条件提示
32. `make_focus_or_camera_binding_hint` — 按 focus 模式构造鼠标组合 | alternate | 组合构造
33. `hint_clutter_focused_edit_condition` — 工具==3/实体字节==9 判定 | — | 条件判定
34. `hint_clutter_focused_edit_hint` — 射线+subject==3→id3 请求 | make_focus... | 条件提示
35. `hint_roof_shape` — 多条件链+raycast+roof 非平顶→id7 请求 | legacy_raycast | 条件提示
36. `make_lbm_drag_binding_hint` — together(steam,alternate(lmb-move)) | — | 组合构造
37. `hint_stairs` — 五阶段（标志 0x201..0x205）状态机+完成事件 tag0xd..0x11 | 多 EventReader | 多阶段状态机
38. `hint_terrain_layering` — 工具 10+事件计数>1→id6 请求/tag6 完成 | — | 条件提示
39. `hint_terrain_focus` — 工具 10+事件字节==0→id6/tag5 | — | 条件提示
40. `hint_terrain_steepness` — 工具 10+事件字节==1→id6(0x400)/tag4 | — | 条件提示
41. `hint_undo` — 数字动作 just_pressed/计时>10→tag2 完成+id6 请求 | 键码来源 U | 条件提示
42. `hint_unlink_windows` — 标志门+事件→tag8 完成/else id6(0x800) | — | 条件提示
43. `hint_deselect_tool` — 工具已选+动作按下+鼠未按→id6(+子类型) 请求 | — | 条件提示
44. `hint_disable_snapping` — 事件字节==0+工具 6→id(键) 请求/tag3 完成 | — | 条件提示
45. `prompt_rollercoaster` — 有事件→id10(0x1200) 请求 | — | 条件提示
46. `add_onboarding` — 21 事件+Startup/Update 系统注册 | — | —

## 4. 冲突与需裁定项（含建议方向）

### 4-A（确定错误）critters firefly 惯性 bias：`-1.0` 应为 `-0.5`
- **证据（三重）**：
  1. update_firefly_goals (0x142316940) fresh 反编译：`fVar22 = *(float*)(&__real_bf0000003f000000 + (ulonglong)(*puVar18 == 0)*4)`；
  2. **反汇编定位**：`0x142316dfd XOR ECX,ECX / TEST EAX,EAX（EAX=*RSI=goal active u32）/ SETZ CL / LEA RAX,[0x142ccfc58] / MOVSS XMM15,[RAX+RCX*4]` → `active==0`（无目标）取偏移 4；
  3. **raw memory @0x142ccfc58（read_memory 16B）** = `00 00 00 3f 00 00 00 bf ...` → 偏移 0 = LE `0x3f000000`=+0.5（有目标）、偏移 4 = LE `0xbf000000`=**-0.5**（无目标）。
  即：无目标 **-0.5**、有目标 **+0.5**。IEEE 下 `-1.0=0xbf800000`，与常量不符。
- **现状**：ANALYSIS.md（含所引 extra-analysis「校正」）把「草稿 -0.5 校正为 -1.0」——**方向反了**；报告自身把 0xbf000000 解码成 -1.0 是错的（应为 -0.5），但其「无目标取 0xbf000000 字」的偏移映射是对的。
- **裁定方向**：**驳回 -1.0 校正**。正确值为无目标 **-0.5**、有目标 **+0.5**。修正 ANALYSIS.md `update_firefly_goals` 条目与附·草案第 5 条；若 `reconstruction/critters/firefly.rs` 已按 -1.0 实现，同步回退并重跑差分。

### 4-B（既有差分项）smooth_critter_velocities NaN 符号位差异
- **证据**：fresh 反编译确认逻辑（`fVar18=powf(retention,dt)`、`fVar19=1-fVar18`、`v=v*fVar18+dir*fVar19`，写 `Multiplier=Query+0x14`）。原码 `ffc00000`（负 NaN）vs Rust `7fc00000`（正 NaN），为 B-firefly-verification.json case 1578 唯一失败项。
- **裁定方向**：不视为模块逻辑冲突；根因仍 U（I：powf/0×inf 求值顺序）。建议：若目标是逐位一致，需复现原求值顺序与 round/NaN 语义（负 NaN 常见于 `-0.0*x` 或特定编译器 SSE 序列）；若目标是行为等价，可登记为 IEEE 未定义语义的已知可接受差异。至少保持 unknown 状态，不要当已解决。

### 4-C（验证通过）clutter add_autoclutter_on_windows Ghidra 反编译失败→radare2
- **证据**：fresh 反编译 0x1422cd7d0 返回 size 364 截断（"bad instruction data"、"Switch with 1 destination removed"、"Truncating control flow"）；radare2 size 1622 完整。报告走 radare2 的处理正确。
- **裁定方向**：接受 radare2 结论。遗留疑点（写块 +0x08 位置读栈槽 RBP-0x60 的别名、6 分支窗类型语义）保持 U，建议后续用 P-code 或动态核对栈槽别名后再定。

### 4-D（验证通过）color color_menu / show_color_options 三路工具全失败
- **证据**：fresh 反编译两者均返回退化输出（"Control flow encountered bad instruction data"、"Instruction at … overlaps instruction"、"Type propagation algorithm not settling"），color_menu 20KB/687B。报告把内部逐行标为 U 而非编造，正确。
- **裁定方向**：接受 U 状态，但注意这是 color 模块**中枢**（逐图标交互→命令分发），业务量占模块核心。建议后续专门攻破：定位重叠指令 0x142296665/0x142296666、0x1422ae470/0x1422ae473，补数据对齐后重反编译，或转 ReVa High-P-code 定点恢复；在攻破前 color_menu/show_color_options 不应计为已恢复。

### 4-E（确定错误）onboarding CameraHintProgress 阈值三处不一致：reset_hints 的 `0.33` 应为 `0.7`
- **证据**：reset_hints (0x141c1c8d0) fresh 反编译：`builtin_strncpy(puVar7+3, "333?333?", 8)` 即字节 `33 33 33 3f 33 33 33 3f` → LE f32 `0x3f333333` = **0.7**、`0x42700000` = 60.0。报告写的 `0x3eaaaaab`(0.333) 是误读。hints_startup (0x227b450) 同一常量（`0x3f3333333f333333`+`0x42700000`）。
- **裁定方向**：**修正 reset_hints 阈值为 0.7/0.7/60**。由此"三处不一致"实为**两套**：启动 & reset_hints = (0.7,0.7,60)，progress_reset = (1.4,1.2,120)。两套并存根因仍 U（建议方向：0.7/60 为"初始教程"节奏、1.4/1.2/120 为"再次提示"提速节奏——可查 `hint_camera_progress_reset` 被谁驱动验证，但不要当结论）。

### 4-E2（确定错误）onboarding §0.2 提示 id 11/12 名称归属互换
- **证据**：hints_startup 把 `0x1f7` 的完成标志写入 rotate 进度槽、`0x1f8` 写入 zoom 槽；hint_camera_progress_reset 按 id 0xa=pan(+0)、0xb=rotate(+4)、0xc=zoom(+8)。报告 §0.2 却写 "11 CameraZoom, 12 CameraRotate"，与其自身 §7.2/§8.1（0xb=rotate、0xc=zoom）矛盾。
- **裁定方向**：修正 §0.2 为 "11 CameraRotate, 12 CameraZoom"（偏移 10→0x1f6、11→0x1f7、12→0x1f8 不变，仅名称）。

### 4-F（清晰度问题）critters Boid 布局注释自相矛盾
- **证据**：random_vel_change (0x141dc2170) fresh 反编译：timer@+0x30、dir@+0x24(8B)/+0x2c、插值输出@+0x18/+0x20；速度在另一指针 +0xc..0x14。BoidFlock::remove 又提"0x18 槽+0x38 副槽"。报告"Boid 0x18{…timer@+0x30…}"把紧凑槽与完整状态混为一谈。
- **裁定方向**：接受实现（逻辑不受影响），但修订布局注释为**两结构**：flock 紧凑槽（0x18，pos+vel 或 pos+vel 紧凑）与每 boid 完整状态（0x38 副槽：dir/timer/插值）。不要合并为单一 0x18 结构。

### 4-G（描述精度）critters frame_update 邻居门描述略粗
- **证据**：fresh 反编译帧循环门为两个条件：`self.vel·nvel >= 0` **且** `cos∠((npos-selfpos)·nvel) >= -0.5`（即 ≤120°）。报告把 -0.5 归为"同向门"，实际 -0.5 在第二个 cos 条件。
- **裁定方向**：不影响算法结论；建议把"heading 门 -0.5"描述为"邻居相对位移与邻居速度夹角 ≤120° 门"，与 `self.vel·nvel>=0` 分列。

## 5. 遗留 unknown 汇总（按模块，均合理，含建议优先级）

### critters（25 项，报告 §未知清单）
重点可后续攻破：CritterLocomotion 值→物种映射（卡点）、CritterVelocityMultiplier 语义、DuckState 0、Trajectory/TrajectoryPlan 布局、butterfly/firefly 查询含 Sheep 疑解析、0x38 副槽字段、`Duck+0xe8`、ripple SSBO 布局、sheep_unstuck 解卡公式。新增建议：**Boid 紧凑/完整双结构布局**（§4-F）与 **firefly bias 值**（§4-A）应列入。

### clutter（15 项，报告 §7）
populate 44KB 碎片化、on_windows 跳表语义+栈槽别名、process `byte==6`、gates/windows 精确几何、manage ColorId 来源、read_bush 92B 布局、load_meshes 50 网格名、BrickTransformSsbo/StairsFloatingBrickParams producer（core 卡点）、subscribe 次数差异。

### color（12 项，报告 §未知清单）
**color_menu/show_color_options 内部逻辑（最高优先，§4-D）**、stride 0x20 命令元素与 tag 消费者、`Set*ColorIdCmd` 字段命名与 `0x4fff`、roof 字段名/分块公式、decorator 5 层循环、as_window/as_flag 字段名、icon 布局、radial 角度基准、hide 每 case 命令映射、replay 14 写者映射、add_color 系统顺序。

### onboarding（12 项，报告 §21.3）
阈值两套并存根因（§4-E 修正后）、HintType/HintPlacement 值集、DisplayHintRequest 布局与 flags、0xc8/0xcd 语义、CameraHintsState/CameraHintProgress 字段语义、各 conditions 判定、AppMode/工具值/is_fence、closure$0 排序键、add_onboarding 排程、hint_stairs 五阶段条件、纯内联 helper。

## 6. 对分析质量的整体评价与后续建议

**整体评价**：
- 四份报告质量**高**：逐函数输入→处理→输出到字节级，O/I/U 标注规范，常量/布局/算法名齐备，未知清单克制不编造。覆盖核对零业务遗漏（含闭包）。
- 存在**两处确定性数值错误 + 一处归属互换**，且恰为"把错值当 O 写"的过度断言（firefly bias -1.0、reset_hints 0.33、id 11/12 名称）。这说明**报告对"校正"类二次结论的复核不足**：critters 的 extra-analysis 校正与 onboarding 的字节串解读都应回炉。教训：对 0xbf/0x3f 类常量应做 IEEE 解码复核，不要沿用上家推断。
- 工具失败点（color_menu/show_color_options、add_autoclutter_on_windows）均如实登记，未以推断填充——符合硬约束。

**后续建议**（只针对模块内部实现）：
1. 依 §4-A/§4-E/§4-E2 修正三处数值/归属；若 `reconstruction/critters/firefly.rs` 已按 -1.0 实现需同步回退并重跑差分。
2. 优先攻破 color_menu/show_color_options（模块中枢，20KB 反编译器失败），用重叠指令定位+P-code 定点恢复；攻破前不将二者计为已恢复。
3. 对 firefly bias 作一次差分回归（RNG 直方图不变的前提下逐位验证 -0.5 路径）。
4. Boid 双结构布局、reset_hints 阈值两套根因列入 unknown，保持 U。
5. 其余 unknown 均可在 core 契约闭合后（AutoClutterRegion/DecoratorExtraInfo/BrickTransformSsbo 等）随实现一并消解。

---
（复核依据：四份 ANALYSIS.md、四份 TASKS.md、`skeleton/B/crates/systems/*/src` 发射清单脚本提取、ghidra MCP shared 抽查 12 函数 + update_firefly_goals 反汇编定位 + raw memory @0x142ccfc58。抽查反编译/反汇编原文存于各模块 `jobs_review*.json`、`jobs_bias*.json`。）
