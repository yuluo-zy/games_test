# systems/backend/vao_update.rs

> 文件作用：**VAO 构建/重建系统。`build_missing_vaos` 遍历无 VAO 的网格建立 VAO（空网格跳过并日志）；`rebuild_vaos` 遍历全部网格按绑定变化重建 VAO（无 VAO 网格警告）。**

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::vao_update::build_missing_vaos` | 0x1408646b0 | 9 | Bevy 系统（ResMut +0x1c tick）。遍历网格位图索引（0xa0B 组件结构 @ mesh 库 +0x100），`FUN_140969550` 构建 + `FUN_140924180` 建 VAO；失败日志 `'Couldn't build VAO: '`（0x142a96490）；空网格日志 `'Mesh is empty. Skip building VAO'`（0x142a96508）。 | O | decomp/decomp-0x1408646b0.txt；rne-read --unks |
| `country_core::systems::backend::vao_update::rebuild_vaos` | 0x140864bf0 | 39 | Bevy 系统。遍历全部网格，`FUN_1409243f0` 重建 VAO（`FUN_1405b0d70` 辅助）；无 VAO 网格警告 `'WARNING: Tried rebuilding a mesh that has no VAOs!'`（0x142a96568）；失败日志 `'Couldn't rebuild VAO: '`（0x142a965c0）。 | O | decomp/decomp-0x140864bf0.txt；rne-read --unks |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（build_missing_vaos 行 20 闭包）

## 备注

- 与 asset_libraries/vao_library.rs（`AssetVAOLibrary::add`/`rebuild_vao`）协作，VAO 句柄组件插入命令（重构 `InsertVaoCommand`）方向一致。
