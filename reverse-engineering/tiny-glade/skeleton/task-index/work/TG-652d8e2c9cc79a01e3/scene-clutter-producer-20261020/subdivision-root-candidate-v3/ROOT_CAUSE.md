# 分段测试旋转符号失败的根因

root真实运行v2测试，5项中4项通过；折线测试第二段在逐分量期望处失败。原诊断Y actual=+0.70710677(bits3f3504f3)，手写expected=-0.70710677(bitsbf3504f3)。这不是2e-7容差，也不是已发现的ARM差分。作者把Euler常见q表征写进fixture，漏了原矩阵转Quat的Y-major选支。

实际Curve2节点(0,0)→(2,0)→(2,2)，采样u0/2/4给三点(0,0,0)/(2,0,0)/(2,0,2)。第二段delta=(0,0,2)，原normal=(-delta.z/len,delta.x/len)=(-1,+0)。原共享helper构造的basis为right=(0,0,1)、up=(0,1,0)、forward=(-1,0,0)。

完整原B140c90fd0/504B已逐条核原RNE字节、连续范围与PDB大小。140c910b6比较0与forward.z=0，910b9 jae进入91105；91105..110得到dif10=up.y-right.x=1、omm22=1-forward.z=1；91113/116不走X-major。91118..14d走Y-major：four_ysq=2，inv4y=.5/sqrt(2)，Y=2*inv4y=+sqrt(.5)，W=(forward.x-right.z)*inv4y=-sqrt(.5)。因此固定原表征是(0,+.70710677,0,-.70710677)，与当前实际Y一致；原W从此路径确定为负，root可用整quat诊断继续交叉核对。

本机真正锁定glam=0.29.3（不是旁装0.30.x）。其neon、sse2、scalar的from_rotation_axes均使用相同m22<=0且dif10>0的Y-major分支，直接输出four_ysq*inv4y和(m20-m02)*inv4y。这里m20=forward.x=-1、m02=right.z=1。utils既有cardinal恢复测试也已有-X方向Y正W负断言。没有证据需要改变共享旋转或分段算法。

V3仅修正这个fixture的两个符号，并增加真实基向量Z→-X、Y→Y的断言。原逐分量阈值2e-7保持；恢复算法全部不动；不把q和-q无条件视为同一个业务值，因为原Transform/蓝图ordered equality逐分量比较二者。候选基于root当前生产before（含它新增bits诊断），不会回退该诊断。未自行运行Cargo、rustc、测试或GPU。
