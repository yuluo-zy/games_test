# RE-ANALYSIS: systems/garden + compute_garden + garden_audio（生态组 · garden）

> 会话：eco re-analyst（2026-10-15）。范围：`systems/garden/{draw_garden,record_garden_history}.rs`、`systems/garden.rs`、`resources/garden_mask.rs`、`systems/history/replay_garden.rs`、`systems/render_loop/compute_passes/compute_garden.rs`、`systems/audio/garden_audio.rs`。
> 依据：既有 terrain-water/collision-eco decomp（域级已 O）+ 本轮 replay_garden 闭包确认 + 串池 rne-read。
> 记号：O=机器码证实 / I=有界推断 / U=未恢复。不改 reconstruction 源码。

## 1. 函数清单表

| 函数 | 地址 | 状态 | 行为一句话 |
|---|---|---|---|
| draw_garden | 0x140ae4620 | O（terrain-water） | 工具 id 分发（9=橡皮/17=花丛），半径=笔刷+1.5（1.5@0x1429258d0），GladeBorder::is_shape_inside 界内校验 → draw_raster\<GardenRasterStagingSsbo\> |
| record_garden_history | 0x141695e10 | O（terrain-water） | Events\<GardenDrawEvent\> 双游标差分（+0x18/+0x38）→ 12B 编辑单元 Arc → History::add |
| systems/garden.rs::build | 0x140b6d290 | O（terrain-water） | 插件注册 draw_garden/record_garden_history/replay 系统 + GardenRaster 资源 |
| replay_garden closure$1::closure$0 | 0x141447500 | **O（本轮升级自 I）** | 48B 命令元素 push 到 cap-first Vec（FUN_1405629f0 扩容；6×qword → {+0x00..+0x28}），generate_replay_cmds_from_edit\<Garden\> 命令构造闭包 |
| replay_garden（单态） | 0x1409f6ff0 | O（history 域） | replay_raster\<GardenRasterStagingSsbo, WorldRaster\<512\>, GardenFilter\> 薄包装 |
| garden_rasterize_from_cpu | 0x140a22140 | O（render-loop W12） | 花园掩码 CPU→GPU 光栅（+0x29b 判 → ssbo.version 更新 → 清脏 → 建纹理 → on_texture_processed + write_texture + dispatch_2d） |
| garden_spawn_vegetation | 0x140a22b80 | O（render-loop W16） | Garden Marching Squares → /garden/marching_squares.cs.hlsl + spawn_small_flowers + spawn_{bushes,flowers}.cs.hlsl；Flower03Ssbo 键 0xF78C9C2EB9E1F6FB |
| garden_audio_init_system / garden_audio_system | 0x140a88fa0 / 0x140a89480 | O（audio-misc） | 花园绘制/擦除音效（4 循环 + update_draw_audio 模板） |
| garden_mask.rs | （name-only，5 派生胶水） | U（字段布局） | GardenMaskSsbo = 花园掩码回读缓冲（世界栅格 512² 域，read_terrain_data 槽 0x14）；无独立业务过程 |

## 2. 全链（画笔输入→掩码更新→绘制→历史，供 restorer 接线）

```
draw_garden(0x140ae4620)：工具 id∈{9,17} 门 → Circle2d 半径=brush_sizes.garden+1.5 → GladeBorder 界内
  → draw_raster<GardenRasterStagingSsbo>(world_raster 域 10 实例状态机：start_new→add_point→resample_and_smooth→rasterize_stroke→copy_to_staging)
  → WorldRaster<512> garden 槽（world_raster_defs 6 槽名序 …GardenRaster@0x142a93dd0）+ GardenRasterStagingSsbo
  → GPU：garden_rasterize_from_cpu(0x140a22140, W12) 掩码上传 → garden_spawn_vegetation(0x140a22b80, W16) 植被生成
历史：
  record_garden_history(0x141695e10)：Events<GardenDrawEvent> 游标差 → 12B 编辑单元 Arc → History::add
  replay_garden：replay_raster<Garden>(0x1409f6ff0) + generate_replay_cmds 闭包(0x141447500, 48B 命令 push)
音频：garden_audio_system(0x140a89480) 4 循环 + update_draw_audio
```

## 3. 修正记录

- replay_garden 章「closure$1::closure$0 = replay_raster\<Garden\> 薄包装，I」→ **O**：实为 48B 命令元素 Vec-push（generate_replay_cmds 命令构造闭包），与 history 域 ReplayCmd 0x30B 结构族同构。
- garden_mask.rs 维持 U（name-only；hash 家族在 0x142c2e210 区常量表，terrain-water DIGEST 已记）。

## 4. 未决（U，不臆造）

1. GardenMaskSsbo 字段布局（name-only SSBO 标记类型；无独立业务过程）。
2. garden_spawn_vegetation 各 FlowerSsbo/BushSsbo/GardenStoneSsbo 布局（render 域族级 U）。
3. Flower03Ssbo NAME（全二进制/SPIR-V/HLSL 穷尽无命中，round3 RENDER-U-CLOSURE 已登记卡点）。

## 5. 证据

- 既有：terrain-water/decomp/decomp-0x140ae4620.txt、decomp-0x141695e10.txt、decomp-0x140b6d290.txt、decomp-0x140a88fa0.txt、decomp-0x140a89480.txt；render-loop/decomp/decomp-0x140a22140.txt、decomp-0x140a22b80.txt。
- 本轮复核：history/decomp/decomp-0x141447500.txt（replay_garden 闭包全量）。
