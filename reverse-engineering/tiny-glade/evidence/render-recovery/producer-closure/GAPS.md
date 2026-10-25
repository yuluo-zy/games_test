# rhapsody 逆向缺口登记册

- 日期：2026-10-12（第三波进行中；G1/G2 已回）。
- 目标：`D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 机制结论见 [SYNTHESIS.md](SYNTHESIS.md)（第十一节为复核后当前结论）。本文件只登记**缺口**。
- 全部为**静态**取证，**零动态证据**（未运行游戏、未捕获帧）。

> **第三波更新（G1/G2 已回）**：**A 类已闭合**；**C2 已更正**；**C5/C6 已关闭**。见下文标记。

---

## 一、覆盖现状（可复核的量化口径）

### 1.1 rhapsody 域（workgraph 450 函数 / 342,968 字节）

| 指标 | 第一轮后 | 第二轮后 | **第三波（G1 后）** |
|---|---:|---:|---:|
| 有取证结论的函数 | 129 | 291 | **450 / 450** |
| 已定界字节 | 170,116（49.6%） | 296,877（86.6%） | **342,968（100%）** |

**⇒ rhapsody 的字节面已 100% 反汇编完成**（291 由 Ghidra 伪代码 + 159 由 PE 直读，见 `g1-pe-leaves/`）。

### 1.2 country_core 渲染面（**此前从未计入分母**）

| 指标 | 数值 |
|---|---:|
| 渲染相关符号 | **1,843**（1,829 带 VA） |
| 已定界字节 | **475,588** |
| `systems::render_loop` | 273 符号 / 86,840 B |
| 具名 pass 函数 | **195** |
| **已取证的 pass / 函数** | **140 个不同地址**（第三波 G3–G7 新增） |
| 顺序层面 | compile 闭包 36 步 + `render_world` 59 步**全部已知**（`c1-country-core/frame-composition.json`） |

**第三波覆盖的 pass 族**：`draw_passes`（main_pass / oc2 / late_forward_and_deferred / deferred_raster / shadow / transparency / UI 全族）、GPU 可见性与绘制列表（cull·bucket / prefix-sum / HZB / 两轮可见性）、GI 与光照（BVH 软件构建 / trace / radiance / eval / restir_di / bin_lights / screen shadows / terrain lighting）、场景生成（upload_raster×6 / 地形链 / 建筑 / 植被 / roof tiles gravity / stairs / leaf particles）、后处理（TAA·TGSR / DoF / blur pyramid / half-res / mips / reprojection / volumetrics / painterly / vermeer / UI）。

索引：`c1-country-core/country_core_render_symbols.json`（VA→名，1.0 MB，53 个模块分组）。

> **country_core 渲染面（475,588 B）仍大于整个 rhapsody 域（342,968 B）**，且 195 个具名 pass 里仍有相当部分未逐个体解剖 —— **这仍是当前最大的缺口**。已解剖的集中在本轮列的这几个族。

---

## 二、按类别登记的缺口

### A 类：rhapsody 剩余未取证函数 —— **✅ 已闭合（G1）**

原登记 159 函数 / 46,091 字节，**已由 PE 直读全部反汇编**（`g1-pe-leaves/pe-leaves.json`，159 项，size 合计 46,091 B，置信度 high 92 / medium 63 / low 4）。

**其中的关键收获**：
- `RawManagedHandle` 的**真实字段名**（来自 `.rdata`）：`resource@+8`、**`resource_checksum@+0x10`**、**`device_id@+0x14`**、`resource_type@+0x16`。⇒ **A4/A5 的 "generation" 实为 `resource_checksum`，"store id" 实为 `device_id`**（行为不变，命名须改）。
- `handle::clone` = **refcount+1 且无分配**；`handle::eq` = `resource` 相等且 `+0x0f..+0x16` 8 字节相等，**`+0x00` 不参与**。
- **资源类型 tag 枚举 0..7 连续**（见 C2 更正）。
- `BufferDesc.memory_location` 枚举 = `{1=GpuOnly, 2=CpuToGpu, 3=CpuToGpuLowPrio, 4=GpuToCpu}`；`TextureViewDesc` 布局全解。
- `image_aspect_mask_from_format` 的表与分支全解。

**遗留（低优先）**：4 类**非叶子**大函数（`create_texture`×4、`create_buffer`×4、`with_setup_cb`×3、`leekspin_print_dump` 11,396 B）**已完整反汇编但未做逐语句语义归纳**；以及若干 medium/low 项见 `g1-pe-leaves/findings.md` 第 9 节。

### B 类：country_core 构图生产者（**最大缺口**）

| 项 | 状态 |
|---|---|
| compile 闭包顺序（36 步） | ✅ 已记录（`c1-country-core/frame-composition.json`） |
| render_world 顺序（59 步） | ✅ 已记录 |
| **195 个具名 pass 的函数体** | ❌ **只解剖了 1 个** |
| `GpuWorld` / `nani` 的 CPU→GPU 数据模型 | ❌ 几乎未动 |
| 7 个 `PassResources` TypeId 的具体类型名 | ❌ 不可反推 |

### C 类：语义/结构未闭合项

| # | 缺口 | 现状 |
|---|---|---|
| C1 | ~~**`queue+0x80` 的写者未定位**~~ **✅ 已攻克（G8）**：写者 = **`Queue::update_pending_submissions`(`0x140bf0b90`)** —— 第 19 行用 ash 槽 `+0x478`（`vkGetSemaphoreCounterValue`）读 `queue+0x88` 的时间线信号量，**第 42 行把返回的 u64 写入 `queue+0x80`**。`reclaim`/`queue_submit` 只读，与登记吻合 |
| C2 | **部分攻克（G1+G8）**：资源类型枚举 `{Texture0, Buffer1, Swapchain2, TextureView3, RenderPass4, Sampler5, PersistentDescriptorSet6, Framebuffer7}` —— **0..7 连续**；跟踪态枚举 `{Untracked0, Tracked1, MarkedUnused2}`。桶 = **9 个句柄 `Vec`（步长 0x18）+ 0x48 记录 `Vec`**；**已定名 `+0x278` = `vkDestroyFramebuffer`、`+0x288` = `vkDestroyRenderPass`**（用 create 槽 `+0x270`/`+0x280` 标定）。**9 ≠ 7** 的解释：多出 2 个排非 store 资源（`+0x6b8`/`+0x6f0` 疑为命令缓冲/池）。**`case 9` 不是跳表字面 case，是 trait/vtable 派发判别值**。**遗留**：9 槽中其余 `vkDestroy` 名受 ash 分叉表限制未定 |
| C3 | **部分攻克（G3+G4+G5+G6+G7+G8）**。**生产者归属已恢复**（PE 扫 167 个调用点）：`0x02` 间接 / `0x1a` `ComputePass::read_rg_storage_buffer` / `0x24` `DrawBuilder::read_rg_storage_buffer` / `0x34` 写·清 / `0x37` `write_rg_storage_buffer` / `0x17` `RasterPass::new_impl`；**访问器**：读 = `0x1a`/`0x1f`、**写 = `0x32`**、并发写 = `0x37`。**合并格完整恢复**：`0x17`+`0x1d`→`0x1e`、`0x1a`+`0x02`→`0x02`、`{0x19,0x1a}`+`0x37`→`0x37`、其它非同组合 panic。**⇒ `0x1e` = 读\|写 合并态**。**遗留**：`0x19`/`0x1d` **扫描不到字面生产者**（应为运行期计算值），语义仍未知 |
| C4 | **登记边界（G8）**：`+0x48` 的 u64 = **`{低 32 = access 掩码, 高 32 = stage 掩码}`**，初值 `{1, 0x2000}`。其余位由**外部 `vk-sync-fork` 的 `FUN_140c10dd0(code) -> (access, stage)`** OR 进来 —— **按约束不逆向该库** |
| C5 | ~~不可取~~ **✅ 已攻克（G1+G8，字节级）**：`BatchCompiler::new`(30B) = 清零 `{Vec cap@0, ptr@+8(=8), len@+0x10}` 与 `cur_batch{start@+0x18, end@+0x20, flag@+0x28}`；`pass_access_range_indices`(5B) = **恒等透传** |
| C6 | ~~无函数体~~ **✅ 已攻克（G1+G8，字节级）**：见 `g1-pe-leaves` 与 `g8-semantics` 第 2 节（`device_uuid` = `P+0x690` 的 16 B；`physical_device` = `P+0x10`；`min_subgroup_size` = `(this+0x40, this+0x44)` 等） |
| C7 | `GraphicsDevice+0x168`/`+0x3f8`/`+0x400`/`+0x408` **身份未定** | 反编译中未作为设备字段出现 |
| C8 | 实例的 `ppEnabledExtensionNames`/`ppEnabledLayerNames` **实际数组内容**；`applicationName`/`engineName` | 由串表推断 |
| C9 | `Surface::create` 的 `0xc`/`0xf` 精确枚举 | **回到 unknown**（A6 的映射已作废） |
| C10 | `O+0x680` 的精确字段映射 | 推断 |
| C11 | descriptor 布局 tag `3`、`6..11` 的语义 | sparse 12 臂已解，语义未定 |
| C12 | `present` 中 `managed+0xb9` 标志；swapchain 槽 `[0x18..0x37]` 成员名 | |
| C13 | `ComputePass`/`DrawBuilder` 的**字段名**（0x190/0xf8 是"按值拷贝长度"，**未必等于分配尺寸**） | |
| C14 | `UniformBinder` 的访问声明点 | 未见其调 `add_resource_access` |
| C15 | `TextureViewDesc` 精确布局；`BufferDesc` 内存类别枚举取值语义；`TextureDesc.usage` 位构造器 | |
| C16 | `dynamic_constants` 的 `device_address`/`into_gpu_lifetime_tracker` 体；`end_render_pass`(`0x1416f7850`) 体 | |
| C17 | `queue_submit` 的**签名不一致**（反编译 5 参 vs 调用点 3 参） | 存疑未解 |
| C18 | `submit_pending_cbs` 内 **20ms 阈值** `uVar9` 的来源 | 疑反编译失真 |

### D 类：方法性缺口（影响所有结论的强度）

| # | 缺口 | 影响 |
|---|---|---|
| D1 | **`disassemble_function` 恒返回 0 指令** | 任何"看到汇编"的断言不可由该工具复现；**已改由直接读 PE 复核** |
| D2 | **`get_xrefs_*` 只有 DATA 引用、无 CALL 边** | 前几轮所有"唯一 caller"都是**伪代码直读推断**，非 xref 证据 |
| D3 | **零动态证据** | 全部结论为静态机器码推断 |
| D4 | PDB 函数签名普遍 `void()` 占位 | 真实 ABI 不可从 PDB 恢复 |

---

## 三、已建立的可复用方法（第二轮的净收益）

1. **PE 直接读跳表**：Ghidra 报 `Bad instruction` 的根因是 int32 相对跳表落在 `.text` 内被当代码解码。按节表映射 RVA→文件偏移、读 int32、`目标 = imagebase + table_rva + rel`。已验证 3 张表。
2. **PE 直接读机器码**：绕过 Ghidra 对微小叶子函数的建函数/反汇编失败。已验证 5 个函数，解出 `PassResourceAccessType::new` 的双参交换等关键事实。
3. **并发 lane 工具**：`scripts/render-recovery/run-lanes.mjs`（每 lane 独立 `database_id` → 独立 Ghidra 项目，真并行）。**注意其 `batchPayload` 曾缺一层 `.error` 下钻，父级已修**。
4. **静态调用图替代 xref**：`workgraph/calls.json`（450 函数）可闭合部分调用者（D2 的绕行方案）。

---

## 四、优先级建议

| 优先级 | 缺口 | 理由 |
|---|---|---|
| **P0** | **B 类：country_core 195 个 pass 的函数体** | 475,588 B，只碰了 1 个；是"游戏怎么 build graph"的真正主体 |
| **P0** | **A 类：rhapsody 剩余 159 函数（PE 直读）** | 方法已就绪，**预期可闭合到接近 100%** |
| P1 | C1/C2/C3/C4 | 影响 barrier 与生命周期语义的完整性 |
| P1 | C5/C6 | 少数函数体不可取，需换方法（PE 直读 + 调用点推断） |
| P2 | C7–C18 | 细节补全 |
| P2 | B 类的 `GpuWorld`/`nani` 数据模型 | CPU→GPU 桥接，服务场景恢复 |
| — | D1–D4 | 方法性限制，**不消除则任何结论只能是静态推断** |
