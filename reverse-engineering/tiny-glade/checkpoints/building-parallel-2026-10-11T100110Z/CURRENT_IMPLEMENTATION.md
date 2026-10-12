# 当前实现保存记录

保存时间：2026-10-11T100110Z（UTC）。用户要求先保存当前全部实现；三个 agent 已中断，源码停止推进。

## 保存范围与状态

本批状态为 **implemented_unverified / work_in_progress**。用户此前要求并行实现优先、暂缓验证，本次保存不执行 Cargo、测试或原指令 oracle。阶段4的54项测试/67项限定差分证明保留为历史冻结快照，不接受本批新增代码。完整建筑 system_acceptances 仍为0；最终目标仍是 C v1.16.0，目前主体基于 B。

归档保存 reconstruction 源码（排除 target 等生成目录）、全部当前 evidence、scripts、tracking、workspace 和根目录文档。原始发行 EXE/PDB/资产、Ghidra 数据库和本机工具位于外部路径，不复制到本源码快照；身份与引用保存在 tracking/evidence。manifest.json 记录每个归档文件的路径、字节数与SHA256；这些哈希仅用于保存完整性，不是业务验证。

## 已落地并登记的本批代码

- Gate120消费者：原木门/地下post分支、支撑砖、真实Bevy门实体、CPU网格属性及Transient所有权；Typed Named Nani描述符与SolidVertexColor marker，更新/删除撤回旧输出。
- SolidVertexColor CPU：Transform48→scaled Mat3列/translation/两个组件存在性标志→64B记录、bounds；独立atlas/CPU增改删状态，未制造GPU handle或NaniActiveInstance。原bundle+48 word是Named enum判别，不是额外Solid组件。
- 平台碰撞：实际台阶geometry→锁定Rapier/Parry/Slotmap→插入/Keep/替换/移除→query更新。DirtyTiles64实际AABB栅格、真实初始mask必填、copy/clear/publish与墙订阅接口已加入候选代码；完整17子世界仍在恢复。
- 矩形/自由墙历史：原UI80 finish→Enclosure历史owner；cancel实际恢复Decorator三tracking容器、全局索引与savedRemoved；free-wall pending slot→StateChange→Source发布。
- DecoratorInfo72：B原始dst/coord/anchor/type/rank/ID字段；真实Source88构造与storage insert/update，直接查实际asset/subtype→OpeningAuthority→现有holes/bricks；NativeID→OpeningID身份映射仍显式。
- 栅栏：raw/public/prepared→threshold→Perchance→angle/resample→post/rail→64B CPU实例，以及World增改撤回/删除。仍未验证，GPU池和material stats边界未关闭。
- 父集成：共享core exports、Bevy阶段/注册、Nani类型与CPU staging、CLI观察事件/运行状态输出。原fork完整调度未恢复。

## 正在恢复、尚未闭合的部分

- building/collision_query_policy.rs 是新独立草稿；17子世界过滤/mapper/原subscriber正在取证，不能把平台subworld7验收扩大为完整RaycastWorld。
- Decorator锚点跟随/底部门profile、cache transform、原iter_modified选择正在恢复；相关已取原始导出一并保存。若尚无注册模块，属于分析/草稿，不能当已接World。
- 栅栏之外完整floor/墙面族、屋顶TriggerVolume/尖顶/梁、其他装饰、玩家楼梯、所有历史owners/存档、C迁移、真实GPU与最终场景仍未完成。
- 原ARM屋顶case14单filler1ULP失败保留，不降低比较精度。
- 新perchance=0.4.0已加Cargo.toml；Cargo.lock尚未统一解析，macaw/once_cell/ordered-float版本与feature仍需对照原锁。当前工程构建状态未知。

## 冲突与下一次恢复工作

完整待核对文件在 tracking/parallel-implementation-conflicts。未声明冲突已解决或本批已review。关键核对：共享register/stage和系统tuple；Native opening与旧intent/sidecar双writer；同帧cancel/new place顺序；source/门/碰撞删除；DirtyTiles copy-clear-publish与订阅mask；RNG依赖Lock解析。

下一次先完成未提交的草稿接口并统一梳理冲突，再按用户届时指示启动构建与原码/World验证。保存记录不继续修改业务代码，不自动重启agent。
