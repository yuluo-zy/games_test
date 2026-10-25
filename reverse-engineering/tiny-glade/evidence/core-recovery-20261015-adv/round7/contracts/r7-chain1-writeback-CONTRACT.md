# Round7 CONTRACT — 链1 写回尾项（真实样本：Decorators 逐槽 / WallState 打包 / GameSaveData）

> 逆向 agent ① 交付。证据：发行树 starting-builds 全部 snapshot 扫描（35+ 样本，含非空
> Decorators/Walls/Terrain）+ history.json。状态：**Decorators serde 逐字段定案可直接落地；
> WallState 0x70B 曲线区样本受限待取证；GameSaveData 240B 二进制待取证**。

---

## 1. Decorators states serde（O，非空样本 33273486/87775451/12638817 等）

```
"Decorators": {"states": [
  [ {addr 判别}, {"state": [ {DecoratorInfo 条目}, ... ]} ],
  ...
]}
```
- **addr 判别**：`{"Wall": <wall_id>}` / `"Terrain"` / `{"Stair": <id>}`（对象/字符串混合判别）。
- **DecoratorInfo 条目**（每地址的装饰列表元素）：
```
[<id int>, {"allow_connection": bool, "clv": int, "color_id": int,
  "dst": {Wall:{anchor:{anchor_ws:[x,y,z], corner:int, platform:int,
          wall_height_relative_y:f32}, coord:{x:f32,y:f32}}
        |Terrain:{pos_xz:[x,z], yaw_radians:f32}},
  "rank": {"x": int}, "seed": int, "stair_width": f32, "ty": "Stairs"|"CottageWindow"|...}]
```
- **② serde 落地**：`DecoratorStorageSnapshot.states` = 上结构；`DecoratorInfo` 字段名按真实样本
  （allow_connection/clv/color_id/dst/rank/seed/stair_width/ty）。
- **0x28B/0x16B 记录对应**：0x28B = 每 {id, DecoratorInfo} 快照记录（round5 §2 形状），0x16B
  子记录 = 每地址状态列表内条目；ID 编码 = 字节旋转 XOR 16B 掩码 0x1429cd080（round6，O）。

## 2. WallState 0x70B 打包 — **样本受限/待取证（曲线区）**

- 已证固定字段（round6 r6-walls-0x78b，O）：+0x54 height f32、+0x60 freehand u8、+0x64 arch u8、
  +0x67 removed_supports u8；JSON 面 8 字段（arch_height/arch_ty/curve/elevation/has_skirts/
  height/is_enclosed/structure_ty）。
- **⚠ 样本受限**：真实 Walls 曲线点列**高度可变（2..104 点）**（35+ 样本扫描），**0x70B 固定
  记录装不下**——WallState 0x70B 的曲线区（+0x00..0x53）打包方式（固定容量上限/指针/分离存储）
  **无法从样本定案**。→ **待取证**（需拆 load_state 逐记录曲线写或 native WallState 定义）；
  ② 现 crate WallAuthoritySnapshot 模型（curve 独立存储）可能已等价，标注不伪造 0x70B 打包。

## 3. GameSaveData 240B 二进制布局 — **待取证（中优先）**

- JSON 面已闭合（r7-chain1-field §7：Camera/History/Sheep/Time of day/Version + EditType serde）。
- 240B 二进制组装（save_game 0x140a3f470 同步保存段 +0x28..+0x100 密集字段）待拆；② 保持
  JSON 面 serde + U 标注。

## 4. 可直接落地 / 待取证
- **可直接落地**：Decorators states serde（addr 判别 + DecoratorInfo 8 字段）、5 域节序+apply 序、
  EditType serde（WallNewSystem/Path/Terrain）、存档节 serde（Camera/Sheep/ToD）。
- **待取证**：WallState 0x70B 曲线区打包（样本受限）、GameSaveData 240B 二进制、Decorators
  0x16B 逐槽、Terrain strokes 二进制（JSON 面已闭）。

## 5. 证据索引
- 发行树 `assets/starting-builds/{demo,full}/*/snapshots/*.snapshot`（35+ 样本，扫描表见 r7 分析）。
- `assets/starting-builds/demo/00-demo2/history.json`（存档 serde）。
- round5 c-apply-section-parsing §2、round6 r6-decorators-serde/r6-walls-0x78b。
