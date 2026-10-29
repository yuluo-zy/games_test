# rhapsody 迁移基线(本机,Linux)

这是 [METAL_EXECUTION_PLAN.md](../METAL_EXECUTION_PLAN.md) 的 WP0:拆分与迁移开始之前的"之前"。之后每一步都用
`tools/rhapsody_test_baseline.py compare` 与它比较。

## 记录条件

| 项 | 值 |
| --- | --- |
| 提交 | `f82cd21036c1cf394cb5895355fb91fedc8b079b`(`crates/rhapsody/src` 自 `d4b49171` 以来未变;录制时 `git status` 对该目录无改动) |
| 命令 | `python3 tools/rhapsody_test_baseline.py record --out crates/rhapsody/baseline/test_results.tsv`(即 `cargo test -p rhapsody --all-features --offline --no-fail-fast`) |
| Vulkan | `VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`(lavapipe,软件实现)。本机同时装有其他 ICD,不强制就会用到别的驱动 |
| 工具链 | rustc 1.96.0;Linux 6.8.0-146-generic;Python 3.12.3 |
| 校验层 | `VK_LAYER_KHRONOS_validation` **不可用** |

## 结果

`crates/rhapsody/baseline/test_results.tsv`:957 个测试,**943 ok、2 FAILED、12 ignored**,共 95 个测试二进制(含 1 个文档测试)。

两次独立录制(一次解析日志,一次实时运行)逐项一致,所以这份基线是稳定的。

### 已知失败(改动之前就存在,不是回归)

| 测试 | 二进制 | 原因 |
| --- | --- | --- |
| `attachment_cache_separates_view_usage_and_rejects_sampling_only_color_and_depth` | `texture_view_usage` | `Unsupported("请求layer不可用: VK_LAYER_KHRONOS_validation")` |
| `native_packed_radiance_sampling_keeps_real_r32_storage_and_color_owners` | `texture_view_usage` | 同上 |

两个都只因为本机没有 Vulkan 校验层。装了校验层的机器上它们应该通过;那是另一份基线,不能与这份混用。

## 录制时踩过的坑(工具已修复,留作记录)

- `cargo test` 默认在第一个失败的测试二进制处停止,之后的二进制不会运行。第一次不带 `--no-fail-fast` 录到的只有 881 个测试,是**不完整的基线**。工具现在总是带这个选项。
- `cargo` 把 `Running <二进制>` 打在标准错误、把测试结果打在标准输出。分开读取再拼接会让测试被归到错误的二进制名下。工具现在合并成一个流。这个缺陷是靠"重复录制一次再比较"发现的。

## 这份基线没有覆盖的

- `country-core`、`tiny-glade` 的测试:本机不能编译 tiny-glade;country-core 只能用 `--target x86_64-pc-windows-gnu` 做类型检查,不能运行。它们的基线要在 Windows 实机上另录。
- 性能。基线只记录功能结果,性能门槛见 METAL_DESIGN.md 第 0.5 节,需要 Windows 实机。
- 全工作区检查:`cargo check --workspace --offline` 在本机失败(依赖 `ddsfile v0.5.1` 不在本地仓库,离线无法下载)。能在本机检查的包见计划第 1.3 节。

## 其他随基线保存的文件

- `deps_report.txt`:`tools/rhapsody_core_split_deps.py` 在同一提交上的输出(P1 每一步之后重跑并比较)。
- 测试影响统计见 `tools/metal_experiments/results/x3_test_impact.md`。
