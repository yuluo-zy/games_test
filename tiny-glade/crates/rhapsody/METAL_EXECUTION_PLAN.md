# Metal 集成:当前可执行的工作计划

本文把 [METAL_DESIGN.md](METAL_DESIGN.md) 中**现在就能开始、不用等 Apple 设备实测**的工作落成可执行的计划:工作包、顺序、依赖、验收、停止条件和进度表。设计依据在 METAL_DESIGN.md 与 [CORE_SPLIT_DESIGN.md](CORE_SPLIT_DESIGN.md);本文只管"做什么、按什么顺序、怎么算做完、什么时候停"。与它们冲突时,以设计文档为准,并回头修正本文。

计划里的所有工作量只用"小/中/大"表示,没有人天估算:当前没有数据支撑精确估算。

## 1. 规则

### 1.1 合入门槛

1. **性能基线先于合入**:METAL_DESIGN.md 第 0.5 节要求,任何可能影响性能的改动合入前,先有 Windows 实机的基线(4 场景 × 3 轮)。没有基线时,工作可以在分支上做,**不合入主线**。
2. **测试规则**:允许的测试修改只有两类——改导入路径;把对 Vulkan 类型的断言改成对中立类型的断言,且断言含义不变。其他测试修改一律先停下报告。
3. **一步一验**:每个工作包拆成若干小步;每步结束都编译、跑第 1.3 节的检查,通过后才进入下一步。

### 1.2 停止条件(出现任何一条就停下来告诉用户,不自行绕过)

| # | 条件 |
| --- | --- |
| H1 | 任何一个已有测试需要改动,而且不属于第 1.1 节允许的两类 |
| H2 | 某一步之后,现有测试的通过/失败集合与第 3 节记录的基线不同 |
| H3 | 需要引入或运行本仓库之外的第三方代码(克隆、下载、安装) |
| H4 | 某个设计假设被代码证伪(例如发现 K1 别名在真实 `ash` 类型上编译不通) |
| H5 | 一个工作包的实际改动面超过其"范围"一栏的描述(例如 S1 需要碰设计里没列的模块) |
| H6 | 需要在已推送的提交上改写历史,或强制推送 |

### 1.3 每步通用检查

```bash
export PATH=$HOME/.cargo/bin:$PATH
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json   # 必须强制:本机还装有其他 Vulkan 驱动
cargo check -p rhapsody -p shader-compiler -p shader-pipeline --offline
CC_x86_64_pc_windows_gnu=gcc AR_x86_64_pc_windows_gnu=ar \
  cargo check -p country-core --target x86_64-pc-windows-gnu --offline     # 仅类型检查,不能运行
python3 tools/rhapsody_test_baseline.py record --out /tmp/current.tsv       # = cargo test --no-fail-fast
python3 tools/rhapsody_test_baseline.py compare crates/rhapsody/baseline/test_results.tsv /tmp/current.tsv
python3 tools/rhapsody_core_split_deps.py                                  # 依赖统计,迁移步骤要重跑并与基线比较
```

`compare` 的退出状态 1 表示有回归、有测试消失、或有 ok/ignored 互换,任何一种都触发停止条件 H2。**不要**
用 `cargo check --workspace`:本机离线时它会因 `ddsfile v0.5.1` 不在本地仓库而失败,这不是代码问题。
`cargo test` 不带 `--no-fail-fast` 会在第一个失败的测试二进制处停下,后面的测试不会运行,所以只能用上面的
`record`。

tiny-glade 在本机无法编译(GTK 依赖、`bzip2-sys` 需要 `windows.h`),它的检查在 Windows 实机上做。凡是需要 tiny-glade 才能验证的验收项,本计划都标为"Windows 待验"。

## 2. 工作包

状态列见第 6 节。"验证"一栏只写**在本机能验证到的程度**。

### WP0 基线记录(其他工作包的前提)

| 项 | 内容 |
| --- | --- |
| 目的 | 让"测试结果不变"有可比较的"之前",并且比较由工具完成 |
| 状态 | **本机部分已完成**(见第 6 节)。Windows 部分(country-core、tiny-glade 的测试,性能)仍待做 |
| 已做 | `tools/rhapsody_test_baseline.py`(录制与比较,有单元测试);`crates/rhapsody/baseline/`:`test_results.tsv`(957 个测试:943 ok、2 FAILED、12 ignored)、`deps_report.txt`、`BASELINE.md`(记录条件、已知失败及原因、录制时踩过的两个坑) |
| 已知失败 | `texture_view_usage` 里 2 个测试,原因是本机没有 Vulkan 校验层;它们是基线的一部分,不算回归 |
| 验收(已达成) | 两次独立录制逐项一致 |
| 仍待 | 在 Windows 实机上为 country-core 和 tiny-glade 录制同样格式的基线;P0 的性能基线 |

### WP1 P1 的 S0:重跑依赖统计

| 项 | 内容 |
| --- | --- |
| 目的 | 确认代码现状与 CORE_SPLIT_DESIGN.md 的数字一致(设计之后代码可能又变过) |
| 做什么 | 运行 `tools/rhapsody_core_split_deps.py` 与 `tools/metal_experiments/x3_test_impact.py`;与设计文档里的数字比对 |
| 验收 | 偏差写进 CORE_SPLIT_DESIGN.md;偏差使某个设计假设失效则触发 H4 |
| 规模 | 小 |

### WP2 P1 的 S1:`prism-gfx` 基础层

| 项 | 内容 |
| --- | --- |
| 前置 | WP0、WP1 |
| 范围 | 新建 `crates/prism-gfx`;特性 `native-types`(依赖可选 `ash`)与 `test-support`;`native_types.rs`;移入 `log_ring`、`profile`、`error`、`sync`(含 `timeline`)、`accounting`、`collections`(含从 `native/submit/mod.rs:12-63` 移来的 `SubmitScratch`)、`retire`;门面按 CORE_SPLIT_DESIGN.md K7 重新导出 |
| 小步(第 2 次修订,见下) | ① 建 crate 与 `native_types.rs` 骨架,门面只加依赖;② 移 `trace`、`log_ring`、`profile`(`profile` 的两处测试移到 rhapsody 侧);③ 移 `error`(K2);④ 移 `sync`(K3,含 `impl TimelineApi for ash::Device`);⑤ 移 `accounting`;⑥ 移 `collections`、`retire` |
| 对原计划的修正 | **顺序**:原计划先移 `collections`、`retire`,编译不过。它们依赖 `accounting` 和 `sync`,后者依赖 `error`,所以顺序必须是 `error → sync → accounting → collections/retire`(`error` 在 `sync` 之前,`trace` 在 `sync` 之前)。**范围**:`trace`(85 行,无依赖)要一起移,`sync/timeline` 依赖它(`crate::trace::SubmitTrace`);`SubmitScratch` 实际只被 `tracking` 使用,移到 S2,不在本包。**测试**:core 里直接用 `ash::vk` 的测试模块要加 `cfg(feature = "native-types")`,否则关闭特性时 `cargo test` 编译不过;这仍满足 K1 规则 2(两种配置下非测试代码都能编译) |
| 验证(本机) | 通用检查(第 1.3 节),其中 `compare` 必须对 `crates/rhapsody/baseline/test_results.tsv` 无回归、无消失;core 在 `--features native-types` 与默认两种配置下都 `cargo check` 通过;K1 检查:core 内 `ash` 只出现在 `native_types.rs`;别名数值测试(开启特性时中立常量与 `ash` 常量逐个相等) |
| 允许的测试改动 | `profile` 中引用 `crate::rg` 的两处测试(`record_trace_tests.rs:125`、`command_workload/tests.rs:26`)移到 rhapsody 侧 |
| Windows 待验 | tiny-glade 编译;性能门槛 |
| 规模 | 中 |

### WP3 P3 的前置分析:手工同步归类(只读)

| 项 | 内容 |
| --- | --- |
| 目的 | P3 把宿主层 47 处手工同步迁入图声明或传输接口之前,需要知道每一处在做什么 |
| 做什么 | 读 `render_host.rs`、`screenshot_capture.rs`、`raster_host.rs` 中的布局转换、屏障、拷贝调用,逐处记录:位置、用途(上传、截图读回、图像初始化、其他)、能否由图声明取代、取代所需的接口;同样处理 `country-core/src/rhapsody_contract.rs:53,65,77` 三个原生命令缓冲方法的全部调用方 |
| 产出 | `crates/rhapsody/P3_SYNC_INVENTORY.md`(表格);归不进三类的单独列出 |
| 验收 | 47 处全部有记录;每处标注"可迁移 / 需新接口 / 暂无法消除"及理由;不改任何源文件 |
| 规模 | 小 |

### WP4 P2 的同步计划快照机制

| 项 | 内容 |
| --- | --- |
| 前置 | WP0;可与 WP2 并行(不依赖移动文件) |
| 目的 | P2 中立化 `sync_plan` 之前,先建立"改动前后输出必须逐字段相等"的尺子 |
| 做什么 | 为现有 `sync_plan`/`plan_check` 测试场景的 `BoundTransition`、`BoundWait` 序列录制快照文件,加一个比对测试;录制只读现有输出,不改 `sync_plan` |
| 验证(本机) | 快照两次录制一致;故意改动一个字段,比对测试必须失败(证明尺子有效) |
| Windows 待验 | 两个生产帧(static、many_draws,各 164 个 pass)的快照要在 Windows 上录 |
| 规模 | 中 |

### WP5 country-core 的泄漏收口设计

| 项 | 内容 |
| --- | --- |
| 前置 | WP3 的 `rhapsody_contract.rs` 部分 |
| 做什么 | 在 WP3 的调用方清单基础上,为 `rhapsody_contract.rs` 的 7 个泄漏接口(命令缓冲 3 个、`vk::Buffer`、`vk::DeviceAddress`、显存预算、`from_raw`)和 `render_frame.rs` 的 `vk::Format` 调试输出,各给出替换方案与受影响的 country-core 测试清单 |
| 产出 | 并入 `P3_SYNC_INVENTORY.md`,或单独成节 |
| 验收 | 每个泄漏接口有"消除 / 收进后端 / 保留并说明理由"的结论;格式调试输出的文案保真要求有对应测试方案 |
| 规模 | 小 |

### WP6 设备能力与配置档的纯逻辑(D14、D15、D17)

| 项 | 内容 |
| --- | --- |
| 前置 | WP2(类型落脚点);设计上不依赖 Metal |
| 做什么 | ① `DeviceCaps` 的中立字段与档位计算(输入为能力数据,输出为档位);② 内存压力作为 `AdaptiveRenderState` 的新输入;③ 质量预设的选择点。默认行为不变 |
| 验证(本机) | 纯逻辑单元测试覆盖档位边界与状态机转换;现有 `AdaptiveRenderState` 测试不变 |
| 规模 | 中 |
| 说明 | 本包先写设计与接口,实现要等 WP2 结束 |

### WP7 ASTC 资产输出(D16)

| 项 | 内容 |
| --- | --- |
| 前置 | 用户授权引入 ASTC 编码器(H3)——**授权前不开始** |
| 做什么 | 资产管线增加 ASTC 输出路径 |
| 验证(本机) | 编码后解码的往返误差在阈值内;不改变现有 BC 路径的输出 |
| 规模 | 中 |

## 3. 顺序与依赖

```
WP0 ──┬─ WP1 ── WP2 ──┬─ WP6
      │               └─ (P1 的 S2–S5:见第 5 节)
      ├─ WP4
      └─ WP3 ── WP5

WP7 (独立,等授权)
```

建议顺序:**WP0 → WP1 → (WP2 ‖ WP3 ‖ WP4) → WP5 → WP6**。WP2、WP3、WP4 互不依赖,可以并行;如果只有一条工作线,按 WP2、WP3、WP4 的顺序做,因为 WP2 是后续所有迁移的落脚点。

## 4. 本计划之外(并说明为什么现在不做)

| 工作 | 阻塞 | 解除条件 |
| --- | --- | --- |
| P0 的 Windows 基线 | 需要 Windows 实机和冻结构建 | 在那台机器上运行 `tools/metal_experiments/p0_gpu_matrix.py`,CPU 基线另用 `tools/perf_headless_series.py` |
| 任何改动合入主线 | 第 1.1 节第 1 条 | 基线数据 |
| P4 后端 trait 定稿、P5/P6 Metal 后端 | trait 形状要等 X5、X1b 的编译结果 | 在 Mac 上完成 `tools/metal_experiments/MAC_CHECKLIST.md` 的步骤 1–3 |
| P7 共享路径优化(跳过空 pass、逐级联剔除) | 逐位一致和性能收益要在 Windows 实机验证 | Windows 基线 + 实机运行 |
| P8 移动配置档的具体数值 | 要等 X2、X4 实测 | MAC_CHECKLIST 步骤 5;X4 的前置条件 |
| P1 的 S2–S5 | 依赖 WP2 的结果,且 K4 要在真实代码上先验证 | WP2 完成后单独出分步计划 |

## 5. WP2 之后的下一批(预告,不在本计划内)

P1 的 S2(访问词汇、`tracking`)、S3(切开 `rg` 的两个污染源,K4)、S4(移入 `rg` 的中立部分)、S5(收尾)。K4(渲染图按后端族泛型)只在替身原型上验证过,真实代码上是最大的不确定项,所以放在 WP2 之后单独评审,不提前承诺。

## 6. 进度表

状态取值:未开始 / 进行中 / 完成 / 阻塞(写原因)。完成必须附验收证据的位置。

| 包 | 内容 | 状态 | 证据 / 阻塞原因 |
| --- | --- | --- | --- |
| WP0 | 基线记录 | 本机部分完成 | `crates/rhapsody/baseline/`(提交 f82cd210 上的 957 个测试);Windows 部分待做 |
| WP1 | S0 依赖统计复核 | 完成 | `baseline/deps_report.txt`:模块行数、`vk::`/`ash::` 数与 CORE_SPLIT_DESIGN.md 第 2 节逐项一致(error 198/17/1、sync 635/21/13、accounting 1515/7/2、tracking 2346/19/7、rg 27157/467/72);`crates/rhapsody/src`、`crates/country-core/src` 自 `d4b49171` 起无改动,没有设计假设被证伪 |
| WP2 | S1 `prism-gfx` 基础层 | 未开始 | — |
| WP3 | 手工同步归类(只读) | 未开始 | — |
| WP4 | 同步计划快照机制 | 未开始 | — |
| WP5 | country-core 泄漏收口设计 | 未开始 | — |
| WP6 | 设备能力与配置档逻辑 | 未开始 | — |
| WP7 | ASTC 资产输出 | 阻塞 | 等待授权引入 ASTC 编码器 |

## 7. 约定

- **分支**:从当前分支起点另开工作分支,例如 `codex/prism-gfx-split`;不改写已推送的历史,不强制推送(H6)。
- **提交**:按工作包的小步提交,提交信息写明所属工作包与步骤;不在一个提交里混入多个工作包。
- **推送**:只在用户要求时推送。
- **报告**:每个工作包结束,更新第 6 节的状态与证据,并列出:改动的文件、测试集合与基线的对比、遇到的任何停止条件。
- **设计变更**:执行中发现设计需要改,先改设计文档并说明原因,再继续;不在代码里悄悄偏离设计。
