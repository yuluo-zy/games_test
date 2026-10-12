# G7 取证：后处理 / TAA-TGSR / DoF / UI

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 方法：`run-lanes --lanes 2`（database `ccpost`）。
- 产物：`job-g7-passes.json`、`REPORT-g7.json`、`decomp/0x*.c` **37 份**（含 `0x140161CD0` compile 闭包、`0x1404EC110` render_world）。
- l1/l2 合并 80 条，逐操作 **78 成功**；唯一失败 = `0x140968bb0` 非函数体（thunk）。
- 仅取证，未改任何游戏代码。

## 0. 父级复核

- **TAA/TGSR 分支**：`decomp/0x1404EC110.c:318` = `cVar1 = *(char *)(param_6 + 0xab)` —— **确认**配置字节位置。
- **访问码逐调用点传入**（**本轮重要结构发现**）：`grep FUN_1408edd00` 显示多数调用点传 **`0x32`**，但 `decomp` 中 `:281` 传的是 **`0x37`** —— **确认**。⇒ **访问码不是绑定函数的固有属性，而是每次调用显式传入的参数。**
- **UI 门控**：`decomp/0x140161CD0.c:1279` = `if ((bVar27 & iVar3 != 0x13) == 0)` —— **确认**。

## 1. TAA vs TGSR 分支（观察）

`render_world` 读配置结构 `param_6`：`cStack_4a = param_6[0xab]`（L318/347）。

| 条件 | 走 |
|---|---|
| `!= 0`（L464） | `calculate_tgsr`(`0x140A3BA60`) |
| `== 0`（L509） | `calculate_taa`(`0x140A3B2F0`) |

**两者互斥，非串行。** `param_6[0x9c]`（float）**同时**传给两者。

符号含 `UpscalingPreset::tgsr_resolution_scale`、`TgsrMain` ⇒ **推断**为时域超分 / 上采样；**缩写的确切含义 unknown**。

## 2. 跨帧历史（观察）

- **`calculate_tgsr`** 内部调 `get_or_create_temporal_pair`(`0x14096D490`，即 N4 的 parity 双缓冲，名长 8)。
- **`calculate_taa` 内未见 `get_or_create_temporal*`**；其历史由 `render_world` 传入纹理按 key 读写：
  读 `0xb8a74ccd835a6be3`（码 `0x19`），写 `0xaa75015a6ad61913` / `0x88219a3ae69892b2`（`0x32`）。
- **TAA 的 parity 管理位置 unknown。**

## 3. pre/post transparency（观察）

顺序：`refraction_copy`(`0x1409BF320`) → `water`(`0x1409BF800`) | `ice`(`0x1409C08D0`)（由 `cStack_49 = func_140ae8320(res+0x58)` 选，非 0 = ice；**整体由全局字节 `*14362C098 == 0` 门控**）→ `pre_taa`(`0x1409C3A80`) → … → taa/tgsr → `post_taa`(`0x1409C40B0`) → `refactoring`。

pre/post 均为 **DrawPass**，遍历 draw-item（pre：`param_8+0x188`，计数 `@+400`；post：`param_4+0x1a0`，计数 `@+0x1a8`，**计数 0 则 return**）。

**推断**：pre 参与 TAA 时域抗锯齿；post 在 resolve 后合成、不入历史。

## 4. DoF（观察）

- `dof_autofocus`(`0x140A17880`，**无条件**) 读模式资源 `0xc2b8e941c91a6428` 首 int `iVar3`，`bVar14 = (iVar3 == 0x13)`；
- `dof_prepare`(`0x140A5A020`) 出 **2 张 COC 纹理**；
- `calc_dof`(`0x140A5A570`) 由全局 `*14362C080` 分两路，写 `0xb981790202418bc8`（`0x32`）。

`iVar3 == 0x13(19)` 是**资源内模式枚举**，控制 DoF 变体及 `param_6[0xa9]`/`[0xaa]` 两开关（gate `generate_mips` + `calc_dof`）；**语义 unknown**。

## 5. 其余 pass（观察）

| pass | 结论 |
|---|---|
| `apply_volumetrics`(`0x1408A44C0`) | 读大气资源 `0xbba451ee6a08a3ff`，按 `+0x4d0` 字节(∈{4,1})选常量，把体积光并入场景色 |
| `compute_shadow_mask_mean`(`0x140A3A6F0`) | 单 pass 求阴影遮罩均值 |
| `bidirectional_blur_pyramid`(`0x1409ED860`) | 按 mip 级循环（down + up），读 `0x11ba5697cc40b3dd`，写 `0xdcdfcf289eb02e68` / `0x88219a3ae69892b2`（`0x37`） |
| `half_res`(`0x1408F6180`) | = `extract_half_res_depth_normals`（半分辨率深度 + 法线，2 输出） |
| `generate_mips`(`0x1400FB890`) | 通用 mip 链（限 `mips < 14`、格式 `0x7a`），写 `0xb466b30ac6fc0292` / `0xed9d1f0a5683f687` / `0x45f26f91023c2ec3`（`0x37`）+ 缓冲 `0x68d4413b4de4f24` |
| `reprojection`(`0x140A0D190`) | 两 pass，由深度 + 相机算重投影 / motion 图，写 `0x88219a3ae69892b2`（`0x32`） |

## 6. UI 路径（观察）

`ui_pass`(`0x140837C40`) / `loading_screen`(`0x140837FF0`) / `wireframe`(`0x1409CCB20`) / `post_taa` **共用同一 DrawPass 模板**（同读世界资源 `0x4bc362195b80f729`、同批绘制 `FUN_140905BE0`/`FUN_14090B460`、同 `(**(code**)(lVar6+0x70))` 间接绘制；**`ui_pass` 与 `loading_screen` 连 pass 名字面量 `&UNK_142A92BE0`（长 2）都相同**）。

⇒ 它们是**同一 RenderGraph 的节点**（compile 闭包 `0x140161CD0` 注册），**非独立路径**。（C1 的第二个 compile 实例 `0x1401647B0`(UI) 本次**未解剖**。）

- `mei_pass`(`0x1409C6700`) 属 `ui_2d_pass` 模块，做 2D 绘制（`FUN_14162C590`/`FUN_141629E90`），读 `PassResources` 槽 `[0]` = `0x8597ee0727b461b3`，绑图集 `0x815bf5036966978c`（`0x1d`），模式字节由 `func_14093fc00` 给。
- `custom_cursor`(`0x1409D1710`) 读槽 `[0]`、校 13 字节标记、绘制光标（`0x9f0d228ee3dba429`，`0x1d`）。

> **关键**：**整段 UI 注册被 `if ((vermeer 返 bool & iVar3 != 0x13) == 0)` 门控** —— **vermeer 仍在收敛时跳过 UI**。
> `mei` 的确切含义 unknown。

## 7. painterly / vermeer（观察）

- **`painterly_pass`(`0x1408A6410`)**：多 compute + 1 draw，用 `get_or_create_temporal`(`0x14096C890`/`0x14096C310`)，读 `0xd9978e541ce9a95e` 等、写 `0x88219a3ae69892b2`/`0x7021fc7ae6e0ac34`(`0x32`) —— **带时域状态的风格化滤镜，算法 unknown**。
- **`vermeer_pass`(`0x1408A82F0`，返 bool)**：用 `get_or_create_temporal`（名长 `0xe`），**维护帧计数并累加 float 至 ≥ 阈值(`_UNK_142925904`)**，然后置 `state = 2` 并推进环形缓冲，**返回 `state != 2`** —— **渐进 / 时域累积（是否收敛）**。命名暗示逐帧光照累积，**用途 unknown**。

## 8. 访问码实测（观察）

| 码 | 出处 |
|---|---|
| **`0x19`** | `FUN_1408EE240` / `FUN_14096F090` / `FUN_140905260` / `FUN_1409056D0`（资源绑定 / 读） |
| **`0x32`** | `FUN_1408EDD00`（TAA / volumetrics / painterly / half_res / reprojection） |
| **`0x37`** | `FUN_1408EDD00` / `FUN_140905260` / `FUN_1402AF750`（blur_pyramid / hzb / generate_mips） |
| **`0x1d`** | `FUN_1408EE800` / `FUN_140908610` / `FUN_1408FD6C0`（DrawPass 纹理绑定：pre_taa / refraction / cursor / mei / painterly / menu_bg / logo / session / lut_gallery / feedback） |

**⇒ 同一个 `FUN_1408EDD00` 在不同调用点取 `0x32` 或 `0x37` —— 码是逐调用传入，不是按函数固定。**（父级已复核。）

新增码 `0x19` / `0x1d` / `0x32`（对照已知 `0x1a`/`0x24`/`0x37`/`0x02`/`0x34`；`0x37` = 写，**与 R2 复核一致**）。

`get_xrefs_to` 对 `calculate_taa`/`tgsr`/`calc_dof` 各仅 1 条 DATA 引用（`0x143696E34`/`0x143696EB8`/`0x1436986C4`）、**无 CALL 边**，印证三者经**已编译图间接派发**（**未据 xref 断言调用关系**）。

## 9. 未知

- `param_6` 的字段名；`iVar3` 枚举（`0x13` 语义）；
- 各 pass 的**名字符串内容**（`0x142ac____` 区不被 `get_strings` 覆盖）；
- `PassResources` 7 个 TypeId；
- **`0x32` 与 `0x37` 的语义区别**；
- `0x1401647B0`（第二个 compile 实例）的角色。

**未宣称构图闭包已闭合。**
