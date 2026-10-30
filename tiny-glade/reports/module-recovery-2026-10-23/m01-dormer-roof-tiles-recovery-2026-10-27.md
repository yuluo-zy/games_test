# M01 dormer 屋顶瓦片尾链恢复，2026-10-27

本轮先实读 B 骨架 `crates/systems/decorator/src/decorator_visual/cottage_dormers/`，恢复既有 `assemble_dormer_roof_tiles.rs`、`util_assemble_gothic_dormer.rs`，保留既有 `maintain_dormer_tiles_draw.rs` 生命周期。另经 root 授权窄修既有 `gothic_wall_window.rs::calc_gothic_half_dormer_profile_ws` 及 `country-core/resources/roofs/roof_shape.rs::Roof::roof_shape_at_intersection_y`；未建业务文件/目录、平行 renderer 或 Bevy 内部容器，未涉及 M07 屋顶编辑工具。

## 实际修复及原映射

上轮两 dormer 入口已有 frame/glass/孔，尾部 `generate_dormer_roofs` 及两瓦片 helper 仍是 None/空 Vec，不能证明屋顶完整。本轮消除此处空主体，接唯一 canonical RoofTileSsbo、实际稀疏分配与 writer。

| 原函数 | VA / PDB code_size | 恢复内容 |
| --- | --- | --- |
| generate_dormer_roofs | 1407cb1a0 / 14207B | 原10组、48B缓存判等、脏墙排序去重、全部 subtype 分支、required SSBO、容量失败语义 |
| assemble_dormer_roof_tiles | 14128db20 / 10926B | 各行实际屋顶截面、圆周跨零、随机行列分割、承托/主瓦/边板、半精度和 RNG |
| cottage side closure | 1412905d0 / 1362B | 两侧竖板完整位置、坡面裁切平面和法向翻转 |
| assemble_gothic_dormer_tiles | 141290b30 / 5713B | 脊条、双侧曲线、深度分割、随机俯仰/延长、真实64B记录 |
| half dormer profile | 1407eb160 / 923B，1407e7dd0 / 1364B，1412ac9b0 / 625B | 真实帽骨双侧、弯法线插值归一化、等级半径、首次墙顶相交、Curve3缓存 |
| roof_shape_at_intersection_y | 1408e3820 | 不带视觉檐口的原结构截面、原 domain/非有限/构造器失败 |
| maintain_dormer_tiles | 1407ce920 / 334B | 既有 DeleteRoofCmd → 原 allocation drop/waste 退役，未重写成假数组 |

原 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。同目录 `m01-dormer-tiles-*-native.{c,asm,py,json}` 保存原证与独立执行脚本，`m01-dormer-tiles-main-system-signature.txt` 保存完整 FunctionSystem。shared Ghidra 只读，未修改其 body/CFG。

## 主系统与生产合同

原10组按顺序：DerivedDecoratorInfoResources、Res<DecoratorStorage>、Res<DecoratorBlueprints>、Res<CachedDecoratorTransforms>、Res<PublicWalls>、Query<&Roof>、原 Res<AssetSsboLibrary>、Res<UiSignifierStream>、ResMut<DormerTiles>、Local<CacheDormerPositions>。当前 safe API 真正写 registry/buffer，root 已批准把 SSBO 组表达为 ResMut，未伪造原内部可变容器。

缓存按装饰 ID 记录实际墙、位置、floor、color、Roof 参数6个 float 与2个尾字节。保留原 float 比较（NaN 不等），不把旋转/缩放/shape 任意加入缓存键。新增/位置或参数变化/换墙/删除分别标记原墙与新墙，排序去重后只重建脏墙。稳定帧仍 required 解析 RoofTileSsbo，但不用 get_mut 人为打脏 registry。每个脏墙复用真实 allocation，清旧实例、逐装饰重新流式写入。

原门为 {1,2,7,8,23} 或 subtype0且rank3；其余不进入。缺实际关联 Roof 或 Query miss 原跳过。首次 subtype 查询可选，重建分支 required 查表。原 `1407cd2d5..1407cd343` 缺 Blueprint **跳过该装饰**，不是 panic；本轮读完整 ASM 后纠正了早期误读，没有伪造空 Blueprint。subtype0rank3 独立 bay 分支在 flat 门之前，其余保留 flat 门。

2使用原三等级宽度+.2、高1、padding.1与侧板；1使用实际 collision bounds、PublicWall.max_y 和对应 padding表；8将原 profile 经实际 scale/quat/position 双侧构造；7/23使用实际墙坐标和 get_gothic_hat_bones，23为独立二点剖面；0rank3复制当前 Roof，positionY+.8、profile=.5、height01=.2、floor=positionY+.2，圆/矩形 padding .15/.35，不把这两参数误写为 ridge/eave。原 Shape.expand(2) 只写无使用的 scratch，没有改传给 helper 的 Roof shape。此分支执行原1407cc5dc..1407cc715共6组验证原实际 payload。

流式 push 容量失败保留原每个单态一次 warn，并继续计算后续 RNG/实例；不回滚、不停止随机序列、不替换成缩减瓦片。输出 roof_id 为实际 Roof 第一 u32，不是装饰 ID。

生产统一调度仍由 root：本系统位于 fullvisual 的真实 Blueprint 发布后阶段；maintain_dormer_tiles 只消费 canonical DeleteRoofCmd，完整 GPU 上传/present 由已有宿主负责。此 owner 没有另挂一条 renderer。

## 几何与数值纠错

cottage 保留每行真实截面、固定底部宽度比例、前向 circular u 跨零、原两次 y 参数映射；不以规则斜面或均匀网格替代。主瓦完整基底 cols(width,slope,width×slope)，只乘原随机 X 俯仰，不能再加“等价”quarterX；其四元数符号会改变半精度 words。两侧板原裁切法向是 `(b-a).cross(direction.cross(Y))`，不是直接 cross(direction)。

gothic 脊起点为 first+(first-anchor)*.05；旋转必须从这个偏置起点计算，不能从未偏置 first 推导近轴方向。原倒数乘向量、乘加分组、双随机抽样顺序、深度和曲线分割均保留。位置分组、四元数和 half packing 已按全记录差分纠正，不放宽误差。

原 roof_shape_at_intersection_y 与视觉 surface.section 不同。旧代码调用带檐口的视觉 surface，造成真实 depth 约26%差异。本轮仅修获授权函数/私有 cap_height：floor相等前先计算原 cap（保失败顺序）、有序范围/NaN门、实际 `(cap+floor)-floor` 分母、圆无假.2尖端、矩形双 complement 插值与真实 Rectangle2d 失败守卫。没有改其它 surface API 统一两个本来不同的合同。

halfprofile 保留 profile首点；每条边映射后若新点低于真实 wall.max_y，则插入首次精确相交并终止。法线在帽骨 center/各端之间插值归一化，不能只做 x镜像或直线位移。圆且subtype7才使用 [.25,.3,.29]，非法rank原panic；其它.38。

## 验收与范围

最终 `CARGO_INCREMENTAL=0 cargo test --offline -p system-decorator --lib dormer`：**41 passed / 0 failed**，收据 `m01-dormer-tiles-final-system-tests.log`，源码与 Cargo.lock SHA 见 `m01-dormer-tiles-validation.json`。

- cottage执行原完整主体及侧闭包12组，312条完整64B记录，所有16 words逐位一致。
- gothic执行原完整主体54组，1485条完整64B记录，所有16 words逐位一致，覆盖双侧、脊、圆/矩形、旋转/scale/等级组合。
- halfprofile执行原完整主函数及两个闭包36组，所有points、points_u、length逐位一致，真实 Rust PublicWalls.calculate/prepare路径提供对应墙顶，而非修改readiness伪造值。
- 原截面完整200组：198输出全words，2组保留原构造器panic；包含domain、非有限、circle/rectangle/ridge方向与profile边界。
- 6个真实World：两dormer×三rank的实际入口→fullvisual持久实体→非空 Sparse瓦片→真实 buffer.gpu_flush dirty bytes；连续3帧不重传，cached位置编辑重建、storage实际删除/fullvisual卸载清瓦、DeleteRoofCmd退役恢复chunks。
- 12个真实World：cottage墙窗0/1、gothic墙窗7、arrow23×三rank，调用真实生成器→Blueprint/fullvisual→本系统；原跳过的0rank1/2与23rank1保持空，其余确有瓦片，随后真实删除卸载。
- 既有 allocation 生命周期5项覆盖完整WallId、重复/不存在事件、reader保留/游标、changed标记、资源drop释放；空firstprofile required失败与out-of-domain不发瓦也通过。

此处没有把完整 native Bevy ECS 容器手写后执行。主系统为完整 CFG/签名静态恢复、bay原有界区间验证及真实Rust World链验收；两个几何helper才是完整原机器码输出差分。CRT桥为 Darwin C **float** 数学函数，未执行原Windows CRT主体；证明相同数学边界的游戏算法，不宣称跨CRT全输入逐位一致。gpu_flush 记录器验证实际 sparse上传数据，**本机真实GPU/present仍由root统一验收**，没有OS真实输入验收。

## 失败与下轮复用经验

1. 半profile差分v1先暴露夹具未建立真实高度缓存，v2暴露最小墙顶0.96 literal相差1ULP，原真实最小值为0x3f75c290；都通过真实public paths修夹具，没有削弱生产required守卫。
2. v3的弯法线差异来自native测试桥ABI：Windows x64六个独立float栈参各占独立8B槽，不能按紧排Vec2复制。原ASM的入口RBP偏移回推后修桥，最终v4才36组全绿。保留v1-v3 fixtures和失败日志，禁止把错误oracle当产品算法修正依据。
3. 主调用链首轮删除panic是共享生成夹具只有桶没有d2s，补真实唯一反向索引后走真正apply_decorator_remove；arrow23首轮空是夹具ty未设置0x3a，与subtype不是同字段。完整业务type/extra合同必须同时满足，不绕过producer直接造BP。
4. 小量ULP可源于顺序、RNG、半精度或旋转符号。先分解points/cache/basis/完整record定位，不改阈值或容差掩盖。
5. 必须核原父调用的真正读写结果：未使用scratch的Shape.expand不等于Roof已扩张；原缺BP的分支是跳过而非required panic。反编译伪签名和ZST捕获要对完整ASM。
6. 新旧scope文档按时间保留：上轮生成器报告明确列出的瓦片缺口由本报告补齐；它不覆盖M09工具UI或整游戏完成度。下一依赖为原Place/Intent所需roof merge proposal，单独取证与验收后再计完成。
