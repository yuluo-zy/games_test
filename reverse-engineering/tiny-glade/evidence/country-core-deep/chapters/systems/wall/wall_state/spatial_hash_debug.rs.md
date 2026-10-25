# systems/wall/wall_state/spatial_hash_debug.rs

> 文件作用：空间哈希调试可视化：DebugGui 开时逐格 DebugRectangle2d + 成员线（MouseDepthValue 平面）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug` | 0x1408ec0e0 | 0 | spatial_hash_debug（逐格 DebugRectangle 调试） | O | audio-misc\decomp\decomp-0x1408ec0e0.txt, wall\decomp\decomp-0x1408ec0e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

### spatial_hash_debug 0x1408ec0e0（4 系统参数，调试可视化）

**参数面**（O）：param_1 = Res<WallSpatialHash>（`lVar3=*param_1`），param_3 = gizmo/绘制资源（`uVar2=*param_3`，FUN_140b04f70 目标），param_4 = DebugGui 资源（**`*(*param_4 + 0xf1) == 1` 门控**），param_2 = 未用。

**处理链**：
1. 门控 false → 直接返回。
2. **画网格线**（y=0 平面）：
   - 顶点计算：`fVar19/fVar21 = {+0x50,+0x54} × _UNK_142aa5d60/64`（=−0.5 → min 角），`fVar22/fVar23 = × _UNK_142aa5d68/6c`（=+0.5 → max 角）；`FUN_140c9cb90` 包成 Vec2 对。
   - **竖线**（行 86–109，i 从 1..dims.x）：`x = lerp(min_x, max_x, i/dims.x)`，线 = {x,0,min_y}→{x,0,max_y}，0x18B 数据（6 f32 = 2×Vec3），`FUN_140b04f70(&cmd, drawer)` 绘制。
   - **横线**（行 116–142，i 从 1..dims.y）：{min_x,0,y}→{max_x,0,y}，`y = lerp(min_y,max_y,i/dims.y)`。
3. **格中心 DebugRectangle**：`FUN_1409f0e80(lVar3, &circle, &cell_list)` = get_cells_from_circle<spatial_hash_debug closure> 收集命中格索引（Vec {ptr,cap=8,len}）；`FUN_140c97e10`/`FUN_140c97540` 构造 debug 圆（MouseDepthValue 平面）；逐格 `FUN_140c98980`（cell_size 换算）画矩形（`FUN_140b04f70`）。
4. **成员线段**：`FUN_140aa5940(lVar3, &circle, &member_list)` = query_with_circle 闭包（version 去重）→ 逐成员 `func_0x000140c9cde0(&fStack_198, member)` 读两端点，画 3D 线（scale=3.0，`uStack_e8=0x40400000`）。

**数据结构**：命令结构 = `{2,2,i64::MIN(0x8000000000000000),4,scale,1}` + 数据指针（i64::MIN = Option None 哨兵，PLAYBOOK §2.3）。成员指针列表 Vec cap-first {ptr@+8, len@+0x10}。

**常量**：`_UNK_142aa5d60/64`=−0.5f（min）、`_UNK_142aa5d68/6c`=+0.5f（max）→ 网格角；`_UNK_142925870`=0.5f；`_UNK_14295a0e8`=−0.5f；`_UNK_142925904`=1.0f（cell_size 换算）；scale 常量 1.0/3.0（0x3f800000/0x40400000）。

**跨模块 callee**：FUN_140b04f70（gizmo 绘制命令分发，**非 WallSpatialHash 查询**）、FUN_1409f0e80（get_cells_from_circle）、FUN_140aa5940（query_with_circle 闭包）、FUN_140c98980/FUN_140c9cb90/FUN_140c9cde0/FUN_140c97540/FUN_140c97e10（gizmo 图元构造）、func_0x000142923ee0（floor thunk，cell_center 换算）。

### 纠错
- **FUN_140b04f70 = 绘制命令**（O）：命令结构 {2,2,i64::MIN,4,scale,1}+线/矩形数据，非 DIGEST 推断的 WallSpatialHash 查询；DIGEST §2.1 对 spatial_hash_debug "合同 O（调试 UI，未深挖）" 现补为逐格网格线+格矩形+成员线段三段绘制。
