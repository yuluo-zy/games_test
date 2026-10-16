# RE-EDITCMD：terrain EditCommand（EditTerrainStrokeCmd）字段序证据

> 会话：interaction re-analyst（2026-10-16，E7）。目的：定原 EditTerrainStrokeCmd（原
> EditCommand）事件的字节布局字段序，对齐 tg-terrain `{id,falloff,height,position}` 声明序。
> 依据：**本轮定点 fresh 反编译**生产者 ui_edit_terrain @0x140aef1f0（34KB）与消费者
> edit_terrain_stroke @0x14094e000（37KB），落盘 `interaction-group/decomp/fresh-0x140aef1f0.txt`
> / `fresh-0x14094e000.txt` + results/out-editcmd.json。记号：O=机器码证实；I=推断；U=未恢复。

## 1. 结论（供 restorer 对齐 serde/ABI）

**原 EditTerrainStrokeCmd = 0x28B（40B），字段序 = `{falloff, height, position, id}`，
id @+0x1c（硬锚点）；与 tg-terrain EditCommand 声明序 `{id, falloff, height, position}`
不一致（原 id 在末尾，tg-terrain 在开头）。**

| 字节 | 字段 | 证据 |
|---|---|---|
| +0x00 | falloff Adjustment disc u32（0=Keep/1=Absolute/2=Delta；实测写 2） | 生产者写 2 |
| +0x04 | falloff.value f32（Delta 量，`fVar75*0.002` 缩放） | 生产者写 fVar75*0.002 |
| +0x08 | height Adjustment disc u32（实测写 0/2） | 生产者写 0 / 2 |
| +0x0c 或 +0x10 | position Vec3（x=cursor.x@首，y/z=StrokeParams +0x08/+0x0c） | 生产者写光标/参数 |
| +0x1c | id u32（stroke id） | 生产者写 uVar3；消费者读 T+0x1c |
| +0x20 | u64（事件 id/计数） | 生产者写 uVar24 |

## 2. 生产者 ui_edit_terrain @0x140aef1f0 记录写入（O，fresh 反编译）

case-2 分派内，按 `|fVar76|<=|fVar75|` 选分支（falloff 编辑 vs height 编辑），各推一条 0x28B 记录
到同 Vec（lVar29，grow 0x140562ab0 族）：

**分支 1（falloff 调整，行 486-493）**：
```
+0x00 = 2                          # falloff.disc = Delta
+0x04 = fVar75 * 0.002             # falloff.value（0.002 每帧灵敏度缩放）
+0x08 = 0                          # height.disc = Keep
+0x0c = CONCAT44(uStack_224, local_228)   # {x=local_228(cursor.x), y=uStack_224(Params+0x08)}
+0x14 = CONCAT44(uStack_21c, local_220)   # {z=local_220(Params+0x0c), +0x18=uStack_21c(Params+0x10)}
+0x1c = uVar3                       # stroke id
+0x20 = uVar24                      # 事件 id/计数
```

**分支 2（height 调整，行 599-606）**：
```
+0x00 = 0                          # falloff.disc = Keep
+0x08 = 2                          # height.disc = Delta
+0x0c = SQRT(fVar68)*20.0 - fVar67  # height.value
+0x10 = CONCAT44(uStack_224, local_228)   # position {x=cursor.x, y=Params+0x08}
+0x18 = local_220                   # position z=Params+0x0c
+0x1c = uVar3                       # stroke id
+0x20 = uVar24                      # 事件 id/计数
```

- **字段序 {falloff, height, position, id} 两侧一致**；id@+0x1c 两侧硬。
- **Adjustment 紧凑打包（O）**：每个 Adjustment = `{disc u32, [value f32 仅当非 Keep]}`；
  非 Keep 的调整带 value 槽（4B），Keep 的只有 disc。⇒ position 起始偏移浮动：
  仅 falloff 调整 → +0x0c；仅 height 调整 → +0x10。**这是原紧凑字节布局（非 8B×2 固定
  Adjustment）**，restorer 建模时注意（若按 tg-terrain 8B Adjustment 固定结构会错位 4B）。
- falloff.value 按 **0.002** 缩放（笔刷强度/帧灵敏度）。

## 3. 消费者 edit_terrain_stroke @0x14094e000（O，fresh 反编译）

- 排空 Events（0x30B 元素，含 EventInstance 包装）；`iVar11 = piVar33[9]`（元素+0x24 =
  **T+0x1c**）作 HashMap（stroke 表 lStack_1d0）查找键 —— **id@T+0x1c 与生产者互证**。
- 命中后：读 T+0x00（float*，SetTerrainStrokePtsCmd 点列 ptr）与 T+0x08（count）应用点集；
  或经 0x28B 记录处理 falloff/height（apply_resolved 逻辑，Falloff∈[0,1]、height Abs/Delta
  限幅在 tg-terrain::edit 共享 crate）。
- 写 0x28B outcome 记录：`{count@0, 3@8, id@0xc, ?@0x14/0x18, ?@0x1c, magnitude f32@0x20}`。

## 4. tg-terrain 对齐指引

- tg-terrain `EditCommand {id, falloff, height, position}` 声明序 **≠ 原字节序**。
  建议按原字节序调整：`{falloff, height, position, id}`（或显式注明 ABI 打包以
  id@+0x1c 为锚、Adjustment 紧凑 value 槽）。
- Adjustment 判别：0=Keep / 1=Absolute / 2=Delta（本报告生产者写 2=Delta、0=Keep 实测）。
- 原 0x28B 尾部 +0x20 u64 = 事件 id（生产者写 Vec 计数 uVar24）——与 bevy EventInstance
  id 对齐候选（U）。

## 5. 未决（U）

1. position 的三分量精确来源（x=cursor.x 已证；y/z=TerrainStrokeParamsCache+0x08/+0x0c，
   +0x18=Params+0x10 语义未定名——falloff/height 当前值候选）。
2. +0x20 u64 语义（事件 id vs recalc 标志）——与 bevy EventInstance id 关系待 consumer
   EventWriter 侧交叉。
3. SetTerrainStrokePtsCmd 精确 T 布局（{points ptr+count@0, id@0x1c, ...}，O 已见，完整字段 U）。

## 6. 证据落盘

- `interaction-group/decomp/fresh-0x140aef1f0.txt`（34KB）/ `fresh-0x14094e000.txt`（37KB）
- `interaction-group/results/out-editcmd.json`
