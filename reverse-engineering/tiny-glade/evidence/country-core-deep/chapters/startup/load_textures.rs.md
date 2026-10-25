# startup/load_textures.rs

> 文件作用：**纹理载入：过滤（should_load）并载入启动纹理（logo/brush/loading/menu 相关），构建 GladeTextureMappings（textures.json 语义键→纹理文件映射）。**

函数总计 19：业务 3 / 内联无独立符号 2 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::load_textures::impl$0::should_load` | 0x1409e6950 | 23 | `impl$0::should_load`：纹理过滤谓词 — 前缀匹配 {logo,brush,loading,menu/pounce_light_logo,menu/heart,menu/noknok_logo,menu/fmod_logo,blue_noise,default_loading_background{, _demo}}，末两项受全局标志 `bRam+0x3634d90` 门控。 | **O** | decomp-0x1409e6950.txt |
| `country_core::startup::load_textures::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140a2f670 | 149 | `visit_str<serde_json::Error>`：serde 枚举字符串反序列化胶水（长度 switch 按首字节匹配，映射到判别值）。 | **O** | decomp-0x140a2f670.txt |
| `country_core::startup::load_textures::load_glade_texture_mappings` | 0x1409e6b60 | 177 | `load_glade_texture_mappings`：读 `textures.json`，构建 GladeTextureMappings — 语义键 `glade:{ivy_leaf_color,canopy_flowers_depth,leaf_particle_colors,terrain_texture,grass_patch,lilypad_flower,canopy_color,bark_color,bark_color_naked}`。`Couldn't load` panic。 | **O** | decomp-0x1409e6b60.txt |

## 内联/无独立符号函数

- `country_core::startup::load_textures::load_textures`（行 53）— **I**：`load_textures` — 启动纹理总入口（内联）。
- `country_core::startup::load_textures::GladeTextureMappings::create_mappings`（行 155）— **I**：`GladeTextureMappings::create_mappings` — 建映射（内联）。

## 派生胶水

- 14 项：closure$0, closure$1, closure$2, deserialize, expecting, visit_map, visit_seq 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **should_load 0x1409e6950（O）**：前缀匹配谓词，串表 0x142ac4780 区（顺序）：
  logo / brush / loading / menu/pounce_light_logo / menu/heart / menu/noknok_logo / menu/fmod_logo
  / blue_noise / default_loading_background / default_loading_background_demo；
  **末两项受 demo flag 0x143634d90 门控**（与 session build/photomode 共用同一全局）。
- **load_glade_texture_mappings 0x1409e6b60（O）**：读 'textures.json' → 语义键
  glade:{ivy_leaf_color, canopy_flowers_depth, leaf_particle_colors, terrain_texture, grass_patch,
  lilypad_flower, canopy_color, bark_color, bark_color_naked} → 失败 panic `Couldn't load `（0x142ac47xx 区）。
- load_textures（内联，行 53）= should_load 过滤后逐纹理载入 + create_mappings（内联）。
