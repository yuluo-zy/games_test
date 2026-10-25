# rhapsody 全量函数级深挖 Playbook（2026-10-15）

目标：**重新审查原版 rhapsody 的函数骨架**——具体到每个函数、数据结构、业务处理逻辑和算法，
产出逐模块 ALGORITHMS.md + 原版 vs `reconstruction/rhapsody-rebuild` 的差距登记。
**不修改任何 reconstruction 源码。** 机器码（B patched EXE / 原 RNE）为权威。

## 1. 数据源与目录

| 项 | 位置 |
|---|---|
| 原版骨架（PDB 元数据，**无地址**） | `skeleton/B/crates/rhapsody/**/*.rs`（849 符号 / 466 compiled） |
| S_PUB32 地址（render-architecture 全集 101k） | `evidence/render-architecture/public-symbols.json`（demangled + va） |
| 你域的合并清单 | `evidence/country-core-deep/rhapsody/domains/<domain>.json`（name/form/line/va/file） |
| 已跑反编译 | `evidence/country-core-deep/rhapsody/domains/<domain>/decomp/decomp-<va>.txt` + `summary.json` |
| 游戏侧调用链调查（必读） | `evidence/country-core-deep/rhapsody/CALL-CHAIN.md`（30 入口 / 19 对上 / 11 缺） |
| rebuild 现状（对照基准） | `reconstruction/rhapsody-rebuild/src/**`（84 .rs）+ `MODULE_STATUS.md` + `ARCHITECTURE.md` |
| 游戏侧契约 stub | `reconstruction/country-core/src/{lib.rs(rhapsody_contract), rhapsody_types.rs, render/ssbo.rs, render/shader.rs, render/rg_helpers.rs}` |

## 2. 域划分（5 路并行）

| 域 | 骨架范围 | compiled | 地址 | 任务侧重 |
|---|---|---|---|---|
| `rg` | `src/rg/*.rs`（15 文件） | 107 | 57 | 图编译/执行/录制/时态/资源注册——**重建核心** |
| `raw-core` | `src/raw/{device,buffer,texture,swapchain,shader,command_buffer,barrier,submit,descriptor_set,binding,render_pass,framebuffer,dynamic_constants,debug,profiler,sampler,semaphore,stats,surface,temp_list}.rs` | 184 | 95 | Vulkan 资源/命令/屏障/管线——**rebuild 大部分已实现，重点核布局与 ABI** |
| `raw-init` | `src/raw/{init,instance,physical_device,format,allocator,crash,disposal,error,bytes,workarounds,ray_tracing}.rs` + `src/leek.rs` + `src/lib.rs` | 87 | 47 | 设备筛选/初始化/分配器/崩溃处理 |
| `managed` | `src/raw/managed/*.rs`（9 文件） | 42 | 12 | managed 生命周期/句柄/GpuOwned |
| `crates` | `crates/{shader-pipeline,gpu-profiler,vk-sync-rs,aftermath-rs,dxgi-enumerate-gpus}/src` | 46 | 28 | 独立子 crate（shader 反射/管线描述、GPU 分析、同步等） |

## 3. 工具

### 3.1 反编译（RE-MCP，预分析 116k 函数已在库）

```sh
# 你域的反编译（已跑或自行补跑；--lock 全局串行防内存崩溃）
python D:/game/reverse-engineering/tiny-glade/scripts/run-rhapsody-decomp.py <domain> <你的dbid>
```

- dbid 唯一（如 `tg_rhX`）。不与其他 agent 重复。
- 输出落盘 `domains/<domain>/decomp/decomp-<va>.txt`，首行是函数名注释。
- 单函数输出 1–100KB，**别整份读进上下文**：先看大小，大函数分段/grep。
- NotFound / 空（多为 Ghidra 未建函数的短函数）：用 rne-read 原字节手解（见 3.2），
  **NotFound 文件不能作为 O 证据**。

### 3.2 原字节/字符串直读（RNE，快）

```sh
PYTHONIOENCODING=utf-8 python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py --hex <va> <n>
PYTHONIOENCODING=utf-8 python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py --fat <va>     # &str {ptr,len}
PYTHONIOENCODING=utf-8 python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py --unks <decomp.txt>  # 所有 UNK_142 引用
```

### 3.3 单个 MCP 查询（xref / 补充反编译）

job 是 JSON 步骤数组；`node scripts/mcp-job.mjs re_mcp_ghidra job.json out.json`。
常用：`decompile_function`、`get_xrefs_to`、`open_database`(file_path=workspace/input/tiny-glade.exe, run_auto_analysis=false)、`wait_for_analysis`、`close_database`。

### 3.4 对照 rebuild（重点）

- rebuild 是**候选实现**（已过 lavapipe GPU 验证），你的任务不是重写它，而是**把原版行为/布局逐函数核清**，
  判定 rebuild 哪些对齐（✓）/偏差（半）/缺失（✗），供后续接线。
- rebuild 结构与骨架同名文件对应（`rg/`、`raw/`、`raw/managed/`、`shader_pipeline/`）。
- 已知偏差（CALL-CHAIN §4 已列）：rebuild 的 `Buffer` 16B desc+句柄 vs 原游戏面 40B 整块；
  temporal 键 &str vs 原 `ustr::Ustr`；`ManagedHandle` 非泛型 vs 原泛型；`RenderPassDesc` 键字段缺；
  `GraphicsDevice` 真实结构 vs 合约 u64 占位；回调签名 `FnMut(PassContext)->bool` vs 原 `FnOnce(u64)->bool`。

## 4. 挖掘纪律（沿用 country-core 规则）

- **标注**：**O**=机器码/PDB 实测；**I**=推断；**U**=未知。每行结论都带标注。
- **ICF 折叠区以 S_PUB32 为权威**：Ghidra 给的裸 ret/身份函数、CU 表"唯一符号"都不可信；
  以 `evidence/render-architecture/public-symbols.json` 的 demangled 名为准。
- **游戏 Vec = cap-first** `{cap@0, ptr@8, len@0x10}`（与 std 不同）——凡推断出 Vec 布局先验证。
- 枚举 niche / Option 哨兵（如 i64::MIN）注意。
- PDB 名可能与行为错位——**机器码为准**。
- 每函数结论写 **role/verdict/evidence**（在哪段反编译/哪个常量表/哪次 rne-read 证实）。
- 不编造 Rust 类型：未证实的字段标 U，不猜字段名。

## 5. 产出（每域一份）

在 `evidence/country-core-deep/rhapsody/domains/<domain>/ALGORITHMS.md` 写：

1. **模块总览**：文件→职责、主要数据结构（类型名 + 布局 O/I 标注）、处理链（函数调用图）。
2. **逐函数表**：`函数名（骨架名） | 地址（有则列） | role | 算法/行为要点 | verdict(O/I/U) | evidence`
   —— compiled_procedure 全覆盖；inline 里角色重要的（如 PassBuilder::render 闭包、draw 命令闭包）也要解。
3. **rebuild 对照表**：原版函数 ↔ rebuild 对应实现，标 ✓/半/✗ + 差异说明。
4. **U 卡点清单**：哪些函数/字段/常量还没解清，写明卡在哪儿。

写完在 `evidence/country-core-deep/round3/MODULE-DEEP-DIG.md` 追加一行登记（agent id、结果摘要）。
不要改 reconstruction 源码、不要动别人的域文件。
