# REVIEW5 — resources/ivy + systems/ivy 五轮终验（reviewer-eco，2026-10-15）

复核对象：restorer 卡6（RESTORE.md 第五版，SsboId 机制对账定案 + send_to_gpu 提交 + garden pin）。

## 逐项复核

### 1. SsboId 机制对账 — ✅ 定案：默认 xxh3_64，现状正确，无需 custom-secret

- **实测复核**（本机对原 RNE 直读）：`tools/country-core-audit/verify_ssbo_id.py`
  （`CCA_EXE` 指向本地 tiny-glade.rne）——原 EXE 0x142b2ebb0 读 192B secret 作 xxh3_17_128
  复算，**两例均命中**：
  - `Cobblestone3RasterStagingSsbo` → `0x802ff23dee42ab47` OK
  - `NaniNextVisibilityBitmaskSsbo` → `0x69cdc275a605bad6` OK
- **secret 比对**：`rne-read --hex 0x142b2ebb0 32` =
  `b8fe6c3923a44bbe 7c01812cf721ad1c ded46de9839097db 7240a4a4b7b3671f`——与默认 xxh3 secret
  首 32B **逐字节一致**（默认 secret 即 `b8fe6c39…ded46de9…cb79e64e…b8084674…`）。
- **结论**：原版 `NameHash::compute` = `xxhash_rust::xxh3::xxh3_64`（默认 secret、seed 0），
  `src/name_hash.rs::NameHash::compute` 现状正确，**无需 custom-secret 改造**。三轮"SsboId::of
  建模缺口"判定**撤销**——机制本就正确；ivy 三个 SsboId 由 ~2000 候选名复算不命中，是
  **NAME 串未定位**（待取证），非机制错误。登记诚实。
- 注：ivy_send_to_gpu 的 NAME 常量用全路径串（0x142a9eb87 名表区）为文档化猜测；真正散列到
  ivy 三 id 的 NAME 未定位，待取证不伪造。

### 2. send_to_gpu 提交 — ✅ 库内业务带 GraphicsDevice，缓冲注册/提交=库外

- `upload_ivy_leaves/upload_ivy_segments` 签名 `(&IvyLeafStorage/&IvyStorage, &mut
  AssetSsboLibrary, &dyn GraphicsDevice)`，内部 `buffer.buffer.gpu_write(device, &elements)` +
  `instance_count`——**库内 render/ssbo（nani）原语**，业务体完整。
- **缓冲注册 / 实际 GPU 提交 = 库外 render 域**（需 GraphicsDevice/RenderGraph 由 render 世界提供）
  登记为接线项，非本批缺失。OK。

### 3. garden pin — ✅ 已接线

- `draw_garden.rs` 调用 `draw_raster::<GardenRasterStagingSsbo, C>`（slot 5）✓。
- `world_raster_defs.rs::staging_rasters!` 宏注册 `GardenRasterStagingSsbo`（upload!(garden,…)）✓。
- 对账通过。OK。

## 结论与 DISPATCH 建议

- **ivy = review_passed（限定）**：卡6闭合——SsboId 机制定案（默认 xxh3_64，机制正确无需改）、
  send_to_gpu 库内业务带 GraphicsDevice 完整、garden pin 已接线。
- **登记项（不阻塞）**：ivy 三个 SsboId 的 NAME 串来源待取证；缓冲注册/GPU 提交=库外 render 域
  接线；grower 主闭包逐相语义/二级表目标/空间哈希格/max_ivy_grow_height/48B↔104B 按
  RESTORE.md §四 待取证。不宣布模块 100%。
- **garden = review_passed（限定）**、**sheep = review_passed（限定）** 维持。

## ⚠️ 跨组编译阻断（需转 state 组）

实测 `cargo test -p country-core --lib`（当前树）**测试目标编译失败**：
`meta_commander.rs:314`（cfg(test) `fence_starts_blocked_and_unblocks`）调用 `fence.unblock(0)`，
而 `meta_commander.rs:49` 的 `pub fn unblock(&self)` 无参 → E0061。`cargo check`（lib）仍通过。
该文件属 state 组新建（初始分派时未建）；此错使**当前全库测试目标不可编译，连 eco 过滤测试也无法
运行**。协调者"全库 1592/1"基线已被打破。请转 restorer-state 修 `unblock(0)`→`unblock()` 后复验。
eco 侧无此问题（ivy/garden/sheep/startup 代码与测试本身无编译错误）。
