# T1 调查：PDB 类型记录能否恢复签名（**明确否定**）

- 目标：`D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb`（274,710,528 B）。
- 方法：自写 MSF + CodeView 解析器（读原始 PDB，只读）+ `llvm-pdbutil 20.1.0` 交叉。两者结果**完全一致**。
- 产物：`t1-pdb-types/{parse_pdb_types.py, parser_output.json, SUMMARY.json, aggregate_symstats.py, symbol_kind_aggregate.json, tpi_full.txt, ipi_samples.txt, symbol_stats.txt, commands.txt}`。
- 未改动原作目录，未跑 cargo。

## 0. 根因（**父级已独立验证**）

PDB 里存着构建命令：**`"-C" "debuginfo=line-tables-only"`**（`grep -a -c` 命中 **120** 次；父级复核）。

⇒ **Rust 侧只产出行表 + 函数级 + inline site，按设计不产出参数、局部变量与类型。** 这**一条解释了以下全部现象**，也是本次调查最有价值的产出。

## 1. 逐类记录数量

**TPI（stream 2）共 6,125**：`LF_PROCEDURE` **1,710**、`LF_MFUNCTION` 86、`LF_ARGLIST` 1,466、`LF_STRUCTURE` 753、`LF_POINTER` 828、`LF_FIELDLIST` 486、`LF_MODIFIER` 215。
**全部是 C/C++ 第三方**（`_cmsContext_struct` = LittleCMS、`_KNONVOLATILE_CONTEXT_POINTERS` = VC CRT、`ISA_AVAILABILITY`）。
⇒ **TPI 里 rhapsody 出现 0 次。**

**IPI（stream 4）共 132,599**：`LF_FUNC_ID` **83,429**、`LF_STRING_ID` 43,625、`LF_MFUNC_ID` 35、`LF_BUILDINFO` 4,838、`LF_UDT_SRC_LINE` 670。

**子流直方图（父级确认 T2 的同类数字）**：`S_LOCAL` 202,093 但**只落在 76 个模块**（`* Linker *` 自身 101,047 + zstd-sys/lcms2-sys/ring(C) + intel_tex_2 ISPC + MSVC CRT）。**纯 Rust 的 `*.rcgu.o` 模块 `S_LOCAL = 0`。** `S_REGREL32` 仅 75、`S_UDT` 4。
模块符号几乎只有 `S_GPROC32`(124,966) / `S_LPROC32` / `S_INLINESITE`(6.45 M) / `S_END` / `S_FRAMEPROC` / `S_LINES` / checksums ⇒ **无参数名、无局部变量、无寄存器相对变量**。

## 2. 参数类型是否可用 → **否**

- TPI 索引 **`0x1001`** = `LF_PROCEDURE{ return = void, #args = 0, cdecl }` —— 即 Rust 的 **`void()` 占位**。
- **83,429 个 `LF_FUNC_ID` 中 80,103 个（96.0%）指向它**，其余指向少量 C/CRT 类型。
- **rhapsody 的 943 个 `LF_FUNC_ID` 全部指向 `0x1001`，无一例外。**

**实例演示**（地址经 `S_PUB32` 公共符号核对，函数身份与 IPI scope 完全吻合）：

| 地址 | 名字 | IPI `FUNC_ID` → 类型 |
|---|---|---|
| `0x140530430` | `rhapsody::rg::resource_info::ResourceInfo::calculate` | **`0x1001` void()** |
| `0x140beabd0` | `rhapsody::rg::executor::GraphExecutor::begin_frame` | **`0x1001` void()** |
| `0x140c04e20` | `rhapsody::raw::device::GraphicsDevice::queue_submit` | **`0x1001` void()** |
| `0x1409dca90` | `rhapsody::rg::pass_builder::PassBuilder<UserData>::add_resource_access` | **`0x1001` void()** |

`ResourceInfo` 的其余方法（`allocate`/`extract_pass_resource_access`/`calculate_lifetime_events`/`clear`/…）**同样全是 `0x1001`**。

## 3. 覆盖率

**450 个 rhapsody 函数中，能恢复有意义参数类型的：`0`。**
所有 rhapsody 函数的类型指针都收敛到同一个 `void()` 占位（个别无 IPI 记录的连占位都没有）。

## 4. `void()` 占位核实

PROJECT_MEMORY 记的"native 签名全为 `void()` 占位"——**独立验证成立，且比记录更强**：
是 **83,429 个 `FUNC_ID` 中的 80,103 个（96.0%）**，而非仅 4,124 个。量级差异可能是旧筛选口径不同，**定性结论一致**。

## 5. ⚠️ 父级更正：T1 有一处**过度断言**

T1 报告写：「"带完整泛型参数的名字"来源已查清：**PDB 里找不到**（mangled 名无泛型、TPI 无、IPI 字符串无 `<`）；那串是**外部（demangler/工程重建）产物**，非 PDB 记录。」

**这是 T1 把结论范围扩大了。** 事实：

1. **父级已用 `grep -a` 在 PDB 文件里逐字节命中**这些名字（`queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo` 命中 **2** 次；`get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources` 命中 **8** 次），并在 offset `31700481` 直接读出了原文。
2. T3 另经 `llvm-pdbutil dump --symbols --modi=N` 追到**模块符号流**（`S_GPROC32` 的 name 字段），并发现 389 个已缓存 module dump 中 **352 个**含 `impl$/closure_env$`。
3. **T1 只检查了 TPI（stream 2）与 IPI（stream 4）的 `LF_STRING_ID`**，**没有检查模块符号流**。它的"PDB 里找不到"实际只是"TPI/IPI 里找不到"。

⇒ **T3 的结论成立**（泛型名是 PDB 数据），T1 的该项断言**作废**。
⇒ 但注意：**这不改变本任务的结论** —— 泛型名只给「路径 + 泛型实参」，**仍不给签名**。

## 6. natvis

3 个 natvis 只描述 `windows_strings::HSTRING/PSTR`，**与 rhapsody 无关**（与 T3 结论一致）。

## 7. 不可得（明确）

**参数类型、参数名、返回类型、局部变量、Rust 结构体/枚举布局、trait/泛型实例化、Rust 调用约定** —— **PDB 均无法给出**。
**根因是构建标志 `debuginfo=line-tables-only`**，不是解析技术问题。

⇒ 字段布局只能靠**反汇编**（工程一直在做的偏移级观察），签名只能靠**调用点 ABI**（T2），且两者都拿不到类型语义。

## 8. 对"可实现性"的最终影响

| 问题 | 答案 |
|---|---|
| PDB 能恢复签名吗？ | **不能**（根因：`line-tables-only`） |
| PDB 能恢复 Rust 类型布局吗？ | **不能**（TPI 里 rhapsody 出现 0 次） |
| void() 占位是全部吗？ | **96.0% 是**，rhapsody 侧 **100% 是** |
| 那泛型名呢？ | 是 PDB 数据，但**只给泛型实参，不给签名** |
| 还剩什么路？ | **反汇编（字段偏移） + 调用点 ABI（参数个数）**，**没有类型** |
