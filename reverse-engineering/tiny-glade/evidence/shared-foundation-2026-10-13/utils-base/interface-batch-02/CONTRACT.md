# utils 顶层接口批次02

基线为原 B；沿用191项已观察接口，不从新 Rust trait/free function 数量扩大分母。原 PDB void() 从未当作真实签名。源码只改 Swizzlers 和 IndexCondenser；测试与取证脚本不计作游戏接口。

| 原接口 | 原业务规则与修复 | 原证据/比较 | 当前缺口 |
|---|---|---|---|
| SwizzlerX0Y::x0y | Vec2 `(x,y)`→Vec3 `(x,+0,y)`，调用原 glam 构造 | 原入口0x140c9cdc0，64组 payload bits逐bit相同 | Rust receiver借用/值ABI与全游戏接线未验收 |
| SwizzlerXVY::xvy | Vec2 `(x,z)`+参数y→Vec3 `(x,y,z)` | 0x140c9cde0，64组逐bit相同 | 同上 |
| SwizzlerXVZ::xvz | **receiver是Vec3**，保留原x/z，以参数覆盖y | 0x140c90010，64组逐bit相同 | 同上；不能按名字猜成Vec2(x,y)+z |
| SwizzlerX0Z::x0z | Vec3 `(x,y,z)`→Vec3 `(x,+0,z)` | 0x140c90030，64组逐bit相同 | 同上 |
| Vec3SliceAs2::as_vec2 | 保留借用source pointer/length，未分配/未重打包；安全运输adapter保留Vec3原storage | module757 record848明确原body4字节，0x14067dd80仅mov rax,rcx;ret；6组运输属性相同 | **原return wrapper、投影轴、元素step/别名和真实consumer未知**；运输证明不能推导紧凑Vec2 slice，更不宣称完整projected API闭合 |
| IndexCondenser::get | 缓存index在dense keys内且key匹配→直接返回；否则查原HashMap，找到后写回Cell；未找到保留旧cache | 原module540 IPI0x74E3/record1668，remove内prefix72组返回/cache逐bit相同；module523 update_instance_mesh同inline接口 | 无独立原函数entry；原GPUWorld consumer未整链运行。恢复remove已实际调用get |
| Condex::last_known_index | 返回当前缓存；不主动查map或校正。将候选u32 cache换成原std Cell，允许shared borrow刷新 | module523 IPI0x74F0/record15836，原producer+0x49a..0x49d仅register copy；get原码cache结果与候选getter一致 | 无独立原getter entry/ABI；TypedGPUWorld payload未接线 |
| IndexCondenser::clear | 原startup consumer调用Std Vec/HashMap clear，容量保留、key空间清空；旧handle按get失效 | module554 IPI0x74EB/record21768，原uninit_clearing中Vec/HashMap clear范围；候选capacity/reset/old-handle测试 | 原完整startup尚未执行；不把候选clear测试说成原入口差分 |

坐标样例含任意32位payload、NaN payload、±0、∞，输出不经算术或normalize。GLAM、std Cell、HashMap、Vec均复用原依赖/标准库，没有重新实现这些库。`native_index.py` 原码oracle使用调用者提供的raw table/vector输入；原std hash和probe机器码执行不变，raw buckets仅是验证存储，不是游戏HashMap实现。

工具修正：第一次as_vec2按next-public上界取得416字节，其中包含邻函数；该上界明确未当作者body使用。Fresh LLVM记录证实size4，随后`query-bounded.json`修正持久worker的body并保存，坐标leaf同样限制到ret。141同址public别名不能当作141真实as_vec2 consumer。

当前状态变化见`interface-state-updates.json`。8项从无逐接口业务证据变为部分已分析；完整consumer/representation未知仍保留。接口表191分母不变，业务已分析91→99，严格未触及100→92，候选98→103；原入口15→20，其中as_vec2只限运输属性；原inline差分get另列1项。

接口批次02不恢复IndexCondenser insert/default/fmt。已有insert适配器仍不能据其名称认定原producer接口闭合；接下来应在OwnedGpuWorldSubset::extend_no_data的原inline范围比较重复key和返回Condex策略，再接真实GPUWorld consumer。

真实检查和最终源码hash见`verification.json`；它包含其他owner同时更新的全utils检查，不能用测试条数当作本批接口数。
