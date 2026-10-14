# render-gap 定点闭合 DIGEST（owner: tg_rg1）

日期：2026-10-14。任务来源：`evidence/country-core-audit/RENDER_GAP_ANALYSIS.md`（§2.1 未引用 19 单元、
§3.1 残留 stub、§5.2 渲染 U 项、§5.4 公共 API）。范围：只恢复 CPU 侧接线/类型/控制流，GPU 算法后置。
标注 O（机器码/反编译实测）/ I（推断，注明依据）/ U（未知，不编造）。

新反编译落盘：`evidence/country-core-deep/render-gap/decomp/`（17 个函数；Ghidra db `tg_rg1`）。
不含与既有证据重复的函数（`render_shader_refactoring` 0x14098f9f0 复用 render-loop/decomp，本轮只补跳转表机器码分析）。

---

## 1. §2.1 未引用 19 单元（逐一反编译核实）

| # | 地址 | 单元 | 原判定 | 实测 | 结论 |
|---|---|---|---|---|---|
| 1 | 0x140555a90 | `asset_libraries::impl$21::register_required_components` | 真实逻辑，需补（I） | 反编译 + typeid_resolve：callees = `enforce_no_required_components_recursion`/`QueuedRegistration::register`/`HashMap::insert<TypeId,ComponentId>` = bevy `#[derive(Component)]` 生成；内含 `Handle<Vao>` 的 TypeId（0x93c2ea0b5dbce370，O） | **胶水**（derive(Component) 泛型实例 `<Handle<Vao>>`）。`Handle<T>` 已在 `asset_libraries/mod.rs:57` derive Component，无需补业务。**纠错**：RENDER_GAP 猜测"bevy Resource/组件注册钩子需补"不成立 |
| 2 | 0x14003ae40 | `render_config` deserialize `visit_str` | serde derive（I） | 反汇编：`visit_str<ron::error::Error>`，8 字节窗口字符串比较 + 跳转表 = serde Deserialize visitor | **胶水**，不补（字符串枚举名：`"Monitor"/"Tgsr"` 等，见 0x14292c064 区） |
| 3 | 0x140039c10 | `render_config` `serialize` | serde derive | 反编译：`serialize_struct("MonitorMatch",3)` + 字段 `name/physical_position/physical_size`（serde json 助手族 FUN_140350320 等） | **胶水**，不补。字段名与 reconstruction `MonitorMatch` 一致（O） |
| 4 | 0x1404d7d50 | `render_config::impl$38::clone` | 多半 derive(Clone)（I） | 反编译：逐字段拷贝 ~0xc0 B 的配置结构 + 2 个字段专用 clone（FUN_1402b8400/FUN_1402b7890，Option/Vec 路径）+ `i64::MIN` niche 检查 | **胶水**（derive(Clone) 展开；`impl$38` 属 `VideoSettings` 一类大结构）。该结构布局（偏移 0x18..0xbd+）留作类型合同参考 |
| 5 | 0x140322a10 | `atmosphere` `serialize` | serde derive | 反编译：`{` + 4 字段 `sun_tint_color/moon_tint_color/sky_low_color/sky_high_color`（串 0x1429bd228/236/245/252）+ `}`；4 个字段各 0xc B（Vec3），偏移 +0/+0xc/+0x18/+0x24 | **胶水**，不补。旁证：该类型 = **`render::atmosphere::DerivedAtmosphere`（4×Vec3，48 B）**——`SunSkyColors::lerp`（0x140a97650）读 3 个 xmmword 于 +0/+0x10/+0x20（3×Vec4），故 serialize 对象不是 SunSkyColors，而是同模块的 `DerivedAtmosphere`（O 字段名/间距；类型名由模块 procs 定） |
| 6 | 0x140968c40 | `render_config::impl$7::from` | **U**：可能 From 转换 | 反编译：`From<&GraphicsDevice>` 类转换 → `{String, String}`（48 B）；String A = `format!("{}", physical_device)`（rhapsody `physical_device::impl$3::fmt`，源 @+0x58），String B = `uuid.to_string()`（`uuid::fmt::impl$1::fmt`，源 @+0x78）；失败 → panic "a Display implementation returned an error unexpectedly" | **真实逻辑**（小自定义 From，非 derive）。语义与 `LastKnownGraphicsDevice{uuid,driver}`（render_config 序列化串 0x14292b810 区含 "LastKnownGraphicsDevice"/"uuid"/"driver"）吻合（I）。reconstruction 未实现，建议补 `impl From<&GraphicsDevice> for LastKnownGraphicsDevice` |
| 7 | 0x1409bdc60 | `build_draw_lists::impl$1::fmt` | Debug/Display，低优先 | 反编译：跳转表 0x142abf408 按判别式分派 | **胶水**（enum Debug/Display），不补 |
| 8 | 0x1412e9e10 / 0x1412ea160 | `asset_libraries::impl$11/12::fmt` | fmt，低优先 | 反编译：`impl$11::fmt` 委托 `impl$12::fmt` 写 `"HandleId"`（串 0x142c2f488，位置 `crates/country-core/src/asset_libraries/mod.rs`） | **胶水**（`Handle`/`HandleId` 的 Debug），已在 `asset_libraries/mod.rs` 实现，不补 |
| 9 | 0x1402b15c0 | `ssbo::impl$22::fmt` | fmt | 反编译：判别式 == 0xb → `"OutOfBounds"`（0x1429ada3f），否则 `"BackendError"`（0x1429ada70） | **胶水**（render::ssbo 错误 enum 的 Debug），不补 |
| 10 | 0x140a241d0 | `nani::gpu_world::impl$44::fmt` | U（机器码与 PDB 归属不符） | 反编译：写 `"InstanceKey"`（0x142ac9ad0，0xb 字符） | **胶水**（实例键 enum 的 Debug）。**纠错**：不是"归属不符"，就是 `GpuWorldInstanceKind` 类 enum 的 Debug 单态化 |
| 11 | 0x140a5de00 / 0x140a5d960 | `dof::impl$11/$7::source_hash` | **需补**（c1 漏标） | 反编译：`calculate_hash(&("uri:entry", self))`；impl$11 串 `/dof/fullres.cs.hlsl:cs_main`（0x142ad34d8，0x1c），impl$7 串 `/dof/fullres_all_lod1.cs.hlsl:cs_main`（0x142ad3478，0x25） | **真实逻辑，但已实现**：`compute_passes/dof.rs` 的 `dof_source!` 宏 `source_hash()` 与机器码逐位一致。只需在 `FullResDof`/`FullResAllLod1Dof` 文档补 0x140a5de00/0x140a5d960 地址标注 |
| 12 | `as_any/as_any_mut/handle` ×~5 | `ssbo_library`/`sparse_instance_buffer` | 平凡 getter，已在 `ssbo_library.rs` 以其他形式实现 | — | **胶水**（`ssbo_carrier!`/vtable getter），只缺地址标注，不补 |

**小结**：19 单元中 **真实逻辑 2 项**（`impl$7::from` 0x140968c40 待补、dof 两个 `source_hash` 已实现仅需标注），**胶水 16 项**（register_required_components、render_config×3、atmosphere、build_draw_lists fmt、asset_libraries fmt×2、ssbo fmt、nani fmt、getter 族），1 项（visit_str）已反汇编确认 serde。全部核实完毕。

---

## 2. §3.1 残留 stub

### 2.1 grading_lut.rs 7 处 unimplemented（0x140b00b20 / 0x140b00bc0 / 0x140b01050 / 0x140b01cc0 / 0x140b01df0 + 2 内联）

audio-misc 域（tg_a8）已全量反编译并写入 `audio-misc/DIGEST.md §2.9` 与
`chapters/resources/grading_lut.rs.md`，decomp 在 `audio-misc/decomp/`（load_cube 0x140b01050 另有一份
`reconstruction/country-core-render/wp19/analysis/decomp/load_cube_140b01050.c`）。核对要点（O）：

- **`flattened` 0x140b00b20**：`{name String, 4×f32, byte}` 扁平化（无独立 decomp 文件，DIGEST O）。
- **`load_unwrapped` 0x140b00bc0**：展平 PNG → RGBA8 → 3D 纹理，路径前缀 `"assets/luts/"`。
- **`load_cube` 0x140b01050**（**PDB 名 `asset_name` 错位**）：完整 `.cube` 解析器——TITLE 跳过 / `#` 注释跳过 /
  `DOMAIN_MIN` 必须 (0,0,0)（`"DOMAIN_MIN must be 0 0 0"`）/ `DOMAIN_MAX` 必须 (1,1,1) /
  `LUT_1D_SIZE` → `"1D LUTs are not supported"` / `LUT_3D_SIZE` 手写十进制 u64 解析 / `size³` 数组 /
  数据行 → `parse_rgb` → **RGB9E5 打包** / 超 `size³` → `"Too many entries in the LUT. Bailing before an overrun"` /
  行数不足 → 越界 panic（grading_lut.rs）/ 3D 纹理名 `"grading lut"`。
- **`parse_rgb` 0x140b01cc0**：`&str` 空白切分 → 3×f32。
- **`GradingLuts::load` 0x140b01df0**：read_dir `"assets/luts"` → 逐 DirEntry（闭包 0x140ae5bb0，按扩展名分派）→ 双 Vec（A stride 0x28、B stride 0x38）。

**落码依据**：以上 5 项的算法/串/panic 文案全 O，可作为替换 `grading_lut.rs` 内 7 处 `unimplemented!` 的实现依据。
（本任务不改源码；改由对应 owner 落码。）

### 2.2 render_ssbo.rs 5 处 typed-gap 载体

| 载体 | 真实元素（O，重建 HLSL） | 元素布局 | 现状态 |
|---|---|---|---|
| `DofAutofocusSsbo` | `DofFocus`（`inc/coc.hlsl:12`） | `{focus0,focus1,lock_bias0,lock_bias1 f32, lock_pos_ws float3, is_locked u32, interp_lock_pos_ws float3, use_tilt_shift u32, interp_lock_bias0/1 f32, focus_scale f32, pad2 u32}` = 64 B | 载体可补 `#[repr(C)] struct DofFocus` + `StorageElement` |
| `DofFocusReadbackSsbo` | `DofFocusReadback`（`inc/coc.hlsl:28`） | `{lock_pos_ws float3, lock_reponse u32}` = 16 B（成员名原拼写 `lock_reponse`） | 同上 |
| `ShadowMaskHistogramSsbo` | `RWStructuredBuffer<uint>`（apply_screen_shadows） | u32[256]（`InterlockedAdd(..[clamp(..,0,255)],1)`） | 载体可补 `u32` 元素，BUFFER_LEN=256（I） |
| `ShadowMaskMeanSsbo` | `RWByteAddressBuffer`（`shadow_mask_mean.cs.hlsl`） | 1×f32（`Load<float>(0)`） | 载体可补 f32，BUFFER_LEN=1（I） |
| `WallColorIdSsbo` | — | NAME `"WallColorIds"`（O，`bind_resources.rs:214`），SsboId `0xaf42315d263aa22a`（O） | 已有载体，只缺元素布局 |

**结论**：5 处载体全部有真实元素形态（4 个来自重建 HLSL，1 个已建模），属可闭合，不属"永不恢复"。
仍需 `new_storage_buffer<T>` 单态化的 NAME/BUFFER_LEN 机器码核对（U，超出本轮）。

### 2.3 各 compute_passes Gap

c1/c2/c3（`B_c3.md`/`B_c2.md`）已清除 19+15 个 pass 文件的 `todo!/unimplemented!`；`Gap` 占位均为
`ssbo_carrier!`/`foreign_carrier!` 载体（LeafParticleSsbo/ParticleEmitterSsbo/GrassBladeDataSsbo/StairsTransformSsbo/
BushSsbo/FlowerSsbo/Flower03Ssbo/GardenStoneSsbo/ShadowMask\*Ssbo/BoatSsbo/RoofTileSsbo 等），已实现
`StorageElement`+库 `SsboType`（`pass_kit.rs:320-354`）。**不属遗留**。`Flower03Ssbo` 的库 NAME 仍 U（B_c2 已列）。

---

## 3. §5.2 渲染 U 项

### 3.1 shader_refactoring 状态机 disc 6..=9（0x14098f9f0，本轮机器码精确化）

两跳转表目标全部计算并对照反汇编（表 0x142ab9ff0 / 0x142aba004，4 字节偏移相对表基）：

**第一张跳转表（disc → 目标）**：
| disc | 目标 | 行为（O） |
|---|---|---|
| 0 | 0x14099063a | 直接返回（不碰 tick）✓ |
| 1 | 0x14098fc78 | kind = 2 |
| 2 | 0x14098fc49 | kind = `payload&3==3 ? 0 : 2` |
| 3/4 | 0x14098fc3e | kind = `payload&3==3 ? 1 : 2` |
| **5** | 0x140990721 | **跳进 ref-temporal `Result::unwrap` 失败 thunk 中部**（`xchg;xchg;adc; lea r8,[rbp+0x40]; mov edx,0x2b; call unwrap_failed`）→ **divergent panic**。**纠错**：c3 写 `5→unreachable!()`，实为 `Result::unwrap()` panic（不是 `unreachable!`） |
| **6** | 0x14099061b | 跳进第二表 disc2 更新处理器中部（跳过 `mov ecx,[r11+4]`/`lea r8d,[rcx+1]`）；`cmp ecx,0x4` 用第一跳前遗留寄存器（ecx=表基低 32 位）→ CF=0 → edx=3、eax=0 → 写 `{disc:3,payload:0}` + 写 tick 到未初始化的栈槽 → **退化**（帧槽 [rbp+0x3d0/3c8/3dc] 未设，指向垃圾地址） |
| **7** | 0x1409905ff | 跳进 desc-tail 拷贝块中部（`movdqa xmm1,[rbp+0x20]` 中部）→ 同退化 |
| **8/9** | 0x1409905d2 | 跳进 `mov edx,0x2` 指令中部 → 同退化 |

**结论**：disc 5 = divergent panic（Result::unwrap）；disc 6..9 = **编译器合并进共享收尾代码的退化分支**，
帧槽未初始化 → 写垃圾地址（UB）。**关键事实：第二表更新规则只产出 disc ∈ {0,2,3,4}（1→{2,0}；2→{2,payload+1}|{3,0}；3/4→{3|4,payload+1}），
故 disc 5..9 实际不可达**。reconstruction 的 `_ => None // 6..=9：U` 按 no-op 是**安全且正确的近似**；
应把注释改为"不可达（更新规则不产出 disc≥5；disc5 若进入则经 Result::unwrap panic）"。`desc_tail_copied`（disc3/4 的
`[frame+0x20] = feedback desc 尾字`）机器码确认（0x1409905e6..0x140990611）。

### 3.2 reset_indirect_args 的 0x1e0 B 暂存区（0x14090a870）——**U 项闭合**

B_c3 判"0x1e0 B 暂存区无任何写入（U）"。反编译 0x14090a870 显示：
```c
FUN_140c02740(..., 0x1e0, 0);                       // allocate staging 0x1e0
if (lStack_150 == 0x1e0) {                           // 长度校验
  func_0x000142923c00(puStack_158, &UNK_142aaaa78, 0x1e0);  // memcpy(staging, &blob, 0x1e0)
```
**暂存区是被 `memcpy` 从静态表 0x142aaaa78（0x1e0 B）写入的**。表内容（O）：
24 × 20 B `DrawIndexedIndirectCommand`，仅 `index_count` 非零：
`[0x1b,0x12c,0x60,0x39,0x24,0x6,0x3c,0x6,0x30,0xae,0,0,0x10b,0x126c,0x42,0xb4,0xf,0x384,0x12,0x384,0x12,0x12,0x12,0x12]`，
其余四字段（instance_count/first_index/vertex_offset/first_instance）全 0。
**纠错**：`reset_indirect_args.rs` 现在把 0x1e0 暂存区当"不写"（`STAGING_SIZES[1]` 用 0xAB 填充测试断言），
应改为 `index_init()`（上述 24 表）。另三块初值复核一致：0x80 = 8×`[0,1,1,0]`（0x142aaa1e0/e8）、
0x10 = `[0,1,0,0]`（0x142aaaf74）、8 B = `0x0009000000000000`。串 "Reset indirect args"（0x142aaaf60，0x13）。
长度不符 panic 源 `rhapsody/src/raw/dynamic_constants.rs`（0x142a956c0）。

### 3.3 CENTERED_PATH（`*0x14362c308`）——**写入者确认不存在**

- `*0x14362c308`（qword，.data 初始值）= `0x143638fac`（O）；读路径 `cmp byte ptr [rax],0`（B_x_out O）。
- 二进制扫描：qword `0x143638fac` 在整个 EXE **只出现一次**（即 .data 初始值本身 0x14362c308），
  **.text 无任何绝对地址引用**；`*0x14362c308` 的读取均经 `[rip+disp]`，写入者为零。
- **分段确认（O）**：0x14362c308 与 0x143638fac 同在 PE sec2（va 0x362b000）；sec2 raw 只到 0x3634c00，
  `0x143638fac`（=0x362b000+0x488fac）在 **rawsize 之外 = 零填充区（bss 类）→ 加载时为 0**，且无写入者。
  （rne-read 对零填充区返回的 `0x50…` 是越界读到邻段 raw 的假象，不可用。）
- 结论：**CENTERED_PATH 无运行期写入者，标志恒 0** → 恒走"向量路径"（==0 分支）。
  B_x_out 的 `AtomicBool(false)` 镜像成立，`float_grid.rs` 无需改（**闭合 U，标注为"静态初值 0，无写入者"**）。

### 3.4 全局 `*0x14362c0d8` / `*0x14362c098`——**语义已还原**

两者都是**指针全局**（.data 初始值，qword），读法 `mov rax,[0x…]; cmp byte ptr [rax],0; jne skip`（render_world 0x1404ec5b1 / 0x1404eca6c）：
- **`*0x14362c0d8` → 0x143634da0** = `freeze_occlusion_culling` 调试开关（O，grass `B_c2.md` 已命名该地址）。
  非 0 → 跳过 `generate_wall_draw_lists_pass2` + `gpu_generate_draw_lists`（`render_world.rs:236` 注释更新为 O）。
- **`*0x14362c098` → 0x143634d90** = 进程级 **demo 开关**（O，`lut_gallery`/`gpu_lut`/`vermeer` 共用的进程布尔，
  `B_s_h.md` S-B 已记录）。非 0 → 跳过折射/水面 pass 链（`transparency_pass.rs:77`/`render_world.rs:238` 注释更新为 O；
  demo 下跳过水折射的语义为 I）。
- 与 CENTERED_PATH 同形：`.data` 指针 → 零填充区字节标志（0x143634da0/0x143634d90 均在 sec2 rawsize 之外，
  加载时 0；写入者属各 owner，本项只还原指针全局语义）。`render_world.rs:236/238` 的"语义 U"可改写为
  `freeze_occlusion_culling`/`demo`。

### 3.5 roof_tiles_gravity 的 f32 uniform 键 `0x601a45ce43af912f`——名字仍 U，绑定行为精确化

反编译 0x140a29b10（+ 闭包 0x14130d8e0/0x1415f9210）：
- `ComputePass::new("Roof tiles"→名字 len 0xa / uri 0x1b)`；`bind_texture(0xc3483cd70105de2f)`；
- 资源 `TypeId 0x4bc362195b80f729 / 0x982798ce80cbcefb`（RoofTileSimulation 族）**在场且门控字节
  `cStack_282 != 8` 时**：`bind_uniform(0x601a45ce43af912f, *(u32*)(buf+0x28))`（f32，O）。
- 名字经 FNV-1a-64 反解（候选集 ~90 个：gravity/roof_gravity/frame_constants_storage_buffer/delta_time_seconds/
  shingle/RoofShingle 等）**均不吻合**，重建 HLSL 无此标识符 → **保持 U**（B_c2 结论不变）。
- 门控 `cStack_282 != 8` 的字节语义 U。
- 闭包 0x1415f9210：20 B PushConstants 数组逐项 `push_constants` + `dispatch_3d(count,1,1)`；数组遍历后按
  `len*0x14` dealloc。与 `roof_tiles_gravity.rs` 现有 `dispatch_with` 一致。

### 3.6 三个 TypeId 组件——**真名已反查**

`typeid_resolve`（`component_ids` bundle 泛型全名 + env27 闭包读序 + f32 缺省 0.8）：
| TypeId（低 64） | 真名（O） | 语义（O） |
|---|---|---|
| `0x66476a13bbcc5e41` | `components::BackgroundTerrain` | `contains` 标记（存在 → 绑磁盘键 0xbd39551337285fb6） |
| `0xa6c0fc01ec67ef4c` | `components::DontReceiveContactShadows` | `Option` 读 → `bool(!has)` |
| `0x192600af2434483b` | `components::OverwriteRoughness` | f32，缺省 0.8 |

证据链：三常量同现于 `bevy_ecs::bundle::impl$24::component_ids<KeepOnLevelUnload, DrawableMeshBundle,
BackgroundTerrain, DontReceiveContactShadows, OverwriteRoughness, …>`（0x1416dcd10，bundle 顺序）；env27 读序
（contains→f32→Option）；`OverwriteRoughness` 的 f32 本性。**确认 `components/mod.rs:70/86/106` 的候选名**。
`draw_callbacks_a.rs:431-433` 的 `MarkerU66476a13`/`FloatU192600af`/`MarkerUa6c0fc01` 占位可替换为真实类型。

---

## 4. §5.4 公共 API

### 4.1 `TextureDesc::from(preset)`（0x140a59890，`resources::texture_presets::impl$1::from`）——已反编译

- 本质：`impl From<TexturePreset> for TextureDesc`，**switch on preset（0..0x1f）**；每 case 经 rhapsody
  `TextureDesc::{new_2d/new_2d_array} → sampled → storage/color_attachment/depth_stencil_attachment/mip_levels`
  建 48 B 的 desc，并在 **+0x20 嵌入 `NameHash`**（xxh3_64(预设名)，O 立即数）。
- **纠错**：reconstruction `texture_presets.rs` 把 0xa59890 当作 `impl From<u8> for TexturePreset`（`From<u8>` 注释
  "O 0xa59890"）——**错位**。0xa59890 是 `From<TexturePreset> for TextureDesc`。`From<u8> for TexturePreset` 无机器码证据。
- `usage()` 的错误：预设 11/12 实为普通 `storage`（`new_2d(0x200,0x200, 0x63/0x4c)`），**不是 depth attachment**；
  仅 0xa(10)=depth_stencil_attachment、0xf/0x10(15/16)=color_attachment、8=mip_levels(5)、0x11/0x12=2D array+depth。
- 预设表（O，`switch` 各 case：尺寸×格式 + 嵌入 hash）：
  0:64×64/0x53/0xe3624a5fa0363c58；1-5,7,9,0xb,0xd-0x16:512×512（格式 0x4c/0x61/0x46/0x7c/0x63/0x53/9 各不同）；
  6,0xe,0xf,0x13,0x14:512×512/0x61；8:512×512/0x7a+mip(5)/0x09fa1678d41f0c18；0xc:256×256/0x4c；
  0x10:512×512/9+color_attachment；0x11:new_2d_array(3,0x7e)/0x98c11fe95d7d3b89；0x12:new_2d_array(1,3,0x7e)/0xe6dee57299866376；
  0x17:225×225/0x4c；0x18:256×256/0x25；0x19..0x1f:复杂/动态路径（反编译坏指令截断，U）。
- **建议**：`create_preset_textures`（`texture_library.rs:351`）现用 `desc_of` 闭包参数绕过；落码时应替换为真实
  `impl From<TexturePreset> for TextureDesc`（可用 preset 表 + `NameHash`）。descript 布局：`{+0..+0x20 描述,
  +0x20 hash, +0x28 byte0, +0x29 u32, +0x2c u32}`（0x30 B，与 B_s_h 的 32 B rhapsody raw TextureDesc 不同）。

### 4.2 `TypedStorageBuffer` 的 `BUFFER_LEN>0` 形态

- `L0_NOTES.md` 已 O：`impl$0::new_storage_buffer<T>`（0x14160f920 起 38 单态化）在 `BUFFER_LEN>0` 时为固定大小
  `BUFFER_LEN×size_of::<T>()`、usage 0x22、memory 1、align 4；有数据先断言 `len%es==0`+4 对齐，`len>=size+es` → panic
  "Excessive data size supplied for buffer {type}"；无数据（r8==0）**不上传**。
- reconstruction `pass_kit::fixed_len_typed_storage`（pass_kit.rs:371）已按该形态落地，差异已注明：`BUFFER_LEN>1` 无数据时
  `TypedStorageBuffer::new` 传全零而原版不上传（U：设备未初始化内容）。
- **结论**：缺口已由 `fixed_len_typed_storage` 接缝覆盖；若要并入 `TypedStorageBuffer::new_storage_buffer`（trait 入口），
  需补 `BUFFER_LEN>0` 分支（固定大小 + None 不上传）。属可选整合，非功能缺失。

---

## 5. 错位/纠错汇总

1. `0x140555a90` 是 `#[derive(Component)]` 胶水（`Handle<Vao>`），不是"需补的 Resource/组件注册钩子"（RENDER_GAP §2.1 误判）。
2. `reset_indirect_args` 0x1e0 B 暂存区**有写入**（memcpy 自 0x142aaaa78 的 24 条 DrawIndexedIndirectCommand），
   B_c3 判"无写入"错误；reconstruction 测试的 0xAB 断言需改。
3. `shader_refactoring` disc 5 是 `Result::unwrap` panic（非 `unreachable!`）；disc 6..9 为合并进共享收尾的退化分支，不可达。
4. `0x140a59890` 是 `From<TexturePreset> for TextureDesc`，reconstruction 的 `From<u8> for TexturePreset` 归因错位；
   `usage()` 对预设 11/12 判 depth 有误。
5. `nani::gpu_world::impl$44::fmt` 就是 Debug 胶水（写 "InstanceKey"），不是"PDB 归属不符"。
6. `render_config::impl$7::from` 0x140968c40 是 `From<&GraphicsDevice> for LastKnownGraphicsDevice`（{uuid,driver} String×2），未实现。
7. `ShaderRefactoringState::disabled`（0x140b1f210，audio-misc 已纠）实为 `fn disabled(&self)->bool`，reconstruction 的构造器模型错误
   （cross-ref：resources/shader_refactoring_state.rs，audio-misc §2.12）。
8. `atmosphere` serialize 的字段揭示其为 `DerivedAtmosphere`（`{sun_tint_color, moon_tint_color, sky_low_color, sky_high_color}` 4×Vec3），
   与 `SunSkyColors`（3×Vec4，lerp 0x140a97650 已证）不同型。

---

## 6. 实现建议（优先级）

1. **落码依据就绪**：grading_lut 7 处（§2.1 的 audio-misc 反编译 + wp19 C 文件）。
2. **`reset_indirect_args.rs`**：0x1e0 暂存区改 `index_init()`（24 条 index_count 表），删"不写"断言。
3. **`texture_presets.rs`**：把 0xa59890 归正为 `From<TexturePreset> for TextureDesc`（预表 + NameHash@+0x20），
   修 `usage()` 11/12；`create_preset_textures` 去掉 `desc_of` 参数。
4. **`render_config`**：补 `impl From<&GraphicsDevice> for LastKnownGraphicsDevice { uuid, driver }`（0x140968c40）。
5. **`draw_callbacks_a`**：`MarkerU66476a13`→`BackgroundTerrain`、`FloatU192600af`→`OverwriteRoughness`、
   `MarkerUa6c0fc01`→`DontReceiveContactShadows`（TypeId 已反查，证据见 §3.6）。
6. **`render_world.rs:236/238`**：`*0x14362c0d8`/`*0x14362c098` 注释更新为 `freeze_occlusion_culling`/`demo`（O）。
7. **`dof.rs`**：给 `FullResDof`/`FullResAllLod1Dof` 补 `source_hash` 地址标注（0x140a5de00/0x140a5d960）。
8. **`shader_refactoring.rs`**：注释把 `6..=9：U` 更新为"不可达（更新只产出 disc 0/2/3/4；disc5 若进入 → Result::unwrap panic）"。
9. **`render_ssbo.rs`**：DofAutofocusSsbo/DofFocusReadbackSsbo/ShadowMaskHistogramSsbo/ShadowMaskMeanSsbo 可补真实元素类型
   （§2.2 表），NAME/BUFFER_LEN 机器码核对留待各 SSBO owner。
10. `TypedStorageBuffer::new_storage_buffer` 的 `BUFFER_LEN>0` 分支为可选整合（`fixed_len_typed_storage` 已覆盖）。

---

## 7. 反编译清单（render-gap/decomp/）

0x140555a90（17889B）、0x140039c10、0x1404d7d50、0x140322a10、0x140968c40、0x1409bdc60、0x1412e9e10、
0x1412ea160、0x1402b15c0、0x140a241d0、0x140a5de00、0x140a5d960、0x14090a870、0x140a59890、0x140a29b10、
0x14130d8e0、0x1415f9210。0x14003ae40（visit_str）Ghidra NotFound，已用 dis.py 反汇编核实。
