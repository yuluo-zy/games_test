# resources/stairs/stairs_assembly_params.rs

> 文件作用：**手工楼梯装配参数族：锚点/对齐/边标识（供 stairs_assemble 系统消费）**

函数总计 16：业务 8 / 内联无独立符号 0 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests::prev_requests` | 0x140274480 | 40 | 查询/计算 | O：PlayermadeStairsAssemblyRequests::prev_requests=&self+0x138 | collision-eco\decomp\decomp-0x140274480.txt |
| `country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams::length_ws` | 0x140959ca0 | 81 | 查询/计算 | O：PlayermadeStairsAssemblyParams::length_ws——点/高 zip→Vec3 链墙空间总长 | collision-eco\decomp\decomp-0x140959ca0.txt |
| `country_core::resources::stairs::stairs_assembly_params::impl$2::from` | 0x140959e70 | 126 | 转换 | I：From 转换 |  |
| `country_core::resources::stairs::stairs_assembly_params::StairWallAnchor::new` | 0x140959e80 | 147 | 构造 | O：StairWallAnchor::new——16B 拷贝 {Vec2 anchor, f32 angle, pad} | collision-eco\decomp\decomp-0x140959e80.txt |
| `country_core::resources::stairs::stairs_assembly_params::StairWallAnchor::wall_normal` | 0x140959e90 | 152 | 查询/计算 | O：wall_normal=(sin(angle),cos(angle)) | collision-eco\decomp\decomp-0x140959e90.txt |
| `country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall` | 0x140959ef0 | 165 | 查询/计算 | O：StairNodeAnchor::try_as_wall——判别==2 才返 &payload(+8)（Wall 判别=2） | collision-eco\decomp\decomp-0x140959ef0.txt |
| `country_core::resources::stairs::stairs_assembly_params::AlignKind::position_offset` | 0x140959f00 | 209 | 查询/计算 | O：AlignKind::position_offset——变体1={flag u8@+1,Vec2@+4,Vec2@+0xc} 组合+常量缩放 | collision-eco\decomp\decomp-0x140959f00.txt |
| `country_core::resources::stairs::stairs_assembly_params::AlignKind::outgoing_dir` | 0x140959f50 | 239 | 查询/计算 | O：AlignKind::outgoing_dir——判别 0/1 否则 panic | collision-eco\decomp\decomp-0x140959f50.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`eq`、`fmt`、`hash` 等 8 项

## 实现方式（算法级）

- `PlayermadeStairsAssemblyParams::length_ws`（0x140959ca0）：点/高 zip → 链式 Vec3 段长求和（墙空间总长）。
- `AlignKind::position_offset`（0x140959f00）：判别 byte0；变体 1 = {flag u8@+1, Vec2@+4, Vec2@+0xc} 组合 + 常量缩放（重建 stub 返 ZERO 错）；`outgoing_dir`（0x140959f50）：判别 0/1 否则 panic。
- `StairNodeAnchor::try_as_wall`（0x140959ef0）：判别==2 才返回 &payload(+8)（Wall 判别=2）。
- `StairWallAnchor::new`（0x140959e80）：16B 拷贝 {Vec2 anchor, f32 angle, pad}；`wall_normal`（0x140959e90）= (sin(angle), cos(angle))。
