# shaders

按 B 发行 SPIR-V 嵌入的源路径整理的 415 个可读 HLSL 文件。覆盖 compute/raster 主文件和 include 支持，功能包括墙/屋顶/植被、地形/水冰、光照/阴影/GI、后处理、UI 与诊断。它们由 `country-core` 游戏 pass 配置、`shader-compiler` 构建器和 `rhapsody` shader 包消费协作使用。

来源是 [`human-readable-shaders/shaders`](../../reverse-engineering/tiny-glade/reconstruction/human-readable-shaders/shaders)，主要提交 `81aa0042`（可读化全量重写）及 `c05fad96`。前序 `2992adf5`、`6d30d3ae`、`ba1acef4`、`db1ecad7` 保存恢复、执行差分与原宏/反射修复过程。逐文件来源见 [`migration-sources.json`](../crates/rhapsody/migration-sources.json)，原验证清单见 [`SHADER_STATUS.md`](../../reverse-engineering/tiny-glade/reconstruction/shader-source/SHADER_STATUS.md) 与 [`FINAL_SWEEP.json`](../../reverse-engineering/tiny-glade/reconstruction/human-readable-shaders/FINAL_SWEEP.json)。

本次纠正原筛选遗漏的 22 文件：`final_post.raster.hlsl`、`water/ice_refl_resolve.cs.hlsl` 和 20 个 include。final_post 颜色后处理已恢复；深度变体原来就是无输出的残留计算，其资源在新 DXC 中消失仍是具体接口缺口，没有补造颜色输出或副作用。ice_refl_resolve 的 EDGE 已逐值证明来自 DXC 表达式重结合和驱动 FMA；严格 `-Gis` 顺序可一致，不应误认成缺少函数而删掉文件。

`manifest.json` 列实际构建输入与原 artifact/源码 recipe 两层宏。TGV 是恢复源码分支选择，不参与原 artifact hash；132 zero-key compute 的原宏列表仍需原 producer 证明。历史可读化 638 stage 新旧源码编译结果一致，仅覆盖可读化无逻辑漂移；此前 lavapipe 的 SAME/EDGE 是有限输入差分，不是整帧或全输入证明。

还缺最终原 include 组织、7 个 typed schema 残差、部分能力/图像格式/物理地址 fork 差异、原完整编译器身份，以及游戏 producer→shader→raster draw→整帧验收。FidelityFX/XeGTAO 等公开算法边界按原证据记录，不把第三方代码计为自有逆向成果。
