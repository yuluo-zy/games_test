# Tiny Glade 三基线比对（A 首发 2024-09-23 / B 旧版 2025-10-08 / C v1.16.0 2026-09-23）

2026-10-10。本文登记三个发行副本的静态差异与互补用途,取代此前只覆盖 B/C 的两版比对稿。

命名约定（全文一致）:

- **A** = 2024-09-23 首发绿色版（3DM 绿色版 + TENOKE 破解）
- **B** = 2025-10-08 免安装版（RUNE 补丁）,即本目录此前唯一登记的"主基线"
- **C** = v1.16.0（2026-09-23）,ACG避难所打包 + RUNE 模板

## 范围与边界

- 证据等级：全文结论为 **E1（文件 / 符号 / 目录观察）**。凡属推断均显式标注。
- 本文**不改变** PROJECT_MEMORY.md 已锁定的依赖基线（其 SHA256 与 901 package 数对应 **B**）,亦不修改重建工程本身;C（及新增的 A）在此登记为**候选基线**,是否升级主基线由后续决策。
- 按本目录 [AGENTS.md](../../AGENTS.md)：引入新基线属于依赖适配。**任何绑定旧版地址 / 身份的差分报告,在切换基线前必须标记过期并重新校验**,不得直接沿用。
- 全程只做只读分析:未运行游戏进程,未修改任何发行副本。

## 1. 基线身份

| 项目 | A 首发（2024-09-23） | B 旧版（2025-10-08） | C v1.16.0（2026-09-23） |
|---|---|---|---|
| 本机路径 | `/Users/yuluo/Desktop/game/A版本内容` | `/Users/yuluo/Desktop/game/B版本内容` | `/Users/yuluo/Desktop/game/C 版本内容最新` |
| 项目文档记法 | 未登记 | `D:\game\ljxsj_92385\Tiny Glade` | 未登记 |
| 分发形式 | 3DM 绿色版（`UNINSTALL.exe`、`3dm/`、`3dmConfig.ini`、`启动游戏.exe`、`修改器/`、`_CommonRedist/`） | 免安装目录（RUNE） | WinRAR SFX 自解压（`XJ07036.exe`，overlay RAR5） |
| 破解 / 打包 | TENOKE（`Crack/steam_api64.dll` + `tenoke.ini`，appid 2198150） | RUNE（`steam_api64.rne` + `steam_emu.ini`） | RUNE 模板 + ACG避难所（`winmm.dll` + `steamclient64.dll` + `steam_emu.ini`） |
| 主二进制 | `tiny-glade.exe` 48,046,080 B | `tiny-glade.exe` 与 `tiny-glade.rne` 各 58,460,160 B | `tiny-glade.exe` 65,500,056 B |
| EXE `TimeDateStamp` | **2024-09-23T14:15:18Z** | **2025-10-08T10:36:14Z** | **2026-09-23T08:41:48Z** |
| `tiny-glade.exe` SHA-256 | `76c998d48a2404b105b431d6d881290969e786e993dd7ae824b6bedab95fd708` | `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323` | `d8f9261b1a4cb677fe91b8f5017bcff63b1312228b3f3264a2071ca7d3ad80a9` |
| rustc | **1.78.0** | **1.86.0** / LLVM 19.1.7 | **1.95.0** / LLVM 22.1.2 |
| `RUSTC_COMMIT` | `9b00956e56009bab2aa15d7bff10916599e3d6d6` | `05f9846f893b09a1be1fc8560e33fc3c815cfecb` | `59807616e1fa2540724bfbac14d7976d7e4a3860` |
| 编译配置 | O3 + `DEBUG=true` + `x86_64-pc-windows-msvc`（由 PDB 推得） | 同左（`toolchain-info.json` 证实） | 同左（`toolchain-info.json` 证实） |
| PDB 大小 | 213,172,224 B | 274,710,528 B | 310,013,952 B |
| PDB 块 / 流 | **52,044 / 4,707** | 67,068 / 4,892 | 75,687 / 5,140 |
| PDB GUID (age=1) | `{98388131-DB1D-D6B0-4C4C-44205044422E}` | `{09F1A02A-49F2-F8C8-4C4C-44205044422E}` | `{6A287060-44E4-AFA2-4C4C-44205044422E}` |
| PDB 剥离状态 | 未剥离（Debug Info / Types / IDs / Globals / Publics 全有） | 同左 | 同左 |
| `build-info/`（Cargo.lock、toolchain-info…） | **缺失** | 有（Cargo.lock 901 package） | 有（Cargo.lock 960 package） |

三个 PDB 是不同身份,不可互换:GUID 仅前 8 字节不同,尾部一致是 rustc 生成方案的结果,不能当作同一构建。A 没有随包 `Cargo.lock`,其 rustc 版本由 PDB 内 `/rustc/<commit>/` 路径反查得到（方法见第 7 节）。

## 2. 符号与源码规模

| 指标 | A | B | C |
|---|---|---|---|
| public symbol（`S_PUB32`） | 87,693 | 101,405 | 110,826 |
| 其中 `flags = function` | 63,334 | 71,606 | 80,627 |
| 其中 Rust legacy mangled（`_ZN…`） | 63,194 | 71,457 | 79,646 |
| PDB linker 模块数（`Mod NNNN \|`） | 4,709 | 4,894 | 5,142 |
| 不重复源文件（含 std / 依赖） | 5,230 | 5,768 | 6,412 |
| 其中 `.rs` 唯一路径 | 5,230 | 5,768 | 6,220 |
| 依赖目录数（`registry/src` 或 `git/checkouts` 顶层） | 367 | 403 | 444 |

规模增量主要来自依赖与引擎换代,而不是游戏自有逻辑翻倍（见第 3 节）。

## 3. 游戏自有源文件路径差异

按 PDB 中 `/home/h3/code/pouncelight/country-slice-private/` 前缀统计（相对私有仓库根,唯一路径）:

| 指标 | 值 |
|---|---|
| A 自有 `.rs` 路径 | **952** |
| B 自有 `.rs` 路径 | 1,044 |
| C 自有 `.rs` 路径 | 1,093 |
| A ∩ B ∩ C | **796** |
| A ∩ B | 915 |
| B ∩ C | 906 |
| A ∩ C | 797 |
| 仅 A 有 | **36** |
| 仅 B 有（对三版并集） | 19 |
| 仅 C 有（对三版并集） | **186** |
| A ∩ B 但 C 无（库内渲染栈,见 4.2） | 119 |
| B ∩ C 但 A 无 | 110 |

**仅 A 独有的 36 个文件**（多为被后续重构拆散的子系统前身,块状分布）:

- `crates/systems/clutter/src/clutter/*` 14 个:`add_birdnests`、`add_clutter_around_gates`、`add_clutter_around_windows`、`add_clutter_on_windows`、`add_lantern_water_setdresssing`、`add_preplaced_clutter`、`add_tree_vegetation_setdressing`、`clutter_animation`、`load_clutter_meshes`、`manage_clutter_entities`、`mod.rs`、`populate_clutter_regions`、`process_clutter_candidates`、`read_bush_positions_from_gpu`。
- `crates/country-core/src/systems/audio/*` 5 个:`audio_ui`、`music_audio`、`music_fade_in_on_start`、`sound_style`、`stairs_audio`。
- `crates/systems/color/src/*` 4 个:`set_flag_connection`、`set_roof_color_id`、`ui_color_roof`、`ui_color_wall`。
- `crates/systems/decorator/src/*` 4 个:`decorator_visual/cottage_dormers/mantain_dormer_tiles_draw.rs`、`decorator_visual/mantain_visual_entities.rs`、`handle_decorator_deletion_cmds.rs`、`signifiers.rs`。
- 其余 9 个:`country-core/src/resources/preplaced_clutter.rs`、`resources/world_raster_defs.rs`、`startup/load_glade_default_customization.rs`、`startup/load_glade_settings.rs`、`systems/render_loop/compute_passes/draw_path_mask.rs`、`systems/render_loop/draw_passes/blit.rs`、`systems/wall/snapping/snapping_mantain.rs`、`systems/wall/wall_outline.rs`、`systems/roof/src/ui_focused/ui_edit_roof_profile_focused.rs`。

**仅 B 独有的 19 个文件**（偏遥测 / 剖析基础设施与楼梯子系统）:`country-core/src/asset_libraries/texture_watch.rs`、`country-core/src/systems/backend/texture_watch.rs`、`country-core/src/render/debug_info_buffers.rs`、`crates/leekspin/src/{lib,ringbuffer}.rs`、`rhapsody/src/leek.rs`、`rhapsody/src/raw/{profiler,stats,workarounds}.rs`、`rhapsody/src/rg/state.rs`、`rhapsody/crates/aftermath-rs/src/{lib,sys}.rs`、`rhapsody/crates/gpu-profiler/src/{backend/ash,shared}.rs`、`systems/decorator/src/decorator_visual/{stairs,stairs_assemble,stairs_railings_assemble}.rs`、`systems/roof/src/utils/components.rs`。

**仅 C 独有的 186 个文件**（mod 体系 + SDL 后端 + 资源管线 crate 化）:

- mod 体系:`crates/tg-mods/src/*`（discovery / identity / legacy_texture / overlay 等）、`crates/turbosloth/src/*`（asset / export / id / registry / store 等）、`crates/clutter-icon-pipe`、`crates/clutter-pack-pipe`。
- 输入 / 后端重构:`crates/usdl/src/*` 与 `crates/usdl-sys/src/*`（SDL 绑定）、`tiny-glade/src/sdl_event_loop.rs`、`sdl_setup.rs`、`gui_runner_sdl.rs`,以及 `winit_event_loop.rs` / `winit_setup.rs` / `gui_runner_winit.rs` 的拆分。
- 资源管线:`crates/asset/src/{foliage,mesh,texture,legacy_mod}.rs`、`crates/asset-pipe/src/{mip_flood,store,texture_processing}.rs`。
- country-core 内:30 个 `systems/*`、17 个 `resources/*`、11 个 `startup/*`、4 个 `render/*`（含 mod 热重载、`swap_replacement_mods`、`bake_mod_textures` 等）。
- 另含 8 个 `target/x86_64-pc-windows-msvc/release/**` 路径（构建脚本产物泄漏进 PDB）。

主要 crate 的自有源文件数轨迹（`crates/x/src` 计,含同 crate 下所有子路径）:

| crate | A | B | C |
|---|---|---|---|
| `crates/country-core` | 491 | 539 | 557 |
| `crates/systems/decorator` | 64 | 74 | 86 |
| `crates/systems/wall-constructor` | 46 | 53 | 66 |
| `crates/tiny-glade` | 40 | 45 | 58 |
| `crates/utils` | 38 | 42 | 43 |
| `crates/systems/toolbar` | 37 | 38 | 49 |
| `crates/systems/roof` | 26 | 26 | 24 |
| `crates/systems/critters` | 24 | 24 | 24 |
| `crates/systems/clutter` | 20 | 20 | 20 |
| `crates/systems/onboarding` | 17 | 20 | 29 |
| `crates/systems/color` | 17 | 17 | 28 |
| `crates/systems/tree-tool` | 11 | 11 | 11 |
| `crates/rhapsody`（库内渲染栈） | 59 | 69 | 0（仅 6 个 `target/` 产物） |
| `crates/asset` + `crates/asset-pipe` | 0 | 0 | C 新增（见上） |
| `crates/usdl` + `crates/usdl-sys` | 0 | 0 | C 新增 16 |

## 4. 互补点

### 4.1 引擎基线的三代演进（对 PROJECT_MEMORY 的关键更正）

| | A | B | C |
|---|---|---|---|
| bevy 来源 | **git 分叉依赖** `bevy-72733c6739490ea8`,checkout `0dd724d` | crates.io **registry** | crates.io **registry** |
| bevy 版本 / 形态 | 无版本号;crates 目录仅 10 个 crate:`bevy_app / bevy_core / bevy_ecs / bevy_input / bevy_ptr / bevy_reflect / bevy_state / bevy_tasks / bevy_time / bevy_utils` | **0.16.0**（`bevy_app…bevy_utils` + `bevy_platform`,无 `bevy_core`） | **0.17.3**（同上,无 `bevy_core`） |

- **更正**：PROJECT_MEMORY.md / [工作区 AGENTS.md](../../AGENTS.md) 记录的"引擎为 Bevy 分叉"只对 **A** 成立。A 的 bevy 是 git 依赖且**含 `bevy_core`、不含 `bevy_platform`** → 属 0.14/0.15 线（`bevy_core` 在 0.16 被移除,`bevy_platform` 在 0.16 引入）。
- **B / C 用的是上游 crates.io bevy,不是分叉。** 因此"已证实 family 0.16.0"应改写为"**B** 使用上游 bevy 0.16.0"。
- 全流程真正私有的并非 bevy,而是渲染栈 **`rhapsody2`** 与 **`shabriri`**（C 的 `Cargo.lock` 用 `git+ssh://git@github.com/PounceLight/...`,其余 PounceLight fork 用 `git+https://`）。其仓库可见性未核实（见第 8 节）。

### 4.2 渲染栈由"库内"迁到"外部依赖"（最高价值）

- A / B 把整套渲染栈放在私有仓库内:`crates/rhapsody/**`（A 59 / B 69 个源文件,含 `raw/`、`rg/` 等）,外加 `crates/shader-variants`、`crates/gpu-allocator`（PounceLight 的 gpu-allocator fork,13 个文件）。
- C 中该栈已外置为 git 依赖 `rhapsody2-06daace556675b61`（checkout `48efa87`）,游戏仓库内只剩 6 个 `rhapsody` 路径,且全部是 build script 生成物（`target/.../out/shabriri.rs`）。
- 第 3 节的 **A∩B 但 C 无 119 个文件** 恰好就是这段库内渲染栈（51 个 `crates/rhapsody/src`、8 个 `crates/rhapsody/crates`、12 个 `crates/gpu-allocator/src`、43 个 `country-core/src` 渲染绑定、1 个 `shader-variants`、2 个 `tiny-glade`、1 个 `tg-iced`、1 个 `systems/roof/visual/roof_snow.rs`）。

**推断（非已证实映射）**：对 B 的独有文件按**同名文件**在 C 的 git 依赖中检索,74 个可在 `rhapsody2` 内定位,80 个可在任意 git 依赖内定位。示例:

| 旧版（库内） | C（rhapsody2 依赖内） |
|---|---|
| `crates/country-core/src/render/pass_builder/compute_pass_builder.rs` | `src/rg/node_builder/compute_pass_builder.rs` |
| `crates/country-core/src/render/pass_builder/draw_stream.rs` | `src/rg/node_builder/draw_stream.rs` |
| `crates/country-core/src/render/pass_builder/framebuffer_binding.rs` | `src/rg/node_builder/framebuffer_binding.rs` |
| `crates/country-core/src/render/shader.rs` | `src/raw/shader.rs` |
| `crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs` | `render-std/rhapsody-post/src/blur_pyramid.rs` |
| `crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs` | `render-std/rhapsody-core/src/reprojection.rs` |

同名匹配只是候选线索,**未做内容级 diff**,不能当成同一函数体。

对渲染器结构而言,A / B 提供"文件粒度组织方式"（A 更贴近首发形态,B 更完整）,C 只提供该栈作为依赖被调用的边界。

### 4.3 `assets-src` 资源管线 schema（仅 B）

B 含 `assets-src/textures/**/assets.ron` 共 16 个文件,描述纹理构建参数（`gamma` / `resize` / `mips` / `compression`,含注释）。**A 与 C 均已删除该目录。**

C 保留了对应的**消费者实现**:`crates/asset-pipe/src/texture_processing.rs`、`store.rs`、`mip_flood.rs`,以及 `crates/asset/src/texture.rs` / `mesh.rs` / `foliage.rs`。C 另有 `crates/asset/src/legacy_mod.rs` 与 `crates/turbosloth/**`,指向 mod 化资源管线。

B 的 RON（数据描述）+ C 的 Rust（消费者）= 资源管线的完整链路。

### 4.4 资源与样例的并集

| 子树 | A | B | C |
|---|---|---|---|
| `assets/` | 913 | 966 | 982 |
| `compiled-assets/` | 322（扁平） | 389（扁平未分包） | 753（`foliage-package` / `texture-package`） |
| `compiled-shaders/` | 361（扁平 `*.bin`,0 子目录） | 404（扁平 `*.bin`,0 子目录） | 343（11 个子目录;321 `*.precomp` + 入口清单等） |
| `legal/` | **无该目录** | 3 | 3 |
| `*.snapshot` | 31 | 36 | 34 |
| `history.json` | 30 | 33 | 35 |

- `history.json` / `*.snapshot`（`assets/starting-builds/**`）：A 30/31、B 33/36、C 35/34。`Version` 字段样本为 16、17。**三版取并集可扩大存档 / 回放语料**。
- `assets/` 交集:B ∩ C = 942（仅 B 24、仅 C 40）;A ∩ C = 867（仅 A 46、仅 C 115）。仅 A 的差异集中在 `starting-builds/full/*` 的 `history.json` / `screenshot.jpg` / `meta.json` 与 `ui/other/hint*.ron`;仅 C 的差异集中在 `meshes/clutter/*`（mod 与节日道具）与 `audio/Desktop/{Christmas,Halloween}.bank`。
- `compiled-assets` 与 `compiled-shaders` 在三版之间**内容格式互不兼容**（扁平未分包 → 扁平 → 按管线分包）,不能直接互换;A/B 的文件名自带信息（如 `_apply_screen_shadows.cs.hlsl_cs_main_0.bin` = 通道 + HLSL 入口名 + 变体哈希）,可当作 C 的 `shaders.list` 条目的交叉索引。
- 仅 A 有的资源示例（相对 B/C 命名差异）:`assets/meshes/clutter/barrel.json`、`assets/meshes/roof_button.json`、`assets/meshes/roof_delete_scribble.json`。
- 仅 C 有的资源示例:`assets/meshes/clutter/barrel_w_plantpot.json`、`assets/meshes/clutter/missing_mod_placeholder.json`（mod 占位）。
- 目录差异:A 有 `Tiny Glade Soundtrack/`（42 文件）、`修改器/`、`3dm/`;B 有 `assets-src/`（16 RON）、`rune_controller_glyphs/`（287 手柄图标）;C 有 `Tiny Glade Soundtrack/`（约 1.2 GB）、`assets/steam-controller/`（3 文件）。B 是**唯一没有原声带**的版本。

### 4.5 依赖与引擎换代

- 引擎:bevy **A = 库内 fork（无版本号,0.14/0.15 线）→ B = 0.16.0 → C = 0.17.3**（见 4.1）。
- glam:A = 0.20.5 / 0.24.1 / 0.25.0 / **0.27.0**;B = **0.29.3**;C = **0.30.10**。（PROJECT_MEMORY 记录的 `glam 0.29.3` 对应 **B**。）
- registry index 目录:A = `index.crates.io-6f17d22bba15001f`;B / C = `index.crates.io-1949cf8c6b5b557f`。
- git 依赖（`git/checkouts` 顶层,按 checkout 哈希）:

| | A | B | C |
|---|---|---|---|
| 总数 | 13 | 13 | 14 |

  - A 独有:`bevy-72733c6739490ea8`、`libfmod-66c5e7ea7338d409`。
  - B 独有:`hotwatch-c8b1849e5bae8439`、`intel-tex-rs-2-9fa17cc65b3a48f9`。
  - C 独有:`rhapsody2-06daace556675b61`、`shabriri-635d53344d427385`、`json-ebf993c3cc323e18`。
  - **B / C 共享同一 checkout 哈希的 11 个**:`append-only-vec`、`cosmic-text`、`crash-handling`、`hotwatch`、`iced`、`intel-tex-rs-2`、`obvhs`、`rapier`、`steamworks-rs`、`winit`、`zeno`。（`rkyv` / `ron` 在 B 为 git、在 C 已不属于 git 依赖,故不计入这 11 个。）
  - A 与 B **没有任何共享 checkout 哈希**（同仓库不同 revision）。
  - `rkyv` / `ron`:A、B 为 git 依赖,C 改为 crates.io `rkyv-0.8.16` / `ron-0.12.0`。
- C 的 `build-info/Cargo.lock` 记录 960 个 package;B 的为 901 个（与 PROJECT_MEMORY 的 SHA256 记录一致）。

**推断**：由于三版同源且 796 个源文件路径一致,同一逻辑跨版函数级 diff 可用于区分"真新增功能"与"随引擎迁移的改写",比单版本盲扫更快。该推断未经验证。

### 4.6 其他

- A 的 `tiny-glade.exe` 直接是 TENOKE 破解产物（`Crack/`），无 `.rne` 对照;B 的 `tiny-glade.exe` 与 `tiny-glade.rne` 同尺寸、`.text` 内 6 字节差异（沿用既有记录），可用于"原版 vs 补丁"对照;C 无 `.rne` 副本。
- B / C 的 `build-info/`（`toolchain-info.json`、`manifest.json`、`credits-crates.txt`、`default_settings.ron`）结构一致,可直接字段级对比;A 无 `build-info/`,但顶层 `manifest.json`（`version: 1` 的校验清单）与 `default_settings.ron` 仍在。
- A 的源码根 `/home/h3/code/pouncelight/country-slice-private/` **同样出现在 PDB 中**（见第 5 节更正）。

## 5. 风险与注意

1. **证据绑定**：现有 `evidence/` 与 `tracking/` 全部绑定 **B**（RNE SHA256 `f507a6ce…`、函数地址如 `edit_roof 0x1421ab7c0`、机器码差分用例、Ghidra 持久库）。切换或引入 A / C 前必须重新绑定,不能混用地址或报告。
2. **依赖假设更正**：PROJECT_MEMORY.md 记录的"已证实 Bevy family 0.16.0"只覆盖 **B**;A 实测为 **git 分叉**（0.14/0.15 线,含 `bevy_core`）,C 实测为 **上游 0.17.3**。不得据版本号推演完整渲染等同上游。
3. **路径情报更正（相对两版稿）**：三份 PDB **都**含有私有仓库根 `/home/h3/code/pouncelight/country-slice-private/`,A 与 C 连 `symbols.o` 都未重映射;B 仅把 `symbols.o` 重映射到 `/tmp/rustce5EQOB/`。此前"旧版已重映射为 `/tmp/rustXXXXXX/`"的说法不准确。
4. **PDB 身份不可互换**：三版 GUID / 符号数 / 源文件表均不同,任何符号查询都要先确认绑定的 PDB。
5. **C 的 mod 体系是新增面**：186 个 C 独有文件里 mod / SDL / 资源管线占绝大多数,分析 C 会显著扩大范围;若目标是复刻首发玩法,A 的 36 个独有文件与 B 更贴近。

## 6. 建议用法

- 主基线候选:**C**（符号最全、源文件最多、`Cargo.lock` 完整、为当前线上版本）;若以"首发形态 + 复刻"为目标,A 是最小、最干净的参考。
- 补丁来源（不改主基线的部分）:
  - 引擎分叉证据 / 最小结构参考 / clutter·audio 前身 → **A**;
  - 渲染栈模块组织与着色器入口索引 → **B**（A 次之）;
  - 资源管线 schema（`assets-src/*.ron`）→ **B**（唯一持有者）;
  - mod 体系 / SDL 后端 / 资源管线 crate 化 → **C**;
  - 存档 / 回放语料 → 三版取并集;
  - 引擎迁移差异与函数级 diff → **A↔B↔C** 三向对照。
- 方法论可复用:三版均为 O3 + `debug=true`,行号表与源文件 SHA-256 齐备,现有"证据绑定 → Rust 实现 → 原机器码差分"流程无需改动。

## 7. 证据与复现

工具:`/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil`。

```bash
A="/Users/yuluo/Desktop/game/A版本内容"
B="/Users/yuluo/Desktop/game/B版本内容"
C="/Users/yuluo/Desktop/game/C 版本内容最新"
PDBUTIL=/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil

# 对 A/B/C 各执行四条;输出前缀任意（本文用 v100_ / old_ / v116_）
$PDBUTIL dump -summary "$A/tiny_glade.pdb"
$PDBUTIL dump -files   "$A/tiny_glade.pdb" > /tmp/v100_files.txt
$PDBUTIL dump -modules "$A/tiny_glade.pdb" > /tmp/v100_mods.txt
$PDBUTIL dump -publics "$A/tiny_glade.pdb" > /tmp/v100_pub.txt
```

- 源文件路径以 `- (SHA-256: …) <路径>` 行抽取;游戏自有路径判据为包含 `pouncelight`;派生路径相对 `country-slice-private/`。
- **function 计数**：`S_PUB32` 的 `flags = function` 在条目**下一行**,统计时需看相邻两行。
- 依赖归属判据:路径含 `.cargo/registry/src/` 或 `.cargo/git/checkouts/`（`checkouts` 后第一段即 checkout 目录名,含哈希）。
- 目录树差异用 `os.walk` / 路径集合比对。
- **rustc 版本反查（A 无 `toolchain-info.json` 时的唯一途径）**：抽取 PDB 内 `/rustc/<40 位 commit>/` 路径,与 `https://static.rust-lang.org/dist/channel-rust-<版本>.toml` 的 `git_commit_hash` 比对。已交叉验证:A `9b00956e…` = 1.78.0;B `05f9846f…` = 1.86.0（与同包 `toolchain-info.json` 一致）;C `59807616…` = 1.95.0（与同包 `toolchain-info.json` 一致）。
- 本机未落盘持久副本;上述命令可完整重放。若需留存,可在 `evidence/multi-baseline/` 下归档这三组导出。

## 8. 待确认

- `rhapsody2` / `shabriri` 仓库的可见性与 commit 存在性（GitHub 直连在本机返回 HTTP 000,未核实）。
- A 的 bevy 分叉能否定位到具体上游版本线与仓库（目前仅凭 crate 集合推断为 0.14/0.15 线）。
- A 的 36 个独有文件中,有多少是**被删除**、多少只是**改名 / 拆分**（未逐个验证）。
- 4.2 的渲染栈同名映射未做内容级 diff,未确认是同一实现。
- A / B / C 的 `assets/` 单侧文件（B∩C = 942,仅 B 24 / 仅 C 40;A∩C = 867,仅 A 46 / 仅 C 115）未逐项区分"新增功能"与"重命名"。
- B 的 `tiny-glade.exe` 与 `tiny-glade.rne` 的 6 字节 `.text` 差异目的未知（沿用既有记录）。
