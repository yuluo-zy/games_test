# render/fsr_plugin.rs

> 文件作用：**FidelityFX Super Resolution 的 bevy 插件接线**——PDB 仅 1 个过程 `plugin_impl::impl$0::build`，
> 无独立业务体（FSR 产物是 history/reactive 计算 pass：`_fsr_copy_history`、`_fsr_init_reactive`，见
> render-passes/INDEX.md）。证据：`evidence/building-runtime/B-focused/14003bb80.{c,asm}` + 本批字节复核。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `plugin_impl::impl$0::build` | 0x14003bb80 | 84 | 猜测：插件 build；证实：**1 字节 RET 空函数（ICF 合并）** | **O**：字节 `c3 cc cc…`（0x14003bb80 起 1 B RET + int3 填充，其后 0x14003bb90 是另一函数 `55 41 57…`）；B-focused 反编译显示该址 = `bevy_app::plugin::Plugin::finish<MainCameraPlugin>` 空实现——PDB 名 `fsr_plugin::build` 与 bevy 空 `finish` ICF 合并。**无独立业务过程**；FSR 通过 compute passes 接线（`_fsr_copy_history`/`_fsr_init_reactive`，render-passes/INDEX.md） | B-focused 14003bb80.c/asm + 本批 rne-read 0x14003bb80 |

## 内联/无独立符号函数

- 无（模块仅 marker 类型 `FsrPlugin`/`FsrPluginState`；reconstruction 保留为 typed gap，符合证据）。

## 纠错/差异

- 骨架判"构造/注册/生成"待推断——实为 ICF 空桩，无业务可恢复（FSR 逻辑全在 compute pass 域）。
- reconstruction `fsr_plugin.rs`（29 B）保留 `FsrPlugin` marker + `FsrPluginState` gap，与证据一致，无需补。
