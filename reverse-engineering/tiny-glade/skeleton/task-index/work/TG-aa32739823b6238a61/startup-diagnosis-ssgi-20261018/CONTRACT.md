# W37 SSGI 的单 binding inactive-content 候选

对应 claim：`TG-aa32739823b6238a61 / startup-diagnosis-ssgi-20261018`，仅 B `calculate_ssgi` / 原 `0x1408743b0`，不宣称 A 等价。本次不改 production、Cargo、GPU 或共享数据库。

## 发行 SPIR-V 证据

原 `_ssgi_temporal_filter.cs.hlsl_cs_main_0.bin` SHA `d31c87e724cf473fea5cc9cf65e66699177e57997eb8b8aaf9651482313920b1`。offset 23 起的完整 compute stage 是 13032 B / 3258 words，SHA `243e9f543962788aa5130959ef59e6e2c8aefd990bb9da3ff8f0af184b0aa7f3`，与已有 scene-adapter 和 root retained SPV 逐字节相同。文件尾还有 stage 之外的元数据，未把整段文件尾误作 SPIR-V。

`verify_and_prepare.py` 完整走完 506 条 SPIR-V 指令，调用本机 `spirv-dis --raw-id`，生成原 ID 的完整反汇编与 `cfg-proof.json`。证明使用原 SSA/CFG，不取决于手写 HLSL。

- 原 `history_valid` 是 `$Globals` member0。`%85 = OpAccessChain ... Globals 0` → `%86 = OpLoad` → `%87 = OpINotEqual %86 0`。
- `%82` 按 `%87` 进入 `%89` 或 `%88`；后者 `%94 = OpPhi false %82, %93 %89`。
- `%88` 按 `%94` 进入处理历史的 `%96` 或 input-only `%97`。
- `history_tex` 是原变量 `%24`，shader binding15。整个函数中只有 `%117` 的 `%119 = OpLoad ... %24` 使用此变量，随后 `%123 = OpSampledImage ... %119 ...`，唯一一次历史采样为 `%124 = OpImageSampleExplicitLod ... %123 ...`。
- `%117` 被 `%82/%88/%96/%109` 支配；没有函数调用、其他历史 texel 读取或可绕开的入口。
- 将 history_valid 精确代入 0 后，唯一可达路径为 **`%82 → %88 → %97 → %95`**，不读 frame_constants，因而结论不受其值影响。`%97` 只读取 input_tex，`%95` 仍然写 history_output_tex 和 final_output_tex。
- history_valid=1 且原 view 历史允许时，存在 `%82→%89→%88→%96→%110→%109→%117→%116→%95` 的历史采样路径。因此有效历史必须继续为 active read。

## 窄变更

在 p3 的全部原绑定完成后、dispatch 前，仅当 `!pair.fresh` 时执行：

```rust
p3.inactive_texture_contents(binding::HISTORY_TEX)
```

当前 `fresh` 的真实含义是有效历史；其 false 直接写原 `history_valid=0` uniform。只标记 p3 的 **binding vector index1**（shader binding15），没有跳过 reprojection、input 或任何输出，也不移除原 `0x19` history 读及其 group1。两项输出仍为 `0x32` / group0；图上 usage、同步和生命周期保留。

候选不初始化历史、不修改 shader、不改变 temporal pair 的 current/previous，不改变 valid-history 后的内容检查。其他 W38/39/41/42/W43 历史或无灯笼 placeholder gate 仍需逐项原证，未在此 patch 中改动。

## 交付与验证边界

`ssgi-single-history.patch` 与完整候选源在同目录；`candidate-manifest.json` 保留 production before SHA、candidate SHA 和 CPU filter：

`only_invalid_ssgi_history_skips_previous_contents`

此 CPU 候选覆盖 invalid 标记 index1、valid 不标记、五个原访问码和 group 不变、另外两个 pass 不受影响。已用 rustfmt 2024 解析候选；没有运行 Cargo 或 GPU。root 需要复核最终 production 指纹后运行该 filter 与统一图验证。
