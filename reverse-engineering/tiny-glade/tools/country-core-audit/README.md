# country-core 审计工具

直接对照**原版 PDB + EXE（版本 B，sha256 `cf048e27…`）**核对 country-core 重建工作的工具集。
纯 Python + `llvm-objdump`（rustup 自带即可），不依赖 Ghidra / MCP，在 Linux 上几秒内可跑完。
审计结论见 `evidence/country-core-audit/AUDIT_REPORT.md`。

## 前置条件

- `tiny-glade.exe`、`tiny_glade.pdb` 在仓库根目录（或用环境变量指定，见 `cca.py` 顶部说明）
- `llvm-objdump`：自动查找 `~/.rustup/toolchains/*/lib/rustlib/*/bin/llvm-objdump`，否则用 `CCA_OBJDUMP`
- 脚本会解析不可信的二进制文件，统一用 `python3 -I` 运行（脚本自己把本目录加入搜索路径）

## 用法（在本目录下执行）

| 步骤 | 命令 | 作用 | 产物（`evidence/country-core-audit/`） |
|---|---|---|---|
| 1 | `python3 -I pdbprocs.py` | 解析 PDB，导出 85k 个过程（地址/大小/全名） | `data/procs.tsv`（54MB，已 gitignore，勿提交） |
| 2 | `python3 -I dis.py <0x地址\|函数名>` | 反汇编单个函数，标注 call 目标名、f32/f64/vec4 常量、字符串 | 屏幕 |
| 3 | `python3 -I addrcheck.py` | 核对文档/代码里引用的地址是否落在真实函数上、名字是否一致 | `data/addr_issues.json` |
| 4 | `python3 -I fnstat.py` | country-core Rust 函数体分类（stub/trivial/partial/real） | 屏幕 |
| 5 | `python3 -I modstat.py` | 按功能模块统计业务逻辑挖掘度（分母=原版机器码字节） | `module_stats.tsv` |
| 6 | `python3 -I calls.py` | 抽取渲染范围内全部函数的直接调用目标 | `data/scope_calls.json` |
| 7 | `python3 -I layers.py` | 渲染依赖分层（剔除 trait 分派边，求 SCC，最长路径分层） | `data/render_layers.json` |

步骤 2–7 依赖步骤 1；步骤 7 依赖步骤 6。

## 核对合同结论的方法

1. 在合同里找标为 **O（Observed）** 的具体结论（常量、分支条件、偏移、调用顺序）。
2. `python3 -I dis.py <地址>`，看反汇编里的常量、比较方向、调用目标是否一致。
3. 注意 Ghidra 输出的两个常见陷阱（本次审计中出错最多的地方）：
   - 偏移是 **dword 单位**，不是字节；
   - 反编译里 `0x14xxxxxxx` 的相同机器码可能被链接器**合并**，PDB 名字对应多个函数，不能直接信名字。

## 局限

- `calls.py` 只抽直接调用；vtable/`dyn` 间接调用点只计数不解析。
- `layers.py` 的组件归属按模块路径规则划分，边界处可能有个别争议；规则在文件顶部 `RULES`。
- `modstat.py` 的“扎实”只检查代码体量与原函数大小是否相称，**不保证逻辑正确**。
- 地址到 Rust 函数的映射依赖函数文档注释或函数体里写了地址；没写地址的实现不会被计入（结果偏保守）。

## l0-harness（第 0 层隔离测试台）

`l0-harness/` 用 `#[path]` 把 `reconstruction/country-core/src/render/` 里第 0 层的文件原样挂进一个小 crate
（依赖 glam / serde / log / anyhow，全部来自本机 cargo 缓存；`parking_lot` 用 `parking_lot_shim/` 占位，仅含 `RwLock`），
可离线编译并运行单测（72 个）：`cd l0-harness && CARGO_TARGET_DIR=/tmp/l0h-target cargo test --offline`。
新增第 0 层文件时，在 `l0-harness/src/render/mod.rs` 里加一行 `#[path]` 即可（要挂到 `render::pass_builder::` 下的文件放在该文件里的内联
`pass_builder` 模块中，并保留 `src/render/pass_builder/` 目录，否则 `..` 路径解析会失败）。需要先 `export PATH=$HOME/.cargo/bin:$PATH`。进度与证据见
`evidence/country-core-audit/L0_NOTES.md`。

## rdva.py

`python3 -I rdva.py 0x142ac7018 …`：按虚拟地址读原 EXE 的字节，并把 `&[&str]` 形式的 `fmt::Arguments` 片段表解出来。
反汇编里 `lea rcx, [rip+…] ; str=b'…'` 只显示前几十字节，panic 文案、日志模板、查表常量靠它读全
（例：`incorrect attribute type`、`Creating pipeline `、`Error opening : `、TGSR 比例表）。也可 `from rdva import rd, q` 当库用。

## 统一测试台（l0-harness）

`l0-harness/` 把渲染相关的真实源文件用 `#[path="@SRC@/..."]` 原样挂进来编译（`@SRC@` 由 `run.sh` 替换成真实路径），
外部库只给最小替身。运行：`l0-harness/run.sh --lib`（`L0H_REAL` 指定源码目录、`L0H_WORK` 指定工作目录、
`CARGO_TARGET_DIR` 默认 `/tmp/l0h-target`）。B 阶段新增工具：`typeid_resolve.py`、`cb_trace.py`、`dsh_*.py`、`dg/`。
