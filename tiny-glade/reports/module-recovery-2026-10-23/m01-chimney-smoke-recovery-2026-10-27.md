# M01 两类烟囱及真实烟雾链复原（2026-10-27）

本轮修复 `chimney_wall`、`chimney_roof` 写旧 scalar 而 fullvisual 读取 canonical builder/40B 装配参数的断链，同时恢复同一 `Smoke` 组件从生成、昼夜厚度、typed Nani 写入到删除回收的链路。没有重建框架或改变 B 目录；本报告不代表整游戏、生产宿主或 GPU 验收完成。

## 原身份与范围

- 原文件：`reverse-engineering/tiny-glade/skeleton/B/crates/systems/decorator/src/decorator_visual/chimney_wall.rs`、同目录 `chimney_roof.rs`、上级 `chimney_smoke_anim_thickness.rs`。
- 原 RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；匹配 PDB SHA256：`2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- 原墙生成器 `1407c21b0`，PDB 物理跨度 12159B；原屋顶生成器 `1407c0910`，5224B。fresh Ghidra body 分别 9538B、4073B，不把 body 大小当作包含 EH 的物理函数完整长度。
- 原厚度 `140818df0`，PDB 物理跨度 3587B；原组件注册 `140854230`；原 Nani Smoke updater `14207cd60`。
- 厚度 B 源码身份 SHA 为 `fd69a11845dbb85fcb064c10d18b38608734ffd8e47f59bf0ac9424d357b56b2`。原 `into_configs` 单态完整类型明确包含 `ResMut<GlobalSmokeControl>, Query<&mut country_core::components::Smoke, ()>, Res<TimeOfDay>, Res<GameTime>`，不是由 `void()` 占位猜参数。
- 参数/源码身份见 [原 B 参数](140529c20-15生成器原B参数.json)、[fresh 方法包](140529c20-15生成器方法审查-native.json)、[两函数原码哈希](m01-hints-chimney-native-identity.json)、[Smoke 原证](m01-hints-smoke-native.json)。B 源码 SHA：roof `7b407bb63ad14c0ea3176a34310eecce8c9cdb261cce3cbd1ea7d4985c7f9bb7`，wall `435a97c1569e3d8c3eff6ab7c79a339bbf9bd0b2cf54655b36840bb7cd23f585`。

## 生成与碰撞

`generate_roof_chimneys` 保持原六组参数：derived、storage、cached transforms、public walls、blueprints、signifiers。`generate_wall_chimneys` 保持原七组，在 blueprints 后还有真实 mesh assets，再到 signifiers。signifiers 在此二函数未产生额外副作用，不虚构提示输出。

两函数按原存储遍历墙、terrain、stairs，逐 modified ID 作 required lookup。当前通用 `DecoratorStorage::iter()` 仅返回墙，`Decorators::iter_modified()` 用 `filter_map` 静默吞缺失记录；本轮只在这两个 owner 内借用真实桶并严格取值，不修改其他模块的迭代 API。roof21/low19 在 outline 已写后调用原 `addr.as_wall()`，非墙地址保持 panic 及此前副作用；pipe20 保留非墙真实 source。缺 modified 记录在 subtype/blueprint 之前失败。

末轮独立原证纠正共享 `insert_for` 合同：原 `140825df6` 返回 Arc data 指针，`140825e09` 写 state 到 data+0x128（现 `u138`，跳过双 collider），并非 data+0x129 的独立 interaction flags。两生成器第三参保持原 state；中央已修共享API，本owner测试纠正旧 flags1 假设，状态0/1分别验真实双map注册/跳过。

屋顶21恢复：

- 高度为 `extra + 3.0`，替换旧 `+0.1`。
- 先保存 canonical transform，再作 required wall/extra 读取；第二次 cache 与 max_y 读取在 collider 建立前，保持失败顺序。
- Smoke builder 为 `nani_smoke`，位移 `(cache.x, height+cache.y, cache.z)`，原 rotation/scale，smoke `(0, enabled)`，state 非零半透明。
- 真实 cuboid 尺寸 `(0.85,height,0.85)`，local center 零，世界中心 `cache.y+half`；碰撞与交互共享同一 Arc。
- 40B 装配：`[2, cacheY-maxY, centerX, centerZ, centerY-half, half+centerY, axisX, axisZ, material, (centerY-half)+3]`。浮点字按 bit 存，保留减半高/加半高的原舍入。

墙19保留完整 wall/path/arch/relative-height/邻居碰撞判断，未用常量判定替代。可放 lower 时 lower→body，不可放时 body→fallback；三角形按原顺序展开，不 weld。原 y overlap 的 JBE 包含 unordered 跳过、arch 严格边界与 segment 零判定均核 ASM。

- 高度先计算 `without_extra=max(maxY-dstY,0)+3`，再 `extra+without_extra`，不能按反编译文本重结合。
- 烟顶 offset 为 `q*(0,0,1/3)`，scale ONE，不使用 collider 的 `.15`。
- 40B 装配首字为 `!lower_valid`，word1 为原 lower_height；word9 为 `without_extra + base`，不是 `extra+3+base`。原 XMM7 在中间计算后已是裁剪墙高差+3，完整 ASM 与逐位夹具交叉确认。

pipe20先发布真实 vent builder，再 required 读取 `decorators/vent_pipe_collision`，因此缺资产仍保留原已有 builder 副作用。vent 位移为 `cache.translation+q*(0,0,.28)`、scale 原 cache；烟雾位移再加不受 scale 影响的 `q*(-.07,1.42,.58)`、scale `.5`、smoke kind1。真实 asset indexed triangles 烘焙至世界坐标后，blueprint collider 世界变换为零位移/单位旋转；没有砖装配，也没有旧 scalar chimney 写入。

## Smoke 组件、算法与 Nani

`country_core::components::Smoke` 是唯一 canonical TypeId；builder 的 `pending_core::Smoke` 重导出它。原注册和 builder/updater 证明 `repr(C)` 16B、align8，字段 `usize key@0 / f32 thickness@8 / u8 kind@12 / u8 enabled@13`，尾两字节只是对齐。没有从旧14B数组猜业务字段。`140854920` 是返回自定义函数指针的 clone_behavior，不是无参 clone 空实现；真实 custom 指针 `1412efc10`。

`chimney_smoke_anim_thickness` 恢复四组参数：唯一厚度表、真实 `Query<&mut Smoke>`、`TimeOfDay`、`GameTime`。现重构资源身份 `ChimneySmokeThickness` 保留；移除手填 spots 的第二状态源。第一遍真实 Query 初始化完整 usize key 并收集 live keys，按原 slice.contains retain，第二遍推进并回写组件。

- 原 active 是 `!(t>=.72 && t<.9)`。原 CMPS 先生成 inactive 再取反，NaN 必须 active；简写成 `t<.72 || t>=.9` 会错。
- 表键用完整 usize，RNG seed 只取 key 低32位；enabled0丢第一随机数用第二个，其余用第一个。
- 非零 target 按原 `r*4 + ((1-r)+(1-r))`，不能改三次串联加法。
- 原厚度步为 `(sign*delta)/target + old` 后 MAXSS/MINSS。正常路径保持同运算；仅 NaN 冷路径重放原 SSE 左右输入 payload/quiet bit 以及新生 invalid 的 `0xffc00000`，不全局改变 math 或盲目归一化所有 NaN。

真实 `nani_update_smoke_ecs` Query 读取同一组件和 Transform/InstanceKey，以现 `GpuWorld` owner 的 Smoke typed buffer 原地更新；payload 尾为 `[kind,keyLow32,thicknessBits,0]`。mapped preflight 增加 Smoke marker12 与真实 `Has<Smoke>`。并行 Lantern owner 的 marker13/has_lantern 及14项 tuple 保留，本轮没有重建 Nani owner。

原生产注册证据：[ctor/xrefs](m01-hints-smoke-register-ctor-native.json)、[set36](m01-hints-smoke-set36-native.json)。厚度是 Update set36 的普通8成员 tuple slot7，window_lights 为 slot6，原 ctor `14002d220` → `in_set1400c9130` → plugin `14034535f` DL=0x24、`140345361` call、`14034536c` add_systems。其 Vec 链标记为空，没有据此额外发明 chain/run_if。Nani leaf 归既有 backend44 普通 updater tuple。生产 App 挂接归中央 agent；以下局部 Schedule 验证不能冒充宿主接线证据。

## 原指令差分和同 World 验收

[原执行脚本](m01-hints-chimney-original-probe.py) 对 RNE 作 SHA 断言，按 PE section 建内存映像，直接执行有界原块，输出以 `open('x')` 写入，避免覆盖其他 owner oracle。[JSON夹具](m01-hints-chimney-original-fixture.json) 及 [Rust只读夹具](m01-hints-chimney-original-cases.rs) 包含32 roof、64 low、32 pipe和324 Smoke step。

- roof：原 `1407c122f..1559` builder/center 与 `1407c16bc..1791` 装配。
- low：原 `1407c2c00..2d6b` 主体/高度，`1407c3b80..3ef8` 烟顶及 `1407c3f17..4017` 装配。
- pipe：原 `1407c2ffd..32a2` vent builder、`1407c34a0..4017` 烟雾 builder。
- 缓存、墙高等是显式 ABI 边界输入；low 的 lower boolean 为两条装配边界输入。该夹具没有执行原 full ECS、完整容器/资产查询或整个 lower 决策函数，不能声称这些也是原程序动态差分。实际复杂决策另以真实发布墙/路径/拱高/邻居的 World 用例检查，不把候选自身当原 oracle。
- Smoke step 取原 `1408197f7` 连续39B，Rosetta直接执行 x86 指令；[C执行器](m01-hints-smoke-step-original.c) 只附加 `movaps xmm1,xmm0;ret` 返回桥。324组包括±0、finite、±inf、正负payload NaN、SNaN与新生 invalid，逐位相等，无容差。原片段SHA `295fb0e4a643d255417f46f6c7e387ef0fe6b1e746b28974953c4b5144df54b5`。默认沙箱探针曾退出132且未落 fixture，经同一有界命令授权后完成；不把执行边界错误记录成业务错误。

最终命令：

```sh
CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline chimney -- --test-threads=1
```

[最终state-contract-v2日志](m01-hints-chimney-tests-final-state-contract-v2.log)：**28 passed，0 failed，419 filtered，0.27s**。其中包含邻居派生与实际砖装配既有回归，不把筛选后的测试数当覆盖百分比。[此前final-v2](m01-hints-chimney-tests-final-v2.log) 的28项是共享state字段修复前快照，不能代替这次复验。

非空 World 验收实际进入：三种记录 → 两个真实生成器 → `instantiate_blueprints` → `chimney_assemble_bricks` 的实际 SparseInstanceBuffer → 真实 Smoke Query → Nani maintain/typed updater → 有界 mapped upload → `apply_decorator_remove` → fullvisual/tracker removal → 实体/碰撞/砖分配/厚度map/Smoke instance全回收。正式state0和预览state1各执行整条链：两者均有真实Smoke/砖/实例，双碰撞map分别三项/零项；独立vent资产测试检查正式state0的实际Arc/source和flags0。上传 device 以现 CPU TestDevice storage 为底，额外严格检查初始化/更新范围与 buffer 长度，未声称是真 GPU；Added声效可见 kind0x13。

保留失败记录：first为真实墙 max_y 尚未 prepare 的 fixture错误；second/third为并行 owner 编译中间态；fourth为本用例缺显式 BrickTransformSsbo 泛型；fifth为24项通过。最终补全所有存储/坏 modified ID后 final有27过1失败：旧测试直接 `Decorators::remove`，只删map未维护追踪，旧 filter_map掩盖它。final-v2改为原 `DecoratorStorage::apply_decorator_remove@140a20670`，准备真实d2s并清modified/标removed/注销索引，保留严格生产panic契约。state-contract首轮27/1暴露独立vent物理夹具仍用state1，修为真实state0及原flags0后最终28/0。没有通过跳过缺记录/放开预览碰撞使测试变绿。

## 本轮可复用经验与剩余边界

1. 先看真实迭代 body，函数叫 iter 不代表遍历全部目的地；modified iterator 的 filter_map 可能掩盖状态损坏。
2. 原删除业务入口与底层 map.remove 合同不同，生命周期用例必须走原追踪入口；失败常能暴露原先夹具不真实。
3. 寄存器跨多个原块复用，40B末字必须追 live XMM 定义；用初值/反编译变量名会把 wall gap错当 extra。
4. 浮点范围取反的 CFG 对NaN有业务意义；MAXSS/MINSS需原指令位型验证，Unicorn异常浮点结论需真实x86交叉。
5. 本轮 source、PDB跨度、原执行fixture与日志哈希见 [verification manifest](m01-chimney-smoke-verification-2026-10-27.json)。共享文件wholefile SHA为该时点快照，不把并行Lantern改动算成本owner修改。

剩余验收归中央：生产原注册组与真实宿主顺序、本机 GPU 实窗视觉、资源稳定性和整模块联合验收。本轮不作OS输入、不改STATUS、不提交Git；已实现的烟囱/Smoke局部链可用于中央整合。
