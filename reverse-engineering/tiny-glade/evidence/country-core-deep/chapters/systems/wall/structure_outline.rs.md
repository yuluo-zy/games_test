# systems/wall/structure_outline.rs

> 文件作用：结构描边系统：消费 OutlineWallCmd/OutlineStairsCmd，Esc 菜单关闭才画；逐墙 ShapeParameters（as_rectangle/as_circle/as_curve2/center/expand/perimeter）+ 曲线采样到 UiSignifierStream 描边 signifier（outline_wall_top_/bottom_/roof_ridge_/roof_side_/circle_*/rect_corner_ 名前缀）。

函数总计 17：业务 1 / 内联无独立符号 0 / 派生胶水 16

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::structure_outline::structure_outline` | 0x140959fc0 | 77 | structure_outline（Esc 门控 + ShapeParameters + outline_* signifier 族） | O | wall\decomp\decomp-0x140959fc0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 16 项

## 实现方式（算法级）

### structure_outline 0x140959fc0（O，结构级；98KB 大系统）

签名：~15 参；param_1/param_2 = 事件流（func_0x000140c16a00/c18130/c16d40/1428f87d0 事件读取器，行 0x18B），param_10/param_11 = 资源，param_12 = AppMode 资源。

**门控与输入**：
1. 事件读取器初始化 + panic 路径（`str::slice_error_fail` 0x1428d9940 @行 0x41=65 / 0x39=57，消息 = 模块路径串 `country_core::systems::wall::structure_outline::structure_outline::f`，file = structure_outline.rs；`str::rfind` 0x140959670 解析路径末段）。
2. **AppMode 门控**：`tag==2 || (tag==10 && 子字节==0)`（O，机器码 `cmp qword [*param_12]`）——非"Esc 门控"；Esc/菜单关闭属另一资源，U。
3. OutlineWallCmd / OutlineStairsCmd 事件行 0x18B：`{u32 id1@+1, f32@+2, u32 id2@+0xc, byte@+0x14, byte@+0x15}`；`f32@+2 > 0` 才处理。两条扫描分支：byte@0x15==0 → 公共墙表（步长 0x2a8，匹配 cand+0x4f / cand+0x27c 双 id）；否则 → `StairsState::get_all_connected_nodes`（0x1408fe8c0）。
4. 命中墙 → PublicWalls 值区；`count>=2 && length>0` 断言（public_walls.rs:50，行 0x32，UNK_142ab4dd8）。

**形状分支（tag@value+0 = uVar14 <2 = 预置形状）**：
- 预置：`ShapeParameters::as_curve2`（0x1411ff2b0）+ `perimeter`（0x1411ff640）+ `Rectangle2d::new_centered_axis_aligned`（0x140c98890）；`Vec<Vec3>::from_iter` 采样闭包（0x141302550 closure_env$8 / 0x1412fd970 closure_env$3 / 0x1412ff650 closure$9 / 0x1412fcdf0 closure_env$6）——沿曲线/形状逐 u 采样生成描边点。
- 屋顶：`Roof::profile_curve_normalized`（0x1408e3420）参与 roof 描边采样；`f32::clamp` panic（0x140d8b480）。
- Freehand：另一曲线采样路径（closure$1 = 0x14095fca0）。
- 高度/法向：0.1f（0x142925984）/ 0.4f（0x142925970）高度参与。

**signifier 输出**（`SignifierFactory::new` 0x14089d8b0 + `Arc::new<closure_env>` 0x140959b80/0x140959c40 + `AppendOnlyVec::push<UiAppendSignifier>` 0x14146c880 + 事件流 0x20B 行 kind=5）：

| 名前缀 | 地址 | 使用行 | 绘制路径 |
|---|---|---|---|
| `outline_wall_top_` | 0x142ab4e70 | 715（预置）、1378（freehand） | 墙顶描边 |
| `outline_wall_bottom_` | 0x142ab4e08 | 1225 | 墙底描边 |
| `outline_roof_side_` | 0x142ab5170 | 1046 | 屋顶侧描边 |
| `outline_roof_ridge_` | 0x142ab5108 | 1149 | 屋顶脊描边 |
| `outline_circle_roof_` | 0x142ab5080 | 854 | 圆顶描边 |
| `outline_circle_side_` | 0x142ab4ff8 | 1484 | 圆侧描边 |
| `outline_rect_corner_` | 0x142ab51c8 | 1611 | 矩形角描边 |
| `outline_side_left` / `outline_side_right` | 0x142ab4ef0 / 0x142ab4f38 | 常量区 | 侧边描边（调用点 U） |

**采样步长**：`uVar66 * 0xb`（11）增量（I：每段 11 采样 / 采样步长 1/11）。

panic/断言：`EventReader` 错误（slice_error_fail 0x1428d9940）、public_walls.rs:50、iterator `.next().unwrap()`（file = /rustc/.../core/src/iter/traits/iterator.rs，0x142ab4b68）、f32 clamp panic（f32.rs，0x142aa5190）。

### 遗留 U
- Esc/菜单关闭资源与门控位。
- `uVar66*0xb` 采样步长精确语义。
- 各 signifier 的 {animation_duration, custom_shader} 参数（Arc'd closure_env 内）。
