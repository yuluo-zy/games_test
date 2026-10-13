# A3 color 组：color_library / placement_data_library / audio_trigger_library

标记：O = 机器码直接观察，I = 推断，U = 未知。工具：`dis.py` / `rdva.py` / 全 .text objdump 找调用点。
测试台：`/tmp/h_color`（l0-harness 副本，见最终报告），本组 38 个单测，全 crate 副本合计 138 个通过。

## 一、color_library.rs（6 个过程）

### `impl$1::default_value` 0x14093b430（252 B）和 `impl$3::default` 0x14093f900（245 B）
- O：两者都把 `xmmword [0x142a9be30] = (1.0, 0.0, 1.0, 1.0)`（品红）写满 37 个 16 B 槽；前者先在 +0 写 `u32 = 1`、数据从 +4 起（共 0x254 B）
  = `Option<ColorLibrarySsbo>::Some`（说明元素类型对齐为 4，即 `[[f32;4];37]` 而非 SIMD `Vec4`）；后者无 tag（0x250 B）= `Default`。
- 旧版错：`Default` 为 `Vec4::ONE`（全白），槽数“37 为 I”实为 O。
- I：`StorageElement::BUFFER_LEN == 1`（有 `default_value` 即单结构体 uniform）。

### `ColorLibrarySsbo::deserialize_from` 0x14093e510（5102 B）
- O：入参是 `Vec<ColorFromJson>` 按值（cap/ptr/len，元素 0x28 B）；循环里 `r15 = cap`（用于释放 name 的 String）、
  `[entry+8]=name ptr`、`[entry+0x10]=name len`、color 三个 f32 @ entry+0x18/0x1c/0x20。
- O：按名字长度 `len-4 ≤ 0xf` 走 `jmp [table]`，其它长度直接落到 panic；每个长度分支内逐项比较（4 字节 `path`；
  8 字节 `sky_base` 用 `cmp qword`；19 字节 `indirect_light_tint` 与 17 字节 `terrain_flattened` 用 SSE `pcmpeqb`；
  `terrain_1/2`、`wall_moss`、`wall_bricks`、`terrain_highlight`、`night_tint` 用 `bcmp(rdx=0x142ab2182…)`，
  字符串池 `terrain_1terrain_2wall_mosswall_bricksterrain_highlightnight_tint` 在 0x142ab2182 起；其余为 `movabs`+xor）。
- O：命中后统一在 0x14093f710 写 `slot = (c[0], c[1], c[2], 1.0)`（`mov dword [r9], 0x3f800000`）。
- O 槽表（临时脚本解码比较链 + 栈上 `lea rax,[rdi+N]` 指针表，槽序号 = N/16）：

| 序号 | 名字 | 序号 | 名字 | 序号 | 名字 |
|---|---|---|---|---|---|
| 0 | temp_color_1 | 13 | fence_color_2 | 26 | terrain_2 |
| 1 | temp_color_2 | 14 | flower_color_1 | 27 | terrain_flattened |
| 2 | temp_color_3 | 15 | flower_color_2 | 28 | wall_1 |
| 3 | roof_color_1 | 16 | canopy_color_1 | 29 | wall_2 |
| 4 | roof_color_2 | 17 | canopy_color_2 | 30 | wall_moss |
| 5 | sky_low_day | 18 | grass_1 | 31 | wall_bricks |
| 6 | sky_high_day | 19 | grass_2 | 32 | ivy_branch |
| 7 | sun_tint_day | 20 | path | 33 | ivy_leaf_1 |
| 8 | sky_low_dusk | 21 | sky_base | 34 | ivy_leaf_2 |
| 9 | sky_high_dusk | 22 | sky_horizon | 35 | terrain_highlight |
| 10 | sun_tint_dusk | 23 | sky_clouds_1 | 36 | night_tint |
| 11 | indirect_light_tint | 24 | sky_clouds_2 | | |
| 12 | fence_color_1 | 25 | terrain_1 | | |

- O panic：`panic_fmt(pieces @0x142ab2208 = ["Init Color Library failed: ", " entry is missing from ColorLibrarySSBO"], arg = String Display)`，
  Location 行 66。panic 前已处理的条目已写入（边遍历边写，无事务）。
- 旧版错：槽偏移（0x50/0x48/0x4c/…，且在 4 字节粒度上）全是臆造，只有 14 个名字且位置全错；格式写成 RON；
  `ColorLibraryRon` 固定字段结构完全不存在；`canopy_color_1 → colors[0]` 等“typed gap”也全错。

### `deserialize_from_path` 0x14155a170（`&PathBuf`）/ 0x14155a320（`&str`）
- O：`fs::read_to_string(path)`，Err → `Result::unwrap` panic（Location 行 114 列 50）；
  `serde_json::from_str::<Vec<ColorFromJson>>`，Err → `unwrap` panic（行 115 列 70）；然后 `deserialize_from`。
- O：`ColorFromJson` 的 serde 字段表（0x14143… `deserialize_struct` 里 `missing_field("name")` / `("color")`；
  `color` = `ArrayVisitor<[f32;3]>`；其余字段 `IgnoredAny` 忽略）。库 serde_json 1.0.85。
- 旧版错：函数“读失败 → 返回 SpannedError”，实际两处都是 `unwrap` panic。

### `update_color_library` 0x14093b530（1502 B）
参数（O，Win64）：rcx = 设置（`+0x58` 的 u8 = 主题下标）、rdx = `&AssetSsboLibrary`（`FrozenMap`）、r8 = `&Arc<GraphicsDevice>`（`[r8]+0x10`）、
r9 = `&mut ColorLibrarySsbo`（CPU 资源，最后被 `memcpy(r9, lib, 0x250)`）。
1. 栈上 37 槽品红；`deserialize_from_path<&str>("assets/colors.json", 0x12)`（无存在性检查 → 缺文件 panic）。
2. 主题目录表（@0x142ab22b8 ptr / 0x142ab22e0 len；u8 直接作下标，无范围检查）= `summer, olden, flowery, autumn, winter`；
   `Path::join("assets/glade"(0xc), theme).join("colors.json"(0xb))`；`fs::stat` 成功才再 `deserialize_from_path<&PathBuf>`。
3. `FrozenMap::get(&SsboId)`：键立即数 `0xef2a68c12cde3f0d`。**用原 EXE 的 XXH3 secret（0x142b2ebb0）复算 xxh3_64("ColorLibrary") = 0xef2a68c12cde3f0d**
   （9..16 字节分支，用 python 手算；对照 "ColorLibrarySsbo" = 0x58d69be238f7e8b7 不符），故 `StorageElement::NAME == "ColorLibrary"`（O）。
4. 缺失 → `Option::unwrap`（行 156 列 10）；`assert_eq!(state.name, type_name::<ColorLibrarySsbo>())`，名字长 0x3e（ssbo_library.rs 行 210）；
   vtable `+0x18` 方法返回 `Buffer*`（空 → 同一个 unwrap 位置，行 156）。
5. `memcpy` 0x250 到临时；`buffer.byte_size (+0x18) < 0x250` → 错误；否则 `update_buffer_bytes(buffer, data, [0..0x250], 0)`；
   Err → `Result::unwrap` panic（行 158 列 10）；Ok（标签 0xb）→ `memcpy(r9, lib, 0x250)`。
- 旧版错：忽略主题层与 `assets/colors.json` 的必须存在语义；以 `path.exists()` 吞错；`get_mut` + `rg_handle` 上传（原版用只读 `get` + `buffer()`）；
  上传失败只是“保留 CPU 副本”，原版是 panic，且写回 CPU 副本只发生在成功之后。
- I：`ColorLibrarySsbo: Resource`；缓冲为 `TypedStorageBuffer<ColorLibrarySsbo>`；主题类型名/变体名；vtable `+0x18` 在重建 `Ssbo` trait 里对应 `buffer()`
  （ssbo_library 文档把 `+0x18` 记为 `as_any`，而这里 `+0x18` 的返回值直接当 `Buffer*` 用，二者冲突，待 ssbo_library 作者复核槽序）。
- 连带改动：`ssbo_library.rs` 新增 `AssetSsboLibrary::get_buffer<T>()`（只读入口，`buffers` 为私有字段）。

## 二、placement_data_library.rs（3 个过程 + drop）

### `AssetPlacementDataLibrary::new` 0x140867700（328 B）
- O：0x60 B。表 A @0（空表 4×u64：ctrl=静态空组、mask/growth/items=0）+ RandomState @0x20；表 B @0x30 + RandomState @0x50；
  `RandomState::new()` 读线程局部 KEYS（k0/k1），每次 `k0 += 1` 写回，第二张表的 k0 比第一张大 1。
- 旧版错：单表 `HashMap<PathBuf, PlacementData>`。

### `AssetPlacementDataLibrary::load<PathBuf>` 0x1409365d0（1674 B）
- O：A = `HashMap<Handle<PlacementDataSsbo>, (TypedStorageBuffer<PlacementDataSsbo>, usize)>`（条目 0x38 B，来自
  `drop_inner_table` 的类型名）、B = `HashMap<PathBuf, Handle>`（条目 0x28 B；PathBuf = 0x20 B）。
- O 顺序：B 非空（`[self+0x48] != 0`）才 `hash_one<RandomState, &PathBuf>`+探测，命中 → `panic_fmt(["AssetPlacementDataLibrary: the placement data from ", " has already been loaded"], path.display())`（行 93 列 13）；
  `read_to_string` 失败 → `panic_fmt(["Couldn't find ", …], path.display())`（行 100 列 63，io 错误丢弃）；
  `serde_json::from_str::<Vec<PlacementDataSsbo>>` Err → `unwrap`（行 101 列 73）；`TypedStorageBuffer::new(device, Some(&data))`；
  `lock xadd [0x143634cd0], 1` 取 handle；`A.insert(handle, (buf, len))`（返回的旧值若非 None 则 drop 句柄）；`B.insert(path, handle)`；
  返回 `(Vec, handle)`（0x20 B），读到的文本释放。
- 旧版错：不读文件、返回 `Result<(),()>`、`add`/`clear` 不存在。
- U：`PlacementDataSsbo` 的 JSON schema（`deserialize_struct` 里有 u32/f32/i32/[f32;3]/[f32;2] 和 `utils::pack_2x16f_uint`，与 `resources/placement_data.rs`
  现有字段不符，属另一工作包）；`Handle::new` 的计数器是否就是 `HANDLE_COUNT`。
- `register_required_components` 0x14086c3d0 = bevy derive 产物，不重写。

## 三、audio_trigger_library.rs（3 个过程）

### `AudioTriggerCollection::boxes` 0x14157ce40（67 B）
- O：`self` = `SlotMap`（`slots Vec` @0，`num_elems u32` @0x1c）。返回 4 个 qword `{ptr + (len?0x1c:0), ptr+len*0x1c, (len != 0), num_elems}`
  = `slotmap::basic::Iter`（`slots.iter().enumerate()` 跳过哨兵槽 0，起始下标 1，`num_left = num_elems`）。唯一调用方
  （0x1422798de，PDB 名被 ICF 折叠成 `handplaced_tree_animation`，实为 `trigger_audio_system`）按 `[slot+0x18] & 1` 判占用、`DecoratorRank::new(idx)`（= `mov eax, ecx`，
  `impl$3::from` 同址，`AudioTriggerHandle` ↔ u32）取键、前 0x18 B 当 `Aabb3` 喂 `Aabb3::intersect_segment`。
- 旧版错：28 B 盒“`min/max/flags`”——其实 28 = slotmap 槽（24 B Aabb3 + 4 B version），flags 是 version；`boxes()` 返回 values 迭代。

### `AudioTriggerLibrary::get_triggers` 0x14157ce90（42 B）
- O：`rdx == 0` 返回 `rax = rcx`；`rdx != 0` → `panic("assertion failed: kind != AudioTriggerKind::COUNT")`（行 85 列 9）。
  调用点传 `edx = 0`。即 `COUNT == 1`，库 = `[Collection; 1]`；`insert_resource<AudioTriggerLibrary>` 的 `call_once` 搬 0x20 B。
- 旧版错：`AudioTriggerCollectionHandle` 与二级 slotmap 纯属臆造；`get_triggers` 返回 `Option`。
- I：唯一合法变体名；`add/update/remove/clear` 的具体语义（内联于 prefab 系统，无符号）。

### `impl$3::from` 0x140833250（3 B）
- O：`mov eax, ecx; ret`，u32 恒等；与 `DecoratorRank::new` ICF 合并，无需 Rust 函数。
