# utils

共享几何、曲线、采样、紧凑索引、数值与文件辅助库。依赖提供的向量、哈希、随机数、半精度和序列化能力直接复用。

## 当前维护状态

2026-10-17 至 18 对所有生产模块完成源码审查与重构，并根据第二轮复审补充随机差分和复杂状态测试，修复非终止循环、失败状态不一致、退化几何、错误变换、周期精度和备份覆盖等问题。保持现有公开模块路径与有效序列化格式；新增 fallible 样条构造器。正常输入的既有数值样例继续运行，错误行为的测试改为产品正确性合同。

**当前版本包含有意的行为修复，历史原指令等价报告不能自动覆盖新源码。** 本轮没有调用逆向工具，不据测试通过声称与原程序所有边界等价。逐项审查、性能证据和验证结果见 [REVIEW.md](REVIEW.md)。

## 模块职责

| 模块 | 职责与约束 |
| --- | --- |
| `geometry/` | AABB、圆、矩形、线/射线/平面/三角交集、重心坐标、UV定位、高度阈值分段 |
| `camera`、`components::transform`、`iso_transform` | 投影、视图、屏幕反投影、SRT及父子刚体变换；旋转要求单位四元数 |
| `curve/` | 折线参数化、投影、裁剪、重采样、平滑；失败的可变操作保持旧状态 |
| `natural_cubic`、`rdp` | 自然/周期样条与迭代折线简化；非法控制点、容差或步长明确拒绝 |
| `loop_range`、`wall_space`、`comparative_space` | 环形索引、借用视图、墙坐标与最短周期位移 |
| `angle_range`、`circular_f32`、`threshold_range`、`subtractive_1d_boolean` | 角度夹取、环形插值、连续阈值分段、保留几何端点的区间裁剪 |
| `float_grid`、`index_condenser` | 双线性采样、稠密索引与句柄缓存修复 |
| `scalar`、`bytes` | 标量插值/钳制与显式端序字节读写；min/max 采用 std 语义 |
| `easing_functions`、`timer` | 缓动与四样本时间过滤 |
| `rgb9e5`、`halver`、`swizzlers`、`name_hash`、`rng` | 紧凑颜色、半精度、坐标投影、名称哈希与确定性随机种子 |
| `selective_deserialize` | 选择性读取嵌套字段；有意保留完成的局部结果并忽略后续解析错误，不用于完整文件校验 |
| `file`、`local_time` | 时间戳文件名、不会覆盖既有目标的备份、缓存固定时区偏移 |

## 错误与边界合同

- 外部数值反序列化通过 `Result` 返回错误。带 `try_` 的采样/样条入口返回结构化错误；`crashy_` 与要求合法索引/尺寸的便捷构造器仍可能 panic。
- 重复曲线点允许存在，墙坐标外推寻找首尾非零段；全退化曲线不外推。零距离切片扩展严格保持原坐标，极短/极长非零段采用稳定长度计算。普通零切线返回零；最小次正规非零方向使用缩放归一化。直接修改公开点集、参数缓存或样条字段后，调用方必须自行维持一致性。
- `LoopSlice` 越界索引会 panic，逻辑索引绕回不意味着视图索引可以越界；空尾切片合法。
- AABB 的线段交集采用闭边界。`Aabb2::intersect_ray` 返回有符号 slab 进入参数，单独的返回值不能充当完整命中判定。
- 重心坐标退化输入仍返回 IEEE 非有限值，UV查询会跳过这类三角形。三角交集使用 f64 相对坐标；共面要求法线近似平行，双方点到平面的距离各不超过自身最长边的 `1e-10`；非共面截段使用舍入误差界，不以长边容差吞掉短交段。零面积三角形返回 `Disjoint`，`Segment` 两端顺序不作保证。`Coplanar` 只表示共面，不保证有限区域相交。矩形 `intersects` 判断闭合轮廓接触，严格包含但边界不接触时返回 false。
- RDP 按有限线段距离检查所有子链，保证被省略顶点的误差不超过 `epsilon`；端点间距是软偏好，不会阻止必要的细分。准备省略顶点时用带舍入误差界的距离上界复核，数值不确定时保守保留。与旧实现的选点结果可能不同。
- 周期距离先保留各输入的相位，半周期相等时选择负方向；周期插值从归一化相位起算，避免大坐标吞掉短位移。
- 网格 legacy 模式保留正边界外插，centered 模式夹取到边缘。区间裁剪保留切口端点，不等价于数学集合差；空半开区间是空操作，重复裁剪不会复制零宽碎片。
- `rotation_from_normal` 的单位输入产生旋转；任意非单位有限法向的输出不保证单位四元数。相机的未锚定 `field_*` 字段保留原有名称，避免误导性的语义重命名。
- 时间使用启动时缓存的固定 UTC 偏移，未初始化或发现失败时用 UTC；不是历史时区/DST转换器。备份名称冲突通过原子 `create_new` 加编号处理，失败尽力清理部分输出；不保证对并发修改的源文件生成一致性快照。

## 测试与验证

```sh
cargo fmt -p utils -- --check
cargo clippy --offline -p utils --lib --test core_properties --test curve_properties --test geometry_properties --test integration_properties -- -D warnings
cargo test --offline -p utils --all-targets
cargo test --offline -p utils --all-targets --release
cargo check --offline -p country-core --lib
```

新增回归集中在 `tests/{core,curve,geometry}_review.rs`、`tests/{core,curve,geometry,integration}_properties.rs` 与文件/时间模块测试，按不变量和故障类型聚合。复杂测试包含独立 f64 参考、顶点排列/交换/变换不变量、固定种子随机输入、长序列状态机及并发 IO。原数值 fixture 保留，运行实现中的大段样例移动至 `src/oracle/`、`camera/tests.rs`、`natural_cubic/native_tests.rs`、`rdp/native_tests.rs`，不通过删除或忽略整组测试掩盖失败。具体运行结果与未验范围见审查报告。

## 未补造的能力

本轮是源码维护，没有新增原语义未知的墙边界转换、Spline2D全局查询或原始Vec3借用投影ABI。`Vec3SliceView2` 仍是安全传输适配器，有所有权的投影明确使用XZ轴。跨平台位级差异和完整游戏/GPU运行不在本轮源码测试结论内。

## 历史来源


- 恢复源码：[shared-foundation/utils](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/utils/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/utils/`。
- 原 B 包/PDB 归属：`crates/utils/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`326cb236`、`3d924ccd`、`3e0387cd`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 导入时源工作树还有修改：`src/comparative_space.rs`、`src/geometry/aabb.rs`、`src/geometry/circle.rs`、`src/geometry/position_from_mesh_uvs.rs`、`src/geometry/ray.rs`、`src/loop_range.rs`、`src/natural_cubic.rs`、`src/rdp.rs`、`reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/base-assets/rdp_tests.rs`、`reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/curves/query_tests.rs`、`reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/curves/sampling_tests.rs`、`reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/geometry/native_tests.rs`；是否选用以审计记录的 selected_revision/source_sha256 为准。未选的改动单列 unselected_worktree_sha256，旧提交验证不自动覆盖新的源内容。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/utils-gaps-2026-10-13/REPORT.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-review-fixes-2026-10-14/REPORT.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-review-fixes-2026-10-14/numeric-followup/REPORT.md)。
- 2026-10-14 原码补全与消费者合并：[本轮报告](../../../reverse-engineering/tiny-glade/evidence/utils-gap-recovery-2026-10-14/REPORT.md)。本轮源码修改及输入 SHA 另列于 `provenance/utils-gap-audit.json`；导入提交只表示历史来源。
- 2026-10-14 B 深挖：[消费者与行号证据](../../../reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14/REPORT.md)，当前新增源码验收见 `provenance/utils-deep-audit.json`。后续范围按用户要求仅 B。
