# systems/backend/input/calculate_input_delta.rs

> 文件作用：输入增量计算：对每个 analog action 按活动/死区状态（kind 0/1/2 = released/moving/deadzone）累加 cursor delta（死区 0.1/0.05），输出 {tag,value} 对 + 10 qword 原始镜像（wp20 合同复核）。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::calculate_input_delta::calculate_input_delta` | 0x1416feeb0 | 14 | 系统:输入处理 | O：wp20 既有反编译（analysis/0x1416feeb0 同名）+ 合同复核：kind2 走 +0.1/0.05 死区、kind1 累加 value+prev_delta、kind0 保留 prev；输出区 @+0xa0 起 10 qword 镜像 | wp20/analysis/0x1416feeb0_calculate_input_delta.txt |

## 实现方式（算法级）（O，wp20/analysis/0x1416feeb0 复核）

**calculate_input_delta @0x1416feeb0（analog delta 累加器）**：
- 入参 = 每个 analog 动作的 {prev, value, kind}；kind = 活动状态枚举（0=released/1=moving/
  2=deadzone）。
- 逐动作算法：
  - **kind2（deadzone）**：`value + 0.1`（死区外推）；`≤ 0.05` 夹取 0.05（最小阈值）；输出
    `delta = value - prev`。
  - **kind1（moving）**：输出 `value + prev`（累加路径）。
  - **kind0（released）**：输出 prev 值（delta=0，状态=3）。
- 三通道独立 @+0x30/0x31/0x32（游标 x/y/z 或分量）；拷贝镜像区 @+0x58..+0x80（6 qword）。
- 输出区 @+0xa0 起 10 qword 原始镜像（供下游 InputTranslationState）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::input::calculate_input_delta::digital_delta`（行 143）— 待推断
