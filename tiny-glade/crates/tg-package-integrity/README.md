# tg-package-integrity

`tg-package-integrity` 校验发行包清单及文件内容，帮助启动和诊断工具识别缺失、损坏和额外文件。调用方显式传入包根目录，模块不会修改进程工作目录或写入发行包。

## 数据模型与公开接口

| 接口 | 职责 |
| --- | --- |
| `MANIFEST_PATH` | 清单固定位置 `build-info/manifest.json` |
| `PackageManifest` | JSON 数据模型：`version: u32` 与有序 `files: Vec<(String, String)>` |
| `FileIssues` | 按 `missing`、`corrupt`、`extra` 分类保存相对路径 |
| `verify_package_manifest(root)` | 读取根目录中的清单，执行完整校验 |
| `verify_manifest_bytes(root, bytes)` | 校验调用方提供的原始清单字节与目录中的文件；不会把字节写入目录，列出的文件仍需存在 |
| `is_valid_manifest_checksum(bytes)` | 仅检查清单原始字节的格式校验门 |
| `find_extra_files(root, known)` | 返回 `io::Result<Vec<String>>`，递归扫描未列入 `known` 的文件 |
| `PackageVerificationError` | 区分清单读取、checksum、JSON、版本、条目、文件和目录扫描错误 |

JSON 清单的常规形态如下；示例中的零摘要只用于展示字段结构：

```json
{
  "version": 1,
  "files": [
    ["assets/world.bin", "0000000000000000000000000000000000000000000000000000000000000000"],
    ["build-info/manifest.json", "ignored"]
  ]
}
```

只支持版本 `1`。未知 JSON 字段会被忽略；Serde 也接受等价的结构体序列表示。每个普通文件必须提供 64 位小写十六进制 BLAKE3 摘要。清单自身仅按精确名称 `build-info/manifest.json` 免除文件内容摘要比较，其摘要字符串不参与验证；列出自身时，磁盘上的对应路径仍必须是可打开的普通文件。文件扫描不会自动把清单加入已知集合，因此从磁盘校验时通常需要显式列出清单自身。

## 业务流程

1. `verify_package_manifest()` 确认清单路径各层没有符号链接，读取普通文件。
2. 对完整原始字节计算 BLAKE3，再向固定种子的 FNV-1a 依次输入 8 字节小端整数 `32` 与 32 字节摘要，要求结果低 16 位为零。这个长度编码不随机器字长或字节序变化。空白和字段顺序都会影响结果。此门用于格式一致性检查，不是签名或来源认证。
3. 反序列化 JSON，检查版本，然后验证条目的路径、唯一性和摘要格式。
4. 整轮复用同一组 worker，任务数不超过 `min(可用 CPU 数, 64, 条目数)`（空清单直接由当前线程完成）。worker 动态领取文件索引，通过 reader 流式计算 BLAKE3；调用线程也参与处理；如果系统无法创建更多线程，由已有 worker 与调用线程处理剩余任务。完成结果按原索引归位，不把整文件载入内存。
5. 通过待扫描路径栈遍历根目录，先枚举完当前目录、关闭其目录句柄，再进入下一目录。确认每个普通文件都在清单中，汇总三类文件问题；全部通过时返回 `Ok(())`。

`missing` 包含无法打开或完整读取的条目，`corrupt` 包含成功读取但摘要不同的条目。两者保持清单顺序。`extra` 按相对路径排序；空目录不会作为额外文件报告。

## 路径与文件系统约定

清单路径使用 `/` 分隔的 UTF-8 相对名称。拒绝绝对路径、空分段、`.`、`..`、反斜杠、控制字符、Windows 路径特殊字符、末尾空格或句点，以及 Windows 设备名称。条目去重使用 `NFC → Unicode 小写 → Unicode 大写 → NFC` 生成比较键，拒绝重复、大小写及规范化别名，包括 `Ä/ä`、`ẞ/ß`、`σ/ς` 等。这个保守规则用于阻止常见跨平台冲突，不等同于所有文件系统的身份判断；清单仍应使用稳定且唯一的规范名称。

读取清单和条目时逐层检查路径：中间层必须是目录，最终层必须是普通文件；不能通过符号链接读取根目录外的文件，也不会打开设备或 FIFO。调用方可以把一个已选定的根目录别名传入模块；根目录本身不要求排除符号链接。

扫描忽略根目录下名称精确等于 `tmp` 的文件或整棵目录。`Tmp`、`sub/tmp`、`file.tmp` 均不属于该例外；清单中显式列出的 `tmp` 内容仍需要校验。其他位置的符号链接、特殊文件、无法表达为 UTF-8 的名称，以及目录枚举失败均返回错误，避免把不完整扫描当成校验成功。已知集合按精确的 `/` 相对路径匹配。

文件校验的线程创建数随 worker 数增长，不随文件总数增长；目录扫描同时最多保留一个目录枚举句柄，不按目录深度累积句柄或调用栈。辅助内存包括每个 worker 的流式读取缓冲、每条目的紧凑校验结果和待扫描路径。

调用方需要提供校验期间保持稳定的文件树。模块使用常规文件系统 API，未提供针对并发替换路径的原子快照或操作系统级句柄隔离。

## 错误语义

| 错误 | 含义 |
| --- | --- |
| `ManifestRead(String)` | 清单读取失败，或清单路径包含链接、特殊文件、错误层级 |
| `InvalidManifestChecksum` | 原始清单字节未通过格式校验门 |
| `ManifestParse(String)` | JSON 无法反序列化为清单模型 |
| `UnsupportedVersion(u32)` | 清单版本不受支持 |
| `InvalidManifestPath(String)` | 条目路径不是允许的包内相对路径 |
| `InvalidManifestEntry { path, reason }` | 路径重复或摘要格式无效 |
| `Files(FileIssues)` | 文件缺失、内容不符或存在额外文件 |
| `Filesystem(String)` | 目录扫描不完整，或出现不允许的文件类型/名称 |

`PackageVerificationError` 实现 `Display` 和 `std::error::Error`，可直接用于 `?` 错误传播与诊断输出。校验按上述处理阶段执行，前一阶段失败时不继续。目录扫描失败时优先返回 `Filesystem`，不把已经收集到的部分文件问题当成完整结果。`verify_manifest_bytes()` 仍要求根目录可扫描；空清单配合不存在的根目录也会失败。

## 使用示例

```rust,no_run
use tg_package_integrity::{PackageVerificationError, verify_package_manifest};

match verify_package_manifest("/path/to/package") {
    Ok(()) => println!("package verified"),
    Err(PackageVerificationError::Files(issues)) => {
        eprintln!("missing: {:?}", issues.missing);
        eprintln!("corrupt: {:?}", issues.corrupt);
        eprintln!("extra: {:?}", issues.extra);
    }
    Err(error) => eprintln!("verification failed: {error}"),
}
```

只需要查找额外文件时，可以独立使用扫描接口：

```rust,no_run
use std::collections::HashSet;
use tg_package_integrity::find_extra_files;

let known: HashSet<String> = ["build-info/manifest.json".into()].into_iter().collect();
let extra = find_extra_files("/path/to/package", &known)?;
println!("{extra:?}");
# Ok::<(), std::io::Error>(())
```

## 诊断与验证

在工作区根目录执行：

```sh
cargo test -p tg-package-integrity --all-targets
cargo test -p tg-package-integrity --doc
cargo clippy -p tg-package-integrity --all-targets -- -D warnings
cargo run -p tg-package-integrity --example inspect_package -- /path/to/package
```

诊断示例输出 JSON，成功退出码为 `0`，校验失败为 `1`，参数错误为 `2`。它先执行库的完整校验，不会提前解析未通过 checksum 的字节，也不会在校验失败后再次读取潜在的不安全路径。

测试覆盖 checksum/JSON/版本处理顺序、完整成功与损坏文件、多文件结果顺序、额外文件规则、非法路径/条目、清单自身缺失或类型错误、串行与并行结果一致性、不完整扫描和符号链接；Unix 子进程在最多 64 个文件描述符的限制下扫描 100 层目录，覆盖深目录的资源边界。非 UTF-8 文件名回归仅在 Linux 编译执行；其他平台还需要各自的文件系统验证。

模块只负责包内校验，启动流程、崩溃报告接线及包发布生成由调用方负责。历史来源见 [foundation-audit.json](../../provenance/foundation-audit.json) 与 [原始调查报告](../../../reverse-engineering/tiny-glade/evidence/small-libs-batch-02-2026-10-13/tg-package-integrity/BUSINESS_RULES.md)。历史 Windows 别名和静默扫描 fixture 保存用于调查参考，不作为保留错误行为的要求。

本轮源码复审、工程化改造及验证边界见 [复审报告](../../reports/building/2026-10-18-building-foundation-review/README.md)。
