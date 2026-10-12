# G3 取证：country_core `draw_passes` 族

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 方法：`run-lanes --lanes 2`（database `ccdraw`）+ **PE 段表 VA→offset 读 rodata** 取 PASS 名字符串（`&str{ptr,len}`，字面量无 NUL 分隔，按长度取）。
- 产物：`job-g3a/b/c.json`、`report-g3a/b/c.json`、`decomp/0X*.c`（26 份）、`pass-inventory.json`。
- 注意：`g3c` 顶层 `ok:false` **仅因** `DrawLists::new(0x1409ba340)` 单条失败，其余 11 条成功。
- 仅取证，未修改任何代码。

## 1. 混合管线四者分工（观察到名字）

**统一惯用法**：全部 draw pass 走「录制 → render」：
`FUN_140867970`（登记 RasterPass，传 `&name, len`）→ 逐条 **0x80 字节**绘制记录：`FUN_140905be0`（建 `DrawBuilder`）+ `FUN_14090b460`（`bind_common`）+ 回调 `*(rec+0x70)` → `FUN_140868370`（`RasterPass::render`）。

| pass | 地址 | 登记的 RasterPass |
|---|---|---|
| `main_pass` | `0x140833bc0` | **6 个**，读 `param_5` 六组 Vec：`G-buffer`(+0x20)、`G-buffer emissive`(+0x38)、`G-buffer velocity`(+0x50)、`Forward`(+0x68)、`ForwardVelocity`(+0x80)、`Velocity`(+0x98) |
| `main_pass_oc2` | `0x140835a40` | **5 个**：`G-buffer OC/2`(+0xb0)、`G-buffer emissive OC/2`(+0xc8)、`G-buffer velocity OC/2`(+0xe0)、`Forward OC/2`(+0xf8)、`Velocity OC/2`(+0x110) |
| `late_forward_and_deferred_pass` | `0x1408373b0` | `Late Deferred`(+0x128/+0x130，非零才渲染) → `Late Forward`(+0x140/+0x148) |
| `deferred_raster` | `0x1409d20b0` | **单个** `Apply deferred lighting`；循环 **5 个 G-buffer 附件**（`lStack_98 + i*0x10` 存在位），串联约 11 次资源绑定后 `FUN_140906a90(..., 3)` 收尾 |

**分工（推断）**：`main_pass` 是首批——不透明体按通道分别写入 G-buffer（color/emissive/velocity）或直接 forward；`oc2` 是 **HZB 遮挡剔除后的第二批**（同名 + `OC/2`）；`deferred_raster` 消费 G-buffer 计算光照；`late_forward` 处理需 GI/后置光照的迟渲染。四者**同时存在**于 `render_world` 顺序中。

## 2. `shadow_pass`(`0x1409cc270`)

名字 `Shadows`。按 TypeId `0xa68561b396784eaa`（= `PassResources` 槽 `[1]`）取 `Res`；`do { … } while (count < 3)` → **3 级联**，级联槽位写 `uVar16 + 2`（即槽 2/3/4，**父级已核对**）；组为单 slice（`param_3: ptr + count`，0x80 步长），每级联 `bind_common` + 回调 + render。

**未知**：正交/透视投影矩阵未读。

## 3. transparency 家族 + TAA

PASS 名字：`Copy refraction target`(`0x1409bf320`)、`Water plane`(`0x1409bf800`)、`Ice plane`(`0x1409c08d0`)、`Pre-TAA transparency`(`0x1409c3a80`)、`Post-TAA transparency`(`0x1409c40b0`)。

- **`water_surface`**：先 RasterPass `Water plane`，再 **2 个 ComputePass**：`Water deferred rt args`(`/water/calc_deferred_rt_args.cs.hlsl`) 与 `Water deferred rt`(`/water/deferred_rt.cs.hlsl`)；用 `clear_rg_buffer`(`0x34`)、`create_raw_resource`。
- **`ice_surface`**：RasterPass `Ice plane` + **4 个 ComputePass**：`Ice refl march`(`/water/ice_refl_march.cs.hlsl`)、`Ice refl trace`(`/water/ice_deferred_rt.cs.hlsl`)、`Ice refr resolve`(`/water/ice_refr_resolve.cs.hlsl`)、`Ice refl resolve`(`/water/ice_refl_resolve.cs.hlsl`)。

**TAA 顺序（观察 `frame-composition`）**：`refraction_copy` → `water`|`ice` → **`pre_taa_transparency`** → … → `calculate_taa` → **`post_taa_transparency`**。
即 **Pre-TAA 透明度在 TAA 之前、Post-TAA 在之后**（post 组 `count == 0` 时早返回）。

## 4. UI

- `ui_pass`(`0x140837c40`) 与 `loading_screen`(`0x140837ff0`) **几乎逐字相同**，**同一 RasterPass 名 `UI`**（仅组偏移不同：`+0x1d0/+0x1d8` vs `+0x1b8/+0x1c0`）⇒ 走**同一渲染图**（都在 compile 闭包内登记，**非独立路径**）。
- `ui_2d_pass`(`0x1409c4470`) = `UI 2D`（形/字 SSBO 流，0x30 步长记录）。
- `mei_pass`(`0x1409c6700`) = `Mei 2D`，用**另一组 TypeId**（`0x8597ee0727b461b3` 槽 `[0]`、`0x864ddca98c2312a6`、`0x958c258b851a69dc`），经 `FUN_140937aa0` 查表 —— **不同资源路径，但仍在同一 RG**。

## 5. 接口与访问码（实测）

- **普通 draw pass 体内不写死访问码**：一律经 `DrawBuilder::bind_common`(`0x14090b460`) **隐式声明**。
- **显式访问码**出现在资源绑定器的**第 4 参**：

| 绑定器 | 访问码 | 出现处 |
|---|---|---|
| `FUN_1408ee800` / `FUN_140908c50` / `FUN_140908610` | **`0x1d`** | deferred_raster / water / ice / ui_2d / mei / pre_taa |
| `FUN_1408ee240` / `FUN_1409056d0` / `FUN_140905260` | **`0x19`** | water / ice（ice 内 29 次） |
| `FUN_1408edd00` | **`0x32`** | water / ice |
| — | **`0x1e`** | ice 内 1 次 |

**父级已核对**：`decomp/0X1409BF800.c:262` 等确为 `FUN_1408ee800(…, 0x1d, …)`；`:361` 等确为 `FUN_1408ee240(…, 0x19, …)`。

**已积累的访问码全集**：`0x1a`(compute 读)、`0x24`(draw 读)、`0x37`(写)、`0x02`(间接)、`0x34`(clear)、**新增 `0x1d`/`0x19`/`0x32`/`0x1e`**。

> **对 GAPS C3 的意义**：`0x19`/`0x1d`/`0x1e` 现在**在真实调用点被实测到**（不再只是合并表常量）。注意它们**都小于 `0x28`**，即**不在** A3 的并发分类区间 `[0x28,0x37]` 内；而新观测的 **`0x32` 在区间内**。命名仍缺。

- **绘制记录结构（0x80）**：`+0x50`/`+0x58` 资源、`+0x64` panic 标记、`+0x65` 标志位、`+0x70` shader 回调。

## 6. `draw_stream::RasterPass::render`(`0x140868370`)

**它不是简单提交**：入口 `param_1[0x3e][4] = 1`；遍历录制命令（`+0x20` 起、数量 `[0x3e]`、**步长 0x60**），打包 **0x10 字节**记录（上限 **0x80 条**），调 `FUN_141524830` 冲刷，并用**开放寻址哈希表**（`& uVar7` 掩码、写 `[uVar12]`、计数 `puVar13[2]/[3]`）做**去重/合并**，再走 `FUN_1409dcd10` / `FUN_140bfc0d0` / `FUN_140bd6b00`。
⇒ **是对录制流做批次归并后转换为 GPU 绘制。** 字段命名未知。

## 7. 其他

- `final_post`(`0x14090d000`)：建 RG 资源 `msaa color out` / `Final blit`，调专用助手 `FUN_1409ce780`，shader `raster_resolve_msaa_to_swapchain`（**MSAA 解析到 swapchain**，**非** RasterPass 构造器路径）；`resolve_to_swapchain`(`0x14090d850`) 是小函数。
- `wireframe`=`Wireframe`、`logo_overlay`=`Logo overlay`、`feedback_report_bg`=`Feedback report bg`、`custom_cursor`=`Custom cursor`、`foliage_exclusion_mask`=`Foliage exclusion`×2、`object_grass_push`=`Grass object push pass` —— 均标准惯用法。
- **`draw_lists::build_draw_lists`(`0x1409ba540`, 181 KB) 纯 CPU**（零 RasterPass/ComputePass 调用）—— 是 GPU 侧 `gpu_generate_draw_lists` 的 **CPU 对应物**。
- `render_shader_refactoring`(`0x14098f9f0`) 本体几乎仅 unwrap/自调用（**调试工具，非逐帧 pass**）。

## 8. 观察 / 推断 / 未知

**观察**：全部 PASS 名字符串、组偏移、接口地址、访问码常量、级联数(3)、G-buffer 附件数(5)、记录步长。

**推断**：四者分工与 `OC/2` = 遮挡剔除第二批、`late_*` = 需光照的迟渲染（**未验证调用者语义**）。

**未知**：
1. 7 个 `Res` TypeId 的类型名；
2. 访问码 `0x19`/`0x32`/`0x1e`/`0x1d` 的**命名**；
3. draw pass 内 `bind_common` 的**隐式访问码值**；
4. `DrawLists::new`(`0x1409ba340`) 未能反编译；
5. `shadow_pass` 的投影矩阵。

**PDB 签名恒 `void()`；未宣称构图闭包已闭合** —— 仅报告实际读到的约 26 个函数体。
