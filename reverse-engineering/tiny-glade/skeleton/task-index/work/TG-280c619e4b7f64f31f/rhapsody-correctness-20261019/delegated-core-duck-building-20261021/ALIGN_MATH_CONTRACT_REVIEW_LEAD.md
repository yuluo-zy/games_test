# AlignKind数学合同独立复核

复核人 core_business_lead，2026-10-21。已读完整73B position_offset和109B outgoing_dir，允许在原source追加独立于size20冻结的数学候选。

140959f03首先XORPS返回ZERO；f0d unsupported直接f44返回ZERO。tag0在f30 JNE f38或f32比较byte2==1时才加载first，故条件flag!=0 || byte2==1；当前canonical flag==0和panic均错误。tag1 first+(flag!=0时second)，再乘(scale*.5)，不得重结合。outgoing_dir tag0/1分别first/second，unsupported在f85..fb7真实panic，不能把position_offset的ZERO策略泛化。保已有Observed facade作为独立交叉检查而非新双权威；中文解释此前size叶未验数学。测试矩阵按合同，root统一Cargo。
