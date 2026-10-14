# utils

游戏自有的几何、曲线、坐标、数值与缓存辅助库，供建筑、相机和渲染数据生产使用。向量、随机数、半精度、容器和序列化直接复用原版本依赖。

## 功能与目录

geometry/（AABB、圆、矩形、射线、三角形、高度图）；curve/（Curve2/3、参数化、采样、切片）；natural_cubic.rs、wall_space.rs、loop_range.rs；easing_functions.rs、rng.rs、rgb9e5.rs、index_condenser.rs；其余命名/文件/计时工具。

## 来源

- 恢复源码：[shared-foundation/utils](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/utils/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/utils/`。
- 原 B 包/PDB 归属：`crates/utils/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`326cb236`、`3d924ccd`、`3e0387cd`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 导入时源工作树还有修改：`src/comparative_space.rs`、`src/geometry/aabb.rs`、`src/geometry/circle.rs`、`src/geometry/position_from_mesh_uvs.rs`、`src/geometry/ray.rs`、`src/loop_range.rs`、`src/natural_cubic.rs`、`src/rdp.rs`、`reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/base-assets/rdp_tests.rs`、`reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/curves/query_tests.rs`、`reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/curves/sampling_tests.rs`、`reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/geometry/native_tests.rs`；是否选用以审计记录的 selected_revision/source_sha256 为准。未选的改动单列 unselected_worktree_sha256，旧提交验证不自动覆盖新的源内容。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/utils-gaps-2026-10-13/REPORT.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-review-fixes-2026-10-14/REPORT.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-review-fixes-2026-10-14/numeric-followup/REPORT.md)。
- 2026-10-14 原码补全与消费者合并：[本轮报告](../../../reverse-engineering/tiny-glade/evidence/utils-gap-recovery-2026-10-14/REPORT.md)。本轮源码修改及输入 SHA 另列于 `provenance/utils-gap-audit.json`；导入提交只表示历史来源。
- 2026-10-14 B 深挖：[消费者与行号证据](../../../reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14/REPORT.md)，当前新增源码验收见 `provenance/utils-deep-audit.json`。后续范围按用户要求仅 B。

## 已完成

已恢复几何与曲线数值规则、自然样条构造和局部求值、3D 全局样条查询、墙空间转换、范围切片、缓动、随机分割、颜色打包与索引缓存 insert/get/remove/clear。insert 遇到重复 key 返回 None，不改动状态；新 key 使用原数组长度作为稠密下标。remove_index 返回移除前的下标，供实例消费者同步交换载荷。

country-core 的索引压缩和 FloatGridCells 采样已改为共享 utils 的适配器；OwnedGpuWorldSubset 的 ID 重试、缓存修复、移除与载荷搬动通过该入口。512×512 字节网格两条分支新增 100 组原指令逐位样例；插入 52 组、完整移除 78 组，fixture 保存在本包内。原采样开关名为 USE_FLOAT_GRID_SAMPLING_FIX，指针槽 `0x14362c308` 指向字节 `0x143638fac`。

B 原楼梯消费者 `densely_sample_spline_segment`（`0x140809ad0`）已恢复为 `Spline2D::densely_sample_segment`：每段至少两点，数量为 ceil(2×跨度)，包含末端；输出首切线及取负的末切线。143 组原构造→采样样例含 7 组越界分支，系数及数值输出逐位核对。原标量构造先对每段差值乘 3、再乘倒数；已修复旧容差测试漏掉的运算顺序差异，20 组原 3D 构造与 120 组全局查询也逐位通过。

Curve 的有所有权转换已证明 XZ 投影：原 map/collect 单态化 `0x1402ba9e0` 读取输入偏移 0/8、输入 stride 12、输出 stride 8。`reinterpret_as_curve2/try_to_curve2` 共用 `project_vec3_slice_xz`，继续复用 glam 与 std。10 组数组共 166 个点（含 NaN payload 与有符号零）逐位核对。

## 尚差的功能与验收

| 项目 | 本轮取证结论 | 当前边界 |
|---|---|---|
| WallSpaceCoord::bound/is_bounded/to_affine3 | 4894 模块过程/内联与 85322 段 C13 行号记录交叉查找；声明起始行 272/265/198 没有可定位机器码。另查 WallBoundCoord 同名声明，未形成正面映射 | 保持未实现；缺记录不能证明死代码或确定未编译 |
| Spline2D 全局段查询/位置/切线 | IPI 声明仍未映射到原码；段选择声明行 164，无对应行号地址。真实楼梯消费链使用已恢复的分段采样 | 全局 API 仍未恢复，分段消费链已验；两者分别记账 |
| Vec3SliceView2 / 原 as_vec2 借用类型 | 四字节入口只证明 RCX→RAX；未变化的 RDX 不证明长度或 slice 接收者。106 个几何域同址调用候选仍有折叠歧义 | 安全借用接口是重建适配选择；原借用类型/投影仍未知。有所有权 Curve XZ 投影轴与 stride 已证明 |
| IndexCondenser 与实例消费者 | 两处原 insert 内联和完整 remove 已差分；共享逻辑已接线 | K/Option 是安全语义适配，不宣称作者原始 Rust 泛型/返回 ABI；分配失败、超大索引和原完整 GPU 进程未验收 |
| FloatGridCells 与世界栅格 | 512×512 字节入口两分支逐位验证并合并 | 开关写入者、其余网格单态化和完整游戏运行仍需验收 |

包内没有 experimental-reconstruction 特性；未知墙方法、2D 全局查询和借用投影访问未进入正式包。借用传输作为显式重建适配器保留。Windows 的现有数值样例逐位验证与其他平台的容差合同分开，不代表跨平台所有输入逐位等价或全部游戏消费者完成。

## 持续开发差异

源工作树数值修复按导入时哈希登记，不能将最后 Git 提交当作这些修复的验收。正式包仅留已恢复内容，已删除此前实验门控的猜测 API 与 feature。原 get/remove/clear 测试直接初始化已观察到的私有数据状态，不借用猜测 insert 生产实现；未改研究目录。geometry/native_tests.rs 选择与已验收报告输入 SHA497528a3 相同的 Git fixture，未选用工作树中反向覆盖已确认 expand 下界和新增未验收样例的改动；两份哈希与选择理由均保留在审计清单。

在 workspace 根执行 `cargo test --locked --offline -p utils --all-targets` 验证正式恢复范围；Release 加 `--release`。消费者回归运行 `cargo test --locked --offline -p country-core --lib`。本轮具体结果和保留的失败记录见上方报告。
