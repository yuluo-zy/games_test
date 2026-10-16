# RE-ANALYSIS: ui_clutter（交互与世界组 · ui_clutter）

> 会话：interaction re-analyst（2026-10-15）。范围：`systems/ui_clutter/{ui_autoclutter,
> rmb_autoclutter, debug_spawn_clutter, decoclutter_show_pins, mod}` + `resources/{clutter_meshes,
> preplaced_autoclutter, history/auto_clutter_edit}`。
> 依据：B EXE 机器码 + 既有 decomp（ui-input/collision-eco 域）+ 本轮新反编译
> （auto_clutter_edit serde：interaction-group/decomp/decomp-0x14003d300.txt、
> decomp-0x140a18500.txt）。记号：O/I/U；「旧结论 X → 新证据 Y」=修正。不改 reconstruction 源码。

## 1. 函数清单表

| 函数 | 地址 | 状态 | 行为一句话 |
|---|---|---|---|
| ui_convert_autoclutter_to_decoclutter | 0x14099ca60 | O（8.1KB） | 右键 auto→deco 转换（4 事件族） |
| get_clutter_rot_in_roof_space | 内联（行 403） | I | 屋顶空间旋转（无独立符号） |
| maintain_autoclutter_parent_hashmap | 内联（行 410） | I | 维护 clutter→parent 映射（无独立符号） |
| handle_rmb_autoclutter | 0x140976ca0 | O（1.4KB 手解） | 右键移除 auto-clutter（hashbrown SIMD 定位 + 旋转 + 0x48B 事件 + History::add tag 0xd） |
| debug_spawn_all_clutter | 0x1409a1f50 | O | 单条命令 + 42 项螺旋散布（sqrt/cos/sin + 类型表 + 地形高度采样） |
| decoclutter_show_pins | 0x1412357c0 | O（6.7KB） | 按 ShowPinnedDecoClutter 切换固定图钉可见性 |
| mod.rs（impl$1::default） | — | O（胶水） | ui_clutter 子模块聚合容器 |
| ClutterMeshTy::from(&str) | 0x1408b83d0 | O（68KB） | 56 变体资产路径匹配 + unreachable panic |
| clutter_mesh_ty_data | 0x1408b8e90 | O | 静态表 switch(0..0x37) → 8B {名,kind} |
| preplaced_autoclutter（default） | — | O（胶水） | 预放置自动杂饰资源（无独立业务过程） |
| auto_clutter_edit serialize | 0x14003d300 | O（473B 本轮） | serde `{"<field>": {"<key>": u64}}` 单字段 |
| auto_clutter_edit deserialize | 0x140a18500 | O（1183B 本轮） | __Field 字段名 visitor（派生 Deserialize） |

## 2. 每函数行为级结论

### 2.1 ui_convert_autoclutter_to_decoclutter @0x14099ca60（O）

- 右键把 auto-clutter 转 deco-clutter：事件族 InstaCreateDeco / InstaCreateStairPoint /
  StartPlaceDecoClutterInteraction / EvRightClickedAutoClutter（wp20 合同 + 本域复核）。
- 变换计算：`FUN_1408b83d0`(ClutterMeshTy::from)×4 + 常量区 UNK_1429cd1/1429258（旋转/变换），
  生成放置数据并推 0x48B 事件；panic：FUN_1428d9940（path/越界）。
- 内联：get_clutter_rot_in_roof_space（行 403，I：屋顶空间旋转）、
  maintain_autoclutter_parent_hashmap（行 410，I：clutter→parent 映射，hashbrown 维护）。

### 2.2 handle_rmb_autoclutter @0x140976ca0（O 本轮机器码复核）

参数面：param_1=命中位置 Vec3（fVar23..fVar27）、param_2=事件载荷块、param_3=HashMap
（表@+0x30、掩码@+0x38、数据@+0x48..0x50）、param_4=组件表、param_5/6=History 类、
param_7=ResMut 事件 Vec（cap@+0x20、ptr@+0x28、len@+0x30、+0x40 计数）。
```
if param_2+0x18(计数)==0 → 空路径
hashbrown SIMD probe（FUN_1405b0d70 + 16B 位图）定位命中 clutter 条目（0x10B 键，-1 哨兵扫描）
  → 条目 +0x10 索引 → 组件表（param_4）取类型化数据（uVar19*0x48 步距，u32 位集 bitmap 判定）
朝向/旋转：
  fVar30 = 点积(_UNK_1429a9080/84/88 × 位置分量)
  二次/交叉项 → SQRT（func_0x000142923e20）^ _UNK_1429258f0（= 旋转平方）
  fVar23 = _UNK_142a10ff0 − FUN_1416f3210(...)   # acos 近似（_UNK_142a10ff0=π 候选）
  abs(_UNK_142925870 0.5 乘) → sin(func_0x142923fd0)/cos(func_0x142923e80) 归一
FUN_1408b83d0(param_2+0x30, param_2+0x38)  # ClutterMeshTy::from(&str) → 类型 u8
推 0x48B 事件到 param_7 Vec：
  {pos 12B@0, …@+0x29 byte=3(subtype), rot u32@+0x2c, entity@+0x30, f32@+0x38, type u8@+0x3c,
   entity u64@+0x40}
FUN_140a86270（事件发送）
alloc 0x18B Arc{1,1,data} → History::add 0x140a84ec0（tag 0xd，&UNK_142ab6980）
panic：FUN_1428d9760(&UNK_142ab66f0, 0x2b=43, …, &UNK_142ab6968)（组件表越界 bounds 检查）
```
- 章节「消费右键命中后 hashbrown SIMD 查找」「推 0x48B EvRightClickedAutoClutter 事件
  （pos/rot、@+0x29=3）」「末尾 alloc 0x18B Arc → FUN_140a84ec0(tag 0xd) 入历史」全部机器码证实。

### 2.3 debug_spawn_all_clutter @0x1409a1f50（O 本轮机器码复核）

- param_2=Events<DebugSpawnClutterCmd>（单条）、param_1=Events<DebugSpawnAllClutterCmd>（批量）。
- **单条命令**：推 0x48B spawn 事件 `{tag=2@+0, subtype=3@+0x29, type u8@+0x3c, entity@+0x40}`。
- **批量命令（螺旋散布 0x2a=42 项）**：
  ```
  for n in 0..0x2a:
    r = SQRT(n)                       # sqrt(n)
    θ = n * _UNK_142abbcec            # 黄金角候选
    x = cos(θ)*r （func_0x142923e80）、z = sin(θ)*r（func_0x142923fd0）
    y = TerrainHeightsData::sample_at_world_xz 0x1409e9640(grid, x, z)   # 地形高度
    type = (&UNK_142aa0013)[n]        # 逐项类型字节表
    推 0x48B {tag=2@0, subtype=3@+0x29, rot=0@+0x2c, pos.x@+0x30, pos.y@+0x34, pos.z@+0x38,
             type@+0x3c, entity@+0x40}
  ```
- 章节「0x2a=42、r=sqrt(n)、θ=n*_UNK_142abbcec、cos/sin、逐项类型字节 (&UNK_142aa0013)[n] +
  FUN_1409e9640」全部机器码证实。用途=确定性铺满场景（调试）。

### 2.4 decoclutter_show_pins @0x1412357c0（O，既有维持）

- 遍历 pinned clutter 集合，按 ShowPinnedDecoClutter 状态切换图钉实体可见性；
  局部 helper FUN_141233f80（图钉槽查找）。事件/串区 UNK_142c1e9/142c1e6；
  FUN_1428d9390（索引分发表）。
- register_required_components @0x141237200（O：bevy 组件注册）、clone_behavior @0x140002370
  （ICF 折叠单行）。

### 2.5 mod.rs（O 胶水）

- 仅 1 项 default 派生胶水；聚合 6 个 clutter 系统（debug_spawn_clutter/decoclutter_show_pins/
  defered_autoclutter_history/rmb_autoclutter/ui_autoclutter/ui_focus_decoclutter），add_ui 挂入。

### 2.6 clutter_meshes（O）

- **ClutterMeshTy::from(&str) @0x1408b83d0（68KB）**：56 变体按资产路径匹配
  （anvil/door_bell/door_krans/stall_bread/rack_brooms/flowerbox/birdnest/firewood/
  cart_w_pumpkins/cart_w_p_candles 等举例名已证）；未识别 → panic
  "clutter mesh type not found: {name}"（内部 unreachable）。
- clutter_mesh_ty_data @0x1408b8e90：静态表 switch(0..0x37) → 8B 条目 {名 ptr, kind}。
- 消费方：populate_autoclutter_regions（0x1422d6ed0）发布 DirtyTiles（碰撞域交叉）。

### 2.7 preplaced_autoclutter（O 胶水）

- 仅 `impl$1::default`（derive 胶水）；无独立业务过程（资源结构字段待 reconstruction 合同）。

### 2.8 auto_clutter_edit（resources/history/auto_clutter_edit.rs，O 本轮）

- **serialize<Compact> @0x14003d300（473B）**：写 `{` → 字段名（format_escaped_str）→ `:` →
  `{` → `serialize_entry<str, u64>`（单键值对）→ `}` → `}`。**serde 形态 = 单字段 struct
  `{"<name>": {"<key>": u64}}`**（字段名/键名串未取，U 低优先）。
- **deserialize<serde_json::Value> @0x140a18500（1183B）**：派生 Deserialize 的 __Field
  field-name visitor（`variant_seed<PhantomData<__Field>>` + struct_variant），JSON map 遍历，
  未知名 → invalid_value。运行侧消费：replay_auto_clutter 0x140aa4a60（undo==0 插墙集 /
  !=0 从 DeletedAutoClutter erase (DecoratorId,DecoratorId)）。
- **旧结论维持**：AutoClutterEdit = 墙派生装饰记录（host wall_id + decorator_ids）；
  手写 serde visitor 族（serialize 0x14003d300 / deserialize 0x140a18500 本轮落盘）。

## 3. 0x48B spawn 事件统一布局（debug_spawn + rmb_autoclutter 互证，O）

```
0x48B：{pos/数据 12B@0 … , subtype u8@+0x29, rot u32@+0x2c, pos/entity@+0x30..0x38,
        type u8@+0x3c, entity u64@+0x40}
```
- rmb_autoclutter 写 rot@+0x2c = 旋转角度（acos/sin/cos 链）、type@+0x3c=ClutterMeshTy。
- debug_spawn 写 tag=2@0、subtype=3@+0x29、rot=0、pos.x/y/z@+0x30/34/38、type@+0x3c。

## 4. 未决（U，不臆造）

1. auto_clutter_edit serde 的字段名/键名字符串（0x14003d300 引用的编译期串，未 --fat 取）。
2. get_clutter_rot_in_roof_space / maintain_autoclutter_parent_hashmap（内联无符号）行为细节
   （I，宿主 = ui_convert_autoclutter_to_decoclutter 0x14099ca60 60KB 内联体）。
3. decoclutter_show_pins 的 pinned 集合数据结构（FUN_141233f80 图钉槽查找内部）。
4. preplaced_autoclutter 资源字段（默认派生，结构未知）。
5. `_UNK_142a10ff0`（handle_rmb_autoclutter 的 acos 基数，π 候选）数值未逐位核。

## 5. 证据落盘（本轮新增）

- `evidence/country-core-deep/interaction-group/decomp/decomp-0x14003d300.txt`
- `evidence/country-core-deep/interaction-group/decomp/decomp-0x140a18500.txt`
