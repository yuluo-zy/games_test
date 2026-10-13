# tree-tool 第二批补全：convert_preplaced_trees / reset_tree_tool / ui_* 业务逻辑（B 主基线，反汇编级证据）

2026-10-13。第二批补全 tree-tool 剩余核心函数：convert_preplaced_trees（预放置树转换）、
startup_tree_tool.rs 的 reset_tree_tool（全内联边界/行为）、ui_place_tree / ui_move_tree /
ui_delete_tree（UI 命令入口）。此前已完成 process_tree_cmds / despawn_trees / replay_tree /
recalc_tree_position_on_terrain_height_change / tree_move_bend 的反汇编级证据（见
tree-process-cmds-analysis.md 与 tree-tool-rest-analysis.md）。

证据来源（本次精读）：
- **reva**（B 持久库，worker `tree-extra-owner`，只读脚本）：`0x14226be70` 全 631 条指令
  反汇编 + void() 反编译、`ui_place_tree/place_tree/random_tree_prefab/ui_move_tree/
  ui_delete_tree` 反汇编/反编译、reset_tree_tool 的 FunctionSystem 族（0x141bb19d0 run_unsafe
  等 22 个符号）、`run_without_applying_deferred`/`run_unsafe` 包装器族定位与调用点上下文
  （0x141bca200/0x141bda260）、find-cross-references 确认 convert 的两个真实调用者。
- **radare2**（RNE 发行原件 oracle）：convert/三个 ui 系统入口 prologue 逐条核对，与 B 一致；
  读取 .data 常量/字符串（0x143034a10/0x143034a20、0x142a7cd48、0x142925904、日志位置记录）。
- **PDB 行号表**（skeleton/B/.../tree-tool/src/<file>.rs.pdb.jsonl.gz）：函数→源码行。
- **插件注册**（plugin build 0x140650520 反编译）：register_event<PlaceTreeCmd/MoveTreeCmd/
  DeleteTreeCmd/InsertTreeCmd/SpawnPreplacedTrees/HideTreesinAreaCmd/HideTreesNextToWallCmd>、
  insert_resource<TreeStorage>、add_systems<OnExit<AppLoadingState>, fn(ResMut<TreeStorage>)>。

## 1) convert_preplaced_trees（预放置树转换）

- 入口 VA **0x14226be70**，size 0x12DB（4827）；Ghidra 函数体
  `[[0x14226be70,0x14226ca91][0x14226ca94,0x14226ca9f][0x14226caa2,0x14226cab0]
  [0x14226cab3,0x14226cabe][0x14226cac1,0x14226cacf][0x14226cad2,0x14226cafb]]`。
  源文件 convert_preplaced_trees.rs，PDB 行号覆盖 32–137；内联 closure$0（line 102，
  = `WallSpatialHash::query_with_circle_callback` 的闭包）、closure$1（line 129，
  = `Drain<TreeEdit>`→EditType 映射闭包）。
- **业务（确认）**：把装载期以 SpawnPreplacedTrees 事件声明的"预放置树"（首发资产级静态树）
  转换为运行时树流程：延迟数帧等装载事件停止→遍历墙空间哈希内的树→水/岩石掩码门控→
  对通过者经 EventWriter 发出转换命令并记录历史起始编辑（History::insert_starting_edit）。
- **参数布局（反汇编证据，7 参数）**：prologue RBP=entry RSP-0x268，栈参数换算
  [RBP+0x290]=arg5、[RBP+0x298]=arg6、[RBP+0x2a0]=arg7、[RBP+0x2a8]=arg8。
  | 参数 | 证据 |
  |---|---|
  | arg1 RCX | **EventReader**（入口 tick 双缓冲运算，SpawnPreplacedTrees；见延迟模式） |
  | arg2 RDX | **EventWriter**（写 stride 0x10 事件：slot+0=bevy id、slot+8=树 u64） |
  | arg3 R8 | ResMut<{u64 计数 @+0, u8 标志 @+8}>（转换状态） |
  | arg4 R9 | Res（传给 PathRastersResources::as_float_grid，水面栅格） |
  | arg5 [E+0x28] | Res<TweakerView>（`as_inner_mut` 0x14067dd80） |
  | arg6 [E+0x30] | Res<WallSpatialHash>（哈希桶 @+8 stride 0x14、实体表 +0x110/+0x118） |
  | arg7 [E+0x38] | 资源（+0x10 存指针被写、+0x1c 取值，疑似 History/事件写入面） |
  调用点 0x141bca978（在 run_without_applying_deferred 0x141bca200 体内）确认栈参数
  [RSP+0x40/0x48/0x50]=arg5/6/7，RCX=LEA[RSP+0x90]，R8=[RSP+0x88]。
- **延迟转换模式（反汇编）**：有 SpawnPreplacedTrees 事件帧 → {计数=1,标志=1}；事件停后
  计数自增，计数>2 时清标志并执行转换块 + 日志 "Adding converted pre_placed tree"
  （.data 0x143035608，line 74）。
- **转换步骤（反汇编）**：`FrozenMap::get<SsboId,Box<BufferState>>` 取
  TerrainRockMaskSsbo（assert 名字 len 0x3f，0x143034a80 附近）；`Decorators::iter_removed_ids`
  （0x140a1b100）迭代移除装饰 id 位图；对每棵树 `PlacementDataSsbo::position`（0x140ad51a0）
  + 6 次 `bilinear_sample_at_world_xz<RasterBufferRef<512>>`（0x140a14df0）+ 岩石掩码
  `sample_at_world_xz`（0x1409ea800）；水中（采样>0）或岩石上→跳过；否则 4 采样和钳 [0,1]，
  若 ≤0 → `Circle2d::new`（0x140c97540）+ `query_with_circle_callback<closure_env$0>`
  （0x1425defd0）；闭包命中则组内部 stride-0x48 条目并经 arg2 发送 stride-0x10 事件
  （payload=树 u64 @slot+8）。
- **收尾**：`from_iter<EditType, Map<Drain<TreeEdit>, closure_env$1>>`（0x141bdb7e0；
  fold 泛型 0x142272220）+ `History::insert_starting_edit`（0x140a844f0），日志
  "History inserted starting edit w {}"（.data 0x1430356d8，line 70）。
- **调用关系**：真实调用者 = bevy `run_without_applying_deferred`（0x141bca200）与
  `InfallibleSystemWrapper::run_unsafe`（0x141bda260，.llvm 合并块；find-cross-references
  totalTo=2）；另有 .data 0x14376f434 指针。consumer 事件：arg2 EventWriter（树 u64 载荷，
  疑似 DespawnPrefabCmd，未钉死）。
- **honest unknown**：convert 的 FunctionSystem 完整签名（包装器名被 ICF/合并块吞并，无法从
  名字读取；ABI 以反汇编为准）；arg2 事件确切类型（stride 0x10/u64 载荷，疑似 DespawnPrefabCmd）；
  arg3 {u64,u8} 资源归属；arg7 资源归属；`<0.6/1.0` 常数（0x142925904=1.0、0x142925908≈0.6）
  的精确语义（随机<0.6 门控疑似复用，place_tree 也有 0.6）。

## 2) startup_tree_tool / reset_tree_tool（全内联边界与行为）

- startup_tree_tool.rs 仅 1 个函数：**reset_tree_tool**（enriched functions=1，无独立
  compiled_instances，compiled_instances 空 → **全内联**；inline_hosts=2 于
  `run_unsafe<FunctionSystem<ResMut<TreeStorage>>>` 与
  `run_without_applying_deferred<InfallibleSystemWrapper<FunctionSystem<ResMut<TreeStorage>>>>`）。
- **注册（硬证据）**：plugin build（0x140650520）`add_systems<..., OnExit<AppLoadingState>,
  fn(ResMut<TreeStorage>)>`——装载状态退出时重置 TreeStorage。
- **FunctionSystem 族（确认）**：single-param `void(*)(ResMut<TreeStorage>)` 族共 22 个符号，
  run_unsafe = **0x141bb19d0**（body [0x141bb19d0,0x141bb1b7e]，90 条指令），system_type =
  0x141bb20e0。启动/运行全部内联在此 run_unsafe 内。
- **行为（反汇编确认）**：run_unsafe 取到现有 TreeStorage 资源后**覆写默认值 0x40 字节**：
  `[RBX+0]=XMM0([0x143034a10])`、`[RBX+0x10]=XMM0([0x143034a20])`（全 0 = 空 hashbrown 头）、
  `[RBX+0x20]=XMM6`（call_once KEYS 随机键）、`[RBX+0x30]=0`（**next_index 归零**）；随后
  `[RSI+0x148]=1`（变更 tick）。即 `*tree_storage = TreeStorage::default()` = 清空 map +
  next_index=0。radare2 确认 0x143034a10={0x143034a00,0,0,0}、0x143034a20=全 0。
- **honest unknown**：0x143034a00 指向内容；KEYS 随机键与 hashbrown 头的字段切分。

## 3) ui_place_tree / place_tree / ui_move_tree / ui_delete_tree（UI 命令入口）

| 函数 | VA / size | 源行 | 业务（确认） |
|---|---|---|---|
| ui_place_tree | 0x142269170 / 0x5A1(1441) | 30–113 | 种树工具 UI 画布系统：读工具状态 0x10、`DEV_OVERRIDE_IS_DEMO_MODE`(0x14362c098)、CursorTerrainRaycast、GladeBorder::is_pos_inside；glade 内点击→发 **PlaceTreeCmd**（stride 0x18，payload {位置/高度 f32 @+8, u32 @+0xc, u32 @+0x10}）；glade 外点击→发 2 个空载荷事件（UI 反馈，身份 unknown）。**不直接发 InsertTreeCmd** |
| place_tree | 0x142269970 / 0x7F6(2038) | 157–190 | **系统签名（硬证据）**：`ResMut<TreeStorage>, EventReader<PlaceTreeCmd>, EventWriter<InsertTreeCmd>, ResMut<History>, Res<ActiveSession>`（run_unsafe 0x141bd45e0 族 / run_without_applying_deferred 0x141bcaeb0）。消费 PlaceTreeCmd（槽 stride 0x18）→ 随机树名（random_tree_prefab + fastrand，`<0.6` 门控 0x142925908）、TreeStorage::next_index → 发 **InsertTreeCmd**（stride 0x48，slot+0 直写 payload，与 process_tree_cmds/replay_tree 一致；name/position/height/byte/flags/index），History::add |
| random_tree_prefab | 0x14226a170 / 0xE1(225) | 192–225 | 随机树 prefab 名选择（process_tree_cmds Insert 分支同款） |
| ui_move_tree | 0x14226b1a0 / 0xCC4(3268) | 32–148 | 移动树 UI：工具状态 6..21（11=移动）、iter_removed_ids、树空间哈希遍历；拖拽（slider3d::into_drag_state）内 glade → 发 **MoveTreeCmd**（stride 0x20：index u64 @+8、position Vec3 @+0x10，与 process_tree_cmds 一致）；glade 外发 2 个 8B 事件（unknown）；松开记录 Move TreeEdit（variant u32@+0x10=1）History::add；signifier closure_env$0（0x1425df760/0x141bd4120） |
| ui_delete_tree | 0x141bd2510 / 0x74F(1871) | 21–103 | 擦除树 UI：擦除工具状态 9/子态 6、clearing_canvas_w_layer、CursorTerrainRaycast；对 iter_removed_ids 位图内距光标半径内的树发 **DeleteTreeCmd**（stride 0x10：index u64 @+8，与 process_tree_cmds 一致），History::add（variant u32@+0x10=2） |

- 三系统均在 `DEV_OVERRIDE_IS_DEMO_MODE`(0x14362c098 全局字节) 非 0 时直接返回。
- **修正任务假设**：ui_place_tree 发 **PlaceTreeCmd**（非 InsertTreeCmd）；InsertTreeCmd 由
  同文件独立系统 **place_tree** 生成。ui_move_tree→MoveTreeCmd ✓、ui_delete_tree→DeleteTreeCmd ✓。
- **honest unknown**：PlaceTreeCmd payload 字段精确语义；ui_place_tree 两个空载荷事件身份；
  ui_move_tree glade 外两个 8B 事件身份；TreeEdit/EditType 判别值（place=0/delete=2/move=1
  与 replay_tree 的 1/2/else 需对齐）语义；各系统 FunctionSystem 剩余参数。

## 事件槽布局（延续 process-tree-cmds-analysis.md，本轮新增核对）

| 命令 | 槽 stride | 槽内字段 |
|---|---|---|
| PlaceTreeCmd | 0x18 | payload u64 @+8（高度/位置打包）+ u32 @+0x10（place_tree 读） |
| InsertTreeCmd | 0x48 | place_tree/replay_tree 从 slot+0 直写 payload；process_tree_cmds 侧 id @+0、payload @+8 |
| MoveTreeCmd | 0x20 | id @+0；index u64 @+8；position Vec3 @+0x10 |
| DeleteTreeCmd | 0x10 | id @+0；index u64 @+8 |
| convert arg2 事件 | 0x10 | id @+0；树 u64 @+8（类型待定） |

## 剩余 unknown（不伪造）

- convert 的 FunctionSystem 完整签名 / arg2 事件类型 / arg3、arg7 资源归属。
- TreeEdit 判别值在 History::add / insert_starting_edit / replay_tree 之间的一致性。
- PlaceTreeCmd payload 与 ui_place_tree 两个空载荷事件的语义。
- ui_move_tree glade 外两个 8B 事件语义。
- reset 默认 TreeStorage 的 0x143034a00 内容与 hashbrown 头字段切分。
