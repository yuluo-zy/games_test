# systems/backend/input/calculate_input_delta.rs

> 文件作用：输入增量计算：对每个 analog action 按活动/死区状态（kind 0/1/2 = released/moving/deadzone）累加 cursor delta（死区 0.1/0.05），输出 {tag,value} 对 + 10 qword 原始镜像（wp20 合同复核）。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::calculate_input_delta::calculate_input_delta` | 0x1416feeb0 | 14 | 系统:输入处理 | O：wp20 既有反编译（analysis/0x1416feeb0 同名）+ 合同复核：kind2 走 +0.1/0.05 死区、kind1 累加 value+prev_delta、kind0 保留 prev；输出区 @+0xa0 起 10 qword 镜像 | wp20/analysis/0x1416feeb0_calculate_input_delta.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::input::calculate_input_delta::digital_delta`（行 143）— 待推断
