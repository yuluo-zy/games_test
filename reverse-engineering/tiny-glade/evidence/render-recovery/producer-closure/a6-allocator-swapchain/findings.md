# A6 取证：allocator 分叉 + swapchain/winit 呈现

- 目标：原发行二进制 `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra（`run_auto_analysis:false`）+ `run-job-queued.mjs`。产物：`job.json`、`REPORT.json`（58 步）、`job-probe.json`、`REPORT-probe.json`、34 份 `0x*.c`。
- 仅取证，未修改任何候选源码 / Cargo / evidence JSON。

## 工具边界（重要）

本轮 Ghidra 库**无 Rust 符号、无用户字符串**（`list_names` / `get_strings` 全 0，只有 27 个 PE 导入名）。函数名靠 `public-symbols.json` 的 VA→PDB 映射得到。`run_auto_analysis:false`，全部目标只返回 **1 条 DATA xref**，无 code caller —— **空 caller 不能证明无调用者**。伪代码 ≠ 作者原 Rust。

## 1. 分配器分叉（观察）

**游戏侧包装 `GpuAllocator` 布局**（与既有 `allocator-functional-validation.json` 一致）：

| 偏移 | 含义 |
|---|---|
| `+0x620` | `peak_report`（`Option`；`new` 置 `i64::MIN` = None） |
| `+0x670` | `peak_device_local` |
| `+0x678` | `peak_system` |
| `+0x680` | `peak_total` |
| `+0x688` | `track_peaks` |

- `new` (`0x140c0d980`)：调 `FUN_140c120d0` 建内部分配器，`memcpy 0x5f8` 拷入本体，peaks 归零，写入 `track_peaks`。
- `allocate` (`0x140c0da40`)：先调 `FUN_140c12dc0`（**分叉库真正的 allocate**）；若 `track == 1`，`FUN_140c15150` 取报表快照，device/system 峰值各自取最大写回 `+0x670`/`+0x678`；**仅当 total 严格增**时更新 `+0x680` 并把整份快照存 `+0x620`（先 drop 旧报表）；另有全局日志级别（`0x14362c3a8`）门控的可选 trace。
- `free` (`0x140c0e150`)：先做泄漏追踪记账，再调 `FUN_140c14680`（**分叉库真正的 free**），**不碰 peaks**。

**分叉库本体** = `FUN_140c120d0` / `140c12dc0` / `140c14680` / `140c15150`（内存类型选择、块/专用分配、失败回退都在此）。**属公开依赖边界，按项目规则未反编译。**

getter `0x140c0e600`/`610`/`630`/`640` 过短无法反编译；偏移由上述交叉证实。

四个同名 `with_global_allocator`（`0x1405b9810`/`9a30`/`140faeae0`/`1416a3620`）行为像"在全局分配器锁（ptr `device+0x678`，锁 `+0x10`）下读峰/格式化"，**与"构造"名不符**，标为**推断**。

## 2. 窗口绑定（观察，强证据；父级已核对 `0x1400315e0.c`）

`raw::surface::Surface::create` (`0x1400315e0`) 反编译父级确认：

```c
func_0x000140698320(&iStack_80, *(longlong *)*param_3 + 0x10);   // display_handle
if (iStack_80 == 0xc) { ... }
FUN_1406982e0(&iStack_80, *(longlong *)*param_4 + 0x10);         // window_handle
if (iStack_80 != 0xf) { ... }                                     // 否则报错
FUN_140c0f820(abStack_48, param_2, param_2 + 0x30, &iStack_38, &iStack_68, 0);  // 建 surface
```

→ 原作走 **`raw-window-handle` 的 `HasDisplayHandle` / `HasWindowHandle`（winit `Window`）**，即**通过 trait 对象间接调用**取原始句柄，**不是直接 HWND**。判别值 `0xc` / `0xf`。

配套符号：`Arc<H> HasWindowHandle/DisplayHandle`（`0x1402e60a0`/`c0`）；`tiny_glade::event_loop::run_event_loop` (`0x14026e4a0`)。

依赖事实：`winit 0.29.15` PounceLight fork（commit `d03de0a0…`，**未核实**）、`raw-window-handle 0.6.1`、`ash-window 0.13.0`。

**推断**：判别值 `0xc`/`0xf` 精确对应 `Win32`/`Windows` 枚举。

## 3. 三层呈现结构（观察）

1. **`raw::Swapchain`** 的 `acquire_next_image` (`0x140c0eb40`) / `present` (`0x140c0ef80`) 是**薄包装**：先校验世代（`chain+0x14 == surface+0xa50`）、状态（`+0x16 == 2`）、在管数量，再委托 managed。
2. **`managed::ManagedSwapchain::acquire_next`** (`0x1418cb520`)：锁后要求状态 `== 2`；游标 `+0xb0`；逐槽结构 `+0x60`（**stride 0x38**）；调设备虚表 offset **`0x6c8`** 的 `vkAcquireNextImage`；**超时硬编码 `3_000_000_000` ns（3 秒）**。
   返回码处理：
   - `0`(VK_SUCCESS) 或 `1000001003`(VK_SUBOPTIMAL_KHR) → **Ok**，游标 = `(idx+1) % image_count`；
   - `1`/`2`(NOT_READY/TIMEOUT)、`0xC4653214`(OUT_OF_DATE = -1000001004) → **Err**；
   - 其他 → 格式化 Err。
   **即 SUBOPTIMAL 当成功交付图像，不标 stale。**
3. **`managed::ManagedSwapchain::present`** (`0x1418cc1b0`)：锁交换链 + 设备队列锁 `device+0x718`；默认队列 `device+0x730`，或辅助队列 `param_6*0xb0 + *(device+0x7e8)`（边界 `device+0x7f0`）；经 `Queue::with_raw` (`0x140c03d70`) 提交。
   返回码：`SUBOPTIMAL`(`0x3b9acdeb`) → 置 bit0；`SURFACE_LOST`(-1000001000) / `OUT_OF_DATE`(-1000001004) → **非致命**；其他 → 设备崩溃 `FUN_140befc70`/`FUN_140befd60` + panic。

## 4. 重建闭环（观察）

- `GraphicsDevice::create_swapchain` (`0x140bfd720`)：**先 `retire`(`FUN_1418ccb50`) 再 `new`**；链存设备侧 slab（`0xd0` 槽，free-list @ `device+0x468`，`FUN_1418bbd30` 哈希索引）。
- `validate_swapchain_and_acquire_image` (`0x140bfd9d0`)：**在 OUT_OF_DATE（态 7）时** `create_swapchain` → drop 旧 → 再 acquire。**这是完整的重建闭环。**
- `resize` (`0x140c0e7f0`)：只登记新 desc @ `managed+0x48`，**不 `device_wait_idle`**。
- `set_vsync` (`0x140c0e8a0`)：写 vsync @ `managed+0x50`。
- `on_focus` (`0x140c0e6b0`)：写焦点 @ `managed+0xc9`。
- `mark_stale` (`0x140c0e950`) 与 `on_focus` 都经**全局 hashbrown 注册表**（乘子 `0x9E3779B97F4A7C15`）排空待释放项。
- frame 在途 = **逐槽复用**；精确槽数与成员未读全。

## 5. 观察 / 推断 / 未知

**观察**：第 1 节包装布局与 track_peaks 门控、第 2 节 trait 对象调用与判别值、第 3 节三层结构与返回码分支、第 4 节重建闭环与副作用。

**推断**：判别值 `0xc`/`0xf` = `Win32`/`Windows`；四个 `with_global_allocator` 的真实语义（名不符实）。

**未知**：
- present mode / 状态码的精确枚举未逐条证明；
- 槽结构 0x38 字节字段语义未读全；
- 分叉分配器策略未进入（**边界**，不逆向）；
- winit commit 未核实；
- xref 完整性未知（仅 DATA）。

## 6. 候选实现差异（最大缺口）

- **`src/raw/managed/swapchain.rs` 仅 13 行 stub**（父级已核：只有类型别名 + `manage_swapchain` + `raw()`），`new` / `acquire_next` / `present` / `retire` / `dispose` / `desc` / `resize` / `set_vsync` **全部缺失**。
- **同步粒度不同**：候选每帧新建 2 个 semaphore + fence；原作**逐槽复用 + 队列锁**。
- **OUT_OF_DATE / SUBOPTIMAL 分支语义不同**，且**没有 validate 重建闭环**。
- 无辅助队列 / 崩溃处理。
- `surface` 只有 `from_win32`，**winit trait 绑定未恢复**；**`surface.rs` 不存在**。
- 分配器：包装语义吻合；候选 `DirectMemoryBackend` 每分配一次专用 `VkDeviceMemory`，**未复刻池**（合规，底层属依赖边界）。缺口 = 底层不复刻。
