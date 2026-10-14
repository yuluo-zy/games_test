# wall 域 HIGHLIGHTS（10 条，供中央汇总）

1. **WallState = 0x70B（112B）权威布局落定**：enclosure 记录@+0x38（tag u32<2=premade）、height f32@+0x54、terrain floor@+0x58（先验）、freehand_ty u8@+0x60、arch_type u8@+0x64、removed_supports u8@+0x67、双 Vec 曲线存储；InnerWalls map 条目 = {WallId u64, WallState 0x70}（步长 0x78）。来源：`_try_update_state` 闭包族 6 函数（0x140aa9800..0x140aaaca0）逐写偏移。

2. **MaterialStats 全布局 + 原子累加器语义**：7 特征长度 prev@+0x28..+0x40 / cur@+0x90..+0xa8；8 个 segments/pillar 计数 u32@+0xac..+0xc8 + seed u64@+0x88；add_*_length 用 CAS 加本特征、扣 plain(+0x90)/lintels 侧(+0x9c) 预算；arches_length_delta = 7 对 |cur-prev| argmax（索引 0..6），plain 带 **-0.01 bias**，前 5 对和>0 门控。set_pillar_placements 带自旋锁(+0x68)+futex 式等待队列。

3. **WallState serde 13 字段名全表**（`curve,height,is_enclosed,elevation,arch_height,arch_ty,style,structure_ty,clutter_version,plaster_peel,removed_supports,is_valid_enclosure,has_skirts`）+ 旧版 WallStyle 结构 11 字段表（freehand_ty…stairs）——类型合同直接可写。

4. **`PublicWallState::wall_space/u_space` 纠错**：机器码 = `{tag!=2, f32}`（Option<f32>），wall_space=+0x28c 发布长度、u_space=全局常量；**判别是形状 tag 不是 is_enclosed**（wp11 合同需改）。get_wall_attachment_height 全流程 O（premade→max_y；freehand→u×raw_len→clamp01→2D 插值→3D 投影→高度数组插值）。

5. **PDB 名错位三连**：0x140b34f20 名 get_inner 实为 WallId::is_valid；0x140b2a680 名 is_premade_shape 实为 max_y_gable；0x1408c56b0 名 is_default_arch_height 实为 get_enclosure_params。另 0x140833570 名 as_wall_mut 返回 u32 值（疑错位）、0x140a88420 名 pillar_count 实为 setter。

6. **DecoratorType 判别族全解**（7 个谓词，字节位运算）：窗口 0x38/0x39/0x3a、可上屋顶 t≠0x3a、楼梯顶 {0x3c,0x3d,0x3e} 或非窗、角吸附 t≠0x3f、can_change_seed {0x38,0x39,0x3a,0x3d}——重建代码可更新枚举值域。

7. **publish_public_wall_state 权威流 O 级确认**：14 系统参数、0x2a8 步长 hashbrown SIMD 遍历、AnimateWallBulldozeBuffer::contains 跳过动画墙、SparseInstanceBuffer::allocate（2 静态类型 id）、OnWallChanged/Deleted 16B 事件行、`!is_adding` 断言；spatial_hash_update 4 参数面 + add_segment（格成员 0x38B）/remove_wall_id 全部对账。

8. **`simplified_curve_2d` 新字段名**：`assertion failed: w.simplified_curve_2d.is_valid()`（public_walls.rs:50）被 publish/spatial_hash_update/clamp_wall_height 三处内联——PublicWallState 的简化 2D 曲线是发布/空间哈希/高度钳制的共享输入（wp11 合同未列）。

9. **原工作室信息泄漏**：panic 源路径 `/home/h3/code/pouncelight/country-slice-private/crates/…`——工作室 **Pounce Light**、开发机用户 h3、私有仓 country-slice-private；依赖版本再次证实 puffin 0.14.1 / bevy_ecs 0.16.0 / parry3d 0.17.0 / rustc 05f9846f。另 eraser archivist 存在倒序存储串（`uespemosarenegylmodnarodsetybdet`，U 语义）。

10. **基建事故 + 索引缺口**：`run-decomp-batch.py --chunks` 的 chunk dbid bug（268 全 NotFound，已绕过并全量跑通）；sysres-procedures.json 缺 12 个 wp12 合同地址（draw_wall、clamp_wall_height、animate_wall_bulldoze 等）+ 3 个 _try_update_state 实例化（aaa350/aaa530/aa9bc0）——索引生成需修，其他域 owner 注意自查缺口。

round2 闭合更新：① u_space 常量 = **1.0f @0x142925904**（早前 0x142c55904 为十六进制加法笔误；原/B 两二进制字节一致、无重定位）→ u_space = {tag!=2, 1.0f} 即预置形状 Some(1.0) 归一区间；② v2 索引重映射后 0x140833570 = DecoratorOpEventKind::id（u32 ✓ 自洽），真 as_wall_mut 在 0x140833540（tag<=1 → &mut self，否则 unreachable panic 行 40）；③ DecoratorStorage 五集合映射见 round2/wall-CLOSURE.md（+0x0 wall/+0x90 terrain/+0x120 stair O，+0x18 roof I，+0x48/+0x78 索引，+0x1b0 可写标志）；④ eraser "倒序串"实为 take_snapshot 的 16B XOR 掩码常量（auVar6），非消息串。
