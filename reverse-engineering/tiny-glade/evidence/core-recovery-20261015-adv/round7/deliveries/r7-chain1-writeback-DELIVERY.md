# Round7 链1 写回 DELIVERY（2026-10-16，真实样本 Decorators serde）

> 实现对象：① r7-chain1-writeback-CONTRACT.md（非空 Decorators/Walls/Terrain 样本扫描
> 35+ 个）。文件权内（systems/save/snapshot.rs）。**未碰 rhapsody/roof/wall 禁区**。
> country-core 自身零错误。

## 真实代码接线

1. **Decorators states serde 逐字段定案**（snapshot.rs，r7-chain1-writeback §1，非空样本
   33273486/87775451 反推）：
   - `DecoratorsSectionJson {states: Vec<(DecoratorAddrJson, DecoratorStateListJson)>}`。
   - `DecoratorAddrJson`（untagged）：`{"Wall": id}` / `"Terrain"` / `{"Stair": id}`。
   - `DecoratorInfoJson`（8 字段）：allow_connection bool / clv / color_id /
     dst{Wall:anchor{anchor_ws[3],corner,platform,wall_height_relative_y}+coord{x,y}
     | Terrain:pos_xz[2]+yaw_radians} / rank{x} / seed / stair_width f32 / ty 串
     （"Stairs"/"CottageWindow"）。
   - apply Decorators 分支改用真实 serde（`serde_json::from_value::<DecoratorsSectionJson>`）。
   - **0x28B/0x16B 打包 + restore_snapshot 真写回待①**（ID 旋转 XOR 0x1429cd080）。
2. **Walls 曲线区样本受限登记**（r7-chain1-writeback §2）：真实曲线 2..104 点可变，
   0x70B 固定记录装不下——**打包方式待①**；crate WallAuthoritySnapshot 独立曲线存储
   可能等价，标注不伪造 0x70B 打包。
3. **GameSaveData 240B 二进制**（§3）：JSON 面已闭（Camera/Sheep/ToD/EditType serde）；
   二进制组装待①。
4. **Terrain strokes JSON 明文**（r7-chain1-field §3，上批已落）：TerrainSectionJson →
   tg_terrain；5 域 apply 序已落。

## candidate 链测试

- 本组文件 lib 类型检查零错误；country-core 自身零错误（排除 rhapsody）。
- DecoratorsSectionJson/DecoratorInfoJson serde 编译验证（真实样本字段名）。

## 原侧验证

- ① r7-chain1-writeback：发行树 35+ snapshot 扫描（非空 Decorators 33273486/87775451 等）；
  真实存档 history.json。

## 仍缺项（待①，登记不造）

1. Decorators 0x28B/0x16B 打包 + restore_snapshot 真写回（ID 旋转 XOR 0x1429cd080 已知）。
2. WallState 0x70B 曲线区打包（样本受限：可变点）。
3. GameSaveData 240B 二进制布局（拆 save_game 0x140a3f470）。
4. EditType serde derive（RasterStroke Path wire {brush_size,is_additive,points}）。
5. 跨轮待①：sheep movement+X4、ui_clutter 0x58B、ivy count_range、mod_key 表2、
   camera_rig/weather/time_of_day、28-tuple 槽位 + Schedule.run。

## 组外申请记录

- 无本轮组外修改。未碰 rhapsody/roof/wall 禁区。
