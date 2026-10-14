# systems/backend/prepare_frame_constants.rs

> 文件作用：**帧常量 CPU 组装。每帧把帧序号写 `frame_constants+0x108 = idx & 3`，按 `USE_TAA_JITTER` 全局（cRam000000014362c0a0）用帧号采样 jitter 表（0x142ae4fc0 = {0,3,1,2,4,...}，`%table_len`），并组装 4 份 `ViewConstants`（`FUN_140b18f60` = ViewportConstantBuilder::build）与 800B 帧常量块。**

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::prepare_frame_constants::prepare_frame_constants` | 0x140af40b0 | 29 | Bevy 系统（多 Res/ResMut；param_2 = ResMut 指向 FrameConstants 缓冲）。`*(uint *)(*param_2 + 0x108) = frame & 3`（门槛 `frame>1`，frame 取 `Res<GameFrameIndex>` 次 u32）；`cRam000000014362c0a0` 全局门控 TAA jitter：索引 = `frame & 3 % table_len`（table_len=*[lVar38+0x10]），jitter 值查表 0x142ae4fc0；0x100B/800B 块多次 memcpy（`func_0x000142923c00`），`FUN_140b18f60` ×4 构建 ViewConstants，`FUN_1405b1be0` 矩阵，`FUN_140d7bd90` 读相机参数；`FUN_140b18af0` 后处理。 | O | decomp/decomp-0x140af40b0.txt；rne-read --hex 0x142ae4fc0（jitter 表） |
| （助手）`FUN_140b18f60` | 0x140b18f60 | — | ViewConstants 构建（`ViewportConstantBuilder::build` 类，23057B 大函数，含矩阵变换）。 | I | decomp/decomp-0x140b18f60.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项（prepare_frame_constants 行 111/165 闭包）

## 备注

- 重构对 `frame_constants+0x108 = idx & 3` 与 TAA jitter `%table_len` 的描述**完全证实**。
- `GameFrameIndex` 资源被读为 `{u32 首, u32 次}`，重构单字段版本可能不完整（I，见 game_frame_index 章节）。
