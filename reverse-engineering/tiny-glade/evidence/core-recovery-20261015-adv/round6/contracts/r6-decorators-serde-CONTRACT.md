# Round6 CONTRACT — Decorators serde（0x28B/0x16B 记录架构）

> 逆向 agent ① 交付。证据：decorator_storage.rs.md 章节（take/restore 全解）+ 反编译
> decomp-0x140a20d70（543 行）+ crate DecoratorStorageSnapshot。状态：**记录形状 + 编码
> 定案可直接落地；JSON serde 逐字段名待取证（中优先）**。

---

## 1. 0x28B 记录形状（O，章节 §遍历装饰记录）

restore_snapshot（0x140a20d70）遍历 **0x28B 步长条目**，逐条读：
```
{ u32 DecoratorId @0（快照编码）, 8B×4 @4..0x14, u32×2 @0x18..0x20 }
```
- **ID 编码**（O）：`encoded = (id 32 位旋转 <<0x20 | id >>0x20) ^ 16B XOR 掩码 0x1429cd080`
  → 恢复 `FUN_140b776d0(map, encoded, key)` 旋转还原 + 移除。
- 子侧状态字节公式（crate 已证，decorator_storage.rs:337）：`(b-2)<=2 ? b-1 : 0`。

## 2. 与 round5 契约的衔接

- round5 c-apply-section-parsing §2：0x28B 记录 = {状态 16B, 子计数@0x10}——章节细化为
  `{u32 id, 8B×4, u32×2}` 的逐槽形状（两者相容：id@0 + 8B×4 = 16B 状态区 + u32×2 计数区）。
- 内层 0x16B 子记录（+0x52 状态字节）= 每记录携带的装饰条目。
- take_snapshot 前置（O）：per_wall/terrain/stairs/removed/decorator_to_storage 全空。

## 3. JSON serde 架构 — **待取证（中优先）**

- 快照 Decorators 节 = 采集器输出的 serde；crate `DecoratorStorageSnapshot{states:
  Vec<DecoratorAddrSnapshot{encoded_id, addr, state}>}`（16B 条目）为**近似**。
- JSON 值形态（结构化对象 vs 扁平记录）+ 逐字段名**待取证**（拆 capture serde）；
  ② 可先按 0x28B 形状 + ID 编码落地 restore（take/restore 已存在，对齐节格式即可）。

## 4. 可实现性
- **可直接落地**：0x28B 记录逐槽形状 + ID 旋转 XOR 编码/解码 + 子侧状态公式 + 空桶前置。
- **待取证**：JSON serde 字段名、0x16B 子记录逐槽。

## 5. 证据索引
- chapters/resources/walls/decorator_storage.rs.md（take/restore 全解：0x28B 条目、ID 编码
  0x1429cd080、前置断言串）。
- wall/decomp/decomp-0x140a20d70.txt（543 行）；crate decorator_storage.rs:299-380
  （DecoratorAddrSnapshot/state 公式，round4 t3-ui_clutter 0x60B 交叉）。
