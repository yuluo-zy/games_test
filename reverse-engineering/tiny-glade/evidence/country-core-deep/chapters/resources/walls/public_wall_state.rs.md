# resources/walls/public_wall_state.rs

> 文件作用：公共墙状态 PublicWallState 0x2a0B：raw Curve2{ptr@+8,len@+0x10,长@+0x30}、3D 曲线@+0x88、高度数组{ptr@+0xe0,len@+0xe8}、max_y/gable 缓存{+0x108/+0x10c, +0x110/+0x114}、形状记录 28B@+0x258（0=Circle/1=Rectangle/2=Freehand）、发布长度@+0x28c、style@+0x299；方法面含 wall_space/u_space（Option<f32>，u_space=1.0f 常量）与锚高投影。

函数总计 17：业务 12 / 内联无独立符号 2 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape` | 0x140b2a5d0 | 126 | get_wall_shape（tag<2 否则 unwrap panic） | O | wall\decomp\decomp-0x140b2a5d0.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape` | 0x140b2a600 | 131 | try_get_wall_shape（tag>=2 → None） | O | wall\decomp\decomp-0x140b2a600.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::max_y` | 0x140b2a620 | 146 | max_y（+0x108 位0 → +0x10c，否则 panic wall heights not calculated） | O | wall\decomp\decomp-0x140b2a620.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable` | 0x140b2a680 | 151 | max_y_gable（+0x110 位0 → +0x114；PDB 名 is_premade_shape 错位） | O | wall\decomp\decomp-0x140b2a680.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y` | 0x140b2a6e0 | 156 | flat_roof_y（max_y - 0.79） | O | wall\decomp\decomp-0x140b2a6e0.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::wall_space` | 0x140b2a740 | 161 | wall_space（{tag!=2, f32@+0x28c} = Option<f32> 发布长度） | O | wall\decomp\decomp-0x140b2a740.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::wall_space_length` | 0x140b2a760 | 173 | wall_space_length（纯 +0x28c） | O | wall\decomp\decomp-0x140b2a760.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::u_space` | 0x140b2a770 | 177 | u_space（{tag!=2, 1.0f@0x142925904} = Option<f32> 归一全区间） | O | wall\decomp\decomp-0x140b2a770.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle` | 0x140b2a790 | 184 | is_rectangle（tag==1 复制 24B） | O | wall\decomp\decomp-0x140b2a790.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::is_circle` | 0x140b2a7d0 | 195 | is_circle（tag==0 复制 16B） | O | wall\decomp\decomp-0x140b2a7d0.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz` | 0x140b2a800 | 217 | corner_pos_and_normal_xz（仅 Rectangle：Rectangle2d::get_point/get_corner_normal） | O | wall\decomp\decomp-0x140b2a800.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height` | 0x140b2a890 | 231 | get_wall_attachment_height（premade→max_y；freehand→u×raw_len→clamp01→2D 插值→3D 投影→高度数组插值） | O | wall\decomp\decomp-0x140b2a890.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape`（行 113）— 谓词/校验
- `country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed`（行 118）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项
