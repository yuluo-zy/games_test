# T3 CONTRACT — ui_clutter parent-map 0x60B 载荷槽位（部分定案）

> 逆向 agent ① 交付。证据：decomp-0x14099ca60.txt（ui_convert_autoclutter_to_decoclutter，
> 0x14099ca60）行 480-520（`uVar30 * 0x60` 槽索引 + 字段读取）。
> 状态：**0x60B 槽位 + 4 字段定案；其余区间待取证**。

---

## 1. AutoClutterParent 0x60B 槽位（O）

`maintain_autoclutter_parent_hashmap`/`ui_convert_autoclutter_to_decoclutter` 内 parent 记录
按 `slot * 0x60` 索引（decomp `puVar22 = (uint*)(uVar30*0x60 + base)`），已观测字段：

| 偏移 | 宽 | 语义（O） |
|---|---|---|
| **+0x00** | u32 | **tag/判别 `< 3` 门**（`if (*puVar22 < 3)` 才进入转换） |
| +0x10 | u32 | 计数/类别候选（puVar22[4]） |
| **+0x30** | u64 | **`ClutterMeshTy::from` 第一参**（`FUN_1408b83d0(*(u64*)(+0x30), *(u64*)(+0x38))`） |
| **+0x38** | u64 | **`ClutterMeshTy::from` 第二参** |

其余区间（+0x04..0x0f、+0x14..0x2f、+0x3c..0x5f）**U**，不编造字段名。

- `FUN_1408b83d0` = `ClutterMeshTy::from`（mesh 类型判别；ui_clutter-DELIVERY 已证）。
- 转换路径：tag<3 → `FUN_1408b8e90(&out, ClutterMeshTy, ...)`（mesh 类型构造）→
  `FUN_1405b0ed0`（deco 注册/转换）→ 走 `InstaCreateDeco`/`StairPoint` 生产（DELIVERY 已落地）。

## 2. ② 待办
- 现源 `AutoClutterParentMap(HashMap<u64, AutoClutterParent>)` 的 `AutoClutterParent` 若只含
  计数器/entity，按上表扩到 0x60B 观测字段（tag@0、+0x10、+0x30/+0x38 mesh 类型参）。
- 未观测区间保持 `U` 标注（不填默认值）。

## 3. 证据索引
- decomp-0x14099ca60.txt 行 444-520（0xa0B clutter 记录 + 0x60B parent 分配 + slot 索引）。
- procs.tsv：0x14099ca60 = `ui_convert_autoclutter_to_decoclutter`（8136B）。
- 现源 systems/ui_clutter/ui_autoclutter.rs:41-46（AutoClutterParentMap 现模型）。
