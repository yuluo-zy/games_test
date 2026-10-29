# M01 楼梯栏杆原算法恢复（2026-10-27）

当前边界：四个原业务体及十八参数端头闭包已在原机器码定位并恢复到B所属现有文件。木/角石352例、端头320例差分已过；主栏杆256例和真实Sparse/物理生命周期验收仍在推进，本文暂不声明完整主装配链已收口。主`playermade_stairs_assemble<true/false>`、reset、前帧缓存、生产注册由freehand/root所有。

## 原证与归属

先实际读取`reverse-engineering/tiny-glade/skeleton/B/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs`及同模块PDB记录，修改现有同路径Rust文件，没有新建业务目录。完整二进制、PDB及逐体SHA见`m01-railing-identity.json`。执行对象为原RNE（f507a6ce…），不是cf048e27…补丁分析镜像；Ghidra写操作仅在独占worker `m01-circle-20261026`，共享库保持只读。

原函数：

| 函数 | 原 VA | PDB代码字节 | 原业务 |
| --- | --- | ---: | --- |
| assemble_railings | 1407f98e0 | 13201 | 两侧偏移曲线、真实曲线重采样、分层砖、坡度/接头门、辅助参数插值、木扶手、横向压顶 |
| railings_tips | 1407fcc80 | 3821 | 十八参数；图关系、归属墙和有界遮挡筛选后的砖/木端头 |
| tips closure | 1407fdb70 | 1043 | alignment tag1、度数1、Some(Wall)、三射线顺序短路 |
| construct_wooden_railing | 1407fdf90 | 3090 | 每点立柱、两层横梁、累积扰动、原RNG、1.6坡度门 |
| add_corner_stone_railing_piece | 1407febb0 | 1395 | 方向点积选端、四类高度、两侧独立抑制角石 |

主函数旧shared body只剩2839B，存在不可达/错误截断。按原SHA/PDB边界修body，并恢复1407f9bc6的4项及1407f9bec的3项相对跳表，才得到完整54KB反编译。完整job/result、原RNE ASM和先前截断体均保留。分析原型只用于揭示寄存器数据流，不能把工具新命名当原业务语义。

## 参数与真实资源合同

主原17参数：SSR两writer、widths/tangents/positions三Vec、同一个fastrand RNG、Brick来源rawu32、Wood来源u32、UiSignifierStream引用、两端bool、StairExtra byte16原rawu8样式、subsegment kind、未活跃rail metadata、两侧抑制、两端Option<f32>、aux f32 slice。SSR起始是96B Brick writer，+0x58是64B Wooden writer；Rust用`RailWriters`借用唯一真实owner，无复制缓冲/缓存或ABI强转。

端头原18参数：StairsState、完整Requests（alignment map）、node ID二元组、RNG、两个source、style、SSR、三个slice、RaycastWorld、DecoratorStorage、UiSignifierStream、两端门。原UiSignifierStream在这两原体未活跃，保留只读参数，不凭空生成指示符。

端头不是“没有Storage对象才生成”：原`try_lookup_storage` RAX=0是`Some(Wall)`，None=3；RDX是真实排除墙。先查alignment再读图，只有度数1/tag1/Wall进入三射线；max_toi分别0.5、width-1夹到0.1、0.5，mask1、排除所属墙；第三射线方向原为(-0,-1,-0)。shared安全delegate由gpu owner补，调用真实Rapier，不转f32MAX或伪造miss。

木叶保持三次float随机后才检查aux边界；柱/梁整数seed是0..1000。坡度>=1.6只跳过第一横梁的push和整数seed，前置三次float随机仍执行。第二横梁继续扰动已扰动端点，沿用第一横梁长度。Sparse容量不足忽略该push结果，继续后续几何和随机，保已经写入的前缀。

## 原指令差分与边界

- `m01_railing_leaves_native.py`：352组、800个完整96/64B记录。执行完整木叶/角石，只桥接Sparse push收集输出，随机/矩阵/几何全是原指令。
- `m01_railing_main_native.py`：256组、3460记录。完整主入口→返回；桥接Rust allocator/reallocator/deallocator并跟踪归还、CRT memcpy/memmove/memset与明确roundf/floorf/ceilf/truncf依赖、Sparse输出。曲线构造/重采样/参数/随机及特殊fold1412938f0全部真实执行，无注入候选输出。各成功输入返回后live allocation为空。主Rust比较尚待最终绿。
- `m01_railing_tips_native.py`：320组、148记录、630次有序射线。执行完整原端头与闭包；仅桥接IndexMap/图/Storage解析结果、物理依赖返回和同前内存/输出边界。射线几何、最大距离、排除wall、调用顺序及短路由原指令产生。此探针不替代真实Rapier验收。

有限实例字段按每个32-bit word和随机最终state严格比较。单点木叶的0*Inf在ARM与x86产生不同NaN符号，单独记录为原函数退化输入边界，只对双方NaN lane作NaN比较；不得据此宣称该退化域96/64B逐字相同。完整主曲线会先拒绝过少/零长度输入，不将NaN符号修补扩散到正常业务。

当前关键日志：`/tmp/m01-railing-leaf-tests-v6.log`（叶绿）、`/tmp/m01-railing-tests-v4.log`（tips/叶绿，主首个差分case40）。后续最终日志和源码指纹仍须更新。

复现命令在tiny-glade根运行：

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_railing_leaves_native.py
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_railing_main_native.py
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_railing_tips_native.py
CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib railing_ --offline
CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib railing_ --features native-physics --offline
```

本机原指令执行需沙箱外Unicorn JIT许可，未启动原Windows进程；无真实OS输入验收。真实GPU/完整主链验收由中央任务统一执行，CPU测试设备的upload请求不能冒称GPU画面。

## 已沉淀的复现经验

1. PDB完整范围+原字节SHA先于反编译文本；跨函数坏跳表只修独占worker，不能以错误shared body当“原算法简单”。
2. 漏参和void callee原型会让Ghidra消除几乎全部矩阵字段。先从Win64 caller栈槽恢复完整参数，再补必要callee，不能把反编译少字段写成默认值。
3. 反编译文本会重排SSE浮点加法，木横梁实际是z²+(y²+x²)，还要保持端点Y先偏移再加扰动。有限值2ULP差异必须回ASM解释，不能加epsilon盖过。
4. 原enum的0不是通用None；Storage的tag0带完整WallId。用实际publisher/reader确认后才写typed match。
5. 相似的宽度公式不同：遮挡探针width-1夹0.1，端头砖几何width-0.5；不能因名称接近合并常量。
6. 原rawu8样式字段有4-case分派，别把此前代码的bool解释沿用到完整原体。非共轴/变宽/曲面/多点/多seed输入能检出单点直线夹具漏掉的问题。
7. 业务叶直接借Sparse唯一writer；保持原逐件部分写入和随机前进。O(1)无向图度数可复用pinned GraphMap邻接迭代器精确size_hint，不复制图或遍历全邻接表。
