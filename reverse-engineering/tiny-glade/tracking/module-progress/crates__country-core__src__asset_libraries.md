# crates/country-core/src/asset_libraries

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

实际decorator collision JSON读取与mesh/shader library生产者边界已有局部恢复/分析。

## 接线与验证

接线：decorator资产子集已接World。

18键资产放置等限定World链；ShaderLibrary只读分析非完整Rust恢复。

## 已恢复的函数和类型范围

- load_decorators/实际collision bounds
- AssetShaderLibrary四个add/get生产者及raster创建分析

## 明确缺口

- 完整asset/pass关联、缓存淘汰和资源所有权
- 新country-core库接线交付缺失

下一闭包：完整asset/pass关联、缓存淘汰和资源所有权

## 证据

- [evidence/building-integrate/findings.md](../../evidence/building-integrate/findings.md)
- [evidence/render-recovery/consumer-chain-review-2026-10-12/README.md](../../evidence/render-recovery/consumer-chain-review-2026-10-12/README.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/asset_libraries/audio_trigger_library.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/color_library.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/configwatch.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/mesh_library.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/meshwatch.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/placement_data_library.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/shader_library.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/ssbo_library.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/texture_library.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/texture_watch.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/asset_libraries/vao_library.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/decorator_placement.rs

[reconstruction/building/decorator_placement.rs](../../reconstruction/building/decorator_placement.rs)

SHA256：`9f1b56e53e73f8cf26ec2ef51aa8cbf818dd5fb514874b314a95d9abef93e345`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `metrics` | 23 | 未由扫描证明；以证据限定范围为准 |
| `wall_decorator_key` | 57 | 未由扫描证明；以证据限定范围为准 |
| `b_wall_collision_asset_name` | 75 | 未由扫描证明；以证据限定范围为准 |
| `mesh_collision_bbx` | 108 | 未由扫描证明；以证据限定范围为准 |
| `validate_curve` | 141 | 未由扫描证明；以证据限定范围为准 |
| `wall_coord_from_world` | 160 | 未由扫描证明；以证据限定范围为准 |
| `wall_coord_to_world` | 205 | 未由扫描证明；以证据限定范围为准 |
| `place_wall_decorator` | 245 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/decorator_assets.rs

[reconstruction/crates/tg-building-runtime/src/decorator_assets.rs](../../reconstruction/crates/tg-building-runtime/src/decorator_assets.rs)

SHA256：`4c6671718f920e8b977ac7d61c1f33631f7030a08c3b866935867921e40464a3`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `decode_collision_mesh` | 66 | 未由扫描证明；以证据限定范围为准 |
| `load_b_wall_opening_assets` | 122 | 未由扫描证明；以证据限定范围为准 |
| `derive_and_place_wall` | 144 | 未由扫描证明；以证据限定范围为准 |
| `load_b_plain_wall_assets` | 209 | 未由扫描证明；以证据限定范围为准 |
| `load_bindings` | 217 | 未由扫描证明；以证据限定范围为准 |
| `place` | 237 | 未由扫描证明；以证据限定范围为准 |
| `b_plain_wall_bindings` | 252 | 未由扫描证明；以证据限定范围为准 |
