# systems/clear_canvas.rs

> 文件作用：**清空画布系统**——事件驱动（EventReader 修剪流）：日志 "Clearing the canvas." →
> InnerWalls::remove_all（0x14085a730）→ 世界光栅 clear_all<512>（0x14093bee0，变体 0x13/0x14/
> 6/5/7/1/2/3/4 + 9/0xb/0xd/0xe）→ 7×clear_preset_texture（0x140a15da0，3 个 color_id 0x5000=
> 20KB 缓冲）→ 上传记录/杂项清零（0x140aaae80×2、0x140aa5150、0x1408950d0、0x14088acf0）。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `clear_canvas` | 0x140a7abd0 | 33 | 见文件作用（16 参数系统） | O：实证 | camera-time\decomp\decomp-0x140a7abd0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `clear_terrain_gpu`（行 187）— 删除/清理：地形 GPU 缓冲重置（0x14000=80KB 分配 + color_id.rs panic 点，并入 clear_canvas 主体尾部）
