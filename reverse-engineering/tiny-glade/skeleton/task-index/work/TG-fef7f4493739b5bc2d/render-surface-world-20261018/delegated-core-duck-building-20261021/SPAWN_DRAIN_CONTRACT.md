# 同一 constructor 生产闭包的 drain / RNG / 复制合同

原源仍TG-fef7，中央经core_business_lead授权必要窄修。原identity-bound native-audio-callsite-v3包含drain构造240B与extend/reverse/progress/drop574B，全部字节和112+46原PDB inline点ok。

- 2310ef7读外层Vec指针，efc读len，f01置len0；完整inline帧Vec::drain，vec/mod.rs2620..2630。2310f5b是Drain::next；23111d0是Drain::drop @vec/drain.rs237。故使用plan.trajectories.drain(..)，保留外Vec容量；不能mem::take令所有者旧allocation失去复用。
- 每Trajectory owned行取点源。先raw uniform_f32作为Pitch→Duck::new。随后原Duck空trajectory Vec::extend_from_slice，从原Trajectory.points复制元素，reverse，spawn后原Trajectory.points allocation于2311155..117a显式释放。不是move Vec所有权。候选new_planned(&traj.points,audio)只做原空Vec.extend_from_slice+reverse，不保留之前clone→另一个API绕路，不抢原points allocation。
- len0合法：2310fb3容量比较允许0，memcpy长度0，2310ff3 len<2直接跳过reverse，进入2311035普通bundle创建。原没有读取first/unwrap或0长跳过；故去掉原先额外continue，API无base/unwrap_or_default。
- 原shader/phase +f0/+f4/+f8都由Duck::new清0；当前用base坐标写三字段是误重建，候选保留Default零值。构造+e4首目标计时为0，候选初始化dwell[0]=0；手工双计时数组在moving=false时第二项不被读，进入moving状态前真实goals会写两项，安全初始化0不宣称原未初始化padding的ABI。
- 2311046是同一row第二次uniform_f32，104b乘原常量VA142925900字节9a9999be(-0.3)，1053存RBP+ac；Duck构造output基址RBP-40，故是Duck+ec progress。顺序必须音频构造→复制/反转→第二抽样写progress，不能提前抽或少抽。
- current plan外层retention/规划其它几何算法不在此窄修；仅消除其出生分支重读未drain轨迹引发每帧重复spawn/7实例与RNG漂移。

随包同World测试包含两条非空+一条空done trajectory，检验每row Pitch/进度交替的6次真实Perchance抽样、三组21个原Audio实例、反转点、初phase/计时值；第二次schedule运行不再spawn/抽样/创建，外Vec ptr/cap不变；随后真实建筑事件启动同3只Duck的Quack(0/7/14)。未运行，root唯一Cargo owner。
