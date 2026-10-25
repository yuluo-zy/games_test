# apply_vulkan_blocklist（FUN_140c0a530）env 链完整枚举（审查轮闭合证据 P0-1）

- 日期：2026-10-15。标注：O=机器码/PDB 实测；I=推断；U=未知。
- 方法：rne-read 0x142b19400..0x142b196f8 字符串区逐字节；反编译 decomp-0x140c0a530.txt 全量核对；
  raw .text 字节扫描 E8 call 目标计数；MCP 反编译 FUN_140d6c690。

## 1. 关键发现：FUN_140d6c690 = std::env::set_var 包装（非读取检查）

MCP 反编译 0x140d6c690（O）：
```
FUN_140d6c690(param_1,param_2,param_3,param_4):
  FUN_140d75000(&lStack_58, param_1, param_2);      # &str→UTF-16 wide（名字）
  FUN_140d75000(&lStack_58, param_3, param_4);      # &str→UTF-16 wide（值）
  iVar2 = SetEnvironmentVariableW(uStack_40, uStack_50);
  if (iVar2 == 0) { uVar4 = GetLastError()<<0x20 | 2; } else { uVar4 = 0; }
  return uVar4;   # Result<(),OsError>：0=Ok
```
即 `std::env::set_var(name, value)` 的本地包装。apply_vulkan_blocklist 对它的 18 次调用（raw 扫描确认
0x140c0a634/0x140c0a7ce/0x140c0aadf/…/0x140c0bdf6 全部 E8→0x140d6c690）都是**写环境变量**，
不是读/查。旧 ALGORITHMS.md §3.1"值含 1 触发"（读语义）作废。

## 2. env 变量完整表（O，地址/长度来自 decomp 中 `pbStack_70=&UNK…; lStack_68=0xNN` 且与字符串池切片完全连续）

| # | env 变量名 | 地址 | 长度 | 游戏写入值 | 语义 |
|---|---|---|---|---|---|
| 1 | VK_LOADER_LAYERS_DISABLE | 0x142b19428 | 24 | `~implicit~`@0x142b19440(10) | 令 loader 禁用全部 implicit 层 |
| 2 | VK_LOADER_LAYERS_ALLOW | 0x142b19508 | 22 | 过滤后层名 join(",") 列表（栈上构建） | 选择性重启用清单 |
| 3 | DISABLE_RTSS_LAYER | 0x142b1951e | 18 | `1` | 强制禁用 RTSS 覆盖层 |
| 4 | DISABLE_VKBASALT | 0x142b19530 | 16 | `1` | vkBasalt |
| 5 | VK_LAYER_bandicam_helper_DEBUG_1 | 0x142b19540 | 32 | `1` | Bandicam |
| 6 | NODEVICE_SELECT | 0x142b19560 | 15 | `1` | NVIDIA device-select 层 |
| 7 | DISABLE_GAMEPP_LAYER | 0x142b1956f | 20 | `1` | GamePP |
| 8 | DISABLE_SAMPLE_LAYER | 0x142b19583 | 20 | `1` | sample 层 |
| 9 | DISABLE_LAYER_NV_OPTIMUS_1 | 0x142b19597 | 26 | `1` | NVIDIA Optimus |
| 10 | DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1 | 0x142b195b1 | 39 | `1` | AMD 可切换显卡 |
| 11 | **EOS_OVERLAY_DISABLE_VULKAN_WIN64** | 0x142b195d8 | **32** | `1` | EOS overlay（Vulkan/Win64）——**单一 32B 名**，非"EOS_OVERLAY_DISABLE"+"VULKAN_WIN64"两个变量 |
| 12 | DISABLE_VULKAN_OW_OBS_CAPTURE | 0x142b195f8 | 29 | `1` | Overwolf OBS 捕获层 |
| 13 | DISABLE_VULKAN_OW_OVERLAY_LAYER | 0x142b19615 | 31 | `1` | Overwolf overlay |
| 14 | DISABLE_VK_LAYER_reshade_1 | 0x142b19634 | 26 | `1` | ReShade |
| 15 | DISABLE_TWITCH_VULKAN_OVERLAY | 0x142b1964e | 29 | `1` | Twitch |
| 16 | DISABLE_MANGOHUD | 0x142b1966b | 16 | `1` | MangoHud |
| 17 | DISABLE_LAYER | 0x142b1967b | 13 | `1` | 通用禁用层 |
| 18 | DISABLE_VK_LAYER_gshade_1 | 0x142b19688 | 25 | `1` | gshade |

合计 **18 个 env 变量**（REVIEW 预估 ≥19；EOS 与 VULKAN_WIN64 实为一个 32B 名）。名字池 0x142b19508 起切片完全连续（0x142b195d8+32=0x142b195f8，…，0x142b19688+25=0x142b196a1）。

## 3. 函数流程（O）

```
puffin span "rhapsody::raw::instance"（0x142b191b0, len 23）
1. set_var("VK_LOADER_LAYERS_DISABLE", "~implicit~")
     .expect("failed to set environment variable `…`: {e}")   # 失败→panic（消息@0x142d0a710, Location@0x142d0a770=std/src/env.rs）
2. 若成功：遍历入参层列表（0x20B 元素），筛 bit0=1 且匹配 "VK_LAYER_*"（0x142b19468, len 9）的层，
   收集 0x18B 条目，FUN_1418cd6c0 以 ","（0x142b1944a）join → allow_list
   puffin span "Selectively enabling implicit Vulkan layers: {allow_list}"（0x142b194b0, len 45）
3. set_var("VK_LOADER_LAYERS_ALLOW", allow_list).expect(…)   # 选择性重启用
4. 若成功：16 个 set_var(DISABLE_*_LAYER, "1") 顺序执行（各 .expect()，失败 panic）
   每个前有 puffin span "[legacy] Force-disabling a Vulkan layer via {env}"（0x142b196a1, len 44）
5. 全成功→return；任何 set_var 失败→ panic（FUN_1428d9390，20 次调用点），函数以 abort 结束
```

## 4. 触发值语义（O）

- 16 个 DISABLE_*_LAYER：写入值固定为 `1`（0x142b196f8，len 1）。即游戏**无条件**（正常路径下）把 16 个已知
  覆盖/辅助层对应的环境变量归一化为 "1"，令各覆盖层自禁。
- VK_LOADER_LAYERS_DISABLE：写入 `~implicit~`，先全局禁用所有 implicit 层。
- VK_LOADER_LAYERS_ALLOW：写入游戏过滤后保留的层名列表（逗号连接），"选择性启用"。
- 无任何"读 env 判断"调用（raw 扫描无 GetEnvironmentVariableW/var_os；函数内仅 set_var 一族 env 调用）。

## 5. 相关字符串（O）

| 地址 | 内容 |
|---|---|
| 0x142b19428 | "VK_LOADER_LAYERS_DISABLE"（24B）|
| 0x142b19440 | "~implicit~"（10B）|
| 0x142b1944a | ","（1B）|
| 0x142b19468 | "VK_LAYER_*"（9B，过滤通配）|
| 0x142b19471 | "*"（1B）|
| 0x142b194b0 | "Selectively enabling implicit Vulkan layers: "（45B）|
| 0x142b19508 | "VK_LOADER_LAYERS_ALLOW"（22B）|
| 0x142b196a1 | "[legacy] Force-disabling a Vulkan layer via "（44B）|
| 0x142b191b0 | "rhapsody::raw::instance"（23B）|
| 0x142b19078 | "crates/rhapsody/src/raw/instance.rs"（35B，Location）|
| 0x142d0a710 | "failed to set environment variable `"（std panic 前缀）|
| 0x142d0a770 | "/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs"（set_var panic Location）|

## 6. rebuild 对照结论

rebuild `apply_vulkan_blocklist(layers, blocked)` 通用过滤 + modern 传空表 → **确认偏差（原 doc ✗ 判定成立且加深）**：
原函数是 18 次 set_var 的强制屏蔽/选择性启用链，不是读 env 的过滤。候选对照实现应改为：
先 `set_var("VK_LOADER_LAYERS_DISABLE","~implicit~")`，过滤/join 后 `set_var("VK_LOADER_LAYERS_ALLOW", join)`，
再对 16 个 DISABLE_* 各 `set_var(name,"1")`，均带 expect panic。
