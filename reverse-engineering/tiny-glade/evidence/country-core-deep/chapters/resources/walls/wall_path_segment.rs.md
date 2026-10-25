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
## 实现方式（算法级）

### WallPathSegment 布局（O 手解）

- +0x20：预备点数组（f32 点，步长 8B=Vec2）。
- +0x58：Vec（cap-first）长度取用（pt_count 尾跳共享 helper 0x1412ef94c）。
- 点计数@+0x28（length_ws 读取 `mov rax,[rcx+0x28]`）。

### pt_count 0x141234970（O 手解）

`4883c158 e9d7b9fcff` = `add rcx,0x58; jmp rel32` → 尾跳 0x1412ef94c（Vec 长度 helper）取 +0x58 Vec 的 len。

### length_ws 0x141234980（O 手解，3D 逐段长度）

```
mov rax,[rcx+0x28] 点计数；xorps xmm0（累加=0）
if (计数 < 2) → return 0（jb +0x138）
r8 = 点数组@+0x20
循环: 取点 i/i+1 三分量 → subss（差分）→ sqrtss（f30f51c3）→ 累加
尾部 movshdup/addss 归并 → f32 返回
段长 = Σ|P[i+1]-P[i]|（3D，含 Y）
```

### length_ws_2d 0x141234ad0（O 手解，XY 平面）

同 length_ws 骨架但无 Z 分量：`0f14c3 unpcklps / 0f5cc3 subps`（XY 差分）→ `0f59c0` 平方和 → `sqrtss` 逐段 → 累加；点步长 ×4（`48c1e002`）。段长 = Σ|P[i+1]-P[i]| 仅 XY。

### 跨模块调用

共享 Vec 长度 helper 0x1412ef94c；length_ws/2d 供墙切分/轮廓预备路径。

### 遗留 U

- +0x20/+0x28 之外字段（起点/终点 u、seam 信息）U。
