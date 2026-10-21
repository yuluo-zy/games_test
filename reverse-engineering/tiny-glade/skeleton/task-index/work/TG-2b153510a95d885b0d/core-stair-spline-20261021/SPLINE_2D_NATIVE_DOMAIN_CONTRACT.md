# 二维样条原入口与密采样失败域合同

任务 TG-2b153510a95d885b0d，normal claim `core-stair-spline-20261021`。唯一生产源 `tiny-glade/crates/utils/src/natural_cubic.rs`，before SHA-256 `012d18f1d17b5fd5e45faa73b0dc548e39500f9705fda0342a970a536576a1ea`。本合同只覆盖二维原入口及其真实楼梯消费者所需失败域；当前没有执行 Cargo、rustc、原目标或 GPU。

## 原证与实际调用身份

原 B PE SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。全部路径及完整文件哈希在 `EVIDENCE_MANIFEST.json`，其条目为完整绝对路径。

复用 TG-f768c2955f5ef10ede/core-duck-building-20261021 已保存完整包：open 140c926b0 / 2611B；periodic 140c930f0 / 2170B；clamped scalar 140c94a10 / 2021B；periodic scalar 140c95200 / 2651B；dense 140809ad0 / 487B；其真实 Vec collect callback 141261550 / 425B。两 ctor 分别在 140c92c22、140c92df7 / 140c934dd、140c93675 调同一 scalar 求解器，两次分别先 X 后 Y；不是混用别的 N 或版本。

本次必要短叶完整 PDB-sized 包 `native-domain-extra-pack` 共 7 请求、2370B，report 为 ok，逐指令文件字节验证、PDB 身份匹配，target_executed=false。边界来自 `ADDITIONAL_PDB_BOUNDARIES.json`：position2d 140c92610/65B（762:252）、tangent2d 140c92660/67B（762:452）、tridiagonal 140c95c60/645B（762:35112）、periodic RHS 141941040/318B（2079:252）、clamped RHS 141941180/438B（2079:508）、periodic coefficients 141941340/452B（2079:940）、clamped coefficients 141941510/385B（2079:1224）。最后四个虽是泛型 function::call_once 名字，完整 body 包含上述 scalar solver 实际调用的业务 callback；不是把标准迭代外壳算作独立恢复成果。

## 原二维构造流程和失败顺序

1. Open 在 140c926dd..e1 比较 points.len 与 1，<=1 走 140c92fcb。Periodic 在 140c9311d..21 比较 2，<=2 走 140c93851。`NATIVE_CONSTANTS.json` 直接从同 PE 读出两个断言文字：`assertion failed: points.len() >= 2`、`assertion failed: points.len() >= 3`。最小长度断言发生在临时几何/分配前。
2. 随后真实分配 chord lengths（open n-1，periodic n，含末点回首点）、累计 knots（open n，periodic n+1，首 +0）、逐段 `1/h`。距离为二维相减、每分量平方、求和、sqrt；累计长度按段顺序 f32 连加。完整 ctor 无有限性、相邻重复点、长度正数、累计 t 严格增长或系数有限性 gate。原 norm/累计产生 0、Inf、NaN 时继续按 IEEE f32 计算，不能提前抛现代错误。
3. Open scalar RHS：无端导数边界为 +0；有端导数时 first=`((v1-v0)*3)*inv_h0 - start*3`，last=`end*3 - ((vn-vprev)*3)*inv_hlast`；内点为右斜率减左斜率（各先差*3，再乘预先 reciprocal）。原 141941207..255/257..29f 与 current quadratic kernel 的这个顺序相符。初始 upper 是有 start 时 0.5，否则 +0；first RHS 除以有 start 时 h+h，否则 1。中间 elimination 和逆向代入均不查 pivot 0/nonfinite，末边界原仍会在 end=None 时写 +0。
4. Periodic scalar 构造循环 RHS、轮转前段 h、diagonal=2*(hprev+h)，作一次 rank-one correction（两个真正三对角求解，不可退化成首尾线段），随后逐段系数。原 140c955a1..5cc 是保留 diag0 之后 doubling 第一项与 last += corner*corner/diag0；140c956f4..717 为 `(result0 - corner*resultlast/diag0)/(1+correction0-corner*correctionlast/diag0)`。现共享内核用 gamma=-diag0 的同一 cyclic algorithm，继续复用；本补丁不把 gamma 表达式改动扩散到现代 try 与 3D。IEEE 失效值仍传播，不声称 NaN payload 的逐位同一，后续 CurveTry 非有限拒绝才是原消费失败边界。
5. 原 scalar coefficient callback 14194159f..5d0 / 1419413f4..425：a=v_i，b=(v_next-v_i)*inv_h - ((c_i+c_i+c_next)*h)/3，c=c_i，d=(c_next-c_i)/(h*3)，跟随原 segment.start_t。当前 coefficients 有同一内核；维持该求解器、Horner/导数，不建立另一个楼梯私有算法。原 position2d/tangent2d 完整短叶分别确认三次 Horner 和 `(3*d*t+2*c)*t+b`。
6. 输出完整真实 segment 数、knots 的首/末 t、periodic 标志。不把坏系数清零，不跳 segment，不去重、不折叠为 line。原 Bump 与当前 Vec 所有权架构不同：保用户 Vec 架构和标准分配失败，不宣称原 Bump allocation identity/OOM 时机逐字节一致。

## 密采样完整失败域

- 140809af7..afe 先 required index（index >= segments.len panic）。之后 next segment.start_t 或 spline.max_t 减 current.start_t 得 span，不夹紧/过滤。
- 140809b2f..b36 计算 `ceil(span+span)`。140809b3b..b77 是 Rust f32→u64 饱和转换：NaN/负值归 0，大于上界/正 Inf 归 usize::MAX；直接 PE 读常数 14295a0dc=2^63、14295a0e0=f32 的 usize::MAX 下侧表示值（不是样本业务上限）。140809b7b..b84 取 max(2)。现代 Rust `as usize` 与本机64位目标对应此语义。
- count 无 `<=2^24` 门，span 无 finite/>0 门。step=span / `(count-1) as f32`；完整 callback 按 unsigned i 转 f32，乘 step，再调用原 Horner。先 count*8 的真实 Vec 布局容量/分配检查，逐 i 填 points，成功后才求 tangent(0)、-tangent(span)。数量极大时遵守 Vec 容量/分配失败，不能加入“安全”截断把错误藏掉。
- 零/负/NaN span 的 count 为 2；允许得到重复或非有限采样，交真实后继处理。正 Inf 数量饱和会在真实 Vec 容量边界失败。没有生成超大 Vec 的测试；可用 Inf 确定触发布局容量溢出而不是实际申请巨内存。

## 真实 producer / consumer 与直接 caller

- 当前生产唯一直接 dense caller 是 systems/decorator/src/decorator_visual/stairs.rs:59..60，委托 canonical 方法；duck 是该文件唯一 candidate writer，正在恢复双30KB request。新请求应调用这两个 infallible 二维入口，无局部 finite/去重/直线替代。
- 原 true14027e470/false140276e80 完整请求：periodic true14028089b / false14027927b、open true140280cc7 / false1402796a2、dense true140280d58 / false140279734 调上述同原方法。没有相邻去重/finite 门；不同合法 Terrain DecoratorId 可以具有相同 finite CachedTransform.translation，tag2 alignment offset 为 ZERO，因此重复控制点是可达域。
- 后继 `Curve2::try_new_from_points(dense,true)` Err 仅跳该 segment；整 path 所有段均无成功时才一次 manual fallback（path.first→second）。当前提前 ctor Err→expect panic 会截掉这条真实回退。这一消费者完整合同见 peer `REQUEST_SPLINE_SEGMENTS_CONTRACT.md`，本文件不改它。
- repo 全量 `rg` 直接引用：本源码、上述 stairs wrapper、utils/src/natural_cubic/native_tests.rs、utils/tests/deep_consumers.rs、utils/tests/curve_review.rs。没有其它当前生产2D ctor caller；不得推断尚未恢复 caller 不存在。
- `deep_consumers` 保存143条 dense 原 fixture，其中7条 panic 都是 index==segments.len；现正常系数、样点和端导数有逐位断言。`native_tests` 保存原2D open/periodic 逐位系数。文件存在不代表当前通过；旧 spline-comparison.log 实际 Cargo --locked 失败，本次 bounded probe 退出132无输出，均不可作通过证据，不重试原 probe。

## 拟定最窄源码方案（待独审后写）

1. 保 SplineError、现代 build_spline 校验顺序、所有 try_* / Spline3D 语义。把原 knot loop 的校验抽为可单态化回调，现代传原 guard；native2D 传 `Result<(), Infallible>` 的不拒绝回调，match 空错误类型消除不可达错误分支。只保一个 norm/累计实现，无二次计算、动态分发、新全局策略或私有楼梯样条。
2. 两二维 infallible ctor 各先原长度 assert，再同文件私有 native2D data builder；仍复用当前二维 coefficients 和 from_data。跳过 point/tangent/segment/coefficient finite gates；不修改现代 try 的严谨域。
3. Dense 移除两原无断言，其余 required index、count cast/max、step、先 points 后两导数保持。原算序中文注释列明确 VA。
4. 同文件有意义边界测试：少点原 panic、2个重复点仍得到真实 NaN系数/2点输出可交CurveTry失败而非提前panic；periodic重复闭点同样构造后失败；非有限输入/可选导数的 permissive native vs strict try；手工合法 segment 的0/负/NaN span真实 count2和导数；Inf span走实际 capacity overflow、index required panic。既存native bit fixtures和try/3D测试不改。
5. 不新增生产文件，不改 Cargo/feature/lib/调用端，不运行 Cargo/rustc/GPU/原 target。本候选只有 root 成组编译与真实业务验证后才可称集成完成。
