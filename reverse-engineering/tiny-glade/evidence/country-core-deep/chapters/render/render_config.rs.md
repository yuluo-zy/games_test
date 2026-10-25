# render/render_config.rs

> 文件作用：**渲染配置类型**——`FullscreenConfig`（分辨率字段）、`DisplayConfig`（monitor/fullscreen 等 14 档）、
> `VideoSettings`（含 `UpscalingPreset`）、`LastKnownGraphicsDevice{uuid,driver}`。serde 派生占大头（32 项）。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…impl$0::serialize<ron::Serializer<String>>` | 0x140039c10 | 7 | 序列化/serde | O（胶水）：ron 序列化派生（RENDER_GAP §2.1 已判 serde derive 无独立逻辑） | render-top/decomp/decomp-0x140039c10.txt（原 render-gap\DIGEST.md） |
| `FullscreenConfig::resolution` | 0x140968ad0 | 25 | 查询/计算 | O：resolution 访问器（FullscreenConfig 分辨率字段） | hex 手解 0x140968ad0（rne-read --hex 本批；原 hex 手解 0x140968ad0（rne-read --hex 本批；原 render-gap\DIGEST.md）） |
| `…impl$0::deserialize::visit_str<ron::Error>` | 0x14003ae40 | 48 | 序列化/serde | O（胶水，round3 round2 复查修正：措辞由“字符串枚举”改“字段名匹配”）：serde 派生 visit_str（DisplayConfig 字段名 monitor/fullscreen 等 14 档长度跳表匹配，非枚举变体） | hex 手解 0x14003ae40（rne-read --hex 本批；原 hex 手解 0x14003ae40（rne-read --hex 本批；原 render-gap\DIGEST.md；rne-read 0x14003ae40；S_PUB32 __FieldVisitor::visit_str）） |
| `…impl$0::previous` | 0x140914840 | 220 | 查询/计算 | O：枚举迭代器 Sequence::previous（与 game_config 0x140914840 同址 ICF；枚举向前遍历胶水） | expansion-b2\decomp\decomp-0x140914840.txt |
| `UpscalingPreset::tgsr_resolution_scale` | 0x140968bb0 | 262 | 查询/计算 | O：TGSR 分辨率缩放访问器（UpscalingPreset 枚举方法） | hex 手解 0x140968bb0（rne-read --hex 本批；原 hex 手解 0x140968bb0（rne-read --hex 本批；原 render-gap\DIGEST.md）） |
| `impl$7::from` | 0x140968c40 | 335 | 构造/注册/生成 | O：`From<&GraphicsDevice> for LastKnownGraphicsDevice{uuid,driver}`（render-gap §2.1 已核实，RENDER_GAP 判需补项） | render-top/decomp/decomp-0x140968c40.txt（原 render-gap\decomp） |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- **`FullscreenConfig::resolution`**（0x140968ad0）：访问器返回分辨率字段（VideoSettings→Fullscreen 分辨率选择，供 `render` 的 swapchain extent 与 `ViewConstants` 视口尺寸）。
- **`impl$7::from`（0x140968c40）**：`From<&GraphicsDevice>` → `LastKnownGraphicsDevice{uuid, driver}`（设备身份快照；RENDER_GAP §2.1 判"U 需补"，round3 已核实 O）。
- **`UpscalingPreset::tgsr_resolution_scale`**（0x140968bb0）：TGSR 缩放因子访问器 —— `calculate_tgsr`/`render_world` 的 TGSR 分支用（帧序 W52 选择）。
- **serde 面**：serialize/visit_str/previous 均为派生胶水（项目规则不计业务）；visit_str 是 DisplayConfig 字段名 14 档长度跳表匹配（非枚举变体）。
- `VideoSettings::upgrade`（行 287 内联，I）—— 设置升级路径。
- 消费链：`render`（render_frame）读 GameConfig/VideoSettings → swapchain 分辨率与降分辨率 OOM 路径（B_s_h H 层）；`ImageFormationConfigs` 的 kind 选择字节 +0xc0（final_post 用，d_gap §4）。

## 内联/无独立符号函数

- `country_core::render::render_config::VideoSettings::upgrade`（行 287）— 待推断

## 派生胶水（32 项：clone、default、deserialize、eq、fmt、next、partial_cmp、serialize、visit_enum、visit_map、visit_str 等）
