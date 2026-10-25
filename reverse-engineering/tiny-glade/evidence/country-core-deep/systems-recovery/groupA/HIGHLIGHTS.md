# groupA HIGHLIGHTS（供中央汇总，≤10 条）

1. **共 52 业务函数 + 10 闭包完成反编译**（water 3 / forest 4 / glade-border 2 / timberframe 4 / tree-tool 13 / color 26）；zeno crate 7 项与 rhapsody create_buffer 归 dependency_reuse。全部反编译落盘 groupA/<crate>/decomp/。
2. **MAX_NUMBER_OF_WALLS=20480 断言**（'assertion failed: wall_index < MAX_NUMBER_OF_WALLS'）—— 墙/门/半木颜色索引共用墙索引空间；门/半木 panic 复用同文案。
3. **SSBO 写链**：CPU Vec<u32>（*ColorIdSsbo）→ 0x140beefa0（StagingSsbo 脏区上传，0xb=缓冲不足）→ GPU；缓冲按类型名从 0x1425f18b0（TypeId→Arc<String>）取，类型名四串 = core 侧 color_id*.md 的 *ColorIdSsbo。
4. **实体颜色记录 0x20B {id, 打包中心, max_y×0.4922, tag}**：tag 6=墙/7=门/0xc=半木/10=装饰物（set_decorator_color_id 楼梯 as_stair 写 color@+0xc）；set_roof_customization 样式字节写屋顶记录 +0x3d，历史变体 4/edit 8。
5. **树命令 0x58B {1,1,变体,id,…} 变体 0=放置/1=移动/2=删除，edit=0xc**（hide_trees/place_tree/ui_move_tree/ui_delete_tree 四系统写入；replay_tree 消费）。despawn α 阈值 **0.01**（0x142960674）。
6. **树-墙共享 WallSpatialHash**：0x1422785d0/0x1425defd0/0x1425df320 = query_with_circle(_callback) 三单态；树间距半径 **7.0**；manage_tree_prefabs 缩放 {0.3..0.75} vs place_tree {0.4922..1.0}（手放/森林不同）。
7. **树 prefab 七路径全量**（naked1/naked2/naked1_no_mushrooms/branchy1/branchy2/autumn·branchy1/2）+ random_tree_prefab 季节/万圣节分派；place_tree 用 splitmix64(id) 确定性变体。
8. **图标网格路径全量 40+**（.rdata 0x3039d78/0x3135358：flags/windows/lights/railings/roofs/bricks/plaster/skirt/halftimber/freehand_ty…）；IconInfo{String, kind@+0x18, u32@+0x24} kind 5=separator('white')/8=freehand_ty（PDB 名 new_flat_roof_rim_ty 名实错位）。
9. **模式/动作门禁**：draw_water mode 9(橡皮)/15(水)；ui_place_tree mode 0x10；ui_delete_tree 9+submode 6；ui_move_tree 0xb；color_menu 动作族 just_pressed（动作资源 +0x98..+0xac）；全部系统共用全局 0x14362c098（UI 抢占）。
10. **U 保留**：show_color_options（Ghidra 双超时，契约经 closure$0+callee 表取证）；0x140987ca0/0x1400885d0 小函数体；record_water_history/timberframe_on_glade_unload/reset_tree_tool 内联；0x140b34f80 归属冲突（HighlightColor::to_rgb vs split_height_into_skirts）。

## 追加（RE-ANALYSIS-2，2026-10-16 reviewer-A 四项补挖）

11. **show_color_options 升级为 O（调用序级）**：rne-disasm 632 call 全量（边界 0x14229a7f0..0x1422a0e18）；5×ColorRadialMenu::create 判别链（墙/装饰混合、楼梯+StairsState::graph、窗户、楼梯变体2、半木/样式）+ 7 种 DecoratorExtraInfo 访问器 + 装饰 kind 字节 {2,6,9,0xa,0xb,0xc} 映射（I）。
12. **color_menu 分支契约**：0xa0B 图标条目 kind@+0x30 {9,4,10} 三路分派 + 13×History::add（edit 类型 0x9，0x30B 命令 {1,0,1,0,f32,u8 sel,u8 sel^1,u8 kind}）+ 动作对（+0x98/0x9c 选择；+0xa0/0xa4、+0xa8/0xac 导航）。
13. **forest 6 栅格身份**：0x140ae4c20 = PathRastersResources::as_float_grid（4 路 {ptr,end} 收集，非转 float；每源 {Vec@+8/+0x20, end@+0x10/+0x28, selector@+0x298}）；0x140a14df0 = WorldRaster<512>::bilinear_sample_at_world_xz（130.0 = ws_to_px 尺度 (w/130+0.5)*512，512 宽，+0x201 偏移）；另 2 栅格 = param_12/param_13 WorldRaster 视图（WaterRaster/GardenRaster 排除面，I）。
14. **timberframe circle 输出 = WoodenPlankSsbo**（类型名 61 字符 @0x142ff8cf0，TypeId 键 @0x142ff8bb8；0x1421b5d00 名查找 + 0x1421b6000 单梁写入；梁数 = max(6, perimeter/0.63)；log 'Timberframe constructor ran out of wooden plank pieces' @0x142ff8f40，门禁全局 0x14362c3a8）。
15. **树 0x48B 记录全布局**（place_tree 写面 + hide_trees/despawn_trees/ui_move_tree 读面交叉）：{Arc<String>@+0, &str@+8, 打包位置@+0x10, 组件表@+0x18(0x30-stride), 打包对@+0x20, @+0x28, 变体 u8@+0x30, 树 id@+0x38(next_index), 记录代@+0x40}；0xa0B 条目 {+0x50→0x58B 子表(含 prefab 路径 String@+0x48/+0x50), +0x94 记录索引, +0x80/+0x88 二级哈希, +0x30 kind, +0x34 值}；容器 A/B 双字段结构 = 树/墙/装饰共用通用 entity storage。
