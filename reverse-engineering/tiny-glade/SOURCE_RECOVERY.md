# Tiny Glade 源码逆向恢复状态

当前优先恢复 Rust 源码。场景回放、GPU 管线和画面验收后置。最终目标仍是完整 Rust 游戏工程；下面完成的是屋顶 CPU 算法源码链，尚不是完整游戏。

## 先核查原始源码

发行树 2080 个文件中未发现 `.rs` 或源码归档。匹配 PDB 的全部 named streams 中，三个 `/src/files/` 流都是 `.natvis` 调试 XML，已实际导出；没有 `.rs`、srcsrv 或 SourceLink 命名流。PDB 提供原模块、函数、源路径及源校验和，但这些不是作者的源文件内容。

核查证据：[report.json](evidence/source-discovery/report.json)、[PDB named streams](evidence/source-discovery/pdb-named-streams.txt)、[屋顶源路径元数据](evidence/source-discovery/roof-source-metadata.txt)。检索限于给定发行树和 PDB 命名流，不声称搜索了作者的机器或私有仓库。

因此当前产物是**由原作机器码恢复的 Rust 实现源码**，以数据行为等价为标准；不宣称获得作者原始文本、注释、泛型定义或完整 ABI。

## 已落实的源码链

| 原作模块/函数族 | Rust 源文件 | 当前源码恢复范围 | 原作校验 |
|---|---|---|---|
| country_core / roof_shape | [surface.rs](reconstruction/roof/surface.rs) | profile 幂映射、圆/矩形截面、世界尺寸/高度、20点曲线 | [23810 case](evidence/roof-surface/verification.json) |
| utils / curve | 同上 `Curve2` | 弧长累计、points_u、构造错误、查找/插值/单位切线 | 包含在同一报告；完整 profile→Curve2→WS→Curve2链1512次 |
| country_core / roof_ridge | [ridge.rs](reconstruction/roof/ridge.rs) | 脊向/脊长、逆参数、尖顶偏移、点/线段世界脊线 | [14776 case](evidence/roof-ridge/verification.json) |
| system_roof / assemble_circular_roof | [tiles.rs](reconstruction/roof/tiles.rs) | 行列数、弧长采样、游戏随机分割/依赖RNG调用、姿态/尺寸/位置、檐口补瓦 | [11467条完整记录](evidence/roof-tiles/circular-verification.json)，37输入包含12个完整原函数入口 |
| assemble_rectangular_roof / 四边条瓦 | [rectangular_edges.rs](reconstruction/roof/rectangular_edges.rs) | 原始展开上下文到四边曲线条瓦，保留RNG/seed推进 | [32输入/1164条](evidence/rectangular-edge-verification.json) |
| assemble_rectangular_roof / 脊瓦 | [rectangular_caps.rs](reconstruction/roof/rectangular_caps.rs) | 自主端点、gable延长、随机分段、Quaternion、64字节打包 | [22组/107条](evidence/roof-surface/caps-verification.json) |
| assemble_rectangular_roof / 面瓦与补片 | [rectangular_faces.rs](reconstruction/roof/rectangular_faces.rs) | 面与行构造、剪裁平面、曲线采样、姿态与瓦片输出 | [29屋顶/11591条](evidence/roof-ridge/composed-faces-verification.json) |
| system_roof / CPU组合 | [pipeline.rs](reconstruction/roof/pipeline.rs) | `edges → caps → faces/fillers`，从Rectangle/Roof输入自主计算 | [33完整输入/19024条](evidence/roof-pipeline-verification.json)，全部64字节一致 |
| tools_wall_shape / 墙高约束；country_core / 标高缓存 | [height_rules.rs](reconstruction/wall/height_rules.rs) | clamp已解析状态的数值后段；max_y/max_y_gable/flat_roof_y完整叶函数 | [6624案例](evidence/scope-priorities/wall-rules-verification.json)，包含NaN/无穷与缺失缓存panic条件 |
| system_roof / edit_roof | [edit_rules.rs](reconstruction/roof/edit_rules.rs) | 已解析Roof/Event的字段编辑、变化量、change tick、gable重建与数值反馈；ECS前段未恢复 | [4514案例](evidence/source-edits/verification.json)，完整状态与通知数值逐bits一致 |

子阶段与完整组合覆盖存在重叠，**不能把这些瓦片数相加当作独立总覆盖量**。所有来源绑定 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。部分数学调用桥接本机真实 CRT，分配器/Writer只承担环境与结果捕获；报告具体说明执行边界。

上述完整 CPU 函数验证仅适用于测试中的合法 pitched 屋顶输入。Flat屋顶、ECS世界布局、编辑消费者、全部异常路径、分配失败和浮点异常标志仍未恢复完毕。原作 history 的参数值用于覆盖，但墙体形状/姿态为合成输入，没有重放场景。

依赖边界已纠正：glam 0.29.3、fastrand 1.8.0、half 2.4.1直接调用原版公开API，已删除临时手写库算法并重新通过完整铺瓦回归。源码覆盖只统计游戏规则；依赖函数元数据仍保留作调用证据，不算还原成果。见[依赖边界](evidence/roof-tiles/dependency-boundary.json)和[全依赖政策](tracking/dependency-policy.json)。Bevy分叉按用户说明记录，版本0.16.0已核实，分叉仓库/commit未核实。

## 组织原作模块与类型

[source-map.json](evidence/roof-tiles/source-map.json)将22个原函数映射到原PDB源路径候选及恢复API，登记 Roof88字节、Rectangle24字节、Curve2原内存56字节、Bounds输出20字节、TileRecord64字节、RNG u64 等观察布局。路径匹配是源码组织证据，尚不是原源行文本映射。

Rust采用显式参数结构/枚举、Curve2和纯函数，保留已确定的数学次序；完整 ABI 未恢复的边界继续提供字节接口。`Width/Length` 的序列化名字到方向 bit 的对应关系待原消费者核实，不补造默认值。

## 当前源码阶段下一步

1. 恢复 `create_roof`/`update_roof_spatial`/`edit_roof` 的原作对象类型、输入输出容器及事件消费者，将CPU核心接入原作权威状态模型。
2. 恢复 Roof history 的反序列化与 Apply/Replay 消费者，确认字段与枚举，并联接源码状态变化；这一步不进行场景回放。
3. 转入墙体曲线、砖块与支撑的完整源码函数，沿相同证据→Rust→原指令校验流程扩展项目。
4. 核心源码和类型联接完成后，再进入场景回放与 GPU；最终验收仍需完整游戏集成。

工程：[Cargo.toml](reconstruction/Cargo.toml)。进度：[PROJECT_PROGRESS.md](PROJECT_PROGRESS.md)。依赖范围：[ORIGINAL_MODULE_SCOPE.md](ORIGINAL_MODULE_SCOPE.md)。
