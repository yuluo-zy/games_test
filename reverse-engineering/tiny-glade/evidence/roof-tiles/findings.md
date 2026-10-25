# 屋瓦生成：已经恢复的实现与验证边界

输入对象固定为原作 `D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne`，SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；配套 PDB 仅用于名称和精确过程范围。原游戏目录只读。本轮没有运行游戏主进程。

## 已恢复且实际执行对照的代码

- [Rust 实现](../../reconstruction/roof/tiles.rs)：完整圆形屋瓦实例生成、自有 utils 随机分区、扩展屋顶边界、矩形组装前置上下文和角点。随机源、半浮点转换、矩阵转四元数和 quaternion×vector 运算现在复用原版公开依赖 API，原先的手写依赖实现已经移除。
- `assemble_roof_tiles` 位于 `0x141a36700`，2965 字节，PDB module 2275。这里的主要工作为 ECS/状态查询；按 Roof `+8` bit0 分派到圆形 `0x1421b3120` 或矩形 `0x1421aeef0`。布局规则来自真实下游函数，而非入口函数名称推测。
- 圆形组装位于 `0x1421b3120`，5096 字节，PDB module 3475。
- `utils::random_splits_into` 位于 `0x140c90050`，760 字节；随机源 `fastrand::Rng::f32` 位于 `0x140caf380`，56 字节。两者的完整原机器码都在差分中实际执行。

## 依赖边界修正

原作 `build-info/Cargo.lock` 确认 system-roof、country-core、utils 使用 fastrand 1.8.0、glam 0.29.3、half 2.4.1。重建 Cargo.toml 使用精确 `=` 版本，Cargo.lock 还锁回原版的 cfg-if 1.0.0、crunchy 0.2.2、instant 0.1.12；注册表 checksum 与原 lock 对照。

`TileRng::next_f32` 只调用 `fastrand::Rng::{with_seed,f32,get_seed}`；`half` 只调用 `half::f16::from_f32().to_bits()`；`matrix_quat` 调用 `glam::Mat3::from_cols` 和 `Quat::from_mat3`；圆形 quaternion×vector 调用 glam 的公开乘法。保留的是游戏/自有 utils 的参数、采样、行列、顺序与记录组装规则，未复制第三方依赖源码。

[依赖 API 对照](dependency-verification.json)：2068 个 f16 输入（包括边界和 NaN）及 1027 个 RNG 状态均与既有原机器码 callable 一致。替换后完整圆形仍然 11,467 条记录零差异，完整矩形另由总流水线报告验证。验证 runner 通过 `scripts/dependency_build.py` 链接 Cargo 精确依赖，不再单独 rustc 编译一个无依赖的手写替身。

## 圆形屋瓦算法

1. 从原 Roof 取得扩展底半径和顶半径（顶半径 0.2），生成 20 点规范曲线，再转换为世界空间曲线及弧长参数。曲面和 Curve2 的独立恢复由同目录 surface.rs 完成。
2. 弧长 `L` 决定行数 `N=max(ceil(L/0.4375),2)`；瓦长基值为 `L/N * f32(0x3fb6db6e)`，即约 1.42857146 倍覆盖重叠。实际行坐标为 `i/(N-1)`，`i=0..N-2`；原控制流没有发出最终 u=1 的一行。
3. 沿弧长插值得到每行半径和高度；中心偏移按 u 乘 Roof tip offset，随后加 circle center。基于半径的 inverse lerp（0.9→0.3）缩小名义瓦宽，周长除名义瓦宽向上取整，至少 5 列。
4. RNG 每次屋顶从状态 0 开始。每行取一个随机旋转相位，再构造分区；普通非零周长的分区振幅严格为原表达式 `(scale*0)/perimeter`，没有另行发明抖动。即使振幅为零，分区内部仍逐次消耗 RNG；省略这些调用会让后续瓦长和 t 全部改变。零周长产生 NaN 后，random_splits 按原实现选振幅上限。
5. 每片瓦的位置为周向相邻端点的弦中点加行中心；宽度为弦长。瓦长再乘 0.8→1.2 的随机因子；坡度来自对应折线段单位切线，姿态包含原 -10° 偏角以及圆周法线矩阵转换。
6. 首行还生成独立的水平檐边补瓦，阈值为 0.1。补瓦的姿态框架与坡面瓦不同；差分中对其 64 字节记录也全部比较。
7. 发出完整 64 字节 RoofShingle 记录，包含位置、裁剪平面、半浮点尺寸和 t、半浮点四元数、roof_id、动画初态、seed、shape/body 标记。字段语义以 GPU 结构名作佐证，值仍按原机器码保留。

## 差分证据

| 对照 | 已执行范围 | 结果 |
|---|---|---|
| [圆形全部实例](circular-verification.json) | 25 个 prepared curve 场景 + 12 份原作不同 profile 参数的完整原函数 entry→RET | 11,467 片瓦，全部 64 字节逐字节相同 |
| [分区/随机源/行数](helpers-verification.json) | 原 random_splits_into + RNG，原 circular 行数指令块 | 800 个分区输入 + 523 个弧长输入，零差异 |
| [矩形前置上下文](context-verification.json) | 完整原矩形入口到 0x1421af1ac，原数值辅助函数、profile 和 Curve2 构造均执行 | 816 个输入，底矩形 24 字节、顶矩形 24 字节、基底 16 字节全部相同 |
| [矩形角点](corners-verification.json) | 原 Rectangle2d::as_points2 完整机器码 | 1200 个有限正尺寸输入，4 角点 f32 全 bits 相同 |

圆形完整函数的外围桥接只有：Rust allocator/free、当前 Windows UCRT 的 float math（powf/ceilf/sinf/cosf/fmodf/atanf）、原 Writer closure 处的记录捕获。曲面、弧长查找、RNG、分区、向量运算、矩阵转四元数和半浮点打包全部执行原 RNE 指令。prepared curve 测试与完整函数测试分开报告，不把注入曲线误称为完整入口执行。

原作历史提供屋顶参数，矩形/圆形尺寸和位置是明确记录的合成输入，因为历史→完整 wall state 重放尚未恢复。测试使用默认 MXCSR 0x1f80；有效有限几何是完整实例测试的当前范围。allocator 行为、SSBO 分配竞争、GPU 动态、所有 panic 行为和真实游戏帧图像不在这些数值验证范围内。

## GPU 接口证据

[原作 SPIR-V 布局](shader-interfaces.json) 在 roof cull/bucket、instanced_roof 顶点/像素阶段、roof_tiles_gravity 计算阶段重复出现 `RoofShingle`，数组步长 64 字节。成员偏移：

| 偏移 | 原 shader 名称 |
|---:|---|
| 0 | base_position |
| 12 | current_height |
| 16 | clipping_plane__ |
| 24 | scale__t |
| 32 | rotation_quat__ |
| 40 | roof_id |
| 44 | velocity__rotation |
| 48 | animation_t |
| 52 | seed |
| 56 | is_circle |
| 60 | is_main_roof_body |

CPU Writer `0x141a3c070` 读取传入记录的 64 字节并写入 SSBO。铺瓦 closure 先直接 CALL Writer，因而保留调用入口的 RCX/RDX；没有保存这两个寄存器不能据此断言瓦片被丢弃。shader 名称、明确成员偏移、64 字节数组步长和 CPU 写入大小形成一致的接口证据。反射结果本身不代表 shader 运算已经恢复。

## 公共 API 与剩余工作

- `assemble_circular_from_observed(&[u8;88], special_mode)`：从 Roof::new 的观察字节表示到完整圆形记录序列，依赖同级 surface 模块。
- `assemble_circular_tiles(&CircularTileInput)`：从 prepared Curve2 输入生成相同记录，便于隔离验证。
- `rectangular_context_from_observed(rect24, roof88)`：得到 bottom/top rectangle 和角点、basis、local_tip_center、world_center_xz；原组装在该处使用 zero scale 算局部 tip center，保留 signed zero。
- `expanded_bounds_observed`、`ObservedRectangle::{basis,corners,bytes,from_bytes}`：供矩形面瓦、檐条和脊瓦组装复用。
- `TileRng`、`half`、`matrix_quat`：薄依赖适配层；`random_splits`、`record` 是自有 utils/游戏的组装辅助。f16 已增加边界和 NaN 原子输入验证，未据有限测试承诺任意未测试 bit 模式。

矩形面瓦、矩形檐条、脊顶瓦分别由其他研究分支恢复；完整矩形流需聚合后再对全记录顺序验证。完整游戏仍需要编辑状态、几何依赖调度、GPU 渲染与动画、地形道路、门窗装饰、音频和 UI 的恢复与集成。
