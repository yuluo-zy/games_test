# render/mod.rs

> 文件作用：**render 模块根**——模块装配 + `GraphicsDeviceResource`（`Arc<GraphicsDevice>` bevy 资源）+
> `AvailableGraphicsDevices`/`AvailableGraphicsDevice`（启动时枚举的物理适配器列表，0xa0 B 元素）。
> 证据：reconstruction `render/mod.rs`（O）+ 本批字节复核 0x140987cb0。

函数总计 4：业务 1 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `AvailableGraphicsDevices::from_preference_order` | 0x140987cb0 | 97 | 猜测：按偏好排序；证实：**就地 `reverse()`**——成对交换 0xa0 B 元素 | **O**（本批字节复核）：`mov rax,rcx; cmp rdx,2; jb +0x10a`（len<2 早退）→ `shr rcx,1`（len/2 轮）→ `lea r8,[rax+0x90]`（元素 +0x90）、`shl r9,5`（×0x20）、movups 对换 0xa0 B 元素——与 reconstruction 的 `devices.reverse()` 一致。输入排列方向/用途（I：调用方按"最不优先在前"给出，列表内部最优在前）不在函数内体现 | hex 手解 0x140987cb0（rne-read --hex 本批；原 hex 手解 0x140987cb0（rne-read --hex 本批；原 reconstruction render/mod.rs + 本批 rne-read 0x140987cb0）） |

## 派生胶水（3 项：default、deref 等）

- `GraphicsDeviceResource` 的 Deref（`[self]+0x10` = ArcInner 数据偏移，O 0x140987ca0）与 Default 胶水——已在 reconstruction 实现（`Deref` 手写），不补。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `AvailableGraphicsDevices::from_preference_order`（0x140987cb0）—— 就地逆序
算法（O，字节复核）：`len < 2` 早退（`cmp rdx,2; jb`）→ `len/2` 轮（`shr rcx,1`）→ `lea r8,[rax+0x90]`（元素 +0x90 偏移、`shl r9,5` ×0x20 步进）→ movups 成对交换 0xa0B 元素 → 等价于 reconstruction 的 `devices.reverse()`。输入排列方向/用途（I：调用方按"最不优先在前"给出，列表内部最优在前）不在函数内体现。
- 数据流：启动时枚举物理适配器 → 按偏好排序 → `AvailableGraphicsDevice` 160B 记录（字段布局 U）→ 选中的 device 装进 `GraphicsDeviceResource(Arc<GraphicsDevice>)`（Deref = [self]+0x10 = ArcInner 数据偏移，O）→ 所有 render pass 的 `&**device` 取 rhapsody GraphicsDevice。

## 纠错/差异

- 旧版直接存设备本体错：资源里是 `Arc<GraphicsDevice>`（`Deref::deref` = `[self]+0x10`，O）。
- `AvailableGraphicsDevice` 160 B 记录字段布局 U（typed gap，reconstruction `_raw: [u64; 20]`）。
- 模块装配的 wp2/wp4 模块清单见 reconstruction 头注释（与 SOURCE_MAP 一致，无缺漏）。
