# render/frame_constants.rs

> 文件作用：**帧常量 UBO 载荷**——`FrameConstants`（主视图 + 太阳级联视图，逐帧写入 `frame_constants` UBO
> set1/b0）+ `RenderViewIndex`（u32 视图槽；`sun_cascade`/`is_shadow_view` 谓词）+ `TREE_SEE_THROUGH` fmt 常量。
> 证据：`reconstruction/country-core-render/wp4/contracts/view_frame_light_atmos_texture.md`（O）。

函数总计 4：业务 1 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `FrameConstants::should_reset_temporal` | 0x140976880 | 105 | 猜测：谓词；证实：返回时间性重置标志 | **O**（11 B，本批字节复核）：`cmp dword [rcx+0x80], 0; sete al; ret`——重标志在 +0x80（wp4 记为"空体寄存器透传"，字节证实为 +0x80==0 布尔） | hex 手解 0x140976880（rne-read --hex 本批；原 hex 手解 0x140976880（rne-read --hex 本批；原 wp4 + 本批 rne-read 0x140976880）） |

## 内联/无独立符号函数

- `RenderViewIndex::is_shadow_view`（行 73）— **I**：u32 视图槽谓词（I from 名称；PDB 成员 `main_view()`/`sun_view()`/`sun_cascade()` 相关）
- `RenderViewIndex::sun_cascade`（行 66）— **I**：返回级联 id（PDB 名 O）

## 派生胶水（2 项：default、eq 等）

- FrameConstants/RenderViewIndex 的 derive（Default/Eq），不补。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `FrameConstants::should_reset_temporal`（0x140976880）
11B：`cmp dword [rcx+0x80], 0; sete al; ret` —— 时间性重置标志在 **+0x80**（wp4 记为"空体寄存器透传"，字节证实为 `+0x80==0 → bool`）。
- 消费链：`frame_constants` UBO（set1/b0）逐帧写入 → `calculate_taa/calculate_tgsr`、`apply_screen_shadows` 等的 `history_valid` uniform 决定是否复用历史纹理（历史对 `fresh` 标志语义，render_loop 章）。
- `RenderViewIndex`（u32 视图槽）：`sun_cascade(c)=c+2` 式级联 id、`is_shadow_view` 谓词（I，PDB 成员 main_view()/sun_view()/sun_cascade() 相关）。
- `TREE_SEE_THROUGH` = Display 串 fmt 常量（O/I）。

## 纠错/差异

- 本批字节复核纠正：`should_reset_temporal` 非"寄存器透传空体"，实为 `[+0x80]==0 → bool`（11 B）。
- U：`FrameConstants` 全 UBO 字段列表（用 shader 反射/catalog 惯例；`main_view`/`sun_view` 成员名 O）；`TREE_SEE_THROUGH` fmt 常量 = Display 串（O/I，render-gap 未涉及）。
- 消费者：渲染循环逐帧写 `frame_constants` UBO（set1/b0，render-passes/INDEX.md）。
