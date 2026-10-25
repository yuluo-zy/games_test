# T3 CONTRACT — EditType 原生 tag 表 + Terrain 历史机制定案

> 逆向 agent ① 交付。权威证据：`EditType` serde visit_str 0x140b3b630（长度分发 → 变体名 → tag）
> + 字符串表 0x142aee127.. 区 + deferred_terrain_history 0x140ac9770 反编译。
> 状态：**可直接落地（需仲裁——共享 history 文件 tag 变更）**。

---

## 1. 原生 EditType tag 表（O，visit_str 0x140b3b630 全解）

visit_str 按名字符串长度（param_3）分发，逐个字节匹配变体名 → `param_1[1] = tag`：

| tag | 名（len） | 匹配证据 |
|---|---|---|
| **0** | **Terrain** (7) | case7：`*p2==0x72726554("Terr") && [p2+3]==0x6e696172("rain")` |
| 1 | Path (4) | case4：`*p2==0x68746150` |
| 2/3/4 | Cobblestone1/2/3 (12) | case0xc：`0x7473656c62626f43("Cobblest")` + "one1/one2/one3" |
| 5 | Water (5) | case5：`0x65746157("Wate"?)r` → `[p2+4]=='r' && *p2==0x65746157` |
| 6 | Garden (6) | case6：`0x64726147("Gard")` + short `0x6e65("en")` |
| 7 | Wall (4) / WallNewSystem (13) | case4 `142aee127="Wall"`；case0xd `0x5377654e6c6c6157...` |
| 8 | Roof (4) | case4 `142aee12b="Roof"` |
| 9 | Color (5) | case5：`0x6f6c6f43("Colo")` + 'r' |
| 10 | DecoratorV5 (11) / Decorator5 (10) | case0xb `0x3556726f7461726f("V5...")`；case10 `short 0x3572('5r')` |
| 11 | Decorator (9/10/12 兜底) | case9 `0x6f7461726f636544("Decorato")`；case0xc else `142aee13f="Decorator"` |
| **12** | **Tree** (4) | case4 else：`142aee14b="Tree"` → `param_1[1]=0xc` |
| 13 | AutoClutter (11) | case0xb else：`142aee14f="AutoClutter"` |
| 14 | Stairs (6) | case6 else：`142aee15a="Stairs"` |

字符串表（rne-read 合并区，各名独立）：`0x142aee127="Wall"`、`0x142aee12b="Roof"`、
`0x142aee14b="Tree"`、`0x142aee15a="Stairs"`、`0x142aee14f="AutoClutter"`、
`0x142aee13f="Decorator"`。

## 2. 对当前实现的修正（②/③ 需要仲裁登记）

当前 `resources/history/mod.rs` `native_tag()`：
- **Tree: 0 → 应为 12**（visit_str case4 "Tree"→0xc）。
- **Terrain: 15 → 应为 0**（visit_str case7 "Terrain"→0）。
- 其余 13 变体（Path/Cobblestone1-3/Water/Garden/Wall/Roof/Color/DecoratorV5/Decorator/
  AutoClutter/Stairs）与原生一致，**不动**。

→ 当前枚举**变体顺序**也需对齐原生（Terrain 应为首变体、Tree 移到 tag12 槽）。这是共享
`resources/history/*` 变更（tag 影响存档/回放序列化），须经中央仲裁后落地。

## 3. Terrain 历史机制 = **不经 EditType**（O，deferred_terrain_history 0x140ac9770）

- 0x140ac9770（= `history_record_terrain` 语义）：读地形编辑（param_2，4 qword 载荷）→
  **构造 0x38B(56B) 记录 `{tag=1, 载荷 6 qword}`** → `FUN_140a84ec0(...)`（命令队列/MetaCommander
  候选）→ push 入 History 的 Vec（param_3 `+0x20 cap/+0x28 ptr/+0x30 len`，+0x40 总计数）。
- **结论**：地形历史直接写 tag=1 记录，**不调用 EditType::native_tag()**。
  当前 crate 的 `EditType::Terrain`（tag=15）与 `ReplayCommand::Terrain` 是**本地模型**，
  非原生 EditType 路径。② 落地地形历史应：0x38B 记录 tag=1（非 EditType::Terrain tag）；
  共享 ReplayBuffer 的 Terrain 变体可作为 CPU 侧等效（保持现实现 + 标注）。

## 4. 可实现性

- **可直接落地**（经仲裁）：EditType tag 表修正（Tree 12 / Terrain 0）。
- **可直接落地**：deferred_terrain_history 0x38B 记录契约（tag=1）。
- 注意：`EditType::native_tag` 的修正会影响 save/history 序列化兼容，必须仲裁 + 与原侧
  save 文件比对后再全量改。

## 5. 证据索引
- decomp：round4/decomp-edittype-0x140b3b630.txt（visit_str 全解）、
  round4/decomp-edittype-0x140ac9770.txt（deferred_terrain_history 0x38B 记录）。
- rne-read 串：0x142aee127..160 区（Wall/Roof/Tree/Stairs/AutoClutter/Decorator）。
- 当前源：resources/history/mod.rs:40-87（tag 表待改）。
