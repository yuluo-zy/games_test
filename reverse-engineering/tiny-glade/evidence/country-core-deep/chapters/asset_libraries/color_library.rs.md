# asset_libraries/color_library.rs

> 文件作用：**颜色库 SSBO**——`ColorLibrarySsbo`（37×Vec4 颜色槽，0x250 B）+ JSON 反序列化（槽名→下标）+ bevy 系统
> `update_color_library`（读 `assets/colors.json` + 主题色文件 → 上传 GPU 缓冲）。证据：`evidence/country-core-audit/A3_color.md` §一（O 全量）。

函数总计 12：业务 5 / 内联无独立符号 0 / 派生胶水 7

## 布局表（O，A3_color）

| 结构 | 布局 | 证据 |
|---|---|---|
| `ColorLibrarySsbo` | `[[f32;4]; 37]` 0x250 B（元素 4 字节对齐，非 SIMD Vec4）；默认 = 37 槽品红 `(1.0, 0.0, 1.0, 1.0)` | impl$1::default_value 0x14093b430 / impl$3::default 0x14093f900 |
| `ColorFromJson` | 0x28 B：`{ name: String @0, color: [f32;3] @0x18 }`，serde 字段表 `name`/`color`（color = ArrayVisitor<[f32;3]>，其余 IgnoredAny） | deserialize_from 0x14093e510 |
| 槽名表（37 项） | temp_color_1/2/3、roof_color_1/2、sky_low_day、sky_high_day、sun_tint_day、sky_low_dusk、sky_high_dusk、sun_tint_dusk、indirect_light_tint、fence_color_1/2、flower_color_1/2、canopy_color_1/2、grass_1/2、path、sky_base、sky_horizon、sky_clouds_1/2、terrain_1/2、terrain_flattened、wall_1/2、wall_moss、wall_bricks、ivy_branch、ivy_leaf_1/2、terrain_highlight、night_tint | deserialize_from 反编译（比较链 + lea 槽表） |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `ColorLibrarySsbo::deserialize_from` | 0x14093e510 | 37 | 猜测：serde 反序列化；证实：**按值 `Vec<ColorFromJson>` 的槽写入器**（非 serde trait） | **O**（5102 B）：按名字长度 `len-4≤0xf` 走跳表逐项字节比较（4B `path`、8B `sky_base`、17B/19B 用 `pcmpeqb`、6 个地形/墙色用字符串池 bcmp @0x142ab2182）；命中写 `slot = (c[0], c[1], c[2], 1.0)`；未知名 → `panic!("Init Color Library failed: {} entry is missing from ColorLibrarySSBO")`（pieces @0x142ab2208，行 66）；边遍历边写无事务 | A3_color.md §一 |
| `ColorLibrarySsbo::deserialize_from_path<&PathBuf>` | 0x14155a170 | 113 | 猜测：读文件反序列化；证实：`fs::read_to_string` + serde_json + deserialize_from | **O**：`read_to_string` Err → `Result::unwrap` panic（行 114 列 50）；`serde_json::from_str::<Vec<ColorFromJson>>` Err → `unwrap` panic（行 115 列 70） | A3_color.md §一 |
| `ColorLibrarySsbo::deserialize_from_path<&str>` | 0x14155a320 | 113 | 同上 &str 泛型实例 | **O**：同 `<&PathBuf>` 实例 | A3_color.md §一 |
| `impl$1::default_value` | 0x14093b430 | 129 | 猜测：默认值；证实：`Option<ColorLibrarySsbo>::Some`（tag=1，数据 +4 起共 0x254 B），37 槽品红 | **O**（252 B）：先写 `u32=1` 判别、数据从 +4 起 → `StorageElement::default_value` 返回 Some | A3_color.md §一 |
| `update_color_library` | 0x14093b530 | 140 | 猜测：更新系统；证实：**bevy 系统** `(设置, &AssetSsboLibrary, &Arc<GraphicsDevice>, &mut ColorLibrarySsbo)` | **O**（1502 B）：栈上 37 槽品红 → `deserialize_from_path<&str>("assets/colors.json")`（**无存在性检查**，缺文件 panic）→ 主题目录表 `summer/olden/flowery/autumn/winter`（@0x142ab22b8，u8 下标无边界检查）`Path::join("assets/glade").join("colors.json")`，fs::stat 成功才再加载 → `FrozenMap::get(&SsboId)` 键立即数 `0xef2a68c12cde3f0d` = **xxh3_64("ColorLibrary")**（用原 EXE XXH3 secret 复算，O）→ 缺失 `Option::unwrap`（行 156）→ `assert_eq!(state.name, type_name::<ColorLibrarySsbo>())`（名长 0x3e）→ vtable `+0x18`（`as_any` 返回的具体 `TypedStorageBuffer` 指针）→ `byte_size(+0x18) >= 0x250` 守卫 → `update_buffer_bytes(buffer, data, [0..0x250], 0)` → Err → `Result::unwrap` panic（行 158）；Ok → `memcpy` 到 CPU 资源 | A3_color.md §一；decomp: backend-misc/decomp/decomp-0x14093b530.txt |

## 派生胶水（7 项：impl$3::default 0x14093f900、ColorFromJson 的 deserialize/visit_str/visit_map/visit_seq/expecting、impl$0::deserialize）

- `impl$3::default` = 无 tag 的 0x250 B 品红填充 = derive(Default) 展开（O，245 B）。
- serde 访客族 = `ColorFromJson` 的 Deserialize 派生（A3_color 已证字段表），不补业务。

## 纠错/差异（A3_color §一，O）

- 旧版错：槽偏移（0x50/0x48/0x4c…4 字节粒度）全臆造，只有 14 个名字且位置全错；格式写成 RON；`ColorLibraryRon` 固定字段结构不存在。
- 旧版 `deserialize_from_path` "读失败返回 SpannedError" 错：实际两处 `unwrap` panic。
- 旧版 `update_color_library`：`path.exists()` 吞错、主题层缺失、用 `get_mut`+rg_handle 上传（原版只读 `get` + `buffer()`）、失败保留 CPU 副本（原版 panic）——全部不符。
- **vtable 槽序冲突**（A3_color 注）：A3 把 `Ssbo` trait `+0x18` 记为 `as_any`（返回值直接当 Buffer* 用）；L0_NOTES 确认 `+0x18` 就是 `as_any`（返回 `(自身指针, Any 虚表)`），调用点取返回对象的 `+0x0` 即内嵌 Buffer —— 二者一致，非冲突。
- `StorageElement::NAME == "ColorLibrary"`（O，xxh3 复算；"ColorLibrarySsbo" 全名哈希 0x58d69be238f7e8b7 不符）。
