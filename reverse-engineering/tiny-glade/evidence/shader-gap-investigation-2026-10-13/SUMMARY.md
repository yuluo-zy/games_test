# Shader 剩余缺口调查与实编译验收（2026-10-13）

本轮承接已推送的 `902bb6e`，最终有效结果为 [corpus-final-native/report.json](corpus-final-native/report.json)。旧 `corpus-native-recipes` 与 `ubo-prefix-recheck` 保留为中间证据，不能替代最终结果。

- 官方 DXC 1.9.2609 实际生成 404/404 新包、638/638 SPIR-V，预处理／编译／打包失败 0；全部包经过实际反射和 Speedy 编解码。
- 完整 typed 反射结构 397/404 相等；公共完整接口 313/638 相等；capability 集合 300/638 相等；非 debug opcode 直方图 88/638 相等。这些指标不证明完整算法或数值语义等价。
- 38 个非零 compute 资产从原 native producer／caller 恢复 8 个宏，全部原 key、工作组大小和 typed 反射一致。132 个 zero-key 原 tuple 仍无独立 producer 证明；XOR 零不能唯一证明空配置。
- 修复 69 个 VS 输入声明、119 个源码中的 796 个 globals 标识符、两 deferred 字段顺序及 solid_vertex_color 的 84B 布局。独立审查确认 34 个误改 PS 输入块已恢复；原 19 个失败包的 38 阶段 uniform 名称、类型、偏移、大小均与原版相同。
- 原 artifact 宏与还原源码选择宏 TGV 分开记录和编译；TGV 不参与原 artifact key。恢复原 runtime load_compiled 的读取／解码和两个错误文本，候选返回 String，未宣称原 anyhow 对象 ABI 相同。
- 21 个相关 Rust 测试、严格 all-targets Clippy 通过；新包 compute GPU 读回及 raster 布局注册通过。没有 raster draw 或整帧验收。

## 仍需恢复

1. [7 个 typed schema 残差](build-entry/final-schema-residuals.json)：final_post 深度变体缺原资源／uniform；gothic_window_bricks、solid_albedo_map_emissive、wall_brick_lod0 各两个变体仅 vertex 输入名称不同，location/type 相同。不得删除活跃输入或注入原 metadata 冒充相等。
2. [编译器 capability 残差](compute/capability-residual-review.md)：13 个新增 Int64 来自 batch_lift.py 主动生成 u64 RawBufferLoad 地址；原 29 处为 uint2 → physical pointer Bitcast。作者公开 RawBufferLoad2 补丁适合原类型形状，但原 DXC 版本／fork commit 未确认，尚未构建和验收此 fork。另有图像格式／精度和 quad 声明差异。
3. 217/404 源依赖集合与原包不同，187 相同；反射一致不代表 include 组织、预处理文本或着色算法一致。完整公共接口仍有 325 阶段差异。
4. 原离线 asset 构建 CLI、provider/cache/temp 命名与清理策略、完整宏状态及原资产状态机仍未知。runtime PDB 和原发行 manifest 不含该离线工具源码／日志。整帧仍后置。

## 来源边界

沿用 [上一轮来源核验](../shader-build-2026-10-13/provenance/PROVENANCE.md)：shader-prepper 为作者公开库（原 0.3.0-pre.3 直接复用），SpirQ 为作者公开 fork（原锁 4f22e6f）；DXC 基于 Microsoft 开源编译器且作者公开部分补丁。游戏本地 shader-compiler／pipeline／variants、asset-pipe 的完整公开上游尚未确认。候选 kajiya／rendertoy 全文和 PDB checksum 不匹配只能排除具体完整文本同一，不能证明全部独立开发或没有借用片段。

输入及交付哈希见 final-inputs.json；native key 复验见 final-native-check.json，独立审查见 build-entry/validation.json 与 interfaces 最终审查记录。历史证据保留原状态。


## 缺口进一步分析

见 [7 个反射差异与 132 个零 key 配置分析](GAP_ANALYSIS.md)：六个包仅 VS 名称不同；final_post 深度原本已有无输出的残留计算，lifting 与新 DXC 优化共同改变资源反射，需核真实 consumer，不能误判为完整颜色后处理丢失。132 资产覆盖126source，空 artifact defines 实编译均通过且 typed schema／LocalSize全等，但原 tuple 仍需原侧构造与追加路径证明。本次为只读分析，没有关闭缺口或更新原语义完成度。
