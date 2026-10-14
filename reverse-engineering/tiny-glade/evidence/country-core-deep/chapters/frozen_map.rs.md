# frozen_map.rs

> 文件作用：**FrozenMap 容器调用面（外部库 `frozen`，离线缓存缺失）。契约已由调用点钉死：值装箱永不移动/删除；insert 键已存在返回已有值引用；表带“借用中”字节。**

函数总计 0：业务 0 / 内联无独立符号 0 / 派生胶水 0（库本体不重写，只复刻契约）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `FrozenMap::insert` | 0x140969bf0 | — | 键已存在→不替换，返回已有值引用（调用方据此拼 panic 消息） | O | reconstruction/frozen_map.rs 注释（调用点证据） |
| `FrozenMap::get` | 0x1405ca870 | — | 键相等才返回；返回 Box 内部值（引用生命周期延长到 &self） | O | 同上 |
| `FrozenMap::get_mut/remove` | (内联) | — | get_mut 内联表探测；remove 对内部 hashbrown remove_entry | O | 同上 |
| `借用中字节` | (数据型) | — | [map+8] 字节；export_from_rg 等遍历入口 try_borrow().unwrap() | O | 同上 |
