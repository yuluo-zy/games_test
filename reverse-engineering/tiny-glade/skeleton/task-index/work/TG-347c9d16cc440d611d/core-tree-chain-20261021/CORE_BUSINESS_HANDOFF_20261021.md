# 核心业务恢复：2026-10-21 有界实现交接

本文记录本轮实际源码、原证和整合边界，不把历史统计、索引行数或合同数量当完成率。生产与Cargo/GPU由root唯一整合；本业务组只写合法work候选及独审。B原RNE身份f507a6ce、PDB2d9686b8；C运行等价仍未由本组证明。

## 已交付且root通报实际CPU通过

- 树编辑第一阶段：真实UI输入→原随机消耗/History→TreeCmd→prefab事件；原原始局部fastrand与实际通知顺序恢复。root通报17/17通过；保留首次PrefabNameRef测试API错误日志，不将修测试算新业务。
- 鸭子建筑事件：实际CmdCheckForSheepInsideBuilding读取→真实墙形/高度→同Duck Quack句柄消费。root通报9/9通过；Events::send按pin Bevy0.16源码校正，原失败保留。
- 水活动：真正RaycastWorld/同100cell owner→有界terrain+collider查询→更新密度；独立sampler桥复用既有FloatGrid。root通报terrain2/2、sampler3/3、水活动3/3通过。
- Info::window_extra：原required tag6与低u32有效载荷恢复，genericextra宿主方言不重写；root通报2/2通过。
- 水ambience init demo门：Demo在创建loop/Grid/Commands前早退；root已消费相同after文件。真实Startup/Update38及Native/Disabled同core接线由root进行；不能因此宣称整套音频原侧已验证。

## 已最终源码独审并交root，待同组实际验证

| 闭包 | 可执行内容 | 真实依赖/边界 |
|---|---|---|
| 树signifier第二阶段 | 原五分支builder、真实Mesh/Drawable/TransientMesh、两真实caller、无每帧Box泄漏UI icon | 四源paired-handoff，原Empty是零索引网格，GPU必须正确跳零draw，不能塞三角形 |
| Duck构造/计划/消费 | Quack/Pitch/TakeOff/Land/4实例Flap，真实AudioLoop，drain保容量，空轨迹合法，每row两次RNG，WaterAudioGrid真实密度 | root已配对整合4patch/两host hunk，正在解exact log-once0.3.1；本组未Cargo，不能标全Duck规划/Quat完成 |
| canonical装饰四表 | Vec<Triangle>+同Arc<RaycastMeshData>+Rectangle3d；真实outline enum；ID→Subtype唯一map；required借用 | 同owner值迁移，非完整业务验收；producer、所有真实caller成组前不能独立破坏production |
| 原26分支装饰loader+box | 228JSON注册路径，110required资产路径，52full/34demo keys，真实indexed/convex/bbox/Sphere/empty+outline；原24顶点36索引入口 | 8原包23719字节重绑定原RNE；6真实loader测试与box原3增3测试未Cargo；watch失败注入未执行 |
| canonical数据API三源桥 | 实际collision metrics、roof宽度、已恢复balcony snap高度读同record；真实geometry夹具 | 只typed owner接口，不将旧full merge/calculate里的rank/sign/family/selector/角吸附缺口谎称完整恢复 |

以上每包的精确before/after、patch与source-review都在各任务原合法work；root消息保存了绝对入口。代码变更会使对应旧审查/测试结果失效。

## 当前正在收口的必要阻点

1. 唯一subtype producer原4576B并非简单清空+两窗TY。必须全wall→terrain→stair插表、重复ID完整诊断/panic、完整chimney19/20第二遍（真实required cache及width/height/距离）；同文件singular恢复完整8×dst分支，half kernel用prepared positions和arch_segment_heights懒读取，exact0/6/22+rank。源码在写完整实际ECS验证夹具，不用legacy bbox fake。
2. canonical四表所有旧visual调用仍须逐原合同迁移真实key/mesh/transform。没有rank0默认几何/额外registry。完整窗producer由decorator_chain负责，十余其它visual不因为通用bbox可画就计已恢复。
3. 真正cache9param的roof63/terrain63/dst4仍缺原业务，真实genericclutter会required transform失败。必要stairs assembly-request/AdjustedStairEdge需要同原算法恢复；原dst4未匹配仍写identity/负100/scaleZERO并更新dirty，而非returnNone。不得补普通identity默认掩盖未实现。
4. 原set25时序是cache先于derive，后clear/generator/collider/flush/full-copy/unleash/UI；不能沿当前错误tuple顺序或因读cache就随意重排。host原链由decorator_chain/root恢复。
5. Demo WaterAudioGrid问题尚无闭包原证：原water init Demo不创建Grid；原move参数required，plan有body early return而move逐鸭despawn。pin Bevy0.16缺required资源默认panic。最外set门/原errorhandler/另producer仍待原证，不能无证插Grid或把整个move加Demo run_if。full demo=false本轮主验证可继续。

## 原证驱动流程经验

- 先以真实producer/consumer固定输入与owner，重构现手写算法。依赖derive、注册胶水和工具连通不计业务恢复。
- PDB短名/ICF别名会误导：树随机门、Duck建筑事件和demo全局曾被错误命名；用实际caller、完整机器字节、PDB wrapper和数据指针共同定位。
- 数值别只看反编译：MOVSS上lane归零、MAXSS unordered、零乘Inf/NaN、加法操作数顺序、严格边界与第二次RNG都可改变业务。
- 共享schema恢复要连原loader/IDproducer/真实caller一起交付。保留重构架构，删除假bbox mesh语义，不建兼容镜像隐藏缺口。
- 保原alloc/生命周期：Vec::drain不是mem::take；每帧HashMap.clear不是重置Default；Arc共享与Vec独占、先watch后发布、替换drop顺序要明确。
- pin依赖源码先查API再写测试。rustfmt parse不能发现Bevy Events写法或类型契约错误；root实际Cargo保留首次失败，再作机械窄修。
- 合同→独审→源码→独审→root成组真实验证；candidate/handoff审核成功不能升级成原侧等价。复用原证包并重验身份/指纹，避免反复全量采集。

主渲染GPU、场景编辑/存档重启以及整体项目接受状态由root报告；本文不覆盖或替代其真实GPU结果。

## 本轮追加冻结/新增原证

- subtype完整producer已最终源码独审通过：TGa790/canonical-wrapper-reuse-v2-20261021最终094ec067，16项源码测试未Cargo；原6ce9 before的完整patch与TG609 clear/Option接口按序交root。旧两wrapper重复数学已删除，统一canonical，原退化NaN传播不继续保错误值。
- 十个visual实际99,482B原证表明四项动态碰撞不能统一库Arc：roof烟囱本体是真cuboid(.85,height+.1,.85)/Y轴/真实pose；wall烟囱是真前段vertices/indexed、ZERO/IDENTITY世界pose；trap为两不同outer Arc的(1.3,.5,1.3) cuboid（旧(0,1.3,1.3)由MOVSD仅读一lane误判，现已按真RIP常量纠正）、native内SharedShape共享、未缩放；cottage dormer真实call10280B collision_geo helper，当前该helper空Vec/错误cache类型。后者必须显式保为缺口，不能拿loader Sphere或bbox替代。
- 楼梯requests完整原证表明其owner含两真实DiGraphMap（ID→AlignKind、ID→AdjustedStairEdge）、previous SampledStairsPoints HashMap与segment map，包含真实current/previous/dirty生命周期。不是把现Vec资源加几个字段就能闭合；完整同源find_alignment/walk_shape/fold游戏callback也是必要业务，不能当依赖泛型略去。
- 原ivy完整1655B明确ID→subtype+rank，逻辑width+0与height+1。此小包只迁移现canonical数据接口，当前ivy的原事件流/坐标/删除完整算法仍明确未恢复，不以此包标complete。


## 2026-10-21 canonical消费闭包第二次独立复核

七legacy visual旧v1仅把bbox API换为真实typed Arc，但仍走legacy蓝图insert，未填storage_addr。沿实际instantiate_decorator_colliders继续审到required expect，发现一旦挂载必panic。已撤回v1可运行结论并冻结v2：原entry时点insert_for(actualWall,Info.state)，代表性generator→两live物理map→registry测试核source/Arc/pose，保唯一allocation及previous snapshot。七v2和cottage dormer真实动态helper/parent已源码独审交root；18源唯一聚合见TG609/clutter-startup-20261020/canonical-values-candidate-20261021/CANONICAL_READY_SUBSET_20261021.json。全部仍待root Cargo/真实主App验收，trap/wallchimney/cache-stairs不伪报完成。

### Option niche不得解释为业务状态

继续追flags完整原字节，定位先前state2“live”注解的ABI误读。证据evidence/building-decorator-info/B-storage-consumer/14155afe0.asm.txt：afeF把R9输入Info留RBX，aff8把RCX返回输出留RDI；b043的RDX是Decorators，b046的R9仍输入Info，b049 HashMap::insert把旧Option写rsp+a0；b04e检查旧Option末字节rsp+e7等于2，b076只把返回RDI+47写2，即None niche。Some分支b07c复制旧72B Info再更新reverse索引。不能把反编译in_RCX叫作输入，更不能据该分支给新Info state2。当前apply_decorator_place以及文案含该错误，已报告root协调实际source TG-889aa76ca030a2e3cd / terrain-curve-20261018；TGf707只是旧审目录，不是该source owner，未擅写。另3个遍历state2 break也须逐一核Iterator返回None，不可凭相似性直接删。

原独审TGf707/building-audit-20261018/v2/decorator-storage-independent-review/REVIEW.md第3/4点已由snapshot/replay两原链独立证明live state0。经验：出现判别字节2必须先恢复传参/返回ABI及Option/Iterator niche，再命名业务字段；必须追一组真实producer→consumer到required资源/physics，而非只验局部字段。

### stairs必要叶精度

AlignKind size20已与候选24B stride区分；再读完整73B position_offset发现此前数学tag0条件反转，unsupported branch应ZERO而outgoing_dir应panic。独立合同批准后另出数学patch，不能把size叶审查冒充数学恢复。请求owner的SubSegment byte1从complete compute证明为真实flag，不能按对齐称padding。Curve2 crashy也发现shared generic fallback的f64小量补偿与原f32 underflow失败差异，已列真实依赖合同，禁止在业务处加epsilon掩盖。


## 2026-10-21 动态烟囱与梯位叶继续交付

wall19/20真实dynamic collider已源码独审，20源聚合含dormer helper/parent和wall，不再把“17”旧元数据当当前数量。wall第一个合同把原+.3误写+.1，经原同RNE常量地址1429258d8交叉核ladder常量抓出，真实bit3e99999a已修。5实际publication/prepared/worldphysics测试待root执行。3源stairs必要叶也独审且隔离patch-F0复现：core2be2ff61、best2f6c8c0b、ladder最终7eb00583（旧a9e740无效）。初版ladder把有序端点ID和无向key混同，独审用29→7升降节点反例要求改有序ID，再单独规范化current/key。

进一步mixed-world审核发现flag漏原Info.ty3d前门、arrow漏subtype23/rank1原skip及提前假premade门；恰好原loader没有23,1，旧v2会实际required失败。tree在相同已授权源内独立v3按原顺序修，不给其它8源机械添typegate。arrow原1407eb500几何helper仍是明确剩余依赖，cached adapter不能叫原算法。此前单一类型fixture到physics通过不足以宣称生产混合世界完整。

两额外canonical编译直接caller（dormer_roof_tiles一行typed get、chimney_bricks仅测试fixture）已live核foreign历史owner报root，未擅写。trap仍待root原world owner共享shape/真实buffer clone，不能用两次primitive重建或单outerArc偷换原所有权。


## 2026-10-21 最新有界交付与保留缺口

- canonical聚合已更新到21个源，包含3入口门v3替换旧v2，以及完整Circle207B叶。索引SHA36e0655a0467a9187b0f00ab5f9700ce9188a0622e6fa7e86934e61cdfe66937（随后若继续更新以实文件为准）。本轮四源均从真实生产before隔离patch-F0退出0且字节等于最终候选，生产未改。flag9ddb0464、arrow641be3dc、roof37d16960、Circle2a14ff82。源独审通过只覆盖所述入口/叶，不是全visual完成。
- Circle原两次ordered strict大于才None，零距离/NaN不加guard或radicand clamp。实际trap调用返回tag的极性是None允许、Some拒绝，同圆Some(NaN)也拒绝。矩形padding原bitbfee147a、circle probe3f6e147a；不用人读小数近似替精确边界。
- trap完整7587B原函数的动态维度已纠正为X/Y/Z=(1.3,.5,1.3)：MOVSD一次带两lane，且必须按真实RIP目标142a85d70读取；flat-only门与其它dormer非flat门相反。pending源码65424ec9尚未计入21源：缺root canonical RaycastMeshData::Clone/同inner SharedShape API，不用双建primitive或同outerArc替代。现零洞/outline/builder仍明确未完整恢复，不能据碰撞恢复挂名视觉闭环。
- stairs node/owner完整类型和最窄真算法已独审、隔离重放并交root：owner2a046da8、stairs96677170；可替换旧2be2/2f6c小叶（增量patch另备）。真实840B graph point fold、455B锚点/degree、185B分段双返回及NaN kind1，不再返回跳表索引。八actualWorld新测试未执行；foreign structure_outline适配与smallvec exact依赖必须成组由root整合。
- +80辅助Vec的原装配器MOVSS/UCOMISS证据支持Params auxiliary_samples:Vec<f32>；SampledPoints第四Vec尚无转接证据，保4B观测words，不能按相同元素大小机械改类型。元数据依原u32位比较保NaN payload。source-only derives不算业务。
- 原force_rebuild实际是已存在边value+100置1（最内source Line134），missing不创建、不清实例、不设置globaldirty；原reset清0。新canonical accessor/mark候选aa5e84f6在TGad966 work已water源码独审并patch-F0精确复现，交root，真实allocation/CPU影子/magic副作用测试已编写。真正invalidate完整4290B包含三次mark而无reset call；现手工present→reset仍是待复原调用，不能把getter完成当lazy链闭合。
- Option niche错误需要准确限制：当前apply_place并不把所有normal state0强写2，默认输入仍0；真实bug是依据newInfo.state而非insert返回old Option选择insert/update。restore也有同类返回ABI错。历史旧“强写map2”推断已撤回，storage是确定的替换/反向索引正确性缺口，尚未证明普通startup硬阻塞。

原证/复核经验另见TG0e52旧work/LEARNINGS_DIRECT_CONSUMER_GATE_20261021.md。完整2×30KB请求、compute10808B、walk/偏好/插值、canonical cache9param与两62KB装配器仍按实际未完成状态保留；合同数量、文件行数、21源窄补丁数均不是项目百分比。


### compute 独立源码收口

stairs完整source现20386293（替node96677170）；TGf768/before-frozen-20261021-compute/freeze.json为最终元数据入口。原10808B与两game callbacks正文已完整恢复、独审并从production87435与node96677两baseline分别F0精确重放，七新actualWorld测试未Cargo。原None/Someempty、ownedVec、visual marker、float/bit/mask/压缩/padding/dirty都保留，不通过默认请求伪造上游闭环；smallvec/bitvec exact依赖仍由root统一。下步在同源恢复walk/偏好/flags/双请求/插值与cache，原62KBmesh仍不因本leaf完成而计完成。

invalidate只读全链继续确认原保S=drain state.dirty_edges与T=旧Params dirty或S命中的派生集合；T用于structural map/retain，主physics/visual/RT/previous删除循环只遍历S。不能union简化它们。该必要caller仍未写源码，当前force叶只确保真正canonical字段/方法可供完整消费者接入。


## 2026-10-21 当前可执行分组导航

优先消费 `CANONICAL_MINIMAL_COMPILE_20261021.json`：28源准确ordered patches（含两个无a/b前缀的-p0），四表/真实26loader/IDproducer/全部typedcaller+两完整窗/哥特窄迁移+真实启动方法hunk。所有源在独立临时目录F0回放/最终hash相等；无新增依赖、排除stairs/cache/validator、不挂visualhost。编译/真实资产加载待root；保留body partial不冒全业务完成。

world SharedShape已root实际整合，native-physics collision65/65，包括inner Arc/深ownedbuffer/compound新增3项。该事实只覆盖本范围；旧before3859 fullpatch不可重放。

storage niche单独两源 `TG889/.../STORAGE_NICHE_SOURCE_PAIR_20261021.json` 已合同+water源码独审通过、F0/hash对齐。恢复old-return Option与owned merge/split/Drain，输入flags逐字保留，顶部tracking不变，未实际Cargo，不依赖四表切换。不要再称原正常place强制map state2；该旧解释已在原包核错，2是Option None返回niche。

requests继续原两个30KB body，同scope closure1+manual完整helper合同已独审允许实现；Shared2D spline zero-span提前panic与Locator缺id是当前consumer必要依赖，由原worker唯一writer推进，尚未source/宿主闭合。UI/history/cache/楼梯不能用本次28源数量外推完成。


### 本轮流程经验（可复用，非完成率）

- 原完整inline容器循环的判别必须追到值来自哪里。map insert返回旧Option的niche，不能按输入flags解释；IntoIter/Drain/IntoIter三条相同表面循环有不同所有权/失败清理，必须核PDB iterator ABI与Drop。先完整函数包再有界inline来源，不重复30KB反编译输出。
- 跨owner整合按真实编译闭包冻结manifest，不能按“已审数量”交包。所有patch记录strip level（本轮derive/collision_geo为p0）、独立F0重放、最终hash；root共享source只交方法hunk，避免全文覆盖同期音频。稳定最小包冻结后不要因额外已审叶频繁改包，后续增量各自明确base。
- 同名tracking方法要按实际address/owner区分：桶内destructive adapter与顶层791B保持live map的tracker清理不能混用。发现错读先保留修正链，不能擅撤回已验证主逻辑。
- 失败分支也是业务，floating guard不能“更稳健”地提前panic：真实two-terrain同位置输入会走原Spline→CurveTry→manual，现代Spline拒绝zero-span截断此闭环。恢复canonical原入口与现代try_*域精确分离，禁止caller私造直线。
- 原长请求拆helper只在完整该helper合同闭合后源实现：closure1、完整manual分别冻结；普通path一个成功段就不回退，全部失败才front/second一次。把错误推测保留在明确勘误中，不能让旧手册指导新实现。
- 业务验证分层报告：syntax/F0/source review不能代替Cargo，Cargo不能代替实际原资产52/34keys与Arc/triangles，CPU installed-system audit不能外推未挂visual/cache/validator；真实GPU门只由root执行。


## 后续原子包与实际状态补充

root已实际整合28源，并在两Triangle真实module路径和角窗ComparativeSpace导入窄修后，actual tiny-glade check v4通过。旧frozen source不可回盖root这些修复，导航actual snapshot保current SHA；完整28组单测/真实资产loader仍等待root。

下一编译包 `STAIRS_REQUESTS_MINIMAL_COMPILE_20261021.json` 是10源+两个exact直接依赖hunk，无host；完整false/true requests最终767ace7424双独审，包含3个按新storage真实remove→place修正的fixture。源码原门全闭不等于原注册specialization/cache/mesh完成。

独立Locator最小5源入口在TG738 `LOCATOR_MINIMAL_SOURCE_PAIR_20261021.json`：四full候选+仅当前cull两行构造ID，不要求完整validator/UI/display大包。peer始终是完整cull唯一writer，后续validator必须对root实际after重基，不能叠旧V4。

factor40B `ROOT_FACTOR_LEAF_20261021.json`只邻接utils/lib方法+2tests，保root其余lib原字节。完整interpolate8622B合同已独审允许源码实现；同数reuse取old geometry/异数取当前插值、old XZ/output XYZ两个类型、NaN Y原XZ成功与opaque第4Vec保持是必要反例，Tree正在canonical policy增量，不能私造曲线。water并行只读cache9331合同，duck唯一stairs/cache本体writer；root唯一生产/Cargo/GPU。


## 本轮收束与实际CPU验证

用户新指令：本轮其它核心业务推进完成后由root提交推送，随后全面转渲染性能/画面完善。本组只收已开interpolate/cache必要闭包，不扩新模块；不足源/consumer者明示。root仍唯一生产/Cargo/Git writer。

实际root business17 check25.69s通过；utils151/151，明确包含二维原失败域7test，不包含随后才合的factor2test；decorator完整requests6个系统场景全pass（逆ID几何、true/false固定一跳、Demo保留、cycle/isolated、direct失败保双边、重复控制点真实manual回退）。未挂原host，因此不是实际UI楼梯全链验收。

同轮core2817pass14fail1ignored、decorator334pass6fail完整失败日志保留。已独立原证核并交窄修：9资产根由root加TG_INSTALL_ROOT；colorSSBO原短名、replay2测试真实Extra/有效flags0、stairsserde原数组期望；chimney局部真实prepare_paths、凸包负例共线、NaN仅非x86放sign而x86保完整bit。待root重跑，不把修候选当pass，不改生产算法/阈值凑结果。旧NaN v1无条件mask已撤回。

后续source唯一writer：duck写完整interpolate/stairs_assemble与cache；tree只补已证XZ constructor policy；water只读cache合同与本轮旧fixture窄hunk。cache仍有原607B xz_rotation_from_forward必要API与完整producer尚未闭，不能以roof的bad替代terrain方向。原fullmesh62KB、host specialization、真实Info stair producer/UI、C等价不因本轮helper/fixture通过记成完成。


## 本轮最终 CPU 结果与最后两源增量

root带真实TG_INSTALL_ROOT实际三crate全绿：country-core2831pass/0fail/1ignored，system-decorator340pass/0fail/2ignored，utils153pass/0fail；原B真实资产full/demo另2/2pass。日志`/private/tmp/tg-core-round-final-tests-20261021.log`。以上包含全部原窄fixture、factor、storage/Locator/双requests等已整合源；前述历史失败日志保留，不修改生产算法来过断言。

最后两源有序入口`INTERPOLATE_XZ_MINIMAL_COMPILE_20261021.json`：先Curve3XZ af00→3275955f（3新增unit+旧tiny构造断言纠正），再stairs_assemble 7eb→67a1ed11（完整8622B六资源consumer+10真实World/history测试）。二者完整源码独审/F0精确重放通过，当前未执行新增13case/Cargo，由root统一接收验证；未新增依赖，不mountApp系统。原oldXZ非有限Y成功而outputXYZ失败、opaque第四Vec保留/dirty不清、同数旧geometry/异数当前geometry、真实资源门与删除已源码化而非仅报告。

cache最终只冻结完整9331B合同（必须合读算序addendum），生产仍f67cb1d3，未写新cache source，不称通过。最窄缺口是canonical607B `xz_rotation_from_forward`；roof63用bad而terrain63用from_forward，zero/NaN/overflow回退不能私造bad(-v)替代。原wall/roof是直接insert，不加terrain/stair的dirty比较；逐item29B get_mut只mark集合，别混顶层323B immutable门。下轮准确入口TG6613/HANDOFF_CACHE_20261021.md；Info63真实producer、62KBmesh、原宿主specialization仍明示缺口。本轮到此收束，等待最后两源实际验收后由root提交推送，之后按用户指令转渲染性能/画面。
