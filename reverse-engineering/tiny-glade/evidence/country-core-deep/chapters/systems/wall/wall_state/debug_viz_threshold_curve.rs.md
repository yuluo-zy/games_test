# systems/wall/wall_state/debug_viz_threshold_curve.rs

> 文件作用：墙高阈值曲线调试：逐公共墙沿发布曲线画 DebugSphere。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve` | 0x140a8b0c0 | 11 | debug_viz_threshold_curve（逐墙 DebugSphere） | O | audio-misc\decomp\decomp-0x140a8b0c0.txt, wall\decomp\decomp-0x140a8b0c0.txt |

## 实现方式（算法级）

### debug_viz_threshold_curve 0x140a8b0c0（3 系统参数，调试可视化）

**参数面**（O）：param_1 = DebugGui 资源（**`*(*param_1 + 0xf3) == 1` 门控**，与 spatial_hash_debug 的 +0xf1 为同一资源不同开关位），param_2 = Res<PublicWalls>（`plVar14=*param_2`，0x2a8 map，mask=`plVar14[1]`，`plVar14[3]`=表项数判空），param_3 = gizmo/绘制资源。

**处理链**：
1. 门控 false → 返回。
2. **逐墙遍历 PublicWalls**（0x2a8 hashbrown SIMD ctrl 组探针，行 135–149）：取 `wall_id = key@-0x2a8`。
3. 每墙：`plVar14[3]==0`（空表）→ panic（`FUN_1428d9310(&UNK_142ad9250)` + `w.simplified_curve_2d.is_valid()`，行 151–155）；hash 二次定位条目（行 160–216）；有效性断言（行 217，`value+0x10<2 || value+0x30<=0`）。
4. **取四条曲线计数最大值**：`uVar15 = max(value-0x208, value-0x1d0, value-0x1b8, value-0x1a0)`（行 220–229；相对条目基址，即发布曲线/高度等 4 个数组的长度）；为 0 → 跳过该墙。
5. **逐点画 4 个 DebugSphere**（`FUN_140b02520(&cmd, drawer)`，行 255–285）：
   - 半径/颜色常量（u32 位型）：0x3dcccccd(0.1f)/色1、0x3d4ccccd(0.05f)/色2、0x3e19999a(0.15f)/色4、0x3e19999a(0.15f)/色3；scale = 1.0f/2.0f/1.0f/1.0f（0x3f800000/0x40000000）。
   - 位置来源：曲线点数组（`value-0x210`，12B 步长 `puVar16+=0xc`）+ 三个 f32 高度/派生数组（`value-0x1d8`、`value-0x1c0`、`value-0x1a8`，4B 步长）。2D 球用点坐标，另 3 球用高度值与点坐标合成（`(float)(y_hi32)` 等）。

**数据结构**：命令结构同 spatial_hash_debug（`{2,2,i64::MIN,4,scale,1}`+数据，FUN_140b02520 = DebugSphere 绘制命令）。

**常量**：半径 {0.1,0.05,0.15,0.15}f、scale {1.0,2.0,1.0,1.0}f、颜色 {1,2,4,3}。

**跨模块 callee**：FUN_140b02520（DebugSphere 绘制）、FUN_1405b0d70（hash）、FUN_1428d9310/FUN_1428d9430（panic 胶水）。

**遗留 U**：四条曲线（-0x208/-0x1d0/-0x1b8/-0x1a0）与三个高度数组（-0x1d8/-0x1c0/-0x1a8）的字段名（发布曲线/高度阈值曲线/…），仅知步长与相对基址；`param_1 & 0xffffffff` 截断寄存器的原因。
