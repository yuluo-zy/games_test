# systems/backend/grow_grass.rs

> 文件作用：**草地生长"仪式"系统。启动后计时 300 秒（`_UNK_142aa5140`=300.0f32），到时校验一组 15B 数据（XOR 解码静态）的 64B 哈希一致后通过 EventWriter 发送 8B 事件并置 grown 标志；涉及等待后台线程句柄（CloseHandle）。属一次性视觉过渡，非植被模拟。**

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 布局表（Timer 资源）

| 偏移 | 大小 | 含义 | 证据 |
|---|---|---|---|
| +0x00 | 4B | elapsed f32（累加 dt） | `fVar67 = *param_2`；`*param_2 = fVar67` |
| +0x04 | 1B | grown 标志（1=已生长） | `*(char *)(param_2+1)` 检查与写入；`*(undefined1 *)(param_2 + 1) = 1` |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::grow_grass::platform::grow_grass` | 0x14172fe80 | 30 | Bevy 系统：`ResMut<Timer{f32,u8}>` + `Res<Time>`（dt@+0x58）+ `ResMut<Events<E>>`。grown 标志已置则 return；`elapsed += dt`；`elapsed <= 300.0` return；否则解码静态（`func_0x000141563a40`+XOR）、计算哈希（`FUN_1420cc020`，32B→64B）、与存储哈希 memcmp 0x40B；匹配则 EventWriter 写入 8B 事件（Vec cap-first @+0x20, len@+0x30, count@+0x40）并置 grown。`CloseHandle` 指示等待后台生成线程完成。 | O | decomp/decomp-0x14172fe80.txt；rne-read --hex 0x142aa5140（300.0f32） |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::grow_grass::platform::is_grown`（行 48）— **O**：谓词 = `*(timer+4) != 0`（grown 标志），并入 grow_grass 入口检查。

## 派生胶水（derive/宏生成；不计独立业务）

（无）

## 备注

- 重构版 `GrassGrowthTimer(pub f64)` + `is_grown = self.0 > 0.0` 与证据不符（证据为 `{f32 elapsed, u8 grown}`），见 DIGEST 差异节。
- 哈希校验用途（防篡改/一次性门）为 I，未进一步定位。
