# Round6 CONTRACT — 收官批次 10 项状态（最终汇总）

> 逆向 agent ① 交付。逐项：可直接落地 / 待取证（标地址）。禁区（rhapsody/roof/wall）已避开。

---

| # | 项 | 状态 | 契约/证据 |
|---|---|---|---|
| 1 | Decorators JSON 逐字段名 + 0x16B 子记录 + restore 逐槽 | 0x28B 形状+ID 编码 **可直接落地**（r6-decorators-serde）；JSON 字段名/0x16B 逐槽**待取证**（中优先） | r6-decorators-serde；decorator_storage.rs.md §遍历 |
| 2 | GameSaveData 240B + session create 96B+32B+24B | AsyncSaveCmd 判别**可直接落地**（r6-async-savecmd）；字段名**待取证**（中优先） | r6-async-savecmd；ALGORITHMS §2.6；session-DELIVERY §2 |
| 3 | sheep movement + X4 | **待取证**（中优先） | r6-sheep-layout §2/§3；0x140a3f470 待拆 |
| 4 | Terrain strokes 格式 | **待取证**（中优先） | flate2 入口 O；tg_terrain serde |
| 5 | ui_clutter 0x58B | **待取证**（中优先） | 0x60B 已定（round4 t3）；AutoClutterRecord 0x58B U |
| 6 | ivy count_range RNG 界 | **待取证**（低优先） | spawn_ivy [rbp+0x190] 调用方栈参映射（round4 t3-ivy-constants §4） |
| 7 | mod_key 表2 键码 | **待取证**（中优先） | 表2 @0x142ae5f64；r6-gamekeybindings §3 |
| 8 | camera_rig/weather/time_of_day | **待取证**（中优先） | chapters；round5 a 契约 G2 #2（startup ToD 系统面 O） |
| 9 | 28-系统逐名 + on_event | **可直接落地**（r6-28-system-names） | into_configs 权威；on_event 应用对象定案 |
| 10 | 5 域（Roofs/WallColors/GateColors/Trees/Raster）写入序 | **待取证**（中优先） | round5 c-apply-targets §1 节序 O；apply 函数/目标映射待证 |

## 收尾说明
- **可直接落地汇总**（② 可立即实现）：Walls 0x78B（record[0]+0x70B+4 字段）、Decorators 0x28B+ID 编码、
  GameKeyBindings 40×12B、AsyncSaveCmd 判别、sheep 已证偏移、28-系统名 + on_event、G6 音频 6 系统。
- **待取证汇总**（均标地址，不编造）：Decorators JSON/0x16B、GameSaveData/session create 字段名、
  sheep movement+X4、Terrain、ui_clutter 0x58B、ivy RNG 界、mod_key 键码、camera_rig/weather/ToD 深层、
  5 域映射、纯 Commands 系统身份。

## 证据索引
- round4/5/6 全部契约（交叉引用如上）；procs.tsv into_configs；public-session-names；
  session-save/ALGORITHMS。
