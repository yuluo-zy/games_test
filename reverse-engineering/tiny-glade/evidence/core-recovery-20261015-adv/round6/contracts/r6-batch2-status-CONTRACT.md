# Round6 CONTRACT — 批次2 状态汇总（剩余项逐项裁定）

> 逆向 agent ① 交付。逐项：可直接落地 / 待取证（标地址与既有证据）。禁区遵守（rhapsody/roof/wall 不碰）。

---

## 1. Decorators serde — **可直接落地**（r6-decorators-serde-CONTRACT.md）
0x28B 记录 {u32 id, 8B×4, u32×2} + ID 旋转 XOR 16B 掩码 0x1429cd080 + 子侧状态公式 + 空桶前置；
JSON 逐字段名待取证（中优先）。

## 2. AsyncSaveCmd 变体 — **可直接落地判别值**（r6-async-savecmd-gamesavedata-CONTRACT.md）
变体 3 OpenFile / 4 CloseFile / 9 CaptureSnapshot / 10 Flush / 0xb Shutdown（ALGORITHMS §2.1）；
变体载荷字段名待取证。GameSaveData 240B / session create 96B+32B+24B 字段名待取证（中优先）。

## 3. sheep spawn·movement + X4 Query — **待取证**（r6-sheep-layout §2/§3）
movement 转移 + X4 活羊 Query（sync save 0x140a3f470 sheep 捕获段）中优先。

## 4. Terrain strokes / ui_clutter 0x58B / ivy RNG 界 / mod_key 键码 — **待取证**
- Terrain：tg_terrain serde（flate2 解压入口 O）。
- ui_clutter AutoClutterRecord 0x58B 字段名 U（round4 t3-ui_clutter-parent-map 0x60B 已定 0x60B）。
- ivy count_range RNG 界 = spawn_ivy [rbp+0x190] 调用方栈参（round4 t3-ivy-constants §4，低优先）。
- mod_key 表2 键码（round6 r6-gamekeybindings §3，中优先）。

## 5. camera_rig / weather / time_of_day 深层补证 — **待取证（长期未深挖）**
- 既有章节：chapters/systems/camera_rig/*、resources/weather.rs.md、time_of_day.rs.md。
- camera_rig：MainCamera/GameConfig 消费面部分 O；weather：Weather/TodSequence 状态机 I；
  time_of_day：startup ToD/weather 系统面 O（round5 a 契约 G2 #2）。深层字段待证（中优先）。

## 6. 28-系统逐名（供宿主，不碰 G1/G3/G4 asset 组实体）— **部分定案**
round5 a 契约：G1(10)/G2(8)/G5(6)/G6(6) 系统面 O；G3(0x1415f4750)/G4(0x1409c8600) 各 1 系统面 O。
28-tuple 槽位映射 + GameLoop::run 内 Schedule::run 调用点待取证（宿主/D 组，低优先）。

## 证据索引
- round4/5/6 既有契约（交叉引用如上各条）；chapters/country-core-deep 各模块章节。
