# resources/walls/wall_path_segment.rs

> 文件作用：墙路径段（WallPathSegment）：墙切分/轮廓预备中间态，含预备点数组与 2D/3D 长度（pt_count/length_ws/length_ws_2d）。

函数总计 4：业务 3 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count` | 0x141234970 | 21 | WallPathSegment::pt_count（尾跳到 +0x58 Vec 长度取用） | O | wall\decomp\decomp-0x141234970.txt |
| `country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws` | 0x141234980 | 24 | WallPathSegment::length_ws（+0x20 点数组逐段 3D 长度累加 sqrt） | O | wall\decomp\decomp-0x141234980.txt |
| `country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d` | 0x141234ad0 | 32 | WallPathSegment::length_ws_2d（同 length_ws 仅 XY） | O | wall\decomp\decomp-0x141234ad0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`clone` 等 1 项
