# M01 屋顶装饰合并提议恢复，2026-10-27

完成 dormer 瓦片冻结后，按 parent 授权继续实际 Place/Intent 依赖。先读 B `crates/systems/decorator/src/merge_proposal_roof_decorators.rs` 骨架、同名当前文件、原调用者 `calculate_decorator_interaction_intent14127cf70` 的完整四分支。业务只修改既有同名模块；Intent 和 wall merge 由 move owner 负责，未建目录或平行架构。

## 否证及修复

旧实现的 `RoofMergeShape`/`RoofMergePosition` 外部角点输入与两盒 `intersect_roof_decorators` 并不是原完整合同。已用借用唯一资源的 typed API 替换：输入真实 Decorators、DerivedDecoratorInfoView、CachedDecoratorTransforms、Roof、候选位置/朝向/屋顶坐标与原独立排除 ID。原调用方已接同名新 API，实际模块联合验收仍由 root/move 收口。

| 原函数 | 原 VA / PDB code_size | 当前归属 |
| --- | --- | --- |
| intersect_roof_decorators<Intent closure1> | 141fe7280 / 726B | 真查询盒和全桶命中序列 |
| intersect predicate | 141fe6f20 / 863B | roof_decorator_intersects + 私有几何 helper |
| find_best_roof_decorator_merge_position | 142000e60 / 2592B | 原命中顺序的完整 min 迭代 |
| first candidate closure | 141fe6a40 / 1247B | 私有 propose，与主迭代内联算法合一 |

RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。`m01-roof-merge-*-native.asm` 保存原完整代码范围；对应 C 仅作辅助，Ghidra 把 predicate body 识别成136B并漏间接跳转分支，不能按其size/伪返回认为业务只有136B。原完整 LLVM 范围与原机器码直接执行覆盖全部863B分支，没有写 shared 数据库。

## 原完整业务

候选查询盒中心是原世界 position，尺寸(width,height,.1)，轴来自原 quat*X 的XZ归一。逐桶记录先排独立 ID，随后 dst tag2/4进入、tag3原panic、其它false；此门早于同族。族是 ty.wrapping_sub(0x38) 若小于8原样保留、否则6，**0x3f对应7，不能min(6)**；族6才额外要求type全等。

通过族门后，严格按原顺序读取 required CachedTransform、subtype、collision。既有盒中心为 cached translation + 世界Y半碰撞高度 - quat*(0,0,0.0990000069)；尺寸(width+.4,height,.3)，轴同 quat*X。不是整个局部Y偏置旋转，也不使用 cached scale 放大逻辑宽度。调用真实 Rectangle3d constructor/overlap，命中返回独立 map ID；不存在缓存/子类型/mesh继续失败，不能用默认盒。

提议迭代对输入命中 ID 逐一 required 查桶；旧 missing candidate 跳过已纠正为原 expect 失败。先查目标 subtype/collision，再判断族与原 widened rank和<=max_rank；不使用 u8回绕。合并记录复制完整 incoming72B，只替换 rank，目的/旋转/未知字节均不提前写。

宽度差是 **该既有 subtype 的 merged rank 碰撞宽度 - existing rank 宽度** 的一半，旧 incoming-width 参数已删除。以真实 position查询 Roof 截面：Roof首shape字节&1表示矩形，**不是corner_attached**。矩形用真实截面的最近边，delta/edge_length*.25；圆用真实周长，不包一圈、不改成欧式世界距离。base.u<new.u时base+delta，其余base-delta，旧实现方向相反。目标 dst_as_roof required，取其coord.y与yaw；计算原 dy²+dx² sqrt。

输出 RoofDecoratorMergeProposal 保存全部定义字段：rotation、coord、完整合并Info、targetID、distance。原native112B载荷的16..32为未写padding，没有用padding造业务字段。原tag4 None由Rust Option表达，与Info.state==2无关。第一个可行NaN距离可原样返回；只有出现实际比较的NaN才unwrap失败。等距保留先遇到候选。原 proposal 内**没有另一次重叠过滤**；完整过滤在 Intent 上游 intersect。曾把旧反编译空谓词解释成proposal缺过滤，现以完整原父子链纠正。

## 原指令差分

`m01-roof-merge-native.py` 直接映射身份绑定原RNE，执行完整142000e60及141fe6a40；仅在明确的内联hash查找边界返回实际解析到的目标，subtype/mesh/cache在原getter桥接。没有仿造Hashbrown或Bevy内部容器。Roof clamp/真实noise/Shape/周长/最近边、类型/rank门、完整Info复制、符号选择、min及panic均执行原指令。

237组：circle/rectangle、族0..7/溢出、rank0..4和、左/右/相等、零/一/多候选与等距顺序、单候选NaN/多候选NaN、缺目标、缺碰撞、非roof目标。84个Some逐位比较yaw/coord、完整72BInfo、targetID、distance；147个None与6个原panic同样核对。

另288组执行原141fe7280完整查询盒构造前缀，以及完整141fe6f20谓词，覆盖独立排除、dst各分支、族与单位/Yaw/Pitch四元数及近/远查询。双方32B完整盒记录逐位比较，不能只用最终6个true/258个false掩盖中心和朝向偏移；24个原panic保持。资源查询桥之外，Quat归一、Rectangle3d构造及overlap为真实原指令。

独立 harness 直接编译现有生产源和同target已构建country-core/utils/glam/bevy_ecs，3/3通过；构建依赖实际metadata选择与命令记录在 `m01-roof-merge-leaf-build.json`，日志 `m01-roof-merge-leaf-tests.log`。这是叶级验证，不等于整包/整游戏验收。正式 `CARGO_INCREMENTAL=0 cargo test --offline -p system-decorator --lib roof_merge` 最终 **3 passed / 0 failed**；见 `m01-roof-merge-final-system-tests.log` 与绑定源码/依赖指纹的 `m01-roof-merge-validation.json`。

真实World测试使用同一 DecoratorStorage.apply_decorator_place 建立桶和d2s、真实Subtypes/MeshLibrary/CachedTransforms、真实Roof组件：实际intersect→proposal、保留独立ID与完整Info、稳定重复、移动cached pose后无命中/提议、apply_decorator_remove后无旧候选。未在此叶擅自手写合并历史操作；完整Intent/Apply/History由原消费者owner统一验证。

CRT桥明确为 Darwin C float 数学函数，不宣称Windows CRT主体逐位等价；原hash资源查找/分配失败与原调度均不在native执行证明范围，Rust真实资源测试补接口。没有OS输入或GPU完成声明，未作整游戏性能基准。

## 过程经验

- 原查表处的寄存器保存也是边界合同：最初直接跳到resolved hash命中落点跨过了RSI=输出指针，native能正常返回却把提议写到其它位置，产生全零假结果。必须保留跨过指令、检查定义输出、断言原RIP返回哨兵，再接受oracle。
- 原DecoratorLibKey返回AL=subtype、DL=rank，按错误字节桥mesh会让所有rank共享宽度，delta恒零而看起来全绿。输入必须让merged rank宽度变化，并检查实际位移方向/数值。
- Ghidra短body/间接switch失联不等于业务少。此处保留fresh partial C，依靠完整LLVM+原指令执行恢复，不向共享数据库写修复。
- Info业务word与独立map ID不同，Roof矩形tag与corner flag不同，原Option tag与Info.state不同；只靠旧字段名会把三个业务分支都接错。
- 容器与getter可桥接已解析对象，业务分支和数值不能桥成预期输出。正式测试与全链验收分开记，邻owner中间态编译失败不通过改其源/默认逻辑绕过。
# 后续共享截面复验

`section_shape` 已仅窄改为 `pub(crate)`，供原屋顶 split 分支复用相同真实截面，算法无变化。`m01-roof-merge-shared-section-tests-v2.log` 最新4项通过，含原3项merge算法/World与另一个owner新增的parent Intent真实World整合；源指纹和本次边界追加在 validation JSON 的 `section_visibility_follow_up` 中，保留旧3项历史收据。
