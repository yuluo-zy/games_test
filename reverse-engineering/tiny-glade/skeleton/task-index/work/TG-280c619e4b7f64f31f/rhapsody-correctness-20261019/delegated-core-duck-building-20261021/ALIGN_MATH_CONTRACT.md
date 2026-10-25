# 与请求 producer 必要绑定的 AlignKind 数学纠错（待独审）

本轮独立于已冻结并交root的 size20 叶，不回写前交付。原full73B/109B分别由PDB515:9752、515:10004绑定，完整原 RNE/PDB SHA 见 align-math-manifest.json；完整native-pack status ok，未执行原target。

`position_offset`：140959f00..f48。先将返回Vec2置ZERO；tag1读取first，flag!=0时加second，然后scale*=0.5；tag0在 f2c..f36 只有 flag!=0 或 byte2==1 才读first并乘scale，其他情况保持ZERO；tag不为0/1也直接在f0d JNE f44返回ZERO，没有panic。当前canonical两处皆错（flag==0、unsupported panic），现Observed facade对这两支反而正确。为真实request路径必须统一 canonical 数学，不能继续保留两权威相反结果。保留原乘法顺序 first(+second)*(scale*.5)，无新增clamp/finite修复。

`outgoing_dir`：140959f50..fbC，tag0返回first，tag1返回second，其他tag才panic，当前canonical正确。勿把position_offset的ZERO策略扩散到outgoing_dir。

测试需要穷举tag0的flag(0/1)、byte2(0/1)四支并与Observed facade逐一相等，tag1两支保原结果，tag2 position_offset为ZERO而outgoing_dir仍panic。size20前叶的旧flag数学测试应在本次纠错同步，不将此前size审查声称为数学已验收。
