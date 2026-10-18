# B WoodenPlank / timberframe 调用闭包

目标仅 B，RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。Matching PDB identity、完整函数范围及原机器码 SHA 在 `native-manifest.json`。只读 RNE / PDB，未修复或修改共享 Ghidra；未动 rhapsody。依赖直接复用锁定 `glam=0.29.3`。

## 原类型与存储

原 CPU `SparseInstanceBufferWriter::push<WoodenPlankSsbo>@1421b6000` 按 instance index 左移6，写四个16B向量；原稀疏构造 `1412e5170` CPU shadow 分配0x800000B，长度0x20000，byte_map4097项，最后一项32，4096个倒序 free chunk，两张64word位图。生产旧32B/2048chunk不符合原输入/消费。现在保留当前 SparseInstanceBuffer 架构，只纠正其元素和容量。

原 typed buffer ctor `141619c80`：NAME=`WoodenPlank`，11B，位于142c7ff61；size0x800000 / usage0x22 / memory1 / alignment4；显式初始数据允许元素数<=0x20000，None不上传。上层 sparse ctor 仅允许None。`get_rg_buffer@1412cce90` 的原键为 `0x153e2eb3e8651534`，生产测试锁定该键，避免保留错误的 `WoodenPlankSsbo` 字符串。

| 字段 | offset | 类型 |
| --- | ---: | --- |
| transform | 0 | Affine3Packed，48B = 四个无填充的Vec3列 |
| gate_seed | 48 | i32 |
| piece_seed | 52 | i32 |
| wall_id | 56 | u32 |
| flags | 60 | u32 |

字段名、offset、signedness均由安装原版四份 SPIR-V 的 OpMemberName/OpMemberDecorate/OpTypeInt 独立读取，见 `original-shader-layout.json`（两 raster vertex、两 cull compute）。CPU 同时证明实际64B拷贝和尾部字面值，不只按 shader 猜结构。

原 sparse CPU shadow 的四个 rdata 向量见 `constructor-default-constants.json`：前三块组成 compact affine identity；尾部142a948e0为四个u32 `{0,0,0,1}`。它不是全零。真实两个 timberframe producer 每个 push 则复制142b14ea0 `{0,0,0,2}`，见 `producer-rdata-references.json`。flags1/2的更深语义未从字面值猜命名。

原 `resources::render::impl$1::from@14089d560` 完整63B仅删除 Affine3A 的四个SIMD填充 word：输入s[0..16]→`[s0,s1,s2,s4,s5,s6,s8,s9,s10,s12,s13,s14]`。当前 canonical `resources::render::Affine3Packed::from(Affine3A)` 已吻合，直接复用。旧文档所谓未知 `RenderInstance` 中间记录在这两个调用点实际就是完整 Affine3A，不再保留 pos/size/quat 替代物。

## ConstructTimberframeCmd

原 rectangle UI producer完整4168B `14155d410.asm`：`14155dc66` 按index*40定位，tag1@0、rect24B@4，`dc83` 单独写 roof gate@28，`dc87` 写 EventId@32。原 rectangle/circle消费者分别在42ff9/7243读取同一gate，0时生成屋顶梁。payload32B + EventId8B；不是40B payload。

现在唯一 canonical事件改成 `[u8;28] shape + u8 roof_gate + Rust安全padding3`，repr(C,align4)，size32。shape复用 `ShapeParameters::to_record/from_observed`。padding仅用于安全Rust表示，不视为原业务字段。roof_gate保留raw u8，未推断名义enum/disabled语义。forest现有slice读取仍解析同一个28B编码，不建第二事件身份。原 UI 的实际完整输入业务/启动系统仍须单独接入；本交付不声称已恢复UI producer全部逻辑。

## rectangle，9275B 原 body

1. 查询同库 WoodenPlank；allocation空才创建；每次先 clear。当前库需要 `&mut` 的 API 以 ResMut 适配，未复刻原库内部可变性。
2. tag rectangle。`as_curve3(cache+8)` 是基高，不是0；闭合曲线五顶点含重复首顶点，原样逐个生成。scale=`(.2, cache+12, .2)`，orientation_mat3→锁定glam Quat→完整Affine。
3. 相邻边距离/1.51后floor→usize；n>=2才插n-1根。位置=`a*(1-t)+b*t`，截面精确bit `0x3e3851eb`（旧0.18相差1ULP，已纠正）。主高完整保留。
4. split_height_into_skirts(main_height)，再chain `[0,main_height]`。每个ring重新 `as_curve3(0)`，四条相邻边；normal→原rotation_from_normal；scale=`(.2,.2,length)`；中心再加 `(height*0,height,height*0)`。保留原0乘法。
5. gate0时：原 roof_height、ridge_ratio、direction=(dim_y>dim_x)、RoofRidgeDims::calc 产生真正的inner rectangle。外沿base=cache8+cache12，inner peak=roofheight+base，两个闭合curve逐点zip生成五根斜梁。
6. inner rectangle perimeter>.5才补ridge梁；原SETBE选择索引(y<=x含unordered)，连接index和index+1。旧用outer rectangle复制自身/连接对角线的逻辑被删除。
7. 各push错误OR累计仍继续计数；每事件末尾写beam count，最后事件胜；末尾OOM日志。

## circle，5102B 原 body

1. 使用rectangle已经创建/清空的同allocation；空则panic，不伪造新资源。
2. count=max(6,trunc(perimeter/1.5))；as_points2_ptcount分母count-1。竖梁位置基高cache8，scale `(.2,cache12,.2)`。原 identity rotation 已从native固定basis指令确认，旧错误在缺失主高scale而非identity本身。
3. 裙层同原height序。实际 itertools tuple_windows 对已闭合的count采样点只发count-1段，禁止再modulo绕接。每段完整length/normal/rotation/position。
4. roof_gate0时，外围点在base，目标是circle.center、peak=roofheight+base；count根**径向斜梁**，不是沿外圈再做一个水平环。

共享原 utils::rotation_from_normal完整936B已复核：finite assert、归一化失败basis、最终非finite quaternion回退identity都有原指令。仅当前手写的第二范数求和括号错误，已窄修成原 `z²+(y²+x²)`，未改其他工具逻辑。

## 验证与边界

新增 actual ECS fixture 跑 canonical EventReader→两个真实系统→同真实 SparseInstanceBuffer writer，显式fixture cache(base1,main4)，CPU device只承载分配描述，不伪称GPU。rectangle2×8：gate1=21，gate0=27；circle radius1：gate1=16，gate0=22。断言完整主高、基高、精确中梁截面、真正ridge位置/方向、circle roof局部Z列=tip-base、全部flags2；另core测试验证原default flags1、4096chunk/8MiB、原hash与fieldoffset。测试名及最终source SHA在 `applied-source-manifest.json`。

本worker未跑Cargo/原Windows动态差分/GPU；只完成rustfmt，统一执行交root。RoofBuffers其它opaque资源、TimberframeConstructionParamsCache真实生产时点、UI整链注册/参数生产、音频系统旧GameTime裸offset与其事件size字段，不在这次已闭合范围；不把本次两构造系统恢复等同于整个timberframe插件或全游戏已恢复。
