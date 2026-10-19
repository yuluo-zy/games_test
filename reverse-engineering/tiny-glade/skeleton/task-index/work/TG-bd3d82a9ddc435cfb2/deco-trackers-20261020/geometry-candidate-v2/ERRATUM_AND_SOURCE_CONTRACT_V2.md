# Geometry V2：原索引归一纠错

V1 candidate manifest e1b9da4e、两源码及旧合同/35bb7a21设计审全部保留，不作为可合依据。07独立source review发现本人在旧设计和V1源码把Chimney层索引分母误记为 collected heights.len()-1。原14200bf6b..bf9d先从rdi(length)减2再转f32，14200c0e8用该值除layer index。因此正确分母是 `heights.len()-2`，最后实际layer到t=1。count3时V1错得到t=.5，size_delta flare完全未进入；这是真行为错误，不是格式问题。当前生产只有已审tracker/Params协议，尚未整合V1几何。

V2只纠正该分母，并加一个实际发出Brick载荷回归：height splits3、base_size=.5、size_delta=.3，首层四砖的affine x-column长度须=.5，末层四砖须=.8；旧len-1表达会失败。原V1两emit checks都用size_delta0，覆盖不了这一反例。没有将这个未执行测试说成通过。

Door源码a446faf1保持逐字不变，07已给passed_scoped_source；Chimney新源码另审。V1 SOURCE_CONTRACT_V1.md其余真实writer、rng顺序、twoVec、failure continuation、未注册/未完整host/foreignowner初始化/Commands/gpu gates仍为本V2必读约束，数值冲突以本erratum为准。

本次仅rustfmt exit0。两源码共5项unit checks（2原Params Eq+3几何payload/RNG checks）均未在本包执行；没有Cargo/rustc/benchmark/GPU/production mutation。真实Sparse writer.push入口源码已接，但测试目前只观察同private emit实际载荷，不称已执行writer、完整host、App或native机器码对照。需要07 source审与root单独有界编译/运行。
