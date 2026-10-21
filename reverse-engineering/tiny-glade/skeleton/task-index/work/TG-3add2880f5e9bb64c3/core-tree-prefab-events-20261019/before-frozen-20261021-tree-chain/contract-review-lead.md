# 树命令处理合同独立复核

复核者：core_business_lead。本次窄委派来自 root，沿原 `core-tree-prefab-events-20261019` claim 工作目录，未更改 claim。仅允许候选实现，生产和执行验证归 root。

直接核对 UI task 下 `process-native/process-tree.asm.txt` 与本合同：

- `142267aaa` 调用前 RCX 为季节字段；随后检查原事件 variant 低两位。不能把共址符号解释为 ActiveWidget 拖动状态。
- `142267bef..142267c08` 的 MAXSS 候选仅在裸树且 olden 条件时保留。`.max(0.7)` 能保留 NaN 时选 0.7 的原行为；其余路径必须保留原 scale。
- `142267dcd..142267ded` 的顺序为 naked/olden 时 mode=3，否则先求 flowery_extra+1，再由原 bool 非零覆盖为0；因此 bool 优先于 flowery 分支。
- `142267ea8` 直接写入此前清零的 XMM14，16字节 flags 全零有原证支持，不能按旧注释制造新字段。
- `142268146..14226814d` fallback 传入原 storage index 与相同 ActiveSession，必须和 producer 的主题语义同时改正。
- 当前源码的 readiness/corruption/drag 辅助入参没有此原系统的 typed wrapper 支持，应从此 consumer 移除，不能创建替代资源掩盖错误身份。

合同允许进入候选。还需候选源码复核，并由 root 执行真实 Bevy 事件、储存替换、删除和历史验证；此复核不代表运行或 C 版本验收完成。
