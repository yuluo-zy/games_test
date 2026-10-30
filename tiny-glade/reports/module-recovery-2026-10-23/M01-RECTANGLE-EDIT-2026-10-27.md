# M01 矩形编辑恢复：原实现、接线与验证边界

本轮沿用 B 骨架原目录 `crates/country-core/src/systems/wall/shapes/rectangle/` 及原 `crates/systems/decorator/src/decorator_on_rectangle_edited.rs`。修改既有重构实现，没有创建另一套原版 crate，也没有更动渲染引擎目录。创建/UI 创建及模块根放置朝向由中央/高度 owner 处理；`ui_modes/edit_orientation.rs` 的复杂控件由 freehand owner 协作恢复。

原指令执行对象为 `B版本内容/tiny-glade.rne`，SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。B PDB SHA256 为 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。FunctionSystem 参数以 B skeleton 的实际泛型参数为准，不能以 `void()` 调试占位或旧进度表代替。

## 原函数与业务路径

| 原文件/函数 | 原入口 | 本轮实际恢复 |
|---|---|---|
| `ui_edit_rectangle_dims::ui_edit_rectangle_dims` | `140ad6250` | 原 11 组参数；读取 AppMode/公开墙/墙实体/真实触发体，使用原 tuple hash + 固定盐注册四边控件，运输完整 56B intersection 和拖拽状态。鼠标 hover 发 deferred HighlightWallCmd，不改变 AppMode。 |
| `ui_edit_rectangle_dims::do_ui_edit_rectangle_dims` | `140ad6e00` | 原 12 组参数；取最后事件，Started 建完整 before/after/Wip 锚点，原 3D Slider 投影、2.25..15 尺寸限制、原边界逐步插值，Held 发布 pins/树反馈，结束发 unit Finish。 |
| `ui_edit_rectangle_orientation::ui_edit_rectangle_orientation` | `140ad8000` | 原 10 组参数；未聚焦四角拾取、聚焦时选最近相机角点，拖拽时锁存角点，写唯一 DO 后立即结束枚举。 |
| `ui_edit_rectangle_orientation::do_ui_edit_rectangle_rotate` | `140ad8be0` | 原 12 组参数；射线平面投影、完整 normalize/acos 多项式/π 分支/符号、初始夹角偏移、ShapeVal 时间插值、原正向 0.1 弧度越界步进、真实几何及音频位置/角点位移。旧反编译因 jumptable 损坏漏失的 Held 分支由原 ASM 恢复。 |
| `ui_modes/edit_dims::{widget,signifier}` | `14155fc70` / `14155eba0` | 未聚焦按原 u 范围裁剪墙触发体；聚焦按公开路径 XZ 最近坐标读取真实底/顶高度并建立手柄盒；拥有型箭头/轮廓工厂和活动编辑线状态；cameraMoved 直接控制真实 UiSounds。 |
| `signifier_edit_dims_line::signifier_edit_dims` | `1408dda10`，工厂 `1408de7e0` | 原 4 参数查询真实 Curve2 组件、Mesh、Animation、Transform；由 eased 取中心对称曲线切片，更新真实网格。原工厂先建空 transient Mesh、duration=0.3、禁用额外 Transform 缩放。 |
| `edit_rectangle` | `14097f620` | 原 5 参数；仅最后请求生效，更新权威墙 curve/enclosure，材质音频 kind4 与建筑音频尺寸/旋转分支；原 200/20 速度尺度与 Time.delta。 |
| `finish_edit_rectangle` | `141464a60` | 原 9 参数；每活动帧先发布完整 OnRectangleEdited80B，再按原双 0.01 守卫直接提交真实 History 或恢复 DecoratorStorage；通知提示/羊，结束仅清 Wip 标签。 |
| `system_decorator::decorator_on_rectangle_edited` | `140819c00` | 原 5 参数、消费同一 OnRectangleEdited；使用编辑后的 InnerWalls、首次归档锚点，归档 Wall+Terrain 且带实际 collision 资源；按原主轴重映射、角点附着、墙空间坐标与原 side guard 更新，跨帧不累计偏移。 |
| `ui/resources/slider3d::Slider3d::update` 矩形单态 | `140b1cc80` | 恢复原 Local 状态 + CursorPositionSS/WindowSize/MainCamera 的 SystemParam；复用现有投影叶子。Started 建立属性偏移，Held/Finished 投影并限制范围；直接读相机 +0/+80 矩阵，避免逆矩阵再求逆造成误差。 |

## 载荷与状态合同

- `EditRectangleCmd` 保留墙与完整矩形，补回原 position Vec3、speed f32、rotation bool。speed 是原相邻尺寸/角点位移，消费者再除 Time.delta。
- `FinishEditRectangleCmd` 原 slot 只有 8B EventId，因此是 unit 信号。before 不在 Finish 中，不能伪造带 wall 的完成事件替代。
- Wip80：tag1=旋转，wall@16、before@32、after@56；tag2=尺寸，wall@8、edge@16、before@24、after@48。保留完整 before 和分支专属锚点；结束只写 tag0。OnRectangleEdited 是完整80B快照，原EventId位于slot+80。
- `UiEditRectangleActiveWidgetResource` 原记录为 Curve2 + WidgetId@56 + Vec3 origin@64。factory必须按 WidgetId 稳定命名，origin每帧更新；不能按 origin 不断新建瞬态网格。原 WidgetId Debug 是 `WidgetId { hash: ... }`，由 `1408e0e30` 与原字符串 `142aa5010/18` 核实。
- 两个原代理 `RectangleDimsEditInput`、`RectangleOrientationEditInput` 已删除。DO 事件都保留完整56B intersection，不把屏幕光标坐标直接当作世界坐标。
- pins 使用中央/GPU owner 恢复的真实 Rectangle union 构造器；不再以 show=true 代替矩形空间触发。

## 本轮纠正的算法误读

1. 原八项 acos 系数的首项是 `baa57a2c`，随后为 `3bda90c5 bc8bfc66 3cfd10f8 bd4d8392 3db63a9e be5bbfca 3fc90fda`。旧实现遗失首项、符号和指数错误，普通旋转角就已偏差；原指令差分发现后按原 PE 修复，没有放宽比较。
2. 装饰物普通锚点不是“两轴同时 clamp 到当前矩形”。原算法先用 before 朝向逆矩阵投影首次归档锚点，按相对半尺寸选主轴，仅主轴随当前尺寸/中心变化，再用当前朝向映回。原 unordered blend 将归一化 NaN 取下限 -1；零尺寸不能早退。
3. 装饰物 u 的原 `14081a165/277` 是 MULSS：normalized u × perimeter，产出墙空间距离。旧代码误除 perimeter。角点分支 coord.y 写 archive anchor.y；普通分支保持原 coord.y。
4. 只看 decompiler 中比较分支会误读 NaN：audio 和 Slider 的 MAXSS/MINSS 原位型必须核 ASM。finish 则只有两个有序 `<=` 同时成立才恢复；NaN 走提交。
5. 原 `1412dce20` 由完整泛型函数名确定是 deferred `HighlightWallCmd`，不是切换编辑模式。
6. `extract_and_restore_to_storage` 原 `140b2c4b0` 清碰撞并 drain 归档桶，不清 culling 集；culling 清理由 `extract_diff` 承担。本轮测试按原边界区分二者，没有为了测试修改共享 owner。

## 验证

- `m01-rectangle-anchor-native.py` 实际执行原 `140819ef6..140819f5e` 和 `14081a174..14081a230`，不桥接业务算术；320组 before/after 朝向、中心、零尺寸、NaN 锚点逐位通过。排除 ECS/hash/allocator，只声称已解析对象的原数学片段一致。
- 同脚本执行 `140ad9472..140ad95a2`，覆盖完整 normalize、dot、acos、π 与 sign；49组 exact f32 位型通过。该片段没有 CRT 三角函数调用；不把它扩大成跨平台 sin/cos/expf 位一致性证明。
- country-core 的尺寸及旋转真实 World 测试覆盖 UI producer→DO→权威 apply→finish：Started/Held/Finished/连续空闲帧、原音频和 pins、真实历史编辑。尺寸还覆盖序列化/反序列化和权威墙删除后的公开墙卸载。
- 动态编辑线真实 World 覆盖原工厂 wake、稳定 WidgetId 身份、同 MeshHandle 更新、0.3s/禁二次缩放、完整曲线以及实体/TransientMesh 回收。
- system-decorator 的真实 World 测试覆盖两帧同一首次归档、当前 InnerWalls、无输入帧不重复修改、真实存储和全局索引恢复；碰撞 payload 实际持有/释放的共有 owner 由已有 native-physics 归档测试单独覆盖。

最终命令、结果及源码摘要见本报告后续验证记录。CPU 合成 UiInteraction 没有发送 OS 输入；本轮不据此声称真实鼠标键盘验收或视觉终验已经完成。生产完整调度/本机GPU联合验收由中央及GPU owner负责，原 get_rectangle_orientation 闲置错误 helper 交高度 owner独立复核。

## 后续复原经验

以原 FunctionSystem 泛型恢复参数及事件身份，再追 writer/reader 字节；先拆清框架循环和私有算术，不能抄丢失 jumptable 的伪码。浮点优先执行原指令，保留失败case并核常量/源操作数，禁止用 epsilon 掩盖。动态视觉需核 factory身份和captured owner、创建到回收的多帧路径，不能只手动 wake 一个工厂后宣布接线。使用公开缓存的借用最近坐标内核，避免为了计算一个手柄高度而每帧复制整条曲线。共享测试并发写入报告前先确认文件 owner，原 JSON 不能被另一个用途的证据覆盖。

## 最终验证记录

`CARGO_INCREMENTAL=0 cargo test -p country-core -p system-decorator --offline --lib rectangle` 最终退出0：country-core 43项通过，system-decorator 16项通过。该过滤包含其它矩形几何测试，不能将59解释为独立业务覆盖率。源码、Cargo.toml/Cargo.lock在测试前后逐文件SHA256相同，收据为 `m01-rectangle-final-validation.json`，完整日志为 `m01-rectangle-final-validation.log`。

尺寸及旋转 World 夹具最终通过实际 `create_rectangle` consumer 建立墙、屋顶创建请求与创建历史；开始下一输入事务后，经历真实 UI/DO 编辑、音频、finish、历史，并用实录 MorphPremadeShape 送入原 WallReplayCommand 正反两向，回写权威源后重新发布公开墙，逐字节核对 before/after enclosure。未变更完成另测恢复分支、提示/羊事件不发、仅清 tag 而保留 inactive 字节。动态线在最后一次修改中复用现有 `update_curve_mesh` 更新属性和索引，保留 Mesh owner/其它属性，避免每帧丢弃整个属性表。

错误闲置 get_rectangle_orientation 已由高度 owner 删除，实际创建两路朝向保持原算法并完成独立864例对照：非NaN/负零及带payload NaN位相同；有限零向量产生的 invalid NaN 仅符号在ARM/x86间不同。详细边界由 `14155d410-矩形创建朝向复核.md` 保留，不能将该864例说成全部位一致。

本owner的待验边界是中央完整调度、本机 GPU 画面/窗口资源长稳、真实输入验收；本轮没有发送OS输入。各已恢复函数的 CPU 算法、原事件接线、编辑历史正反方向、实际装饰归档及动态线卸载均已有对应测试，不能用这些测试替代整游戏视觉验收。


## 最终收据冻结（2026-10-27）

`m01-rectangle-final-validation.json` 记录两 crate 的矩形筛选验收：country-core 44 条、system-decorator 16 条全部通过，测试期间全部被测源文件与清单 SHA256 未变化。维持真实 World 创建、跨帧尺寸/旋转拖拽、历史回放及卸载覆盖；未做 OS 输入或宣称 GPU 画面验收。

末审额外恢复原尺寸箭头工厂的零动画时长，以及 flat/outline 各自的 world/UI pass；原 DecoratorMaybeMut::get_mut 的 modified_ids 副作用显式落在 side=0 更新分支，side=1 不标脏，避免装饰几何更新后画面不重建。直接分离借用并遍历原存储，移除每帧收集 ID 的分配。原侧 320 个归档锚点和 49 个有符号旋转角样本继续逐位对照；未放宽浮点差分标准。
