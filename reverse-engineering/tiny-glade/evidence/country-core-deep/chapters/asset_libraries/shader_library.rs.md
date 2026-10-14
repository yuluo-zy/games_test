# asset_libraries/shader_library.rs

> 文件作用：**着色器资产库**——`AssetShaderLibrary`（0xe0 B）：raster/compute 两张 `FrozenMap` + 两张名字表
> （`raster_names: RwLock<HashMap<String, Handle>>`、`compute_names: RwLock<HashMap<ComputeShaderSourceHash, Handle,
> NoHashHasher>>`）。`add_raster`/`add_compute` 带查重与 panic；`get_compute_by_name` 供 `ComputePass::new` 复用缓存。
> 证据：`evidence/country-core-audit/A3_shader.md`（O 全量）+ wp4 asset_libraries 合同。

函数总计 8：业务 6 / 内联无独立符号 2 / 派生胶水 0

## 布局表（O，A3_shader，总 0xe0）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `raster_shaders: FrozenMap<Handle<RasterShaderProgram>, Box<RasterShaderProgram>>` | add_raster 0x140936ef0 |
| +0x40 | `compute_shaders: FrozenMap<Handle<ComputeShaderProgram>, Box<ComputeShaderProgram>>`（`add rsi,0x40`） | add_compute 0x140937560 |
| +0x80 | `raster_names: RwLock<HashMap<String, Handle<RasterShaderProgram>>>`（锁 +0x80、ctrl +0x88、mask +0x90、growth +0x98、items +0xa0、RandomState +0xa8/+0xb0，桶 0x18 B） | new / get_raster_handle_by_name |
| +0xb8 | `compute_names: RwLock<HashMap<ComputeShaderSourceHash, Handle<ComputeShaderProgram>>>`（锁 +0xb8、ctrl +0xc0、mask +0xc8、items +0xd8，桶 0x10 B，key 直接当哈希 = NoHash） | new / get_compute_by_name |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `AssetShaderLibrary::new` | 0x140936c60 | 35 | 猜测：构造；证实：两张空表 + RandomState | **O**（656 B）：两张表 RandomState 取线程局部 KEYS（k0 自增），锁字全 0，ctrl 指向静态空组 | A3_shader.md |
| `AssetShaderLibrary::add_raster` | 0x140936ef0 | 46 | 猜测：注册 raster；证实：**签名 `(&self, program: *RasterShaderProgram(0xd8 B), force_new: bool)`**，名字取自程序 `+0x48/+0x50`（&'static str） | **O**（1639 B）：写锁 → `!force_new && items!=0` 才查重 → 命中则逐字段比较 `desc`（+0x08/0x10 path；+0x30/0x38/0x40 Option<String>；+0x20/0x28 Vec<(String,String)> 元素 0x30 B）→ 全等：放锁、drop 新程序、返回已有 handle；不等：`panic_fmt("AssetShaderLibrary: the shader name {} is already taken")`（pieces @0x142ab10d0，:62:21）。未命中/跳过：`lock xadd [0x143634cd0],1` 取 handle → `HashMap::insert(&str, handle)` → Box(0xd8) → `FrozenMap::insert`，Err → `panic("assertion failed: self.raster_shaders.insert(handle, Box::new(shader)).is_ok()")`（:75:9） | A3_shader.md |
| `AssetShaderLibrary::add_compute` | 0x140937560 | 80 | 猜测：注册 compute；证实：**签名 `(&self, program: *ComputeShaderProgram(0xa0 B), hash: u64)`** | **O**（886 B）：写锁；`items!=0` 且键存在 → `panic_fmt`（同一 pieces，参数 = `shader_uri::impl$18::fmt` derive(Debug) `ComputeShaderSourceHash { hash }`，:89:13），**不比较内容**；否则 handle → insert → Box(0xa0) → FrozenMap::insert → 失败断言（:100:9） | A3_shader.md |
| `AssetShaderLibrary::get_compute_by_name` | 0x1409378e0 | 135 | 猜测：按哈希取程序；证实：**返回 `Option<(Handle, &Program)>`**（rax=handle、rdx=程序指针、0=None） | **O**（446 B）：读锁（含 deadlock 资源登记）查表 → **放锁** → `FrozenMap::get(handle)`；缺失 → None（不 unwrap）。调用者 `ComputePass::new` 命中复用（A4 O）。旧版只返回 handle 错 | A3_shader.md |
| `AssetShaderLibrary::get_raster_handle_by_name` | 0x140937aa0 | 144 | 猜测：按名取句柄；证实：**返回 Option<Handle>** | **O**（525 B）：`(&self, &str) -> Option<Handle>`（rax=命中、rdx=handle）；`items==0` 直接 None，否则 `hash_one<RandomState,&str>` → 探测（h2 比对 + len + bcmp） | A3_shader.md |
| `AssetShaderLibrary::debug_raster_get_name` | 0x140937cb0 | 169 | 猜测：调试取名字；证实：读锁下线性扫描返回 `Option<&str>` | **O**（352 B）：`(&self, Handle) -> Option<&str>`（rax=ptr，0=None）；SSE 组位图遍历，桶 `[-8]==handle` 时返回桶 `[-0x18]/[-0x10]`（键 &str）。旧版返回克隆 String 错 | A3_shader.md |

## 内联/无独立符号函数

- `get_compute`（行 109）— **I**：薄封装 `FrozenMap::get`（消费者内联，A3_shader 保留）
- `get_raster`（行 116）— **I**：同上
- PDB 名存在但无独立过程的（A3_shader 已删臆造入口）：`get_compute_handle_by_name` / `get_compute_shader_name` / `get_raster_by_name` / `get_raster_mut` / `extract_raster_shader_names`——procs.tsv 里 shader_library:: 只有上述 6 个过程。

## 纠错/差异（A3_shader，O）

- 旧版形参 `(name, program)`/`&mut self`/相等判据 `source_name+compiled_variants`/panic 文案/键用拥有的 String/无 `force_new`——全错。
- 旧版 `add_compute` 文案 `{:016x} … already in the library` 臆造；键类型 u64 newtype 无 Debug 形态。
- I：`force_new` 语义名；`ComputeShaderSourceHash` 定义于 `shader_uri`（O 符号路径）；`RasterPipelineDesc` 第二字段原版为 `Option<String>`（O 比较序列 niche 判别）。
