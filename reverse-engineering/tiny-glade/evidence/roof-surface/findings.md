# 屋顶曲面与弧长曲线：原指令差分结果

原作输入 `D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne`，SHA-256 为 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。PDB 精确过程范围来自 module 498、1607、759；[过程索引](procedures.json)、[曲面指令](surface-slice.json)、[曲线构造指令](curve-slice.json)、[采样指令](sampling-slice.json)、[inverse_lerp 指令](inverse-lerp-slice.json)保存原始证据。

可运行 Rust：[surface.rs](../../reconstruction/roof/surface.rs)。[验证脚本](../../scripts/roof-surface/verify.py)实际执行成功，[验证报告](verification.json)记录覆盖及 CRT 身份。

## 确认的算法

- `Roof::get_roof_shape`：`0x1408e4920`，477 字节。参数 `profile` 在对象 `+0x2c`，先计算 `q=1.5*profile+(1-profile)*0.5`，再计算输入截面坐标 `t` 的 `t^(1/q)`，由此混合屋檐底部与屋脊顶部尺寸。矩形为标记 1，其他标记进入圆形分支；有效游戏类型的枚举定义仍需类型消费者确认。
- `Roof::profile_curve_normalized`：`0x1408e3420`，851 字节。生成 **20 个**原始 f32 常量坐标 `u`，纵坐标为 `u^q`，最后一点强制 `(1,1)`；调用原作 Curve2 构造器。该链是离散曲线采样，不能据此宣称整个游戏使用解析高次曲面或 NURBS。
- `profile_curve_ws`：`0x1408e3d40`，758 字节。把每点变成 `(u*top+(1-u)*bottom, v*height)`，再重新计算 Curve2 弧长和归一化弧长坐标。其 SIMD 路径与标量尾部路径均已按原指令验证。
- 矩形底部两个维度均添加 `0.28 + eave_length_ws`。顶部两个候选维度是 `dimension*ridge+(1-ridge)*0.1`；根据 `+0x3c` 的低 bit，另一维固定 `0.1`。`Width/Length` 到该 bit 的名字对应关系尚未验证。
- 圆形底部半径为原半径加 `(eave_length_ws*1.2+0.28)*0.5`，顶部半径为 `0.2`。输出只写前 20 字节，后 8 字节原样保持。
- `RoofShapeParams::eave_length_ws`：`0x1408e3c30`，62 字节，**返回单一 f32**，不是 Ghidra 猜测的两个值；表达式为 `(sqrt(profile)+(1-sqrt(profile))*0.5)*(1.5*eave+(1-eave)*0)`，保留原始乘零项的舍入/非有限行为。
- `Roof::height`：`0x1408e3260`，278 字节。`height` 参数 `<=0.1` 返回 0；其余为 `height*12+(1-height)*0.4`，再受形状尺寸相关 smoothstep 限高约束（圆形从 8、矩形从 9 向 12 变化）。平方先计算 `u*u`，再乘 `3-(u+u)`，保持 SSE 舍入顺序。NaN 分支及 min/max 按原 SSE source 选择实现，panic 格式化未验证。

## Curve2 构造与采样

`utils::curve::Curve::try_new_from_points<Vec2, CurveCoordSemantic2D>` 在 `0x141469430`，698 字节。累积各相邻点的 `sqrt(dy*dy+dx*dx)`，每个累计值除以总长度得到 `points_u`。原对象 56 字节：点 Vec 的容量/指针/长度在 `0/8/16`，points_u Vec 在 `24/32/40`，总长度 f32 在 `48`。

构造器少于 2 点返回错误 0；零总长或归一化结果非有限返回错误 1；当第三参数低 bit 为 1 时，检查相邻重复点并返回错误 2 和段索引。有效类型以 Rust `Curve2 { points, points_u, length }` 实现，保留原数值，不复现分配器指针地址。

坐标查询 `0x141467ce0` 先验证输入有限，夹紧到 `[0,1]`，以偏向右侧的二分搜索找到区间，再调用原 `utils::inverse_lerp` `0x140c904a0`；点查询 `0x141467270` 使用这个坐标进行 f32 线性插值；单位切线 `0x141468f90` 两个分量均乘以逆长度。Ghidra 伪代码曾漏掉切线第二分量的归一化，实际实现以指令为准。

二维单位切线已经还原，尚未据此验证完整屋顶世界空间法线、法线贴图或着色器输出。

## 实际验证范围

所有输出比较使用精确 f32 bits/写入字节，没有容差放宽：

| 项目 | 成功比较次数 |
|---|---:|
| 完整 get_roof_shape（边界/随机） | 3,004 |
| 完整 height | 2,500 |
| 完整 eave_length_ws | 2,500 |
| profile 点生成到消费者入口 | 209 |
| WS 点变换全部循环路径 | 240 |
| 完整 Curve2 构造（含错误分支） | 460 |
| 完整坐标、位置、单位切线组合 | 5,513 |
| 原作 history 参数截面 | 6,560 |
| 原作 history 参数高度 | 1,312 |
| **完整 profile → Curve2 → WS → Curve2 调用链** | **1,512** |

组合采样的每个 case 分别执行原坐标、位置与切线函数。完整 profile 链比较全部 20 个点、20 个 points_u 和总长；包括随机参数和原作 328 个参数快照，各快照使用合成圆形/矩形、两个 ridge bit。原作 history 文件 hash/pointer 来源见父级 `roof-fixtures.json`；测试未重放原场景和墙体，不能当作完整游戏一致性测试。

前三种函数在独立 Python 进程映射原 PE 并调用实际 x64 指令，仅解析其 `powf` 导入；没有执行 PE 入口和游戏。当前 host 把 `api-ms-win-crt-math-l1-1-0.dll!powf` 解析到 `C:/Windows/System32/ucrtbase.dll`，版本和 hash 记录在报告。该环境与原作进程实际加载的 CRT 版本关系尚未观测。

曲线相关调用链使用 Unicorn 执行原指令。桥接仅涉及内存分配/释放与上述实际 CRT powf；数值构造、搜索、插值、切线、inverse_lerp 全部保留原机器码。原分配失败路径、对象指针相等性、浮点异常标志、panic 日志不属于验证范围。

## 参数名字和接口

`Roof::new` 的原指令已经证明它把 28 字节参数块拷到 `+0x24..+0x40`。几何消费者确定：`+0x24/28` 位移、`+0x2c` profile、`+0x30` height、`+0x34` ridge、`+0x38` eave、`+0x3c` 方向 bit。Rust `RoofShapeParams` 采用原作 JSON 命名，**其字段名字与具体序列化偏移之间仍是几何语义对应推断，尚未执行原反序列化消费者来确认名字**；字节偏移和数学行为则已经独立确认。

接口 `section` 返回 `RoofSection::Circle { frame, radius, word_0x18 }` 或 `Rectangle { frame, dimensions }`。frame 和尾 word 保持原字段信息，尚未擅自指定其旋转、颜色或其他业务含义。`normalized_profile_curve`、`world_profile_curve` 返回可运行 `Curve2`，供屋瓦布局、脊线和后续 mesh 生成使用。
