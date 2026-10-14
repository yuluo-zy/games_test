# render/render_config.rs

> 文件作用：**待推断**（由章节 agent 按函数名+证据填写）

函数总计 39：业务 6 / 内联无独立符号 1 / 派生胶水 32

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…impl$0::serialize<ron::Serializer<String>>` | 0x140039c10 | 7 | 序列化/serde | O（胶水）：ron 序列化派生（RENDER_GAP §2.1 已判 serde derive 无独立逻辑） | render-gap\DIGEST.md |
| `FullscreenConfig::resolution` | 0x140968ad0 | 25 | 查询/计算 | O：resolution 访问器（FullscreenConfig 分辨率字段；见 render_config 章节布局） | render-gap\DIGEST.md |
| `…impl$0::deserialize::visit_str<ron::Error>` | 0x14003ae40 | 48 | 序列化/serde | O（胶水）：ron 字符串枚举反序列化 visitor（RENDER_GAP §2.1 已判 serde derive；字符串枚举名可核对） | render-gap\DIGEST.md |
| `…impl$0::previous` | 0x140914840 | 220 | 查询/计算 | O：枚举迭代器 Sequence::previous（与 game_config 0x140914840 同址 ICF；枚举向前遍历胶水） | expansion-b2\decomp\decomp-0x140914840.txt |
| `UpscalingPreset::tgsr_resolution_scale` | 0x140968bb0 | 262 | 查询/计算 | O：TGSR 分辨率缩放访问器（UpscalingPreset 枚举方法） | render-gap\DIGEST.md |
| `impl$7::from` | 0x140968c40 | 335 | 构造/注册/生成 | O：`From<&GraphicsDevice> for LastKnownGraphicsDevice{uuid,driver}`（render-gap §2.1 已核实，RENDER_GAP 判需补项） | render-gap\decomp |

## 内联/无独立符号函数

- `country_core::render::render_config::VideoSettings::upgrade`（行 287）— 待推断

## 派生胶水（32 项：clone、default、deserialize、eq、fmt、next、partial_cmp、serialize、visit_enum、visit_map、visit_str 等）
