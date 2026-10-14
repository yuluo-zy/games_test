# B utils 消费者深挖与未定位接口

2026-10-14，用户明确只分析 B。交付 `D:/game/tiny-glade/crates/utils`；未修改旧 shared-foundation 研究源码，不分析 tiny-garden。此次继续查机器码消费者、全部 PDB 行号和类型/内联信息，并修复原指令差分暴露的源码偏差。

## 找到并恢复的行为

### 真实 2D 样条消费链

两组 `generate_playermade_stair_requests<false/true>` 分别调用 B 的 open/periodic Spline2D 构造。沿调用点继续追踪，进入原 `system_decorator::decorator_visual::stairs::densely_sample_spline_segment`，地址 `0x140809ad0`，PDB module 451，过程范围 487 字节，包含异常清理尾部；正常流在 `0x140809c5f` 结束。这不是凭 3D 查询镜像推导的 2D 实现。

原入口可观察 RCX=输出结构、RDX=Spline2D、R8=段下标。Spline2D 的 segments pointer/len 为 +0/+8，min/max 为 +16/+20，periodic 字节 +24；原段 stride 44，保留三个 start_t 字段。这些布局用于 oracle，正式安全 Rust 不模仿内部 ABI。

- 段下标越界进入原 panic。
- 跨度取 next.start_t-current.start_t，末段取 max_t-current.start_t。
- 点数 `max(2, ceil(span+span) as usize)`；步长 `span/(count-1 as f32)`。
- 逐项在 `i*step` 求局部位置，包含末端；返回起始切线和**取负**的末端切线，不归一化。

正式 `Spline2D::densely_sample_segment` / `DenseSplineSegment2D` 作为该消费者的安全语义入口。原所属模块是 system_decorator；方法名与返回类型是重建适配，不声称作者原 Rust API。未凭此恢复 `Spline2D::position_at_t/tangent_at_t/find_segment_and_dt`。

### 标量构造运算顺序修复

严格比较消费者的末切线时，旧容差测试通过的构造出现 1 ULP 系数及 2 ULP 切线差异。继续检查 `0x141941180` RHS closure、`0x141941510` coefficient producer、`0x140c94a10` open solver 和 `0x140c95200` cyclic solver。原 RHS **先将每段差值乘 3，再乘预先求出的倒数，然后相减**；旧实现先作斜率之差再乘 3。c1 原码乘倒数，旧实现直接除法。已按原指令顺序修复 open/periodic 和带端切线的共享标量路径。

诊断差异来自本次实际执行与工具输出；没有把未保存的早期终端输出包装成独立原始日志。新的严格测试同时比较构造系数/knots/min/max，避免只验证消费者而漏掉上游差异。

### 有所有权投影的轴和 stride

`Curve::try_to_curve2` IPI `0x1372` 有两个内联宿主；`reinterpret_as_curve2` IPI `0x257c` 有四个，含楼梯/墙构造、楼梯 UI、curve2_serialized_as_curve3 反序列化。相应消费链调用 `0x1402ba9e0`，原标记为 std `Vec::from_iter<Map<Copied<Iter<Vec3>>, fn(Vec3)->Vec2>>` 单态化。机器码读取输入 offset 0/8，推进 12 字节，输出推进 8 字节，即 XZ 投影。

只恢复游戏选择 XZ 的策略：新增 `project_vec3_slice_xz`，Curve 两种转换共用它，继续调用原版本 glam swizzle 和 std iter/collect，不重新实现库的分配/迭代。reinterpret 保留已缓存参数/长度，try 保留已有重算投影长度的策略。

**纠正旧结论：** `as_vec2` 的同址四字节 `mov rax,rcx; ret` 只证明 RCX→RAX。RDX 未变化不证明它是长度，更不能据此确定 slice 接收者或返回类型。106 个几何域 E8 同址候选有折叠歧义，不据地址标签强行归属。当前 Vec3SliceView2/Vec3SliceAs2 是明确声明的安全重建传输适配；原借用类型、原借用投影轴/stride 仍未知。有所有权 Curve 的正面证据不能移用到它。

## 全 B 搜索结果与残余

[deep-inventory.json](deep-inventory.json) 重新扫描 4894 个模块，覆盖 LF_FUNC_ID/LF_MFUNC_ID、所有过程及嵌套 inline scope，并查 TPI。Rust PDB 的 void() 占位不作类型/ABI 证据；TPI 未提供这些结构/接收者定义。

[source-line-inventory.json](source-line-inventory.json) 独立解析全部 4858 个含 C13 的模块、85322 段 DEBUG_S_LINES；文件身份使用 DEBUG_S_FILECHKSMS 和 B `/names`，另外登记 DEBUG_S_INLINEELINES。module 761 的原 LLVM `dump -l` 输出用于交叉核对，逐条范围/行号相同。原路径和作者源 checksum 保留。仅是源码校验和，未取得作者 Rust 文本。

| 方法 | B 声明证据 | 本次继续搜索 |
|---|---|---|
| WallSpaceCoord::to_affine3 | IPI 0xaef7，wall_space.rs:198 | 无独立过程/inline host；源声明区间无对应物理行号地址 |
| WallSpaceCoord::is_bounded | 0xaefd，行 265 | 同上 |
| WallSpaceCoord::bound | 0xaefe，行 272 | 同上 |
| WallBoundCoord 同名三方法 | 0xaf11/0xaf18/0xaf19 | 同名类型不是 WallSpaceCoord；无正面过程/inline 映射，不移植猜测逻辑 |
| Spline2D::find_segment_and_dt | 0xaf37，natural_cubic.rs:164 | 无独立过程/inline host；无对应物理行号地址 |
| Spline2D::position_at_t/tangent_at_t | 0xaf56/0xaf57 | IPI 存在，未取得对应编译体或 inline 行号声明；已找到真实分段消费者 |

wall_space.rs 的 20 段物理行号范围仅覆盖当前已定位方法/自动实现；natural_cubic.rs 11 段。负搜索只描述 B 保留的调试记录，不能证明未编译、死代码或算法内容。B named streams 没有嵌入的 Rust 文本（只有 natvis 等）。没有足够作者实现证据时不造三个墙方法或 2D 全局查询。

原 .text E8 扫描找到两处 open/两处 periodic 构造消费者，局部位置一处、切线两处及 13 个投影候选。原始候选与精确 CALL 后反汇编保存在 [direct-call-candidates.json](direct-call-candidates.json) 和各 asm 文件；未检查的候选指令边界不作为调用次数定论。正面恢复依赖明确的 PDB 宿主、静态原码与实际 oracle 入口执行共同支撑。

## 原指令验收

[consumer-native-fixtures.json](consumer-native-fixtures.json) / [native_consumers.py](native_consumers.py) 绑定原 RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

- 143 组构造→dense 输入，其中 7 组原越界 panic；其余输出 points 和两个 tangent 逐位比较，同时严格比较全部构造系数。覆盖 open/periodic、各段/末段、各端切线选项、短段/直线/随机控制点。极端分配失败/巨大采样数/完整楼梯 ECS 未验收。
- 有所有权投影 10 数组 / 166 点，含任意 32-bit 浮点 payload、NaN、有符号零，点序及全部输出 bits 一致。
- 复用原已执行 fixture 的 20 组 3D 构造，新增严格检查所有系数/min/max 和 120 组全局位置/切线查询。原 fixture 输入不改，原通用 oracle 的分配/内存库桥记录保留。

执行原自有指令，沿用分配/内存库边界桥；dense 额外调用当前 Windows 原 UCRT ceilf，不以 Python ceil 替换。不是原完整游戏进程或原 CRT 文件版本鉴定。fixture 旧 limits 字段中的 “unused” 不构成未使用结论，以本报告的 unresolved 定义为准。

当前 Windows Debug/Release 各 **93 项**（87 unit + 3 dense/projection/3D integration + 3 numeric integration），无失败/忽略；utils all-targets `-D warnings` Clippy 通过，country-core **1142** CPU 测试通过。完整命令、源码/Cargo 输入 SHA 和期间变化见 [gate-results.json](gate-results.json)。首次 92 项门禁保存在 initial-gates，加入共享 3D 严格回归后重新验证。无新增依赖。

全局 `scripts/update-progress.py` 额外校验未通过：它在既有 `reconstruction/Cargo.toml` 源码哈希处中止，尚未走到本次 utils 台账。未改该共享研究 manifest 或覆盖旧验证哈希；失败保存在 global-progress-check.log，不宣称全项目台账门禁通过。本次 formal workspace 的上述测试与独立审计通过。

## 持久数据库

实际 Windows 官方 MCP SDK→ReVa/Ghidra worker `utils-gap-20261014`，patched B SHA `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。按 PDB 范围和原字节 SHA 准备六函数。dense void() 占位改为观察到的三寄存器 C 存储原型 `void* dense_segment_B(void*,void*,ulonglong)`，仅用来辅助反编译；它不是作者 Rust 签名。原反编译仍不能正确表示切线的双 XMM 返回，算法判断以机器码和 oracle 为准。

初次 const void* 解析失败报告保留；基本类型重试成功。save 与重开结果另存 save-result/reopen-result；仅修改本 Windows clone，未同步原数据库或历史 Mac overlay。工具准备/脚本本身不计业务交付。

当前库仍为 partial：墙三个方法、2D 全局查询、借用 as_vec2 原类型/访问、采样开关写入者、完整消费者/原进程未验收。A/C 按用户要求不纳入后续任务；也不宣称任意输入或跨平台逐位等价。
