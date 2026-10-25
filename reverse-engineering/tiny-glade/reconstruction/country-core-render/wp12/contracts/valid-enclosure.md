# 有效包围（valid_enclosure）契约 — valid_enclosure/mod.rs

## ShapeParameters（数据族；PDB 过程 0x1411ff2a0..0x1411ff6f0 + serde 0x1416eb640..）

**枚举（O）**：`Curve(Curve)` / `Circle(Circle2d)` / `Rectangle(Rectangle2d)`；serde
serialize/deserialize 族（0x1416eb640..0x1416ed020，O 过程规模）。

**方法（O 调用链，各 leaf 反编译规模）**：
| 方法 | 地址/规模 | 语义 |
|---|---|---|
| `center` | 0x1411ff2a0 (15B) | 形状中心 |
| `as_curve2` / `as_curve3` | 0x1411ff2b0/2f0 (55/61B) | 转为 2D/3D 曲线 |
| `as_circle` / `as_rectangle` | 0x1411ff6f0 (77B) | 变体访问 |
| `contains_point` | 0x140ad2950 (21B) | 点包含 |
| `contains_shape` | 0x1411ff330 (720B) | 形状包含 |
| `perimeter` | 0x140ad2950 | 周长 |
| `get_normal_at_u` / `get_pos_at_u` | 0x140ad2950 | 曲线采样 |
| `get_closest_u_from_pos` | 0x140ad2950 | 最近 u |
| `expand` | 0x1411ff680 (105B) | 向外扩张 amount |
| `from`（impl$0/impl$2） | — | circle/rectangle 互转 |

**GPU 效果**：无直接；被 structure_outline / shape_placement_signifiers / ui_continue_wall
的 `contains_point` 消费。
**typed gap**：Circle/Rectangle 内部尺寸（utils geometry，复用依赖）；包含/扩张算法 U
（除叶规模 O 外，行为未逐值恢复）。
