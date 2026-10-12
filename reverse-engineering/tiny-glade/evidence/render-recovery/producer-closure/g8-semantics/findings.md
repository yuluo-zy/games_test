# G8 取证：C 类未闭合语义项攻关

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 方法：**PE 直读**（字节级解码）+ `run-lanes` + **PE 扫描 167 个 `add_resource_access` 调用点的 `mov r8b, imm`**。
- 产物：`evidence/render-recovery/producer-closure/g8-semantics/`（5 组 job/REPORT、`decomp/*.c` 25 个、`g8-c5c6-decode.txt`、`semantics.json`、`pe.py`）。
- 仅取证，未触碰任何游戏代码。

## 0. 父级复核

- `semantics.json` 存在，含 7 条缺口条目（C1–C6、C10）。
- **C1 的写者**：父级独立核对 `n1-device/0x140bf0b90.c:19` = `iVar5 = (**(code **)(param_2 + 0x478))(*(undefined8 *)(param_2 + 0x5c8), param_1[0x11], &iStack_20);` —— 用 ash 槽 `+0x478`（`vkGetSemaphoreCounterValue`）读 `param_1[0x11]`（= `queue+0x88`，时间线信号量）。**结构成立。**

## 1. C5 —— **攻克**（PE 直读）

- **`BatchCompiler::new`(`0x140be95e0`, 30 B)** 字节：
  `48 89 C8` / `48 C7 01 00000000` / `48 C7 41 08 08000000` / `0F 57 C0` / `0F 11 41 10` / `0F 11 41 19` / `C3`
  ⇒ 把 `{Vec cap@+0x00, ptr@+0x08(= 8，悬垂对齐), len@+0x10}` 与 `cur_batch{start@+0x18, end@+0x20, flag@+0x28}` **全部清零**。
  `flush_batch`(`0x140be9600`) **独立印证**批次 = **24 B `{start, end, payload}`**（与 SYNTHESIS 11.5 一致）。
- **`pass_access_range_indices`(`0x140be9690`, 5 B)** = `89 C8 89 D2 C3` = `mov eax,ecx; mov edx,edx; ret` ⇒ **(start, end) u32 恒等透传**。
  调用者 `add_pre_pass_barriers`(`0x1402c3d00`) 用它读 **stride-8** 表 `{start@+0, end@+4}` 的 start，再做 `end < start` 越界 panic。
  （**观察 = 字节；字段名 = 推断。**）

> 与 G1 的独立结果**一致**。

## 2. C6 —— **攻克**（PE 直读）

| 函数 | 字节/结论 |
|---|---|
| `device_uuid`(`0x140bf84c0`) | 拷贝 `P+0x690` 的 **16 字节**（`P = *(device+0x658)`）→ `deviceUUID` |
| `physical_device`(`0x140bf8950`) | 返回 `P+0x10` |
| `debug_utils`(`0x140bf8960`) | `Option<&DebugUtils>::as_ref`，字段在 `device+0x870` |
| `refresh_memory_budget`(`0x140bf8f80`) | `80 79 1A 00 / 74 10` —— 若 `device+0x1a` 标志 ≠ 0，尾跳到 `0x140c01620` = `PhysicalDevice::refresh_memory_budget(P+0x10)` |
| `min_subgroup_size`(`0x140bf8fa0`) | `8B 41 40 / 8B 51 44 / C3` → 返回 `(*(this+0x40), *(this+0x44))` |

## 3. C1 —— **攻克**：`queue+0x80` 的写者找到了

**写者 = `Queue::update_pending_submissions`(`0x140bf0b90`)**：

- 第 19 行调设备 ash 槽 `+0x478`，参数 `(device+0x5c8, queue+0x88, &out)`（= **`vkGetSemaphoreCounterValue`**，时间线信号量在 `queue+0x88`）；
- **第 42 行把返回的 u64 写入 `param_1[0x10]`（= `queue+0x80`）**。

⇒ `reclaim` / `queue_submit` 只读它 —— 与 GAPS 的登记吻合。**写点 = 观察；槽名 = 推断。**

## 4. C3 —— **部分攻克**（合并格完整恢复 + 生产者归属）

**合并格完整恢复**（`0x1409dca90` 与 `0x140533b10` **逻辑相同**）：

| 组合 | 结果 |
|---|---|
| `0x17` + `0x1d`（任序） | `0x1e` |
| `0x1a` + `0x02` | `0x02` |
| `{0x19, 0x1a}` + `0x37` | `0x37` |
| 其它非同组合 | **一律 panic** |

⇒ **`0x1e` 是"读 | 写"的合并态。**

**生产者归属**（PE 扫描 **167 个** `add_resource_access` 调用点的 `mov r8b, imm`）：

| 码 | 生产者 |
|---|---|
| `0x02` | 间接（`multi_draw_indirect` / `draw_indexed_indirect` / `dispatch_indirect`） |
| `0x1a` | `ComputePass::read_rg_storage_buffer` |
| `0x24` | `DrawBuilder::read_rg_storage_buffer` |
| `0x34` | 写 / 清（`reset_indirect_args`、`clear_rg_buffer`、`rg_buffer_copy_from_slice`） |
| `0x37` | `write_rg_storage_buffer` |
| `0x17` | `RasterPass::new_impl` |

**访问器（accessor）调用点的参数码**：**读 = `0x1a`/`0x1f`、写 = `0x32`、并发写 = `0x37`**。

> ### 父级更正（我此前对 G6 的判断需修正）
> 我在 G6 的 findings 里写"G6 的 0x32 **无证据支撑**、不予采信"。**该判断只对了一半**：
> - G6 的**证据**确实不在其保留文件里（我核查属实）—— 这点不变；
> - 但 G6 的**实质结论（写 = 0x32）由 G8 的独立 PE 扫描 167 个调用点佐证成立**；
> - G6 真正错的是**框架** —— 它把这个当成"对 C1 的更正"。实际是**两套不同 API**：builder 的 `write_rg_storage_buffer` = **`0x37`**，accessor 的 `write` = **`0x32`**。**不构成矛盾。**

**账本布局**经比较指令确认 = `{index@+0, sync_group@+4, access_code@+8}`（**印证 SYNTHESIS 11.2 的更正**）。

**未攻克**：`0x19` / `0x1d` 在合并表里被引用，但**扫描不到任何字面生产者**（应为**运行期计算值**），语义仍未知。

## 5. C2 —— **部分攻克**

- **桶** = **9 个句柄 `Vec` `{ptr, cap, len}`（步长 0x18）+ 第 10 个 0x48 字节记录 `Vec` `{ptr@[0x1c], len@[0x1d]}`**，元素 tag `@+0x44`（`== 2` 即停）。
- **9 个 `Vec` 各由独立的 device vtable 槽排出**：`+0x1a8`、`+0x1c0`、`+0x188`、`+0x138`、`+0x228`、`+0x248`、`+0x278`、`+0x288`，另一路走第二张表（`+0x6b8`，器件句柄 `+0x6f0`）。
- **已定名**：**`+0x278` = `vkDestroyFramebuffer`、`+0x288` = `vkDestroyRenderPass`**（用 create 槽 `+0x270`/`+0x280` 标定）。
- 7 个 store = 7 个 `dispose<T>` 单态（Texture / Buffer / Sampler / RenderPass / Framebuffer / Swapchain / PersistentDescriptorSet）。
- ⇒ **9 ≠ 7**：多出的 2 个排**非 store 资源**（`+0x6b8`/`+0x6f0` 那条疑为命令缓冲/池）。
- **`case 9`**：0x48 记录路径断言派发判别值 `== 9`；派发链 `FUN_140c0e150 → FUN_140c14680`，后者在 165 / 328–329 行写 `*out = 9`（当条目 dyn-trait 虚表调用 `(**(entry[1]+0x48))(...)` 返回 9，或 `record[0] == 0` 时）。
  ⇒ **它是 trait/vtable 派发判别值，不是 int32 跳表的字面 case。**

## 6. C4 —— 登记边界（未逆向依赖）

`+0x48` 的 u64 = **`{低 32 = access 掩码, 高 32 = stage 掩码}`**，初值 `{1, 0x2000}` = `0x0000200000000001`。
其余位在 `FUN_140c10af0` / `FUN_140c10940` 内按访问码 OR 进来；映射函数是**外部 `vk-sync-fork` 的 `FUN_140c10dd0(code) -> (access, stage)`**。**按约束只登记边界。**

## 7. C10 —— 部分攻克

- `device+0x658` = 指针 `P`。`P+0x690` = 16 B `deviceUUID`；`physical_device()` = `P+0x10`；`refresh_memory_budget` 传 `P+0x10`。
- `GraphicsDevice::create` 里**直接读** `*(u32)(PhysDev+0x680)` 并有 **`< 0x2000 → 报错`** 保护（与 `maxDescriptorSetUpdateAfterBindSampledImages` 吻合），并把 `PhysicalDevice` 存入 `device+0x658`（`puVar62[0xcb]`）。
- **关键结论**：该 limit 是包装体的**缓存字段**（create 时直接读，站点**不再调** `vkGetPhysicalDeviceProperties`），与 `+0x690` 的 UUID 是**同级字段**，**不经 `+0xd0` 的 VkPhysicalDevice 句柄**推导。
- **未完全钉死**：`physical_device()` = `P+0x10` 而 create 读 `*(param_2+0x680)`，故 **`+0x680` 的绝对基址仍有 `P` 与 `P+0x10` 的歧义**。

## 8. 仍未知

1. **`0x19` / `0x1d` 的名字**（扫描不到字面生产者，应为运行期计算值）；
2. 9 个 destroy 槽中**除 `+0x278`/`+0x288` 外的 `vkDestroy` 名**（受 ash 分叉表限制）；
3. **`+0x680` 的绝对基址**；
4. C4 各 bit 义（**属外部库**）。

**全程静态；未运行游戏。**
