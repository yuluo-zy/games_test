# Tiny Glade 逆向：当前状态与判断

- 日期：2026-10-12（三轮取证 + 一轮可行性调查之后）。
- 目标：`D:/game/ljxsj_92385/Tiny Glade/tiny-glade.exe` / `tiny-glade.rne`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB `tiny_glade.pdb`（274,710,528 B）。
- 本文件是**判断**，不是证据；证据索引见文末。
- 涉及 rhapsody 的取证过程见 [evidence/render-recovery/producer-closure/](evidence/render-recovery/producer-closure/)。

---

## 一、一句话结论

> **函数面读完了、主要机制清楚了、直接调用图建好了；但"能否 1:1 复刻"卡在签名与类型上，且静态手段已到头。**
> **换目标为"按机制自己写一个可运行的实现"，则基本可行 —— 因为那不需要原作的 API 面，只需要 shader 合同与 GPU 布局对齐。**

---

## 二、已经确立的（分档，不混为一谈）

### A 档：机制清楚，可照着写

`raw::managed::*`（句柄 ABI 逐字节、generation 规则、`dispose` 不调 `vkDestroy`、16 字节退役记录）、`rg::executor`（帧循环、帧节流 `VecDeque<PendingSubmission>`、`ensure_active_cb` 每 batch 一个 CB）、`raw::barrier`（并发判定规则逐行）、`rg::temporal`（`0 → export → 2 → 次帧 reimport → 1` 状态机）、`rg::resource_pool`（每键独立 LIFO 空闲栈）、`rg::resource_registry`（`populate_resources` 按来源×用途分支）、`rg::compiled`、`raw::semaphore`、`raw::format`、`raw::dynamic_constants`（256 KiB 页 bump 分配器）、`raw::buffer`/`raw::texture`（`BufferDesc` 16B / `TextureDesc` 32B，含 `.rdata` 真字段名）。

### B 档：主干清楚，有明确未闭合项

`raw::device`（布局大部分有；多个分派槽名是**推断**）、`raw::init`/`raw::instance`（扩展清单有，**`ppEnabledExtensionNames` 实际数组未恢复**）、`raw::swapchain`（分支与重建闭环清楚；9 个 destroy 槽只定名 2 个）、`raw::shader`（blend/vertex 跳表已解；**反射生产链在外部 crate**）、`raw::disposal`、`rg::pass_builder`、`rg::pass_api`、`rg::recorder`。

### C 档：country_core 构图机制（第三波新增）

`draw_passes` 四方混合管线分工、`shadow_pass` 3 级联、water/ice 的 `.hlsl` 文件名、UI 与 loading 共用模板、cull/bucket 的完整 binding 表、20 字节间接命令、两轮可见性的全局字节门控、GI 的 CPU 软建 BVH（SAH 常数）+ candidate 链、场景生成链、TAA/TGSR 分支、DoF、vermere 收敛门控 UI。

顺序层面：**compile 闭包 36 步 + `render_world` 59 步全部已知**。

### D 档：仍未命名 / 未定（见 [GAPS.md](evidence/render-recovery/producer-closure/GAPS.md)）

访问码 `0x19`/`0x1d` 的语义（**扫描不到字面生产者，应为运行期计算值**）；7 个 `PassResources` TypeId 的类型名；`restir_di` 蓄水池细节；`bin_lights` 分桶；`painterly`/`vermer` 算法；SPIR-V 内的遮挡判据；9 个 destroy 槽中其余 7 个的名字；`+0x680` 绝对基址；`RawManagedHandle +0x00` 身份。

---

## 三、三条结构性障碍（本轮的定性结果）

### 障碍 1：**签名与类型 —— 静态无解**（T1/T2/T3 三路独立收敛）

| 路径 | 结论 | 根因 |
|---|---|---|
| PDB 类型记录 | **不能** | 构建标志 **`-C debuginfo=line-tables-only`**（PDB 中命中 120 次）；**rhapsody 在 TPI 里出现 0 次**；943 个 `LF_FUNC_ID` **全部指向 `void()` 占位** |
| 符号名 | **不能** | 全库 **legacy 修饰（`_ZN` 71,457 条）**，按设计不编码签名；**无 v0 符号**（`_R` 仅 2 条且是 MSVC 运行期检查） |
| 调用点 ABI | **只能得参数个数** | 覆盖 **47.3%（213/450）**；**永不含类型**；且 `arg_count` 是机器实参寄存器个数，**不是 Rust 源实参个数** |

**⇒ 没有任何静态源能给出 rhapsody 的参数类型或返回类型。**

附带解开一处悬案：`queue_submit` 反编译的 **5 参不被机器证据支持**（实为 `rcx`/`rdx` + 一个栈槽，**≤3**）。

### 障碍 2：**调用图 —— 直接边解决，间接边无解**（T4/T5）

- **直接调用图已建成**：全镜像 **419,936 条 `call rel32`，99.97% 解析到函数起点**；三法（已提交 / objdump 重跑 / 自研 capstone）**边集零差异**。
- **country_core 层补上了**：1,829 符号 → **1,147 唯一 VA**，层内 2,494 条边。
- 主链三方互证：`render → compile → render_world → {各 pass}`。
- **间接调用静态无解**：全镜像 20,840 处，**只解出 0.35%**；主因是 **14,258 条的基址是 `this` 入参**（需过程间分析）。

### 障碍 3：**零动态证据**

未运行过游戏、未捕获帧。所有分支结论都是静态推断。

---

## 四、关于"不 1:1 复刻，自己写" —— 判断

**可以做，而且已经有人在做了**（`reconstruction/rhapsody-rebuild/` 本身就是按机制写的候选，不是 1:1 复刻）。

**关键：把目标从"复刻原作 API"改成"自己写一个能跑的"，障碍 1 的杀伤力大幅下降** —— 签名和内部结构变成了你自己的设计自由。

**但边界由两样东西划定，它们不能自由发挥：**

1. **Shader 合同**。`compiled-shaders` 里 **404 个编译后 shader 是既成事实**（234 raster / 170 compute）。它们的 descriptor set/binding 号、push constant 布局、顶点属性、`RWStructuredBuffer<T>` 的 stride 都是固定的。你写 CPU 侧代码必须去适配它们，**不是反过来**。
2. **GPU 可见的布局**。缓冲区里一个字节错了画面就错：
   - `BufferDesc`（16B）/ `TextureDesc`（32B）的语义
   - 实例记录：64 字节 `RoofShingle`、`GpuInstance`、Nani 的 atlas/allocator 布局
   - `DrawIndexedIndirectCommand`（**20B = 5×u32**）、各 SSBO 的元素 stride
   - `SynchronizationScope` 的三向量元素尺寸（0x18 / 0x38 / 0x48）

**可以自由发挥的**：Rust 签名、模块划分、错误类型、内部数据结构（候选用 `BTreeMap` 替代原作单 atomic 高水位，只要行为对就行）、分配策略、并发模型。

**⇒ 判断：写得出"能跑的"，写不出"等价的"；而且"能跑"的前提是先把上面那两类布局对着 shader 逐一坐实。**

### 必须警惕的一点

候选 `rhapsody-rebuild` 能编译能跑，**但它有 15 处是候选自创的机制**（`GraphBatchSync`、内容寻址的 descriptor 缓存、`BTreeMap<domain,seq>` 跟踪、`ray_tracing.rs`、13 行 swapchain stub…）。**"能跑"不等于"还原"** —— 两者要分开记，这也是本项目 AGENTS.md 反复强调的。

---

## 五、本轮自我更正台账（三条）

| # | 我此前的说法 | 更正 | 依据 |
|---|---|---|---|
| 1 | "这些函数各仅 1 条 DATA 引用（**vtable 槽 `0x1436A55xx`**）⇒ 经 vtable 间接派发" | **错。那是 `.pdata` 异常目录的 `BeginAddress`**，与调用无关；这些函数**本来就有直接调用边** | T5 + 父级按节表映射 RVA 复核 |
| 2 | 第二轮"逐行核对确认"了 A2 的账本 `{index, access_type, sync}` | **字段标反**。正确为 `{index@+0, sync_group@+4, access_code@+8}`；我的确认**继承了同一个 Ghidra 绑定错误** | R2 + 父级 PE 读 `PassResourceAccessType::new` = `mov eax,edx; mov edx,ecx; ret` |
| 3 | "G6 的 `write = 0x32` 无证据、**不予采信**" | 措辞过重。其实质结论**成立**（G8 独立 PE 扫 167 个调用点佐证）；错的是它"更正 C1"的框架 —— builder 与 accessor 是**两套 API** | G8 |

另有一处**需要降级的既有证据**：`public-symbols.json` 的 `demangled` 字段是 `llvm-cxxfilt` 的**有损**产物（101,405 条中含真泛型的 **0 条**；含 `$` 转义的 49,348）。**凡引用它得出的类型结论应降级。**

---

## 六、下一步：只有一条路有增量

**静态手段已到头**。唯一还能突破的是**动态**，而它恰好正对着障碍 1：

| 动态能补 | 做法 |
|---|---|
| **真实参数值 → 签名/类型** | hook rhapsody 包装入口（`create_texture 0x1405b9b00`、`create_buffer`）转储 `RCX/RDX/R8/R9` + 栈 |
| **7 个 TypeId 的类型** | hook `Components::get_resource_id 0x1400dd6f0` 或 `PassResources::new_from 0x140909310` |
| **访问码 `0x19`/`0x1d` 的归属** | hook `add_resource_access 0x1409dca90` 转储 `(index, code, sync)` + 调用者 |
| **间接调用图** | 采样追踪（ETW / Intel PT） |
| **所有"推断"的分支** | 直接观察走哪边 |

**已探明的前置障碍**：游戏选择逻辑 `is_supported 0x140bda840` **拒绝 `is_old_intel`**，判据是 `vendorId == Intel` 且设备名匹配 **`HD Graphics`**；本机是 **`Intel(R) UHD Graphics 630`** —— **名字里含 `HD Graphics`**。⇒ **它很可能拒绝在这台机器上启动。**

环境实测：Vulkan 实例 **1.3.204**，设备 apiVersion **1.3.215**，**已装 layer 数为 0**（连 `VK_LAYER_KHRONOS_validation` 都没有），**RenderDoc 未安装**。

**建议**：先做**受控启动实验**（在游戏目录的副本上，不动原作），把失败点确认下来。过不了这关，动态路线不用谈。

---

## 七、证据索引

| 文档 | 内容 |
|---|---|
| [evidence/render-recovery/producer-closure/SYNTHESIS.md](evidence/render-recovery/producer-closure/SYNTHESIS.md) | 第一/二轮机制总账（第十一节为复核后结论，含三次补正） |
| [evidence/render-recovery/producer-closure/ROUND3_FINDINGS.md](evidence/render-recovery/producer-closure/ROUND3_FINDINGS.md) | 第三波（G1–G8）详录 |
| [evidence/render-recovery/producer-closure/GAPS.md](evidence/render-recovery/producer-closure/GAPS.md) | 缺口登记册（A/B/C/D 四类 + 优先级） |
| 各 `a*/r*/n*/c1*/g*/t*/findings.md` | 每个 agent 的独立结论 + 原始 job/REPORT/`0x*.c` |
| `t1-pdb-types/` `t2-callsite-abi/` `t3-names/` `t4-callgraph/` `t5-indirect-vtable/` | 本轮可行性调查五份报告 |

**方法性限制（对所有结论生效）**：Ghidra `disassemble_function` 恒返回 0 指令；`get_xrefs_*` 无 CALL 边且会把 `.pdata` 误报为引用；PDB 签名恒 `void()`；**零动态证据**。
