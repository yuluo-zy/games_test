# systems/backend/game_frame_index.rs

> 文件作用：**帧序号资源。`game_frame_index` 为全内联函数（dependencies.json inline_instances [560, 40168]/[1611, 2310144]），无独立物理符号；其资源 `GameFrameIndex` 由 `prepare_frame_constants` 消费（写 `frame_constants+0x108 = idx & 3`）。**

函数总计 1：业务 0 / 内联无独立符号 1 / 派生胶水 0

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::game_frame_index::game_frame_index`（行 7）— **I**：全内联帧推进；无独立地址。在 prepare_frame_constants（0x140af40b0）中以 `Res<GameFrameIndex>` 形式被读：`uVar44 = *puVar14`（首 u32），`if (1 < uVar44) { *(frame_constants+0x108) = puVar14[1] & 3; }`。**证据显示资源至少 2×u32**（首字段为门槛、次字段为帧号），重构的 `GameFrameIndex(pub u32)` 单字段可能不完整（布局 I）。

## 派生胶水（derive/宏生成；不计独立业务）

（无）

## 备注

- 帧号写 `frame_constants+0x108` 与 `& 3`（TAA jitter 循环）由 prepare_frame_constants 章节交叉证实。

## 实现方式（算法级）

**game_frame_index（全内联，I）**：
- 帧推进并入调用点，无独立物理符号（dependencies.json inline_instances [560,40168]/[1611,2310144]）。（I）

**消费侧算法（prepare_frame_constants @0x140af40b0，O）**：
1. `Res<GameFrameIndex>` 读作 2×u32：首 u32 为门槛，`if (1 < uVar44)` 才写帧号。（O）
2. `frame_constants+0x108 = 次 u32 & 3`（帧号 `idx & 3`，TAA jitter 周期）。（O）

**数据结构（I）**：`GameFrameIndex` 布局 ≥2×u32（门槛@+0x00 / 当前帧号@+0x04）；重构单字段 `GameFrameIndex(pub u32)` 可能不完整。（I）
