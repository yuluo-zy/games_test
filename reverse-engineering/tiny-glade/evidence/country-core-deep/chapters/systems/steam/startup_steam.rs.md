# systems/steam/startup_steam.rs

> 文件作用：**Steam 启动系统（首帧一次性资源插入）。**

函数总计 4：业务 1 / 内联无独立符号 3 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::steam::startup_steam::startup_steam | 0x140a9bbf0 | 42 | 0x140a9bbf0：首帧（全局帧计数）一次性 FUN_1412e0660 Commands 插入 Steam 启动资源束（6 qword 常量 0x142adafb0）；steamworks 客户端 init 族 0x142923700/0x1429236e0 | O | audio-misc\decomp\decomp-0x140a9bbf0.txt |

## 实现方式（算法级）（O，decomp-0x140a9bbf0 + DIGEST §2.11）

**startup_steam @0x140a9bbf0**：
1. **首帧门禁**：全局帧计数（0x140d4d740 Instant 计数）判断启动帧，一次性执行。（O）
2. `FUN_1412e0660(commands, args, 位置串)` → Commands 插入 **Steam 启动资源束**：
   6 qword 常量 0x142adafb0（默认句柄/配置打包）。（O）
3. `0x142923700`/`0x1429236e0` = steamworks 客户端 init 族（库侧边界）。（O）
- check_get_action_set_handle（行 12）/check_get_analog_action_handle（行 22）/
  check_get_digital_action_handle（行 32）= 动作句柄校验包装，内联 I。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::steam::startup_steam::impl$0::check_get_action_set_handle`（行 12）— 谓词/校验
- `country_core::systems::steam::startup_steam::impl$0::check_get_analog_action_handle`（行 22）— 谓词/校验
- `country_core::systems::steam::startup_steam::impl$0::check_get_digital_action_handle`（行 32）— 谓词/校验
