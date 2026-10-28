# country-core::utils

这里提供游戏侧的编辑几何、脏区追踪、网格加载、调试绘制和音频循环工具。它与工作区的 [`utils` crate](../../../crates/utils/README.md) 是两个不同的模块：本目录保留游戏调用策略，共享 crate 提供曲线、采样、索引等可复用实现。

本文按 2026-10-18 实际源码与调用方整理。代码正确性问题、反例和证据见 [REVIEW.md](previous-REVIEW.md)，检查记录见 [审查报告](../2026-10-18-country-core-utils-review/README.md)。本次没有修改业务实现，也没有完成原版行为等价验收。

## 文件与功能

| 文件 | 业务逻辑 | 当前消费者或边界 |
|---|---|---|
| [mod.rs](../../../crates/country-core/src/utils/mod.rs) | Bevy Resource 访问、消费事件队列、世界坐标转屏幕坐标；再导出哈希函数 | Resource 访问用于绘制参数等；哈希复用 `render::utils`。事件 helper 和屏幕转换尚未检索到本目录外的直接调用 |
| [dirty_tiles.rs](../../../crates/country-core/src/utils/dirty_tiles.rs) | 用 64 行 `u64` 记录 64×64 个脏瓦片；矩形/笔画置位、相交查询、筛选、快照清空、OR 合并 | `resources::world_raster`、碰撞世界、墙发布及历史栅格回放 |
| [conservative_line_raster.rs](../../../crates/country-core/src/utils/conservative_line_raster.rs) | 把带半径线段保守栅格化，通过回调输出整数格坐标 | `DirtyTiles::rasterize_world_space_stroke`；自身不裁剪到 64×64 |
| [float_grid.rs](../../../crates/country-core/src/utils/float_grid.rs) | 再导出共享 512×512 双线性采样、130 世界尺寸和兼容性开关 | `RasterBufferRef<512>`；算法在共享 crate，本文件主要是适配和测试 |
| [index_condenser.rs](../../../crates/country-core/src/utils/index_condenser.rs) | 稳定 `u32` ID 映射到稠密索引，修复过期索引提示，交换删除 | `render::nani::gpu_world::OwnedGpuWorldSubset`；存储和映射策略复用共享 crate |
| [child_wall.rs](../../../crates/country-core/src/utils/child_wall.rs) | 将子墙区间 `[a,b]` 再切分，生成新 ID 和两个插值端点 | `systems::wall::eraser::archivist`，支持擦除后的子墙追踪 |
| [geo.rs](../../../crates/country-core/src/utils/geo.rs) | 点列生成/更新白色折线网格，二维点映射到 XZ 平面，调整闭合曲线起点 | 复用 `Mesh` 和共享 `Curve2`；本轮未检索到这些公开 helper 的本目录外直接调用 |
| [project_cursor_onto_ray.rs](../../../crates/country-core/src/utils/project_cursor_onto_ray.rs) | 像素坐标转比例屏幕坐标，再作透视校正，将光标投影到世界射线 | `systems::ui::resources::slider3d` 的三维滑块 |
| [load_json.rs](../../../crates/country-core/src/utils/load_json.rs) | 读 JSON 网格、解析属性、校验索引覆盖范围、记录源路径 | `AssetMeshLibrary` 初次加载/热重载，成功后替换资产 |
| [debug_viz.rs](../../../crates/country-core/src/utils/debug_viz.rs) | 球/矩形/折线请求构造、颜色映射、折线延迟实体工厂 | 碰撞可视化等使用请求类型；折线工厂能唤醒，但完整可绘制实体尚未接通 |
| [audio_utils.rs](../../../crates/country-core/src/utils/audio_utils.rs) | 环形事件实例播放器，支持延迟停止和 FMOD `loop` 参数控制 | `AudioCore::instantiate_loop` 及树、水、屋顶、地形等音频系统 |
| [components/transform.rs](../../../crates/country-core/src/utils/components/transform.rs) | 游戏自有 TRS 组件、矩阵、朝向、look-at 和近似比较 | drawable、羊控制、路径、指示符动画；数学复用 glam/共享 Transform |

## 主要数据流

### 编辑区域 → 脏瓦片 → 发布或上传

`DirtyTiles` 是 512 字节位图；第 `y` 行的第 `x` 位对应一个瓦片，单瓦片边长为 8 个栅格像素。世界 XZ 平面使用约 130 的边长。

- 矩形以 `[min_x,min_z,max_x,max_z]` 传入。坐标经 `mul_add(scale,32)`，下界 floor、上界 ceil，再钳到 `0..63`；遍历采用半开区间。因上界也钳到 63，矩形路径不会置第 63 行/列；这是当前刻意保留的原规则，不能直接改成常见的 `0..64`。
- 笔画至少需要两个点；取各点 XZ，转换半径后逐段调用保守线栅格。`set_cell` 只接受 `0..=63`，因此笔画路径可以置第 63 行/列。
- 单点查询 `cell_from_world_space` 使用截断和边界判定，与矩形的 floor/ceil/clamp 不同；NaN 映射到格 0，轻微负的缩放结果也可能截断为 0。
- `extend` / `StateDelta::apply_to` 按位 OR，多个变更不会彼此覆盖。`take_and_clear` 先复制再清空；碰撞世界发布该快照。
- `retain` 按行、行内低位到高位访问已置位格，拒绝的格被清除。世界栅格上传用它保留尚不能处理的瓦片。

保守线栅格选择绝对增量较大的轴作为主轴，排序端点，再逐列计算带宽并输出闭区间格子。交换 X/Y 支持另一主轴；退化线段仍可输出一个格。它会枚举网格外坐标，再由 `set_cell` 丢弃，因此输入跨度直接影响开销；极大坐标的回绕问题见审查 F1。

### 存档版本 → 采样策略 → 世界栅格读取

共享 `FloatGridCells` 使用行优先索引，`RasterBufferRef<512>` 返回 `u8 / 255.0`。默认世界范围为 X/Z 各 130。

旧模式按格角采样，在正边界保留外推行为；新模式按格心采样并钳到边缘格。`CENTERED_PATH` 是同一份共享原子开关，初始为 false。`LoadVersionGates::from_version` 以 `version >= 0x1a` 选择新模式，`apply_centered_path` 提供写入接口；**当前生产加载路径尚未调用该写入接口**。不能把“开关已有定义”理解为读档兼容链已经闭合。

### 稳定 ID → 稠密槽位 → 并行实例数组

`IndexCondenser<T>` 的类型参数只作标记，内部委托 `SharedCondenser<u32>`。插入重复 ID 返回 `None`，`push` 则 panic；解析先验证缓存提示，再查映射并写回正确索引。

删除返回被删除项的旧稠密索引，末项交换到该槽位，映射同步修复。消费方必须对位置/实例数据数组做同样的交换删除；已有句柄的提示允许过期，下次解析会修复。ID 删除后重用不包含 generation 防护，调用方负责生命周期。

### 屏幕拖拽 → 世界编辑点

`slider3d` 提供窗口尺寸、两个相机矩阵和 `Ray {origin,direction}`。先将光标换算到比例屏幕空间，再投影射线起点与 `origin + direction`，计算屏幕方向上的参数，限制到指定 range，最后按两个裁剪 w 作透视校正插值。

关键约定：

- `first` 先应用、`second` 后应用；第二次变换使用前次结果的 xyz 补 w=1。
- `transform_screen_space` 当前 y 公式为 `(1-b)/height*2+1`，不能擅自替换为常见的 `1-2*b/height`；源码保留的是原证据解释，本文未重新核验该机器码。
- range 限制的是屏幕插值参数；透视修正后世界参数不必相同。逆序或 NaN range 会 panic。
- 点投屏返回像素坐标并翻转 y；没有视锥剔除，也没有对零窗口、w=0、非有限矩阵提供有效性结果。

### JSON 文件 → Mesh → 资产热重载

JSON 需要 `indices.buffer`、属性名数组 `attributes`，以及每个同名属性的 `type` 和 `buffer`。支持 `int×1`、`float×1/2/3/4`；向量每项必须具有准确的分量数。每个属性非空，最大索引必须小于该属性长度；空索引允许。

```json
{
  "indices": {"buffer": [0]},
  "attributes": ["Vertex_Position"],
  "Vertex_Position": {"type": ["float", 3], "buffer": [[0, 0, 0]]}
}
```

`try_load_json_as_mesh` 对文件、语法和属性错误返回 `Err`；`load_json_as_mesh` 用 unwrap 将它们提升为 panic。成功的 Mesh 记录路径和 stamp=0。资产库在返回成功后才替换旧 Mesh，但目前缺少位置等保留属性的语义类型校验，可能把不兼容数据当作成功发布，见 F2。这里的“Result 单态仍 panic”旧注释不代表当前实现。

### 调试绘制与音频循环

`DebugLine::show` 克隆点/颜色，计算折线总长，将带名字的闭包放进 `UiSignifierStream`。唤醒时选择普通/虚线 shader，创建指示符实体，并插入线拓扑；虚线参数为总长×4。`SignifierEntityBuilder::build` 目前仅落部分实体组件，尚未解析 mesh/shader 或插入完整 drawable/pass 组件。球和三维矩形目前只有请求构造器；没有对应 `show`。因此已有测试能证明请求入流和组件写入，不能证明屏幕上能看见这些图元。

`AudioLoop` 对一组 `LoopEntry` 轮换：

1. `start_if_not_playing` 先取消待停止时间；若已启动直接返回，否则按配置写 `loop=1`，只对 Active 实例调用 start，然后置 started。
2. 首次 `stop(now)` 只记时间；之后必须严格超过阈值才执行停止。持续收到播放请求会取消这段等待。
3. 使用 loop 参数时先写 `loop=0`，仅配置显式停止且实例 Active 才调用 stop；不用 loop 参数时对 Active 实例直接 stop。
4. 成功后轮换 cursor、清 started 和时间。start/stop 失败经 `Result` 传播；毫秒计时刻意采用 u32 回绕。

实例集合需要非空，cursor 需要有效。实际实例创建已在 `resources::audio_resources::AudioCore::instantiate_loop` 实现；本文件“未做 instantiate_loop”的注释已过期。默认描述器创建 3 个实例，真实 FMOD 设备仍是独立验证范围。

## 其他合同与使用限制

- `ChildWall::slice` 校验的是**插值结果**位于 `[0,1]`；不强制 `t1/t2` 自身在此范围，也不强制端点递增。传入的新 ID 写到结果中；`new` 不校验。
- `geo::rewire_enclosed_curve` 切出两半、按 0.1 阈值合并临近端点、后半接前半。split=0/1 或任一段过短返回 `None`；其余非法输入可能 panic。函数不先验证曲线闭合，调用方应提供有效闭合曲线。
- 点列网格写入 `Vertex_Position`、全白 `Vertex_Color` 和顺序索引；空点列会 panic。这些 helper 构造折线数据，绘制拓扑由消费方决定。
- Transform 默认为平移零、单位旋转、缩放一；forward 是旋转后的本地 -Z。字段序为 rotation / translation / scale，当前布局测试检查 0/0x10/0x1c 与大小 0x30；不能套用 Bevy Transform 的字段序。
- `get_res_mut` 返回 Bevy 的 `Mut<T>`，保留其变化检测语义。事件 helper 会消费全部双缓冲事件；`get_last_event` 取最后一个的同时也丢弃之前所有事件。它们不适合需要各自 reader 游标的订阅者。
- 公开依赖和共享算法直接复用，不把容器、Serde、glam、Bevy 的实现算作新增游戏业务。源码注释的 O/I/U 是既有证据分类，本文不据此升级恢复进度。
