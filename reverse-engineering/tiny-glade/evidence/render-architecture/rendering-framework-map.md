# Tiny Glade 完整渲染框架包层次地图

研究对象：`D:/game/ljxsj_92385/Tiny Glade` 发行树，基线为 `build-info/Cargo.lock`（SHA256 `9e0cb4a0fa173122fa7cd552aa639f5ee977abc22c74aec92398808c707184bd`）。

本文件回答“渲染框架由哪些包组成、包之间怎样分层、后续从哪里逆向”。它把**渲染直接相关包**从 901 个锁文件包中筛出；未列出的包不是不存在，而是没有被判定为渲染恢复主线的一部分。每项都区分：

- **观察**：由 Cargo.lock、PDB 源路径、发行资产清单或已保存反编译结果直接支持。
- **推断**：由依赖方向、命名和调用关系组合出的工作模型。
- **未知**：需要继续用 PDB/机器码/运行时或 fork 源码确认，不能用包名补齐。

## 一句话结论

原作不是 Bevy 的标准 `bevy_render` 管线，而是：

```text
Tiny Glade / Bevy App-ECS 调度
        ↓
country-core 游戏渲染数据、实例、专用 pass、游戏 shader 接线
        ↓
rhapsody::rg 渲染图、资源图、同步计划、录制与执行
        ↓
rhapsody::raw 资源/命令/Vulkan 封装
        ↓
ash + ash-window + Vulkan 驱动/平台
```

旁边有两条支线：

```text
asset → asset-pipe → asset-pipe-bin
                         ↓
             shader-compiler → shader-pipeline → shader-variants
                         ↓
                  compiled-shaders/*.bin
```

以及：

```text
country-core / rhapsody
  ├─ fsr / fsr-sys                 # 超分辨率接线
  ├─ obvhs                         # GPU BVH/光线候选的外部库候选
  ├─ gpu-allocator                 # Vulkan 显存分配
  ├─ vk-sync-fork                  # Vulkan 同步分类/屏障辅助
  ├─ gpu-profiler                  # GPU 时间/标记
  └─ aftermath-rs / dxgi / nvapi  # 崩溃诊断、GPU 选择/厂商路径
```

## 层次与包清单

### L0：应用、窗口与调度边界

| 包 | 作用 | 分类 | 依据/备注 |
|---|---|---|---|
| `tiny-glade` | 应用入口、帧循环、`render_world`、最终 present 接线；依赖 `country-core`、`rhapsody`、shader 包 | 游戏自有，必须逆向 | Cargo.lock 直接依赖；反编译已定位 `render`/`render_world` |
| `bevy_app` `0.16.0` | App/plugin 生命周期 | 直接复用 | 原锁/PDB确认；不是渲染实现 |
| `bevy_ecs` `0.16.0` | ECS 世界、查询、系统调度 | 直接复用 | 原锁/PDB确认 |
| `bevy_tasks` `0.16.0` | 任务/并行调度，rhapsody 也直接使用 | 直接复用 | 原锁确认 |
| `bevy_input` `0.16.0`、`bevy_time` `0.16.0`、`bevy_state` `0.16.0` | 输入、时间、状态驱动渲染更新 | 直接复用；游戏消费者需逆向 | `country-core`/`tiny-glade` 依赖 |
| `winit 0.29.15` | 窗口生命周期与平台事件 | 复用 PounceLight fork | 原锁 git commit `d03de0a0...`；不恢复库实现 |
| `raw-window-handle 0.6.1` | 将窗口句柄传给 Vulkan | 直接复用 | `rhapsody`/`ash-window`边界 |
| `ash-window 0.13.0` | Vulkan surface 创建辅助 | 直接复用 | 原锁依赖 `ash` + `raw-window-handle` |

**排除项**：原锁中没有 `bevy_render`、`bevy_pbr`、`bevy_core_pipeline`、`wgpu`、`naga`。因此“Bevy 渲染器”不能作为这套发行构建的底层假设；Bevy 在这里主要是 App/ECS/任务/输入/时间框架。Bevy 是分叉这一点是用户给定事实，但 fork 仓库和 commit 仍未知。

### L1：游戏渲染数据与专用 pass（`country-core`）

`country-core 0.2.0` 是运行时游戏渲染层，不等于通用 Vulkan 层。PDB 有 38 个 `crates/country-core/src/render` 源路径。

| 子域/文件族 | 负责内容 | 后续入口 |
|---|---|---|
| `render/nani/{gpu_world,instance_data,buffers,mesh_atlas,trimesh,systems}` | GPU world、实例数据、mesh atlas、实例更新、GPU 驱动物件 | 先恢复 CPU 对象到 GPU 字段映射，再恢复 cull/bucket |
| `render/pass_builder/{compute_pass_builder,draw_builder,pass_resource_access,pass_resources,bind_resources,uniform_binder,draw_list,draw_stream,framebuffer_binding,shader_uri}` | 游戏侧 pass 声明、资源读写、uniform、draw/dispatch/indirect | 连接到 `rhapsody::rg::pass_api/pass_builder` |
| `render/{ssbo,uniform,vao,shadowed_buffer,sparse_instance_buffer,dirty_index_range}` | 类型化/稀疏/阴影/脏区 buffer 与 CPU→GPU flush | 追踪 create/update/flush/drop 生命周期 |
| `render/{mesh,load_texture,shader,render_config,frame_constants,view_constants,directional_light,atmosphere}` | mesh/纹理/shader 接线、帧/视图常量、光照和大气配置 | 逐项恢复描述结构和消费者 |
| `render/{nani/cull_shaders,indirect,raster_single,fsr_plugin}` | GPU cull、间接命令、单体 raster、FSR 接线 | 与发行 `.bin` shader 反射交叉验证 |

**观察**：PDB 中 `GpuWorld`、`TypedStorageBuffer<T>`、`ShadowedBuffer<T>::gpu_flush`、`ComputePass::read_rg_storage_buffer/write_*`、`DrawBuilder::multi_draw_indexed_indirect` 均有符号/源路径证据。

**推断**：屋顶、墙、植被、楼梯等 CPU 状态先变成实例记录/atlas 区间，再由 compute cull/bucket 和 indirect draw 批量提交。

**未知**：每种实例类型的完整字段布局、CPU 更新时序、descriptor 实际句柄、LOD/分桶规则和真实每帧 pass 拓扑仍未恢复。

### L2：渲染图与资源图（`rhapsody::rg`）

`rhapsody 0.1.0` 的 `rg` 是这套自建渲染器的逻辑核心。PDB 记录 15 个 `crates/rhapsody/src/rg` 源路径。

| 模块 | 作用 | 恢复重点 |
|---|---|---|
| `resource`, `resource_info` | buffer/image 资源声明、用途、访问记录 | `ResourceInfo::calculate`、用途枚举、字段布局 |
| `resource_registry`, `resource_pool` | 资源注册、临时资源复用、导入/导出存活管理 | take/insert、跨 pass/跨帧所有权 |
| `graph`, `pass_api`, `pass_builder` | 图节点/pass API 和资源依赖声明 | pass 顺序、闭包捕获、失败路径 |
| `compiled`, `state` | 图编译结果、状态与屏障计划 | `RenderGraphState::compile`、compiled graph 字段 |
| `recorder`, `executor` | batch 录制、command buffer 组织、执行 | `GraphExecutionRecorder::execute_compiled`、提交边界 |
| `binding` | descriptor/resource binding 抽象 | shader 反射与实际绑定映射 |
| `import_export`, `temporal` | 外部资源、history/temporal pair | 交换链、TAA/TGSR、GI history 的生命周期 |
| `util` | 私有辅助 | 仅在被上层调用时恢复 |

**已确认的规则**：访问链建立、生命周期事件、并发访问判断、buffer/image usage 转换、pre-pass barrier 生成均已有机器码证据；这不是“每两个 pass 固定插屏障”。

### L3：Vulkan 资源、命令与 GPU 生命周期（`rhapsody::raw`）

PDB 有 29 个 `raw` 直接模块和 9 个 `raw/managed` 模块。

| 族 | 包/模块 | 职责 |
|---|---|---|
| 实例/设备 | `raw::instance`, `init`, `physical_device`, `device`, `format`, `error` | Vulkan instance、物理设备选择、logical device、格式/错误 |
| 资源 | `buffer`, `texture`, `sampler`, `binding`, `descriptor_set`, `render_pass`, `framebuffer`, `shader` | Vulkan object 创建/绑定/销毁及 descriptor/pipeline 输入 |
| 命令/同步 | `command_buffer`, `semaphore`, `submit`, `barrier`, `temp_list`, `dynamic_constants` | command recording、队列提交、同步、临时常量 |
| 交换链 | `swapchain` | acquire/present/resize/out-of-date |
| 所有权/释放 | `disposal`, `raw::managed::{handle,resources,buffer,texture,sampler,descriptor_set,render_pass,framebuffer,swapchain}` | managed handle、资源表、延迟销毁和租约 |
| 诊断 | `profiler`, `stats`, `debug`, `crash`, `ray_tracing` | GPU 标记/统计/崩溃/可选光追接口 |
| 辅助 | `allocator`, `bytes`, `leek`, `lib` | 分配、字节视图、低层 glue |

**关键状态链**：`create → record → submit → complete → reclaim`。managed 对象、command buffer、semaphore、timeline/回收桶必须联合恢复，不能只凭单个 `Drop` 函数推断释放安全。

### L4：公开 Vulkan/同步/分配器依赖（必须复用）

| 包 | 锁定版本/来源 | 处理边界 |
|---|---|---|
| `ash` | `0.38.0+1.3.281` | 直接复用 Vulkan API，不重写 |
| `gpu-allocator` | `0.26.0`，lock 中无 source | 先确认原 local/fork 来源与补丁；库实现不逆向 |
| `vk-sync-fork` | `0.4.0`，lock 中无 source | 先确认 fork 来源与 classifier 差异；已证实 `is_write` 属于外供库边界 |
| `gpu-profiler` | `0.1.0`，依赖 `ash` | 先分类来源，复用实现 |
| `aftermath-rs` | `0.1.3` | NVIDIA Aftermath 诊断胶水；不当作游戏算法 |
| `dxgi-enumerate-gpus` | `0.1.0` | Windows GPU 枚举辅助 |
| `nvapi-hi` | `0.1.0` | NVIDIA 能力/厂商接口辅助 |
| `leekspin` | `0.1.0`，lock 中无 source | 环形缓冲和去重诊断输出；PDB 符号确认 `RingBuffer`/`print_deduplicated_with_count`，先分类来源 |
| `raw-window-handle` / `ash-window` | `0.6.1` / `0.13.0` | 窗口到 surface 的公开边界 |

`gpu-allocator`、`vk-sync-fork`、`gpu-profiler` 的 local/path 属性不等于游戏自有源码；这是 L0 依赖归属任务。

### L5：Shader 编译、布局与变体工具链

这一层同时服务离线构建和运行时加载，不应与 shader 算法本身混为一谈。

| 包 | 作用 | 分类 |
|---|---|---|
| `shader-pipeline 0.1.0` | shader/pipeline 序列化、键和布局元数据 | 私有候选，需核实源归属 |
| `shader-compiler 0.1.0` | HLSL/变体编译、SPIR-V 处理、反射/预处理接线 | 私有候选，需核实源归属 |
| `shader-variants 0.1.0` | variant 选择/组合 | 私有候选，需核实源归属 |
| `rspirv 0.11.0+1.5.4` | SPIR-V 解析/构造 | 公开依赖，复用 |
| `spirq 0.6.2` | SPIR-V 反射；PounceLight/h3r2tic git rev | 复用精确 commit |
| `shader-prepper 0.3.0-pre.3` | shader 预处理辅助 | 公开依赖，复用 |
| `speedy 0.8.6`, `regex`, `memchr`, `relative-path`, `constcat`, `bytemuck` | 编译器 glue/序列化/字节布局 | 公开依赖，复用 |
| `motomoto 0.1.0` | 带 tag/chunk 的二进制 Reader/Writer；country-core 直接依赖 | local 包，先确认格式用途和来源；PDB 已见 `Reader`/`Writer`/`ChunkIterator` |
| `asset-pipe-bin` | 离线资产和 shader 构建入口 | 私有工具，需核实是否纳入恢复范围 |

发行树有 `compiled-shaders` 404 个 `.bin`：234 个 raster、170 个 compute。它们是运行时输入/证据，不是原 HLSL 源码。SPIR-V 反射已确认 Nani cull/bucket、indirect dispatch、half-res depth/normal、GI、TAA/TGSR、DoF 等入口，但算法仍需单独恢复。

### L6：资产、纹理与格式管线

| 包 | 作用 | 分类/边界 |
|---|---|---|
| `asset 0.1.0` | 游戏资产 schema/路径/序列化胶水 | 游戏私有候选，恢复 schema/消费者 |
| `asset-pipe 0.1.0` | 纹理/mesh/资产转换管线 | 游戏私有候选，恢复自有格式和调用 |
| `asset-pipe-bin 0.1.0` | 资产管线 CLI/批处理 | 工具候选，按是否影响发行构建决定 |
| `image 0.25.1`, `ddsfile 0.5.1` | 图片/DDS 解码 | 公开依赖，复用 |
| `intel_tex_2 0.4.0` | BC/纹理压缩；PounceLight git rev | 复用精确 commit |
| `ispc_rt` | `intel_tex_2` 的底层运行时 | 公开/外供边界，先复用 |
| `rkyv 0.7.39`, `ron 0.8.1`, `serde`, `serde_json` | 存档/资产 schema 序列化 | 复用解析器；逆向游戏 schema/版本迁移 |
| `obvhs 0.1.0` | BVH/几何辅助候选；h3r2tic git rev | 复用库；确认与 GPU BVH 的调用边界 |

### L7：游戏效果与外部算法边界

这些不是一个平面层，而是挂在 L1/L2/L5 上的效果支线：

| 包/资产 | 目前判断 |
|---|---|
| `fsr 0.1.11` / `fsr-sys 0.1.11` | country-core 的 FSR 接线；公开库直接复用，恢复 `fsr_plugin` 的游戏参数/时序 |
| `FidelityFX 1.0.0` | lock 中无 source、依赖为空；可能是外部/SDK 或占位条目，归属未证实 |
| `GPU-Raytracer 1.3.0` | lock 中无 source、依赖为空；不能仅凭名称断言使用了完整硬件光追 |
| `XeGTAO 1.0.0` | lock 中无 source、依赖为空；需确认是否实际链接/调用 |
| `fallback-vulkan-1.dll` | 发行运行时 Vulkan loader fallback，不是 Rust crate |
| `GFSDK_Aftermath_Lib.x64.dll` | NVIDIA GPU 崩溃诊断运行时，对应 `aftermath-rs` |

已观察到的软件 BVH/三角形追踪 shader 声明，不能扩大成“全程序没有硬件光追路径”；`raw::ray_tracing` 和 `GPU-Raytracer` 的关系仍需具体调用证据。

## 直接消费者边界

Cargo.lock 中直接依赖 `rhapsody` 的包是：`country-core`、`tiny-glade`、`system-color`、`system-decorator`、`system-onboarding`、`system-roof`、`system-tree`、`system-wall-constructor`、`system-water`、`tg-gpu-selector`、`tg-text`、`timberframe`、`toolbar`。

这些包不是都属于“通用渲染框架”：

- `country-core` 是核心游戏渲染适配层；`tiny-glade` 是帧入口和最终整合。
- `system-*`、`timberframe`、`toolbar`、`tg-text` 是游戏功能/内容/UI 消费者；恢复它们如何产生或更新 GPU 数据，但不并入 rhapsody 底层。
- `tg-gpu-selector` 与 `tg-iced` 是启动期 GPU 选择/UI 支线；它依赖 rhapsody 读取设备能力，但不代表主游戏画面由 iced 渲染。
- `egui`/`egui-winit` 是游戏 UI 输入/绘制接线；发行 shader 中存在 `_egui.raster...`，实际 UI pass 仍走自建图执行层。

## 运行时数据流地图

```text
建筑/地形/植被/水/天气 CPU 状态
        │
        ├─ country-core::render::nani / mesh / buffers
        │       └─ GPU World、MeshAtlas、实例/间接参数、常量
        │
        ├─ country-core::render::pass_builder
        │       └─ 声明 read/write/attachment、draw/dispatch、shader URI
        │
        └─ tiny_glade::render_world
                └─ upload/rasterize/cull/shadow/main/GI/transparent/post
                         │
                         ▼
                 rhapsody::rg::Graph / ResourceInfo
                         ├─ 访问链与生命周期事件
                         ├─ resource registry/pool
                         ├─ temporal history/import/export
                         └─ compiled graph + pre/post barriers
                         │
                         ▼
                 rhapsody::raw::managed/raw
                         ├─ Vulkan objects / descriptors / pipelines
                         ├─ command buffers / semaphores / submit
                         ├─ delayed disposal / reclaim
                         └─ swapchain acquire/present
                         │
                         ▼
                       Vulkan GPU
```

静态反编译观察到的帧功能族包括：CPU→GPU 上传、地形/建筑/水/植被生成、Nani GPU cull/bucket、shadow、HZB、主绘制、GI/SSGI/ReSTIR 候选、透明/折射、体积、DoF、TAA 或 TGSR、UI、swapchain resolve/present。这里是“构图调用族”，不是已经验收的动态执行时间线。

## 推荐逆向路径

### P0：先锁定边界

1. 从原锁文件固定版本、source、feature；确认 `gpu-allocator`、`vk-sync-fork`、`shader-*`、`asset-*` 的 local/fork 归属。
2. 建立包→PDB 编译单元→源码路径映射；不要把 901 个包或泛型胶水算作游戏源码。
3. 对 `ash`/Vulkan/Bevy/ECS/serde/rkyv/公开 shader parser 一律复用，不重写实现。

### P1：最低层规则与生命周期

1. `BufferDesc → TextureDesc → TextureViewDesc → ImageBarrier/BufferBarrier`。
2. `raw::instance/init/physical_device/device → buffer/texture/view/sampler`。
3. `managed handle/resources → command_buffer/semaphore/submit → disposal/reclaim`，按 `create → record → submit → complete → reclaim` 联合追踪。

### P2：渲染图

1. 完成 `rg::resource_info/resource_registry/resource_pool`。
2. 恢复 `rg::graph/pass_api/pass_builder/compiled/state` 的 CPU 编译结果。
3. 恢复 `rg::recorder/executor/import_export/temporal`，优先做小图 CPU 差分和 barrier 轨迹。

### P3：游戏 GPU 接线

1. `GpuWorld`、typed/shadowed/sparse buffer、MeshAtlas 和实例字段。
2. `ComputePass`/`DrawBuilder` 的资源访问、indirect 参数和 descriptor binding。
3. 固定几个最小 pass：Nani cull/bucket → indirect draw；upload/rasterize terrain；shadow → main pass。

### P4：shader 与效果

按依赖和可验收性分组恢复，而不是按文件名全量展开：

1. cull/bucket、dispatch args、HZB、间接绘制；
2. depth/normal、reprojection、GI/SSGI/ReSTIR；
3. shadow、water/refraction、volumetric、DoF；
4. TAA/TGSR、FSR、final/post/UI、swapchain resolve。

## 当前完成度与不可过度声明的部分

- 已有 5 个私有资源图/用途规则通过机器码案例，见 `evidence/render-recovery/verification.json`；这不等于 L0–L5 完成。
- 已有 93 个 PDB 源路径索引（`rhapsody` 55、`country-core/src/render` 38），但 native 签名是 `void()` 占位，不能当成完整作者源码。
- 已确认自建管线和关键帧构图调用，但完整资源注册池、barrier 全图、提交/回收、动态 descriptor/extent、shader 算法、GPU 画面和性能均未验收。
- 不能把 `FidelityFX`、`GPU-Raytracer`、`XeGTAO`、`gpu-allocator`、`vk-sync-fork` 的包名或 local 属性直接当作游戏自有算法。

## 证据入口

- 依赖：`tracking/original-Cargo.lock`、`tracking/dependency-policy.json`
- 发行资产：`D:/game/ljxsj_92385/Tiny Glade/build-info/manifest.json`
- PDB/render 源索引：`evidence/render-skeleton/report.json`、`evidence/render-skeleton/files.json`
- 自建架构：`evidence/render-architecture/findings.md`
- 自底向上计划：`evidence/render-recovery/bottom-up-plan.md`
- 已验证 CPU 规则：`evidence/render-recovery/verification.json`

