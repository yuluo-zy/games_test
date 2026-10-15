# asset_libraries 模块算法总览（2026-10-15 深挖 + round3 闭合）

> 覆盖 12 文件 / 387 业务函数。本文档把各章节"作用"升级为**算法级描述**：数据结构布局、注册/查询/删除控制流、关键常量、热重载状态机、模块间调用。
> 分级沿用 O/I/U：O = 机器码/反编译直接观察，I = 推断，U = 未知。
> 本批新证据：`asset-libraries/decomp/decomp-*.txt`（tg_m1 反编译 27 函数 + tg_ca1 round3 反编译 PlacementDataSsbo 反序列化族）+ rne-read 常量复核；章节逐文件补"算法要点"节。
> round3 闭合（2026-10-15，详见 `asset-libraries/CLOSURE-ROUND3.md`）：PlacementDataSsbo JSON schema（§7）、texture_watch 消费端（§9）、load_json_as_mesh 引用（§3）、get_rg_buffer panic 槽序与 FrozenMap::insert 语义（§14 裁决项）。
> 与本批无关的既有证据（A3_*.md、L0_NOTES、round3）仍为底层来源。

---

## 0. 全局机制（所有库共用）

### 0.1 全局 handle 计数器 `HANDLE_COUNT` @ 0x143634cd0（O）

`lock xadd [0x143634cd0], 1`——每次取号递增，**失败路径也消耗 id**（vao add 先于校验取号）。共享方：
mesh(add 0x14092cc60)、vao(add 0x140924180)、shader(add_raster/add_compute 0x140936ef0/0x140937560)、
placement(load 0x1409365d0)。**ssbo 库不使用**（add_new 0x1412bdaf0 全机器码无 lock xadd——SsboId 哈希即键，库内无 Handle 概念）。
组件/资源真实在场（d_cb_a §2）：`Handle<Mesh>` = 0x877acce28b84e29f、`AssetSsboLibrary` = 0x8226cc786e679355、`AssetMeshLibrary` = 0xd455e4bbfd4f066e。

### 0.2 FrozenMap 语义（O，round3 修正 + 本批机器码）

`FrozenMap<K, Box<V>>` 布局：借用字节 @+8、hashbrown {ctrl@+0x10, mask@+0x18, items@+0x28, RandomState@+0x30}；自带 std RwLock（状态字 @+0：0=无锁、0x3fffffff=写持有、读者计数）。

- **`insert(&self, key: K, value: Box<V>) -> Result<(), ()>`**：键已存在 → **丢弃新值 Box**（vtable drop → dealloc(data,size,align) → dealloc(box,0x40,8)），**表不变**，返回 Err(1)；键缺失 → 写 `{key,value}` 桶（0x10 B）、items+1、返回 Ok(0)。**永不替换**（"freeze"）。
- `get(&self, &K)`：h2 = key>>0x39 SSE 组探测 + 全键比较。
- 借用簿记：get_mut/remove 命中借用字节 @+8 → 冲突时 panic（"called `Result::unwrap()` on an `Err` value"）。

### 0.3 游戏 String/Vec 布局（O）

- 游戏自有 `String` = **cap-first** `{cap@0, ptr@8, len@0x10}`；`Option<String>` None 以 **cap = i64::MIN** 为 niche（configwatch 消费端 `cap >= 0` 判 Some，dealloc(ptr, cap, 1) 释放，置 None = 全零 + cap 0x80 高字节）。
- 游戏 `Vec` = cap-first `{cap@0, ptr@8, len@0x10}`（texture DescriptorMap pending {cap@0x90, ptr@0x98, len@0xa0} 等）。
- std `Option` niche / enum 判别以 i64::MIN 为基址（watch 回调事件变体 `tag ^ 0x8000000000000000`）。

### 0.4 构建事实（本批新发现，O）

- panic 位置串指向 `/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/…`——**Linux 构建机交叉编译 Windows**，私有仓库名 country-slice-private。
- rustc 提交 `05f9846f893b09a1be1fc8560e33fc3c815cfecb`（library 路径串）。notify 变体序按 notify 4.0.17（texture watch 掩码 0x4c + 变体数 9 推出，I）。

---

## 1. ssbo_library.rs（334 业务函数，12 模板族归并）

### 数据结构

| 结构 | 布局 | 证据 |
|---|---|---|
| `AssetSsboLibrary` | `FrozenMap<SsboId, Box<BufferState>>`（借用@+8、hashbrown@+0x10） | impl$6::default 0x14091eb60 |
| `SsboId` | u64 = `xxh3_64(T::NAME 短名)`；编译期常量内联（0x1412bdbca movabs）+ .rdata 静态（0x1429adf08） | verify_ssbo_id.py |
| `BufferState` | 0x40 B：`{data@0, vtable@8, name.ptr@0x10, name.len@0x18, rg RwLock{lock@0x20, handle 20 B@0x28..0x3b, None-tag@0x36=5}}` | BufferState::new 0x14091eb30 机器码 |
| `Ssbo` trait vtable | `[drop, size, align, +0x18 as_any, +0x20 as_any_mut, +0x28 buffer, +0x30 handle]`；impl$0=TypedStorageBuffer(0x28)、impl$1=MappedStorageBuffer(0x38)、impl$2=InstanceBuffer(0x30)、impl$12=SparseInstanceBuffer(0x90) | L0_NOTES |

### 注册/查询/删除算法（O，机器码级）

**add_new<T>**（73 实例）：`T::new_storage_buffer(device, None)` → 栈上组装 0x40 B BufferState → Box(alloc 0x40,8) → **直接 `FrozenMap::insert`（无前置 get）**：
- Ok(0) → 返回；
- Err(1)（键已存在，insert 已 drop 新 Box、表不变）→ 读表内既有 BufferState 的 `name{ptr@+0x10, len@+0x18}`：`len == type_name.len` 且 memcmp 相等 → panic "Tried to register the same storage buffer twice: {NAME} (type: {full})"（pieces 0x142c2d628）；否则 → panic "Tried to register two storage buffers with the same \`NAME\`: {full}"（0x142c2d568）。
- **add_new_with_data<T>**（9 实例）：同，仅 `new_storage_buffer(device, Some(&data))`（`len*size_of::<T>()` 字节）。

**get_rg_buffer<T>**（81 实例，0x1402b31a0 三段式）：
1. `FrozenMap::get(&SsboId)` 缺失 → 格式化 panic "Buffer {full} (type: {NAME}) is not available in the library. Forgot to register it?"（pieces = ["Buffer "(0x1429ae008/7), " (type: "(0x1429ae00f/8), " is not available in the library. Forgot to register it?"(0x1429ae017/0x39)]；args = [全名(0x1429fcd5c/0x49=73), 短名(0x1429ae158→0x1429ae138/28)]——**机器码参数序：全名在前**，与 wp4/reconstruction "Buffer {NAME(短名)} (type: {full})" 槽序相反，裁决见 §14.1）；
2. 读锁 rg：`+0x36 != 5`（Some）→ 拷 20 B 句柄 @+0x28..0x3b 输出、放锁返回；
3. `+0x36 == 5`（None）：放读锁 → **再查一次表**（缺失 unwrap @0x1429adf78）→ `assert_eq!(name, type_name)`（len 0x49 + memcmp 0x1429fcd5c）→ `vtable+0x30`（Ssbo::handle）→ `rg.import_unowned(handle, 0x37)`（FUN_140bd4fc0）→ **第三次查表**（缺失 unwrap @0x1429adf90）→ 写锁存 Some → 句柄输出。
- **渲染导入按需发生**（CPU 面 get 之后），非注册时；`export_from_rg` = 全表遍历 + 每条目 `take()`（先写 +0x36=5 再 `rg.export(handle, 0x37)`）。

**get_mut<T>**（5 实例）：表空/缺失 → None；命中 → assert_eq name → vtable+0x20 as_any_mut。

### 关键常量表

| 常量 | 值 | 含义 |
|---|---|---|
| SsboId 例 | 0x802ff23dee42ab47 = xxh3_64("Cobblestone3RasterStagingSsbo")；0xef2a68c12cde3f0d = xxh3_64("ColorLibrary")；0x69cdc275a605bad6 = xxh3_64("NaniNextVisibilityBitmaskSsbo") | 短名哈希（非完整类型名） |
| rg 绑定 | 0x37 | import_unowned/export 队列族实参 |
| BufferState | 0x40 B | 数据+胖指针+名字+锁+句柄 |
| None 标签 | +0x36 == 5 | rg Option<ImportedBuffer> = None |
| 撞键返回 | Err(1)，新值被 drop | FrozenMap 不替换 |

### 模块间调用

add_new → `T::new_storage_buffer`（SsboType 族：NAME/BUFFER_LEN/default_value/new_storage_buffer 单态化 334 函数）；get_rg_buffer → FrozenMap::get（0x1405ca870）+ rg.import_unowned（0x140bd4fc0）；export_from_rg → rg.export（0x140bd5120）。消费方：`pass_resource_access::read/write_access_buffer<T>`（A4，81 型）、d_cb_a/b 69 draw 回调、`update_color_library`（ColorLibrarySsbo 键 0xef2a68c12cde3f0d）、`get_cpu_shadow_words`（ShadowedBuffer 4 字节影子，TerrainRockMaskSsbo）。

---

## 2. texture_library.rs（13 业务函数）

### 数据结构（总 0xc8 B，O）

| 偏移 | 字段 |
|---|---|
| +0x00 | `textures: Vec<NamedTexture>`（stride 0x38） |
| +0x18 | `bindless_textures_dset`（0x18 B 持久描述符集） |
| +0x30 | `by_name: HashMap<NameHash, NamedTexture, NoHashHasher>`（桶 0x18） |
| +0x50 | `presets: HashMap<TexturePreset, PresetTextureState, RandomState>`（桶 0x68） |
| +0x80 | `std::sync::RwLock<DescriptorMap>`：futex@+0x80、poison@+0x88、数据 {pending Vec {cap@0x90,ptr@0x98,len@0xa0}, indices Vec {cap@0xa8,ptr@0xb0,len@0xb8}, next u32@0xc0} |
| — | `NamedTexture{handle, is_virtual}`；`PresetTextureState{texture@+8, rg_handle 0x24 B@+0x40, None 标签=0x22 字节==7}` |

**`Option<DescriptorIndex>` 槽 = 8 B `{tag:u32@0, value:u32@4}`，None = {0,0}**（assign_auto_resize 0x1412f8f60 机器码 `mov [slot],1; mov [slot+4],idx` 证实——非 niche 优化布局）。

### 双表一致性（textures Vec + by_name）

- `add(Option<String>, Texture) -> AssetIndex(旧 len)`：Some → by_name 查重（重名 panic "AssetTextureLibrary: the name {} is already taken"，pieces 0x142ab1b60）→ `textures.push` → `trace!("Loaded texture {:?} as {:?}")`（级别 5，**入 by_name 之前**）→ Some → `assert!(by_name.insert({handle, is_virtual:false}).is_none())`。
- `create_name_mapping(name, target)`：查 target（缺失 panic）→ `by_name.insert(hash(name), {target.handle, is_virtual:true})`；**insert 在先**——旧值真纹理 → panic "Onoz, …is a real texture…"（无回滚）；旧值别名 → 静默覆盖。
- 查询：`get_handle_and_texture_by_name(hash)` = 命中且 `index < len`；`get_by_name(hash)` 两条 panic（未命中 / 越界）。

### bindless 分配与 dset 刷新

- `handle_to_bindless_index(asset)`：读锁 → `indices[asset]` tag!=0 → 返回 value；否则放读锁（**不复查**）→ `idx = next; next += 1` → `assert!(idx < 0x1fe0)`（0x1fe0 = REQUIRED_BINDLESS_DESCRIPTOR_COUNT − RESERVED_DESCRIPTOR_COUNT）→ `pending.push(asset)` → `assign_auto_resize(indices, asset, Some, idx)`（==len push、<len 覆盖、>len 全 0 补齐）。注：Ghidra 反编译中分配段无独立锁指令（A3 记"取写锁"；函数尾 `lock xadd +0xc0000001` 为 std 写守卫释放展开——并发安全依赖 bevy 主线程，按机器码如实记录）。
- `bindless_textures_dset()`：写锁 → pending 空直接返回 &dset(+0x18)；否则 Bump::new → **drain pending（len 置 0 不析构）** → 逐项闭包（0x140a96740）：`indices[a].expect` → `textures[a]` → `get_vk_image_view(tex,0).unwrap()` → bump `{0,0,view,5}` → `update_persistent_descriptor_set` → Bump 析构 → 返回。

### 关键常量表

| 常量 | 值 | 含义 |
|---|---|---|
| bindless 上限 | 0x1fe0（8160） | REQUIRED − RESERVED |
| rg 绑定 | 0x22 | import_unowned/export（ssbo 系为 0x37） |
| 描述符 | {0,0,view,5} | VkDescriptorImageInfo 形状 |
| 预设 | 0..=0x19 共 26 | enum_iterator::All<TexturePreset>（跳转表 0x142c65c18） |
| dset 建参 | {len 0x11, u32 1, u8 4} × 1 | bindless_textures dset |
| 预设 None | 0x22 字节 == 7 | PresetTextureState.rg_handle None 标签 |

### 模块间调用

add → by_name（NoHash）；image_name_to_index → `NameHash::compute`（0x140c9cd80）→ handle_to_bindless_index；bindless_textures_dset → rhapsody get_vk_image_view + update_persistent_descriptor_set；import/export_preset → rg.import_unowned/export（0x22）。create_preset_textures → `From<TexturePreset> for TextureDesc`（0xa59890，**属 texture_presets.rs 未恢复**）。

---

## 3. mesh_library.rs（12 业务函数）

### 数据结构（总 0xd0 B，O）

| 偏移 | 字段 |
|---|---|
| +0x00 | `Vec<Handle<Mesh>>`（cap-first；**modified 脏标记队列**，get_mut/remove 无条件压入） |
| +0x18 | `Arc<Mutex<Vec<_>>>`（transient_meshes；元素类型 U） |
| +0x20 | `FrozenMap<Handle<Mesh>, Box<Mesh>>`（0x40 B 主表） |
| +0x60 | `RwLock<HashMap<String, Handle<Mesh>>>`（names，桶 0x20） |
| +0x98 | `RwLock<HashMap<PathBuf, Handle<Mesh>>>`（paths，桶 0x28） |

### 注册/查询/删除算法

- `add({asset: Mesh(0x78), name: Option<String>})`：HANDLE_COUNT++ → source_file Some 克隆路径 → Box → FrozenMap::insert（Err → 断言 panic 0x142aafbf1）→ names.insert（同名覆盖）→ paths.insert。**mesh 表键冲突 = panic（ssbo 表为双文案 panic）**。
- `get_mut`：**无条件 push modified（含不存在 handle）** → 借用检查 → 表空 None → 哈希查。
- `remove`：借用 unwrap → remove_entry（drop Mesh + dealloc 0x78,8）→ names.retain（!= handle）；**不动 paths 表**（悬垂路径映射保留——下次 reload 该路径会 panic，设计如此）。
- `reload_mesh_src(path)`：paths 查（缺失 panic 0x142aafe20）→ `assert_eq!(extension, "json")`（0x142aafd10）→ get_mut().unwrap() → `load_json_as_mesh`（0x140b0eeb0，Err = indices.cap==i64::MIN niche）：Ok → **drop 旧 Mesh + memcpy 0x78 B 原地替换**（依赖方句柄仍有效）；Err → log::error（旧网格保留）。

### 关键常量

| 常量 | 值 | 含义 |
|---|---|---|
| Mesh | 0x78 B（indices.len @+0x10，source_file Option<PathBuf> @+0x18） | 桶内对象 |
| 桶 | 0x20 B（names）/ 0x28 B（paths） | 两张名字表 |
| json 断言 | 右值静态 "json" @0x142aafd10 | reload 只接受 json |
| 借用冲突 panic | 0x142ab57c0 | FrozenMap 借用 unwrap |

### 模块间调用

add/get_mut/remove/reload ↔ FrozenMap；消费方：`systems::backend::meshwatch`（0x14085b4b0）与 `vao_update::rebuild_vaos`（0x140864bf0，遍历 modified 重建 VAO）。`load_json_as_mesh` 真实实现**已在 utils 深挖闭合**（0x140b0eeb0 族 + `read_attribute_from_json` 0x14083e210 + `Mesh::set_attribute` 0x140a35240，全流程见 `utils/ALGORITHMS.md` §1，证据 `utils/decomp/decomp-0x140a35240.txt`）——本模块不再列为 U。

---

## 4. shader_library.rs（6 业务函数）

### 数据结构（总 0xe0 B，O）

raster_shaders FrozenMap@+0x00（Box 0xd8）、compute_shaders FrozenMap@+0x40（Box 0xa0）、
raster_names RwLock<HashMap<String,Handle>>@+0x80（桶 0x18）、compute_names RwLock<HashMap<u64,Handle,NoHash>>@+0xb8（桶 0x10，键直接当哈希）。

### 注册算法

- `add_raster(program: *RasterShaderProgram(0xd8 B), force_new: bool)`：写锁 → `!force_new && items!=0` 才查重 → 命中则 `FrozenMap::get(existing).unwrap()` **逐字段比较 desc**（+0x08/0x10 path；+0x20/0x28 Vec<(String,String)> 元素 0x30 B；+0x30/0x38/0x40 Option<String> niche 判别）→ 全等：放锁、drop 新程序、**返回已有 handle（幂等复用）**；不等：panic "AssetShaderLibrary: the shader name {} is already taken"（pieces 0x142ab10d0，:62:21）。未命中：HANDLE_COUNT++ → raster_names.insert → Box(0xd8) → FrozenMap::insert（Err 断言 :75:9）。
- `add_compute(program: *ComputeShaderProgram(0xa0 B), hash: u64)`：写锁 → 键存在 → panic（同一 pieces，参数 = ComputeShaderSourceHash Debug，:89:13）——**只比键不比内容（compute 无复用路径）**；否则 handle → compute_names.insert（NoHash）→ Box → insert（断言 :100:9）。
- 查询：`get_compute_by_name` = 读锁查表 → **放锁** → FrozenMap::get（缺失 None，不 unwrap；ComputePass::new 复用缓存）；`get_raster_handle_by_name` = hash_one + h2 探测；`debug_raster_get_name` = 读锁整表线性扫描。

### 关键常量

| 常量 | 值 | 含义 |
|---|---|---|
| 全局 handle | 0x143634cd0 | 与 mesh/vao/ssbo/placement 共用 |
| 桶 | 0x18 B（raster_names）/ 0x10 B（compute_names） | 名字表 |
| force_new | true 跳过查重直接注册 | 语义名 I、行为 O |

---

## 5. color_library.rs（5 业务函数）

### 数据结构

`ColorLibrarySsbo` = `[[f32;4]; 37]` 0x250 B（4 字节对齐非 SIMD Vec4）；默认 37 槽品红 (1,0,1,1)（xmmword 0x142a9be30）。
`ColorFromJson` 0x28 B：{name String@0, color [f32;3]@0x18}（serde 字段 name/color，color = ArrayVisitor<[f32;3]>，其余 IgnoredAny）。

### 槽名→下标解析算法（deserialize_from 0x14093e510，5102 B）

按名字长度分派：`len-4 <= 0xf` 走跳表，各长度分支逐项比较（4 B `cmp dword`、8 B `cmp qword`、17/19 B `pcmpeqb`、6 串共用字符串池 bcmp @0x142ab2182）→ 命中写 `slot = (c0,c1,c2,1.0)`（mov dword +0x3f800000）→ **边遍历边写无事务** → 未知名 panic "Init Color Library failed: {} entry is missing from ColorLibrarySSBO"（pieces 0x142ab2208，行 66）。槽表 37 项见 A3_color（temp_color_1..3 → night_tint，序号 0..36）。

### update_color_library 上传算法（0x14093b530）

1. 栈上 37 槽品红 → `deserialize_from_path<&str>("assets/colors.json")`（无存在性检查，缺文件 panic）。
2. 主题目录表（0x142ab22b8，5×{ptr,len}：summer(6)/olden(5)/flowery(7)/autumn(6)/winter(6)，本批 rne-read 复核）→ `Path::join("assets/glade").join(theme).join("colors.json")` → fs::stat 成功才再加载（**主题可选、基础必须**）。
3. `FrozenMap::get(&SsboId)` 键立即数 **0xef2a68c12cde3f0d = xxh3_64("ColorLibrary")** → 缺失 unwrap（行 156）→ assert name（len 0x3e）→ vtable+0x18 as_any → `byte_size >= 0x250` 守卫 → `update_buffer_bytes(buffer, data, [0..0x250], 0)` → Err unwrap panic（行 158）→ Ok → memcpy 0x250 回 CPU 资源（**成功才回写**）。

### 模块间调用

update_color_library ← configwatch 消费端（0x14085b1c0 直调）；→ AssetSsboLibrary（FrozenMap::get + vtable buffer）+ GraphicsDevice update_buffer_bytes。deserialize_from_path 双泛型实例（0x14155a170 &PathBuf / 0x14155a320 &str）两处 unwrap。

---

## 6. vao_library.rs（4 业务函数）

### 数据结构（总 0x60 B）

表 A HashMap<Handle<Vao>, VaoEntry>@+0x00（桶 0x50：key@0、Vao@8{bindings@8, index_buffer@0x20}、index_count@0x48）；表 B HashMap<Handle<Mesh>, Handle<Vao>>@+0x30（桶 16 B）。

### 算法

- `add(device, mesh_lib, mesh_handle) -> Result<Handle<Vao>, anyhow>`：①mesh_to_vao 命中 → Ok(已有)（幂等）→ ②**HANDLE_COUNT++（先于校验）** → ③mesh_lib.get 缺失 → anyhow "Mesh is not in the  mesh library"（**两个空格**，0x142aae830）→ ④indices 空 → "Mesh is invalid"（0x142aae800）→ ⑤build_vao（0x140909690，Err 透传）→ ⑥双表登记。
- `rebuild_vao(mesh) -> Option<anyhow::Error>`：expect "Mesh is not in the mesh library"（单空格，0x142aae840，**与 add 文案是两个串**）→ indices 空 → **静默 Ok** → mesh_to_vao 未命中 → "The requested mesh doesn't exist"（0x142aae898）→ vaos 未命中 → "VAO handle is invalid"（0x142aae8c0）→ build_vao → **drop 旧 bindings + 旧索引缓冲** → 写 index_count = indices.len()。
- `remove(mesh)`：mesh_to_vao.remove_entry → vaos.remove_entry + drop Vao。
- 消费端 `rebuild_vaos`（0x140864bf0）：遍历 mesh_lib.modified → 仅 has_vao 的处理（否则 warn）→ 失败 handle **写回 modified**（下帧重试）。

### 关键常量

| 常量 | 值 | 含义 |
|---|---|---|
| build_vao | 0x140909690 | render::vao::build_vao（wp2 合同） |
| VaoEntry | 0x50 B（+index_count@0x48） | 桶条目 |
| 全局 handle | 0x143634cd0 | 与 mesh/shader/ssbo/placement 共用 |

---

## 7. placement_data_library.rs（3 业务函数）

### 数据结构（总 0x60 B）

表 A HashMap<Handle<PlacementDataSsbo>, (TypedStorageBuffer<PlacementDataSsbo>, usize)>@+0x00（条目 0x38 B）+ RandomState@0x20；表 B HashMap<PathBuf, Handle>@+0x30（条目 0x28 B）+ RandomState@0x50。

### load<PathBuf> 算法（0x1409365d0）

①表 B 非空才查重，命中 → panic "AssetPlacementDataLibrary: the placement data from {path} has already been loaded"（pieces 0x142ab0fd0，行 93 列 13）→ ②`read_to_string` 失败 → panic "Couldn't find  {path}"（**两个空格**，0x142ab1018，行 100 列 63）→ ③`serde_json::from_str::<Vec<PlacementDataSsbo>>` Err unwrap（行 101）→ ④`TypedStorageBuffer::new(device, Some(&data))` → ⑤HANDLE_COUNT++ → ⑥A.insert(handle, (buf,len)) → B.insert(path, handle) → 返回 (Vec, handle)。

### PlacementDataSsbo JSON schema（O，round3 闭合，tg_ca1 反编译族）

`serde_json::from_str` 反序列化链（tg_ca1）：`FUN_141405cf0`（from_str 入口）→ `FUN_1414106e0`（Vec 包装）→ `FUN_140967640`（逐元素收集，stride **0x30**）→ `FUN_14142f860`（**visit_map 大函数，28.5KB**：对象键 dispatch，调 `FUN_140b3b960`=__Field visit_str 一次、`FUN_140c90df0`=pack_2x16f_uint 两次）→ 各字段值解析器。

**JSON 字段（__Field visit_str 0x140b3b960 长度分派 + FIELDS 串表 0x142c5cd48/0x142c5d028..058，O）**，tag = 声明序：

| tag | JSON key（长度） | 类型 | 解析器 | 必需 | 结构偏移 |
|---|---|---|---|---|---|
| 0 | `position`（8） | `[f32; 3]` | FUN_141413820 | **必需** | +0x00 |
| 1 | `scale`（5） | `f32` | FUN_14140cfd0 | **必需** | +0x0c |
| 2 | `rotation_degrees`（16，别名 `rotation_angle`） | `f32` | FUN_14140cfd0 | **必需** | +0x10 |
| 3 | `id`（2） | `i32/u32` | FUN_14140d270 | **必需** | +0x14 |
| 4 | `custom0`（7） | `u32`（pack_2x16f_uint 产物） | FUN_141412c00 | 可选（缺省 0） | +0x18 |
| 5 | `custom1`（7） | `u32`（pack_2x16f_uint 产物） | FUN_141412c00 | 可选（缺省 0） | +0x1c |
| 6 | `bend_vector`（11） | `[f32; 2]` | FUN_1414132b0（8B 数组） | 可选（缺省 0） | +0x20 |
| 7 | `pad0`（4） | `u32` | FUN_141412c00 | 可选（缺省 0） | +0x28 |
| — | 未知键 | — | FUN_141409100（跳过） | — | — |

- **主名 = `rotation_degrees`**（FIELDS 串表与缺失字段错误用 16 字符名；`rotation_angle` 是别名——两者都映射 tag 2）。reconstruction 现字段名 `angle` 与 JSON key 不符。
- **元素尺寸 0x30 = 48 B**（三处独立证实：FUN_1416164c0 缓冲创建 `param_4 * 0x30`、FUN_141405cf0 Vec drop `len * 0x30`、FUN_140967640 逐元素 stride 0x30）。reconstruction `{position, scale, angle, _gap}` 仅 20 B，**缺 id/custom0/custom1/bend_vector/pad0 五个字段**。
- **缺失/重复字段错误**（visit_map 内 0x42c5cd48..058 串表，O）：position/scale/rotation_degrees/id 缺失各自 panic（字段名长 8/5/16/2）；custom0/custom1/bend_vector/pad0 重复 panic（长 7/7/11/4）。
- `default_anim_t`（SOURCE_MAP 行 43）**不在 __Field 枚举内**（8 字段无 anim_t）→ 属 placement_data.rs 中另一类型（非 PlacementDataSsbo）；+0x2c 的 4 B 未映射（疑 padding/anim_t，I）。
- 真实资产核对（`assets/glade/{theme}/trees/*.json`，O）：`[{"position":[f32;3],"id":u32,"scale":f32,"rotation_angle":f32(度),"pad0":0,"pad1":0}, …]`——**rotation_angle 值为度**（如 112.97）；`pad1` 不在字段表（未知键被跳过）。
- **度→弧度转换**：JSON `rotation_angle` 值为**度**（真实资产，O）；`to_affine3a`（0x140ad5020，机器码复核）读 +0x10 值 × `_UNK_142925870`（**=0.5**，mulss [rip+0x01e50803]→0x142925870）后 sin/cos，**按弧度解释**（O）。故 SSBO 存储前必有度→弧度转换：解析器（FUN_14140cfd0）为普通 f32 无转换；.rdata float 常量池存在 **π/180 = 0x3C8EFA35 @ 0x142aab4a4**（rne-read 证实）——转换疑在 visit_map 旋转分支的 SIMD mulss（Ghidra 反编译扁平化未显式），**I 待集成裁决**（若 SSBO 存度则 to_affine3a 为原版怪癖）。

---

## 8. audio_trigger_library.rs（2 业务函数）

### 数据结构

`AudioTriggerLibrary` = `[AudioTriggerCollection; 1]` 0x20 B（get_triggers 断言 `kind != COUNT`，COUNT==1，行 85 列 9）；`AudioTriggerCollection` = slotmap::SlotMap<AudioTriggerHandle, Aabb3>（slots Vec@0 cap-first、num_elems u32@0x1c；槽 0x1c B = 24 B Aabb3 + 4 B version）；`AudioTriggerHandle` = u32 恒等（impl$3::from 0x140833250 = mov eax,ecx，与 DecoratorRank::new ICF）。

### 算法

- `boxes()`（0x14157ce40，67 B）：返回 `slotmap::basic::Iter` 4 qword `{ptr+(len?0x1c:0), ptr+len*0x1c, (len!=0), num_elems}`——`slots.iter().enumerate()` 跳过哨兵槽 0、起始下标 1、剩余数 = num_elems。
- 消费端 `trigger_audio_system`（0x1409eafa0，PDB 名 ICF 折叠 handplaced_tree_animation）：逐槽 `[slot+0x18] & 1` 判占用（version 奇偶，**不是 flags**）→ DecoratorRank::new(idx) 取键 → 前 0x18 B 喂 `Aabb3::intersect_segment`。
- add/update/remove/clear 内联于 prefab 系统（无独立符号，I）。

---

## 9. 三个 watch（configwatch / meshwatch / texture_watch）

### 数据结构对比（O）

| 结构 | 布局 | 队列语义 |
|---|---|---|
| `ConfigWatch` | 0x38 B：Option<GenericHotwatch>@0（首 dword==3 = None）、Arc<Mutex<Option<String>>>@0x30（ArcInner 0x30：强/弱=1、futex@0x10、poison@0x11、**单槽 Option<String>@0x18**） | **单槽，新事件覆盖旧事件** |
| `MeshWatch` | 0x40 B：Arc<Mutex<HashSet<PathBuf>>>@0（ArcInner 0x48：HashSet@0x18 起）、Mutex<()>@8（watch_lock）、Option<GenericHotwatch>@0x10 | **集合累积**（消费前不合并） |
| `TextureWatch` | 0x10 B：Option<GenericHotwatch>@0、Arc<Mutex<HashSet<RelativePathBuf>>>@8（ArcInner 0x48） | **集合累积（存名字非路径）** |

### 事件判别（O，tg_m1 反编译 0x14091e430 / 0x14091e7a0 / 0x14091edb0）

DebouncedEvent 变体 = `tag ^ 0x8000000000000000`（i64::MIN 基址编码）：

| watch | 处理变体 | 掩码/判据 |
|---|---|---|
| configwatch | 仅 Write（3） | `tag == 0x8000000000000003` |
| meshwatch | Create(2) / Write(3) | 两分支 == 2、== 3 |
| texture_watch | Create(2) / Write(3) / Rename(6) | `变体 < 7 && (0x4c >> v) & 1`（Rename 有专门分支） |

**共性：回调写入闭包捕获的注册对象（configwatch=注册路径串、meshwatch=注册路径、texture_watch=注册名），不是事件路径；回调不读文件。** 处理流程：日志（级别 3）→ Mutex lock（poison → unwrap panic）→ 写队列。

### 消费端（每帧 drain，O）

| 消费端 | 地址 | 算法 |
|---|---|---|
| configwatch 系统 | 0x14085b1c0 | 锁同一 Mutex → Option<String> cap>=0 判 Some → 级别>2 时 log::info → **`update_color_library(settings, &AssetSsboLibrary, &Arc<GraphicsDevice>, &mut ColorLibrarySsbo)`**（FUN_14093b530；ResMut tick@+0x1c）→ 释放 String（dealloc(ptr,cap,1)）→ 置 None（cap=i64::MIN） |
| meshwatch 系统 | 0x14085b4b0 | 锁集合 → items!=0 且级别>2 → 日志 → **hashbrown SSE 遍历每桶 → `reload_mesh_src(&mut mesh_lib, path)`** → 第二遍释放全部键 + memset(ctrl,0xff) + 清 items = drain/clear（**一次消费全清**） |
| texture_watch 消费端 | 0x140aab370 | `systems::backend::texture_watch`（round3 闭合，w9 + tg_ca1 复核）：锁 changed 集合（ArcInner Mutex@+0x10）→ poison unwrap panic → 集合非空且级别>2 → log "Texture watch: detected changes: "（0x142add810，len 0x21）→ FUN_140b73a50（u16 位图载体）×2 遍历 hashbrown 组 → 逐名字 **`NameHash::compute`（FUN_140c9cd80）→ 查 texture_library `by_name` 表（@+0x30 hashbrown）**（FUN_140917860 校验）+ `func_0x00014003bb80`（1B ret no-op 空 drop 占位）→ 失败 → log "Error processing textures: "（0x142add878）；一次消费清空。FUN_140b73a50 身份（u16 位图载体细节）留待后续 worker |

### 关键常量

| 常量 | 值 | 含义 |
|---|---|---|
| 监视延迟 | 100 ms（Duration{0, 0x5f5e100}） | new_with_custom_delay（旧版 50 ms 全错） |
| 初始化失败 | expect "hotwatch failed to initialize" | 三个 watch 同文案 |
| 注册失败 | expect "failed to watch file!" | 三个 watch 同文案 |
| 事件序 | notify 4.x / 4.0.17 | 变体序号 I（掩码+变体数推出） |

### 模块间调用

ConfigWatch → color_library::update_color_library（0x14093b530）；MeshWatch → AssetMeshLibrary::reload_mesh_src（0x14092d8d0）→ vao_update::rebuild_vaos 链式；TextureWatch → `systems::backend::texture_watch`（0x140aab370，round3 闭合）→ texture_library by_name（NameHash 查表）。`use_watch` 门控：false 时全部静默 no-op。**热重载状态机**：注册（watch）→ 事件（回调写队列）→ 每帧系统 drain 消费 → 重载/上传 → 队列清空；事件在消费间隔内累积/覆盖，消费是一次性全部处理。

---

## 10. mod.rs（Handle 核心类型）

- `HandleId::new()` = `lock xadd [0x143634cd0], 1`（全局原子，O）；`Handle<T>` = {id: HandleId} 8 B、derive(Component)、is_valid 谓词 I；`Asset` = {name: String}。
- `impl$21::register_required_components<render::vao::Vao>`（0x140555a90）= bevy derive(Component) 展开（含 Handle<Vao> TypeId 0x93c2ea0b5dbce370）——RENDER_GAP "需补业务"判断不成立（纠错）。
- fmt 胶水：Handle/HandleId Debug（串 "HandleId" @0x142c2f488，位置 crates/country-core/src/asset_libraries/mod.rs）。

---

## 11. 模块间调用总图（O）

```
hotwatch (notify)                    bevy 每帧系统
   │ 事件回调（写注册对象）                │ drain
   ▼                                    ▼
ConfigWatch ──pending──► systems::backend::configwatch ──► update_color_library ──► AssetSsboLibrary.get + GraphicsDevice.update_buffer_bytes
MeshWatch ────set──────► systems::backend::meshwatch ─────► AssetMeshLibrary.reload_mesh_src ──► load_json_as_mesh ──► vao_update.rebuild_vaos ──► AssetVAOLibrary.rebuild_vao
TextureWatch ─set──────► systems::backend::texture_watch 0x140aab370 ──► texture_library by_name（NameHash 查表）
AssetSsboLibrary ◄── get_rg_buffer/read/write_access_buffer（pass_resource_access A4，81 型）
AssetTextureLibrary ◄── image_name_to_index / bindless_textures_dset（渲染面）
AssetShaderLibrary ◄── ComputePass::new（get_compute_by_name 缓存复用）
HANDLE_COUNT 0x143634cd0 ◄── mesh/vao/shader/ssbo/placement（lock xadd 共用）
```

## 12. 本批纠错/细化汇总

1. **FrozenMap::insert 语义**（round3 已修正）：键存在 → 丢弃新值、返回 Err(1)、表不变；本批确认 ssbo add_new 依赖此语义（insert-first，panic 在后，非前置 get）。
2. **add_new 双 panic 文案**机器证实（0x142c2d628/0x142c2d568/短名 0x142c2d840）。
3. **BufferState::new 七次写**机器证实（0x14091eb30 36 B；name {ptr@0x10,len@0x18}，与 add_new 栈组装一致）。
4. **Option<DescriptorIndex> 槽 = 8 B {tag@0, value@4}**（assign_auto_resize 0x1412f8f60 机器码）——非 niche 布局。
5. **handle_to_bindless_index 分配段无独立锁指令**（Ghidra 如实）；A3 "取写锁"表述降级为 I（std 守卫展开）。
6. **theme 表 5×{ptr,len}** rne-read 复核（summer/olden/flowery/autumn/winter，长度 6/5/7/6/6）。
7. **构建环境**：/home/h3/code/pouncelight/country-slice-private（Linux 交叉编译 Windows）+ rustc 05f9846f893b09a1be1fc8560e33fc3c815cfecb。
8. **文案细节**："Mesh is not in the  mesh library" 两个空格 vs rebuild 单空格；"Couldn't find  " 两个空格——reconstruction 落码须原样保留。
9. **vao add 先取号后校验**（失败也消耗 id）机器证实。
10. **meshwatch 消费端** = hashbrown 遍历 + reload_mesh_src + drain/clear 两遍算法（0x14085b4b0 全流程）。

## 13. 遗留 U（不臆造）

- compute shader 的 RasterPipelineDesc 第二字段 Option<String>（I，另一工作包）。
- `FUN_140b73a50` 身份（texture_watch 消费端的 u16 位图载体，2 调用点参数全同）——留待后续 worker（见 §9 消费端）。
- PlacementDataSsbo +0x2c 的 4 B 字段（疑 padding/anim_t，I）；度→弧度是否在 SSBO 存储前转换（I 待集成裁决，见 §7）。

## 14. round3 裁决项（待集成 owner）

1. **get_rg_buffer panic 参数槽序**（0x1402b31a0，tg_ca1 复核）：机器码 `format_args!` 参数序 = **[全名(0x1429fcd5c/0x49=73), 短名(0x1429ae158→0x1429ae138/28)]**，渲染 `"Buffer {full} (type: {short}) is not available…"`；reconstruction `ssbo_library.rs:21` 注释与 `get_rg_buffer` 实现（T::NAME 短名在前）渲染 `"Buffer {short} (type: {full})…"`——**槽序相反**。行为等价（消息内容一致）但参数顺序不同；待集成裁决是否对齐机器码参数序。
2. **FrozenMap::insert 返回契约**（0x140969bf0）：机器码 = 键已存在 → **丢弃新值 Box + 表不变 + 返回 Err(1)**，调用方再查表（ssbo add_new 实测）；reconstruction `frozen_map.rs:5-6` 注释/实现返回 `Err(&V)`（已有值引用，调用方直接拼 panic 消息），新值靠参数作用域析构。**行为等价但结构不同**（drop 时机/调用方是否重查）。另：机器码借用字节 @+8 的 `try_borrow().unwrap()` 冲突 panic（get_mut/remove 与遍历互斥）reconstruction 未建模（占位 RwLock 无借用字节）——需补或登记降级。**只登记，不改源码**。
