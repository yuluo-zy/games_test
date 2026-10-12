# T5 调查：间接调用与 vtable 解析

- 目标：PE `workspace/input/tiny-glade.exe`（SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`）。
- 方法：按 `.pdata`（116,269 个函数体）**逐函数有界解码**（单次线性扫描会失步）+ capstone + 数据流/寄存器别名 + vtable 段识别。**全程只读**。
- 产物：`t5-indirect-vtable/{analyze_indirect.py, resolve_and_vtables.py, indirect-callgraph.json(65 条内部间接边), vtables.json, indirect_raw.json, indirect_resolution.json, vtables_raw.json}`。
- 未碰任何禁改路径。

## 0. 父级复核（关键纠正）

| 断言 | 复核 | 结果 |
|---|---|---|
| `0x1436A55xx`–`0x1436A5AF8` **不是** vtable，是 **`.pdata`** | 按节表映射 RVA | `0x1436A55F8` → RVA `0x36a55f8` → **`.pdata`**；`0x1436A5AF8` → RVA `0x36a5af8` → **`.pdata`** ✓ |
| `0x143634E40`（leek 环形缓冲）在 `.data` | 同上 | **`.data`** ✓ |
| `0x143310B40`（range/调用点表）在 `.rdata` | 同上 | **`.rdata`** ✓ |

⇒ **T5 成立。**

## 1. ⚠️ **对全工程的一处重要纠正**

我在会话中以及多份 findings 里反复写过：
> "这些函数**各仅 1 条 DATA 引用**（**vtable 槽 `0x1436A55xx`–`0x1436A5AF8`**），无代码 caller ⇒ 经 vtable 间接派发"

**这个解释是错的。** 那两个地址落在 **`.pdata` 异常目录**（`RUNTIME_FUNCTION` 的 `BeginAddress` 字段），**不是虚表槽**。所谓"1 条 DATA 引用"**就是 `.pdata` 里指向该函数的 `BeginAddress`**，是异常处理元数据，**与调用无关**。

**波及范围**：A1、A4、R1、R2、R3、R4、T2 等 findings 中"vtable 槽 `0x1436A55xx`"的表述**均应作废**。

**正确的理解**（T5 给出）：
- `begin_frame`(`0x140beabd0`)、`immediate_empty_bucket`(`0x140bf92f0`)、`queue_submit`(`0x140c04e20`) **都不是 vtable 槽**。
- 后两者在 `calls.json` 中有**直接 `E8` 调用边**。
- `begin_frame` 在 **450 函数集内无入边**，因为它的调用者 `render_loop::render`(`0x1404e8010`) 属 **tiny_glade**，**不在 `calls.json` 的函数集里**（该图只覆盖 450 个 rhapsody 函数的**内部**边）——这正是 T4 要补的洞。

## 2. 全镜像间接调用分类（观察）

按 `.pdata` 函数体界定（覆盖 `.text` **92.9%**，解码 **9.32 M** 条指令）：

| 形态 | 数量 |
|---|---:|
| 间接 call（`FF /2`，`call r/m64`）**合计** | **18,593** |
| ├ `call [rip+disp]` | 4,135 |
| ├ `call [reg+disp]` | 7,858 |
| ├ `call reg` | 6,591 |
| └ `call [reg+idx*scale+disp]` | 9 |
| `call` 绝对值 | 0 |
| `FF /3`（far call） | 0 |
| 间接 jmp（`FF /4`） | 6,223（`jmp [rip]` 96 / `[reg+disp]` 91 / `reg` 6,036） |
| 直接 `rel32` 转移（背景值） | 643,887 |

**`call [reg+disp]` 的 disp 直方图以 `+0x18` 绝对占优（3,257 条）** —— 典型的 **trait 方法槽 `call [vtable+0x18]`**（`drop`/`size`/`align` 之后的首槽）。

## 3. `call [rip+disp]` 的目标：**全部是数据槽，产出 0 条内部边**

- **3,999 条**命中 **32 个 DLL 的具名导入**（440 个名字；IAT 槽存 `IMAGE_IMPORT_BY_NAME` RVA）；
- **133 条**为**序号导入**（IAT 高位 `0x8000…| 序号`）；
- **3 条**指向 CFG 派发指针 `0x143191170`（`GuardCFDispatchFunctionPointer`）。

⇒ 这类指令**不产出任何内部调用边**。

## 4. vtable 识别（观察）

**本镜像的函数指针表用 `64 位绝对 VA` 编码（不是 32 位 RVA）。**

- `.rdata`/`.data` 中 u64 连续的 `.text` 指针段：**7,553 段 / 47,058 槽**；
- **高置信（≥3 槽且 ≥60% 目标是 `.pdata` 函数入口）：772 张**；
- 其中 **390 张**前面紧邻 `[drop_in_place(指向 .text), size, align]` 头 ⇒ **标准 Rust dyn 虚表**；
  样例：某 8 槽表解出 `PngDecoder as ImageDecoder::{dimensions, original_color_type, …}`。
- 槽数分布以 **3 槽（435 张）** 为主，最大 **124 槽**。

**另**：`.rdata 0x143310b40` 有一段 `(函数 RVA, u32 元数据)` 的 **range / 调用点表**（与 vtable 无关）。

## 5. 与既有结论的一致性

1. **`GraphicsDevice+0x78` 的 ash 设备函数表**是 **loader 运行时填充**的外部指针，**镜像内不构成静态表** ⇒ **静态不可解**（与 SYNTHESIS/GAPS 一致）。
2. `immediate_empty_bucket`、`queue_submit` 有**直接 `E8` 边**；`begin_frame` 无入边（原因见第 1 节）⇒ **三者均非 vtable 槽**。
3. `ManagedResourceStore` / `RenderPassApi` **无方法出现在 dyn 虚表** ⇒ 与直接调用模型一致。
4. **450 个 rhapsody 函数中 13 个确为虚表方法槽**：`raw::error::impl$N::fmt`（即 `dyn Error` 的 Debug/Display）、`rg::recorder::impl$1::render<country_core::…>`、`GpuAllocator::inner`、`DynamicConstantsChunk::device_address`、`ComputePipelineDesc::new` 等。⇒ **trait 派发确实触达部分 rhapsody 方法。**

## 6. 内部间接边与不可解析比例

用直线段内最近定义（`lea`/`mov rip`、寄存器别名、全局槽二次解引用）解析：

- **得到 65 条内部间接边**（54 个源函数），**仅占 18,593 条的 0.35%**。
- **不可解析主因**：**14,258 条**的基址寄存器**在函数内无定义**（是 `this` 入参、或更早从结构体字段载入，需**过程间/指向分析**）；**136 条**槽值非 `.text`（外部/其它）。
- `call reg` 的纯虚派发**在静态下本质不可定**。
- 其余不可得：ash 设备表内容（运行时）、133 条序号导入名、`.pdata` 未覆盖的 **7% `.text`**。

## 7. 假阳性控制

- **仅把 64 位 VA 段当作 vtable 候选**；
- **32 位 RVA 段被明确否决**（96,212 段 / 343,608 槽，`begin_frac` 仅 **0.002**）—— 是 range/偏移表，**典型陷阱**；
- 要求 **≥3 槽且 ≥60% 命中函数入口**；
- 再用 Rust 头 `[drop, size, align]` 复核（**390/772**）；
- 内部边要求**直线段、无分支穿越、无寄存器覆写**，且槽/目标都校验落在 `.rdata`/`.text`。

## 8. 结论

| 问题 | 答案 |
|---|---|
| `call [rip+disp]` 能解出内部边吗？ | **不能**（全是 IAT，0 条） |
| vtable 能识别吗？ | **能** —— 772 张高置信，其中 390 张是标准 Rust dyn 表 |
| 内部间接边能解多少？ | **0.35%（65 / 18,593）** |
| 为什么这么低？ | **14,258 条的基址是 `this` 参数** ⇒ 需过程间分析；`call reg` 本质静态不可定 |
| `0x1436A55xx` 是 vtable 吗？ | **不是，是 `.pdata`**（**纠正全工程的一处误述**） |

**诚信**：静态边 ≠ 运行时执行；ICF 合并地址会让多个逻辑函数共享入口；Ghidra 的 `disassemble_function` 恒返 0，故反汇编全部由 PE 字节经 capstone 自做（分析中已给出字节，如 `41 ff 55 00` = `call [r13]`）。
