# A3 texture 组：texture_library.rs / texture_watch.rs 对照机器码核对

O = 机器码直接观察，I = 推断，U = 未知。工具：`dis.py` / `rdva.py`（版本 B）。测试台：`/tmp/h_texture`（43 个测试含 load_texture 的 9 个，全绿）。

## AssetTextureLibrary 布局（O，`new` 0x14093bb10 逐字段写入，0xc8 B）
`+0x00 textures: Vec<Texture>`（stride 0x38）；`+0x18 dset`（0x18 B）；`+0x30 by_name: HashMap<NameHash, NamedTexture, NoHash>`（桶 0x18）；
`+0x50 presets: HashMap<TexturePreset, PresetTextureState, RandomState>`（桶 0x68）；`+0x70` RandomState(k0,k1)；
`+0x80 std::sync::RwLock<DescriptorMap>`（futex 字 +0x80，poison +0x88，数据 +0x90：`pending Vec<AssetIndex>` +0x90、`indices Vec<Option<DescriptorIndex>>` +0xa8、`next u32` +0xc0）。
类型名来自 PDB：`AssetIndex`、`NamedTexture`、`PresetTextureState`、`DescriptorMap`、`DescriptorIndex`；`NamedTexture{handle, is_virtual}` 的字段名来自 `add` 里断言的源码文本（O）。

## 逐函数
- **new 0x14093bb10 (O)**：只调 `create_persistent_descriptor_set("bindless_textures", {len 0x11, u32 1, u8 4}, 1)`，其余字段空；presets 的 RandomState 取线程局部 KEYS（k0 自增）。旧版同样只建 dset，但把 bindless 状态建成 parking_lot + dirty 标志（无此物）。
- **add 0x14093c200 (O)**：`name: Option<String>`（cap==isize::MIN 为 None）+ `Texture`(0x38) 共 0x50 B。Some → 先 `by_name` 查重，重名 → `panic!("AssetTextureLibrary: the name {} is already taken")`（pieces 0x142ab1b60）；然后 push 到 textures，`trace!("Loaded texture {:?} as {:?}")`（target `country_core::asset_libraries::texture_library`，级别 5，None 名字也打印、在入表之前）；Some → `assert!(by_name.insert(..{handle,is_virtual:false}).is_none())`（文案 0x142ab1c16 全文入代码）。返回 `AssetIndex(旧 len)`。旧版：panic 文案、克隆整个 NamedTexture 进两处、“纹理存两份”全错。
- **create_name_mapping 0x14093c720 (O)**：查 `target`（缺失 → `Was asked to map {name} to {target}, but the latter does not exist as a named texture`），再 `insert(hash(name), {target.handle, is_virtual:true})`；旧值存在且 `!is_virtual` → `Onoz, {name} already exists, and is a real texture. Create a new (arbitrary) name, and map it to a real texture instead, like 'imaginary/pony' -> 'the_real_texture_asset'.`——**panic 在 insert 之后**（真纹理表项已被覆盖，有测试）；旧值是别名则静默覆盖。旧版把参数当 `(name, Texture)`、文案是臆造。
- **get_handle_and_texture_by_name 0x14093c970 (O)**：参数是**已算好的 NameHash**（rdx），返回 `Option<(AssetIndex,&Texture)>`；命中但下标 ≥ len → None（`cmp; cmovb`）。旧版取 &str 且返回新造 Handle。
- **get_by_name 0x14093ca90 (O)**：参数 NameHash；未命中 → `No such texture: {hash}. Available: {Vec<&NameHash>}`；命中但越界 → `Texture not loaded: {hash}`。hash 的格式化特征 `impl$7::fmt` 是 Debug 还是 Display：U（用 Debug）。旧版返回 Option。
- **create_preset_textures 0x14093bd80 + fold 0x141474b90 (O)**：对 `enum_iterator::All<TexturePreset>`（跳转表 0x142c65c18，值 0..=0x19 共 26 个）逐个 `TextureDesc::from(preset)`（0xa59890，**属 texture_presets.rs，未恢复，代码里以闭包参数代替**）→ `create_texture(name="(NameHash)", desc, TextureInitialData::None)`（名字是 `NameHash::debug_name` 0x140c9cdb0 的常量；Err→unwrap panic）→ `PresetTextureState{texture, rg_handle:None}`；收集进新 HashMap 后整体替换 `presets`。入口经 `ResMut` DerefMut（末尾 `[rcx]=[rsi+0x1c]` 即 set_changed，I）。旧版空桩。
- **clear_preset_texture 0x14093bee0 (O)**：`presets[preset]`（缺失 → `no entry found for key`）；`size = vk_format_info(format).bytes_per_texel * (w*h*d)`（w*h*d 按 u32 回绕再扩 u64）；全 0 字节、mip 0、extent=纹理 extent、两个标量实参 1 与 0x22 调 `update_texture`；Err→unwrap。Texture 内偏移：format +0x20，extent +0x24/+0x28/+0x2c（占位 Texture 无深度，取 1，U）。
- **import_texture_presets_to_rg 0x14093cd30 (O)**：遍历 presets；每个 `assert!(preset.rg_handle.is_none())`（"assertion failed: preset.rg_handle.is_none()"），`rg_handle = Some(import_unowned(&texture, rg, 0x22))`。桶内：Texture @+8，handle 0x24 B @+0x40，None 标签 = 第 0x22 字节==7。
- **export_preset_textures_from_rg 0x14093ce30 (O)**：遍历；`rg_handle.take().unwrap()`（None→`Option::unwrap` panic，且已先置 None）后 `export(handle, rg, 0x22)`。
- **get_rg_texture 0x14093cf30 (O)**：缺失 → `no entry found for key`；None → `Option::unwrap` panic；返回 0x24 B 句柄拷贝。
- **handle_to_bindless_index 0x14093d310 (O)**：`&self`；std RwLock（futex）。读锁（poison→unwrap）查 `indices[asset]` 为 Some 直接返回；否则放读锁、取写锁，**不复查**；`idx = next; next += 1`（先自增），`assert!(idx < 0x1fe0)`（文案 0x142ab1fa0 全文，源码为 REQUIRED_BINDLESS_DESCRIPTOR_COUNT − RESERVED_DESCRIPTOR_COUNT，仅差值是 O），`pending.push`，`assign_auto_resize(indices, asset, Some(idx))`（0x1412f8f60：=len push，<len 覆盖，>len 以全 0=None 补齐）。旧版上限 0x1fdf、分配规则与数据结构都不对。
- **bindless_textures_dset 0x14093d040 (O)**：写锁；`pending` 非空才 `Bump::new` + drain，逐项 closure（0x140a96740）：`indices[a]`（`expect("asset descriptor index")`）、`textures[a]`、`get_vk_image_view(tex, 0).unwrap()`、bump 内 `{0,0,view,5}`；调 `update_persistent_descriptor_set(&dset, iter)`；返回 `&self.dset`（+0x18）。PDB 里 closure 的父函数名是 `update_bindless_texture_table`，应已被内联进此函数（I）。
- **impl$2::image_name_to_index 0x14093d690 (O)**：`NameHash::compute(name)`，未命中 `No such texture: {name}`；命中 → `handle_to_bindless_index(entry.handle)`，不看 is_virtual。

## TextureWatch
- **new 0x1408bab30 (O)**：`use_watch` → `GenericHotwatch::new_with_custom_delay(secs 0, nanos 0x5f5e100)` = **100 ms**（旧版 50 ms 错），失败 `expect("hotwatch failed to initialize")`；`changed: Arc<Mutex<HashSet<RelativePathBuf>>>` 空（Arc 块 0x48 B）。
- **watch 0x1408bad70 (O)**：签名 `(&self, name: String, path: &Path)`（rdx=&String，r8:r9=&Path）。先 clone Arc；hotwatch None → drop Arc 与 name 返回；否则 `hotwatch.watch(path, 闭包{name, arc})`，失败 `expect("failed to watch file!")`。旧版把参数当 PathBuf 并在回调里读文件字节——原版不读文件。
- **closure 0x14091edb0 (O)**：`DebouncedEvent`（notify 4.0.17）仅 Create(2)/Write(3)/Rename(6)（掩码 0x4c）处理：Mutex 锁（poison→unwrap），`insert(RelativePathBuf(name.clone()))`；插入的是**监视时的名字**而不是事件路径。变体序号按 notify 4.0.17 源码顺序对应（I，由 `0x4c` 掩码 + 变体数 9 推出）。
- 消费端（读取并重载 changed 集合）不在三个过程里，U。

## 占位（仅代码内标注）
`NoHashHasher`、`TextureLibraryDevice`/`TextureRenderGraph` trait、`format_bytes_per_texel`、`hotwatch_contract`、`RelativePathBuf`；`bevy_ecs::resource::Resource` 用手写 impl（非 derive）。
