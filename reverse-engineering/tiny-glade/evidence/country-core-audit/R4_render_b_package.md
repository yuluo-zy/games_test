# R4：B 包（render 修正与 API 缺口）落码

日期：2026-10-14。依据：`render-gap/DIGEST.md` §2.2、§3、§6。落点：`tiny-glade/crates/country-core`。标注 O/I/U。

## 落码
| 项 | 内容 |
|---|---|
| 4 个 SSBO 元素类型 | `DofAutofocusSsbo`（64 B，`BUFFER_LEN` 8）、`DofFocusReadbackSsbo`（16 B，映射缓冲 memory 4）、`ShadowMaskHistogramSsbo`（u32×256）、`ShadowMaskMeanSsbo`（f32×1）。迁到各自**原版 NAME 路径所在的模块**（`compute_passes::{dof_autofocus, apply_screen_shadows}`），删除 `resources::render_ssbo` 里的 `Gap` 载体。布局来自重建 HLSL，`BufferDesc` 大小/用途/memory/元素大小来自 `new_storage_buffer` 汇编，`SsboId` 用 xxh3 对照 EXE 立即数 |
| `TypedStorageBuffer::new_storage_buffer` | 补 `BUFFER_LEN > 0` 单态化形态：固定大小、**用途始终 0x22**、memory 1、无数据不上传、超限 panic |
| `LastKnownGraphicsDevice` | `from_display`（0x140968c40）：先格式化物理设备、再 uuid（求值顺序 O），`+0` 放 uuid、`+0x18` 放物理设备串 |
| 3 个组件真名 | `BackgroundTerrain` / `DontReceiveContactShadows` / `OverwriteRoughness`（f32，缺省 0.8）替换 `Marker*`/`Float*` 占位，字段同步改名 |
| `shader_refactoring` | disc 5 的 panic 改为 `Result::unwrap` 失败文案；disc 6..=9 注明实际不可达 |
| 全局开关语义 | `*0x14362c0d8` = `freeze_occlusion_culling`；`*0x14362c098` = demo 开关（注释） |
| `dof` | 两个 `source_hash` 补地址标注（0x140a5d960 / 0x140a5de00） |

## 用汇编核对后发现的现有实现问题
1. **`TypedStorageBuffer::new` 对 `BUFFER_LEN == 1` 用 uniform 用途 0x12**，但 `new_storage_buffer<ShadowMaskMeanSsbo>`（0x141613720）的 `BufferDesc::new(4, 0x22, 1)` 是 **0x22**。原版有两个构造函数，此前 `BUFFER_LEN > 0` 的单态化形态是借 `new` 绕过的。现在 `new_storage_buffer` 自带该分支；`pass_kit::fixed_len_typed_storage`（仍经 `new`）对 `BUFFER_LEN == 1` 的类型仍有此偏差，**未改**（它的其它使用者都是 `BUFFER_LEN > 1`）。
2. 我先前以为 810 B 的过程是 `new_storage_buffer`，实为 `get_rg_buffer<T>`；它带来的有用信息是**完整类型路径**（`country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo` 等）——说明这些类型原本就属于 compute pass 模块，而不是 `resources`。

## 未做 / U
- `WallColorIdSsbo`：`new_storage_buffer`（0x142859840）的 `BufferDesc` 是 `0x14000 / 0x22 / 1`（元素 4 B，`BUFFER_LEN` 0x5000），且默认数据来自 `color_id_roof::impl$0::default_value`（0x14099ca50，非零表）；默认值表未分析，保留 typed gap。
- `DofAutofocusSsbo` 的 8 个元素各自的含义（每相机一行，按 `MainCamera +0x4d0` 的相机下标选行）U。
- `LastKnownGraphicsDevice` 的字段名/顺序为 I（来自序列化串）。
