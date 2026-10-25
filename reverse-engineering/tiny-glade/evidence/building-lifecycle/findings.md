> 后续原始采集已证实并更正早期解释：PublicWall+0x10=raw Curve2点数，+0x30/+0x28c=raw长度；AtlasPart0与subset kind9替代早期rank/drawable推测。本轮raw采集、逐拱段gable、Nani注册与实际World进展见[最新记录](../building-gather/findings.md)。旧段落保留其当时范围，不能视作当前缺口清单。

# 建筑生命周期业务恢复：2026-10-11

原指令差分绑定B的RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。复用现有Ghidra patched-EXE库只读取证：[B-focused/report.json](B-focused/report.json)、[B-association/report.json](B-association/report.json)。ABI未确认处提供观察投影或明确的依赖接口，不编造原Rust布局。

## 本轮恢复范围

| 业务入口 | 已恢复Rust | 原作范围及边界 |
|---|---|---|
| publish_public_wall_state | [wall_publication.rs](../../reconstruction/building/wall_publication.rs) | 恢复数值规约与已确认字段发布：相对高度min初值24、位置y min初值100/max初值0；额外字段按flag限0.3..2.5或0.5..14；发布失效两个max-y缓存flag。dirty来源采集、terrain采样、Arc/几何资源组合仍未完成 |
| wall_roof_hashmap | [roof_associations.rs](../../reconstruction/building/roof_associations.rs) | std::HashMap接收已解析Query rows，恢复形状/缺失墙清理、重复关联约束、阶段13/14限制、wall presence变化通知；排除hash probing和Bevy查询遍历，完整producer外围未原码整函数验收 |
| create_roof后段组件 | [roof_components.rs](../../reconstruction/building/roof_components.rs) | TileSimulation24字节初始化（仅写0..21）、Animation两通道零初值及增长/淡出规则；TriggerVolume确认语义空初始化，原三容器元素类型与ABI未恢复，使用Empty初始化计划，不制造Vec头 |
| mark_roof_for_deletion / delete_roofs | [roof_deletion.rs](../../reconstruction/building/roof_deletion.rs) | 以wall ID匹配请求、每个query row单次标记、pivot及sound kind1；已标记且动画第一通道有序零才请求despawn。实际Commands flush/despawn由引擎处理 |
| 创建→动画→删除联接 | [roof_lifecycle.rs](../../reconstruction/building/roof_lifecycle.rs) | 明确区分发出删除标记意图、引擎提交标记、动画更新、最终移除意图；不实现虚构的即时调度 |

所有上述模块使用标准HashMap/Vec/浮点API。没有恢复容器、Arc引用计数、Bevy实体/archetype表、Query遍历、Commands队列内存或分配器实现。数值更新保留原次序与已确认的x86 invalid-NaN兼容边界。

## 已确认的游戏规则

- 活屋顶关联到有效circle/rectangle墙时，同一wall ID重复关联会失败。已标记删除的屋顶允许替换关联值；有效墙形状下仍会保留关联，直到实体真正移除后再刷新。
- 找不到墙或墙形状不再支持屋顶时，已标记者继续删除；未标记者发出清理请求，阶段13/14触发原panic约束。缺失墙不是统一MissingWall错误。缺失与形状失败分支均执行原指令验证。
- 发布字段复制原shape/style/height，重新发布清除max-y/max-y-gable缓存有效标记。不能把新发布状态直接当成缓存已计算；缓存生产者仍待恢复。
- 动画未删除时，非flat第一通道速度bits0x4036db6e；flat乘2。第一通道>bits0x3f7d70a4时第二通道才增长，速度bits0x40555555，同样乘flat系数。删除时两通道分别dt*(-4)、dt*(-10)，各自限制0..1。
- 已标记且第一通道==0（含-0）才请求despawn；NaN不等于零。删除请求重复出现不重复标记同一已解析query row。

## 验证

[ B-verification.json ](B-verification.json)共有2316个原指令案例：Tile初始化256、动画/移除864、关联约束288、缺失墙12、发布数值512、请求匹配/pivot256、已确认发布字段投影128。发布投影只比较登记的字段，同时检查候选没有写未确认字段；不验证整个672字节对象的库容器所有权。Association的HashMap返回值和despawn API以报告声明的库边界提供/捕获；业务条件和数值段执行原指令，未写库替代算法。

[B-cycle-verification.json](B-cycle-verification.json)128例组合原创建初始化、动画增长、删除匹配、已提交标记后的淡出与移除意图。额外2项Rust集成测试检查删除过程中关联保持、移除后仅一次presence通知，以及发布后缓存失效/未确认字节保留。这些测试不是场景或真实引擎注册验收。

当前锁定workspace测试共4项（原history2项+建筑2项）通过，release构建通过，registry依赖仍15包且全部匹配B原锁version/source/checksum。此前参数、回放、历史与Wall→Roof链已经在新workspace回归。所有当前报告包含Rust源码及锁文件哈希。

## 三版比较与依赖筛除

[version-map.json](version-map.json)按私有PDB路径与定义namespace映射发布、关联、组件和删除7个函数族。B/C的roof_animation.rs与roof_trigger_volume.rs源checksum相同，仍是游戏业务，不因共享而筛除。

[three-version-component-verification.json](three-version-component-verification.json)实际执行A/B/C小范围原指令：每版128个TileSimulation初始化与当前Rust逐字节一致，共384例；每版另核查Animation::new零初值和5个is_being_animated判定。该结论只覆盖这些叶函数，不证明三版完整发布/动画系统/删除调度等价。

[B-types/search.json](B-types/search.json)在B PDB TPI的6125条type records中未找到五个目标游戏类型名字；这是限定名字/TPI范围的负结果，不能宣称全部PDB没有任何可用布局线索。[trait-candidates.json](B-types/trait-candidates.json)另登记44个包含public-wall关键字的非直接符号，并按实际定义root排除库wrapper；泛型参数含游戏类型不等于核心实现。

## 未关闭的工作

完整墙发布的dirty/geometry/terrain gathering、缓存填充和资源所有权尚未恢复。关联整函数外围、真正Bevy注册及调度仍未验收。创建还缺drawable、spire/support等bundle业务，以及TriggerVolume元素schema；方向/Flat消费者与墙砖/开口/门窗仍未贯通。实际despawn和队列执行、场景/渲染仍是后续工作，当前不能称为完整建筑系统。

下一步优先追踪发布后的max-y缓存填充和系统注册顺序，恢复创建剩余组件/关联更新的真实调用边界，再沿墙构造与开口/附着推进。依赖源码继续复用，原码校验只统计确认的游戏业务范围。
