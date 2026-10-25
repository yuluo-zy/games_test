# Gothic真实513B projective旋转必要callee

仅B原作RNE；TG73d工作收取既有gothic主调用者必要合同，不写utils/lib.rs。该源由Water单一writer在607冻结after35ddf39a后顺序增量，不能覆盖607或另建私有旋转实现。主10658B与513/253三个完整PDB范围/所有逐指令字节绑定见 GOTHIC_MAIN_AND_ROTATION_BYTE_PROOF_V1.json。源PDB明确759:10116 513B不同于759:11376 607B。

## 真输入输出

1407e8f82 x0y（前面已从hat两端XZ差求2D单位法线）后，1407e8f95调用140c911d0。原513 ABI RCX=out Quat16B、RDX=&Vec3；语义API应为 `xz_projective_rotation_bad(direction: Vec3)->Quat`，不取Y，不先做3Dnormalize，不以607/现504代替。首部c911db/df只读X和Z；main绝不是传原2DVec2 ABI。原253 c90ed0是同家族Mat3函数（outRCX、两个f32输入XMM1/XMM2），513将对应归一化/基转换内联，不存在强行调用253的证据。

## 首归一化与失败分支

c911e4..c91205：r=1/sqrt(z*z+x*x)，加法次序Z平方+X平方。c91209..23f的bit谓词等价r为严格正、有限、非零（含正subnormal），不能用distance.is_finite或normalize_or_zero猜替代。真分支c91249 X*=r,Z*=r；假分支保持XMM0预载1并将Z写+0，即forward=(1,+0,+0)。XZ零/非有限/平方溢出或下溢均必须按此fallback，Y的NaN/Inf完全不参与。不要在成品Quat上normalize或clamp。

## 基与Quaternion

c91254..c912ad：先用确切f32乘减构造Y×forward的right，再无条件按原XYZ平方分组归一化right。不是任意选择一个与forward正交的向量。right=(z,0,-x)的正负零仍取原乘减结果；同族607若已抽原矩阵→Quat四分支，复用那个唯一原策略，不让另一个helper走宿主glam不同分支。

c912bf 以forward.z和0的UCOMISS进入四个实分支：z>0进入c912c4，右轴对角+1有序>0走c912db，否则c9135b；z<=0进入c9130e，1-right.x有序>0走c91321，否则c9138b。其它取sqrt、.5/根及lane重排全在原513B，请直接依asm逐行复用共享from-basis策略，不用本概述推导一个不同的atan2公式。最终c913c5存16B；无副作用、无panic、无资源。

完整literal 1429cd0c0=Y轴乘法向量、1429345b0=253中Y列，1/.5均在proof原字节。必须读这些具体lane，不假定未用高lane值。

## 必验边界（未执行）

同607原基转换在cardinal/noncardinal下验证Quat和其真实旋转基；保持z<=0/右对角不同四分支。输入Y任意含NaN/Inf必须同XZ结果；XZ±0/NaN/Inf/f32MAX/1e-30必须原fallback+X。正finite各种幅度、原f32平方overflow/underflow边界须锁；不能仅验证最终finite然后删失败分支。CPU/GPU/root执行归中央，本合同无运行验收。
