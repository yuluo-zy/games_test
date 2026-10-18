# geometry：基础网格与墙砖发布

本模块把几何参数转换为 CPU 网格，并把墙系统生成的砖实例发布到共享 GPU 缓冲。实现以功能正确和可维护为准；历史反汇编是格式来源，当前代码不再模拟对象偏移、moved-from 哨兵或原失败副作用。

## 结构与职责

| 文件 | 职责 |
|---|---|
| [cube.rs](cube.rs) | 轴对齐盒；每面明确列出端点、法线和 UV，规范化反向区间 |
| [plane.rs](plane.rs) | XZ 正方形；整边长输入，法线朝 +Y |
| [sphere.rs](sphere.rs) | 参数校验、调用 hexasphere 细分、组装球面位置/法线/UV |
| [instanced_wall.rs](instanced_wall.rs) | 砖来源编码、96 字节公共载荷、待上传状态与分配拥有权 |
| [runtime.rs](runtime.rs) | ECS 中延迟创建、批量上传、失败重试及 GPU 就绪状态 |
| [tests](tests) | 原有几何合同与新增发布/错误恢复测试；与生产实现分开 |

## 基础网格合同

所有生成器返回 [Mesh](../render/mesh.rs)，位置、法线、UV 分别存储，索引为 u32。不存在“把 Mesh 当作交错顶点字节流”这一接口。

| 网格 | 顶点 / 索引数 | 属性 |
|---|---|---|
| Box | 24 / 36 | 位置 Float3、面法线 Float3、UV Float2、RGB 白色 Float3 |
| Plane | 4 / 6 | 位置 Float3、+Y 法线 Float3、UV Float2、RGB 白色 Float3 |
| Sphere | `10(n+1)²+2` / `60(n+1)²` | 位置 Float3、单位球法线 Float3、UV Float2，无默认颜色 |

盒体三轴端点自动排序，平面负边长按绝对值解释。二者保留原有不可失败的接口，非有限参数会在入口明确断言；零尺寸允许生成退化几何。需要处理外部不可信参数的调用方应先校验。

`Sphere::to_mesh` 返回 Result：细分最大 79，半径必须有限且非负；零半径允许退化为点。顶点数估计采用饱和运算，超大输入不会回绕成小数量。单位球 UV 使用经纬映射，acos 前夹取 y 的舍入误差。UV 接缝不额外拆点，材质仍需选择相应的采样方式。

细分算法直接复用 `hexasphere 12.0.0`，向量运算复用 glam。没有自建球体细分、向量库或内存布局转换器。prefab 的 [rt_cube_positions](../systems/prefab.rs) 继续使用盒体索引展开的 36 个位置。

## 砖实例只有一份格式

`BrickTransformSsbo` 是对齐 4、大小 96 字节的 repr(C) 类型，由 bytemuck 的 Pod/Zeroable 派生检查无填充。`recovered_building::wall_constructor::WallBrickInstance` 现在直接再导出这一类型，CPU 铺砖与 GPU 上传不再维护两份相同结构，不需要中转 Vec 或逐字段转换。

| 偏移 | 字段 | 合同 |
|---|---|---|
| 0x00 | affine_columns | 四列三分量仿射矩阵 |
| 0x30 | deformation | 四个变形参数，默认全零 |
| 0x40 | surface | 三个表面参数，默认 -100 禁用哨兵 |
| 0x4c | seed | 生产者给出的逐砖序号/种子 |
| 0x50 | material | 砖源编码 |
| 0x54 | wall_distance | 两个墙向距离参数 |
| 0x5c | flags | shader 实例标志 |

默认仿射矩阵全零，并非单位变换；有效矩阵由生产者填充。`observed_bytes` 为已有序列化/诊断保留固定小端输出。部分 shader 字段的精确语义仍以生产者为准，不由恢复命名推导额外规则。

砖来源规则保持兼容：输入低 32 位中的 100000、100001、100002、100003、100011 编为 bit31 家族；边编号左移 4 位并加 bit30 家族位。该格式不提供任意 64 位墙 ID 的全局唯一性。

## 更新、提交和删除

```text
construct_wall_bricks 产生公共实例列表
    → InstancedWall::new / update（标为待提交）
    → PostUpdate / AssetMaintenance::Geometry
    → 先检查容量，再批量替换稀疏 chunk
    → 合并 dirty 区间，try_gpu_flush 提交真实设备
    → 绘制入口按完整类型名查询底层缓冲句柄
```

`InstancedWall` 独占分配，不实现 Clone。其列表私有，只能通过 `update` 标记修改；`pending_instances` 提供只读检查，`is_upload_pending` 用于查询状态。

- **无更新**：直接返回 `Ok(false)`，保留上次几何。
- **有效更新**：复用旧 chunk，只申请增长部分；成功后释放 CPU 列表，返回 `Ok(true)`。
- **显式空更新**：释放该墙的 chunk，正确表达删除全部砖。
- **容量不足**：返回 `CapacityExceeded`，旧几何和完整待提交数据均保留，后续帧自动重试。
- **错误缓冲**：返回 `ForeignStorage`，不允许同类型缓冲间误用 chunk。
- **设备失败**：保留 dirty 位供下帧重试，状态 `gpu_ready=false`；实际墙砖回调暂缓绘制，防止读取不完整设备更新。
- **实体删除/组件移除**：分配析构入共享回收队列，下一次同步归还 chunk。

CPU 容量检查失败不会留下部分替换；设备调用本身不承诺可回滚，因此 GPU 失败采用阻止消费并重试全部脏范围的策略。

## 生产接线与性能

现有 [资产维护调度](../asset_libraries/runtime.rs) 增加 Geometry 阶段，排在 Rebuild 之后。真实宿主 [CoreRenderBridge::install_asset_libraries](../../../tiny-glade/src/render_bridge.rs) 初始化 AssetSsboLibrary 和 WallBrickUploadStatus；GameFrameDriver 已在渲染读取世界之前执行同一 PostUpdate 调度。自定义宿主使用 runtime 时也应初始化这两个资源。

砖缓冲按需创建，空场景和从未拥有砖的空墙不会分配它。容量仍为 65,536 个 chunk × 32 槽，即 2,097,152 个实例。CPU 影子占 192 MiB，设备缓冲同容量；保留已有 shader 容量合同，没有添加另一套动态池。

生产调度使用 `SparseInstanceBuffer::try_new_owned`，存储持有创建它的设备 Arc 和原始代际句柄。销毁存储、资产库或 World 时自动调用该设备的 `release_buffer`，不依赖 runtime 的销毁顺序；晚于存储销毁的墙分配只持有弱引用，不会重复释放设备缓冲。宿主槽及时归还，已提交 GPU 命令持有的底层资源继续由宿主现有引用与延迟销毁机制保护。句柄副本不拥有资源。借用设备的旧 `try_new` / `new_storage_buffer` 接口仍由调用方管理释放，新生产接线应采用拥有设备的构造入口。

创建时不再分配和上传额外的整块 192 MiB 清零数组；GPU 只消费已写入且有效的槽。批量替换每墙只锁一次空闲列表，按 chunk 连续复制。每帧共享缓冲最多一次设备提交，脏区间相邻时合并；静止帧没有设备写入。成功后不为每堵墙额外保留已提交的 CPU Vec 容量。

绘制通过独立 `lookup_buffer` 查询句柄，不再要求稀疏缓冲伪装成带普通实例数的缓冲。原实例计数查询保留给常规实例绘制。

## 验证与边界

在 workspace 根执行：

```sh
cargo test -p country-core --lib --test asset_libraries --locked --offline
cargo check -p tiny-glade --test geometry_gpu --locked --offline
RHAPSODY_REQUIRE_VULKAN=1 cargo test -p tiny-glade --test geometry_gpu --locked --offline -- --nocapture
```

最终结果、保留的失败日志和当前源码指纹见 [重构报告](../../../../reports/geometry-refactor-2026-10-18/README.md)。原审查的 G1/G2 状态见 [REVIEW.md](REVIEW.md)。历史报告仅记录当时的实现；本次改变了输入、拥有权和错误处理，不沿用旧原作逐位等价结论。

后续缓冲生命周期修复及重新验证见 [生命周期修复报告](../../../../reports/geometry-lifecycle-fix-2026-10-18/README.md)；该报告单独记录先前因磁盘耗尽未执行的 GPU 检查的重跑结果。

本模块提供几何与实例缓冲的生产消费入口。全局间接 draw-list/culling、墙代理实体装配、完整游戏场景和最终画面仍属于渲染系统的职责，不能由局部上传或回读测试宣称完成。
