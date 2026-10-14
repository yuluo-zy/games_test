# systems/render_loop/vermeer.rs

> 文件作用：Vermeer 风格化笔触揭示链（会话开始动画）：`VermeerState`（enum2$，判别 +0x48：0 待采集/1 已采集/2 未启用；+8 Option<揭示源纹理>，+0x3e==7=None；+0x54/+0x55 bool）、`activate`/`deactivate`/`into_disable_hold`/`is_active`/`should_freeze_camera` 状态机、`VermeerPlugin::build`、`vermeer_pass`（compute+raster 链）、`vermeer_render_from_target`、`startup_vermeer`。reconstruction `reconstruction/country-core/src/systems/render_loop/vermeer.rs` 已按机器码全量恢复（O）。

函数总计 13：业务 9 / 内联无独立符号 3 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::vermeer::vermeer_render_from_target` | 0x1408a8d70 | 0 | 从已采集目标渲染：`/vermeer/{generate_strokes.cs.hlsl:cs_main, raster_strokes.raster.hlsl, combine.raster.hlsl}`；串证据：vermeer_strokes、vermeer_combine、vermeer_raster_strokes、Vermeer debug target（combine pass 名）、vermeer_target；笔触数 STROKES_PHOTOMODE 0xa00 / STROKES_DEFAULT 0x8000、MAX_DT 0.0333、REVEAL_SECONDS 3.0、WARMUP_FRAMES 2（reconstruction 常量 O） | O | render-loop\decomp\decomp-0x1408a8d70.txt；reconstruction vermeer.rs |
| `country_core::systems::render_loop::vermeer::VermeerState::activate` | 0x1408a8060 | 65 | activate：`(+0x48 != 2 && +0x3e != 7)` 时先 drop 旧 +8 托管纹理；置 `*param_1=0`、+0x3e=7、+0x40=0、+0x48=0（待采集）、+0x54=param_2、+0x55=param_3 | O | render-loop\decomp\decomp-0x1408a8060.txt |
| `country_core::systems::render_loop::vermeer::VermeerState::deactivate` | 0x1408a8140 | 77 | deactivate：释放状态（单参 self，reconstruction O） | O | render-loop\decomp\decomp-0x1408a8140.txt；reconstruction vermeer.rs |
| `country_core::systems::render_loop::vermeer::VermeerState::into_disable_hold` | 0x1408a81e0 | 81 | into_disable_hold：阶段==2 时 panic "must be activated first"（UNK 文案 len 0x17，reconstruction O） | O | render-loop\decomp\decomp-0x1408a81e0.txt；reconstruction vermeer.rs |
| `country_core::systems::render_loop::vermeer::VermeerState::is_active` | 0x1408a8260 | 94 | is_active = `(+0x48 != 2)`（`cmp dword [rcx+0x48],2; setne al`，手解机器码） | O | 手解：`rne-read.py --hex 0x1408a8260 24`：`83 79 48 02 0f 95 c0 c3 …` |
| `country_core::systems::render_loop::vermeer::VermeerState::should_freeze_camera` | 0x1408a8270 | 97 | should_freeze_camera = `(+0x48 != 2) && byte+0x55`（`cmp[+0x48],2; setne; and al,[+0x55]; ret`） | O | 手解：`rne-read.py --hex 0x1408a8270 24`：`83 79 48 02 0f 95 c0 22 41 55 c3 …` |
| `country_core::systems::render_loop::vermeer::impl$1::build` | 0x1408a8280 | 111 | VermeerPlugin::build：注册 VermeerState 资源 + 系统（reconstruction O；NotFound 手解为 thunk 型插件 build） | O | render-loop\decomp\decomp-0x1408a8280.txt；reconstruction vermeer.rs |
| `country_core::systems::render_loop::vermeer::vermeer_pass` | 0x1408a82f0 | 137 | vermeer_pass（2679 B）：compute+raster 链组装；串证据：vermeer_target、Vermeer 各 pass 名 | O | render-loop\decomp\decomp-0x1408a82f0.txt；reconstruction vermeer.rs |
| `country_core::systems::render_loop::vermeer::startup_vermeer` | 0x1408aa720 | 443 | startup_vermeer：`load_shader_into_library` ×2（/vermeer/raster_strokes + combine；reconstruction O） | O | render-loop\decomp\decomp-0x1408aa720.txt；reconstruction vermeer.rs |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `max_stroke_count`（行 254）— photomode 判定笔触数上限（0xa00/0x8000）
- `raster_strokes`（行 347）— `_vermeer/raster_strokes.raster.hlsl` 光栅笔触
- `raster_combine`（行 395）— `/vermeer/combine.raster.hlsl` 合成

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（GenerateStrokes::pipeline_desc 的 `as_compute_pipeline` 胶水，reconstruction O）

## 关键事实

- Pass 名（O）："Vermeer acquire target" / "Vermeer generate strokes" / "Vermeer raster strokes" / "Vermeer debug target"（combine 原版名）；shader URI：`/vermeer/{acquire_target.cs.hlsl, generate_strokes.cs.hlsl:cs_main, raster_strokes.raster.hlsl, combine.raster.hlsl}`。
- 资源：`VermeerDebugControl`、`VermeerState`、`Events<WriteSettingsFileCmd>`、`AssetTextureLibrary`（TypeId 常量 O）。
- 与 painterly 共用 PassHost 缝（reconstruction 架构 O）。
