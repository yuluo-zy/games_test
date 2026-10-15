# systems/wall/snapping/telegraph_walls_are_merged.rs

> 文件作用：合并预告线系统：PlayLineAcrossWall{a,b} 到 Curve::crashy_new_from_points+crashy_slice 连线两墙端点到 LineAcrossWallSignifier（animation_duration/custom_shader）。

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::telegraph_walls_are_merged::impl$0::register_required_components` | 0x14086d1b0 | 21 | 吸附/数值 | U |  |
| `country_core::systems::wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged` | 0x14086ad70 | 38 | telegraph_walls_are_merged（连线两墙端点 signifier） | O | wall\decomp\decomp-0x14086ad70.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

### telegraph_walls_are_merged 0x14086ad70（O，结构级）

**PDB 名错位**：decomp 头标注 `impl$0::register_required_components`，索引登记为 telegraph_walls_are_merged（register_required_components 真身 0x14086d1b0，U）。

签名（8 参）：param_1 = 事件流 / Query，param_2 = &PublicWalls（plVar42 = *param_2；+0x18 ctrl、+0x20 哈希态），param_3 = 资源，param_4 = 墙值引用（*param_4 = pppppfStack_228），param_5 = 资源，param_6 = 资源，param_7 = &mut Mut\<UiSignifierStream>（+0x1c tick；行 0x20B {seq@0, byte@+8=5, 8B@+9, 8B@+0x11, f32@+0x18}），param_8 = 资源。

处理链（逐墙生成预告线）：
1. **遍历 PublicWalls**（hashbrown 0x2a8 步长，全墙）。每墙：
   - `count>=2 && length>0`（value+0x10/+0x30）失败 → panic（public_walls.rs:50，行 0x32，UNK_142a96e50）。
   - `count = simplified 曲线点数@value+0x48`；越界上限（>0xaaaaaaaaaaaaaa）→ panic；`count==0` → 分配 4B 占位。
   - 分配 `count*0xc` 字节（12B Vec3 点阵）；`func_0x000140613c10`。
2. **曲线平移复制**（SIMD 0x30B 块 ×4 点）：新点 = 旧点 + 偏移；偏移 = `f32@value+0x284 × 0.5f（0x142925870）× [0|1]`（fVar70 = 0, fVar71 = off*0.5）；尾数单点循环（12B/点）。语义：把墙曲线按 wall 起始偏移平移生成预告线（O，分量对应 I）。
3. `Curve::try_new_from_points<Vec3,3D>`（0x141469230）→ 新曲线。
4. **写 signifier 流**（AppendOnlyVec::push\<UiAppendSignifier>，0x14146c880）：行 {seq, kind=5, pos 8B, u64, f32}。
5. 注册绘制回调（0x28B/0x20B 记录写 FUN_1412dc8f0 / FUN_1412d8c60 + FUN_141564d30 函数指针 + u64 0x4040000000000001）——I：线条绘制分发（Lines/Sphere 类）。

panic/断言：
- `assertion failed: signifier.straighten_curve_cache.length_ws() != 0.0`（0x142a96d8c，行 0x45=69）。
- `assertion failed: !u.is_nan()` / `assertion failed: (0.0..=1.0).contains(&t)`（0x142aa53d0，curve_u.rs:29，行 0x1d）——CurveU 投影校验。
- `Result::unwrap()`（0x142c63340，行 0x2b=43，snapping_maintain.rs 域）。
- `panic_bounds_check`（curve/mod.rs 域，0x142c38b78）。
- `handle_alloc_error`（0x1428d9000）。

常量：0.5f（0x142925870）、0.5f（0x1429258e0/e4）、f32@value+0x284 偏移量（O 字段，U 语义）。

### 遗留 U
- 两墙端点的**成对**关系：机器码按单墙平移复制，未见 {a,b} 双墙配对（PlayLineAcrossWall 事件源待核）。
- kind=5 signifier 行结构、0x4040000000000001 常量、FUN_1412dc8f0/1412d8c60/141564d30 回调语义。
- value+0x284 偏移字段名。
