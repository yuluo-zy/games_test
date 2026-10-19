# Anchor-follow B 九参数及两tracker dirty实际合同 V1

root已正常claim TG-2dfe262879222ff8a5给deco-trackers-20261020。本包只读原证/接口设计，不是source候选、注册或App通过。原4640B@14081c790、PDB module466 record1772绑定B RNE/PDB；完整body已在main tracker work保存，复用不重新抓整库。普通return为d51f，其后panic/unwind仍在4640B记录范围。

## 真参数与当前缺口

原FunctionSystem fnptr候选有9参数，结合寄存器/stack及四event stride、双tracker inline实际写确认：PublicWalls；OnWallDeleted reader；OnWallChanged reader；RecalculateRoofCmd reader；AdjustWallHeightCmd reader；ResMut<ChimneyTracker>；ResMut<DoorTracker>；ResMut<DecoratorStorage>；DerivedDecoratorInfoResources。没有CachedDecoratorTransforms参数，没有single-wall参数，没有AnchorFollowWalls返回集合。

entry SP→RBP=−0x1a8；rcx/public、rdx/deleted、r8/changed、r9/roof；caller stack+20/+28/+30/+38/+40分别height/chimney/door/storage/derived→RBP1d0/1d8/1e0/1e8/1f0。9参数type metadata自身标candidate-only，不能单靠生成签名证明字段/ABI；本合同引用的原register/event/inline实际读写是补强。

现owned源是四参数单wall手工wrapper，读removed_storages冒充删除event、只标一个wall、返回Vec而不写两个tracker；只处理单wall普通路径并在尾mark_all_modified冒充roof失效。全库source调用搜索仅其定义/mod reexport，实际App未注册。public observed_from_info72B bridge还有同文件tests；该bridge不因system签名恢复而删除。旧shim可改为显式legacy API，但不能继续冠原九参数系统成功名或被注册。

## 原执行顺序和修改面

1. 先按Bevy EventReader原未读顺序消费OnWallDeleted，两段buffer上的16B row逐项取+8WallId，cb11实际Storage.on_wall_removed。不是从Storage的removed destination registry反推事件，不先清tracker/blueprint状态。
2. cb3c对**同canonical derivedResources**执行From双ref。随后消费OnWallChanged16B row，取+8WallId；cc68 wall_decorators_mut，有桶则cc75 mark_all_modified。无论桶是否存在，cc8d和cc9d都将wall加入**两个tracker真实dirtyset**。原PDB inline on_wall_changed0x64A4 records6824/6964对应cc8a..cc92、cc9a..ccad；native资源+30是dirtyset，ResMut change tick在cc84/cc92先写。当前canonical tracker.on_wall_changed复用其已有私有set，不另建HashSet/Vec registry。
3. 消费RecalculateRoofCmd原32B row，取+10WallId，cda4..ce3d是invalidate_roof_decorators第一inline；消费AdjustWallHeightCmd48B row，取+18WallId，cf44..cfdd同第二inline。两者都wall_decorators_mut，有桶才遍历**全authority map iter_maybe_mut**，只当Info+22==2，ce38/cfd8请求DecoratorMaybeMut::get_mut，标该roof装饰为modified。不能改成遍历原modified子集，也不能mark_all_modified其余wall装饰；不能合并/去重event而声称原调用频率。
4. 所有四event后才cff9 Storage.iter_walls_mut，全墙迭代；d0d7 iter_modified_mut并先next。只有存在实际modified项的wall才查PublicWalls，缺失原panic“decorators contains wall...”@d531；不能像旧shimwarn+return，也不能对空modified桶无证新增PublicWalls lookup失败。
5. 对modified Info，原dstTag+22按 `if raw.wrapping_sub(2)<3 {raw−1}else{0}`分派：side0 wall普通重锚；side1保留无操作；side2/3等非可行状态panic（d1e0/d60e）。普通路径 d220 as_wall_mut→attachment height→nearest raw2D segment→严格[0,1]t与index边界→原lerp→wallspace project_clamp；d2e1/d2e5写dst+0/+4。此部分可复用已证core math，不能改变周期长度/NaN/error策略。
6. **普通写回之后**才d2ea检查dst+21==1；若true，d2ff同View按DecoId try_get_subtype。None只跳bottom调整，**普通更新仍已写入**。当前core::move_wall_decorator_following_anchor在bottom flag时提前返回BottomDoorConsumerRequired，因此不能直接以该函数当完整原branch，也不能清假flag骗它；需canonical普通prefix独立接口或已授权原owner适配，避免重复造算法。

## Bottom-door真实原公式与依赖门

此分支已追实际算术，但尚未证明所有currentcanonical profile schema映射，不可注册成完整恢复。

- SomeSubtype后native取PublicState原value+40指向Vec3曲线、+48长度（通过bucketend−2a0换算，d30d/d330）；长度1→unwrap_failed，0→原panic；使用`maxss(current_dst_x,0)`作目标，通过原CurveSlice extend_right_until@1412898e0（remap point getter141206c40）取得segment index与t，必须满足i/i+1边界。该profile不是猜bbox/flat0，当前height_outline/prepared.positions是否就是同一条原字段尚需PublicWallState owner字段证据。
- profile_y=`y0 + t*(y1−y0)`，原d3de..d3e8；随后nativemaxss(profile_y,0)（NaN返回rhs0）。不得先clamp全部profile数据或用max_y代替profile。
- 保存Info.rank byte+44；返回Subtype ref首byte经过833250薄wrapper。原b25c90完整5B由01独核：入ECX=rank、EDX=lookupbyte，返回RAX低byte=lookupSubtype、RDX低byte=rank。现hostKey::new(subtype,variant)字段名不能反推原ABI次序；canonical逻辑key应由01提供typed接口，真实collision查View第二ref，d3d0→14125cf90，miss原panic。不得用Legacy DerivedDecoratorInfo::new或第二map补成功。
- actual collision f32@38/@3c，`half_span=(f32@3c−f32@38)*.5`；`candidate=maxss(profile_y,0)+half_span+.05`；末值为native max-number选择(half_span,candidate)，candidate NaN时回half_span（d410..d41e）。保存dst+18=末值；调用samePublicWallState.max_y@b2a620，保存dst+10=末值−max_y，dst+4=末值。本合同只绑定offset读写与表达，不凭currentBBX字段名假称原collision schema完成。
- shared DerivedResources/View 由01 TG609唯一owner恢复，Chimney Removed和本system共用。View.try_get_subtype(DecoratorId)是可空真实lookup；collision/typedkey/真实loader/currentLegacy转换仍须同canonicalowner审，不复制opening/collision registry。

## 合法实现方向及当前依赖

恢复时九参数真实系统直接调用现两个tracker.on_wall_changed，无on_wall_changed新registry；删除走真实Storage.on_wall_removed。Roof-only invalidation可在ownedsource用公开Decorators.map与modified_ids的安全split borrow逐选中ID标记，不克隆全map；最终普通modified迭代优先split借用已有modified_ids与map，避免旧wrapper按每wall collect Vec。但exact原MaybeMut/get_mut/immutable边界必须与Storage owner06合同一致，不能用mark_mutable关闭guard。现Storage.iter_maybe_mut只是共享(id,&Info)，DecoratorMaybeMut还错包整个Decorators；这是依赖适配缺口，不因名字相同称可直接消费。

普通prefix依赖core已恢复算法，但bottom提前gap与原先写后optionallookup冲突需要显式解决；完整system必须包含真实profile和collision尾部，不能只添两个dirtyset后宣4640B恢复。当前anchor source候选/注册尚未授权；本包交07后等root依原owner委派必要接口。不会修改foreign Storage/PublicWalls/Derived或共享utils。

## 覆盖/验证计划（未执行）

需真实event reader四流两帧、delete先于changed、缺bucket仍dirty双tracker、roof/height只标Info.tag2、重复events保读取进度、全walls仅modified触发重锚、missingPublicWalls原错误、side0/1/invalid分支、bottom None保普通更新、profile0/1/边界与collisionmiss、NaN/+0 native max选择、sharedcanonicalresources/World隔离。不得用返两个Vec或空collision资源伪造通过。当前没有Cargo/rustc、benchmark/GPU或App结果。
