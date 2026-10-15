# geometry/sphere.rs

> 文件作用：**程序化球体几何（icosphere）**（MODULE_GAP 判 0% 覆盖，本节补齐）。`sphere::try_from` 从序列化数据构建细分球体，依赖 `hexasphere-12.0.0`（公开 crate，直接复用不逆向库实现）。

函数总计 2：业务 1 / 派生胶水 1。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$2::try_from` | 0x140a06b00 | 19 | **业务** `TryFrom<Sphere> for Mesh`（icosphere 构建入口）：见下方算法节。签名（机器码）＝ `(out: &mut Mesh, subdivisions: u64, radius: f32)` → 即 `Sphere { subdivisions, radius }` 两字段（reconstruction 仅 radius 一字段——登记差异）。 | **O** | decomp-0x140a06b00.txt |

## 派生胶水

`impl$0::from`（1 项，From 转换）。

## 算法级结论（round 算法深挖，2026-10-15）

### `TryFrom<Sphere> for Mesh`（0x140a06b00）全算法

1. **细分上限检查**：`subdivisions > 0x4f(=79)` → 格式化 panic `Cannot create an icosphere of {subdivisions}…`（0x142ac6758，fmt arg = `core::fmt::num::impl$9::fmt` 0x140d8d6c0）。
2. **hexasphere 调用**：`hexasphere::Subdivided::new<[f32;2], IcoSphereBase, closure_env$0>`（0x14176b7a0）——库侧生成细分球（顶点 16B stride = Vec3A）；游戏侧闭包 = **按 radius 缩放顶点**（`Vec::from_iter<[f32;3], Map<Copied<Iter<Vec3A>>>>` 0x1413096c0：每顶点 x,y,z × radius → 12B Vec3 缓冲）。
3. **网格装配**：
   - `Mesh::set_attribute<Vec<[f32;3]>>`（0x140a34c90）×2：`Vertex_Position`（0x142ac6553，len 15）+ `Vertex_Normal`（0x142ac6562，len 13）；
   - `Mesh::set_attribute<Vec<[f32;2]>>`（0x140a355f0）：`Vertex_Uv`（0x142ac656f，len 9）；
   - 索引：`hexasphere::Triangle::add_indices`（0x141797940，0x58-stride 三角形记录循环）填充三角索引；
   - 收尾 `drop_in_place<Subdivided<[f32;2],IcoSphereBase>>`（0x140878b40）释放库中间体。
4. 输出 Mesh（param_1 14 qword 区）+ 分配失败 panic（0x142ac66a8 = sphere.rs 源路径，0x142ac6328 = core iterator.rs collect 路径）。

要点：
- **库实现不逆向**（hexasphere 12.0.0 公开依赖）：游戏侧仅"参数校验 + 半径缩放 + 装配 Mesh 属性/索引"。
- 三角数/顶点数公式未在游戏侧出现（由 hexasphere 内部产生）；`(sub+1)²×10+2` 常量仅在 panic 格式参数区出现（推断为库顶点数提示，非游戏逻辑）。
- 8 个调用点（reconstruction 注）：由各消费方（球体装饰/碰撞预览等）复用该构建入口。
