# Canonical Brick/Gothic 两源冻结源码独立审查

**通过，无静态阻断；必须与已独审builder配对整合。** 绑定SOURCE_DELIVERY_V1.json 108c3bccb0c3e1171f94725673f14aac6f3f0bf367e6f6ac2f89d37f1b1abed4：components/mod.rs ba3edf70→06858b80、render/nani/systems.rs 0ba537b2→af8faaba。作者lead，审者Water未写两源。独立读全部两patch/候选、原3084/3794完整包的实际业务块与typedQuery、共用build/World维护/flush/removal路径；两原包1521条指令再次逐byte对f507PE一致。复用已准入合同74d7551f和peer设计审960631a0，后者未冒充本次SOURCE审。

## 载荷与唯一TypeId

BrickColor透明六u32，from_words/words和小端from_native_bytes无浮点运算/掩高位；保第5word全值，solid consumer自身shl31、Gothic consumer自身movzx byte差异由已恢复build_*处理，没有把某shader规则上移共享类型。Gothic字段为4+2+3+3+3 f32数组且reprC，四字节对齐/60B；from_words/words顺序是原主producer已重排的side4、edge2、first3、center3、second3，不能误用gethat返回顺序。测试包括NaN/-0/Inf rawbits往返。

当前production旧构造只有builder两处TupleConstructor，pairedTG1298 builder必须同时由root整合；lead已独审该不同作者候选4e845→2ce17098，SOURCE_REVIEW_LEAD_V1.json 3d9e3540。该pair以canonical重导出+from_native_bytes保旧rawfields与!=2分支，避免本core两源单独产生永远无人写入的第二TypeId。本审不自称独审了作者peer的全部Gothic主producer。

## 系统/宿主/失败门

- 原kind6在1420764e1/4e3取得真实ownedsubset后才启动Query，kind7在1420770fd/0ff同样；新两函数每次系统只取一次相应锁，空查询仍走该required门，未逐行锁或新建GpuWorld。
- typed Query真实域仅With对应marker+required key/Transform/Brick（Gothic加Deformation），没有Changed/descriptor额外过滤。当前宿主用&mut InstanceKey修复dense hint是已有架构适配，bypass_change_detection明确避免伪Changed；不是声称原查询可变。没有row Vec、payload镜像或新registry。
- build_solid_vertex_color_bricks(p,r,s,brick)与build_gothic_window_bricks(r,p,s,brick,def)参数顺序正确，现原数值与halfpacking内核复用不重写。Gothic原def+18..38三组3word和+0..14三组half在新words映射中完整，不复制另一矩阵算法。
- 维护preflight的14marker nestedtuple/11输入outertuple与解构计数匹配，kind6需Brick、kind7需Brick+Deformation，Transform与exact1marker原宿主边界保留；缺payload/歧义在key发布前panic，不能默认值或沉默skip。Bevy0.16 QueryData tuple上限15已读pinned源码，这两嵌套tuple均在范围内。该preflight是现代宿主断言，不冒原新增业务分支。
- 两update加入现同update tuple，maintain→显式ApplyDeferred→原无序updates→同flush顺序不变。实体删除仍RemovedComponents<Key>→真实Local.active.remove→GpuWorld.remove_instance；payload/dirtyrange/计数沿同OwnedSubset更新与flush，不另有清理表。缺subset及Externalsubset原required失败策略复用现canonical，未新增默认Owned。

## 六测试与机械复现

2组件测试+4 ECS测试均逐读：real World/real MeshAtlasLibrary/真实两OwnedSubset及shadow；明确GPU尾字/15f32序；修改Brick/Deformation后更新、desc移除仍With-only；key.id保留、bypass不产生Changed<Key>；真实flush64/112B范围/roll计数与despawn最终归零；逐payload/marker/Transform/歧义失败前无key；空Query也要求subset。MockDevice仅代替硬件写入，不是虚构成功业务几何。新组件/Query/Commands/World/MockDevice API与pinned版本未发现静态错配。

两文件独立正反F0均rc0且after匹配交付全部hash，edition2024 skip_children解析均rc0。未Cargo/rustc/GPU、未改生产/candidate。当前core两源不能单独宣称Gothic主producer、资产注册、set10/25、真实画面或卸载全链已完成；root三源同闭包执行CPU，GPU另验。
