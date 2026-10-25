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
- **round3 闭合（2026-10-15）**：JSON schema 已解出，见下方 §算法要点；U 解除。

## 算法要点（本批深挖 2026-10-15，decomp: asset-libraries/decomp/decomp-0x1409365d0 + rne-read）

### load<PathBuf> 流程（0x1409365d0，1674 B，返回 (Vec<PlacementDataSsbo>, Handle) 0x20 B）

1. 表 B 非空（`[self+0x48] != 0`）才查重：`hash_one<RandomState,&PathBuf>` + 探测命中 → `panic_fmt(["AssetPlacementDataLibrary: the placement data from ", " has already been loaded"], path.display())`（pieces 0x142ab0fd0/0x142ab0ff0，行 93 列 13；**先查重后读文件**）。
2. `fs::read_to_string(path)` 失败 → `panic_fmt(["Couldn't find  ", …])`（pieces 0x142ab1018 起——**"Couldn't find " 后有两个空格**，行 100 列 63，io 错误丢弃）。
3. `serde_json::from_str::<Vec<PlacementDataSsbo>>` Err → unwrap panic（行 101 列 73）。
4. `TypedStorageBuffer<PlacementDataSsbo>::new(device, Some(&data))`（`len*size` 字节上传 GPU）。
5. `lock xadd [0x143634cd0], 1` 取全局 handle。
6. `A.insert(handle, (buf, len))`（旧值非 None 则 drop 句柄）→ `B.insert(path, handle)`（返回 (Vec, handle)，文本释放）。

### 关键常量/布局

| 项 | 值 | 含义 |
|---|---|---|
| 表 A | HashMap<Handle<PlacementDataSsbo>, (TypedStorageBuffer<PlacementDataSsbo>, usize)> @+0x00（条目 0x38 B） | handle → 缓冲+条数 |
| 表 B | HashMap<PathBuf, Handle> @+0x30（条目 0x28 B） | 路径 → handle（查重 + 复用入口） |
| RandomState | @+0x20 / @+0x50（k0 自增，第二张 k0 大 1） | 两张表各自哈希种子 |
| 全局 handle | 0x143634cd0 | 与 mesh/vao/shader/ssbo 共用 |

### 模块间调用 / U

- `load` → TypedStorageBuffer::new（SsboType 族）+ 全局 HANDLE_COUNT；`register_required_components` 0x14086c3d0 = bevy derive 胶水。
- **PlacementDataSsbo JSON schema（round3 闭合，tg_ca1 反编译族，O）**：`from_str::<Vec<PlacementDataSsbo>>` 链 = `FUN_141405cf0`（from_str）→ `FUN_1414106e0`（Vec）→ `FUN_140967640`（逐元素收集，stride 0x30=48B）→ `FUN_14142f860`（visit_map，28.5KB，调 `FUN_140b3b960`=__Field visit_str + `FUN_140c90df0`=pack_2x16f_uint×2）。
  - **JSON 字段（tag=声明序）**：`position:[f32;3]`(必需)、`scale:f32`(必需)、`rotation_degrees:f32`(必需；**别名 `rotation_angle`**——FIELDS 串表 0x142c5cd48/0x142c5d028..058 主名 16 字符)、`id:i32/u32`(必需)、`custom0:u32`(可选,pack_2x16f)、`custom1:u32`(可选,pack_2x16f)、`bend_vector:[f32;2]`(可选)、`pad0:u32`(可选)。未知键跳过（tree JSON 的 `pad1` 即跳过）。
  - **元素 0x30=48B**（缓冲创建/Vec drop/逐元素 stride 三处证实）；reconstruction `PlacementDataSsbo{position,scale,angle,gap}` 仅 20B，**缺 id/custom0/custom1/bend_vector/pad0**。
  - 真实资产（`assets/glade/{theme}/trees/*.json`）：`rotation_angle` 值为**度**（112.97 等）；解析器（FUN_14140cfd0）为普通 f32，未见度→弧度转换——与 `to_affine3a`（0x140ad5020，+0x10 值×0.5→sin/cos）的弧度解释关系待集成裁决（I）。
  - 证据：`asset-libraries/decomp/decomp-0x140b3b960.txt`（visit_str）、`decomp-0x14142f860.txt`（visit_map）、`decomp-0x14140cfd0/0x14140d270/0x141412c00/0x1414132b0.txt`（字段解析器）、`decomp-0x1416164c0/0x141405cf0/0x1414106e0/0x140967640.txt`（元素尺寸）；详见 `asset-libraries/ALGORITHMS.md` §7 与 `asset-libraries/CLOSURE-ROUND3.md`。
- 消费方（读取表 A 的渲染/CPU 侧）未见独立过程。
