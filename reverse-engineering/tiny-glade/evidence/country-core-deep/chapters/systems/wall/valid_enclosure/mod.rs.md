# systems/wall/valid_enclosure/mod.rs

> 文件作用：有效围合 ShapeParameters：Curve/Circle/Rectangle 枚举（tag@+0 位 0 判别、载荷@+4）；center/as_curve2/as_curve3/as_circle/as_rectangle/contains_point/contains_shape/perimeter/get_normal_at_u/get_pos_at_u/get_closest_u_from_pos/expand；serde 名 IsEnclosedNoYesPremadeShapeIsEnclosedV2。

函数总计 45：业务 20 / 内联无独立符号 2 / 派生胶水 23

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1416ebaf0 | 11 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >` | 0x1416ebe70 | 11 | 序列化 | I | wall\decomp\decomp-0x1416ebe70.txt |
| `country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x140259300 | 11 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1416eb640 | 11 | 序列化 | I | wall\decomp\decomp-0x1416eb640.txt |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>` | 0x1416ebf70 | 22 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1416ebea0 | 22 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >` | 0x1416ec140 | 22 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >` | 0x1416eca30 | 62 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::center` | 0x1411ff2a0 | 82 | ShapeParameters::center（15B 极短） | O | wall\decomp\decomp-0x1411ff2a0.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2` | 0x1411ff2b0 | 104 | ShapeParameters::as_curve2 | O | wall\decomp\decomp-0x1411ff2b0.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3` | 0x1411ff2f0 | 118 | ShapeParameters::as_curve3 | O | wall\decomp\decomp-0x1411ff2f0.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point` | 0x140ad2950 | 125 | ShapeParameters::contains_point/get_closest_u_from_pos（叶转发） | O | wall\decomp\decomp-0x140ad2950.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape` | 0x1411ff330 | 132 | ShapeParameters::contains_shape（parry3d 路径） | O | wall\decomp\decomp-0x1411ff330.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u` | 0x1411ff600 | 191 | ShapeParameters::get_normal_at_u（位 0 判别分发） | O | wall\decomp\decomp-0x1411ff600.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u` | 0x1411ff620 | 198 | ShapeParameters::get_pos_at_u（位 0 判别分发） | O | wall\decomp\decomp-0x1411ff620.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter` | 0x1411ff640 | 205 | ShapeParameters::perimeter（位 0 判别分发） | O | wall\decomp\decomp-0x1411ff640.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos` | 0x1411ff660 | 212 | ShapeParameters::perimeter（位 0 判别分发 + 载荷几何） | O | wall\decomp\decomp-0x1411ff660.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::expand` | 0x1411ff680 | 220 | ShapeParameters::expand（向外扩张 amount） | O | wall\decomp\decomp-0x1411ff680.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::as_circle` | 0x1411ff6f0 | 233 | ShapeParameters::as_circle/as_rectangle | O | wall\decomp\decomp-0x1411ff6f0.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle` | 0x1411ff740 | 241 | 访问器/转换 | I | wall\decomp\decomp-0x1411ff740.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::valid_enclosure::impl$0::from`（行 46）— 待推断
- `country_core::systems::wall::valid_enclosure::impl$2::from`（行 290）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_bytes`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str`、`visit_u64` 等 23 项

## 实现方式（算法级）

### ShapeParameters 记录布局（O，28B = 0x1c）

`{tag u32@+0, payload 24B@+4}`；**判别 = tag 位 0**（0=Circle，1=Rectangle；tag<2 即"预置形状"，>=2 为 Freehand 非本记录）。

- **circle payload 16B**：`cx f32@+4, cy f32@+8, r f32@+0xc, 备用 f32@+0x10`
- **rectangle payload 24B**：`cos f32@+4, sin f32@+8, cx f32@+0xc, cy f32@+0x10, hw f32@+0x14, hh f32@+0x18`（cos/sin = 旋转矩阵 {cos,sin,−sin,cos} 前两分量）

**对齐验证（O，多函数交叉）**：center 0x1411ff2a0 `mov eax,[rcx]; movss xmm0,[rcx+rax*8+4]; movss xmm1,[rcx+rax*8+8]` → 圆 {+4,+8}、矩形 {+0xc,+0x10}；contains_shape 读圆 r@+0xc、矩形 cos/sin@+4/+8、center@+0xc、hw/hh@+0x14/+0x18；expand 写 f64@+0x14（= 矩形 hw/hh）。

### 方法（O/I，含手解字节）

| 方法 | 地址 | 算法 |
|---|---|---|
| center | 0x1411ff2a0 | `payload[tag*8]` 两 f32（圆 +4/+8，矩形 +0xc/+0x10） |
| as_curve2 | 0x1411ff2b0 | 圆 → `FUN_140c97cf0(payload, 0.2f)`；矩形 → `FUN_140c98cf0(payload)` |
| as_curve3 | 0x1411ff2f0 | 圆 → `FUN_140c97e10(payload, 0.2f, z)`；矩形 → `FUN_140c991f0(payload, z, 采样数)` |
| get_normal_at_u | 0x1411ff600 | 位 0 双路尾跳：圆 → 0x140c978e0；矩形 → 0x140c99e10 |
| get_pos_at_u | 0x1411ff620 | 圆 → 0x140c975a0；矩形 → 0x140c99800 |
| get_closest_u_from_pos | 0x1411ff640 | 圆 → 0x140c97970；矩形 → 0x140c99530 |
| perimeter | 0x1411ff660 | 圆 → 0x140c976f0（2πr）；矩形 → 0x140c9b000（I：2·(hw+hh)·系数） |
| expand | 0x1411ff680 | 整记录拷贝 `{tag,qword@+4,f32@+0xc,u32@+0x10}`；**圆**：`r@+0xc += amount`，且 `f64@+0x14 += (2a,2a)`（2 f32 车道）；**矩形**：`f64@+0x14 = (2a,2a)`（**hw=hh=2·amount 覆写**，语义 U——调用方不在 wall 域 decomp，见 legacy） |
| as_circle | 0x1411ff6f0 | 圆 → `&payload`；矩形 → **fmt panic**（`FUN_1428d9390(&UNK_142c19910,&UNK_142c19920)`，`{:?}` 断言） |
| as_rectangle | 0x1411ff740 | 矩形 → `&payload`；圆 → **fmt panic**（UNK_142c19950/142c19960） |
| contains_point | 0x140ad2950 | 位 0 尾跳：圆 → 0x140c980ee；矩形 → 0x140c9a62e；本体 @+0x15（圆：|P−C|²<r² 含 f64 精度 movsd） |
| contains_shape | 0x1411ff330 | 见下 |

### contains_shape 算法（O，0x1411ff330）

- **self=circle**：取 other 中心（圆 → `FUN_140c975a0(payload,0)`；矩形 → `FUN_140c99600(payload,0)`），**`|other.center − self.center|² < self.r²`**（严格 <；把 other 视为点，保守包含）。
- **self=rectangle**：逐点测试 other 的采样点（全部在矩形内才 true）：
  - other=circle：采样 **u ∈ {0, 1/6, 1/3, 1.0, 2/3, 5/6}**（常量 0x142c197c8=1/6、0x142a86330=1/3、0x142925870=1.0、0x142b1e8b8=2/3、0x142c197cc=5/6）；
  - other=rectangle：**4 角**（`FUN_140c99600(payload,0..3)`）；
  - 每点先经 `FUN_140c9a630`（**Rectangle2d::contains_point** 叶）粗测，最后一点再走**内联精确判定**：旋转到局部系 `rx = dx·cos − dy·sin, ry = dx·sin + dy·cos`，判 `|rx|<=hw && |ry|<=hh`（K=1.0@0x14293ac60..6c，`movmskps==0xf`）。

### 叶函数（utils/geometry，共享库无 PDB，地址供 central 关联）

- **circle as_curve2** 0x140c97cf0：`count = clamp(ceil(r·2π/step), 3, …)`（2π=0x40c90fdb@0x142934674；min 3 点；负→0，超 i64 域→3）；随后闭包 0x1419401b0 计算**累积弦长**（弧长参数化 → Curve2 {points, cum_len}）。源码 `crates/utils/geometry/circle.rs`（@0x142b2e478 区路径串）。
- **circle get_pos_at_u** 0x140c975a0：`θ = (1−u)·2π`（模 2π 归一）；`P = center + r·(cosθ, sinθ)`（cos=0x142923e80、sin=0x142923fd0；u=0 → 点 (cx+r, cy)）。
- **circle get_normal_at_u** 0x140c978e0：同 θ；返回 (cosθ, sinθ) 切线/法向。
- **circle perimeter** 0x140c976f0：2πr。
- **rect as_curve2** 0x140c98cf0：4 角 = `center ± R·(hw·c, hh·c)`（c=−1.0@0x14295a0e8 参与旋转铺排），**闭合回路 5 点**（首角重复），再由闭包 0x1419401b0 加累积弦长；面积 `hw·hh<=0` → 空曲线早退。
- **Rectangle2d::contains_point** 0x140c9a630：局部系 AABB 判定（0x1429258f0 符号 XOR 旋转）。

### 常量表（新增）

| 地址 | 位型 | 值 | 含义 |
|---|---|---|---|
| 0x142934674 | 0x40c90fdb | 6.2831855 (2π) | 圆采样角度全周 |
| 0x1429cd068 | 0x3e4ccccd | 0.2 | circle as_curve2 步长 |
| 0x14295a0e8 | 0xbf000000 | −1.0 | 矩形角铺排系数 |
| 0x142925870 | 0x3f000000 | 1.0 | contains_shape 圆采样 u / as_curve3 u |
| 0x142c197c8 | 0x3e2aaaab | 1/6 | 圆采样 u |
| 0x142a86330 | 0x3eaaaaab | 1/3 | 圆采样 u |
| 0x142b1e8b8 | 0x3f2aaaab | 2/3 | 圆采样 u |
| 0x142c197cc | 0x3f555555 | 5/6 | 圆采样 u |
| 0x14293ac60..6c | 0x3f000000 | 1.0 | 矩形包含 K（无 epsilon） |

**纠错**：① `contains_shape` 是"点采样包含"而非 parry3d 碰撞（DIGEST §2.6 的 parry3d 路径串属于依赖侧 cuboid，本函数无 parry 调用）；② `get_closest_u_from_pos` 真身 @0x1411ff640（PDB 表把 perimeter 挂在 0x1411ff640、get_closest_u 挂 0x1411ff660 —— 实为 **get_closest_u_from_pos@0x1411ff640、perimeter@0x1411ff660**，按任务文档 & 手解字节序为准）；③ PublicWallState 形状区 @+0x258 = `{tag u32, payload@+0x25c}` 扁平记录，shape_placement/preview_snapping_line/contains_shape 交叉一致（Ghidra 在 preview_snapping_line 把 +0x25c 8B 读误标指针，实为 payload 拷贝）。
