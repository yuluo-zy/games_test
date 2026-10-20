# country-core / asset_libraries 逐文件还原度复核（B 版，2026-10-19）

性质：只读审计。未修改任何源码、未重跑反编译、未执行 cargo。
范围：`D:/game/tiny-glade/crates/country-core/src/asset_libraries/**` 共 15 个 `.rs`。
对照：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/asset_libraries/**`（12 个文件，纯 PDB 元数据骨架）、
`D:/game/reverse-engineering/tiny-glade/evidence/audit-2026-10-19/fn_audit.json`、
`D:/game/tiny-glade/crates/country-core/recovery/STUBS-2026-10-19.md`（仅作线索，已发现过期项）。

---

## 0. 结论先行

- **没有 `todo!` / `unimplemented!`；生产路径没有空函数体。** 全目录仅两处“占位”字样都在 `#[cfg(test)]` 注释里（`audio_trigger_library.rs:104`、`ssbo_library.rs:585`）。
- `fn_audit.json` 的 **88 个 `no_trace` 全部不是真正缺失**：87 个是泛型单态化 / 闭包 / derive 生成，1 个（`mesh_library::active_widget_state`）是**同地址错归属**，属于 `systems::ui::resources::ui_state::UiState`。
- `ssbo_library.rs` 的 **78 个 `no_trace` 是 78 个具体 SSBO 类型的 `as_any`/`as_any_mut` 单态化**，由当前 4 个 blanket impl 覆盖；78 个类型标识在当前 workspace 全部存在，多数由宏生成。**不是缺失，也不是被内联掉，而是原版每个具体类型各生成一份，当前用泛型 impl 等价生成。**
- STUBS 列出的 6 个“完全没有迹象”函数，5 个已被内联实现吸收（`add_raw`、`update_bindless_texture_table`、`add_existing`、`get_dyn`、`watch_path`），**只有 `AssetTextureLibrary::get_mut_by_name` 仍无对应实现**。
- 真正需要关注的是**消费端接线**而不是库函数：`AssetPlacementDataLibrary::load/clear` 在生产代码中无调用者、`TextureWatch::watch` 从未注册任何纹理路径、`TransientMesh` 的生产者只落了调试可视化、nani 的 bindless 纹理行没有生产者。
- 综合完成度：**约 90%（中等置信度）**。函数名 1:1 覆盖 71/80≈89%，语义覆盖约 79/80≈99%；行为合同与生产接线的缺口把整体拉到 90% 左右。

---

## 1. 文件级状态表

行数=当前源码；骨架行数仅表示 PDB 元数据量，不代表原源码行数。
`no_trace` 数来自 `fn_audit.json`。`descriptor_table` / `runtime` / `transient_mesh` 在 B 骨架中**没有同名文件**（见 §5）。

| 文件 | 行数 | 生产 fn/ trait fn | 骨架同名文件 | audit in_file / no_trace / va_only | 性质 | 判定 |
|---|---:|---:|---|---|---|---|
| `mod.rs` | 130 | 15 | 1687 行 | 1 / 2 / 0 | `HandleId`/`Handle<T>`/`Asset` 身份类型 | 真实实现；仅缺 `Asset: From<String>` |
| `audio_trigger_library.rs` | 193 | 6 | 1062 行 | 1 / 0 / 0 | slotmap AABB 触发盒注册表（CPU） | 真实；不负责播放 |
| `color_library.rs` | 631 | 13 | 1334 行 | 3 / 2 / 0 | 37 槽调色板 JSON 加载 + GPU 上传 | 真实；语义有收紧 |
| `configwatch.rs` | 184 | 4 | 642 行 | 2 / 1 / 0 | 配置文件 watcher（100 ms） | 真实；消费者已接 |
| `descriptor_table.rs` | 132 | 9 | 无 | 无 | Bindless 描述符所有权/租约 | 真实；rhapsody 后端 |
| `mesh_library.rs` | 414 | 16 | 3597 行 | 8 / 2 / 0 | CPU 网格、名字/路径索引、修改登记 | 真实；两处与原合同有偏差 |
| `meshwatch.rs` | 180 | 5 | 1659 行 | 2 / 1 / 0 | 网格源路径 watcher（100 ms） | 真实；`watch_path` 被吸收 |
| `placement_data_library.rs` | 162 | 3 | 1291 行 | 1 name_only / 1 / 0 | 摆放 JSON → typed SSBO + 路径去重 | **部分**：`clear` 缺、生产无 `load` 调用者 |
| `runtime.rs` | 207 | 7 | 无 | 无 | ECS 每帧 Reclaim/Reload/Rebuild/Geometry | 真实；已挂 PostUpdate |
| `shader_library.rs` | 319 | 12 | 3023 行 | 5 / 0 / 0 | 光栅名字去重、计算源码哈希注册 | 真实 |
| `ssbo_library.rs` | 709 | 55 | 39521 行 | 3 / 78 / 4 | 类型化存储缓冲注册表 + 图导入缓存 | 真实；78 no_trace 全为单态化 |
| `texture_library.rs` | 1015 | 43 | 4898 行 | 13 / 0 / 0 | 纹理、别名、25 预设、描述符发布 | 真实；2 个 helper 未以同名存在 |
| `texture_watch.rs` | 259 | 10 | 683 行 | 2 / 1 / 0 | 纹理逻辑名 watcher（100 ms） | 库真实；**生产无注册者** |
| `transient_mesh.rs` | 184 | 6 | 无 | 无 | 组件 Drop → 队列 → 批量回收 | 真实；生产者不完整 |
| `vao_library.rs` | 285 | 9 | 1454 行 | 3 / 0 / 0 | mesh → VAO → 顶点/索引缓冲 | 真实 |

---

## 2. 空壳 / 水分扫描

### 2.1 关键字扫描结果

```
grep -rn "todo!\|unimplemented!\|占位\|待取证\|待接线\|未实现\|TODO\|FIXME\|placeholder\|stub" src/asset_libraries
```

- 命中 0 个 `todo!` / `unimplemented!` / `FIXME` / `TODO`。
- 仅 2 条测试注释含“占位”：
  - `D:/game/tiny-glade/crates/country-core/src/asset_libraries/audio_trigger_library.rs:104`（slotmap 测试台说明）
  - `D:/game/tiny-glade/crates/country-core/src/asset_libraries/ssbo_library.rs:585`（测试设备回填说明）

### 2.2 “只日志/只默认值/只错误”清单（逐条定性）

| 位置 | 内容 | 定性 |
|---|---|---|
| `texture_library.rs:110-133` | `impl TextureLibraryDevice for GraphicsDevice` 四个方法全部 `Err(unsupported(...))` | **诚实的默认接缝**。生产路径不是它，而是 `tiny-glade/src/render_host.rs:468-574` 的 `TextureHost`。不算未实现生产逻辑，但直接拿 `GraphicsDevice` 建纹理会失败。 |
| `descriptor_table.rs:56-61,92-97,117-126` | 未接堆时返回 `unsupported`；`sampled_capacity` 返回 `u32::MAX` | 记录型后端的设计分支；有真实 `from_heap` 路径。 |
| `ssbo_library.rs:27-32` | trait 默认 `cpu_shadow_words`/`instance_count` 返回 `None` | trait 默认值，实例有覆盖（`ShadowedBuffer`、`InstanceBuffer`）。 |
| `shader_library.rs:173`、`texture_library.rs:189`、`texture_watch.rs:92` | `impl Resource for X {}` | Bevy 标记 impl，不是空壳。 |
| `configwatch.rs:32-43`、`meshwatch.rs:69-77`、`texture_watch.rs:95-107` | 回调只记录/去重，不做文件 I/O 与 GPU | **原作设计**（文件线程只登记通知），非水分。 |
| `runtime.rs:93,118,152,182,204` | 失败只 `log::error!` 并保留队列 | 有意的可重试语义；对应状态未丢失。 |
| `color_library.rs:260` | 主题文件 `NotFound` 静默跳过 | 与原版“基础 + 可选主题覆盖”一致。 |
| `mesh_library.rs:168` | 热重载失败只记日志、保留旧网格 | 与 README 声明一致；但见 §6 的 `get_mut` 顺序偏差。 |

### 2.3 空体

除 3 个 `impl Resource for X {}` 标记 impl 外，没有空函数体。`Default`、getter、trait 转发均为正常实现。

---

## 3. `no_trace` 逐条判定（88 条）

| 文件 | 数量 | 符号形态 | 判定 |
|---|---:|---|---|
| `ssbo_library.rs` | 78 | `impl$N::as_any` / `as_any_mut` 的 78 个具体类型单态化（$0=38、$1=21、$2=19） | **全部被 blanket impl 覆盖**：`ssbo_library.rs:35-57`（`ShadowedBuffer<T>`）、`:63-76`（`TypedStorageBuffer<T>`）、`:78-94`（`InstanceBuffer<T>`）、`:96-109`（`MappedStorageBuffer<T>`）。78 个类型标识全部存在于当前 workspace；`Nani*`/`Brick*`/`Cobblestone*`/`TerrainHeight*` 等由宏生成。原版每个具体类型生成一次，当前泛型 impl 会生成同样单态化，**不是缺失、不是内联丢失**。 |
| `ssbo_library.rs` | 4 `va_only` | `get_mut<BrickTransformSsbo/RoofTileSsbo/WoodenPlankSsbo/SparkleSsbo>` | 由泛型 `AssetSsboLibrary::get_mut<T>`（`:323-328`）覆盖；4 个类型均有 `SsboType` impl。 |
| `color_library.rs` | 2 | `deserialize_from_path<PathBuf>`、`deserialize_from_path<&str>` | 由泛型 `deserialize_from_path<P: AsRef<Path>>`（`:187-191`）覆盖。 |
| `configwatch.rs` | 1 | `watch::closure$0` | 当前闭包 `:70-72`。 |
| `meshwatch.rs` | 1 | `watch::closure$0` | 当前闭包 `:53-64`。 |
| `mesh_library.rs` | 1 | `reload_mesh_src::closure$1` | 当前 loader 闭包 `:166-169`（`reload_mesh_src_with`）。 |
| `mesh_library.rs` | 1 | `active_widget_state` | **错归属**。骨架同地址 `5368718416` 的完整名是 `country_core::systems::ui::resources::ui_state::UiState::active_widget_state`（`skeleton/.../mesh_library.rs:1876`），调用方是 `construct_wooden_gate_doors` 等；与 mesh 库无关。 |
| `mod.rs` | 1 | `impl$21::register_required_components<Vao>` | `#[derive(bevy_ecs::component::Component)]`（`mod.rs:43`）生成。 |
| `mod.rs` | 1 | `impl$11::fmt<rhapsody::raw::texture::Texture>` | 泛型 `impl<T> fmt::Debug for Handle<T>`（`mod.rs:95-99`）生成。 |
| `placement_data_library.rs` | 1 | `load<PathBuf>` | 当前 `load(..., path: PathBuf)`（`:31-51`）。 |
| `texture_watch.rs` | 1 | `watch::closure$0` | 当前闭包 `:131-133`。 |

**结论：88/88 已解释；0 个需要补写函数的 `no_trace`。**

---

## 4. 关键结构实读结论（13 条）

1. **`mod.rs` 身份系统是真实实现。** `HandleId::new` 用进程级 `AtomicU64` + `fetch_update(checked_add)`，零保留，编号耗尽 panic（`:15-25`）；`Handle<T>` 只含 id + `PhantomData<fn() -> T>`，Clone/Copy/Eq/Ord/Hash/Debug 齐全（`:42-99`）。原版 `impl$1::from`（骨架源码行 42）对应 `Asset: From<String>`，当前只有 `Asset::new(String)`（`:103-115`），**缺一个 From impl**，无调用者，风险低。
2. **`mesh_library` 是真实 CPU 注册表。** `FrozenMap<Handle<Mesh>, Mesh>` + 名字/路径双索引；`add` 写三个表（`:57-77`）；`reload_mesh_src_with` 先读文件、成功后才替换并登记修改（`:143-170`）；`try_reload_mesh_src` 提供可恢复错误入口（`:172-191`）。
3. **但 `mesh_library` 有两处与 REVIEW 记录的原作合同相反。** (a) `get_mut` 先 `get_mut` 命中后才 push `modified_meshes`（`:84-88`），而 REVIEW §7 记录原作“查找之前登记修改”；(b) `remove` 会 `retain` 清空 `names` 与 `paths`（`:119-130`），而 REVIEW §7 记录原作**不清路径表**。需人工用原指令复核后再定性。
4. **`vao_library` 是真实 GPU 路径。** `add` 调 `try_build_vao`；`rebuild_vao` 先建新 VAO，再 `release_vao` 旧缓冲并保留 VAO 身份（`:76-102`）；空索引直接删除旧 VAO（`:85-88`）。真正的缓冲创建在 `render/vao.rs:71-108`（索引缓冲 + 每属性顶点缓冲），设备端落到 `BufferHost`。
5. **`transient_mesh` 回收链本身可靠。** Drop 只入队（`:39-46`）；`drain_meshes_to_delete` 在锁外执行删除，并用 `DeletionBatch::drop` 在 panic 展开时归还未完成项（`:48-81`）；`delete_dropped_transient_meshes` 先克隆 `Arc` 再调用（`:84-94`）。**缺口在生产端**：只有 `utils/debug_viz.rs:256` 创建 `TransientMesh`；`systems/ui/signifiers/signifier_factory.rs:415-431` 明确注明 `TransientMesh` 未落库；门系统用的是另一套 `tg-building-runtime/src/gates.rs:66` 的 `GateTransientMesh`。
6. **`shader_library` 无壳。** 光栅按 `(name, desc)` 去重，同名不同描述 panic，`force_new` 保留旧对象并覆盖名字（`:51-80`）；计算程序按 `ComputeShaderSourceHash` 索引，使用 `NoHashHasher`（`:83-103`）；查询/调试名/按 raw id 查询齐全（`:105-164`）。
7. **`ssbo_library` 是真实类型化注册表。** `Ssbo` trait 提供 buffer/handle/CPU 影子视图（`:21-33`）；注册时以 `NameHash(NAME)` 为键、`type_name::<T>()` 做冲突检测（`:270-288`）；图导入按周期缓存并在 `export_from_rg` 清缓存（`:192-207,382-388`）；`snapshot_graph_leases` 共享同一缓存（`:308-320`）。`add_existing`/`get_dyn`/`get_mut_dyn` 在原版是内部 helper（PDB 分别内联进 `add_new`、`get<T>`、`get_mut<T>`），当前由 `register`/`get`/`get_mut`/`lookup`/`lookup_buffer` 直接实现，语义等价。
8. **`ssbo_library` 的图侧消费是真实的。** `draw_callbacks_a.rs:216-237`、`draw_callbacks_b.rs:266-285` 通过 `lookup`/`lookup_buffer` 把 `(name, InstanceInfo)` 映射为 `SsboInstanceView`；`draw_shaders/ssbo_tags.rs:67-79` 用 xxh3 槽名查缓冲并断言完整类型名。78 个具体 SSBO 类型都能走这条路径。
9. **`texture_library` CPU 侧真实，GPU 侧靠宿主。** 纹理由 `AssetIndex` 标识、名字别名虚拟化（`:219-298`）；25 个预设整批创建、失败整批回滚（`:334-363`）；清零复用 rhapsody 格式表并做溢出检查（`:381-422`）；描述符索引有真实 `BindlessHeap` 路径与记录型回退（`:457-491`）；提交失败保留 pending（`:502-526`）。`get_vk_image_view`/`update_texture` 的**生产实现不在本目录**，而在 `tiny-glade/src/render_host.rs:468-574`。
10. **`texture_library` 两个同名 helper 缺失。** 原版 `get_mut_by_name`（骨架源码 215，内联宿主 `systems::backend::texture_watch::texture_watch`）与 `get_handle_by_name`（骨架源码 220，内联宿主 3 个 `nani_update_*`）当前没有同名公开函数；现有 `replace`（`:545-568`）和 `get_handle_and_texture_by_name`（`:279-284`）提供部分等价能力。
11. **`color_library` 是真实加载/上传链。** 37 个槽名与 GPU 布局一致（`:17-64`）；`try_update_color_library_in` 先读基础 `colors.json`、再覆盖主题文件，校验全部通过后 `update_buffer_bytes`，最后才发布 CPU 副本（`:230-274`）。相对原作额外加了“非有限数值”拒绝（`:175-178`），属于行为收紧。
12. **`audio_trigger_library` 真实且消费端存在。** slotmap 带版本键（`:9-33`）；`add/update/remove/clear/get_triggers`（`:41-86`）被 `systems/prefab.rs:474,594,632` 与 `systems/audio/trigger_audio.rs:142` 实际调用；不播放声音，符合原作分工。
13. **`placement_data_library` 的库体小但数据布局已不是占位。** `PlacementDataSsbo` 现为 48 B 真实布局（`resources/placement_data.rs:9-30,91-100`，含 `rotation_degrees` alias、`anim_t_packed`、`bend_vector`）；`load` 做路径去重、JSON 解析、typed buffer 创建（`:31-51`）。**但 `clear` 缺失，且全 workspace 没有生产代码调用 `load`**（仅测试），所以运行时摆放表始终为空。

---

## 5. 三个“无骨架同名文件”的模块

| 文件 | 说明 | 判定 |
|---|---|---|
| `descriptor_table.rs` | 从原 `texture_library` 的描述符逻辑拆出；`rhapsody_contract.rs:196` 重新导出 `DescriptorSet`；`TextureHost` 用 `from_heap` 构造真实表。 | 工程拆分，真实实现。 |
| `runtime.rs` | 新增 ECS 维护调度：`Reclaim → Reload → Rebuild → Geometry`（`:54-70`），消费 `MeshWatch`/`ConfigWatch`/`TextureWatch`，回收 transient/普通删除网格，重建 VAO。`GameFrameDriver::new` 与 `game_loop.rs:93-96,297` 挂到 `PostUpdate`。 | 真实接线模块。 |
| `transient_mesh.rs` | 原逻辑应该散在 mesh library 附近；当前独立成文件。 | 真实实现，生产者不完整。 |

---

## 6. CPU 侧 / GPU 侧 / rhapsody 接线

### 6.1 真实存在的 CPU 侧

| 能力 | 位置 | 状态 |
|---|---|---|
| 资产身份与句柄 | `mod.rs:15-99` | 真 |
| CPU 网格注册/按名/按路径/重载 | `mesh_library.rs:57-191` | 真 |
| 网格修改登记 + VAO 重建消费 | `runtime.rs:159-207` | 真（已挂 PostUpdate） |
| 普通删除 + 瞬态删除回收 | `mesh_library.rs:119-135`、`transient_mesh.rs:48-94`、`runtime.rs:72-85` | 真 |
| 着色器注册/去重/查询 | `shader_library.rs:39-165` | 真 |
| SSBO 注册/类型校验/图缓存 | `ssbo_library.rs:218-389` | 真 |
| 纹理注册/别名/预设 CPU 状态 | `texture_library.rs:201-448` | 真 |
| 调色板 JSON 加载/主题覆盖/原子发布 | `color_library.rs:220-274` | 真 |
| 摆放 JSON → SSBO | `placement_data_library.rs:31-51` | 库真，**无调用者** |
| 音频触发盒 | `audio_trigger_library.rs:41-86` | 真，消费端存在 |
| 配置/网格 watcher | `configwatch.rs:25-79`、`meshwatch.rs:18-87` | 真，注册者存在（`init_color_library.rs:17`、`startup_grass.rs:65` 等） |
| 纹理 watcher | `texture_watch.rs:87-151` | 库真，**无 `.watch()` 调用者** |

### 6.2 GPU 上传/绑定

| 能力 | country-core 内 | 生产实现在哪里 | 状态 |
|---|---|---|---|
| SSBO 缓冲创建/更新/映射/地址 | `ssbo_library.rs` → `render/ssbo.rs:181-249`（`#[cfg(not(test))] impl GraphicsDevice for rhapsody_contract::GraphicsDevice`）→ `buffer_host()` | `tiny-glade/src/render_host.rs:118-240`（`BufferHost`） | 真 |
| VAO 缓冲创建/释放 | `vao_library.rs` → `render/vao.rs:71-112` | 同上 `BufferHost` | 真 |
| 调色板上传 | `color_library.rs:263-272` | `BufferHost::update_buffer_bytes` | 真 |
| 纹理创建/上传/清屏 | `texture_library.rs` 的 `TextureDevice`/`TextureLibraryDevice` 接口 | `tiny-glade/src/render_host.rs:430-574`（`TextureHost`） | 真 |
| Bindless 描述符分配/租约 | `descriptor_table.rs:36-132` | `rhapsody::bindless::BindlessHeap` + `TextureHost::create_persistent_descriptor_set` | 真 |
| 纹理图导入/导出 | `texture_library.rs:136-171`（RG access `0x22`） | `rhapsody_types::RenderGraph` | 真 |
| 默认 `GraphicsDevice as TextureLibraryDevice` | `texture_library.rs:110-133` | 只有错误返回 | **故意失败的接缝**，生产不用 |

### 6.3 与 rhapsody 的接线点

- `ssbo_library.rs:12-18`：`rhapsody_boundary::{Buffer, GraphicsDevice, ResourceHandle}`；RG access `0x37`；导入 `graph.import_unowned`、导出 `graph.export`。
- `descriptor_table.rs:5-8`: `rhapsody::bindless::{BindlessHeap, BindlessLease}`、`rhapsody::raw::texture::TextureView`、`ash::vk`；`allocate_texture` 调 `heap.sampled_image(view, SHADER_READ_ONLY_OPTIMAL)`，`lease()` 返回 `Arc<BindlessLease>`，`retire()` 只释放库内引用。
- `texture_library.rs:174-178`：格式字节数用 `rhapsody::raw::format::observed::try_vk_format_info(...).word_at_04()`，未知格式显式失败。
- `runtime.rs:23-43`：`AssetLibraryRuntime` 持有 `Arc<dyn GraphicsDevice>` 与 `Arc<dyn TextureLibraryDevice>`；`render_bridge.rs:140-173` 用同一 `BufferHost`/`TextureHost` 安装，`game_loop.rs:93-96,297` 注册维护系统。
- `render/ssbo.rs:181-249`：唯一 `GraphicsDevice` 包装；无 `buffer_host` 时返回错误并打 `WIRING §4.8` 日志，不伪造成功。
- `shader_library` 本身不直接调用 rhapsody；`RasterShaderProgram`/`ComputeShaderProgram` 由 shader 后端创建后入库。

---

## 7. 仍未复现清单（14 条，带证据）

### A. 真实功能/接线缺口

1. **`AssetPlacementDataLibrary::clear` 不存在。**
   - 原作：骨架 `skeleton/B/.../asset_libraries/placement_data_library.rs:586-588`（源码行 41），调用者 `startup::clearing::uninit_clearing`（同文件 `:1238-1243`；`skeleton/.../startup/clearing.rs:1359`）。
   - 当前：`placement_data_library.rs` 只有 `new`/`load`（`:21-51`）；全 workspace `grep uninit_clearing` 无函数；`crates/country-core/src/startup/clearing.rs` 整个文件不存在。
2. **摆放数据在生产中从不加载。**
   - `AssetPlacementDataLibrary::load` 唯一调用点是本文件 `#[cfg(test)]`（`:87-157`）；`loading_graph.rs:210`、`scene_compute.rs:75`、`render_bridge.rs:537` 只读取资源，不调用 `load`。
   - 后果：若上层绘制依赖 placement 表，会得到空表。需人工确认原版 `init_clearing`/放置配置的完整消费者。
3. **`TextureWatch` 没有任何生产注册者。**
   - `texture_watch.rs:127-136` 的 `watch()` 在全 workspace 无调用；`gameplay_startup.rs:42-43` 只 `insert_resource(TextureWatch::new(watching))`。
   - 因此 `runtime.rs:121-155` 的纹理热重载系统虽然是真实消费者，但永远不会收到名字。
4. **原 `systems::backend::texture_watch::texture_watch` 系统未复现，`get_mut_by_name` 缺失。**
   - 骨架 `texture_library.rs:2097-2099`（源码行 215）、`:4701-4707`（内联宿主正是该 backend 系统）。
   - 当前替代方案是 `runtime::reload_textures` + `AssetTextureLibrary::replace`（`:545-568`），行为不同（重新分配描述符而不是原地改纹理）。
5. **`AssetTextureLibrary::get_handle_by_name` 缺失，nani bindless 纹理行无生产者。**
   - 骨架 `texture_library.rs:2258-2260`（源码行 220），内联宿主 `nani_update_solid_vertex_color_window_glass`、`nani_update_cloth`、`nani_update_uber`（`:4688-4697`）。
   - 当前 `render/nani/systems.rs:307-330,515-535,565-590` 的 `WindowGlassRow`/`ClothRow`/`UberRow` 只在定义处出现，workspace 中没有构造这些 row 的生产者；`normal_map_bindless_index`/`albedo_bindless` 没有取值来源。
6. **`TransientMesh` 生产端不完整。**
   - 只有 `utils/debug_viz.rs:256` 创建。
   - `systems/ui/signifiers/signifier_factory.rs:415-431` 明确写 `TransientMesh` 未落库。
   - 门系统使用独立的 `tg-building-runtime/src/gates.rs:66` `GateTransientMesh`（`:370` spawn），不进入共享回收队列。
7. **`AssetPlacementDataLibrary` 的 `clear`/`load` 之外，原 `add` 也被吸收。**
   - 骨架源码行 108；当前 `load` 内联 `Handle::new()` + `buffers.insert` + `paths.insert`（`:46-49`）。语义等价，但若原 `add` 有独立的行为（例如从外部已建缓冲登记），当前无对应入口。**需人工**。

### B. 与原作合同不一致（行为偏差，需人工复核）

8. **`AssetMeshLibrary::remove` 清空名字与路径表。**
   - 当前 `mesh_library.rs:119-130`。
   - REVIEW.md §7 记录原作合同：“`MeshLibrary::remove` 不清路径表；对旧路径再次 reload 可能 panic”。当前行为更“干净”，但不等价。
9. **`AssetMeshLibrary::get_mut` 登记顺序。**
   - 当前先 `get_mut` 成功后才 push（`:84-88`）；加载失败不会登记修改。
   - REVIEW.md §7 记录原作“在查找之前登记修改；加载失败保留旧 mesh 但仍登记变更”。当前测试名 `get_mut_pushes_handle_before_lookup_even_when_missing`（`:290`）与实际断言不符，测试名需修正/行为待复核。
10. **纹理别名覆盖真实名字的 panic 时序。**
    - 当前先检查再 panic，旧真实映射保留（`texture_library.rs:252-277`；测试 `:723-733`）。
    - REVIEW.md §7 记录原作“先 insert 后 panic”。当前是行为修正，不是 1:1。
11. **`RockyTerrainVertexSSBO` / `RockyTerrainTriangleSSBO` 被改名为 `RockyTerrainVertexSsbo` / `RockyTerrainTriangleSsbo`。**
    - 骨架名见 `fn_audit` 的 `no_trace` 列表；当前 `resources/rocky_terrain.rs:38,60`。
    - 库键是 `StorageElement::NAME`（"Rocky Terrain Vertices"/"Triangles"），不是 Rust 类型名，因此功能不受影响；但 PDB 符号对照需注明改名，**需人工**确认是否有按 `type_name` 比较的外部逻辑。
12. **`Asset: From<String>` 缺失。**
    - 骨架 `mod.rs:182-198`（`impl$1::from`，源码行 42，内联调用 `Asset::new` 源码行 31）。
    - 当前只有 `Asset::new`（`mod.rs:103-115`）。无生产调用者，置信度高但影响低。

### C. 已吸收、不是缺失（列出以防误报）

13. **`add_raw` / `update_bindless_texture_table` / `DescriptorIndex::allocate` / `clear_texture` 已吸收。**
    - 骨架 `texture_library.rs:1250-1252`（add_raw → `add`）、`:3157-3159`（→ `try_bindless_textures_dset`）、`:39-41`（→ `try_handle_to_bindless_index`）、`:3870-3872`（→ `try_clear_preset_texture`，内联宿主是 `clear_preset_texture`）。
    - 当前分别由 `add`（`:219-250`）、`try_bindless_textures_dset`（`:502-526`）、`try_handle_to_bindless_index`（`:457-491`）、`try_clear_preset_texture`（`:381-422`）实现。
14. **`add_existing` / `get_dyn` / `get_mut_dyn` / `watch_path` / `AudioTriggerCollection::clear` / `AssetVAOLibrary::build_vao` 已吸收。**
    - `add_existing` → `register`（`ssbo_library.rs:270-288`）；`get_dyn`/`get_mut_dyn` 是 `get<T>`/`get_mut<T>` 的内部 helper（骨架 `:12629-12631,12666-12670,36853-36862`），当前 `get`/`get_mut`/`lookup`/`lookup_buffer` 覆盖；`watch_path` → `watch`（`meshwatch.rs:43-67`）；`Collection::clear` → `Library::clear`（`audio_trigger_library.rs:82-86`）；`build_vao` → `render/vao.rs` 自由函数（`vao_library.rs:68` 调用）。

---

## 8. 基线文档纠错

1. `STUBS-2026-10-19.md:41` 的 asset_libraries “空壳=1” 未见对应生产空函数；3 个 `impl Resource for X {}` 是标记 impl。**需人工确认该计数口径。**
2. `asset_libraries/README.md` 仍称摆放库使用 `{x, y}` 占位 schema；当前 `resources/placement_data.rs` 已是 48 B 真实布局（`:9-30,91-100`）。**README 过期。**
3. `asset_libraries/REVIEW.md` 的行号与“当前未实现”描述对应修复前源码；其中描述符表占位、热重载消费者、transient 回收等问题在当前源码已变化。不能再用它判定当前缺口。
4. `STUBS` 的“6 个完全没有迹象函数”中，5 个已在本轮确认被吸收；仅 `get_mut_by_name` 仍缺。

---

## 9. 完成度与置信度

| 维度 | 完成度 | 依据 | 置信度 |
|---|---:|---|---|
| 文件/符号直接覆盖 | 89%（71/80 具名函数按 STUBS） | `STUBS-2026-10-19.md:41,93` + 骨架/源码逐文件对照 | 高 |
| 语义覆盖（含吸收/单态化） | ~99%（79/80） | 6 个 no-sign 中 5 个找到等价实现；88 no_trace 全部解释 | 中高 |
| 行为合同 1:1 | ~85% | §7.B 的 4 处偏差 + README 承认的工程化改动 | 中 |
| CPU 侧库管理/加载/缓存 | ~95% | §6.1；placement 无调用者、texture watch 无注册者扣分 | 高 |
| GPU 上传/绑定（生产路径） | ~90% | `BufferHost`/`TextureHost` 真实实现；country-core 默认适配器故意报错 | 中高 |
| 运行期接线（消费者/生产者闭合） | ~80% | §7.A 的 6 条接线缺口 | 中 |
| **综合** | **约 90%** | 函数覆盖高、接线与行为合同拖累 | **中** |

**不宣称**：本审计未重跑反编译、未执行 GPU/端到端游戏、未验证原进程逐位等价；所有“真实实现”指当前源码有可执行逻辑并通过静态调用点检查，不等价于原作行为已验收。
