# startup/startup_blit.rs

> 文件作用：**blit 阶段 shader 载入（交换链 resolve/srgb decode）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_blit::startup_blit` | 0x140901430 | 11 | `startup_blit`：载入 3 个 blit shader — resolve_to_swapchain, resolve_msaa_to_swapchain, blit_stretch_decode_srgb（`/blit/*.raster.hlsl`）。 | **O** | decomp-0x140901430.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。
