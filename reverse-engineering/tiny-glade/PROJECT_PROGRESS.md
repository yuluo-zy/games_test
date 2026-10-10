# Tiny Glade 原作还原：里程碑与进度

更新：2026-10-10T10:38:19.651657+00:00。目标：完成一个 Rust 项目，完整复现当前原作构建的游戏实现与行为。

**当前不是完整游戏。整体完成百分比暂不报告：完整原作功能清单及验收分母仍在建立。**

进度必须区分：定位原作 → 结构/调用链确认 → Rust 实现 → 局部机器码差分 → 系统场景验收。仅反编译不算还原验收。

当前用户指定**源码恢复优先**；场景回放和 GPU 研究后置。源码关口与最终系统验收分列，局部差分通过不替代完整游戏集成。

| ID | 里程碑 | 最终状态 | 源码关口 | 前置验收 | 验收条件 | 证据 |
|---|---|---|---|---|---|---|
| M00 | 原作身份与研究输入 | 完成验收 | — | — | RNE/PDB 匹配身份；副本哈希一致；EXE 差异登记 | [baseline.json](evidence/binary/baseline.json)<br>[identity.json](workspace/input/identity.json) |
| M01 | 可构建 Rust 工程与证据追踪 | 完成验收 | — | M00 | Cargo 锁定构建成功；每个已验收算法对应原作地址/报告；追踪工具可复现 | [Cargo.toml](reconstruction/Cargo.toml)<br>[Cargo.lock](reconstruction/Cargo.lock)<br>[project-build-verification.json](evidence/project-build-verification.json)<br>[update-progress.py](scripts/update-progress.py) |
| M02 | 已定位 Roof 状态函数 | 完成验收 | — | M00 | Roof::new/ty/is_gable 的观察输出通过机器码差分；浮点/内存边界登记 | [roof_state.rs](reconstruction/roof/roof_state.rs)<br>[roof-state-verification.json](evidence/native/roof-state-verification.json) |
| M03 | 屋顶曲面核心 | 进行中 | 源码实现及原码校验通过：截面/profile/Curve2构造与采样完整CPU链；23810用例 | M00, M01 | 原作实际曲面类型及参数来源确定；曲面/导数/法线 Rust 可运行；各原作分支差分通过 | [surface.rs](reconstruction/roof/surface.rs)<br>[verification.json](evidence/roof-surface/verification.json) |
| M04 | 脊线与屋顶形状组装 | 进行中 | 源码实现及原码校验通过：脊长/脊向/尖顶偏移及世界点/线段；14776用例 | M00, M01 | 脊向/脊长/檐口/屋顶分支输入输出明确；完整几何子链 Rust 与机器码差分 | [ridge.rs](reconstruction/roof/ridge.rs)<br>[verification.json](evidence/roof-ridge/verification.json) |
| M05 | 铺瓦核心 | 进行中 | 源码实现及原码校验通过：矩形33输入19024条完整记录；圆形37输入11467条，其中12为完整原函数入口 | M00, M01 | 屋顶采样到瓦实例布局/索引/变换的完整原作链恢复；CPU/GPU职责明确；差分通过 | [pipeline.rs](reconstruction/roof/pipeline.rs)<br>[roof-pipeline-verification.json](evidence/roof-pipeline-verification.json)<br>[circular-verification.json](evidence/roof-tiles/circular-verification.json) |
| M06 | 屋顶编辑与完整场景验收 | 进行中 | 部分源码及原码校验通过：4514案例：已解析Roof/Event的编辑后段、变化量与数值通知规则；ECS查询/调度未完成；场景后置 | M02, M03, M04, M05 | 历史命令到状态/曲面/脊线/瓦实例贯通；矩形、圆形及原作其他已观察类型编辑与撤销验证 | [edit_rules.rs](reconstruction/roof/edit_rules.rs)<br>[verification.json](evidence/source-edits/verification.json)<br>[findings.md](evidence/source-edits/findings.md) |
| M07 | 世界数据/历史/保存恢复 | 进行中 | 恢复中：静态历史结构已解析；Rust消费者/字段映射待恢复 | M00, M01 | 33 历史样本消费者、版本迁移、ID与快照恢复实现；撤销/重做/重放与原作对照；缺失样本显式登记 | [findings.md](reconstruction/history/findings.md)<br>[sample-verification.json](reconstruction/history/sample-verification.json) |
| M08 | 墙体与砖/支撑/开口 | 进行中 | 部分源码及原码校验通过：6624案例：已解析状态的墙高约束后段及3个标高缓存叶函数；墙轮廓/砖/支撑/开口待恢复 | M01, M07 | 各实际原作形状与相交关系实现；墙编辑到实例输出差分；建筑场景验收 | [height_rules.rs](reconstruction/wall/height_rules.rs)<br>[wall-rules-verification.json](evidence/scope-priorities/wall-rules-verification.json) |
| M09 | 门窗/装饰/附着关系 | 未开始 | — | M06, M07, M08 | 放置/尺寸/变形/冲突/宿主变化与恢复实现；原作样本与局部规则验证 | 尚无验收证据 |
| M10 | 地形/道路/水/园景 | 未开始 | — | M01, M07 | 坐标/笔刷/曲线/插值及跨对象变化传播实现；对应原作场景操作对照 | 尚无验收证据 |
| M11 | 输入/拾取/任务调度 | 未开始 | — | M06, M08, M09, M10 | 原作时序与脏更新依赖明确；连续拖动、反向、撤销与异步任务一致性通过 | 尚无验收证据 |
| M12 | 资源/着色器/渲染 | 未开始 | — | M01, M06, M08, M09, M10 | 原作实际 pass、实例、材质与资源格式恢复；固定相机画面及性能对照 | 尚无验收证据 |
| M13 | 生态/天气/音频/UI/相机 | 未开始 | — | M07, M11, M12 | 观察到的外围系统行为/参数/资源依赖逐项实现并验证 | 尚无验收证据 |
| M14 | 完整 Rust 游戏集成验收 | 未开始 | — | M06, M07, M08, M09, M10, M11, M12, M13 | 可运行交互游戏；全部确认的原作功能清单关闭；历史、场景、视觉、性能及异常恢复回归通过 | 尚无验收证据 |

## 已验证局部算法

| 算法 | 状态/范围 | 实现 | 原作参照 |
|---|---|---|---|
| WindowSize::aspect_ratio | verified_local：仅局部 f32 结果，1280 输入 | [Rust](reconstruction/math/window_aspect.rs) | [差分报告](evidence/native/window-aspect-verification.json) |
| Roof::new/ty/is_gable | verified_local：仅字节写入/布尔返回；256 构造器 + 两函数各1348输入 | [Rust](reconstruction/roof/roof_state.rs) | [差分报告](evidence/native/roof-state-verification.json) |
| Roof surface/Curve2 | verified_local：23810用例，原指令/CRT/曲线链；异常范围见报告 | [Rust](reconstruction/roof/surface.rs) | [差分报告](evidence/roof-surface/verification.json) |
| RoofRidgeDims/tip | verified_local：14776用例，点/线段脊线核心 | [Rust](reconstruction/roof/ridge.rs) | [差分报告](evidence/roof-ridge/verification.json) |
| Rectangular CPU full pipeline | verified_local：33完整输入、19024条64字节记录；没有prepared geometry注入 | [Rust](reconstruction/roof/pipeline.rs) | [差分报告](evidence/roof-pipeline-verification.json) |
| Circular CPU tiles | verified_local：37输入、11467记录；25prepared/12完整入口 | [Rust](reconstruction/roof/tiles.rs) | [差分报告](evidence/roof-tiles/circular-verification.json) |
| Wall height numeric policy / cached heights | verified_local：6624案例；clamp仅数值后段，3个缓存叶函数含缺失panic条件；ECS/哈希实现排除 | [Rust](reconstruction/wall/height_rules.rs) | [差分报告](evidence/scope-priorities/wall-rules-verification.json) |
| Roof resolved edit core / feedback rules | verified_local：4514案例；完整88字节状态、changed tick、变化量、gable重建及反馈数值；已解析后段，不是完整ECS系统 | [Rust](reconstruction/roof/edit_rules.rs) | [差分报告](evidence/source-edits/verification.json) |

## 尚未关闭的边界

- 发行树/PDB没有取回作者原始Rust文本，当前为行为恢复源码
- Roof历史字段名字到ABI偏移和Width/Length bit映射待原消费者核实
- 屋顶CPU算法通过限定原码校验；Flat屋顶/完整ECS调用者及全部异常路径未完成
- 22个快照引用目标缺失
- 完整游戏、场景回放与GPU尚未验收，后两项按用户要求后置

## 当前后置工作

- 场景回放与运行交互验收：用户要求当前不做
- GPU/渲染/着色器动态算法：用户要求当前不做

## 下一批可执行任务

1. S01/S03/S04：恢复Roof/Wall权威类型、create/update/edit状态消费者和事件触发；复用Bevy查询/容器
2. S02/S05：定位特殊墙高flag生产者，恢复height history提交/撤销与游戏对象合法性规则
3. S06/S07：优先恢复墙轮廓、construct_brick_columns与支撑源码；复用glam/RNG/half
4. S21/S22：恢复原rkyv/ron fork之上的游戏schema与版本迁移；不运行场景回放

工程：[Cargo.toml](reconstruction/Cargo.toml)。完整研究计划：[MASTER_PLAN.md](MASTER_PLAN.md)。源码任务与优先级：[TASK_BACKLOG.md](TASK_BACKLOG.md)。依赖排除与持久记忆：[PROJECT_MEMORY.md](PROJECT_MEMORY.md)。机器可读台账：[project.json](tracking/project.json)。

依赖列描述最终验收依赖，源码阶段可以沿已确认的数据接口继续推进。排期以实际源码覆盖和验证吞吐滚动更新，不承诺未经测量的整游戏完工日期。更新台账后执行 `scripts/update-progress.py`，它会核验已完成项的依赖、证据文件与局部差分结果。
