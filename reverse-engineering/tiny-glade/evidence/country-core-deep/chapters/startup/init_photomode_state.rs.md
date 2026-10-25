# startup/init_photomode_state.rs

> 文件作用：**照片模式状态初始化与启用。**

函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::init_photomode_state::PhotomodeState::on_enable_photomode` | 0x140870570 | 36 | `PhotomodeState::on_enable_photomode`：机器码 — 写 float 到字段0、字段+4=-1、+8/+0xa/+0xc 清零 = 初始化 PhotomodeState 默认值（视全局标志分支）。 | **O** | hex 手解（rne-read --hex 0x140870570，51B）→ startup/decomp/decomp-0x140870570.txt；原 tg_b2 NotFound |

## 内联/无独立符号函数

- `country_core::startup::init_photomode_state::init_photomode_state`（行 15）— **I**：`init_photomode_state` — 注册 PhotomodeState 资源/初始化（内联）。

## 派生胶水

- 1 项：default 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖；机器码手解）

- **on_enable_photomode 0x140870570（O，机器码，2026-10-15 复验）**：
  `cmp byte [0x143634d90], 0`（demo flag）→ demo：`movss xmm0, [0x142925904]`（**1.0**）/
  非 demo：`movss xmm0, [0x1429cd078]`（**3.0**）→ `[rcx+0]=fov; [rcx+4]=-1; [rcx+8]=0; [rcx+0xa]=0; [rcx+0xc]=0; ret`。
  即 PhotomodeState 默认：{ fov: demo?1.0:3.0, +4: -1(哨兵), +8/+0xa/+0xc: 0 }。常量复验：
  `0x1429cd078=0x40400000`（3.0f）、`0x142925904=0x3f800000`（1.0f）。
- 邻接函数（同一文件第二实例，纠错：原记 0x140870590 应为 **0x1408705b0**）：
  `mov rax,rcx; cmp demo; …; movss [rax]=fov; mov word[+0xc]=0x100; mov qword[+4]=-1`（fov 语义同族，
  字段宽不同：qword[+4] 与 word[+0xc]，应为另一结构/另一 impl）。
- 注：本地址在 tg_b2 数据库反编译为 NotFound（小函数），机器码手解为准（rne-read --hex 0x140870570），
  证据落盘 startup/decomp/decomp-0x140870570.txt。
