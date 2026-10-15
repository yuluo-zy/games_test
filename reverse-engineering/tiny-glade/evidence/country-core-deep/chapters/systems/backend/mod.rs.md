# systems/backend/mod.rs

> 文件作用：**后端系统族插件注册。`BackendPlugin` 三入口：`add_backend_systems`（App::add_systems 注册 ~7 个后端系统并配置 GameSystemSet）、`insert_backend_resources`（3 分支状态机插入后端资源/按路径装载）、`add_backend_events`（注册 6 个事件/资源类型）。含 `TimedFrameEvent`。**

函数总计 4：业务 3 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::impl$0::add_backend_systems` | 0x140942c80 | 66 | `fn add_backend_systems(app: &mut App, config)`：对每个后端系统构造 SystemConfig（`FUN_140b218a0`+`FUN_141453000`，GameSystemSet 变体 1/2/3/5/10/0x2c），`FUN_140b67280` = `app.add_systems(Update, ...)`；`if (*(config+0xe7) == 1)` 条件追加一个系统。 | O | decomp/decomp-0x140942c80.txt |
| `country_core::systems::backend::impl$0::insert_backend_resources` | 0x140942f80 | 171 | `fn insert_backend_resources(app, config)`：按 `*config` 判别（0=全新 / 1=带状态 / 2=按路径）三分支。0/1 分支经 `FUN_140911710`+`FUN_140af8c50` 组装 + `FUN_141301640`（`insert_resource<T>`，TypeId 哈希 `UNK_142ab3340`）；2 分支拼路径（`FUN_140d76ed0`）后 `FUN_1409499c0` 装载。 | O | decomp/decomp-0x140942f80.txt |
| `country_core::systems::backend::impl$0::add_backend_events` | 0x140943660 | 201 | `fn add_backend_events(app)`：对 6 个类型哈希（0xc0703d5101ef3264 等）经 `FUN_1400dd6f0`（name-hash 查 app+0x38 注册表）+ 注册函数 `FUN_141511490`/`FUN_141515270`/`FUN_1415092a0`/`FUN_14151f350`/`FUN_141517dc0`/`FUN_141508640` 各注册一个事件/资源类型。 | O | decomp/decomp-0x140943660.txt |

## 实现方式（算法级）

**add_backend_systems @0x140942c80（O）**：逐系统构造 SystemConfig（FUN_140b218a0 + FUN_141453000，GameSystemSet 变体 1/2/3/5/10/0x2c）→ `FUN_140b67280` = `app.add_systems(Update, ...)`，注册 ~7 系统；`if (*(config+0xe7)==1)` 条件追加一系统（身份 U 细分）。（O）

**insert_backend_resources @0x140942f80（O）**：按 `*config` 判别三分支状态机：
- 0（全新）/1（带状态）：`FUN_140911710` + `FUN_140af8c50` 组装 → `FUN_141301640` = `insert_resource<T>`（TypeId 哈希 UNK_142ab3340）。（O）
- 2（按路径）：拼路径（FUN_140d76ed0）→ `FUN_1409499c0` 装载。（O）

**add_backend_events @0x140943660（O）**：6 类型哈希（0xc0703d5101ef3264 等）经 `FUN_1400dd6f0`（name-hash 查 app+0x38 注册表）+ 六注册函数（FUN_141511490/FUN_141515270/FUN_1415092a0/FUN_14151f350/FUN_141517dc0/FUN_141508640）各注册一个事件/资源类型。（O）

**索引错位（I）**：sysres 把 `insert_backend_resources::closure$0` 挂在 0x140943660（实为 add_backend_events）；closure$0 真实地址 U。（I）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（`insert_backend_resources::closure$0`，见备注）

## 备注

- **索引错位**：sysres-procedures-v3.json 把 `insert_backend_resources::closure$0` 映射到 0x140943660，但该地址反编译行为 = `add_backend_events`（6 类型注册），与 SOURCE_MAP（add_backend_events=0x140943660）一致。closure$0 实际地址未定位（I）。
- 配置标志 `+0xe7` 决定追加系统的具体身份未定位（U 细分）。
