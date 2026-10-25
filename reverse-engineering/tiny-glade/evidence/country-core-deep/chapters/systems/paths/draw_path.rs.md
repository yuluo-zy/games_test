# systems/paths/draw_path.rs

> 文件作用：**路径绘制：UI 拖拽→DrawState 状态机→WorldRaster<512> 四变体绘制+互擦**

函数总计 27：业务 9 / 内联无独立符号 4 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::paths::draw_path::DrawState::update` | 0x140ae4bd0 | 64 | 系统:状态机 | O：DrawState::update——raw=ui_mode-2 夹取 0..3；raw∈{1,2}→mode=min(mode-1,raw)；返回 0/3 | collision-eco\decomp\decomp-0x140ae4bd0.txt |
| `country_core::systems::paths::draw_path::_::impl$0::apply` | 0x14003bb80 | 79 | Bevy 参数胶水 | O（round3 round2 复查修正）：0x14003bb80=ICF 折叠区（S_PUB32 437 符号，含 Plugin::finish/cleanup、iced Widget 空方法、SystemParam 胶水、marker 等），该址含 PathRastersResources SystemParam apply 符号 → 身份成立；机器码 1B ret = 无状态胶水 no-op （round1 曾误判“该址唯一符号=Plugin::finish”，身份未证实结论作废） | rne-read 0x14003bb80；S_PUB32 public-symbols.json |
| `country_core::systems::paths::draw_path::_::impl$0::get_param` | 0x141507a40 | 79 | Bevy 参数胶水 | O：SystemParam 胶水（apply/queue/validate） |  |
| `country_core::systems::paths::draw_path::_::impl$0::init_state` | 0x140ae5b20 | 79 | Bevy 参数胶水 | O：SystemParam impl | collision-eco\decomp\decomp-0x140ae5b20.txt |
| `country_core::systems::paths::draw_path::_::impl$0::new_archetype` | 0x14003bb80 | 79 | Bevy 参数胶水 | O（round3 round2 复查修正）：0x14003bb80=ICF 折叠区（S_PUB32 437 符号，含 Plugin::finish/cleanup、iced Widget 空方法、SystemParam 胶水、marker 等），该址含 PathRastersResources SystemParam new_archetype 符号 → 身份成立；机器码 1B ret = 无状态胶水 no-op （round1 曾误判“该址唯一符号=Plugin::finish”，身份未证实结论作废） | rne-read 0x14003bb80；S_PUB32 public-symbols.json |
| `country_core::systems::paths::draw_path::_::impl$0::queue` | 0x14003bb80 | 79 | Bevy 参数胶水 | O（round3 round2 复查修正）：0x14003bb80=ICF 折叠区（S_PUB32 437 符号，含 Plugin::finish/cleanup、iced Widget 空方法、SystemParam 胶水、marker 等），该址含 PathRastersResources SystemParam queue 符号 → 身份成立；机器码 1B ret = 无状态胶水 no-op （round1 曾误判“该址唯一符号=Plugin::finish”，身份未证实结论作废） | rne-read 0x14003bb80；S_PUB32 public-symbols.json |
| `country_core::systems::paths::draw_path::_::impl$0::validate_param` | 0x140b4b430 | 79 | Bevy 参数胶水 | O：SystemParam 胶水（apply/queue/validate） |  |
| `country_core::systems::paths::draw_path::PathRastersResources::as_float_grid` | 0x140ae4c20 | 88 | 查询/计算 | O：PathRastersResources::as_float_grid——4 路 f32 采样视图（CPU+8/+0x10 vs GPU 暂存 +0x20/+0x28，按 +0x298 标志） | collision-eco\decomp\decomp-0x140ae4c20.txt |
| `country_core::systems::paths::draw_path::draw_path` | 0x140ae4d00 | 129 | 系统:绘制 | O：draw_path——UI 模式串 "eraser"/"path"/"cobblestone" 分派 9 个 world_raster::draw_raster 单态（每变体绘制+擦除组合） | collision-eco\decomp\decomp-0x140ae4d00.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::paths::draw_path::DrawState::finish`（行 43）— 生命周期控制
- `country_core::systems::paths::draw_path::DrawState::start_or_continue`（行 50）— 生命周期控制
- `country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz`（行 108）— I：世界 xz 双线性采样（路径栅格；draw_path 内联）
- `country_core::systems::paths::draw_path::erase_other_path_variants`（行 429）— 删除/清理

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 14 项

## 实现方式（算法级）

完整算法见 `collision-eco/ALGORITHMS.md` §5。要点：

### DrawState::update（0x140ae4bd0，llvm-objdump 全解）
`r = clamp(ui_mode−2, 0, 3)`；`r==0 → mode=0`；`r∈{1,2} → mode = (mode−1≥2) ? r : 2`；`r==3 → mode = (mode−1<2) ? 3 : 0`；写回 self+0。

### draw_path（0x140ae4d00）主链
1. 模式字节 → `mode−5 ∈ {2,3,4}`（UI 模式 7/8/9）才处理；`==4` 分支先查标志 `(flags&5)!=0 → return`。
2. 笔刷 = `*资源 × 0.5525`（_UNK_142a95dd0），`assert brush.is_finite()`（0x142ae2e06）。
3. UI 串匹配（FUN_14092df50）：`"eraser"`（0x142ae2e80, len 6）/ `"cobblestone"`（0x142ae2f10, len 0xb）/ `"path"`（0x142ae2ef0, len 4）→ 匹配记录 {+0x18 标志字节, +0x38 名称 ptr, +0x19 变体字节}。
4. 笔刷矩形：cursor 位点（flag bit0 + x@+4 + z@+0xc）× 尺寸 = brush·0.8（_UNK_142a16cb8）。
5. 绘制状态推进（FUN_140ad2970 谓词）→ 历史命令 Vec push（8B 元素 {命令 id, 计数器}）+ 条件二推（记录标志>1 且名称匹配且变体≠2）。
6. 变体互擦：`variant = clamp(记录变体−2, 0..3)`；当前模式字节 ∈{1,2} → 置 2 并按记录擦除。
7. 9 个 `world_raster::draw_raster` 单态分派（0x140ab35f0/0x140ab6de0/0x140ab8090/0x140abba80/0x140ab2340/0x140ab0f10/0x140ab9340/0x140ab4750/0x140aba770，每变体绘制+擦除组合）。
