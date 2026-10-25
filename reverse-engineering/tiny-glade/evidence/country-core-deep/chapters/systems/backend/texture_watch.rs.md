# systems/backend/texture_watch.rs

> 文件作用：**纹理热载系统。每帧锁 `TextureWatch` 互斥队列，日志 `'Texture watch: detected changes: N'`，按位图索引遍历变更纹理并调 `FUN_140b73a50`（递归位图树遍历重载助手）逐项重载；错误日志 `'Error processing textures: '`；panic `Result::unwrap()`（texture_watch.rs:43）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::texture_watch::texture_watch` | 0x140aab370 | 9 | Bevy 系统（Res/ResMut）。锁 TextureWatch 互斥项（`*param_2+0x30 → +0x10`）；日志 `'Texture watch: detected changes: '`（0x142add810 区段）+ 计数、`'Error processing textures: '`（0x142add878）；u16 位图迭代器遍历变更纹理，逐项调 `FUN_140b73a50(&..., 0x70, 0x10)` 重载；含 `FUN_140917860`（校验/比较）与 `FUN_140be24b0`/`func_0x00014003bb80`（句柄释放对）。 | O | decomp/decomp-0x140aab370.txt；rne-read --unks/--hex 0x142add810、0x142add878 |
| （助手）`FUN_140b73a50` | 0x140b73a50 | — | 纹理位图索引树遍历器（递归 bitset，无字符串），按 0x20B 槽位迭代重载待处理纹理。 | I | decomp/decomp-0x140b73a50.txt |

## 实现方式（算法级）

**texture_watch @0x140aab370（系统，O）**：
1. LOCK TextureWatch 互斥项（`*param_2+0x30 → +0x10` 标志）；非空闲 → panic `Result::unwrap()`（texture_watch.rs:43，0x142a949f0）。（O）
2. 日志 `'Texture watch: detected changes: '`（0x142add810 区段）+ 变更数；错误路径 `'Error processing textures: '`（0x142add878）。（O）
3. u16 位图迭代器遍历变更纹理，逐项调 `FUN_140b73a50(&..., 0x70, 0x10)` 重载。（O）
4. 含 `FUN_140917860`（校验/比较）+ `FUN_140be24b0`/`func_0x00014003bb80`（句柄释放对）。（O）

**FUN_140b73a50 @0x140b73a50（重载助手，I）**：纹理位图索引树遍历器（递归 bitset，无字符串），按 0x20B 槽位迭代待重载纹理；重载经 startup `load_textures.rs` 路径（跨域，`'texture asset lib'` 串证据）。（I）

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

（无）

## 派生胶水（derive/宏生成；不计独立业务）

（无）

## 备注

- 字符串 `'texture asset lib'`/`load_textures.rs`（startup）在 --unks 大串表中出现，指示重载经 startup 装载路径（跨域线索）。
