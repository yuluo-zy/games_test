# T2 调查：调用点 ABI 反推签名的可行性与上限

- 目标：PE `workspace/input/tiny-glade.exe`（SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`）+ PDB `D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb`（274 MB）。
- 方法：自写 MSF/PDB 解析器 + PE `.pdata` 函数边界 + `.text` 全量 `E8 rel32` 扫描 + 调用点切片还原实参寄存器。**只读**，未改动任何受限文件。
- 产物：`t2-callsite-abi/{abi_recover.py, analyze_sigs.py, pdb_scan.py, stream_scan.py, stream_scan2.py, signatures*.json, demo_instances.json, undecidable.json, pdb_findings.json, pdb_per_module.json, signatures_summary.json}`。

## 0. 父级复核

- `signatures_summary.json`：`targets 450`、`pdata_functions **116,269**`、`recovered_with_argument_count **213**`、`undecidable_no_direct_callsite **237**`；`arg_count_histogram {0:15, 1:42, 2:66, 3:63, 4:27}`；`confidence {high:175, medium:26, low:12}`。**核对一致。**
- `signatures_queue_submit.json`：`n_incoming_direct_callsites: 1`、`consensus.observed_arg_regs ["rcx","rdx"]`、`sources {rcx: mem:[rbp+352], rdx: reg:rsi}`、一个栈槽 `[rsp+32] ← rbx`、`confidence "high"`。**核对一致。**

## 1. **PDB 里 `S_LOCAL` 对 Rust 游戏码全为空**（关键否定）

自写解析器扫描全部 **4,892** 个流（绕过损坏的 DBI ModInfo，改用「流以 `S_OBJNAME` 开头」识别模块流）。

**全 PDB 记录直方图**：

| 记录 | 数量 |
|---|---:|
| `S_LOCAL` | 101,046 |
| `S_FRAMEPROC` | 85,308 |
| `S_GPROC32` | 62,480 |
| `S_LPROC32` | 22,832 |
| `S_OBJNAME` | 5,054 |
| **`S_REGREL32`** | **37** |
| `S_REGISTER` / `S_BPREL32` | **0 / 0** |

**实例（真记录，但全是 C/C++ 对象）**：`S_REGREL32 {name:"this", reg:335(RSP), off:48}`、`{name:"block", off:8}`；`S_LOCAL {name:"InputBuffer", flags:1}` —— 名字全是 **Windows 色彩管理 / 运行库**符号。

### 关键否定
- **4,328 个 Rust 游戏模块（obj 路径含 `tiny_glade/rcgu.o`）的 `S_LOCAL` 计数全为 0**；纯 Rust 模块的 `max S_LOCAL = 0`。
- 全仓库仅 **75** 个模块有 `S_LOCAL`，全部是随构建打包的 **C 依赖**（lcms2-sys、zstd-sys、ring）与 **MSVC 运行库**对象。

⇒ **PDB 对 rhapsody 函数不提供任何参数寄存器/栈偏移信息。**

**旁证**：游戏函数的 `S_GPROC32.FunctionType` 在 TPI 里解析为 **`LF_PROCEDURE 0x1001` = `void()`**（`ParameterCount 0`，空 `ARGLIST 0x1000`）—— **占用型**。但 `S_GPROC32` 的 **name 字段是可读 Rust 全路径且带泛型实参**（偶发含 `void (*)(Res<..>,..)` 函数指针类型）—— 属**名称层**信息，**非签名层**。

## 2. 纯 ABI 反推：能力与上限

工具 `abi_recover.py`：解析 PE `.pdata`（`RUNTIME_FUNCTION`）取**全部函数边界**（**116,269 个**）→ 对 `.text` 全量扫 `E8 rel32` 找**所有**直接调用点（不止 `calls.json` 的 450 个 caller）→ 定位 `call` → 向后切片还原 `RCX/RDX/R8/R9` + `XMM0-3` + `[rsp+disp]` 栈实参，遇 `call`/`ret`/`jmp` 设屏障。

**实例（均附原始 `e8xxxxxxxx` 字节）**：

| 函数 | 结果 | 置信度 |
|---|---|---|
| `ResourceInfo::calculate`(`0x140530430`) | 3 个调用点**全部** `RCX,RDX,R8,R9` | high |
| `submit_pending_cbs`(`0x140bebdf0`) | 3 点一致 **2 实参**（`rcx←rdi`，`rdx←[rbp+528]`） | high |
| `update_pending_submissions`(`0x140bf0b90`) | 2 点 **2 实参** | high |
| `add_resource_access`(`0x1409da90`) | **0 个直接调用点**（地址仅以 RVA 出现在数据里） | — |
| `PassBuilder::render`(`0x1409dd4d0`) | **0 个直接调用点** | — |
| `queue_submit`(`0x140c04e20`) | 唯一直观点 `0x140bebfd6`：`rcx←[rbp+..]`、`rdx←rsi`，另 `[rsp+0x20]←rbx` | high |

### **解开 `queue_submit` 的签名矛盾**
反编译给出 **5 参**，而机器实参证据是 **`rcx`/`rdx` 两个寄存器 + 一个栈槽**，**至多 3**。
⇒ **反编译器的 5 参模型不被支持**；该函数的真实参数**少于 5**。这个悬了几轮的不一致**到此定性**（Ghidra 是从栈帧用量推断的参数个数）。

## 3. 覆盖率（450 个 rhapsody 函数批量）

| 指标 | 数值 |
|---|---:|
| 有 ≥1 直接调用点 | **213 / 450（47.3%）** |
| 其中 high 置信 | 194 / 450（43.1%） |
| **不可判定** | **237 / 450（52.7%）** —— 原因**全部是「无直接调用点」**（间接 / vtable / 仅取地址） |

213 个的实参个数分布：`0→15, 1→42, 2→66, 3→63, 4→27`。42 个含第 5（栈）槽；59 个观测到 `XMM`（浮点）实参。
实参来源类别：reg-copy 203、栈/帧内 mem 201、其它 mem 62、立即数 22。

> **rhapsody 核心大量经间接派发** —— `add_resource_access`、`PassBuilder::render` 都是 **0 个直接调用点**。这直接限制了静态 ABI 恢复的上限。

## 4. 诚实边界（原理上不可得）

- **类型永远拿不到**：调用点只给「个数 + 来源类别（寄存器/栈/常量）」，**不含类型**。本报告的 `arg_count` 是**机器实参寄存器个数**，**不是 Rust 源实参个数**（Rust 不走 C ABI：胖指针、间接传参、sret 常见）。已按此口径产出，**未编类型**。
- 经 vtable / 函数指针的调用（本次 **52.7%**）**静态不可见**；仅取地址者亦然。
- 被上游"继承"的实参寄存器（未在本窗口写入）**不可观测**，标低置信。
- **无 `S_LOCAL`、函数类型为 `void()` 占位** ⇒ **单靠该 PDB 无法直接得到 rhapsody 签名**；调用点 ABI 是当前**唯一**可用途径，且只能给「个数 + 来源」。
- ICF 共享地址的多个语义名未合并（沿用 workgraph 口径）。

## 5. 本轮附带产出（对 T4/T5 有价值）

**`.pdata` 的 `RUNTIME_FUNCTION` 给出 116,269 个函数边界** —— 这比 workgraph 的 450 个函数面大得多，**是建立全量调用图的现成边界来源**。

## 6. 结论

| 问题 | 答案 |
|---|---|
| PDB 能给出参数吗？ | **不能**（Rust 游戏模块 `S_LOCAL` 全 0） |
| ABI 反推能给什么？ | **实参个数 + 来源类别**，覆盖 **47.3%** |
| 能给类型吗？ | **不能**，且**原理上不能** |
| `queue_submit` 5 参矛盾？ | **解开** —— 机器证据支持 ≤3 |
| 剩余一半怎么办？ | **只能靠 T5（vtable/间接解析）** |
