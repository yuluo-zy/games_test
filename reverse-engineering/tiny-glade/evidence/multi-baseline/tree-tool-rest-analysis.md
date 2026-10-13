# tree-tool 剩余核心函数业务逻辑分析（B 主基线，反汇编级证据）

2026-10-13。本轮将 tree-tool 模块四个剩余核心函数从清单升级为反汇编级证据：
despawn_trees / replay_tree / recalc_tree_position_on_terrain_height_change /
tree_move_bend，并确认 reset_tree_tool 与 convert_preplaced_trees 边界。

证据来源（本次精读）：
- **reva**（B 持久库，worker `tree-rest-owner`，只读脚本）：四函数全指令反汇编 +
  void() 反编译辅助、函数边界/签名、bevy system_type / validate_param / run_unsafe
  包装器名称（携带 FunctionSystem 完整参数签名）、plugin build（0x140650520）反编译。
- **radare2**（RNE 发行原件 oracle）：关键分支逐条核对（UCOMISS/JBE、事件槽写回、
  prologue），与 reva/B 反汇编一致；读取 .data 浮点常量与 panic 字符串。
- **PDB 行号表**（skeleton/B/.../tree-tool/src/<file>.rs.pdb.jsonl.gz）：函数→源码行。

地址修正说明：前一轮 process-tree-cmds-analysis 记 despawn_trees 为 PDB VA
0x142268A30 且"与相邻函数边界不符"。本轮 probe 证实 0x142268A30 位于
**tree_move_bend**（0x1422689c0..0x142268dba）体内；真实 despawn_trees 主体 =
**0x1422686d0**（PDB 符号 + Ghidra 函数体一致，size 751）。

## 1) despawn_trees（process_tree_cmds.rs 的 Delete 实际销毁方）

- 入口 VA 0x1422686d0，size 0x2EF（751）；Ghidra 函数体
  `[[0x1422686d0,0x14226873c][0x142268740,0x1422687c8][0x1422687d0,0x14226886a]
  [0x142268870,0x1422689be]]`。源文件 process_tree_cmds.rs，PDB 行号覆盖 203–214。
- **系统签名（硬证据）**：bevy `system_type<...FunctionSystem<...>>` 包装器
  0x141bb2110 =
  `Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>, EventWriter<DespawnPrefabCmd>`。
  即 Delete 分支由 process_tree_cmds 打 TreeMarkedForDeletion 标记 + 播删除音效，
  本系统逐帧检查标记树动画状态并**实际发送 DespawnPrefabCmd**。
- **业务步骤**（反汇编证据，两处查询路径经 `[query_state+0x108]` 分派，bevy 查询内部）：
  逐实体取 TreeAnimation 列（archetype 列查找 @+0x110 索引）：
  1. 读 TreeAnimation f32 @+4（组件 stride 0x10）。
  2. `UCOMISS XMM6(0.01), f` + `JBE 跳过`：当 **f < 0.01**（严格小于，NaN 除外）时发送。
  3. 发送 DespawnPrefabCmd：事件槽 stride 0x10，`slot+0` = bevy 事件计数/id，
    `slot+8` = **Entity（u64）**（来自查询实体列，`[R8+idx*0x10]`）。
  4. 事件缓冲扩容走 alloc grow_one（bevy Events send 内部），panic 定位串
     `0x143034d10`（.cargo/registry 路径，alloc 内部）。
- **常量（radare2 确认）**：0.01f = 0x3c23d70a @ 0x142960674。
- **调用关系**：consumer = bevy schedule 包装器；callee 无游戏函数（全内联）。
  DespawnPrefabCmd 的下游 consumer 为 prefab 销毁系统（不在本模块）。
- **修正前未知项**：Delete 分支确认不发送 DespawnPrefabCmd；实际发送方为
  despawn_trees，payload = **entity**（非树 index）。
- **honest unknown**：TreeAnimation f32@+4 的确切字段名/语义（size 0x10，疑似
  淡出/进度计时，<0.01 = 动画近完成触发销毁）；entity 是否即 prefab 实例根实体
  （TreeMarkedForDeletion 在 Delete 分支经 Commands 加到 prefab 实例实体，假定成立）；
  `[query_state+0x108]` 分派的两 archetype 布局差异（bevy 内部）。

## 2) replay_tree（历史回放）

- 入口 VA 0x141bc34f0，size 0xAA2（2722）；函数体
  `[[0x141bc34f0,0x141bc3a10][0x141bc3a20,0x141bc3b79][0x141bc3b90,0x141bc3c17]
  [0x141bc3cf0,0x141bc3d0a][0x141bc3d26,0x141bc3d85]]`。源文件 replay_tree.rs，
  PDB 行号覆盖 17–121。
- **系统签名（硬证据）**：plugin build（0x140650520）`in_set<GameSystemSet>` +
  `chain_ignore_deferred` 注册 =
  `ResMut<ReplayBuffer>, EventWriter<InsertTreeCmd>, EventWriter<MoveTreeCmd>,
  EventWriter<DeleteTreeCmd>, Res<GameTime>`。
  寄存器映射：RCX=ReplayBuffer、RDX=Insert 写者、R8=Move 写者、R9=Delete 写者、
  [RSP+0x28]=GameTime。
- **业务步骤**（反汇编证据，448 条指令）：
  1. `ReplayBuffer::front_mut_bucket()`（0x1409f1a80）取缓冲桶（Vec<ReplayCmd>，
     stride 0x30）；空则返回。读 ReplayBuffer f32 @+0x28（延时除数）。
  2. 逐 ReplayCmd：tag 字节 @+0 非 8（TreeEdit）→ 跳过。
  3. TreeEdit 负载 = `[cmd+0x10]`；变体 u32 @+0x10：**1=Move、2=Insert、其余=Delete**。
  4. 若 cmd.elapsed（f32 @+0x18）== 0.0（非 NaN）→ 重发命令事件：
     - Move（变体 1）：index @+0x30，position @+0x20/+0x28（flag==0）或
       @+0x14/+0x1c（flag!=0）；MoveTreeCmd 槽 stride 0x20。
     - Insert（变体 2）：String::clone 名字，读 TreeEdit +0x18/+0x20/+0x38/+0x40/
       +0x44/+0x4c/+0x50；InsertTreeCmd 槽 stride 0x48。
     - Delete（其余）：index @+0x18；DeleteTreeCmd 槽 stride 0x10。
  5. 时序记账：flags 字节 @+8 bit0 == 0 → 记入待移除集合（立即回放）；否则（延时）：
     delay = `[cmd+0xc] / ReplayBuffer[+0x28]`（下钳 0.05=0x3d4ccccd）；
     若 delay < elapsed → 记入待移除；否则 `elapsed += GameTime[+0x28]`（delta）。
  6. 循环后：逆序待移除下标，逐 `Vec::remove` 出桶并按下标 tag 释放负载
     （switch 0–9：8 = `drop_slow<Arc<TreeEdit>>`）。
- **命令事件槽布局（与 process_tree_cmds 一致，reva/radare2 核对）**：
  - MoveTreeCmd 槽 0x20：{ u64 id @+0, u64 index @+8, Vec3 position @+0x10 }。
  - DeleteTreeCmd 槽 0x10：{ u64 id @+0, u64 index @+8 }。
  - InsertTreeCmd 槽 0x48：本函数从 slot+0 直接写 payload（**bevy id 不入槽**——
    印证"bevy 事件框架 id 放置不统一"）；process_tree_cmds 侧 id 写在 @+0、
    payload 从 @+8 起。两处均为同一 InsertTreeCmd 类型。
- **调用关系**：consumer = ReplayBuffer（country_core::resources::history::
  replay_cmd_buffer）；emitter = MoveTreeCmd/InsertTreeCmd/DeleteTreeCmd 事件 →
  process_tree_cmds 消费。callee：front_mut_bucket（0x1409f1a80）、
  sync_point_count（0x140949170，5 字节 stub）、String::clone（0x140d7bd90）、
  alloc grow_one/memmove/drop_slow（依赖内部）。
- **honest unknown**：TreeEdit（serde 序列化树编辑）变体字段完整布局（含
  `[cmd+0x1c]` flag 字节选两种负载布局的语义）；ReplayBuffer+0x28 除数语义
  （回放倍速）；front/back 双缓冲细节（ReplayBuffer 内部）。

## 3) recalc_tree_position_on_terrain_height_change（地形衔接）

- 入口 VA 0x142268dc0，size 0x386（902）；函数体
  `[[0x142268dc0,0x142268f0a][0x142268f10,0x142269023][0x142269030,0x142269053]
  [0x142269060,0x142269145]]`。源文件 recalc_tree_position_on_terrain_height_change.rs，
  PDB 行号覆盖 12–35。
- **系统签名（硬证据）**：system_type 包装器族（0x141bb2050 等）=
  `Query<&mut PrefabInstanceData>, Res<TerrainHeightsData<TerrainHeightSsbo>>,
  EventReader<OnTerrainChange>`。
  寄存器：RCX=Query、RDX=Res<TerrainHeightsData<TerrainHeightSsbo>>（采样用）、
  R8=EventReader<OnTerrainChange>（country_core::systems::terrain::
  delayed_terrain_changed_msg::OnTerrainChange）。
- **业务步骤**（反汇编证据，218 条指令）：
  1. prologue 内联 Events/EventReader"有新事件吗"检查（双缓冲 tick 运算，
     Events 结构 @+0x10/+0x18/+0x30/+0x38/+0x40）：无新事件 → 直接返回。
  2. 逐 PrefabInstanceData（stride 0x30）：写 bevy change tick。
  3. 边界门控：`|x| <= 65.0 && |z| <= 65.0`（XMM6 = 0x7fffffff abs 掩码 @0x14292e980；
     阈值 65.0 = 0x42820000 @0x142a83814；radare2 确认）。
  4. `position()`（0x140ad51a0，复制组件 Vec3@+0 的 x/z）→
     `TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>`（0x141246fb0）→ 高度。
  5. 若采样高度 != 已存 y（PrefabInstanceData+4）→ 写 `max(高度, 0.0)` 到 +4。
- **调用关系**：consumer = OnTerrainChange 事件（地形系统延迟发送）；callee =
  position（0x140ad51a0）、sample_at_world_xz<TerrainHeightSsbo>（0x141246fb0）。
- **honest unknown**：±65 边界语义（疑似世界 130×130 半宽裁剪）；"高度 <=0 钳 0"
  是否为地形法线方向约定；TerrainHeightsData<TerrainHeightSsbo> 资源内部布局。

## 4) tree_move_bend（弯曲）

- 入口 VA 0x1422689c0，size 0x3FB（1019）；函数体
  `[[0x1422689c0,0x142268a81][0x142268a90,0x142268aee][0x142268af0,0x142268dba]]`。
  源文件 tree_move_bend.rs，PDB 行号覆盖 8–42。
- **系统签名（硬证据）**：validate_param_unsafe 包装器 0x141bd92a0 =
  `Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>, Res<GameTime>`。
  寄存器：RCX=Query、RDX=Res<GameTime>（delta @+0x28）。
- **业务步骤**（反汇编证据，229 条指令，每实体）：
  1. `position()`（0x140ad51a0）取 PrefabInstanceData 位置 x/z 作目标。
  2. `utils::time_continuous_lerp`（0x140c904e0，= `target+(current-target)*expf(-delta*rate)`）
     两次：TreeMoveBend.x / .y 向目标缓动（delta=GameTime[+0x28]，rate=5.0）。
  3. 距离 `|bend-target| >= 0.1` 时：`step_vec = min(dist, 5.0) * (3.22/dist) *
     (bend-target)`（逐分量，反汇编 0x142268d09..0x142268d2a），
     `bend = target + step_vec`（ADDPS + MOVLPS 写回 TreeMoveBend）。
  4. 变更检测：step_vec 分量（MOVD 取 f32 位）与 PrefabInstanceData+0x24/+0x28
     存储值比较；不同则写 tick + `[+0x24] = step_x | step_y<<32`（打包 u64）。
- **常量（radare2 确认）**：0.1 @0x142925984（dist 下限）、5.0 @0x142925988
  （lerp rate）、3.22 @0x142925904（step rate）。
- **调用关系**：callee = position（0x140ad51a0）、time_continuous_lerp（0x140c904e0）。
  写 TreeMoveBend（Vec2，stride 8）与 PrefabInstanceData+0x24/+0x28。
- **honest unknown**：TreeMoveBend 与 PrefabInstanceData+0x24/+0x28（打包 int2）的
  视觉语义（疑似"树移动时滑移/倾斜量 + GPU 变更量"）；3.22 常数来源；PrefabInstanceData
  0x24/0x28 之外字段。

## 边界确认（内联/大函数，仅登记）

- **startup_tree_tool / reset_tree_tool**：无独立顶层符号（compiled_instances 空），
  内联进 bevy `run_unsafe<FunctionSystem<void(*)(ResMut<TreeStorage>)>>` 包装器
  （enriched inline_hosts 确认）。plugin build 显示注册于 **OnExit<AppLoadingState>**，
  签名 `fn reset_tree_tool(mut tree_storage: ResMut<TreeStorage>)`——装载完成时重置
  TreeStorage。内部仅清零 TreeStorage（map/next_index）。
- **convert_preplaced_trees**：入口 0x14226be70，size 0x12DB（4827），函数体
  `[[0x14226be70,0x14226ca91]...[0x14226cad2,0x14226cafb]]`。6 参数（RCX=Query、
  RDX=EventWriter、R8=标志/资源、[E+0x38]/[E+0x50]/[E+0x58]）。消费
  `SpawnPreplacedTrees`（plugin build 注册事件）历史树编辑
  （`Drain<TreeEdit>` → from_iter/fold 泛型 0x141bdb7e0/0x142272220，closure_env$1），
  调用 `WallSpatialHash::query_with_circle_callback<closure_env$0>`
  （0x1425defd0）、`TweakerView::as_inner_mut`、`PlacementDataSsbo::position`、
  `PrefabName::as_name_ref`、`History::insert_starting_edit`、
  `Decorators::iter_removed_ids`。疑似装载时把预置树转换为运行时树实体。
  本轮未逐条精读（内联严重），仅登记边界。

## 共享依赖函数签名（证据）

| 函数 | VA | 说明 |
|---|---|---|
| `utils::time_continuous_lerp` | 0x140c904e0（81B） | `current,target,delta,rate` → `target+(current-target)*expf(-delta*rate)` |
| `PlacementDataSsbo::position` | 0x140ad51a0（22B） | 复制组件 Vec3 @+0（x,y,z 12B），调用方取 x/z |
| `TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>` | 0x141246fb0（844B） | 世界 xz → 高度；除以 130.0 栅格采样，含 USE_FLOAT_GRID_SAMPLING_FIX |
| `ReplayBuffer::front_mut_bucket` | 0x1409f1a80（39B） | ReplayBuffer 取前端桶 |
| `sync_point_count` | 0x140949170（5B） | 空 stub（Ghidra 名 InputStream 误导） |

## PrefabInstanceData 布局（本轮新证据，size 0x30=48）

- +0x00：position Vec3（position() 复制 12B；recalc 更新 y@+4）。
- +0x0c：f32（process_tree_cmds Move 分支 inverse_lerp 读）。
- +0x24：f32 step_x（tree_move_bend 写，截断 int；变更打包 u64 写入）。
- +0x28：f32 step_y。
- 其余字段（+0x10..+0x24、+0x2c..+0x30）unknown。

## TreeMoveBend 组件（本轮新证据）

- Vec2（stride 8）：+0x00 f32 x、+0x04 f32 y。tree_move_bend 每帧写回。
- 语义 unknown（疑似树移动滑移量/弯曲量）。

## TreeAnimation 组件（本轮新证据，despawn 侧）

- size 0x10；f32 @+4（despawn_trees 与 0.01 比较）。字段名 unknown。

## 剩余 unknown（不伪造）

- despawn_trees 的 0.01 阈值浮点语义；entity 是否即 prefab 根。
- replay_tree 的 TreeEdit serde 布局 + ReplayBuffer+0x28 除数语义。
- recalc 的 ±65 边界语义。
- tree_move_bend 的 3.22 常数来源与 +0x24/+0x28 视觉语义。
- convert_preplaced_trees 完整业务（内联严重，仅边界）。
