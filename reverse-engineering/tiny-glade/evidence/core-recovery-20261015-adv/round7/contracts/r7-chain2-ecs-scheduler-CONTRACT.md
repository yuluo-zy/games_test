# Round7 CONTRACT — 链2：ECS 调度器（28-tuple 槽位 + GameLoop Schedule.run + 纯 Commands 身份）

> 逆向 agent ① 交付。证据：procs.tsv into_configs + round5/6 契约 + capstone GameLoop。
> 状态：**系统名定案；槽位 fn_ptr 精确映射 + Schedule.run 调用点待取证（低优先）**。

---

## 1. 28-tuple 系统序（O，into_configs 权威；供宿主接线）

**G1（FUN_140a2bf40，10 系统）**：load_textures / load_shader×2（+FinalPost）/
custom-cursor SSBO / load_prefabs / config-watch / 纯 Commands / terrain-height+PathRaster /
audio-samples（round6 r6-28-system-names §1）。
**G2（0x140839b80，8）**：init_photomode / startup ToD-weather-glade / mesh×3 / color / texture /
ScheduledTerrainReadback。
**G3（0x1415f4750，1）**：mesh-atlas。**G4（0x1409c8600，1）**：shader 变体。
**G5（0x141736760，6）+ G6（0x140b6bd50，6）**：渲染/音频。

## 2. 28-tuple 槽位 fn_ptr 映射 — **待取证（低优先）**

- FUN_140a97110 表 0x1c0B = 28×16B {fn_ptr, cfg}；四 builder 输出分配（G1 10 + G2? + G3 1 + G4 1）——
  精确槽序需解 FUN_140a97110 组装 trace（round4 t2-run_game_thread 已标）。
- 系统名已定（§1），宿主按名接线不等槽位映射即可；槽位映射仅存档核对用。

## 3. GameLoop::run 内 Schedule.run 调用点 — **待取证（低优先）**

- `tiny_glade::game_loop::GameLoop::run`（0x1404d8250，4899B）= 宿主线程驱动（round5 a 契约 §6）；
  Schedule.run 执行桥内部调用点在 4899B 内，capstone 可定位（run_game_thread 两处
  `new(0x1404d8100)+run(0x1404d8250)`）。
- ②/宿主：GameLoop = 宿主态，country-core 只保证注册；不重建。

## 4. 纯 Commands 系统身份 — **待取证（低优先）**

- G1 #7 纯 Commands 系统（无资源参）身份未定；可能是启动资源注册命令（`Commands`-only）。

## 5. 证据索引
- round6 r6-28-system-names；round5 a 契约；procs.tsv into_configs；capstone GameLoop 0x1404d8250。
