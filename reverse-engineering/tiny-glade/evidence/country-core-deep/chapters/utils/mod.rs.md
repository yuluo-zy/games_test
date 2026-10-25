# utils/mod.rs

> 文件作用：**游戏自有杂项工具（事件注册表读取、地图边界、屏幕投影、资源取值、标题转换）**。
> `get_last_event`/`drain_events` 走**自定义 `EventRegistry`**（TypeId→Events HashMap，
> hasher@+0x38, buckets@+0x1e0, len@+0x210, cap@+0x218, tick@+0x5b0），**非 Bevy Events**
> （0.16 Bevy Events 是双 VecDeque 结构，与此不同）；未注册事件 panic
> `Event not registered: {TypeName}`。`world_space_to_screen_space` 为 4×4 VP 矩阵变换 +
> 透视除 + 屏幕缩放。与共享 `utils` 无重叠（共享无事件注册表/屏幕投影）。

## 核心算法

```
get_last_event<T>() -> Option<T>:
  按 T 的 TypeId（`type_id::of`）哈希（实例：0x6ecb4684a4c9e1f9 / 0xdac2c590678dd6c0）查注册表
  → 未注册 panic `Event not registered: {TypeName}`（UNK_1429ae260）
  → 取 Events 末条（拷贝载荷）
drain_events<T>() -> Vec<T>: 同注册表，排空入 Vec
world_space_to_screen_space(world_pt, view_proj, screen_size):
  4×4 矩阵乘列（world 乘 VP）+ 透视除（w 分量）+ 屏幕尺寸缩放（原字节手解）
get_res/get_res_mut/is_inside_map_bounds/convert_function_expr_to_title:
  内联宿主，名称推断（I）
```

函数总计 10：业务 5 / 内联无独立符号 3 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `get_last_event<CustomCursorSsboCreatedCmd>` | 0x1402b4910 | 47 | 事件注册表取末条 | **O** | TypeId hash `0x6ecb4684a4c9e1f9`；panic `Event not registered`(0x1429ae260) |
| `get_last_event<OsCursorCaptureCmd>` | 0x1402b4aa0 | 47 | 同上（枚举变体） | **O** | 同注册表 |
| `get_last_event<OsCursorVisibilityCmd>` | 0x1402b4c40 | 47 | 同上 | **O** | 同注册表 |
| `drain_events<OsWindowCmd>` | 0x1402b47f0 | 55 | 注册表排空入 Vec | **O** | hash `0xdac2c590678dd6c0` |
| `world_space_to_screen_space` | 0x140ac8780 | 83 | 世界→屏幕 | **O** | 原字节：矩阵乘列 + 透视除 + 屏幕缩放 |

## 内联/无独立符号函数

- `get_res`（行 34）— **I**（资源取值，名称推断）
- `get_res_mut`（行 41）— **I**（资源可变取值）
- `is_inside_map_bounds`（行 75）— **I**（地图边界谓词，名称推断）

## 派生胶水（2 项：closure 等）
