# Toolbar TB-R5：原过山车1ms动态复核

本记录只覆盖 `crates/tiny-glade/src/rollercoaster.rs` 的原每车动态与速度更新，为该文件实现负责人提供可重复原证，不声明骑乘、音频、实体及卸载整链已完成。没有修改生产源码、渲染模块或共享进度台账。

原作身份：B RNE SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。原 B 路径由 `skeleton/B/crates/tiny-glade/src/rollercoaster.rs` 确认。取证主体为同目录 `toolbar-rollercoaster-update-native.asm`，反编译仅辅助变量追踪。

## 状态字段与次序

原 State：`+0..17` Vec，`+18..37` Spline3D，`+38` selected usize，`+40` t，`+44` speed，`+48` step_accum。每帧加有效 dt，冻结相机时有效 dt=0；每个子步减 `f32::from_bits(0x3a83126f)`。剩余不足1ms退出，最多1000个子步。原比较 `ucomiss/ja` 对NaN不会提前退出，不能用无法终止的裸while替代上限。

原 Cart stride `0x1b0`，动态字段为：

| 原偏移 | 内容 | 使用关系 |
| --- | --- | --- |
| `17c..184` | J：三分量世界惯性 | 摄像机偏移、羊反应触发 |
| `188..18c` | Bx、By：两个摆量 | 倾斜四元数与轮部音效 |
| `190` | C：第三摆量 | 与B共同计算局部惯性，但没有B的每步趋零摩擦 |
| `194..19c` | A：三分量局部加速度阻尼 | 音频等后续效果 |
| `1a0` | peak：衰减峰值 | 轮部摆动阈值与过零检测 |

这些偏移是读原 ABI 的证据，不要求 Rust 使用私有二进制布局。Rust应使用语义字段；尚无源名证据的部分不能假装找回作者字段名。

## 速度与轨道参数

`dt=.001`，先取 `Spline.tangent_at_t(t + selected*2.5)` 的Y分量。**这里及每车惯性都使用未归一化切线**；只有求 look_at 四元数前单独归一化。

原 `3b861b..883a` 顺序：

```text
raw = speed + (tangent_y * -dt) * 10
speed = max(raw, 2)
rate = f32(raw_flag_byte) + drive
if rate > 0: speed = continuous_lerp(speed, 15, dt, rate)
speed *= expf((brake + brake + .01) * -dt)
speed = clamp(speed, (1-brake)+(1-brake), 50)
tail_t = t + f32(cart_count-1)*2.5
start = spline.t_max - 8
u = clamp((tail_t-start)/(spline.t_max-start), 0, 1)
smooth = (3-(u+u))*(u*u)
target = lerp(speed, 2, smooth)
speed = continuous_lerp(speed,target,dt,max(speed-2,1)*.5)
bottom_flag |= raw < 2 && tangent_y > .2
t = clamp(t + speed*dt, spline.t_min, spline.t_max)
```

`lerp(a,b,s)=(b-a)*s+a` (`140c904d0`)；`continuous_lerp(a,b,dt,r)=(b-a)*(1-expf(-dt*r))+a` (`140c904e0`)。必须保留算术顺序，不能额外 clamp插值系数或以线性速度推进替代。`raw_flag_byte` 的 getter `1408c5680` 只有 `movzx byte[rcx]; ret`；Ghidra显示的其他ICF别名不是此资源的语义证明。后续主实现负责人从原 `rollercoaster.rs.structures.json.gz` 的 `RollercoasterState.procedure_occurrences` 内完整FunctionSystem类型核验了tuple末项为 `Modifiers`，因此此处读取 `Modifiers.flags` 低byte（disable_snapping）。此类型归属证据与getter算法证据分开保存。

## 每车动态完整公式

本节每一个乘加均为独立 f32 运算，不能随意使用融合乘加，也不能归一化传入的q。

令 `oldTan=Spline.tangent_at_t(old_t+index*2.5)`、`newTan=Spline.tangent_at_t(new_t+index*2.5)`，

```text
dv = oldTan*oldSpeed - newTan*speed
J += (dv*28)*dt
if selected: J += ((rotate(q,+X)*steer)*dt)*6
J *= damping_J
was = abs(Bx_before) >= crossing_threshold && peak_before > peak_threshold
```

`J` selected 操控方向是 **正X**：`3b8e3c`载入的 `142925904`实际为 `3f800000`，另外两个静态向量也符合 rotate(+X)。不能因入轨朝向使用NEG_X而把横向物理力也反号。

定义 `h(x)=(1-abs(x)).max(0).min(1)`。羊 transform 查找成功时使用体积质量系数，否则使用120：

```text
mass = ((((shape_x+shape_x)*shape_y)*shape_z)/shape_reference)*120
local = rotate(conjugate(q),(dv*mass)*dt)
if selected:
    Bx += h(Bx)*((steer*dt)*5)
    By += h(By)*((signed_zero(steer)*dt)*5)
    C  += h(C) *((signed_zero(steer)*dt)*5)
Bx = (Bx+h(Bx)*local.x).max(-1).min(1)
By = (By+h(By)*local.y).max(-1).min(1)
C  = (C +h(C) *local.z).max(-1).min(1)
Bx -= min(abs(Bx),friction)*copysign(1,Bx)
By -= min(abs(By),friction)*copysign(1,By)
peak = max(abs(Bx),oldPeak)*damping_J
cross[index] |= was && abs(Bx) < crossing_threshold
A = (oldA+clamp(rotate(conjugate(q),(dv*32)*dt),-1,1))*damping_A
```

`signed_zero` 是 **位与符号掩码所得±0**，原 `3b8483`只保留横向输入符号位；不是signum，也不是第二个输入轴。预先计算的 `steer*dt*5` 再乘h，不能仅按反编译器显示的交换顺序实现后声称逐位等价。C无趋零摩擦是原流程，不应为了复用统一Vec3摩擦而改变。每分量`.max(-1).min(1)`反映原 NaN回退行为；Rust `.clamp` 对NaN的返回语义不同，边界审查时要区分。

峰值max有额外逐位约束：相等时保留oldPeak；oldPeak为NaN时取abs(Bx)。原`maxss`+NaN选择序列在±0相等时保留oldPeak符号，不能直接依赖当前平台`f32::max`零符号行为。实现负责人以1728例夹具发现并修正该差异后报告逐位通过；此为原纯动态边界通过，不代替完整producer验收。

通用四元数向量旋转必须保留原非单位q的行为：

```text
diag = qw*qw - ((qy*qy + qx*qx) + qz*qz)
dot2 = ((qy*vy + qx*vx) + qz*vz) * 2
rotate(q,v) = (v*diag + q.xyz*dot2) + cross(q.xyz,v)*(qw+qw)
```

原后续倾斜输出是 `q * from_rotation_arc(UP,normalize([Bx,By+2,0]))`；装饰物显示再乘Y轴−π/2。羊位置偏移为结果四元数旋转`[.128,.697,.068]`。C没有参与这里的倾斜向量。羊随机事件位于两个夹具片段之间：存在羊且 `length(J)>.16` 且 `fastrand::f32()>.99930024`，必须由主实现保留事件和真实组件消费，不能因本夹具跳过就省去业务。

| 常量 | 原f32 bits |
| --- | --- |
| dt | `3a83126f` |
| damping_J | `3f7efa61` |
| damping_A | `3f5a2618` |
| friction | `3b03126f` |
| shape_reference | `3e8c9ba5` |
| crossing_threshold | `3727c5ac` |
| peak_threshold | `3ccccccd` |

## 可重复差分夹具

`probe_toolbar_rollercoaster_cart_native.py`：直接执行原 `1403b8d68..1403b9260` 与 `1403b94b0..1403b95d9`，产生1728例 `tests/fixtures/native/toolbar-rollercoaster-cart-dynamics.ron`。没有桥接业务函数；前后未归一化切线、速度与look_at结果是明确输入边界。跳过两片段间的羊随机事件发送，只恢复原9263在A片段前的常量1加载。覆盖selected/非selected、羊/无羊、不同切线/速度、边缘摆量、正负0以及略非单位四元数。

`probe_toolbar_rollercoaster_speed_native.py`：执行原 `1403b841e..1403b844e`阻尼及下限算术，再执行 `1403b861b..1403b883a`速度/t更新，产生5400例 `tests/fixtures/native/toolbar-rollercoaster-speed.ron`。原lerp、continuous_lerp、raw-byte getter均执行其原指令；唯一桥是原import `expf`→本机Darwin libm `expf(c_float)`。因此这是明确带跨数学库边界的夹具，不能宣称与UCRT expf逐位相等。

以上脚本使用tools/venv的Python + pefile + Unicorn，正常执行需要允许本机JIT。执行命令与脚本同名，不启动游戏或窗口。JSON同名报告保存了列定义、原二进制身份、指令区间、文件哈希和边界。

## 本轮经验

1. 分清原物理函数主体CFG字节数与PDB/EH总体大小；不能拿start+PDBsize推断连续代码覆盖。
2. ICF别名不能决定资源类型，读取getter本体与真实调用指针更可靠。
3. 向量shuffle先落成寄存器lane表，常量读取原RNE字节，避免将相似方向/负零误认为另一个轴。
4. 复杂算法可以按明确依赖边界生成原指令夹具，但报告必须标出跳过的ECS/随机/设备边界。局部夹具不是整链完成。
5. 非单位四元数夹具能检出“使用看似等价的单位q公式”这种简化；保留原表达式优先于不受证据支持的优化。
