# 原set10/set25顺序与卸载必要增量 V2

本包是完整原证调查加三个最窄候选，并非完整宿主挂载。旧window-host-contract-v1保持冻结；其member14概括缺少实际中间组装阶段，本合同纠正。新源码只补当前scoped卸载helper遗漏的真实四表注册器、原插件卸载入口委托该唯一helper，以及现有测试夹具的必要资源初始化。未写生产，未Cargo/GPU，未挂载set25或完整OnExit。

## 原身份与证据

原Plugin17521B与outer19 chain4650B继续复用同任务window-host-contract-v1/NATIVE_HOST_PROOF_V1.json绑定。新HOST_UNLOAD_ADDITIONAL_BYTE_PROOF_V2.json逐指令核18完整PDB范围共6141条，含原9451B卸载、七个必要preconfig、两个早晚stairs包装器、fullvisual三成员链、中间四成员tuple、三个组装constructor与三个实际run_unsafe。所有字节与B RNE一致，PDB范围亦核。三真实业务call由ASSEMBLY_CONSUMER_BINDINGS_V2.json保留PC/目标/指令字节/PDB身份，不能仅凭长泛型签名猜系统。

## set10与set25

原set10：clear_out_wall_holes1407f98c0/31B经原wrapper2943b0/495B和in_set2957c0/1118B注册Update。原global stage set10在set25前。原墙洞清帧不是clear_out_visual_entities一部分。

原set25：最外层19成员`.chain()`，不是`.chain_ignore_deferred()`。这决定Commands可见性，不能为了少flush改错外链。必要两个flush都由原254d90构造，wrapper252850直接调用8d8360：一次在collider-only之后、validation之前；第二次在unleash之后、lantern/UI之前。原member0/1是墙高/墙编辑重锚与UI/自动杂物分组，不是早晚stairs泛型。其完整typed tuple参见SET25_NESTED_TUPLE_MEMBER_TYPES_V1.json。

外层顺序：

1. member0 config105360 `.chain()`：墙高/重锚、编辑事件子组、tracker标脏、末尾cache。
2. member1 config26c790 `.chain()`：自动杂物、预配置UI/交互组和后续UI消费者。
3. member2 cache。
4. member3 config287410 `.chain()`：on_stairs_change → restore → generate_playermade_stair_requests<false> → interpolate → restore → cache。因此原版会在该组末尾再次cache，不可删成一次。
5. member4 derive_decorator_subtype；member5是clear_blueprints/clear_out_visual_entities无内部chain的tuple。
6. member6十五generator无内部顺序tuple；member7 collider-only；member8 flush；member9完整validate。
7. member10..13归档/剔除/历史消费者；member14是两个并行的预配置链F/H；member15unleash；member16第二次flush；member17lantern；member18UI dispatch。

## stairs专化实际绑定

STAIRS_SPECIALIZATION_VTABLE_BINDINGS_V1.json从实际PE读取vtable+0x50指针，两个387B包装器已全字节核：

- 早组config287410 index2，vtable1429a9da0 → run275590，2756e3直接call276e80，确定请求`<false>`。
- 晚组config286610 index1，vtable1429a9e58 → run275720，275873直接call27e470，确定请求`<true>`。

晚组F原`.chain_ignore_deferred()`，完整18成员typed列表保存在JSON，当前只锁必要请求专化而不把其它相同签名成员自行按名字映射。它与下面H是外member14内部并列配置，没有跨F/H全序。Duck负责两个62KB真正mesh producer，不能把请求专化当mesh泛型或统一run_if；Duck另证true只处理不在RemovedSupports集合、false只处理在集合且二者均先dirty/interpolated门，宿主需保真实两单态。

## fullvisual之后遗漏的实际中间阶段

H=1403d6ce0/1898B原`.chain_ignore_deferred()`有三个成员：

1. instantiate_blueprints（完整视觉消费者）；
2. 无内部chain的四成员tuple（1400d4290/888B）；
3. copy_blueprints_to_prev。

中间四成员是：G预配置屋顶瓦片组、chimney_assemble_bricks、door_stairs_assemble_bricks、upload_dormer_holes_to_gpu。G=492d80/919B原`.chain_ignore_deferred()`，含屋顶瓦片组装与删除屋顶维护两个消费者。三直接身份已锁：

- ctor161670/vtable14295d228 → wrapper160950/1609B → PC160d86 call7f8db0 chimney_assemble_bricks；
- ctor5cddc0/vtable142a07458 → wrapper5cd2e0/907B → PC5cd4e5 call7ff130 door_stairs_assemble_bricks；
- ctor3e6c20/vtable1429d51b0 → wrapper3e61d0/773B → PC3e63bc call7b1d40 upload_dormer_holes_to_gpu。

不能继续使用旧helper的fullvisual→copy二节点近似，跳过真实砖/瓦片/孔洞消费者。以上只确认注册身份和先后，不等于其全部原业务源码已验收。现屋顶瓦片生成源仍有dormer_roof_height/is_bay_window返回None的缺口，不能在当前状态挂入假装有画面。

## scoped卸载唯一owner必要修正

完整原9451B卸载合同继续引用TG-bd3d82a9ddc435cfb2/deco-trackers-20261020/decorator-unload-contract-v1/CONTRACT_B_V1.md和RESOURCE_OPERATION_AND_OWNER_MATRIX_B_V1.json。原参数是16+5共21项，执行顺序中affordance.clear在graph.clear之后；最后才Commands remove_resource<StairsTopDownMesh>。本包复用已有system_decorator::lifecycle::ExistingDecoratorUnloadOwners，不在插件复制第二套资源reset实现。

StairColliderRegistry现已是真实canonical四表，旧合同“缺失该源”过时；它必须required地插在StairsState与DoorTracker之间，用Default赋值drop旧四表并释放capacity，不用map.clear保留旧分配。此资源拥有registry记录，不代表全局RaycastWorld collider全部消失。新helper两测试检查四表capacity归零、current/previous蓝图共享Arc仅各drop一次、灯重置、实体不被该helper擅自despawn、topdown资源到deferred apply前仍存在，以及缺required注册器不会运行部分reset。

插件decorators_on_glade_unload只委托这同一个scoped helper；没有OnExit注册hunk，不能与root窗口灯片段无序重复挂载。现有general clearing源码已存在，使用Without<KeepOnLevelUnload> Commands.despawn，是独立实体生命周期owner；本consumer本身没有despawn循环。真实onexit二者的deferred消费与同buffer废弃回收仍须由root整场验证，不凭资源表空了当GPU释放完毕。

测试adapter必须按顺序：corelead graph fixture c7deea3a→10bffbae（只把span key改(edge,0)），再应用本包10bffbae→registry-init版本。不能拿本包完整candidate覆盖尚未合入graph接口的旧production；manifest显式列该依赖。

## 实际可达性与未解除硬门

当前run_game真实调用build_startup_resources；build_recovered_visual_systems仍仅定义。它目前反向after(UI/unleash)、resource_exists门与旧简化fullvisual链均不能作为恢复完成的依据。本包不向这个错误helper继续塞两个fn，不拿event/resource存在门掩盖未恢复数据。

下一可合并App wiring必须成组具备：完整cache9331/实际两次请求/插值/原派生与清帧、全部必要generator、两次flush/完整validation、原F/H并列及H中间四阶段、fullvisual/copy真实owner、sameWorldNani canonical consumers。corelead负责cache、graph、stairs mesh/visual owner；我负责宿主唯一候选，后续据他们冻结API逐hunk配对。

完整卸载仍需PlaceInteraction活跃载荷、Audio原四map、StoneColumn第二registry、StairsVisual真实edge/entity/RT/三buffer与TopDownMesh实际ManagedHandle Drop闭合。Graph新候选正在同canonical owner收口，本包不覆盖它。不能因本次补齐注册器/方法委托就宣称完整9451B语义或会话重开已通过。
