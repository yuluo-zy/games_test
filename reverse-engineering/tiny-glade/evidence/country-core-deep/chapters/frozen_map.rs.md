# frozen_map.rs

> 文件作用：**FrozenMap 容器调用面（外部库 `frozen`，离线缓存缺失）。契约已由调用点钉死：值装箱永不移动/删除；insert 键已存在 → 丢弃新值 Box + 返回标志 1 + 表不变（round3 修正，"返回已有值引用"与机器码不符）；表带“借用中”字节。**

> **round3 集成登记**：`reconstruction/country-core/src/frozen_map.rs` 现实现 `insert -> Result<(), &V>` 返回**已有值引用**（与机器码 `Err(1)` + 丢弃新值 Box 的结构不同，行为等价），且**未建模借用字节 @+8 的冲突 panic**（get_mut/remove 与遍历互斥）。只登记待集成裁决，不改源码（见 `asset-libraries/ALGORITHMS.md` §14.2 / `CLOSURE-ROUND3.md` §5）。

函数总计 0：业务 0 / 内联无独立符号 0 / 派生胶水 0（库本体不重写，只复刻契约）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `FrozenMap::insert` | 0x140969bf0 | — | 键已存在→不替换：丢弃新值 Box（vtable drop+free），返回标志 1，表不变；键缺失→写入 key+value、len+1，返回 0（round3 复查修正：原"返回已有值引用"与机器码不符） | O（round3 复查修正） | round3 MCP 反编译 0x140969bf0（tg_rev1）；reconstruction/frozen_map.rs 注释 |
| `FrozenMap::get` | 0x1405ca870 | — | 键相等才返回；返回 Box 内部值（引用生命周期延长到 &self） | O | 同上 |
| `FrozenMap::get_mut/remove` | (内联) | — | get_mut 内联表探测；remove 对内部 hashbrown remove_entry | O | 同上 |
| `借用中字节` | (数据型) | — | [map+8] 字节；export_from_rg 等遍历入口 try_borrow().unwrap() | O | 同上 |

## 实现方式（算法级）（O，round3 MCP 复查）

**FrozenMap 容器调用面（外部库 `frozen`，本体不重写，只复刻契约）**：
- `insert @0x140969bf0`：键已存在 → **不替换**：丢弃新值 Box（vtable drop+free），返回
  标志 1，表不变；键缺失 → 写入 key+value、len+1，返回 0（round3 复查修正：原“返回已有
  值引用”与机器码不符）。（O）
- `get @0x1405ca870`：键相等才返回；返回 Box 内部值（引用生命周期延长到 &self）。（O）
- `get_mut`/`remove`：get_mut 内联表探测；remove 对内部 hashbrown `remove_entry`。（O）
- **借用中字节 [map+8]**：遍历入口（export_from_rg 等）`try_borrow().unwrap()` 使用。（O）
- 值装箱永不移动/删除（契约）。（O）
