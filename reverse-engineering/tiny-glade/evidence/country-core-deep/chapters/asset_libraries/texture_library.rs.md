# asset_libraries/texture_library.rs

> 文件作用：**纹理资产库**——`AssetTextureLibrary`（0xc8 B）：textures Vec + 名字表 `HashMap<NameHash, NamedTexture,
> NoHashHasher>` + presets 表 + bindless 描述符集（`bindless_textures` dset + `RwLock<DescriptorMap>` pending/indices/next）。
> 支持 add/create_name_mapping/preset 生命周期（create/clear/import/export）+ bindless 索引分配与 dset 刷新。证据：
> `evidence/country-core-audit/A3_texture.md`（O 全量）。

函数总计 27：业务 13 / 内联无独立符号 6 / 派生胶水 8

## 布局表（O，A3_texture，总 0xc8）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `textures: Vec<NamedTexture>`（stride 0x38） | new 0x14093bb10 |
| +0x18 | `bindless_textures_dset`（0x18 B 持久描述符集） | new / bindless_textures_dset |
| +0x30 | `by_name: HashMap<NameHash, NamedTexture, NoHashHasher>`（桶 0x18） | new / add |
| +0x50 | `presets: HashMap<TexturePreset, PresetTextureState, RandomState>`（桶 0x68） | new / create_preset_textures |
| +0x70 | RandomState（k0,k1） | new |
| +0x80 | `std::sync::RwLock<DescriptorMap>`（futex +0x80、poison +0x88、数据 +0x90：`pending Vec<AssetIndex>` +0x90、`indices Vec<Option<DescriptorIndex>>` +0xa8、`next u32` +0xc0） | new / handle_to_bindless_index |
| — | `NamedTexture { handle, is_virtual }`（PDB 名 + add 断言文案 O）；`PresetTextureState { texture, rg_handle: Option }`（桶内 Texture @+8、handle 0x24 B @+0x40、None 标签 = 第 0x22 字节==7） | add / import_texture_presets_to_rg |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `AssetTextureLibrary::new` | 0x14093bb10 | 63 | 猜测：构造；证实：只建 bindless dset | **O**：只调 `create_persistent_descriptor_set("bindless_textures", {len 0x11, u32 1, u8 4}, 1)`，其余字段空；presets RandomState 取线程局部 KEYS。旧版把 bindless 状态建成 parking_lot + dirty 标志（无此物） | A3_texture.md |
| `AssetTextureLibrary::create_preset_textures` | 0x14093bd80 | 80 | 猜测：预置纹理生成；证实：26 预设逐个建纹理 | **O** + fold 0x141474b90：对 `enum_iterator::All<TexturePreset>`（跳转表 0x142c65c18，值 0..=0x19 共 26 个）逐个 `TextureDesc::from(preset)`（0xa59890，**属 texture_presets.rs**）→ `create_texture(name=NameHash::debug_name 常量)` → `PresetTextureState{texture, rg_handle:None}`；收集进新 HashMap 整体替换 presets。入口经 ResMut DerefMut（末尾 set_changed，I）。旧版空桩 | A3_texture.md + render-gap §4.1 |
| `AssetTextureLibrary::clear_preset_texture` | 0x14093bee0 | 105 | 猜测：清预置纹理；证实：全 0 字节 + mip 0 + update_texture | **O**：`presets[preset]`（缺失 → `no entry found for key`）；`size = vk_format_info(format).bytes_per_texel * (w*h*d)`；全 0 字节、mip 0、extent=纹理 extent、两标量实参 1 与 0x22 调 `update_texture`；Err→unwrap | A3_texture.md |
| `AssetTextureLibrary::add` | 0x14093c200 | 109 | 猜测：注册纹理；证实：**入参 `(name: Option<String>, Texture(0x38))` 共 0x50 B**，返回 `AssetIndex(旧 len)` | **O**：Some → 先 by_name 查重，重名 → `panic!("AssetTextureLibrary: the name {} is already taken")`（pieces 0x142ab1b60）；push 到 textures；`trace!("Loaded texture {:?} as {:?}")`（target `country_core::asset_libraries::texture_library`，级别 5，**入表之前**打印）；Some → `assert!(by_name.insert(..{handle, is_virtual:false}).is_none())`。旧版 panic 文案/克隆两份/“纹理存两份”全错 | A3_texture.md |
| `AssetTextureLibrary::create_name_mapping` | 0x14093c720 | 153 | 猜测：名字→现有纹理映射；证实：target 查找 + 别名登记 | **O**：查 `target`（缺失 → `Was asked to map {name} to {target}, but the latter does not exist as a named texture`），`insert(hash(name), {target.handle, is_virtual:true})`；旧值存在且 `!is_virtual` → `Onoz, {name} already exists, and is a real texture. …`——**panic 在 insert 之后**（真纹理表项已被覆盖）；旧值是别名则静默覆盖 | A3_texture.md |
| `AssetTextureLibrary::get_handle_and_texture_by_name` | 0x14093c970 | 190 | 猜测：名字取纹理；证实：**参数是已算好的 NameHash**，返回 `Option<(AssetIndex, &Texture)>` | **O**：命中但下标 ≥ len → None（`cmp; cmovb`）。旧版取 &str 且返回新造 Handle 错 | A3_texture.md |
| `AssetTextureLibrary::get_by_name` | 0x14093ca90 | 198 | 猜测：名字取引用；证实：NameHash 查询 + 两条 panic 文案 | **O**：参数 NameHash；未命中 → `No such texture: {hash}. Available: {Vec<&NameHash>}`；命中但越界 → `Texture not loaded: {hash}`（hash 的 fmt 是 Debug 还是 Display：U）。旧版返回 Option 错 | A3_texture.md |
| `AssetTextureLibrary::import_texture_presets_to_rg` | 0x14093cd30 | 224 | 猜测：预置导入 rg；证实：逐个 import_unowned | **O**：遍历 presets；每个 `assert!(preset.rg_handle.is_none())`（"assertion failed: preset.rg_handle.is_none()"），`rg_handle = Some(import_unowned(&texture, rg, 0x22))` | A3_texture.md |
| `AssetTextureLibrary::export_preset_textures_from_rg` | 0x14093ce30 | 234 | 猜测：预置导出；证实：rg_handle.take + export | **O**：遍历；`rg_handle.take().unwrap()`（None→`Option::unwrap` panic，且已先置 None）后 `export(handle, rg, 0x22)` | A3_texture.md |
| `AssetTextureLibrary::get_rg_texture` | 0x14093cf30 | 243 | 猜测：取 rg 纹理；证实：纹理表查找 + Option unwrap | **O**：缺失 → `no entry found for key`；None → `Option::unwrap` panic；返回 0x24 B 句柄拷贝 | A3_texture.md |
| `AssetTextureLibrary::bindless_textures_dset` | 0x14093d040 | 255 | 猜测：刷新 dset；证实：drain pending + bump 临时区 + 更新描述符集 | **O**：写锁；pending 非空才 `Bump::new` + drain，逐项 closure（0x140a96740）：`indices[a]`（`expect("asset descriptor index")`）、`textures[a]`、`get_vk_image_view(tex, 0).unwrap()`、bump 内 `{0,0,view,5}`；调 `update_persistent_descriptor_set(&dset, iter)`；返回 `&self.dset`（+0x18）。PDB closure 父函数名 `update_bindless_texture_table` 已内联（I） | A3_texture.md |
| `AssetTextureLibrary::handle_to_bindless_index` | 0x14093d310 | 300 | 猜测：分配 bindless 槽；证实：`&self`，读写锁两段式 | **O**：读锁查 `indices[asset]` Some 直接返回；否则写锁**不复查**；`idx = next; next += 1`（先自增），`assert!(idx < 0x1fe0)`（文案 0x142ab1fa0 全文 = REQUIRED_BINDLESS_DESCRIPTOR_COUNT − RESERVED_DESCRIPTOR_COUNT），`pending.push`，`assign_auto_resize(indices, asset, Some(idx))`（0x1412f8f60）。旧版上限 0x1fdf、分配规则与数据结构都不对 | A3_texture.md |
| `impl$2::image_name_to_index` | 0x14093d690 | 349 | 猜测：名字→bindless 下标；证实：NameHash::compute + handle_to_bindless_index | **O**：`NameHash::compute(name)`，未命中 → `No such texture: {name}`；命中 → `handle_to_bindless_index(entry.handle)`，不看 is_virtual | A3_texture.md |

## 内联/无独立符号函数

- `DescriptorIndex::allocate`（行 35）— **I**：u32 bindless 槽分配（O 名；体并入 handle_to_bindless_index）
- `AssetTextureLibrary::add_raw`（行 145）— **I**：add 的原始数据变体（无独立符号）
- `AssetTextureLibrary::get_mut_by_name`（行 215）— **I**：名字可变查询
- `AssetTextureLibrary::get_handle_by_name`（行 220）— **I**：名字取句柄
- `AssetTextureLibrary::update_bindless_texture_table`（行 263）— **I**：已内联进 `bindless_textures_dset`（PDB 父名 O）
- `clear_texture`（行 359）— **I**：纹理清理助手

## 派生胶水（8 项：closure、default 等）

- closure 族（add 查重闭包、bindless dset 闭包 0x140a96740 已 O 描述）；default = derive。
- 占位标注：`NoHashHasher`、`TextureLibraryDevice/TextureRenderGraph` trait、`format_bytes_per_texel`、`hotwatch_contract`、`RelativePathBuf`；`Resource` 手写 impl（A3_texture 注）。

## 纠错/差异（A3_texture + render-gap，O）

- 旧版 bindless 状态建模为 parking_lot + dirty 标志——原版是 `std::sync::RwLock<DescriptorMap>` + pending/indices/next。
- 旧版 `handle_to_bindless_index` 上限 0x1fdf（实为 `idx < 0x1fe0` 断言）与分配规则错误。
- 旧版 `create_name_mapping` 参数当 `(name, Texture)`、文案臆造——原版是别名映射。
- **render-gap 纠错**：0xa59890 是 `From<TexturePreset> for TextureDesc`（非 `From<u8> for TexturePreset`）；预设 11/12 为普通 storage 非 depth；`create_preset_textures` 落码时应替换 `desc_of` 闭包参数为真实 impl（预表 + NameHash@+0x20，0x30 B desc）。
- `TexturePreset` 26 值（0..=0x19）由 create_preset_textures 跳转表证实（O）。

## 算法要点（本批深挖 2026-10-15，decomp: asset-libraries/decomp/）

证据：tg_m1 反编译 `decomp-0x14093c200`（add）、`decomp-0x14093c720`（create_name_mapping）、`decomp-0x14093d310`（handle_to_bindless_index）、`decomp-0x14093d040`（bindless_textures_dset）、`decomp-0x14093bd80`（create_preset_textures）、`decomp-0x14093d690`（image_name_to_index）+ 机器码手解 0x1412f8f60（assign_auto_resize）。

### 双表一致性维护（textures Vec + by_name HashMap）

- `add(Option<String>, Texture(0x38)) -> AssetIndex`：①Some → by_name 查重（NoHash 探测），重名 → `panic!("AssetTextureLibrary: the name {} is already taken")`（pieces 0x142ab1b60/0x142ab1b80）→ ②`textures.push` → ③`trace!("Loaded texture {:?} as {:?}")`（target `country_core::asset_libraries::texture_library`，级别 5，**入 by_name 之前**，None 名也打印）→ ④Some → `assert!(by_name.insert(hash(name), {handle, is_virtual:false}).is_none())`（0x142ab1c16，行 122）。返回 `AssetIndex(旧 len)`——**下标先于别名表**。
- `create_name_mapping(name, target)`：查 target（缺失 → panic "Was asked to map {name} to {target}, but the latter does not exist…"）→ `by_name.insert(hash(name), {target.handle, is_virtual:true})`；**insert 在先**——旧值是真纹理（!is_virtual）→ panic "Onoz, {name} already exists, and is a real texture…"（真纹理表项已被覆盖，无回滚）；旧值是别名 → 静默覆盖。
- 查询链：`get_handle_and_texture_by_name(hash)` = 命中且 `index < len` 才返回（`cmp; cmovb`）；`get_by_name(hash)` = 未命中 → "No such texture: {hash}. Available: {Vec<&NameHash>}"；命中越界 → "Texture not loaded: {hash}"。

### bindless 索引分配（DescriptorMap @+0x90）

`handle_to_bindless_index(&self, AssetIndex) -> DescriptorIndex`（decomp-0x14093d310）：

1. 读锁（std RwLock futex @+0x80、poison @+0x88）。
2. `indices` 表（ptr@+0xb0、len@+0xb8）：`asset < len && slot.tag != 0` → 命中：取 `slot.value`（@+4），放锁返回。**Option<DescriptorIndex> 槽 = 8 B {tag:u32@0, value:u32@4}，None={0,0}**（assign_auto_resize 0x1412f8f60 机器码：`mov [slot],r8d=1; mov [slot+4],r9d=idx` 证实）。
3. 未命中：放读锁 → **不复查** → `idx = next; next += 1`（@+0xc0，先自增）→ `assert!(idx < 0x1fe0)`（panic 串 "assertion failed: descriptor_index.0 < GraphicsDevice::REQUIRED_BINDLESS_DESCRIPTOR_COUNT as u32 - RESERVED_DESCRIPTOR_COUNT as u32" @0x142ab1fa0，行 0x8f=143）→ `pending.push(asset)`（cap-first Vec {cap@+0x90, ptr@+0x98, len@+0xa0}，grow FUN_140562870）→ `assign_auto_resize(indices, asset, Some, idx)`（0x1412f8f60：asset==len 直接 push；<len 覆盖；>len 先 grow 再全 0 补齐到 asset 前）→ 返回 idx。
   - **注**：Ghidra 反编译中分配段未呈现独立写锁 cmpxchg（A3 dis.py 记为"取写锁"）；函数尾的 `lock xadd +0xc0000001` 与守卫构造（FUN_1428d86b0 反相标志）为 std RwLock 写守卫释放的展开——锁协议归属 std，分配段并发安全依赖 bevy 主线程单线程调用（`&self`）。按机器码如实记录：**next/pending/indices 的改写无独立锁指令**。
4. `bindless_textures_dset(&self) -> &DescriptorSet`（decomp-0x14093d040）：写锁 → `pending.len==0` → 直接返回 &dset(+0x18)；否则 ①快照 indices{ptr,len} → ②Bump::new → ③**drain pending**（len 置 0，Copy 元素泄漏不析构）→ 逐项闭包（0x140a96740 / FUN_1416a2320）：`indices[a].expect("asset descriptor index")` → `textures[a]` → `get_vk_image_view(tex,0).unwrap()` → bump 内 `{0,0,view,5}` 描述符 → `update_persistent_descriptor_set(&dset@+0x18, iter)` → ④Bump 块链析构 → 放锁返回。

### 关键常量表

| 常量 | 值 | 含义 |
|---|---|---|
| bindless 上限 | 0x1fe0（8160） | REQUIRED_BINDLESS_DESCRIPTOR_COUNT − RESERVED_DESCRIPTOR_COUNT |
| 导入绑定 | 0x22 | import_unowned/export 实参（ssbo 系 0x37） |
| 描述符写入 | {0,0,view,5} | bump 内 VkDescriptorImageInfo 形状（5 = 采样器标志） |
| 预设数 | 0x19+1 = 26 | enum_iterator::All<TexturePreset>（跳转表 0x142c65c18） |
| dset 建参 | {len 0x11, u32 1, u8 4} | create_persistent_descriptor_set("bindless_textures", …, 1) |
| NamedTexture | 0x38 B（handle + is_virtual） | by_name 桶 0x18 B |
| PresetTextureState | 桶 0x68 B（Texture@+8、rg_handle 0x24 B @+0x40、None 标签 = 0x22 字节==7） | presets 表 |

### 模块间调用

- `add`/`create_name_mapping` → by_name HashMap（NoHash）；`image_name_to_index` → `NameHash::compute`（0x140c9cd80）→ handle_to_bindless_index；`bindless_textures_dset` → rhapsody `get_vk_image_view` + `update_persistent_descriptor_set`；`import_texture_presets_to_rg` → `rg.import_unowned(&texture, 0x22)`。
- 消费端：渲染面 `bindless_textures_dset()` 每帧取 dset；`image_name_to_index` 供名字→bindless 槽（着色器纹理绑定路径）。
