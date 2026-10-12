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

Rust采用显式参数结构/枚举、Curve2和纯函数，保留已确定的数学次序；完整 ABI 未恢复的边界继续提供字节接口。B 原序列化消费者现已确认 `Width=0/Length=1`；A/C 内存布局与缺失 rim 字段默认值尚未确认，不补造默认值。

## 当前源码阶段下一步

1. 恢复 `create_roof`/`update_roof_spatial`/`edit_roof` 的原作对象类型、输入输出容器及事件消费者，将CPU核心接入原作权威状态模型。
2. 恢复 Roof history 的反序列化与 Apply/Replay 消费者，确认字段与枚举，并联接源码状态变化；这一步不进行场景回放。
3. 转入墙体曲线、砖块与支撑的完整源码函数，沿相同证据→Rust→原指令校验流程扩展项目。
4. 核心源码和类型联接完成后，再进入场景回放与 GPU；最终验收仍需完整游戏集成。

工程：[Cargo.toml](reconstruction/Cargo.toml)。进度：[PROJECT_PROGRESS.md](PROJECT_PROGRESS.md)。依赖范围：[ORIGINAL_MODULE_SCOPE.md](ORIGINAL_MODULE_SCOPE.md)。

## 三版本工程首批增量

新增 [tg-history](reconstruction/crates/tg-history/src/lib.rs)、[参数适配器](reconstruction/roof/parameters.rs)、[回放生产者](reconstruction/roof/replay.rs)。三个版本98份真实历史/733条屋顶命令解码通过；B 参数映射1024例、eave有限输入2097例、VisualEdit回放1600例、离散回放7168例、真实历史到已解析编辑核心708例通过原指令对照。

当前 ARM 构建新增 x86 invalid-NaN 兼容边界，解决首次重验的43例符号差异，编辑核心4514例现全部通过。旧 Windows 报告继续作为历史记录；当前构建的验收参照 [B-edit-rules-host-verification.json](evidence/multi-version/B-edit-rules-host-verification.json)。此前曲面、瓦片等全链报告不能直接证明扩展 workspace 和当前主机重验通过。

[本轮发现与精确边界](evidence/multi-version/B-parameter-findings.md)登记原地址、字段、验证范围和未恢复项。下一步是原 ECS 对象关联与 lifecycle 消费者、完整历史事务，再沿墙体与世界状态扩展；未达到完整游戏验收。

## 2026-10-11 建筑业务与依赖过滤

新增 [Wall业务状态](reconstruction/building/wall_state.rs)、[Roof创建初始化](reconstruction/building/roof_creation.rs)、[墙驱动Roof更新](reconstruction/building/roof_spatial.rs)。2659个状态/读取guard案例、848个初始化案例、512个原回放→创建→墙更新联接案例通过。代码复制/比较的是游戏字段，通用map probing、ECS实体表/容器与scheduler排除并复用依赖。CLI提供已解析输入接口，尚不注册实体。

三版Cargo/PDB依赖排除索引与新指导见[DEPENDENCY_FILTERING.md](DEPENDENCY_FILTERING.md)。B/C构造器局部字节相同、A字段偏移不同；跨版完整建筑语义尚未确认。新源码边界与原指令证据见[建筑发现](evidence/building-state/findings.md)，完整建筑状态/几何/附着/事务仍待联接。

## 建筑生命周期第二批

新增[发布业务字段](reconstruction/building/wall_publication.rs)、[实体关联](reconstruction/building/roof_associations.rs)、[创建与动画组件](reconstruction/building/roof_components.rs)、[删除消费者核心](reconstruction/building/roof_deletion.rs)、[生命周期联接](reconstruction/building/roof_lifecycle.rs)。2316个B原指令案例、128个组合生命周期、384个三版TileSim初始化通过。完整发布/所有权、引擎注册调度、创建其余bundle仍未恢复；精确边界见[生命周期证据](evidence/building-lifecycle/findings.md)。


## 原始建筑输入与 Nani 联接更新（2026-10-11）

当前完整增量与纠错以[原始建筑链记录](evidence/building-gather/findings.md)为准。raw Curve2/512²地形→原发布→cache→逐arch gable→Roof CPU/Nani→真实Bevy删除清理已联接；六帧raw CLI与17项workspace测试通过。新增源码：wall_collection.rs、wall_spatial_index.rs、wall_gable.rs、tg-building-runtime/src/nani.rs。发布中的raw点数/length和Nani enum/subset解释已经更正，不能沿用旧推测。

三版terrain共1536、原空间coords/cells共1536、B原始采集1280、dirty1280、逐arch gable320、Roof StoneFloor CPU1024分别通过限定原指令范围。库容器/派生/ECS实现仍直接复用。完整B创建10组件显式Bundle且required图为空；原fork排程、GPU activation与C新增Nani消费者未验收。墙freeform/砖列/支撑/开口、门窗和完整事务仍是建筑系统后续工作。

上方屋顶铺瓦位精确报告属于原Windows历史构建。当前ARM33个矩形输入仍有case14单条filler坐标1ULP差异，失败报告保留；不得把当前功能整链通过扩大成全部数值位精确或完整游戏验收。


## 2026-10-11 专用skill与屋顶编辑链

native-rust-reconstruction用户skill已创建并校验；按需参考覆盖三版依赖/原模块映射、原指令验证、Bevy链与token控制。具体地址/字段仍留本项目。新增roof_direction/roof_customization，真实Query无删除过滤已由PDB参数确认。方向同值仍通知geometry/gable/sound；Missing解析失败通过fault停止该消费者。Full Flat style与rim分开，后者是replay tag4；等值仍写/tick，变化才通知wall holes。Rim from来自cmd，record1即使同值仍产生history mutation。

B方向1024、B/C customization各1536及各4skip guards通过；八帧CLI及新增5个多帧World测试通过，整workspace22项。Create/Delete/Direction/Rim离散replay实际World分发及Delete undo重建已接；History requests为adapter待处理，不宣称原History commit/cursor/snapshot事务恢复；wall holes仍仅通知未生成几何。全部证据见evidence/building-edits/findings.md。

下一步：RecalculateWallHoles消费者接墙/门窗；History add/commit/cursor和VisualEdit实际World消费者；freeform缓存、墙砖列/支撑/开口。C Flat/Rim两个resolved消费者已执行验证，C Nani及完整迁移仍pending。旧ARM瓦片1ULP失败保留。


## 2026-10-11 开口消费者与Rim事务闭环

RecalculateWallHoles实际reader在墙publisher，以InnerWalls存在检查转OnWallChanged，非直接写hole；当前已接到四类门窗actual raw贡献与WallHoleStorage跨帧/周期/排序/缓存及删除清理。原finalizer caller使用源Curve2长度且shape tag!=2闭合；不要猜3D长度或仅0/1闭合。位置和资产collision为明确resolved输入，原asset/UV/roof-aware morph、其他hole生产者及视觉Bundle/GPU仍未恢复。

原History start/add/finish/cursor和Rim命令展开恢复，真实World完成显式账本→Rim录制→commit→instant undo/redo→分叉截redo→门窗更新。启动账本不默认猜，未配置保持adapter队列；原token Drop不commit。其他MultiEdit owners、快照装载、保存、VisualEdit实际World和timed调度仍未关闭。C Replay判别值与B不同，A历史布局不同。

当前24项workspace测试、十二帧新CLI链；B invalidation512/storage768/opening3072/history462分别通过限定原码范围。范围与三版出处见evidence/building-holes/findings.md、building-history/findings.md。下一步优先asset collision/Decorator宿主投影、其他开口贡献与墙砖裁切消费者，以及其他历史owners/快照联接。C迁移、GPU/fork排程及ARM瓦片1ULP问题仍单列。
