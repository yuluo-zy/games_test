# 树放置合同独立复核

复核者：core_business_lead。生产基线 `fa53d49a61dd7496cddc9af054c6fb1899fa1676`；本文件只记录合同复核，未编译、未运行原程序、不代表 C 行为验证。

已直接读取原字节绑定的 `native/report.json`、完整 `random-prefab.asm.txt`，及 `place-tree.asm.txt` 的 RNG、packed 浮点、事件提交、History 调用段。报告的四项请求均成功，B RNE/PDB 的完整 SHA 与项目身份一致，非 patched Ghidra 映像。完整函数边界另由 PDB procedure 确认；collector 本身不证明边界或业务语义。

确认以下合同可进入候选实现：

- 每个 storage index 创建独立 fastrand 状态，第一次 `f32` 后同一状态生成范围整数，再生成第二次 `f32`。当前手写变体算法从原 index 另起状态并依赖全局浮点 RNG，确有行为偏移；复用 workspace pinned `fastrand=1.8.0` 合理。
- packed 第二 lane 的下限为零，不能从相邻地址读出旋转下限。候选应保留原乘加次序，不引入 fused multiply-add。
- 原 random selector 的资源实参为同一 `ActiveSession`；当前 readiness/corruption/drag 的命名和输入不能作为真相。季节身份还需保留 PDB inline/caller 的引用，不能只依据 ICF 主符号。
- Insert 事件在 `142269d62..142269ddb` 提交，History 在 `142269e94` 才调用，当前顺序必须修正。
- `random_tree_prefab` 有两个真实 caller。修复 producer 不能代替 missing-prefab consumer 修复；整合必须同时审 processor 同链候选。

允许在本合法工作目录生成候选。生产变更、Cargo 与运行验证均由 root 统一执行。仍需源码复核后才允许交付集成，不能仅凭本合同宣称树编辑、删除或历史闭包已经完成。
