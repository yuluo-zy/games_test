# M01 装饰移动主系统：原接口、恢复过程与验收

本轮把既有 `crates/systems/decorator/src/ui_systems/ui_move_decorator.rs` 的八参数结构桩替换为原 B 的移动主系统，保留手工重构目录与 canonical 存储、物理、历史、蓝图和指示符 owner。没有改动渲染引擎或宿主；墙和屋顶拆分分别复用 circle、rectangle owner 的真实实现。

当前 CPU 验收覆盖真实系统多帧编辑、越界回滚、结束提交、种子点击、楼梯续建，以及实际归档差分的撤销/重做。GPU、OS 输入和全游戏历史账本验收由中央链继续处理；不能将下表解释为整个主机链已验收。

## 原版身份与 B 目录

- RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 匹配 PDB SHA256：`2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- 原 B 路径实际读取：`skeleton/B/crates/systems/decorator/src/ui_systems/ui_move_decorator.rs`；SourceSHA `357534ae787ab6f22870376a6eadef5a4fe12a7d749d992d3b0beae62b68dd5c`。
- 原主系统 `1407a4850 / 12790B`，应用闭包 `1407a7a50 / 1069B`，自动杂物提示闭包 `1407a82f0 / 466B`。
- 必要旧 owner：`ui_systems/move_ops.rs` 的 `14127fe10 / 527B` 与闭包 `141280020 / 457B`；原 `utils/src/lib.rs::xz_projective_rotation_from_forward` 为 `140c91640 / 619B`。
- 投影复用原 `140ac8780 / 174B` 的两级点变换，不合并矩阵改变舍入。

七份 LLVM 身份绑定整段证据保存在 `m01-move-*-full.{json,asm.txt}`。`m01-move-evidence-report.json` 记录原字节核验与实际开销：7 requests，8 analysis processes，2 version processes，首次缓存命中 0；采集约 0.675 秒。取证不是行为验收。

独占 worker 中以原 PDB 边界修复两张四项跳表：相位跳转 `1407a582c / 142a829d8`，目标域跳转 `1407a64b5 / 142a829e8`。完整反编译、修复任务和原参数表分别保存为 `m01-move-private-cfg-repaired.c`、`m01-move-private-repair-job.json`、`m01-move-original-systemparams.txt`。未更改共享分析库；独占 worker 已正常保存并停止。

## 业务合同与修复

### 实际输入与状态

恢复原十六组 `SystemParam`：Time、系统 Local/Modifiers/初始化事件、五资源 RaycastResources/真实光标、存储和归档/蓝图库/楼梯/模式、墙与三种 ECS 查询、延迟历史、位移速度、正式 ID、缓存、真实 UiState/stream、音频、九资源备份、边界、自动杂物、RaycastWorld、四种教学事件。stream 使用安全 `ResMut`；demo 通过 required `Res<DemoModeEnabled>` 读取唯一资源，没有平行门禁或缺失资源静默视为false。

原 `Local<Option<MoveDecorationInteractionState>>` 保存部件、约束高度、抓取偏移、独立 ID、垂直偏移、抓取点和初始基高。原16字节旋转缓存保存相机逆水平旋转下的完整四元数。旧全局三字段 Resource 与 `cached_yaw` 不能代表这些状态。

输入原字节：2 悬停、3 开始、4 持续，0/1 共同进入结束分支。旧注释把4当释放，会改变移动、备份及提交时机。首步无条件取走上帧有效移动点；缺请求或 UI 被抢占也遵守该顺序，不在门禁外清零所有速度。

### 开始、移动与结束

- 悬停从真实缓存和存储定位装饰，按楼梯高度/命中位置选择顶部或底部握点；通过实际相机投影、带屏幕偏移的射线、墙触发体 UV 求抓取偏移。查询/几何缺失沿原失败；不以零向量或默认墙替代。
- 开始时缓存完整相对四元数，发送原拾取音/拆分教学事件；九 owner 备份未创建时只请求 CREATE 并返回。
- 持续移动按 Wall/Roof/Terrain/Stair 四域处理朝向、原相机正反面门、楼梯高度约束/禁用吸附、真正 Intent/merge 与已恢复拆分函数。
- 拆分结果 Vec 长度非零的开始帧只重新校正抓取偏移并发送 WindowSplit，随即返回。返回 `+18` 一度被疑为 Option，经调用点和被调用者独立核验确定是墙结果 Vec 的长度；没有添加假字段。
- 屋顶拆分返回完整目的，主系统用实际 Roof/TriggerVolume 重建其位置，有真实 UV miss 时才使用原姿态回退；不私自向缓存写入非原数据。
- 地形越界请求原边界显示和反馈、恢复九 owner 最近有效状态，速度归零。普通拒绝走 RESTORE/图标。无支持目的走原删除指示符及 Remove，未用不消费的事件代替。
- 结束的小位移按原类型规则改变种子，或设置完整楼梯 endpoint 并发送原19字节初始化状态。初始化 `+8` 是垂直偏移的 f32；已与 Place owner 同步成 `height_offset`，没有冒充 ID。

### 独立源地址、目标地址与历史

原 Accepted 同时有源地址 `+00..0f`、目标地址 `+10/+18` 和目标参数 `+20...`。旧 typed 输出只有源地址；Wall 候选也误用了源墙，因此同墙测试掩盖了跨域缺口。

现 `AcceptedDecoratorIntent.storage` 保持原源地址，新增 `target_storage` 表示真实目标。Wall 目标参数使用目标墙；Roof 也保留独立目标墙 ID。Place 与 Move 均使用此共享合同。原 Roof Intent 派生碰撞时仍读取原 Info/源地址，没有先改 Info 为 Roof；专门补了原 Place 的 WallCompound 基线加独立 Roof 候选回归，避免调用者制造目的掩盖原算法。

实际应用闭包先归档源/目标，合并先移除移动 ID，再修改目标 ID；普通移动更新独立定位索引。额外楼梯字段、rank、原事件位置和 preview state 分开处理。真实 CPU 测试从此 consumer 生成跨域差分，再调用 canonical `restore_decorators` 两个方向，检查存储、ID 索引及历史语义状态，并检查合并的 Remove→Change 原事件顺序。这是实际差分回放验收，不等于工具栏历史按钮和全游戏事务账本已验收。

### 安全瞬时 ID 与提示生命周期

`TransientDecoratorIdGen<'a>` 现独占借用正式计数器；`DecoratorIdGen::transient` 返回栈上生成器。preview 不消耗正式 ID，结束时直写相同计数器，保留 wrapping 与下游失败前已经发生的分配副作用。删除了原安全函数持有任意裸指针的接口，全部实际调用者已核对并迁移，没有 copy-base/事后 reserve。

删除指示符旧实现持有返回后释放的 Arc 裸地址，并把假 Mat3 前四项当四元数。现按原完整 Quat、`decorator_delete_scribble`、零动画时长建立 owned factory。自动杂物提示恢复实际 bounds/BackOut 圆形描边及 `link` 世界空间图标。两类工厂均实际唤醒、建立实体并验证 despawn/Arc 释放。

## 验收收据

| 范围 | 结果 | 限定 |
| --- | --- | --- |
| 原 RNE `140c91640` 完整619B | 128组 | 全输出有限字严格bit；非有限只按NaN分类 |
| 原 RNE `140ac8780` 完整174B | 128组 | 两级矩阵/输入/输出原顺序 |
| 原主系统四元数基本块 | 128组 | 原地址执行，明确块边界，不声称完整主系统 |
| 原主系统握点选择基本块 | 108组 | 包含高度阈值、顶底、NaN及原as_stair访问 |
| Rust `original_move_` | 10 PASS | 含本轮9项及既有move_dst1项；0 fail/ignored |
| utils 完整 lib | 183 PASS | 当前共享源码实际总数，0 fail |
| core安全瞬时ID（native-physics） | 1 PASS | preview/overflow/失败前副作用 |
| Intent原合同回归 | 3 PASS | 128完整原Stair样本与真实四域/合并/源目标门 |
| 共享apply消费者 | 11 PASS | seed策略修复后实际现有全部tests |
| Rotate完整原函数夹具 | 1 PASS | 90原样本×2真实demo上下文；180次ECS执行 |

`m01-move-math-native.py` 可独立重建并经 Rosetta 直接执行492组原指令；RNE/PDB 与代码 hash、原输入输出、Rust vectors 均持久化。没有为通过测试扩大误差。

保留首次失败：迁移夹具先把 Roof Info 放入 Terrain 源，触发原 `as_wall` guard；改为实际可存在的 Terrain clutter，而未削弱 guard。越界夹具的30000屏幕像素实际仍落在65米内，旧断言因此失败；改为先断言真实世界命中越界的100000像素，再检查备份回滚，未改变生产边界。

### Seed 的 demo 上下文与回放边界

原982B `apply_decorator_change` 的 `140a20a9a..ab0` 先以有符号delta更新Info.word2c，再读取 `143634d90`，写 `u45=(!demo)*5`。修复前typed consumer固定5，现按显式上下文写0/5；没有在UI调用后直接改存储。唯一的两处活动seed写者是 Move 与 Rotate，后者也迁移了真实资源参数。

`DeferredDecoratorOpEvents` 是已应用后的通知：原 `unleash_decorator_op_events` 只排空并写EventWriter，现读者为音频、楼梯组装与Modifiers门禁；没有再把seed事件交给apply重复执行的producer。因此不在原事件中编码demo上下文。历史使用完整 `DecoratorHistoryState.u49` 保存u45，并由 `restore_decorators` 读前/后快照直接重建Info，不重新按当前demo运算；真正回放测试检查其保留。backup也复制完整Storage及事件快照。

保留demo回放首次失败日志 `m01-move-demo-first-failure.log`：原seed测试仅把clutter的ty改为Flag61，保留style0；历史to_info按Flag规范成style5，导致无效夹具的扩展槽比较失败。现夹具同时设置真实 `DecoratorExtraInfo::Flag(FlagExtra(0))`，没有修改原历史转换或减弱状态断言。Rotate原90个完整函数夹具按两个真实demo上下文执行共180次，原活动union字节及Info其余72B比较继续保留；新增上下文不声称重新采集180份原函数输出。

## 本轮仍需中央处理的边界

1. 实际 GPU/宿主与 OS 输入由中央统一，保留用户暂不进行真实输入验收的约束。本轮不操作窗口、不改宿主和渲染线程架构。
2. 全游戏历史事务选择、真实保存重启、完整渲染和物理更新仍需联合验收。本轮差分重放不代替这些结论。

## 下一轮应复用的经验

- PDB 类型名与 ICF 主符号都不是实参语义。原 `StructureType::is_fence` 同址函数实际读 Modifiers.disable_snapping；常量 getter、rank1、demo 全局门要沿调用者逐槽区分。
- sret 的 Vec cap/ptr/len 必须对照消费面和释放面；不能见到非零 qword 就生成 Option。
- 同墙、同域输入会掩盖源/目标定位器丢失，必须同时检查 Info、独立 map key、bucket、事件和回放两侧。
- 原全函数机器执行与局部块执行分别计数。固定原 PE 地址重放纯数值块可省下大量容器 hook，但不能据此宣称整个主系统差分。
- 本机 ARM Unicorn 最小 NOP 即退出132，不应把这个环境失败解释成原业务错误。本轮使用已有 Rosetta 原 PE 执行桥，保留原地址、实际RET哨兵和失败出口；没有重写算法当oracle。
- 新增共享字段需立即通知并窄修真实调用者。不要为旧日志中的编译错误新增假 Default、错误重导出或重新引入裸指针接口。
