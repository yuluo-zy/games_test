# 活板门动态 collider 与真实输入门合同

范围为该10源canonical直接consumer的最后trap源；保已有视觉架构，不把现Hole/visual占位洗成全恢复。source before f39131636473db4d2a1186e0490717cf110b6200988193d37607753244777a67，TG0e52 owner core-visual-data-20261021已live。仅work候选，root统一production/验证。

## 原身份、完整函数与纠错

B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，matching PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。TGdfec主work native-pack/tg-0e52e2ec64528325e0 原完整7587B（1407be730），本work input-gates-pack完整382B helper statusok、trap-input-constants-and-gates.json 直接PE映射常量。water旧collider合同尺寸第一lane错误，已独立errata；本合同采用同RNE真实 (1.3,.5,1.3)，绝非 (0,1.3,1.3)。本次还纠正此前口头Some允许的错误：原明确None允许、Some跳过。

## producer到entry原门与失败次序

1. 原beb55查询PublicWalls.roof_entity，然后Bevy Query实体/代数/组件存在检查beb67..bec4c失败跳过当前wall，成功取得唯一Roof引用。现代同现cottage模式 `walls.roof_entity(wall)` + `Query<&Roof>::get`，缺失/组件缺失跳过，不造Roof。
2. bec53调用Roof::is_flat，bec64 JE到外wall循环，因此仅flat允许。完整16B helper@1408e3380比较constant142925984 bits3dcccccd(0.1) >= Roof+30并SETAE；NaN false。与cottage/gothic的非flat相反。先于iter_modified，因此非flat墙上的缺ID/cache不会错误panic。
3. bee24 required ID→subtype原唯一owner；只5/10继续。无Info.ty新过滤，无mesh表/固定rank0。其他subtype不读cache/extra，不创建entry。bee6a required CachedDecoratorTransforms，缺失保持panic，不fallback默认pose。
4. 读取原Roof enclosure本体：tag Roof+8低bit；payload Roof+0xc。矩形tagodd：原24B field {axis@c,center@14,size@1c}，copy为typed Rectangle2d字段，直接原contains_point_padding(cacheXZ,Vec2::splat(f32::from_bits(bfee147a)))，false跳过。padding两lane均-1.859999895；不是全尺寸1.3/不是1.125。原130B与当前canonical helper对照一致，借用现函数不造算法。
5. 圆tageven：原16B {center@c,radius@14,rotation@18}；原new_rotated@140c97560带cacheXZ、probe radius f32bits3f6e147a(.9299999475)、原roofrotation，原intersect_circle@140c97a40。calleeNone tag0/Some1，bef21 CMP0，bef30 JNE skip：**相交Some拒绝，None继续**。这是“洞不能碰圆边”的实际交点测试，不能替换contains/bbox。需配对TGee62恢复207B原float域，保持同圆/NaN原Some拒绝。无原tag<2额外guard，遵低bit。
6. bounds允许后bef4c WallId→StorageAddr::Wall，bef65 Info+47原state到第5参；bef7d insert_for(id,Wall(wall),info.state())。先真实entry再构造/clone shape，失败保原metadata/allocation复用与部分mutation。不要末尾legacyinsert或清零flags。

## 原真实动态 shape 与外内owner

bef89从xmm9保存全dimensions.xy=(1.3,.5)，bef92 Z literal3fa66666=1.3；bef9c/befa6全localtranslation0，befc3唯一一次RaycastMeshData::new_cuboid调用。真实halfextents(.65,.25,.65)、local identity rotation/hash真实值；cached.scale不参与。

bf000..bf322克隆RaycastMeshData：owned buffers按原Option语义复制，内部原SharedShape Arc引用+1；clone data在全新outer Arc1。bf335..bf3d9把原data move到另全新outer Arc2。两个outer Arc不同、内shape Arc相同、hash/localpose/全部缓冲值相同；不是Arc::clone同outer，不是创建两个同值primitive。依赖root修canonical world shape持有/Clone，当前Box<dynShape>还不能保原内共享。该必要依赖未解决前本source不偷造适配/不判全闭包。

bf43c set_positioned_colliders(cloneArc1,originalArc2,&cached.translation,&cached.rotation)。bf441..bf484检查唯一fullTransform槽未写、然后保存cachedrotation/translation/scale全三Vec4到blueprint+10/+20/+30；现代set_cached_transform(transform)。slots一经Mesh要求sourceAddr，必须前面canonicalinsert_for真实entry。全缓存位姿供真实instantiate_decorator_colliders消费，不再vec旧collision payload。

## 现架构与边界

保单一DerivedDecoratorSubtypes required owner，像cottage特殊动态生成不需要读DecoratorMeshLibrary，不另造表。新增Query是原参数必要现代adapter。raw Roof投影只本source现私有typed helper，不新生产文件，不解码另一份registry。不触及root App/Cargo。

当前原后续DormerHole1.125、entity panel/shader/outline仍有旧零payload/未挂接业务，**不能宣称这次恢复它们**；本轮只必要编译和真实collider/门/owner闭包，不无证改其复杂数学，也不沿零payload给“完整游戏已完成”验收。

## 真实World验收计划

flat rectangle真实Roof+PublicWall publication+modified subtype5/10+WindowExtra/Info.state1+原cachedpose，运行generator，再instantiate真实world colliders：两槽Mesh不同outer、同inner/相同geometry/hash，sourceWall/interactionflags和全cachedtransform正确；shape halfextents(.65,.25,.65)，scale不参与。多轮uniqueentry缓冲复用/共享prevsnapshot保留。

外门覆盖missingroof/query/notflat先跳过即便缺ID；flat目标缺ID先panic；非目标subtype不cache；目标缺cache在bounds前失败；矩形padding切边与圆相交返回Some拒绝，圆严格套内None允许，同圆SomeNaN拒绝，reject均不insert；允许后requiredstate源消费。真实资源/shape与fixture不得以mock geometry/默认成功替代。root统一cargo，本文不是已运行声明。
