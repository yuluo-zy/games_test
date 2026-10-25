# system-clutter RESTORE.md（组 B restorer，2026-10-16）

crate 落点：`tiny-glade/crates/systems/clutter/`（workspace 已挂载，`cargo check -p system-clutter` 通过）。
证据基线：B patched EXE（reva/radare2 RNE oracle），`clutter-placement-analysis.md` +
`clutter-extra-analysis.md`。**未复用 reconstruction/clutter 的 TODO/占位**（reviewer-B
预警：hash_gate_placement 返 0、push_spawn 空宏、process_candidates 占位等均按 B 机器码
重新实现）。

## 1. 覆盖（SOURCE_MAP 37 业务，全部落位）

| 文件 | 业务函数（VA） | 本 crate 实现 |
|---|---|---|
| lib.rs | impl$0::build 0x140652050 | `configure`（AutoClutterState 等资源 + 19 系统链） |
| autoclutter/mod.rs | add_region 0x140ee3fd0、mark_dirty 0x140ee48c0、remove_region_from_lazy_grid 0x140ee4a70、flush_candidates_pending_removal 0x140ee4bf0、register_required_components 0x140ee4ee0 | 全 5（布局：0x20/0x28 区域候选 0x98、0x30/0x38/0x40 pending u16、0x48/0x50/0x58 输出 0x68、0x60.. 区域表、0x90 epoch） |
| populate_autoclutter_regions.rs | init_state 0x1422e1b40、subscribe 0x1422d6930、populate 0x1422d6ed0 | 全 3（过期区域 retain/冲刷/脏格合入/水位岩石过滤/种类分派/哈希键/add_region） |
| process_autoclutter_candidates.rs | 0x1422ce260 | 枚举过滤 → 排序 → byte+0x30==6 强制路径 / Rectangle3d 重叠剔除 → 0x68 输出（源→输出字段映射表） |
| add_autoclutter_around_gates.rs | 0x1422cc390 | gate_left/gate_right 双放置 + 键 tuple<&str,u32,WallId> + add_region |
| add_autoclutter_around_windows.rs | 0x1422cefe0 | 3 add_region（窗左右下沿/上沿；键 tuple<&str,i32,usize,u32>） |
| add_autoclutter_on_windows.rs | 0x1422cd7d0 | 跳表 6 分支（候选+0x00 种类；==6 跳过；宿主子结构 0xf0/0xf8/0x1a8/0x1b0/0x1b8 + Vec 下标）+ 旋转投影（(0,0,1,0)/1.0 + 2*p.z*p 反射）+ type=5 输出 |
| add_birdnests.rs | 0x1422c9c70 | 脏格去重 → type=4 输出（0.15 偏移、scale 1.0） |
| add_lantern_water_setdresssing.rs | 0x1422d2fa0 | 地形装饰物 5 圆周 jitter + 入水 + 键 "lantern" + type=5 |
| add_tree_vegetation_setdressing.rs | 0x1422d3800 | 树 12 圆周 jitter + 入水 + 键 "tree" + type=5（is_dragging/is_started 门控） |
| add_preplaced_autoclutter.rs | 0x141ce5900 | 预放置条目 → add_region |
| manage_autoclutter_entities.rs | 0x1422d04e0 | 现存集差集 → spawn_empty + insert<ColorId/ClutterMarkedForDeletion> / despawn；朝向叉积 + jitter |
| autoclutter_animation.rs | 0x1422ccef0 | Easing 弹入动画 + 完成 despawn |
| load_clutter_meshes.rs | 0x1422c0760 | ClutterMeshes insert_resource |
| read_bush_positions_from_gpu.rs | 0x140eebaa0 | Aabb2 采样矩形 + ipnsort 排序（GPU 回读缝） |
| inter_roof_merlons/detect.rs | 0x141ce3a70 | dirty_walls（三事件链）+ is_flat 门控 + expand×2 交点 → WallHoles::add |
| inter_roof_merlons/construct.rs | 0x1422d4870 | 半木/平顶无沿门控 + ceilf(长/0.46) 段切分 + jitter 抖动 + MerlonBrick（kind 4） |
| inter_roof_merlons_audio.rs | 0x1422d4230 | 光标命中城垛 → 音频请求事件 |
| detect_intra_shape_corners.rs | dirty_walls 0x141cd16b0、detect 0x141cd1a90、intersect_shapes×2 (0x140edbf60/0x140edd250)、add_hole_at_shape_intersection 0x141cd3ce0 | 全（墙对 → 角点 y = max_y-0.67 → 形状交线 → RawWallHole{center,dims,kind} → WallHoles::add） |
| stitch_bricks.rs | spawn_stitches 0x141ce65b0、spawn_stitch_bricks 0x141ce7190 | 全（StairsFloatingBrickParams 0x40 槽 + BrickTransformSsbo 实例投影） |
| auto_clutter_edit.rs | （core recovered_systems::clutter::AutoClutterEdit 已恢复） | history_record_auto_clutter (1,1,WallId) + replay_auto_clutter 生长推进/移除 |

## 2. 关键决策

- AutoClutterState 布局按反汇编（0x20/0x48 两候选缓冲 + pending u16 + 区域表 + epoch），
  字段语义按源→输出映射表（clutter-extra §process 表）。
- on_windows 的 0x50 输入候选 / 6 分支宿主子结构 / 反射投影按 radare2 全函数反汇编实现。
- host 几何（门/窗/树/装饰物/城垛宿主）以宿主填充的资源/组件为缝（GateHost/WindowHost/
  TerrainDecoratorPositions/TreePositions/MerlonHostGeometryList），业务链（jitter/水检/
  键/写输出）为真实实现。
- WallHoles 洞 = core `RawWallHole{center:[f32;2], dimensions:[f32;2], kind}`。

## 3. 编译与验证

- `cargo check -p system-clutter`：通过（24 警告，无错误；temp workspace 验证环境同
  critters 说明）。
- 与 reconstruction/clutter 差异：mod.rs 布局/方法为真实实现（非占位）；process/
  add_* 输出记录按证据表；merlons/stitches 按 host_geometry 语义实现。

## 4. 未决（不伪造）

- populate 44KB 内联闭包（pick_clutter_mesh L918 / align_to_nearby_wall L944）精确策略；
  on_windows 写块位置槽 RBP-0x60 别名疑点；BrickTransformSsbo/StairsFloatingBrickParams
  剩余字段（0x18/0x2c/0x40/0x54 语义）；AutoClutterEdit 其余变体（证据仅证 Delete{id}）；
  replay 命令 stride 0x30 与 (1,1,WallId) 记录的封装关系；DebugRectangle3d 可视化（宿主
  debug 缝）。

## 5. FIX 交接（reviewer-B 21 条，2026-10-16 第二轮）

| # | 修复 | 落点 |
|---|---|---|
| F1 | mark_dirty 真实实现：懒网格格 → GlobalId 查区域表（hash_one 语义）→ 推 pending + 条目 +0x42=0xffff；未命中 unreachable panic | autoclutter/mod.rs（按 0x140ee48c0 反编译） |
| F2 | flush_candidates_pending_removal：排序（insertion_sort/driftsort 语义）→ Vec::remove（memmove 0x98）→ kind==4 Arc 释放 → 区域 candidate_index 一致性维护 | autoclutter/mod.rs（按 0x140ee4bf0 反编译） |
| F3 | populate 真实化：clutter_kind_for_cell 水位输入、区域几何 = Aabb2::from_center_radius（格中心+格半宽）、align_to_nearby_wall（就近墙法向）、懒网格登记 | populate_autoclutter_regions.rs |
| F4 | manage 增删语义纠正：新建 = spawn_empty+ColorId；删除 = insert<ClutterMarkedForDeletion>（不直接 despawn）；更新路径（closure_0/朝向/抖动/类型分支）接线 | manage_autoclutter_entities.rs 重写 |
| F5 | birdnest +0x08 = Arc 强计数（SpawnShared 枚举：ArcStrong/Prefab） | mod.rs SpawnShared + 各写入点 |
| F6 | process 比较器 = 源 +0x34 值；重叠矩形按区域几何（0x20 记录）；过滤条件 kind != 0 | process_autoclutter_candidates.rs |
| F7 | subscribe_to_tabloids：DirtyTilesPublisher（tabloid Publisher）subscribe 接线（= RaycastWorld::subscribe_to_changes 语义） | populate_autoclutter_regions.rs |
| F8 | spawn_stitches 位置 = 段法向归一（1/SQRT）+ x0y swizzle + jitter 沿法向 | stitch_bricks.rs |
| F9 | load_clutter_meshes：ClutterMeshTy 56 资产路径表 + register_all | load_clutter_meshes.rs |
| F10 | wall_is_halftimbered 真实：style byte >= 2（0x1408c5670 语义；经 WallAuthoritySnapshot.source.style） | construct_inter_roof_merlons.rs |
| F11 | 六处 calculate_hash 改固定种子（std DefaultHasher::new() 固定键，弃 hashbrown RandomState） | 全部哈希函数 |

编译：temp workspace 三 crate 全通过；已同步 `tiny-glade/crates/systems/clutter`。

## 6. F3 重写（re-analyst-B 契约 RE-ANALYSIS-F3.md，2026-10-16 第三轮）

`clutter_kind_for_cell` 恒 1 的骨架化已按机器码契约重写：

- **两字节判别**：区域表项 +0x40 mesh-type（28 项跳表 0x14304382c：0/1 加权随机挑网格、
  2 跳过、3/4 固定候选槽 +0x25/+0x26、7 计数累积 [cand+0xad]、9+ 第三跳表）+0x41 kind
  （5 项跳表 0x143043818：0/1/2/3/4、>4 default → 预设槽组 +0x48/+0x50/+0xc0/+0x168；
  万圣节独立槽组；槽记录 +0x30 == -1 哨兵 → 权重 0）。
- **水位阈值 = 0.0**（0x142960690；原 kind==2 && water<=0.01 模型删除——0.01 是
  mesh-type 0/1 位面落点的最小钳制 maxps(0.01,Δ)→floorf→xvy，0x1429cd300 packed）。
- **center 非恒格中心**：wet/type<3 = 加权挑网格落点；dry/type<3 = 固定位（记录 +0x24
  取负）；type>=3 = 圆周落点（θ = rng×2π，0x142934674，种子 = 键哈希）。
- **candidate_index 非恒 0**：= 区域候选表交互索引（add_region push 语义）。
- 键 = `calculate_hash<tuple<GlobalId,u32>>`（0x140fabc80，固定种子）。
- legacy_raycast 掩码 0x30fb 结果 ==2 门（非 native 壳返回未命中，同 core 壳）。

落地：populate_autoclutter_regions.rs `process_dirty_cell`/`weighted_mesh_landing`/
`circular_position`/`MeshGroupWeights`；三 crate 编译通过，已同步。
