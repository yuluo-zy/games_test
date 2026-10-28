# Tiny Glade 基础库业务功能清单

2026-10-13 B八库实现已进入共享workspace，具体手写规则和未接消费者已更新；theme-server-proto仍按数据契约记录。414项公共依赖身份与原B锁一致。详见[本轮交付](../../evidence/foundation-unstarted-2026-10-13/REPORT.md)。C按用户指示后置。

## 构建与测试

交付范围：`asset`、`asset-pipe`、`paths`、`lang-lib`、`theme-defs`、`tabloid`、`utils`。源码位于 `reconstruction/shared-foundation/crates/`，按 B 版原作业务规则恢复，接口采用可审查的 Rust 形式，不宣称作者源码或原 Rust ABI；通用库使用原锁定版本。

在 `reconstruction/shared-foundation` 执行：

```bash
cargo test --locked --offline -p asset -p asset-pipe -p paths -p lang-lib -p theme-defs -p tabloid -p utils
cargo clippy --locked --offline -p asset -p asset-pipe -p paths -p lang-lib -p theme-defs -p tabloid -p utils --all-targets -- -D warnings
```

`offline` 要求依赖已缓存；首次获取依赖时可省略。两个 `shipped_*` 测试需要本机 B 版发行目录（Mac：`/Users/yuluo/Desktop/game/B版本内容`；Windows 旧记法：`D:/game/ljxsj_92385/Tiny Glade`），原作文件不随 Git 发布。已采集的原指令对照 fixture 位于 `evidence/`，属于测试输入，需与源码一起检出。

当前功能与剩余限制以本清单和 `evidence/foundation-review-fixes-2026-10-14/REPORT.md` 为准；[业务功能对照](../reference/FOUNDATION_BUSINESS_COMPARISON.md) 是 2026-10-14 的补全前快照，仅作差异计数参考。完整游戏调用方接线不属于七库补全范围。

本清单按作者在游戏侧实现的规则、组合策略和生产消费流程划分功能。主体是共享基础、存档、资产、UI、主题、shader与渲染库；B为主要基线，A/C按各自证据登记。建筑、地形、动物等country-core和systems业务系统另列，不在本基础库清单展开。

每一行是一个可继续分析或联接的功能闭包，可以对应多个函数。已有业务证据、仅定位入口、数据契约与公共依赖边界分别标记；不能把所有行解释成已证明由作者手写的算法，也不能按行数计算完成率。

## 功能纳入规则

- 纳入：游戏的参数选择、种子组合、状态转移、缓存失效、资源关系、数据布局选择、默认/继承/兼容规则，以及有定义归属证据的私有数值和几何算法。
- 排除：Clone/Copy/Eq/Hash/Debug/Default派生、drop_in_place、容器自动析构、Serde/RKYV/Speedy自动编解码、生成binding、宏展开、通用库内部算法，以及纯getter/重导出单独计数。
- 特例：手写收尾如果承担实际规则，归入对应功能。例如motomoto结束chunk时回填长度、渲染资源等GPU完成后回收；不另列Clone或Drop任务。
- 类型边界：项目字段和格式是数据契约。只有查明手写默认值、继承、验证、版本迁移或消费者约束，才列相应业务规则；派生visitor不算手写功能。

## 状态说明

| 状态 | 含义 |
|---|---|
| 已有规则或限定实现 | 已有业务分析或限定实现，功能未完整验收 |
| 已分析未实现 | 已有原规则分析，尚无对应实现 |
| 功能已定位 规则待分析 | 原业务入口已定位，规则待分析 |
| 数据契约 | 数据或协议契约，尚未证明存在独立手写逻辑 |
| 来源与私有增量待分类 | 功能入口存在，公共来源与私有增量待分类 |
| 依赖复用 | 直接复用依赖，仅恢复游戏配置或调用策略 |
| 仅包身份 待定位 | 仅有原包身份，功能待定位 |

“功能已定位”只说明原符号、源路径或消费者指出了该范围，不保证具体算法、手写来源、字段或运行行为已经确认。“来源待分类”的库必须先排除公共实现。所有功能的完整原作验收状态仍为未登记，本清单不提升历史进度。

本次按原锁补查B的52个local条目、C的53个local条目，排除各版15个游戏主体或系统包后，基础包无遗漏；另外登记C固定Git来源中的渲染和shader附属包。原锁不包含的潜在功能、PDB未观察接口及全部第三方库不在完备性声明内。

## 按库索引

| 库 | 作用 | 功能或定位条目 |
|---|---|
| [utils](#utils) | 游戏自有坐标、曲线、几何、数值与状态辅助策略 | 58 |
| [tabloid](#tabloid) | 把游戏增量状态分发给多个消费者 | 6 |
| [paths](#paths) | 游戏账号、存档和安装目录策略 | 5 |
| [lang-lib](#lang-lib) | 游戏语言选择、回退和缺失文案策略 | 7 |
| [motomoto](#motomoto) | 游戏快照的chunk容器协议 | 7 |
| [tg-package-integrity](#tg-package-integrity) | 验证发行文件是否缺失、损坏或多出 | 7 |
| [tg-integrity-report](#tg-integrity-report) | 将完整性问题转换成玩家可操作的报告 | 5 |
| [tg-iced](#tg-iced) | 游戏专用对话框与独立GUI运行包装 | 5 |
| [tg-gpu-selector](#tg-gpu-selector) | 游戏GPU选择、初始化错误与Vulkan配置界面 | 5 |
| [crash-reporter-gui](#crash-reporter-gui) | 游戏崩溃后的玩家交互流程 | 5 |
| [feedback-reporter](#feedback-reporter) | 玩家反馈的数据组织和发送 | 5 |
| [tg-text](#tg-text) | 文字片段、字形缓存及UI形状编码策略 | 9 |
| [theme-defs](#theme-defs) | 供主题下载和每日主题消费者共享的数据契约 | 2 |
| [theme-server-proto](#theme-server-proto) | 主题服务响应的项目协议契约 | 2 |
| [theme-fetcher](#theme-fetcher) | 获取活动、下载主题并维护内容缓存 | 7 |
| [geforcenow](#geforcenow) | 游戏是否运行在云端环境的判断 | 2 |
| [usdl](#usdl) | C的SDL平台安全包装，来源待分类 | 3 |
| [usdl-sys](#usdl-sys) | C的SDL底层FFI与动态装载包装 | 2 |
| [aftermath-rs](#aftermath-rs) | NVIDIA GPU崩溃诊断接口包装 | 3 |
| [dxgi-enumerate-gpus](#dxgi-enumerate-gpus) | Windows显卡枚举包装 | 2 |
| [gpu-profiler](#gpu-profiler) | 游戏GPU时间标记与性能结果包装 | 3 |
| [asset](#asset) | 游戏资源描述、继承、发现与纹理交换格式 | 10 |
| [asset-pipe](#asset-pipe) | 把游戏图像描述生产成纹理资产 | 10 |
| [asset-pipe-bin](#asset-pipe-bin) | 原资产生产命令行入口 | 2 |
| [rhapsody](#rhapsody) | 游戏自己的Vulkan资源、图执行和GPU生命周期组合策略 | 21 |
| [shader-pipeline](#shader-pipeline) | 原shader产物格式、反射数据和pipeline描述 | 6 |
| [shader-compiler](#shader-compiler) | 组合原公开库和DXC生成游戏shader资产 | 8 |
| [shader-variants](#shader-variants) | 项目自己的raster宏变体策略 | 2 |
| [build-id](#build-id) | 原构建身份配套包 | 1 |
| [state-stream](#state-stream) | 原锁中的状态配套包，具体用途待定位 | 1 |
| [tg-mods](#tg-mods) | 发现Mod、维护身份并形成游戏资源覆盖层 | 10 |
| [clutter-icon-pipe](#clutter-icon-pipe) | 由杂物mesh与材质烘焙游戏图标 | 7 |
| [clutter-pack-pipe](#clutter-pack-pipe) | 导入杂物资产并生产资源包和预览 | 8 |
| [asset-pipe-build](#asset-pipe-build) | C新增资产构建配套包 | 2 |
| [foliage-pipe](#foliage-pipe) | C植被资产生产配套包 | 1 |
| [grass2-shader](#grass2-shader) | C草地shader配套包 | 1 |
| [frozen](#frozen) | 原资产库使用的FrozenMap包装 | 1 |
| [leekspin](#leekspin) | 诊断输出和环形记录包装 | 2 |
| [conservative-line-raster](#conservative-line-raster) | 世界笔画到保守栅格覆盖的接口 | 3 |
| [turbosloth](#turbosloth) | C资产构建依赖、缓存与打包框架 | 9 |
| [turbosloth-macros](#turbosloth-macros) | C资产框架宏生成配套 | 1 |
| [gpu-allocator](#gpu-allocator) | Vulkan内存分配依赖 | 1 |
| [vk-sync-fork](#vk-sync-fork) | 原Vulkan同步分类与屏障依赖 | 2 |
| [kajiya](#kajiya) | 原锁中的渲染配套包，游戏使用边界待核实 | 1 |
| [sdl2](#sdl2) | C的SDL窗口依赖 | 1 |
| [FidelityFX](#fidelityfx) | AMD渲染算法SDK | 1 |
| [XeGTAO](#xegtao) | 环境遮蔽依赖 | 1 |
| [GPU-Raytracer](#gpu-raytracer) | GPU几何查询或光追配套依赖 | 1 |
| [rhapsody-core](#rhapsody-core) | C渲染共享view与图像归约策略 | 3 |
| [rhapsody-dof](#rhapsody-dof) | C景深pass包装 | 1 |
| [rhapsody-gi](#rhapsody-gi) | C环境遮蔽与间接光pass包装 | 2 |
| [rhapsody-post](#rhapsody-post) | C后处理模糊金字塔 | 2 |
| [rhapsody-shadow](#rhapsody-shadow) | C方向光阴影pass包装 | 2 |
| [rhapsody-taa](#rhapsody-taa) | C时域抗锯齿与TGSR包装 | 3 |
| [rhapsody-deferred](#rhapsody-deferred) | C渲染仓库中仅锁定身份的包 | 1 |
| [shaderface](#shaderface) | C shader数据、变体身份与binding描述 | 4 |
| [shabriri-rt](#shabriri-rt) | C生成shader接口的运行时binding与编译包装 | 3 |
| [shabriri](#shabriri) | C shader生成配套包，仅主包身份明确 | 1 |

## 共享 CPU 基础

### utils

游戏自有坐标、曲线、几何、数值与状态辅助策略。版本范围：A、B、C。

边界：glam/half/fastrand/serde/std/xxhash等实现直接复用。自动派生与纯getter不单列功能；不同维度与泛型规则不能用Curve2证据替代。

#### 数值 状态与墙空间

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 角度限制 | 按最小角和跨度构造可跨周期的角度范围，限制编辑角度 | 已有规则或限定实现 | 独立wrap及极端端点策略未全部分析 |
| 周期数值与插值 | 归一化周期值，计算环形距离并沿环形路径插值 | 已有规则或限定实现 | 构造归一化、半周期平局及非有限值边界 |
| 相机投影 | 由镜头、长宽比和相机位置旋转生成投影与视图关系 | 已有规则或限定实现 | 完整相机资源组装与消费者 |
| 相机朝向投影 | 提取眼睛位置、前方、右方及XZ平面朝向，处理近垂直回退 | 已有规则或限定实现 | 完整模式切换及输入在country-core侧 |
| 镜头变化 | 设置垂直视野；镜头平滑与更新入口尚待联接 | 已有规则或限定实现 | 平滑/update生产者未恢复 |
| 空间投影与区域关系 | 在线性或循环空间计算距离、投影、夹紧、区间减法 | 已有规则或限定实现 | classify_side、regions_intersect及零/负周期 |
| 缓动与偏置 | 按游戏枚举选择插值曲线，对过渡进度施加缓动 | 已有规则或限定实现 | 其余已观察缓动族、越界和NaN输入 |
| 带时间的备份 | 按日期格式生成保留原名的备份路径并复制文件 | 已有规则或限定实现 | IO失败、非Unicode路径和日志策略 |
| 地面网格采样 | 把世界XZ映射至网格并做双线性采样，支持已观察采样器 | 已有规则或限定实现 | 固定/动态网格标志、边界外推与真实资源 |
| 半精度数据包装 | 将游戏向量/数组按既定分量转换成f16用于紧凑数据 | 已有规则或限定实现 | 仅恢复打包组合；half算法复用，完整生产者待接 |
| 对象索引压缩 | 为对象key维护紧凑索引，删除时调整索引并支持缓存解析 | 已有规则或限定实现 | insert生产策略、GPUWorld与启动消费者 |
| 本地时间缓存 | 缓存平台时区偏移，初始化失败使用UTC，再转换时间 | 已有规则或限定实现 | 实际刷新频率和平台失败流程 |
| 循环索引与切片 | 把有符号索引映射到周期，遍历直线/闭环范围及子切片 | 已有规则或限定实现 | LoopSlice、反向遍历与异常周期规则 |
| 资产名称标识 | 按原xxhash调用生成名称key并提供调试名称策略 | 已有规则或限定实现 | 真实名称生产者与冲突/查找消费者；hash算法复用 |
| 自然样条构造 | 由2D/3D控制点和可选端点切线生成样条系数 | 已有规则或限定实现 | 重复点、少点、非有限与极端规模 |
| 周期样条构造 | 构造闭合样条的节点及周期边界条件 | 已有规则或限定实现 | 求解器剩余入口和退化周期输入 |
| 样条位置和切线 | 寻找采样区段，夹紧参数，计算曲线位置及切线 | 已有规则或限定实现 | 全部区段/端点和真实编辑消费者 |
| 轮廓简化 | 按RDP阈值选择保留点索引，保留已观察几何规则 | 已有规则或限定实现 | stitch_with、索引消费及调用者非负阈值约束 |
| 颜色紧凑编码 | 按游戏使用的RGB9E5范围和共享指数规则打包颜色 | 已有规则或限定实现 | 非有限值和实际GPU字段；不扩成通用颜色库 |
| 随机种子策略 | 变换游戏输入种子并调用原fastrand初始化RNG | 已有规则或限定实现 | 各生成消费者种子来源；不恢复RNG实现 |
| 快照字段选择 | 只提取History内snapshots等所需字段，保留部分成功与吞错策略 | 已有规则或限定实现 | 真实Snapshot类型及原完整visitor |
| 一维区间裁剪 | 从范围扣除一个或多个区段，输出剩余有序范围 | 已有规则或限定实现 | 连续多次裁剪、反向/非有限范围 |
| 坐标分量适配 | 把XZ等平面分量放入指定3D轴，并桥接借用切片 | 已有规则或限定实现 | Vec3SliceAs2投影轴、步长及真实借用消费者 |
| 阈值区段提取 | 从样本寻找满足阈值的区段并按原策略保留回调结果 | 已有规则或限定实现 | NaN singleton与回调完整原入口 |
| 帧时间过滤 | 按warmup和历史样本过滤墙钟delta，控制初始帧输出 | 已有规则或限定实现 | Bevy Time/SystemParam实际联接 |
| 墙曲线坐标 | 在曲线参数、墙体沿线距离和世界XZ间映射，叠加高度 | 已有规则或限定实现 | nearest-u、外推及真实墙/装饰消费者 |
| 墙空间偏移与范围 | 沿墙限制或放开偏移，判断墙空间区域相交并形成变换 | 功能已定位 规则待分析 | bound、offset、region和to_affine3规则 |

#### 曲线与曲线区段

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 曲线合法化 | 构造2D/3D曲线，处理重复点、无效点列和闭合条件 | 已有规则或限定实现 | remove_duplicates全规则及Curve3/Thresholded3D实例 |
| 曲线长度与分段 | 累计沿线距离，建立点/区段参数与内部坐标映射 | 已有规则或限定实现 | 全部泛型度量策略与段边界 |
| 曲线位置和方向 | 在参数或内部坐标采样位置、切线与中心切线 | 已有规则或限定实现 | 未归一切线、非受控采样及泛型实例 |
| 曲线最近点 | 从世界位置找到近似曲线参数或投影位置 | 已有规则或限定实现 | Curve3查询策略与完整几何消费者 |
| 曲线侧向判断 | 按最近区段与叉积判断点位于曲线哪一侧 | 已分析未实现 | 尚无原映射实现；不同维度策略需分开 |
| 曲线裁切 | 按区间裁切曲线，支持循环裁切与写入目标点列 | 已有规则或限定实现 | 全部跨接缝和失败路径 |
| 曲线区段视图 | 创建CurveSlice，测量区段并向两端扩展至边界 | 功能已定位 规则待分析 | 17个已观察区段接口具体业务仍未闭合 |
| 曲线重映射 | 重新映射曲线区段的坐标与二维参数 | 功能已定位 规则待分析 | CurveSlice remap/remap_2d与消费者 |
| 曲线反向与延长 | 反向点列并重建参数；在末尾扩展新点 | 已有规则或限定实现 | 反向已有实现，try_extend已分析但未实现 |
| 按间距重采样 | 按空间间距或区段规则重新生成曲线点列 | 功能已定位 规则待分析 | try_resample与各remap/per-segment策略 |
| 按点数重采样 | 用目标近似点数确定间距，处理特征点和终点 | 已分析未实现 | 最终算法与所有remap变体未实现 |
| 曲线平滑与移动 | 对点列平滑、加权平滑、原位平滑或整体平移 | 功能已定位 规则待分析 | 平滑权重、闭合端点及实际调用约束 |
| 曲线维度投影 | 在Curve2、Curve3、Curve3XZ间转换并选用XZ度量 | 功能已定位 规则待分析 | 转换、reinterpret及阈值度量完整规则 |
| 曲线循环参数 | 对闭环参数加减、距离、包含和跨接缝插值 | 已有规则或限定实现 | 普通CurveU夹紧/min/max和实例边界 |

#### 对象变换

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 变换朝向与比较 | 游戏Transform的look-at和容差比较，数学调用glam | 已有规则或限定实现 | 真实组件消费者 |
| 变换矩阵组装 | 从位移/旋转/缩放形成矩阵、仿射变换和前方向 | 功能已定位 规则待分析 | 手写组合与生成注册胶水的归属区分 |

#### 几何 查询与坐标映射

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 包围盒查询 | 构造/扩展包围盒，检测包含、相交以及射线/线段命中 | 已有规则或限定实现 | from_points和线段交叠剩余入口 |
| 圆形轮廓采样 | 计算周长/面积/参数点、切线、法线并离散为点列 | 已有规则或限定实现 | Circle2d lerp、旋转与全采样边界 |
| 圆形几何查询 | 判断点内外、带符号距离、最近周界点、射线/圆相交 | 已有规则或限定实现 | 全部退化与调用链 |
| 圆形转曲线 | 把圆形或圆弧范围变成Curve2/Curve3 | 已分析未实现 | as_curve2已分析未实现；闭合末段及其他变体待确认 |
| 矩形轮廓采样 | 把带朝向矩形转点列/曲线，按周界参数找边和点 | 已有规则或限定实现 | 圆周区段裁切、Curve3及转变换 |
| 矩形内外与交叠 | 计算包含、padding和二维/三维矩形交叠 | 已有规则或限定实现 | 矩形/圆混合相交、退化输入 |
| 矩形边缘距离 | 求最近边界距离及内外符号 | 已分析未实现 | 普通/有符号距离均尚未实现 |
| 矩形边角法线 | 按边和角计算朝向，特殊角点使用未归一法线和 | 已有规则或限定实现 | 特殊角点法线已分析未实现，其余最近边策略待确认 |
| 矩形变形 | 扩大尺寸、沿特定边扩展，设置3D尺寸、平移和XZ朝向 | 已有规则或限定实现 | expand_along_edge与rotate_projective_xz规则 |
| 射线和曲面拾取 | 计算射线与球/三角形/平面交点和最近交距 | 已有规则或限定实现 | 鼠标屏幕坐标生成Ray及完整拾取消费者 |
| 三角形交叠 | 分类三角形关系，计算交线与有序端点，处理退化及容差 | 已有规则或限定实现 | 内层剩余入口与真实几何消费链 |
| 网格UV位置 | 用重心坐标在三角形UV与空间位置之间映射 | 已有规则或限定实现 | get_triangle_coords_from_mesh_uv与barycentric2 |
| 高度图阈值切分 | 按高度阈值将高度图划成不同区域 | 已有规则或限定实现 | 全资源消费者与边界输入 |
| 屏幕坐标转换 | 从世界位置转换裁剪空间和屏幕空间 | 功能已定位 规则待分析 | ws_to_cs/ws_to_ss原规则与调用者 |
| 几何支撑辅助 | brace.rs仅有定义来源，具体几何业务尚待定位 | 功能已定位 规则待分析 | 不能按文件名补造支撑类型及算法 |

依据：[tracking/shared-foundation-interfaces.json](../../tracking/shared-foundation-interfaces.json)；[evidence/shared-foundation-2026-10-13/utils-base/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-base/CONTRACT.md)；[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### tabloid

把游戏增量状态分发给多个消费者。版本范围：A、B、C。

边界：Arc/Mutex/RwLock/Vec由原依赖实现；Clone/Default/Deref与容器析构不是功能条目。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 发布增量 | 每次发布推进序号，将新delta合并进仍被订阅者或reader持有的批次 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 慢消费者累积 | 消费者已有pending时保留原批次并继续累积变化，不覆盖成最新单值 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 新订阅起点 | 新订阅只接后续变化，不补历史状态 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 取走待处理批次 | 消费者一次取走pending并记录已读序号 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 读写同步策略 | 持有reader时后续发布等待；无外部持有的批次才清除 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 游戏状态接入 | 对接DirtyTiles和ColliderDiff的empty/apply_to语义 | 已有规则或限定实现 | 实际生产者和消费者未接线 |

依据：[evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### paths

游戏账号、存档和安装目录策略。版本范围：A、B、C。

边界：其他平台分支未核实；std文件系统与KnownFolder接口复用。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 用户数据根目录 | Windows优先SavedGames已知目录，失败读USERPROFILE并追加Saved Games/Tiny Glade | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 覆盖目录与账号隔离 | 显式根目录也追加Tiny Glade，再按Steam用户ID隔离账号 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 存档目录布局 | 统一saves、beta-saves、tmp、deleted-saves、backup-saves及账号配置目录 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 启动目录初始化 | 按原顺序创建目录，后续失败保留先前已创建目录并传播上下文 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 安装位置与初始化限制 | 初始化后记录当前工作目录；未初始化访问和重复初始化失败 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |

依据：[evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md](../../evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### lang-lib

游戏语言选择、回退和缺失文案策略。版本范围：A、B、C。

边界：YAML解析器复用；UI/ECS初始化和完整热切换消费者未验收。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 语言包加载 | 读取assets/lang/{locale}.yml的首个YAML文档，按字符串映射组织文案 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 语言切换 | 替换所选语言，保留已返回文本引用的有效性 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 默认语言回退 | 请求语言加载失败使用en-US；环境变量可禁用回退更新 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 文案查找 | 先查所选语言再查默认语言，未初始化时按原策略处理key | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 缺失文案缓存 | 首次缺失生成稳定占位文案并记录日志，重复缺失使用缓存 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 语言偏好保存 | 持久化请求locale，读取失败使用en-US；保留文件文本不trim | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 加载失败分类 | 区分文件/扫描错误、内容schema不匹配及默认包加载失败 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |

依据：[evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md](../../evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md)；[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 存档与完整性

### motomoto

游戏快照的chunk容器协议。版本范围：A、B、C。

batch-02（2026-10-13/14）：库内业务逻辑全部实现并反编译验证（add_chunk/Drop/iter/next/as_tag/tag 编码 + 173 原 Reader RNE 案例）；未闭合仅存档消费者接线与 A/C。[证据](../../evidence/small-libs-batch-02-2026-10-13/motomoto)。

边界：chunk封口包含手写Drop承载的协议规则，但不列Drop任务；byteorder/std IO/flate2/JSON实现复用。

最新review已补限定Cursor的原Writer和chunk封口运行对照。通用流IO失败、最终延迟错误消费者和真实存档仍未闭合。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 资源段标识 | 把字符串原UTF8前8字节按LE组成tag；严格/截断两种长度策略 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 连续chunk写入 | 写tag和长度占位，再原样写入payload，不自行压缩 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| chunk封口 | 结束段写入后回填实际payload长度，再恢复流末尾位置 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 延迟写入错误 | 先消费前段封口错误，错误时不转换新tag或写入新段 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| chunk迭代读取 | 按header跳过payload并返回借用数据；保留原末尾16字节边界 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 损坏段处理 | 长度越界返回错误并保留已更新的游标，异常长度按原seek规则处理 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 游戏快照接线 | Snapshot、动态资源tag、Terrain由存档消费者生产，读取侧交给解压/schema | 已有规则或限定实现 | 完整capture/load消费者、迁移与压缩设置待恢复 |

依据：[evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md](../../evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md)；[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### tg-package-integrity

验证发行文件是否缺失、损坏或多出。版本范围：A、B、C。

batch-02（2026-10-13/14）：库内业务逻辑全部实现并反编译验证（校验门/verify 链/extra 扫描 + 29 容器案例 + 真实 2.7GB 包 + symlink/权限探针）；登记报告/崩溃 GUI 消费者接线；未闭合仅消费者接线与 A/C。[证据](../../evidence/small-libs-batch-02-2026-10-13/tg-package-integrity)。

边界：复用blake3/fnv/relative-path/easy-parallel/serde；不重写哈希和线程库。

最新review补了原x64 Hash组合门禁、跨批与部分Windows文件边界。完整verify、罕见schema和报告消费者仍未验收；线程创建和整文件读取成本保留。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 清单原字节门禁 | JSON解析前校验BLAKE3后接指定FNV/数组Hash规则，空格也覆盖 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 清单版本门禁 | 读取version/files，只接受已观察版本1 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 逐文件完整性 | 读取每个清单条目，按精确小写hash文本比较；读取失败归缺失/不可读 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 清单自身排除 | 只精确跳过manifest自身逐文件校验，不宽泛按名字跳过 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 额外文件发现 | 递归扫描清单外条目，仅忽略根tmp及原已观察的symlink/失败条目 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 问题汇总 | 重复清单条目保留，汇总missing/corrupt/extra，三类全空才成功 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 完整性结果接线 | 向完整性报告和崩溃报告消费者传递结果 | 已有规则或限定实现 | 原完整入口、报告消费者和特殊OS边界 |

依据：[evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md](../../evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md)；[evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 启动与报告界面

### tg-integrity-report
| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 报告结果整理 | 三列表均空才归一成功，保留输入顺序 | 已有规则或限定实现 | 完整verify原入口的真实消费者 |
| 验证状态展示 | missing/corrupt/extra映射到状态与本地化section，真实窗口适配 | 已有规则或限定实现 | 精确原样式/罕见输入 |
| 修复建议 | 固定Steam完整性建议链接与对应文案key | 已有规则或限定实现 | 完整平台/调用者上下文 |
| 删除额外包文件 | 单次unlink；成功只删同名extra，失败保留且不重新校验 | 已有规则或限定实现 | 真实删除消费者未执行；root限制为重建接入保护 |
| 报告界面交互 | update/outcome与crash内嵌报告联接，真实原iced独立窗口 | 已有规则或限定实现 | 完整原启动链及像素等价 |

本轮B候选证据：[manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json)；[validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)。A/C不升级，完整原作功能未验收。


将完整性问题转换成玩家可操作的报告。版本范围：A、B、C。

边界：仅登记原业务入口，不执行删除或发送动作；iced实现复用。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 报告结果整理 | normalize_verification_result入口整理供界面展示的结果 | 功能已定位 规则待分析 | 具体排序/分组与错误规则 |
| 验证状态展示 | verification_status和view展示当前校验状态 | 功能已定位 规则待分析 | 状态到文案和界面映射 |
| 修复建议 | advice根据问题提供建议 | 功能已定位 规则待分析 | 各类问题的建议与链接 |
| 删除额外包文件 | remove_unrecognized_package_file处理用户删除操作 | 功能已定位 规则待分析 | 允许路径、失败提示及重新校验顺序 |
| 报告界面交互 | update与run_integrity_report_gui组织界面与操作 | 功能已定位 规则待分析 | 真实启动消费者及最终结果 |

依据：[skeleton/B/crates/tg-integrity-report/src/controls.rs](../../skeleton/B/crates/tg-integrity-report/src/controls.rs)；[skeleton/B/crates/tg-integrity-report/src/lib.rs](../../skeleton/B/crates/tg-integrity-report/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### tg-iced
| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 消息对话框 | 手写level/按钮模式、标题/说明替换和关闭结果；原叶对照，原iced窗口适配 | 已有规则或限定实现 | 原图标/精确样式与所有原调用者 |
| 界面状态更新 | Controls update与ProgramWithOutcome闭合到原iced窗口关闭 | 已有规则或限定实现 | 完整原GUI事件链及真实游戏调用者 |
| GUI运行选项 | 真实Application、命令/结果/Tick联接；16ms频率为重建选择 | 已有规则或限定实现 | 原窗口默认值/调度频率与全run选项 |
| 游戏文本包装 | 文本key交给lang-lib原本地化策略，standalone只读加载locale | 已有规则或限定实现 | 原文本widget字体/样式完整映射 |
| 窗口与事件后端 | B原iced/winit/cosmic-text fork后端；窗口探针可运行 | 已有规则或限定实现 | C mouse-button/event-batch后置，B精确像素未验收 |

本轮B候选证据：[manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json)；[validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)。A/C不升级，完整原作功能未验收。


游戏专用对话框与独立GUI运行包装。版本范围：A、B、C。

边界：iced/winit/SDL引擎复用；不将控件库、事件循环库整体纳入逆向。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 消息对话框 | 按级别、标题、说明构建并显示对话框 | 功能已定位 规则待分析 | 级别/按钮/关闭结果完整规则 |
| 界面状态更新 | view/update/program_outcome把玩家操作转成退出或继续结果 | 功能已定位 规则待分析 | 各Controls的状态机 |
| GUI运行选项 | run_gui_with_options组织独立窗口运行及回调 | 功能已定位 规则待分析 | 窗口配置、关闭、错误与启动调用者 |
| 游戏文本包装 | tg_iced::text提供项目自己的文本入口 | 功能已定位 规则待分析 | 本地化/样式调用策略 |
| 窗口与事件后端 | B的GUI runner以及C的winit/SDL runner包装平台事件 | 功能已定位 规则待分析 | C mouse-button映射和event-batch规则须独立分析 |

依据：[skeleton/B/crates/tg-iced/src/dialog.rs](../../skeleton/B/crates/tg-iced/src/dialog.rs)；[skeleton/B/crates/tg-iced/src/gui_runner.rs](../../skeleton/B/crates/tg-iced/src/gui_runner.rs)；[skeleton/C/crates/tg-iced/src/gui_runner_sdl.rs](../../skeleton/C/crates/tg-iced/src/gui_runner_sdl.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### tg-gpu-selector
| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 显卡选择界面 | 选择Option索引、更多信息/取消/outcome及真实原iced窗口 | 已有规则或限定实现 | 真实设备列表生产者和所有原reason标签 |
| 驱动更新建议 | 原vendor表及条件建议/链接，classification由rhapsody边界输入 | 已有规则或限定实现 | 真实driver_support生产者 |
| 初始化错误说明 | raw图形错误族与建议override、本地化key恢复 | 已有规则或限定实现 | 真实Vulkan失败消费者与完整reason语义 |
| Vulkan白名单配置 | layer/vendor tag去留切换，重复项一起移除，Apply快照 | 已有规则或限定实现 | opaque u32原schema及实际device创建消费 |
| 兼容措施配置 | 原六bool措施映射、标签和Apply/Cancel结果 | 已有规则或限定实现 | 真实render初始化消费及持久化策略 |

本轮B候选证据：[manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json)；[validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)。A/C不升级，完整原作功能未验收。


游戏GPU选择、初始化错误与Vulkan配置界面。版本范围：A、B、C。

边界：该包原私有UI入口已观察，整体来源仍待核实；只恢复游戏选择与提示策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 显卡选择界面 | 运行GPU选择GUI并将选择或退出作为program_outcome | 功能已定位 规则待分析 | 设备列表输入、默认选择和确认行为 |
| 驱动更新建议 | 按设备/厂商给出driver advice和更新链接 | 功能已定位 规则待分析 | 具体设备条件与厂商映射 |
| 初始化错误说明 | 把图形初始化错误本地化并转换成界面信息 | 功能已定位 规则待分析 | 错误族、建议与fallback规则 |
| Vulkan白名单配置 | 展示并切换allowlist项 | 功能已定位 规则待分析 | 键、持久化和对创建策略的影响 |
| 兼容措施配置 | 展示workarounds并处理操作 | 功能已定位 规则待分析 | 措施映射和真实render初始化消费者 |

依据：[skeleton/B/crates/tg-gpu-selector/src/gpu_selection.rs](../../skeleton/B/crates/tg-gpu-selector/src/gpu_selection.rs)；[skeleton/B/crates/tg-gpu-selector/src/lib.rs](../../skeleton/B/crates/tg-gpu-selector/src/lib.rs)；[skeleton/B/crates/tg-gpu-selector/src/vulkan_config.rs](../../skeleton/B/crates/tg-gpu-selector/src/vulkan_config.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### crash-reporter-gui
| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 崩溃说明展示 | 显卡driver帧/日志、前8帧OBS/FPS、Vulkan loader与integrity优先级 | 已有规则或限定实现 | 原真实crash producer及像素/tooltip |
| 更多诊断信息 | more_info是建议URL；ShowDetails独立切换 | 已有规则或限定实现 | 原外部网页实际动作未执行 |
| 复制诊断信息 | 复制传入诊断文本；成功提示严格<1秒，错误持续显示 | 已有规则或限定实现 | 剪贴板OS动作未执行 |
| 报告交互状态 | Sending不重复回调；Tick检查is_finished后join，状态/outcome与原iced联接 | 已有规则或限定实现 | 原全GUI事件/失败payload ABI与调度频率 |
| 崩溃回调联接 | 一次callback创建真实worker；内嵌integrity删除→状态链测试 | 已有规则或限定实现 | 原panic采集/服务发送生产者未接入 |

本轮B候选证据：[manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json)；[validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)。A/C不升级，完整原作功能未验收。


游戏崩溃后的玩家交互流程。版本范围：A、B、C。

边界：GUI只提供交互；实际报告采集/发送由真实callback和其他包消费，不按常见崩溃工具补造。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 崩溃说明展示 | main_view/view显示崩溃信息与操作入口 | 功能已定位 规则待分析 | 字段、状态与本地化文案 |
| 更多诊断信息 | more_info_button提供进一步信息入口 | 功能已定位 规则待分析 | 展开或外部链接的实际动作 |
| 复制诊断信息 | copy_to_clipboard处理玩家复制操作 | 功能已定位 规则待分析 | 具体内容和平台失败路径 |
| 报告交互状态 | update/tick_message处理运行中的事件和结果 | 功能已定位 规则待分析 | 发送进行中/完成/失败及退出条件待沿原状态确认 |
| 崩溃回调联接 | run_crash_reporter_gui接入真实panic reporter回调 | 功能已定位 规则待分析 | mock与真实崩溃路径区别 |

依据：[skeleton/B/crates/crash-reporter-gui/src/lib.rs](../../skeleton/B/crates/crash-reporter-gui/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### feedback-reporter
| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 反馈内容组织 | text/meta与可选screenshot/save/benchmark按原顺序/MIME组织 | 已有规则或限定实现 | 原全部字段名/ABI与实际上下文生产者 |
| 目录附件打包 | 只打包指定相对文件；missing跳过，目录原panic；root文件名前缀/0755/Deflated | 已有规则或限定实现 | 完整原zip入口及极端IO/nonUnicode边界 |
| 性能资料打包 | benchmark.puffin/info.txt固定名；失败非致命省略附件 | 已有规则或限定实现 | 实际benchmark路径/字段生产者 |
| 发送反馈 | connect_timeout5秒，发送失败/非2xx body/读失败规则；真实reqwest adapter | 已有规则或限定实现 | 完整原发送入口与真实服务未执行 |
| 游戏入口联接 | Bevy command→Local pending替换/两文件或计数>=601→SendingFinished；保留注入provider | 已有规则或限定实现 | country-core/UI实际producer未接入 |

本轮B候选证据：[delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json)；[validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json)。A/C不升级，完整原作功能未验收。


玩家反馈的数据组织和发送。版本范围：A、B、C。

边界：HTTP和ZIP库复用；此次仅制作清单，不发送报告。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 反馈内容组织 | report_player_feedback和build组织反馈请求 | 功能已定位 规则待分析 | 文本/上下文/附件实际字段 |
| 目录附件打包 | zip_dir将选定目录放入反馈附件 | 功能已定位 规则待分析 | 包含排除规则、相对路径和大小限制 |
| 性能资料打包 | zip_benchmark_data组织benchmark数据 | 功能已定位 规则待分析 | 实际文件选择及格式 |
| 发送反馈 | send_report提交报告并处理请求结果 | 功能已定位 规则待分析 | 服务目标、重试/超时/失败规则 |
| 游戏入口联接 | 区分玩家反馈与其他报告调用者及用户操作时序 | 功能已定位 规则待分析 | 实际UI/回调生产者 |

依据：[skeleton/B/crates/feedback-reporter/src/lib.rs](../../skeleton/B/crates/feedback-reporter/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 文字与界面

### tg-text

文字片段、字形缓存及UI形状编码策略。版本范围：A、B、C。

batch-02（2026-10-13/14）：库内业务函数全部实现，核心算法反编译验证（rect/prepare/render/scaled/set_internal_scaled_metrics/replace_editor/take_editor/get_glyph/upload_pending_glyphs/consume/append + 51+21+20+88 原 RNE 案例）；cosmic::Action 确认 32 字节/扁平标签 0x0–0x21；保留项——简单 setter 未逐一 fresh 复核、render 返回类型为重建扩展、TextRenderer::new 装配细节、消费者接线与 A/C。[证据](../../evidence/small-libs-batch-02-2026-10-13/tg-text)。

边界：cosmic-text/etagere/serde/RON复用；前三个字体/流子域有证据，完整文本渲染未验收。

最新review确认已有CPU规则，并纠正历史文档的字符串反序列化标签误读；实现复用String::deserialize。TextFragment、TextRenderer和真实字形/GPU消费链仍缺。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 按UI用途选字体 | Label/MenuButton/TextEditor/Hand按角色查字体；缺失不自行换另一用途 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 字体度量缩放 | 按表scale缩放两项度量，缺省scale为1，不夹紧数值 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| UI形状指令构建 | 按调用顺序累积填充色、图片和圆角描述 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 绘制流编码 | 输出u32协议：色值原样、图片索引低16位与缩放模式、圆角0..255 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 图片名称解析 | 按出现顺序向resolver查询名称；纯索引不查询；无图片resolver路径遇名称失败 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 文字片段编辑与布局 | TextFragment维护attrs、buffer、度量和重排准备条件 | 功能已定位 规则待分析 | 所有权、dirty/redraw和prepare策略 |
| 光标选择与装饰 | 把文字布局run转换成字形/图元及光标、选择等消费者数据 | 功能已定位 规则待分析 | 原具体支持项、bounds与pixels-per-point失效 |
| 字形缓存与图集 | TextRenderer按字形查询图集，跟踪使用与代次，处理空间不足 | 功能已定位 规则待分析 | 原cosmic-text/etagere接线、淘汰规则 |
| 文字GPU生产消费 | 上传字形纹理和绘制数据，联接toolbar与rhapsody | 功能已定位 规则待分析 | 真实资源生产者和渲染消费者 |

依据：[evidence/small-libs-review-2026-10-13/tg-text/REVIEW.md](../../evidence/small-libs-review-2026-10-13/tg-text/REVIEW.md)；[evidence/small-libs-review-2026-10-13/integration/ERRATA.md](../../evidence/small-libs-review-2026-10-13/integration/ERRATA.md)；[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 主题与活动

### theme-defs

供主题下载和每日主题消费者共享的数据契约。版本范围：A、B、C。

边界：已观察符号主要为派生Clone/Eq；不为本包凭空创建活动选择算法。时间窗选择属于theme-fetcher和游戏插件。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 活动可用数据 | 活动数据携带目录及开始/结束SystemTime | 数据契约 | 字段契约不等于独立手写算法 |
| 数据准备状态 | 表示pending、无当前活动及准备失败文本等消费者状态 | 数据契约 | 原变体名/显示措辞及全交换规则 |

依据：[evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md](../../evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### theme-server-proto
| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 服务响应结构 | B ActiveEvent start/end=i64,cid=String，EventsResponse events；派生serde复用 | 数据契约 | 原服务完整兼容规则；无独立手写验证证据 |
| 协议消费者约束 | 时间选择/cache等规则在theme-fetcher消费者，非协议包派生解码器 | 数据契约 | 完整游戏主题消费；无独立迁移算法证据 |

本轮B候选证据：[contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json)；[handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)。A/C不升级，完整原作功能未验收。


主题服务响应的项目协议契约。版本范围：A、B、C。

边界：不列visit_map/visit_str/serde解码器任务；只有证实项目手写验证后才新增业务功能。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 服务响应结构 | 确定主题服务协议字段、可选项和版本兼容要求 | 数据契约 | 当前已观察主要是派生Deserialize，真实字段须沿fetch_events恢复 |
| 协议消费者约束 | 区分服务格式和theme-fetcher活动选择所需业务数据 | 数据契约 | 未证明本包有独立手写验证/迁移逻辑 |

依据：[skeleton/B/crates/theme-server-proto/src/lib.rs](../../skeleton/B/crates/theme-server-proto/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### theme-fetcher
| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 活动列表获取 | 原endpoint、ETag更新顺序、Date偏移、Cache-Control与304保留Result、backoff不因成功重置 | 已有规则或限定实现 | 完整原fetch/run入口oracle |
| 当前活动选择 | 半开时间窗、负时间clamp、重叠最后项获胜、只发布变化结果 | 已有规则或限定实现 | 实际每日主题plugin消费者 |
| 内容下载 | 目录才命中；200/长度/内容cid hash后提取，失败保留到新列表 | 已有规则或限定实现 | 完整原download入口及真实服务 |
| 内容缓存验证 | checksum验证；过期严格>14天，未来时间继续校验；错误条目独立 | 已有规则或限定实现 | 极端文件系统/整链原入口 |
| ZIP提取与验证 | 原enclosed_name/顺序/失败保留文件；完整目标path与hash构成CRC；只验证列出的文件 | 已有规则或限定实现 | 原完整提取/验证入口oracle与路径极端边界 |
| 缓存清理 | rogue文件移除，老/坏checksum目录删除，条目错误独立 | 已有规则或限定实现 | 真实cache大规模行为与特殊OS失败 |
| 主题更新消费 | 库内run下载所有Pending含未来事件；sender只发布变化，断开退出 | 已有规则或限定实现 | 真实tiny-glade主题plugin/ECS consumer |

本轮B候选证据：[contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json)；[handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)。A/C不升级，完整原作功能未验收。


获取活动、下载主题并维护内容缓存。版本范围：A、B、C。

边界：HTTP/ZIP/哈希库复用，恢复下载缓存和活动选择组合策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 活动列表获取 | fetch_events向服务取得活动数据 | 功能已定位 规则待分析 | 请求配置、响应字段、重试与刷新频率 |
| 当前活动选择 | run根据协议时间窗决定当前活动，生产ActiveEvent/状态 | 已有规则或限定实现 | 原已观察半开区间及SystemTime转换；完整编排待分析 |
| 内容下载 | download_blob获取活动内容 | 功能已定位 规则待分析 | 下载失败、重试、大小/校验和缓存命中 |
| 内容缓存验证 | validate_cas_cache_entry验证CAS条目可用性 | 功能已定位 规则待分析 | 缓存身份、过期和不完整条目规则 |
| ZIP提取与验证 | extract_with_checksums和verify_folder_checksums校验提取结果 | 功能已定位 规则待分析 | 条目约束、目录校验和失败回滚 |
| 缓存清理 | trim_cas清理不再需要的内容缓存 | 功能已定位 规则待分析 | 淘汰标准、活动引用及磁盘边界 |
| 主题更新消费 | 向每日主题插件交付数据目录和可用状态 | 已有规则或限定实现 | 完整run时序、取消/错误和插件联接 |

依据：[skeleton/B/crates/theme-fetcher/src/checked_zip.rs](../../skeleton/B/crates/theme-fetcher/src/checked_zip.rs)；[skeleton/B/crates/theme-fetcher/src/lib.rs](../../skeleton/B/crates/theme-fetcher/src/lib.rs)；[evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md](../../evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 平台与诊断

### geforcenow
| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 云端运行判断 | 固定DLL与三导出、init非零原码传播；成功query/shutdown/卸载 | 已有规则或限定实现 | 真实vendor SDK行为、原错误标签与全消费链 |
| 云端分支消费 | 仅库入口已恢复；哪些实际启动/路径/配置消费云判断仍待联接 | 功能已定位 规则待分析 | 原真实consumer未验收 |

本轮B候选证据：[delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json)；[validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json)。A/C不升级，完整原作功能未验收。


游戏是否运行在云端环境的判断。版本范围：B、C。

边界：SDK/平台接口不是游戏自有实现任务。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 云端运行判断 | is_running_in_cloud决定平台环境分支 | 功能已定位 规则待分析 | 具体环境探测与失败回退 |
| 云端分支消费 | 定位哪些启动、路径或配置行为消费该判断 | 功能已定位 规则待分析 | 消费者未定位，不凭平台名补造功能 |

依据：[skeleton/B/crates/geforcenow/src/lib.rs](../../skeleton/B/crates/geforcenow/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### usdl

C的SDL平台安全包装，来源待分类。版本范围：C。

边界：SDL本体、通用窗口/事件库不逆向；游戏输入约束在实际消费者。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 窗口与显示接口 | 包装窗口建立、显示模式、全屏、high-DPI和原生handle | 来源与私有增量待分类 | 包装来源和项目必要差异 |
| 事件与输入接口 | 包装事件poll/wait、键码、鼠标捕获和相对模式 | 来源与私有增量待分类 | 项目调用与映射增量 |
| 平台初始化接口 | 装载SDL并管理video/event/mouse子系统 | 来源与私有增量待分类 | 库来源和游戏初始化策略 |

依据：[skeleton/C/crates/usdl/src/event.rs](../../skeleton/C/crates/usdl/src/event.rs)；[skeleton/C/crates/usdl/src/keyboard.rs](../../skeleton/C/crates/usdl/src/keyboard.rs)；[skeleton/C/crates/usdl/src/lib.rs](../../skeleton/C/crates/usdl/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### usdl-sys

C的SDL底层FFI与动态装载包装。版本范围：C。

边界：纯FFI与平台函数不列手写游戏功能；只研究明确项目装载策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 动态后端选择 | 装载SDL库及符号并选择active API | 来源与私有增量待分类 | 原路径、fallback和错误策略 |
| 平台ABI映射 | 映射事件、窗口、键鼠与系统window-info接口 | 依赖复用 | 源身份/必要ABI差异待核实，不以FFI函数数量计业务 |

依据：[skeleton/C/crates/usdl-sys/src/backend.rs](../../skeleton/C/crates/usdl-sys/src/backend.rs)；[skeleton/C/crates/usdl-sys/src/error.rs](../../skeleton/C/crates/usdl-sys/src/error.rs)；[skeleton/C/crates/usdl-sys/src/event.rs](../../skeleton/C/crates/usdl-sys/src/event.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### aftermath-rs

NVIDIA GPU崩溃诊断接口包装。版本范围：A、B、C。

边界：NVIDIA SDK不逆向；只恢复项目配置、callback内容和消费策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| GPU崩溃回调 | 包装GPU crash dump、shader debug info、description和marker回调 | 来源与私有增量待分类 | 包装来源与游戏NvAftermathDelegate实际内容 |
| 崩溃处理完成状态 | 查询并等待诊断状态，决定何时消费dump | 来源与私有增量待分类 | 状态、等待及超时策略 |
| shader诊断关联 | 取得shader debug info identifier以联接shader与dump | 来源与私有增量待分类 | 实际记录生产与报送消费者 |

依据：[skeleton/B/crates/rhapsody/crates/aftermath-rs/src/lib.rs](../../skeleton/B/crates/rhapsody/crates/aftermath-rs/src/lib.rs)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### dxgi-enumerate-gpus

Windows显卡枚举包装。版本范围：A、B、C。

边界：DXGI与wide字符串通用转换复用；不列转换函数独立业务。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 显卡信息枚举 | enumerate取得系统GPU列表与显示名称 | 来源与私有增量待分类 | 包装来源及字段过滤 |
| GPU列表消费 | 把DXGI信息用于实际选择或诊断 | 功能已定位 规则待分析 | 与Vulkan设备匹配和排序由消费者决定 |

依据：[skeleton/B/crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs](../../skeleton/B/crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### gpu-profiler

游戏GPU时间标记与性能结果包装。版本范围：A、B、C。

边界：Vulkan query/ash实现复用；性能图展示属于真实游戏消费者。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 每帧时间标记 | 开始帧、建立命名marker并向命令记录时间戳 | 来源与私有增量待分类 | 包装来源、marker生命周期和原消费者 |
| 提交帧结果读取 | 结束帧后取回GPU计时结果 | 来源与私有增量待分类 | 完成时机、未就绪与失败处理 |
| 持续时间解释 | 按GPU tick和timestamp period生成纳秒结果并关联名称 | 来源与私有增量待分类 | 原计时精度及实际统计消费者 |

依据：[skeleton/B/crates/rhapsody/crates/gpu-profiler/src/backend/ash.rs](../../skeleton/B/crates/rhapsody/crates/gpu-profiler/src/backend/ash.rs)；[skeleton/B/crates/rhapsody/crates/gpu-profiler/src/shared.rs](../../skeleton/B/crates/rhapsody/crates/gpu-profiler/src/shared.rs)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 资产生产与格式

### asset

游戏资源描述、继承、发现与纹理交换格式。版本范围：A、B、C。

边界：serde/RON/rkyv/image/DDs解码器复用；字段、默认、继承和项目格式是功能，derive不是。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 配置继承 | 处理未指定、显式空值、父配置和子覆盖，形成最终资源描述 | 已有规则或限定实现 | 全部concrete_default owner、错误路径及类型 |
| prefab发现 | 扫描资源目录并收集prefab配置及引用文件 | 已有规则或限定实现 | 全部扫描异常、side effects与真实热重载 |
| prefab参数合成 | 应用scale/albedo等配置的默认与覆盖，收集依赖 | 已有规则或限定实现 | 所有资产默认值和消费者约束 |
| 纹理描述扫描 | 从路径、配置和数组命名建立纹理desc，按名称排序 | 已有规则或限定实现 | 原CLI外层与异常目录 |
| 纹理默认策略 | 缺省compression为Color，保留原sRGB/尺寸等字段语义 | 已有规则或限定实现 | 未证明全部默认owner和原字段声明 |
| 纹理数组检查 | 要求层索引连续且header一致，按layer-major组合 | 已有规则或限定实现 | 真实运行时消费及全错误边界 |
| 纹理交换格式 | 生产/读取游戏magic、version与尺寸/mip/格式/payload契约 | 已有规则或限定实现 | archive回写已验证，不等于原全图片producer |
| 资源依赖与热更新 | 把prefab/纹理文件变化传播到country-core资产库 | 功能已定位 规则待分析 | 游戏缓存、热重载与资源所有权 |
| C网格与植被资产 | C的mesh/foliage模块描述供新资产生产者消费的数据 | 功能已定位 规则待分析 | C独立schema、导入与调用链 |
| C旧Mod资源兼容 | legacy_mod/texture新增文件中的兼容规则 | 功能已定位 规则待分析 | C实际字段与迁移策略 |

依据：[evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)；[evidence/shared-foundation-2026-10-13/assets-small-first/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/interface-coverage.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### asset-pipe

把游戏图像描述生产成纹理资产。版本范围：A、B、C。

边界：image/ddsfile/Intel压缩库/easy-parallel/rkyv复用；原完整源图缺失，完整重编译对照未闭合。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 图像工作空间 | 普通图解码后按sRGB策略对RGB做transfer，alpha保持 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 尺寸与压缩资格 | 压缩资格看源图两维是否至少4，可压缩尺寸对齐4 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 图像缩放 | 使用原image的CatmullRom按目标尺寸采样，恢复项目选择策略 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| mip生成 | 每层从base采样并按原策略物化RGBA8，处理边缘重复padding | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 压缩模式选择 | BC7按alpha全不透明与否选择原basic preset；实际压缩调用依赖 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| DDS输入透传 | 按项目白名单保留DDS尺寸/mip/格式，忽略普通图options并取layer0 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 纹理数组产出 | 按一致header与layer顺序合并各层字节 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 增量构建缓存 | 按desc、源mtime、magic/version判断是否重建，保留header-only命中行为 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 多资源构建汇总 | 逐资源交给easy-parallel，任务完成后写meta/list，错误或panic不写目录索引 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| C新增纹理处理 | mip_flood/store/texture_processing的新生产策略独立分析 | 功能已定位 规则待分析 | C来源/算法与新资源构建链 |

依据：[evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### asset-pipe-bin

原资产生产命令行入口。版本范围：B、C。

边界：仅包身份已知，不能根据工具名给出已确认手写流程。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 资产构建入口定位 | 原锁确认依赖asset-pipe等包，需要确定实际命令与生产目标 | 仅包身份 待定位 | runtime PDB未定位作者源路径 |
| 命令与输出策略定位 | 确认参数、输入输出目录和错误退出如何影响游戏资产 | 仅包身份 待定位 | 待取得发行构建或工具证据；不编造CLI flags |

依据：[skeleton/B/Cargo.lock](../../skeleton/B/Cargo.lock)；[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)；[evidence/shader-build-2026-10-13/provenance/PROVENANCE.md](../../evidence/shader-build-2026-10-13/provenance/PROVENANCE.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 渲染框架

### rhapsody

游戏自己的Vulkan资源、图执行和GPU生命周期组合策略。版本范围：A、B、C。

边界：ash/Vulkan/allocator/vk-sync/平台库复用；这里只恢复项目资源和调度策略。C来源commit已知，函数体仍以独立证据验收。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 设备适配与选择 | 读取项目白名单、判断设备/驱动兼容与surface能力再创建设备 | 已有规则或限定实现 | 完整排序、驱动编码和配置来源 |
| 资源用途映射 | 把游戏buffer/image用途转换成Vulkan用途和访问策略 | 已有规则或限定实现 | 公开vk-sync分类器复用 |
| 资源创建与初始化 | 按游戏descriptor创建buffer/image/view/sampler，上传初始数据 | 已有规则或限定实现 | setup/texture上传已有同步候选；原并入下一图提交而不等待的时序未闭合 |
| GPU数据传输 | 组织映射、staging、拷贝和读回，明确CPU与GPU可用时间 | 已有规则或限定实现 | 全部生产者、失败路径与游戏资源 |
| shader与pipeline创建 | 消费项目compiled资产/reflection形成shader、布局与pipeline | 已有规则或限定实现 | 全变体选择和真实raster/pass绑定 |
| descriptor绑定 | 由反射生成布局、池和写入，绑定managed资源与动态常量 | 已有规则或限定实现 | 稀疏绑定、全部资源生产者及真实Command |
| 附件与渲染目标 | 组织render pass/framebuffer和附件组合 | 已有规则或限定实现 | 完整原图与各raster consumer |
| 渲染图声明 | 描述pass、资源及访问，并导入/导出外部资源 | 已有规则或限定实现 | 游戏pass接入和全API闭合 |
| 渲染图编译 | 计算资源first/last use、访问关系和执行批次 | 已有规则或限定实现 | 完整原compile失败/恢复与alias生产者 |
| 资源池与注册 | 按图资源描述复用物理资源并维持注册和句柄关联 | 已有规则或限定实现 | 真实游戏持有关系和全域生产者 |
| 录制与屏障 | 从compiled access产生barrier；按原分块预算/贪心切分规则录制并给callback提供当前Command | 已有规则或限定实现 | 原分块策略有证据；命令池来源、开关含义及首个shader callback仍有缺口 |
| 提交序号与队列 | 生产command sequence，组织wait/signal和实际submit参数 | 已有规则或限定实现 | 完整原调用链、命令池来源和多队列策略 |
| 完成后资源回收 | CPU退役与GPU完成分离，按序号桶保护并回收使用中的资源 | 已有规则或限定实现 | 全部对象种类、失败和性能边界 |
| 跨帧资源 | 按key维护temporal双缓冲/parity，支持import/export与resize替换 | 已有规则或限定实现 | reset、淘汰及真实跨帧消费者 |
| 窗口呈现 | 选择surface/swapchain、获取并呈现图像，处理变化和失败 | 已有规则或限定实现 | fullscreen exclusive及完整窗口状态 |
| 兼容措施与诊断 | 组织workarounds、调试标记、统计、profiling与GPU崩溃联接 | 已有规则或限定实现 | 全部设备策略、Aftermath等来源边界 |
| 完整游戏帧执行 | 把country-core passes联接进同一录制、submit及回收链 | 已有规则或限定实现 | 资产库状态机、raster draw和原整帧等价 |
| C独立渲染实现 | C采用rhapsody2仓库，raw/rg与render-std分包需独立对齐 | 功能已定位 规则待分析 | 不能用B地址或B候选实现宣布C完成 |
| 显存预算与峰值 | 项目分配包装跟踪预算/峰值、失败采样并在失败时回滚 | 已有规则或限定实现 | 原本地allocator分叉及完整统计口径 |
| 持久渲染session | 跨帧复用chain、资源池、timeline和命令缓冲，处理resize与失败停止 | 已有规则或限定实现 | 原完整失败展开与并行状态 |
| 游戏格式映射 | 把游戏实际format字段映射到GPU格式与兼容属性 | 已有规则或限定实现 | 原私有字段与公开格式查询分别验收 |

依据：[reconstruction/rhapsody-rebuild/MODULE_STATUS.md](../../reconstruction/rhapsody-rebuild/MODULE_STATUS.md)；[tracking/module-progress.json](../../tracking/module-progress.json)；[evidence/render-recovery/consumer-chain-review-2026-10-12/README.md](../../evidence/render-recovery/consumer-chain-review-2026-10-12/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## shader 工具链

### shader-pipeline

原shader产物格式、反射数据和pipeline描述。版本范围：A、B。

边界：speedy只承担codec；项目schema及名称策略需恢复。字段schema不拆成派生序列化接口。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 编译产物容器 | 描述多stage SPIR-V、编译请求及stored reflection的项目布局 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| pipeline描述组合 | 组织compute/raster需要的描述对象、可选字段与stage关系 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 资源反射合同 | 定义descriptor类别、数组数量、push constants、vertex input与local size | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 资源名称与别名 | 按stored公共名称、debug alias和原索引规则查询资源 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 宏定义身份 | 将define集合与shader key/变体查询联接 | 已有规则或限定实现 | 38个非零compute key的完整输入策略 |
| 产物加载与错误 | 读取compiled资产并向shader创建消费者传递上下文 | 已有规则或限定实现 | 完整加载错误、缓存所有权与原副作用 |

依据：[evidence/shader-toolchain-2026-10-13/SUMMARY.md](../../evidence/shader-toolchain-2026-10-13/SUMMARY.md)；[evidence/shader-build-2026-10-13/pipeline/delivery.json](../../evidence/shader-build-2026-10-13/pipeline/delivery.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### shader-compiler

组合原公开库和DXC生成游戏shader资产。版本范围：B。

边界：已有真实DXC新producer，不再写无DXC；404容器可构建不证明原shader语义。B runtime PDB未定位本包作者源路径。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 源码provider与include | 向shader-prepper提供源文件，组织include chunks与依赖记录 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 宏和行号编排 | 把项目宏及#line关联写进实际编译输入 | 已有规则或限定实现 | compute原宏tuple与路径解析精确规则 |
| DXC请求配置 | 根据stage/入口组合目标、flags与外部编译请求 | 已有规则或限定实现 | 原DXC版本/fork与全部配置producer |
| 反射生产 | 调用原SPIR-Q，把结果投影到项目schema并合并stage资源 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| SPIR-V后处理 | 通过rspirv组织module及源文本LF等观察规则，保留调试元数据 | 已有规则或限定实现 | 原私有postprocessor函数体与错误策略 |
| shader打包 | 按项目schema组装新SPIR-V和反射，再调用speedy输出容器 | 已有规则或限定实现 | 真实游戏消费者与全部边界未验收 |
| 批量构建与变体 | 对恢复shader corpus实际编译，按raster变体生成产物 | 已有规则或限定实现 | 170个compute原宏tuple未独证，其中38非零key未知 |
| 编译生命周期 | 处理临时文件、错误、取消、清理和缓存 | 功能已定位 规则待分析 | 原流程未定位，不以候选进程包装冒称原规则 |

依据：[evidence/shader-build-2026-10-13/SUMMARY.md](../../evidence/shader-build-2026-10-13/SUMMARY.md)；[evidence/shader-build-2026-10-13/pipeline/delivery.json](../../evidence/shader-build-2026-10-13/pipeline/delivery.json)；[evidence/shader-build-2026-10-13/dxc/delivery.json](../../evidence/shader-build-2026-10-13/dxc/delivery.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### shader-variants

项目自己的raster宏变体策略。版本范围：A、B。

边界：不列Clone函数；拷贝define集只是生成变体这一功能的实现动作，原规则在整体功能里恢复。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| raster变体生成 | 按原已观察规则从基础defines生成SHADER_VARIANT_DEPTH_ONLY=0与1两种raster变体 | 已有规则或限定实现 | 全部原配置到具体shader的消费关系 |
| 变体到pipeline联接 | 将生成的宏组合交给compiler与shader key消费者 | 已有规则或限定实现 | 完整编译配置、缓存key和资产库联接 |

依据：[evidence/shader-build-2026-10-13/pipeline/delivery.json](../../evidence/shader-build-2026-10-13/pipeline/delivery.json)；[evidence/shader-build-2026-10-13/provenance/package-classification.json](../../evidence/shader-build-2026-10-13/provenance/package-classification.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 构建与状态边界

### build-id

原构建身份配套包。版本范围：B、C。

边界：包名不能证明具体build hash算法；若仅生成常量/宏，不列手写业务任务。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 构建标识消费定位 | 确定游戏版本或产物中如何使用该包身份 | 仅包身份 待定位 | 源路径及功能尚未定位 |

依据：[skeleton/B/Cargo.lock](../../skeleton/B/Cargo.lock)；[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### state-stream

原锁中的状态配套包，具体用途待定位。版本范围：B、C。

边界：原包身份已知但PDB主模块表未定位手写实现；归属确认后才能列算法。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 状态流业务定位 | 定位实际类型、调用者及状态变更传递规则 | 仅包身份 待定位 | 不把它与tabloid或历史系统直接等同 |

依据：[skeleton/B/Cargo.lock](../../skeleton/B/Cargo.lock)；[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## C 版资产与 Mod

### tg-mods

发现Mod、维护身份并形成游戏资源覆盖层。版本范围：C。

边界：C独立输入与算法；不套B接口；serde/ZIP/文件系统实现复用。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| Mod目录发现 | 扫描root和parts，区分目录Mod、ZIP及无parts目录 | 功能已定位 规则待分析 | 扫描条件和错误策略 |
| ZIP缓存整理 | 决定提取根，处理sidecar、last-used和orphaned sidecar清理 | 功能已定位 规则待分析 | 缓存身份、路径约束与淘汰门限 |
| Mod身份 | 读取/生成ModId，处理owner hash、记录条目匹配及旧sidecar名称 | 功能已定位 规则待分析 | 身份格式、mint规则与兼容 |
| 安装目录管理 | shelve_mod与deleted root管理被收起的Mod | 功能已定位 规则待分析 | 操作触发、移动/失败/恢复规则 |
| 资源目录合并 | 维护catalog，处理Workshop条目、supersedes和加载顺序 | 功能已定位 规则待分析 | 优先级与冲突裁决 |
| 活动覆盖层 | 设置active mods，构建overlay并识别shadowed paths | 功能已定位 规则待分析 | 刷新、失效与实际资产消费者 |
| 缺失Mod记录 | 检测存档记录引用的missing mod entries | 功能已定位 规则待分析 | 提示与fallback策略 |
| 资源读取接入 | 通过overlay解析路径、读取配置与列举资源文件 | 功能已定位 规则待分析 | ChangedPaths到真实资源重建 |
| 旧纹理兼容 | parse_legacy_texture解释旧纹理资源 | 功能已定位 规则待分析 | 旧schema与层路径映射 |
| Mod发布资格 | is_publishable及匹配记录确定项目允许的发布状态 | 功能已定位 规则待分析 | 原资格条件；不是此次执行发布 |

依据：[skeleton/C/crates/tg-mods/src/discovery.rs](../../skeleton/C/crates/tg-mods/src/discovery.rs)；[skeleton/C/crates/tg-mods/src/identity.rs](../../skeleton/C/crates/tg-mods/src/identity.rs)；[skeleton/C/crates/tg-mods/src/legacy_texture.rs](../../skeleton/C/crates/tg-mods/src/legacy_texture.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### clutter-icon-pipe

由杂物mesh与材质烘焙游戏图标。版本范围：C。

边界：这是C项目资产工具的组合策略；GPU接口、PNG和DDS库不逆向重写。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 图标输入发现 | icon_sources与parse_clutter_mesh识别图标mesh输入 | 功能已定位 规则待分析 | 来源筛选与属性读取规则 |
| 材质纹理装配 | 按texture recipe加载packaged blob与map textures | 功能已定位 规则待分析 | map种类、缺图处理和资源绑定 |
| 图标镜头与样式 | icon_camera与IconStyle shader_params决定图标构图和参数 | 功能已定位 规则待分析 | 相机尺寸、背景、光照与样式常量 |
| GPU烘焙资源 | 选择设备并建立可复用bake resources | 功能已定位 规则待分析 | 缓存、创建错误与生命周期 |
| 图标渲染 | render_icon和bake_icon_png_from_mesh生产图标画面 | 功能已定位 规则待分析 | pass/descriptor和像素输出 |
| 色调与底面 | 加载Tony McMapface LUT并按原入口添加floor quad | 功能已定位 规则待分析 | 实际触发、采样与地面表现 |
| PNG产出 | encode_png编码烘焙结果 | 功能已定位 规则待分析 | 翻转、alpha、输出命名；PNG库复用 |

依据：[skeleton/C/crates/clutter-icon-pipe/src/lib.rs](../../skeleton/C/crates/clutter-icon-pipe/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### clutter-pack-pipe

导入杂物资产并生产资源包和预览。版本范围：C。

边界：GLB/image/rkyv/turbosloth算法来源要先分类；项目节点命名、材质映射和构建组合是候选业务。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| GLB对象导入 | 遍历node/primitive/material/image并形成杂物对象及mesh | 功能已定位 规则待分析 | 坐标、属性和对象过滤 |
| 材质映射 | 按MapKind选纹理prefix/options，解释shades_like与emits | 功能已定位 规则待分析 | 具体材质字段和组合规则 |
| 内嵌图像提取 | extract_encoded_image读取GLB图像输入给纹理构建 | 功能已定位 规则待分析 | 支持格式与错误策略 |
| 座位节点语义 | 识别coaster seat命名并从node transform形成座位信息 | 功能已定位 规则待分析 | 命名判定和变换规则 |
| 杂物包构建 | build_clutter_packs组织mesh、material、对象列表和依赖 | 功能已定位 规则待分析 | 包schema、名称与增量构建 |
| 对象纹理构建 | build_object_textures选择并生产对象纹理 | 功能已定位 规则待分析 | 复用asset-pipe的参数映射 |
| 包预览图 | preview_grid_shape和bake_pack_preview_jpeg生成组合预览 | 功能已定位 规则待分析 | 排列、背景、边缘feather与输出 |
| Mod杂物图标 | build_mod_clutter_icons生成Mod专用图标包 | 功能已定位 规则待分析 | package目录、cache和真实UI消费者 |

依据：[skeleton/C/crates/clutter-pack-pipe/src/lib.rs](../../skeleton/C/crates/clutter-pack-pipe/src/lib.rs)；[skeleton/C/crates/clutter-pack-pipe/src/mod_icons.rs](../../skeleton/C/crates/clutter-pack-pipe/src/mod_icons.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### asset-pipe-build

C新增资产构建配套包。版本范围：C。

边界：仅包身份与依赖链，不声称有已定位手写功能。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 构建流程定位 | 确认该包在C资产构建中的实际输入和输出 | 仅包身份 待定位 | runtime PDB主库存未定位源码 |
| 构建触发定位 | 确认build-time或runtime调用者 | 仅包身份 待定位 | 不能仅凭build包名编造build.rs规则 |

依据：[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### foliage-pipe

C植被资产生产配套包。版本范围：C。

边界：原锁中的local条目不是私有算法证明；不按包名猜散布或生成规则。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 植被资产生产定位 | 沿asset::foliage与原依赖链定位本包生产者 | 仅包身份 待定位 | 实际入口和算法未定位 |

依据：[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### grass2-shader

C草地shader配套包。版本范围：C。

边界：shader代码的游戏算法可恢复，宏生成binding和通用编译器不可计作手写逻辑。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 草地shader功能定位 | 定位其与country-core草地shader/生成代码的关系 | 仅包身份 待定位 | 作者手写源码、生成部分与实际函数尚待分离 |

依据：[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 来源待分类的基础包装

### frozen

原资产库使用的FrozenMap包装。版本范围：A、B、C。

边界：不恢复HashMap/Box通用实现，不把名称含Mesh/Shader的泛型实例计成游戏算法。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 资产对象稳定访问 | 原mesh/shader/SSBO库使用map插入与访问，稳定引用策略待分析 | 来源与私有增量待分类 | 手写策略、共享访问和公开同源确认 |

依据：[skeleton/B/crates/frozen/src/lib.rs](../../skeleton/B/crates/frozen/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### leekspin

诊断输出和环形记录包装。版本范围：B。

边界：公开来源与项目私有增量待核实；环形容器本身不自动成为游戏逻辑。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 重复诊断汇总 | print_deduplicated_with_count按重复次数整理输出 | 来源与私有增量待分类 | 去重key、输出周期和真实诊断消费者 |
| 诊断环形记录 | RingBuffer append/to_inner维护并交付记录 | 来源与私有增量待分类 | 容量、覆盖与导出策略 |

依据：[skeleton/B/crates/leekspin/src/lib.rs](../../skeleton/B/crates/leekspin/src/lib.rs)；[skeleton/B/crates/leekspin/src/ringbuffer.rs](../../skeleton/B/crates/leekspin/src/ringbuffer.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### conservative-line-raster

世界笔画到保守栅格覆盖的接口。版本范围：A、B、C。

边界：首先核实上游，复用通用栅格算法；游戏调用策略留在实际消费者。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 线段保守覆盖 | conservative_plot_line枚举笔画所覆盖的格子 | 来源与私有增量待分类 | 公共算法来源与必要差异 |
| 坐标量化策略 | 低层量化范围/断言处理坐标与delta | 来源与私有增量待分类 | 边界精度与原异常条件 |
| 游戏脏格接入 | dirty_tiles笔画/历史路径/楼梯消费者使用覆盖格子 | 功能已定位 规则待分析 | 恢复消费者坐标、宽度和dirty策略，不先重写算法 |

依据：[skeleton/B/crates/conservative-line-raster/src/lib.rs](../../skeleton/B/crates/conservative-line-raster/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### turbosloth

C资产构建依赖、缓存与打包框架。版本范围：C。

边界：框架功能不因项目内路径成为游戏业务；先查来源，后只恢复私有增量。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 构建请求与依赖 | BuildContext组织build/require、文件读取和dependency登记 | 来源与私有增量待分类 | 框架公开来源与项目特有策略 |
| 构建产物缓存 | load/cached_value/store_asset和finish_build管理产物 | 来源与私有增量待分类 | 类型缓存、共享请求与失败重试 |
| 源文件新鲜度 | 观察file stamp/state，判断依赖与recipe是否仍fresh | 来源与私有增量待分类 | mtime/content、缺失文件和缓存有效性 |
| 构建记录更新 | 读取build record并重写记录的文件依赖 | 来源与私有增量待分类 | 记录格式和变化传播 |
| 构建版本注册 | registered_version/versions确定recipe版本关系 | 来源与私有增量待分类 | 版本更新与失效策略 |
| 内容与recipe身份 | 组织AssetContentId/AssetRecipeId和名称索引 | 来源与私有增量待分类 | 原ID输入组合；哈希库复用 |
| 资源包安装 | PackagePass组织install closure、manifest和package state | 来源与私有增量待分类 | 依赖闭包、增量安装与缺失asset |
| 构建结果导出 | exports/parse_path/index_value/export_asset提供资产导出 | 来源与私有增量待分类 | 实际路径选择与消费者 |
| 原子文件更新 | write_atomic辅助更新持久文件 | 来源与私有增量待分类 | 原失败、临时文件及恢复策略 |

依据：[skeleton/C/crates/turbosloth/src/asset.rs](../../skeleton/C/crates/turbosloth/src/asset.rs)；[skeleton/C/crates/turbosloth/src/export.rs](../../skeleton/C/crates/turbosloth/src/export.rs)；[skeleton/C/crates/turbosloth/src/id.rs](../../skeleton/C/crates/turbosloth/src/id.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### turbosloth-macros

C资产框架宏生成配套。版本范围：C。

边界：不是独立游戏功能清单；必要schema输入和游戏recipe保留到实际生产者。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 宏包来源与生成边界 | 核对原宏来源和生成的框架接口 | 依赖复用 | 原来源未定位；不列自动生成resolve/visit/decode函数任务 |

依据：[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## 复用依赖边界

### gpu-allocator

Vulkan内存分配依赖。版本范围：A、B、C。

边界：不恢复free-list、dedicated allocator、容器或整个分配器；游戏分配选择在rhapsody。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| GPU内存分配接口 | 提供资源allocation/map/free能力，由rhapsody按需求配置 | 依赖复用 | 只核实原fork来源和影响调用的必要补丁 |

依据：[DEPENDENCY_FILTERING.md](DEPENDENCY_FILTERING.md)；[tracking/dependency-exclusions.json](../../tracking/dependency-exclusions.json)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### vk-sync-fork

原Vulkan同步分类与屏障依赖。版本范围：A、B、C。

边界：is_write已确认是依赖；不计为游戏自有叶函数。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 访问分类接口 | 供游戏框架解释访问类型与读写分类 | 依赖复用 | 精确fork来源及必要差异 |
| 屏障转换接口 | 由游戏资源访问决定何时调用依赖生成同步描述 | 依赖复用 | 调用策略在rhapsody，不重写公开同步库 |

依据：[DEPENDENCY_FILTERING.md](DEPENDENCY_FILTERING.md)；[tracking/dependency-exclusions.json](../../tracking/dependency-exclusions.json)；[skeleton/B/crates/rhapsody/crates/vk-sync-rs/src/lib.rs](../../skeleton/B/crates/rhapsody/crates/vk-sync-rs/src/lib.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### kajiya

原锁中的渲染配套包，游戏使用边界待核实。版本范围：B、C。

边界：不据包名或作者公开同名仓库恢复整个引擎；必要私有调用差异单独登记。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 实际调用与来源定位 | 核对原锁local身份与游戏实际使用的模块及公开同源 | 来源与私有增量待分类 | 公开相似源码不能替代原私有schema或证明包等价 |

依据：[skeleton/B/Cargo.lock](../../skeleton/B/Cargo.lock)；[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)；[evidence/shader-build-2026-10-13/provenance/PROVENANCE.md](../../evidence/shader-build-2026-10-13/provenance/PROVENANCE.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### sdl2

C的SDL窗口依赖。版本范围：C。

边界：游戏平台选择和事件映射在tg-iced/usdl及实际输入消费者。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 平台窗口依赖适配 | 核对原0.38.0 local来源、平台feature及必要补丁 | 依赖复用 | 不重做SDL事件、窗口与输入引擎 |

依据：[skeleton/C/Cargo.lock](../../skeleton/C/Cargo.lock)；[DEPENDENCY_FILTERING.md](DEPENDENCY_FILTERING.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### FidelityFX

AMD渲染算法SDK。版本范围：B、C。

边界：不因local路径重新实现SDK；不从GPU-Raytracer包名推断原游戏用了硬件光追。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 游戏调用参数 | 恢复游戏传入该依赖的资源、参数和调用时机 | 依赖复用 | 依赖实现明确排除，调用策略归具体pass |

依据：[DEPENDENCY_FILTERING.md](DEPENDENCY_FILTERING.md)；[tracking/dependency-exclusions.json](../../tracking/dependency-exclusions.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### XeGTAO

环境遮蔽依赖。版本范围：B、C。

边界：不因local路径重新实现SDK；不从GPU-Raytracer包名推断原游戏用了硬件光追。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 游戏调用参数 | 恢复游戏传入该依赖的资源、参数和调用时机 | 依赖复用 | 依赖实现明确排除，调用策略归具体pass |

依据：[DEPENDENCY_FILTERING.md](DEPENDENCY_FILTERING.md)；[tracking/dependency-exclusions.json](../../tracking/dependency-exclusions.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### GPU-Raytracer

GPU几何查询或光追配套依赖。版本范围：B、C。

边界：不因local路径重新实现SDK；不从GPU-Raytracer包名推断原游戏用了硬件光追。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 游戏调用参数 | 恢复游戏传入该依赖的资源、参数和调用时机 | 依赖复用 | 依赖实现明确排除，调用策略归具体pass |

依据：[DEPENDENCY_FILTERING.md](DEPENDENCY_FILTERING.md)；[tracking/dependency-exclusions.json](../../tracking/dependency-exclusions.json)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。


## C 版渲染附属包

### rhapsody-core

C渲染共享view与图像归约策略。版本范围：C。

边界：同仓库来源commit已知；不能用B的HLSL或pass验证代替C。公开算法只恢复其调用策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 相机与视图常量 | 组织C相机矩阵、视图参数、pixel offset与mip bias | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |
| mip与深度归约 | generate_mips/depth_min_max组织图像mip及深度min/max归约 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |
| 重投影图 | render_reprojection_map构造跨视图或跨帧重投影输入输出 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |

依据：[skeleton/C/rhapsody2/render-std/rhapsody-core/src/camera.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-core/src/camera.rs)；[skeleton/C/rhapsody2/render-std/rhapsody-core/src/lib.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-core/src/lib.rs)；[skeleton/C/rhapsody2/render-std/rhapsody-core/src/reduction.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-core/src/reduction.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### rhapsody-dof

C景深pass包装。版本范围：C。

边界：同仓库来源commit已知；不能用B的HLSL或pass验证代替C。公开算法只恢复其调用策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 景深pass编排 | render_dof_with_coc按已有CoC组织景深渲染 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |

依据：[skeleton/C/rhapsody2/render-std/rhapsody-dof/src/lib.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-dof/src/lib.rs)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### rhapsody-gi

C环境遮蔽与间接光pass包装。版本范围：C。

边界：同仓库来源commit已知；不能用B的HLSL或pass验证代替C。公开算法只恢复其调用策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 环境遮蔽编排 | calculate_gtao组织GTAO计算资源和pass | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |
| 屏幕辐射评估 | eval_gi_screen_radiance组织GI screen-radiance计算 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |

依据：[skeleton/C/rhapsody2/render-std/rhapsody-gi/src/lib.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-gi/src/lib.rs)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### rhapsody-post

C后处理模糊金字塔。版本范围：C。

边界：同仓库来源commit已知；不能用B的HLSL或pass验证代替C。公开算法只恢复其调用策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 正向模糊金字塔 | render_blur_pyramid组织逐级模糊 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |
| 反向和双向模糊 | render_rev_blur_pyramid及bidirectional组合多层模糊 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |

依据：[skeleton/C/rhapsody2/render-std/rhapsody-post/src/blur_pyramid.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-post/src/blur_pyramid.rs)；[skeleton/C/rhapsody2/render-std/rhapsody-post/src/lib.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-post/src/lib.rs)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### rhapsody-shadow

C方向光阴影pass包装。版本范围：C。

边界：同仓库来源commit已知；不能用B的HLSL或pass验证代替C。公开算法只恢复其调用策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 阴影级联设置 | setup_directional_light_cascades计算方向光级联输入 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |
| 阴影贴图生产 | render_directional_light_shadow_maps2组织各级阴影图渲染 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |

依据：[skeleton/C/rhapsody2/render-std/rhapsody-shadow/src/lib.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-shadow/src/lib.rs)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### rhapsody-taa

C时域抗锯齿与TGSR包装。版本范围：C。

边界：同仓库来源commit已知；不能用B的HLSL或pass验证代替C。公开算法只恢复其调用策略。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 时域历史创建 | create_temporal_history组织历史资源 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |
| TAA pass | render_taa使用历史数据组织时域抗锯齿 | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |
| TGSR变体编排 | render_tgsr/2及aux/weights变体组织输入和pass | 功能已定位 规则待分析 | C仅有原入口与PDB骨架；具体算法、手写与生成边界及消费者未恢复 |

依据：[skeleton/C/rhapsody2/render-std/rhapsody-taa/src/lib.rs](../../skeleton/C/rhapsody2/render-std/rhapsody-taa/src/lib.rs)；[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### rhapsody-deferred

C渲染仓库中仅锁定身份的包。版本范围：C。

边界：已知仓库身份不等于已恢复实现。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 渲染包用途定位 | 确认本包实际调用与作者源路径 | 仅包身份 待定位 | PDB未定位，不凭deferred名猜G-buffer/light算法 |

依据：[skeleton/C/rhapsody2/README.md](../../skeleton/C/rhapsody2/README.md)；[skeleton/C/rhapsody2/_unlocated-packages/rhapsody-deferred/Cargo.toml](../../skeleton/C/rhapsody2/_unlocated-packages/rhapsody-deferred/Cargo.toml)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### shaderface

C shader数据、变体身份与binding描述。版本范围：C。

边界：PounceLight/shabriri固定commit来源；Speedy派生read_from和生成ShaderVar接口不单列功能。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| binding集合组织 | 构建/追加命名参数block和descriptor项，按名称查找block | 功能已定位 规则待分析 | 私有布局、数量和冲突规则 |
| 变体key | ShaderPermutationKey to_hash联接变体身份 | 功能已定位 规则待分析 | 原key输入策略，hash库复用 |
| 反射类型尺寸 | Scalar/Vector/Matrix/Array/Struct nbyte供参数布局消费 | 功能已定位 规则待分析 | 对齐、stride和runtime array规则 |
| 预编译参数检查 | 收集未分配parameter block并识别runtime descriptors | 功能已定位 规则待分析 | 编译绑定错误和真实游戏消费者 |

依据：[skeleton/C/shabriri/crates/shaderface/src/binding_set.rs](../../skeleton/C/shabriri/crates/shaderface/src/binding_set.rs)；[skeleton/C/shabriri/crates/shaderface/src/lib.rs](../../skeleton/C/shabriri/crates/shaderface/src/lib.rs)；[skeleton/C/shabriri/crates/shaderface/src/permutation.rs](../../skeleton/C/shabriri/crates/shaderface/src/permutation.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### shabriri-rt

C生成shader接口的运行时binding与编译包装。版本范围：C。

边界：运行时组合策略可分析；生成文件和宏接口数量不计手写功能。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 参数偏移与绑定 | 从array element计算binding/byte位置并生成descriptor/bind操作 | 功能已定位 规则待分析 | 手写runtime层与生成协议的边界 |
| 尾部binding布局 | TailBindingSetLayout与compile组织最终绑定布局 | 功能已定位 规则待分析 | 布局合并、冲突/失败规则 |
| 预编译shader枚举 | iter_precompiled_shaders提供产物迭代 | 功能已定位 规则待分析 | 目录/key策略与实际加载消费者 |

依据：[skeleton/C/shabriri/crates/shabriri-rt/src/api.rs](../../skeleton/C/shabriri/crates/shabriri-rt/src/api.rs)；[skeleton/C/shabriri/crates/shabriri-rt/src/compile.rs](../../skeleton/C/shabriri/crates/shabriri-rt/src/compile.rs)；[skeleton/C/shabriri/crates/shabriri-rt/src/iter.rs](../../skeleton/C/shabriri/crates/shabriri-rt/src/iter.rs)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

### shabriri

C shader生成配套包，仅主包身份明确。版本范围：C。

边界：不把生成的shabriri.rs当作者手写代码，也不编造生成器算法。

| 功能或定位范围 | 游戏侧规则或待确认规则 | 状态 | 剩余缺口 |
|---|---|---|---|
| 生成工具用途定位 | 确认本包是否构建时运行及实际输入输出 | 仅包身份 待定位 | PDB无主包源路径/命名空间，构建期用途仍是推断 |

依据：[skeleton/C/shabriri/README.md](../../skeleton/C/shabriri/README.md)；[skeleton/C/shabriri/_unlocated-packages/shabriri/Cargo.toml](../../skeleton/C/shabriri/_unlocated-packages/shabriri/Cargo.toml)。全部来源和SHA快照见[机器清单](../../tracking/foundation-business-functions.json)。

## 公共算法复用边界

以下公开库的算法不进入作者手写游戏功能清单。原版本、commit和必要分叉差异按各版原锁核对。

| 依赖 | 处理 |
|---|---|
| std、Bevy、glam、fastrand、half | 复用原版本；只恢复游戏调用、种子、打包和必要分叉差异 |
| serde、serde_json、RON、rkyv、speedy | 复用codec/derive；项目字段、默认、继承、迁移和容错策略独立登记 |
| shader-prepper | 复用0.3.0-pre.3公开include crawler；项目provider/chunk/宏/#line编排在shader-compiler |
| DXC | 复用公开编译器；核实原版本及必要fork差异，不重写HLSL编译器 |
| spirq、rspirv | 复用原commit/版本；项目反射投影、后处理和打包在shader-compiler |
| cosmic-text、etagere | 复用字体整形和图集分配；项目文字编辑、缓存失效与绘制准备在tg-text |
| image、ddsfile、压缩库、ZIP、HTTP | 复用解码/压缩/网络算法；项目参数、过滤、缓存与错误编排留在生产者 |
| ash、winit、iced、SDL、NVIDIA SDK | 复用接口和平台实现；项目配置、映射与回调内容独立登记 |
| 哈希、容器、锁和线程库 | 直接复用原依赖，不逆向它们的泛型实例 |

## 后续按功能推进

1. utils优先形成墙轮廓和装饰所需的坐标、区段、重采样、距离及法线闭包；逐维度明确几何语义。
2. asset和asset-pipe从原配置到最终资源及热更新消费者联接；motomoto从chunk协议接到真实Snapshot/Terrain消费者。
3. tg-text从字体角色和形状流推进到文字片段布局、字形缓存和GPU准备；tg-iced处理真实界面状态机。
4. shader和rhapsody用真实资产、pass、Command、submit及回收联接推进，保留原宏配置、原shader语义和候选流程的区别。
5. 尚无算法证据的主题、报告、Mod等包先解释真实业务规则；来源待分类库先查上游。派生或库实现不转成算法任务。

## 覆盖范围与原台账关系

本次登记58个库的286条功能或定位记录，其中139条有业务规则分析或限定实现，98条只有功能范围定位，26条须先分类来源，4条仅数据契约，10条仅包身份，9条明确复用依赖。这些是重新划分后的目录记录，绝不是286项已确认手写算法或完成数量。

[原模块进度](../reference/MODULE_PROGRESS.md)、[旧逐接口台账](INTERFACE_PROGRESS.md)和[旧未映射接口](UNRECOVERED_INTERFACES.md)保留。旧接口分母包含派生、trait/schema和内联条目，不用它作为本功能清单的任务数；清单不回写旧验收结果。

本清单排除的游戏主体与系统包：country-core、forest、system-clutter、system-color、system-configuration、system-configuration-sys、system-critters、system-decorator、system-deps、system-glade-border、system-onboarding、system-roof、system-tree、system-wall-constructor、system-water、timberframe、tiny-glade、toolbar。

机器清单逐条保存功能状态、证据边界、版本、来源以及输入SHA：[foundation-business-functions.json](../../tracking/foundation-business-functions.json)。
