> 后续原始采集已证实并更正早期解释：PublicWall+0x10=raw Curve2点数，+0x30/+0x28c=raw长度；AtlasPart0与subset kind9替代早期rank/drawable推测。本轮raw采集、逐拱段gable、Nani注册与实际World进展见[最新记录](../building-gather/findings.md)。旧段落保留其当时范围，不能视作当前缺口清单。

# 建筑业务恢复增量：2026-10-11

本轮按用户要求执行 [依赖排除指导](../../DEPENDENCY_FILTERING.md)。业务实现限定于 B，A/C用于依赖来源和构造器布局对比；所有原指令差分绑定 B RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。Ghidra原函数读取复用已登记的patched EXE数据库，只读、无自动重分析：[focused/report.json](B-focused/report.json)。

## 三版筛除依赖

对照三版Cargo与PDB，[dependency-exclusions.json](../../tracking/dependency-exclusions.json)登记 B/C 分别849/907个外部来源package直接复用，无source条目共52/53个，其中已确认SDK/库也直接排除，当前仍需核实归属的是47/50个；A缺少锁文件，使用3238条registry及460条git源码路径明确来源。合计854/910个package登记复用。package/source条目数不等于函数数，不计入业务恢复覆盖率。

291条建筑私有源路径登记为业务候选。共有124条私有源路径在所出现版本具有相同checksum，仍保留为私有候选，不能因相同就排除。例如B/C valid_enclosure/mod.rs相同，仍属墙体业务。函数定义在Bevy/serde/std/容器等库时，即使泛型名称含Wall/Roof，也只用于接口/调用取证，禁止写替代实现。

PublicWalls::get/roof_entity反编译中大量哈希探测属于库实现，本轮排除，未编写哈希算法或Bevy实体/archetype表。恢复的是成功匹配后的游戏检查和已解析对象上的游戏状态更新。外部接口以明确的调用者边界保留。

## 新增Rust模块与原作范围

| 原游戏规则 | 当前Rust实现 | 原指令范围/结果 |
|---|---|---|
| PublicWalls::get成功读取检查 | [wall_state.rs](../../reconstruction/building/wall_state.rs) | 0x140a91fbe之后：观察计数>=2，height>0（NaN失败）；count原字段名未知 |
| wall cache/shape/style业务投影 | 同上 | max_y flag+0x108/value+0x10c；shape+0x258..0x274；style+0x299；is_halftimbered原叶函数byte>=2 |
| update_roof_spatial已解析更新 | [roof_spatial.rs](../../reconstruction/building/roof_spatial.rs) | 0x1421a7980..0x1421a7c28，保留原游戏调用与有效预分配队列；2659案例通过 |
| create_roof初始化 | [roof_creation.rs](../../reconstruction/building/roof_creation.rs) | 0x1421ab0f6..0x1421ab26a，停止在后续组件初始化/spawn前；848案例通过 |
| 原回放到创建再到墙更新联接 | 上述模块加roof/replay.rs | 512案例：Create正向与Delete撤销→原事件→Roof初始化→更新墙状态→Roof/附属组件/有序通知 |

[空间更新报告](B-spatial-verification.json)覆盖35个读取guard、1600个状态边界及1024随机输入，Roof88、related48、change ticks和通知字段逐字节一致。缺失cache/shape特殊分支按报告范围检查，源SSE异常标志未验证。

空间更新细节：每次成功解析都写Roof高度/标记及related组件标记；shape tag0比较4个浮点、tag1比较6个，ordered equality把正负零视为相等，NaN使其不相等。tag>1跳过shape替换。样式和形状同时变化各自产生一条kind2通知，顺序保留，不去重。Roof::ty为true时related+16..28写零；为false时写pivot，其中y加bits0xbf30a3d7。related的完整原类型与其他字段未命名。

[创建报告](B-creation-verification.json)检查创建事件到Roof88字节、类型和gable时的wall-ID通知；原创建记录+8为wall ID，+16为4字节外观参数，+20..32为12字节外观数据，+32..60为Roof参数。外观完整语义及JSON缺失默认值仍未确认，只复制已观察字段。Roof+0x55..0x58保持调用者字节。最初报告误比较bool返回寄存器上部残留导致624例失败，纠正为ABI定义的低字节后848例通过；没有修改原码或候选结果来忽略有意义的差异。

[联接报告](B-chain-verification.json)使用原指令生产create事件、原指令初始化、原指令空间更新作参照。参数和wall形状是确认布局上的合成输入，不是真实场景加载；缺失JSON字段未补造。[CLI报告](B-cli-verification.json)另外核查两个真实入口与两种非法输入，大u64 ID用字符串保留。

## 三版布局比较

[constructor-version-comparison.json](constructor-version-comparison.json)按三版登记原符号只读读取构造函数到首个return。B/C这一构造器字节完全相同；A后部高度/标志字段在+0x40/+0x44/+0x48，B/C在+0x4c/+0x50/+0x54并复制额外12字节到+0x40。不能把B的88字节对象布局直接套给A，也不能把构造器相同扩大为B/C整个业务等价。各版参数生产者与完整类型仍需核查。

## 构建与边界

当前workspace测试和release锁定构建通过，新增CLI `building-roof-create` / `building-roof-update`；均显式标记resolved_only，创建不宣称已spawn实体。原15个registry依赖保持不变；未增加或重写底层库。此前VisualEdit/离散回放/历史联接/参数/解码在新工程下已回归，报告含当前Rust源码和Cargo.lock哈希。

当前还未恢复：PublicWall发布及ID/实体关联生产者；TileSimulation、TriggerVolume、Animation、drawable等创建后段；ECS注册与实际事件调度；删除动画/移除、方向/样式消费者；Flat几何；墙轮廓/砖列/支撑/开口及门窗；完整历史事务/快照；场景/GPU。B旧曲面、铺瓦全链报告仍是历史构建证据，当前workspace待重验。完整建筑系统与完整游戏尚未验收。

下一步沿publish_public_wall_state、create后段游戏组件、delete生命周期和墙构造消费者推进；仅恢复游戏控制流及业务参数，底层依赖复用。
