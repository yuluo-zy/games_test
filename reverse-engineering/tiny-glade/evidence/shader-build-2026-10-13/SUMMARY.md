# 实际 shader 构建、预处理/打包与来源交付

本轮交付可调用的构建链：恢复的 HLSL/include → 原 shader-prepper 展开 include/pragma once → 宏与逻辑 #line → 实际 DXC 原参数 → 精确 SpirQ fork 独立反射 → SPIR-V 后处理 → 原 Speedy `.bin` schema。现有 rhapsody 工程直接使用这些模块，没有另建重写公共依赖的替代包。

404 个新包、638 个阶段已全部生成。393 包使用 Microsoft 官方 DXC 1.9.2609；另 11 个在 1.9 发生实际编译器异常的 compute 包使用官方 **preview 1.10.2605.37** 成功。原部署版本或 fork 仍未知，两种实际编译器身份及先前失败保留在逐项报告，不能把它们认作原版编译器。

| 验证 | 实际结果 | 范围 |
|---|---|---|
| 新源码构建与容器重读 | 404 包/638 阶段成功 | 使用实际恢复源码与当前配置；无原 metadata 注入 |
| 原 SPIR-V 独立再反射 | 404 包完整反射 canonical 字段零差异 | 验证反射转换/schema；不是源码重编译 |
| 新编译完整反射 | 210/404 包与原一致 | 194 包有差异，逐字段保留 |
| 新编译阶段公共接口 | 302/638 与原一致 | 完整公共反射比较；不证明算法等价 |
| capability / 非 debug opcode histogram | 617/638、100/638 与原一致 | 即使一致也不证明指令语义一致 |
| 当前 include 依赖集合 | 213/404 一致，191 不同 | 依赖由真实 source provider 读取生成 |
| 独立产物审查 | 1446 项 SHA256/长度零差异 | 404 prepared、638 raw SPIR-V、404 fresh.bin，两位 reviewer 独立核查 |
| 最终受影响测试 | 10/10；严格 all-targets Clippy 通过 | 预处理/反射/打包、原请求、实际 DXC、wire 顺序 |
| 新包 GPU 消费 | Intel UHD 630 fill 64 word 均为 `0xc001d00d`；egui 两布局成功 | compute 实际执行；raster 仅 layout，没有 draw/整帧 |

完整原宏 manifest 尚未恢复：170 个 compute 包的原宏 tuple 均未独立证实，其中 38 个非零 define key 明确未知。当前 driver 的空宏配置是诊断基线，不冒充未知原变体；132 个零 key 只与空宏兼容，也不能仅靠 XOR hash=0 证明原 tuple 为空。234 个 raster 包已有 `SHADER_VARIANT_DEPTH_ONLY=0/1` 原证据。每项新包都记录实际配置、原 key 与当前 key，未知变体不会冒充原文件名部署。

原 SPIR-V 再生成 metadata 的 wire audit 为 27 包字节完全相同、377 包仅 map 记录顺序不同，全部 typed 业务字段与长度相同。新 producer 对 map 确定排序，原 `HashMap` 迭代顺序不作业务目标；这一结果不等于新编译包字节完全相同。

来源结论：

| 模块 | 已证实身份 | 当前处理 |
|---|---|---|
| shader-prepper 0.3.0-pre.3 | 作者 Tomasz Stachowiak/h3r2tic 原生公开 MIT 库；原 checksum 匹配；7 个源码文件与发布 commit 经换行归一化一致 | 直接复用，恢复游戏 provider 和连接规则。它扫描 include，完整宏/条件预处理仍由 DXC 做。[固定发布源码](https://github.com/h3r2tic/shader-prepper/tree/845491a0e4c90847b2197972d743a02202219f25) |
| SpirQ 0.6.2 | 作者对 PENGUINLIONG 库的公开 fork；原锁明确固定 `4f22e6f…`，修复 OpUndef 导致提前结束扫描 | 精确复用原 commit。[原 fork 补丁](https://github.com/h3r2tic/spirq-rs/commit/4f22e6f149750401705d47ae15a60e77cd5a6905) |
| DXC | Microsoft 公共 LLVM/Clang 派生编译器；作者公开存在 storage capability 与 RawBufferLoad2 补丁；原游戏是否使用这些分支未证实 | 复用官方工具并保留版本差异。当前官方版本缺 RawBufferLoad2；目前恢复源码未使用该 API，不能推断原作不依赖它。[官方项目](https://github.com/microsoft/DirectXShaderCompiler)，[作者 capability 补丁](https://github.com/h3r2tic/DirectXShaderCompiler/commit/fd2e4ec9fb2c1b9b2e58a35de759fd6068419fb1)，[作者 RawBufferLoad2 补丁](https://github.com/h3r2tic/DirectXShaderCompiler/commit/16ceb560954f2238dcdad73a75d5e5842e58c839) |
| rspirv、speedy、relative-path 等 | 原锁确定的公共依赖，版本/source/checksum 匹配 | 直接复用公共实现，仅恢复调用和数据结构规则 |
| shader-compiler、shader-pipeline、shader-variants、asset-pipe、asset-pipe-bin | 原 workspace 的 local 工具包；Cargo/PDB 证明包/源码归属，但公开上游、完整作者文本与许可尚未核实 | 恢复项目配置、schema、命名与组合策略；不能仅因 local 或检索无结果判为全部从零自研 |

完整来源证据在 [PROVENANCE.md](provenance/PROVENANCE.md)，包含 exact Cargo/PDB 条目、公开 fork 源码、作者 public 仓库检索范围及 A/B/C 边界。C 已迁往 shabriri/shaderface，不能据此假定 B 同源。本轮针对 B；未修改原发行文件。

构建 API 是 [`build_pipeline`](../../reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs)：输入 `BuildSpec` 的 source root、逻辑路径、宏及阶段/入口，返回实际 dependencies、prepared source、fresh stages、完整 reflection 和 encoded bytes。生产者没有原 reflection 参数。源码职责在 [pipeline/CONTRACT.md](pipeline/CONTRACT.md)。原 provider 精确路径/错误、缓存、临时文件/取消/清理策略及完整 asset-pipe-bin CLI 没有原实现证据，当前适配明确标记为候选；资产构建入口在此交付的是 shader 构建 API 和实跑批量 audit CLI。

从 `D:\game` 验证当前交付：

```powershell
cargo test --locked --manifest-path reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/Cargo.toml --test shader_build_pipeline --test shader_compiler_contract --test shader_dxc_recompile --test shader_rebuild_consumer -- --nocapture
cargo clippy --locked --manifest-path reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/Cargo.toml --all-targets -- -D warnings
cargo run --locked --manifest-path reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/Cargo.toml --example shader_rebuild_consumer_gpu -- reverse-engineering/tiny-glade/evidence/shader-build-2026-10-13/pipeline/fresh-fill.bin reverse-engineering/tiny-glade/evidence/shader-build-2026-10-13/pipeline/fresh-egui-color.bin reverse-engineering/tiny-glade/evidence/shader-build-2026-10-13/pipeline/fresh-egui-depth.bin
```

批量实际重编译可使用 `shader_recompile_audit`，参数为 DXC 路径、**新的**输出目录和可选 filename prefix/旧 report.json（选择其中失败项）。它依赖原资产作为独立 oracle，但产物不注入原 metadata；计算宏未知项使用明确的诊断配置。以下命令保留本轮目录，使用新的 `local-recheck-*` 名称；重复执行须再次换目录。

```powershell
cargo run --locked --manifest-path reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/Cargo.toml --example shader_recompile_audit -- D:/game/tools/shader-toolchain/dxc-v1.9.2609/bin/x64/dxc.exe D:/game/reverse-engineering/tiny-glade/evidence/shader-build-2026-10-13/dxc/local-recheck-19
cargo run --locked --manifest-path reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/Cargo.toml --example shader_recompile_audit -- D:/game/tools/shader-toolchain/dxc-v1.10.2605.37/bin/x64/dxc.exe D:/game/reverse-engineering/tiny-glade/evidence/shader-build-2026-10-13/dxc/local-recheck-110 D:/game/reverse-engineering/tiny-glade/evidence/shader-build-2026-10-13/dxc/local-recheck-19/report.json
```

最后两条是复跑方法；本轮实际数据来自 [v2 全量](dxc/corpus-final-v2/report.json)、[v3 源码修复复验](dxc/failed-recheck-v3/report.json)、[v4 官方另一版本复验](dxc/av-recheck-v4/report.json)，按文件唯一合并在 [merged-audit.json](dxc/merged-audit.json)。这三轮报告不改写，源码修复的 before 快照保留。

最终日志：[tests-current.log](tests-current.log)、[clippy-current.log](clippy-current.log)、[fresh-consumer-gpu-current.log](fresh-consumer-gpu-current.log)。集成与当前输入绑定：[integration.json](integration.json)。互审：[compiler → pipeline](dxc/review-pipeline.md)、[pipeline → compiler](pipeline/review-compiler.md)、[独立 producer/consumer](provenance/pipeline-producer-review.json)。

构建链可用；完整原 shader 算法/宏配置与工具副作用恢复仍有缺口。资产库状态机、游戏 pass 接线和整帧验收属于后续范围。本轮没有将整个 crate、完整资产 CLI 或游戏标记为完成。
