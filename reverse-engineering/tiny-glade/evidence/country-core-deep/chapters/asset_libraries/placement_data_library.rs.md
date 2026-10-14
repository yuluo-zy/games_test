# asset_libraries/placement_data_library.rs

> 文件作用：**放置数据库**——`AssetPlacementDataLibrary`（0x60 B）：表 A = `HashMap<Handle<PlacementDataSsbo>,
> (TypedStorageBuffer<PlacementDataSsbo>, usize)>` + 表 B = `HashMap<PathBuf, Handle>`。`load(path)` 读 JSON →
> 建 GPU 缓冲 → 取全局 handle → 双表登记，返回 `(Vec, handle)`。证据：`evidence/country-core-audit/A3_color.md` §二（O）。

函数总计 6：业务 3 / 内联无独立符号 2 / 派生胶水 1

## 布局表（O，A3_color）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | 表 A：`HashMap<Handle<PlacementDataSsbo>, (TypedStorageBuffer<PlacementDataSsbo>, usize)>`（条目 0x38 B，drop_inner_table 类型名 O）+ RandomState @0x20 | new 0x140867700 |
| +0x30 | 表 B：`HashMap<PathBuf, Handle>`（条目 0x28 B；PathBuf 0x20 B）+ RandomState @0x50 | new 0x140867700 |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$2::register_required_components` | 0x14086c3d0 | 10 | 猜测：bevy 注册；证实：bevy derive(Component) 产物 | **O**：bevy 注册胶水（A3_color 判），不重写 | A3_color.md §二 |
| `AssetPlacementDataLibrary::new` | 0x140867700 | 34 | 猜测：构造；证实：两张空表 + 两个 RandomState | **O**（328 B）：0x60 B；表 A @0（空表 4×u64）+ RandomState @0x20；表 B @0x30 + RandomState @0x50；`RandomState::new()` 读线程局部 KEYS（k0 自增），第二张表 k0 比第一张大 1。旧版单表 `HashMap<PathBuf, PlacementData>` 错 | A3_color.md §二 |
| `AssetPlacementDataLibrary::load<PathBuf>` | 0x1409365d0 | 84 | 猜测：加载数据；证实：JSON 读取 + 缓冲创建 + 双表登记 | **O**（1674 B）：B 非空（`[self+0x48]!=0`）才查重，命中 → `panic_fmt(["AssetPlacementDataLibrary: the placement data from ", " has already been loaded"], path.display())`（行 93 列 13）；`read_to_string` 失败 → `panic_fmt(["Couldn't find ", …])`（行 100 列 63）；`serde_json::from_str::<Vec<PlacementDataSsbo>>` Err → unwrap（行 101 列 73）；`TypedStorageBuffer::new(device, Some(&data))`；`lock xadd [0x143634cd0], 1` 取 handle；`A.insert(handle, (buf, len))`；`B.insert(path, handle)`；返回 `(Vec, handle)`（0x20 B） | A3_color.md §二 |

## 内联/无独立符号函数

- `AssetPlacementDataLibrary::clear`（行 41）— **I**：清空双表（无独立符号）
- `AssetPlacementDataLibrary::add`（行 108）— **I**：登记已有缓冲（无独立符号；PDB 名存在）

## 派生胶水（1 项：impl$1::load::closure$0）

- load 的闭包（无独立符号；I）。

## 纠错/差异（A3_color，O）

- 旧版错：不读文件、返回 `Result<(),()>`、`add`/`clear` 不存在。
- U：`PlacementDataSsbo` 的 JSON schema（deserialize_struct 含 u32/f32/i32/[f32;3]/[f32;2] 与 `utils::pack_2x16f_uint`，与 resources/placement_data.rs 现有字段不符，属另一工作包）；`Handle::new` 计数器即 `HANDLE_COUNT`（I，O 为 `lock xadd [0x143634cd0]` 同址模式）。
