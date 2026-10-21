# 请求 caller 的 Spline 与密采样复用边界

原身份遵守本目录 manifest.json，B RNE f507a6ce / PDB 2d9686b8；仅静态原证，当前未执行原 target 或 Cargo。

## 实际 caller 与控制点

完整 true 请求 14027e470/30226B：140280322 的 BL 选择周期或开路径；周期 14028032a..86e 按 path 邻接 pair 取 a 的 position+AlignKind.position_offset(width)，每 pair 一点（闭环 path 首尾同 ID，故不复制末 ID）；14028087c 要求至少三点，14028089b 调原周期 ctor 140c930f0 后 1402808b7..8e6 才追加第一点供下面的逐段 zip。

开路径 14028036a..86c 同样逐 pair 的前端 a 加 offset；首点时 tag0 取 firstVec2、tag1 取 secondVec2 为可选起始导数，tag2 无导数（14028082b..86c）。最后 140280bc9 调 closure1(last, previous) 取得末控制点，tag0/1 对应 outgoing 的两分量取负作为末导数（140280bce..c06），再 append 控制点，140280cc7 调原开 ctor 140c926b0。

原 caller **没有去重、有限性门或控制点非零距离门**。有效不同 DecoratorId 可以有相同 cache translation；alignment tag2 的 position_offset 是 ZERO，故两个不同 terrain 节点同位置会原样传相邻重复控制点。未发现可据以证明现 ctor 强校验不可达的上游不变量。因此不能把现代 Spline2D 成功正常域与原等同扩大为完整失败域等价。

原两 ctor 均只有最小长度门，随后真实求解，没有原 finite/zero-span 提前拒绝；现 natural_cubic::build_spline 会 NonFinitePoint、NonFiniteTangent、InvalidSegment、NonFiniteCoefficients 提前返回 Err，公开 infallible 方法把 Err panic，失败时间/是否 panic 与原不同。

## 密采样原全体487B+callback425B

原 140809ad0 真实 PDB451:60160；完整包 ../dense-spline-readonly/native-pack。按 index required segment（out-of-range panic），span=next.start_t/current max_t - segment.start_t，count=max(ceil(span+span) as usize,2)，step=span/(count-1)。先真实 Vec 分配并逐 i 计算 segment.position_at_local_t(i as f32*step)，再计算 tangent(0) 与 -tangent(span) 输出。无 span finite/>0 断言；无 count<=2^24 限制，原极大 count 走真实 allocator 错误边界。

现代 utils::Spline2D::densely_sample_segment 数学正常域相同，但两项主动断言引入原未有失败门。不得在请求本地另造 sampler 规避共享唯一 owner；应由 root 对 TG-2b153510a95d885b0d canonical 方法协调最窄修复，保现 Vec 架构。2026-10-21 live show 此任务 claim=NULL，duck 未 claim/写 shared。

## 验证限制

旧 native_spline_rdp.py/fixture 确系绑定 B 并真实执行 native/std/bump，仅替分配/内存接口，正常样例缓存可作原侧证据。旧 spline-comparison.log 实际是 Cargo --locked 失败，不是验证通过。本次只适配 PE 路径复用旧脚本的 bounded probe 进程退出132，无结果输出，不能据之给任何失败案例结论；记录 bounded-probe-attempt.json，未继续重试/另建 pipeline。

现 Vec owned 架构保留用户重构。原每请求 Bump、每 path reset 的 allocation retention 与现代逐 Spline Vec ownership 成本不同，不能声称 allocation identity；不能重造平行 Bump Spline。C 行为独立未验证。
