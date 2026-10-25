# 上层模块恢复 agent 共享 playbook（2026-10-18）

> 从本会话 13 大轮 / ~30 子轮的实战提炼。**每个 agent prompt 的核心清单**。
> 配合：`2026-10-17-recovery-sop.md`、`2026-10-16-logic-verify-protocol.md`（分歧检查表 8 条）、`2026-10-17-domain-quickref.md`。

## A. 判定与复核（最高收益）

1. **restorer 自判不算判等**。任何"判等"必须有独立 reviewer 的 verdict；本会话 A 面 53 项复核抓出 23 处真实缺陷（~43%）。
2. **复核必须检查"可运行面"**：类型/函数存在但**未注册、未 spawn、未接 schedule** = 未实现。典型案例：toolbar `WidgetByteState` 孤儿，四系统无 schedule。
3. **复核必须枚举副作用**：restorer 常多加二进制不存在的副作用（`clear_out_visual_entities` 的清表/清帧）。逐副作用对照。
4. **复核必须覆盖 undo/回放旧位路径**：最常漏（tree Move-undo 旧位 `edit+0x14/+0x1c` 整条缺失）。
5. **证据等级**：restorer 自判 < 独立 reviewer < 数值断言 < Unicorn 执行差分。登记每函数等级，不以低等级冒充高等级。

## B. 符号/类型陷阱

6. **ICF（identical code folding）**：不同函数共享地址/符号。`0x1405b0d70=hash_one`、`0x140053320=remove_entry`、`0x140a1b100=TreeStorage::iter` 等 5 别名、`grow_one<tuple>` 被折叠成名义类型。**禁止仅凭符号名推断类型**；用 caller/callee、字节布局、stride 交叉验证。
7. **事件/命令载荷**：`EventInstance{id@0,payload@8}` 前缀；stride = 8+payload+对齐。ICF 会让名字误导（`RequestAppModeChange` 实为 `AppMode` 16B，不是 `(String,SettingsMenuState)`）。
8. **布局用尺寸闭合验证**：bundle 0x68 = 组件尺寸之和；`ComponentDescriptor{size,align}` 立即数优先于目测（TaffyNode 20B/align4、ShowOnly ZST）。差 1 字节也要解释。
9. **"有符号"≠"已发射"，"无符号"≠"未发射"**：查 `procedure_instances`、inline 宿主、call 点。
10. **core 类型会演进**：消费方要按当前 core 复核（PrefabInstanceData 0x58、CritterGoal{position,goal_tag}、ChunkId 等）；归属/版本变化登记。

## C. 反编译卡点

11. **Ghidra shared 空反编译勿信**：先查 shared overlay 是否损坏（codelen=0）→ 用 ReVa 专属写 worker fresh clone + 长超时。
12. **jumptable 破坏**：直接解 `.rdata`（`table_base + signed32`）；Ghidra 在跳表处截断是根因。二级表可能物理重叠（主表+4 项），按后缀索引验证，不硬选。
13. **radare2 先 `aa` 再 `pdg`**：很多"反编译失败"只是缺 analyze（half_timbering 13KB 即出）。
14. **NOP 对齐垫暴露原版未初始化读**（add_autoclutter_on_windows）——这是原行为，不要"修好"。
15. **hardpoint 三次不可解** → 结构级判等 + 各段 CALL/数据流登记，标未实现，不重复撞线。

## D. 语义细节

16. **NaN/ULP 按机器指令语义**：`MAXSS/MINSS` 的 unordered 返回第二操作数；`UCOMISS+SETC` 的 NaN 行为；Rust `f32::max/min` 不同。常量按位比对（0.18 vs `0x3e3851eb` 差 1 ULP 也要登记）。
17. **bevy changed-tick 写是框架自动行为**，不是业务写面，不要建模为 U。
18. **门控极性/事件顺序**是高频错误：`!=` vs `==`、`>=` vs `>`、先读后写 vs 先写后读、事件 `last()` vs `first()`、耗尽 vs 只读 1 个。

## E. 协作/工程

19. **src 互斥 + 在途冲突回避**：他组在途文件只出合同（地址+stride+字段+消费点）；不硬写、不代改。
20. **编译解阻（跨域小改）必须标注 owner 复核**；不悄悄改（PlayTimberframeSound 补字段、`pub use`）。
21. **并发 cargo 构建会打满磁盘（ENOSPC）**：分批 check、清理临时 target；不要所有 agent 同时全量 build。
22. **配额/网关错误（402/429/400）**：立即重试或错峰；每路回执带测试基线（单调不减）。
23. **收口三态**：完成 / 登记（缺类型·跨域·hardpoint，写明解锁条件）/ 阻塞；登记必须可被下一波直接接手。
24. **判等结论转数值断言**（原生常量/边界），让回归机器可查。
