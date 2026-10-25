# systems/render_loop/draw_passes/common_draw_params.rs

> 文件作用：共享 draw 绑定参数（所有 draw pass 复用的绑定）：`CommonDrawParams::new`（帧时间 uniform）、`DrawBuilder::bind_common`（shadowmap + Transform 矩阵 + uniform）、墙色/屋顶色自定义、砖擦除高亮、石/木地板纹理绑定。

函数总计 8：业务 5 / 内联无独立符号 3 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common` | 0x14090b460 | 50 | 通用绑定：读 `param_2+0xe0` 设备指针（+0x1f8）；`cVar2=*(param_3+1)` 写回 `param_2+0xf0`（shadowmap 绑定开关）；从 param_4（Transform）浮点构造 3×3 旋转矩阵（fVar4/fVar5/fVar1/fVar6 = x/y/z/w，2x 项公式 0x14090b460:69-92）+ 平移/缩放行；经 FUN_141629b90/141629a20/141629d20（hashes 0x14658de104bef99d/0x1e5485ef2e9f9384/0x531bb9318eea4971）绑定矩阵 + shadowmap + uniform；FUN_1416286f0 收尾 | O | render-loop\decomp\decomp-0x14090b460.txt（矩阵构造明确：0x14090b460:69-92 正交旋转行 fStack_1ac/19c/18c） |
| `country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wall_color_customization` | 0x14090bb70 | 69 | 绑定墙色自定义纹理：read_texture（墙色），wp5 O | O | render-loop\decomp\decomp-0x14090bb70.txt；wp5 draw-passes §9 |
| `country_core::render::pass_builder::draw_builder::DrawBuilder::bind_brick_erase_highlight` | 0x14090be10 | 93 | 砖擦除高亮：`CursorTerrainRaycast::position` + uniform（光标位置），wp5 O | O | render-loop\decomp\decomp-0x14090be10.txt；wp5 draw-passes §9 |
| `country_core::render::pass_builder::draw_builder::DrawBuilder::bind_stone_floor_textures` | 0x14090c300 | 116 | 绑定石地板纹理：read_texture（wp5 O） | O | render-loop\decomp\decomp-0x14090c300.txt；wp5 draw-passes §9 |
| `country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wooden_floor_textures` | 0x14090c980 | 167 | 绑定木地板纹理：read_texture（wp5 O） | O | render-loop\decomp\decomp-0x14090c980.txt；wp5 draw-passes §9 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `CommonDrawParams::new`（行 26）— `utils::get_res` + `Duration::as_secs_f32`（帧时间 uniform，wp5 O）
- `DrawBuilder::bind_roof_color_customization`（行 84）— read_texture（屋顶色）
- `DrawBuilder::bind_window_color_customization_textures`（行 89）— read_texture（窗色）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `bind_common`（0x14090b460，0x14090b460:69-92）—— 3×3 旋转矩阵构造
算法（O，decomp 逐行）：
1. 读 `param_2+0xe0` 设备指针（+0x1f8）；`cVar2 = *(param_3+1)` 写回 `param_2+0xf0`（**shadowmap 绑定开关** = CommonDrawParams.flag u8，B_i_api §1.1 纠错：bool uniform 值 = `flag==0`）；
2. 从 `param_4`（Transform `{rotation Quat@0, translation@0x10, scale@0x1c}`）浮点构造 **3×3 旋转矩阵**：fVar4/fVar5/fVar1/fVar6 = x/y/z/w 四元数 → 正交旋转行（`fStack_1ac/19c/18c`，2x 项公式 = `compute_matrix`）→ 平移/缩放行；
3. 经三个 uniform 单态化（hashes `0x14658de104bef99d`/`0x1e5485ef2e9f9384`/`0x531bb9318eea4971`）绑定矩阵 + shadowmap + 帧时间（`CommonDrawParams::new` = `Duration::as_secs_f32`）；
4. FUN_1416286f0（`bind_color_library<DrawBuilder>`）收尾。
- **CommonDrawFlag**（O，B_i_api §2）：`#[repr(u8)]` 0 Off/1 On/2..=4 ShadowCascade0..2；`shadow_pass` 传 `cascade+2`；`write_uniform<u32>` 写整个字节（2..=4 不被截成 1）。
- 消费方：**全部 raster draw pass**（main/transparency/shadow/wireframe/ui/deferred…）经 `run_list_pass` 的公共绑定序。

### 2. 颜色自定义绑定族
`bind_wall_color_customization`（0x14090bb70）/`bind_brick_erase_highlight`（0x14090be10，A/C/D 资源 + 三次 write_uniform，draw_builder 章 §4）/`bind_stone_floor_textures`（0x14090c300）/`bind_wooden_floor_textures`（0x14090c980）：各为 read_texture 序列（墙色/石地板/木地板纹理）+ uniform；被 draw_lists 回调（env12/13/16/17/34 等）绑定序复用。
- 纠错：旧版 `bind_roof_color_customization`/`bind_window_color_customization_textures`（行 84/89 closure 内联）无 PDB 对应，d_main 已删。

## 关键事实

- `bind_common` 是全部 raster draw pass（main_pass/transparency/shadow/wireframe/ui…）的公共绑定入口；矩阵 = `Transform::compute_matrix`（wp5 同述）。
- 结构体 `CommonDrawParams` 字段布局：U（new 绑定时间 uniform 为 O）。
