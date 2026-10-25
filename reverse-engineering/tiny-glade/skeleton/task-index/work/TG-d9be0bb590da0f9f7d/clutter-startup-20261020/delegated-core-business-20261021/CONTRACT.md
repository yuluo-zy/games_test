# Canonical WallSpace::subtract 原66B与unordered

仅现wall_space.rs的ComparativeSpace for WallSpace::subtract；root同原claim授权，保clutter-startup-20261020。before c144df9480f2b15ba5a76bfe71208c3475166f8afe1968a32cbe4f231b4a4e1b。通用CircularComparativeSpace及WallSpace其它project/distance/lerp不动，不能另造业务私有subtract。

原B f507a6ce完整140c91f60/66B在TGf768/.../readonly-preferences-water/required-leaves-native-pack/wall-space-subtract-full，reportok；本worker逐条独读，直接PE常量已存subtract-half-constants.json：14295a0e8=bf000000=-.5，142925870=3f000000=.5。原入口test loop低bit，非loop直接a-b；loop先f32 d=a-b，只修正一次，无rem_euclid/多圈归一化、无length有效性assert。

完整控制流：若有序d<(-.5)*length，返回d+length；否则若有序d<.5*length，返回d；否则返回d-length。第一次原UCOMISS lower,d +JBE，使unordered进入第二判据；第二UCOMISS upper,d +JA只在upper>d有序时原样返回，否则包括unordered执行减法。因此不能写else if d>=half，NaN length+有限d时会错误保d，而原返回NaN。无f64、tie补偿、clamp或finite特殊分支。原负half相等保负half，正half相等减L；loopfalse不读length。

真实消费者：同墙stairs alignment_preferences取space.subtract(to.u,from.u)→diff.signum决定方向；合法length10,to5,from2^-26，原先f32舍入d=5→-5，当前通用wide比较以wide略小5而回+5，实际方向翻转。WallSpaceCoord::subtract同Canonical方法自然复用，旧通用CircularComparativeSpace::subtract保持wide策略，不让此修复误改其它消费者。

测试：合法半周期舍入反例-5同时直接验证通用Circular仍+5；±half tie、near两边、仅一次多圈修正、非loop直减、-0符号；looplength NaN+有限d原NaN、非looplength NaN仍有限直减。无shape/default/私有替身。root统一Cargo，本worker只候选。
