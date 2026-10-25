# 楼梯视觉唯一 owner、重置与卸载合同

## 范围、身份与准入

仅恢复现有 country-core/resources/stairs/visual.rs 的真实数据 owner、MagicSupports::clear、StairsVisualState::reset_edge/remove_edge 及必要 typed floor-chunk/API；保留已有 force_rebuild 的 existing-only 语义。不新建生产目录、registry、buffer 或几何替身。source当前before aa5e84f63204e496991be005ae6a312a80f3ade509a3135322bdef6e96449bec；TG-ad9668ff273a982820 保 owner core-stair-visual-20261021，root/lead 明确窄委派后在此目录唯一候选。目标索引C仅任务导航，本合同原证全部为B，未宣称C已验。

发行RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；配套PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。原完整reset140aa8a60/2322B、remove140aa8620/1087B在旧owner/native-pack；本目录native-pack含Magic.clear140aa8210/1039B、drop edge1406056a0/785B、drop Magic1405ffbe0/210B。以完整机器码和直接producer/consumer约束为准，不以私有反编译C或函数名推字段。

## 真实拥有的数据

原hash bucket tuple是0x110B（8B StairGraphEdge + 0x108B StairsEdgeVisualState）。旧注释把tuple尺寸当value尺寸不准确；remove原拷贝0x108B直接确证。现代Rust不依赖该私有ABI做指针偏移，按同一ownership和业务字段恢复，不用padding数组伪造源码字段。

| 原value偏移 | 恢复的唯一typed内容 | 直接证据 |
|---|---|---|
|00|Vec<Entity>，真实实体列表|reset排队despawn、drop按cap*8|
|18|Vec<glam::Vec4>，floor/top-down顶点|Duck原helper1407de8b0+callback141fe5f70真实写6点并由main flatten消费；drop cap*16 align16|
|30|Vec<(WallId,[f32;4])>，墙洞center/dimensions|main14000e6b0..ef真实push，305B add_wall_holes1407d1f80读取同槽并把4float传InProgressHoleStorage::add(kind4)|
|48|内联MagicSupports，非另一个全局资源|reset实际self+48传Magic.clear；drop逐字段|
|90|Vec<现StairsPillarProposal>|原pillars_visual14077d520/14591B于d7d9调Vec::clone<StairsPillarProposal>@141201d60（PDB1415:20760/258B），d7de..d81a释放旧槽再move clone；不是凭44B大小猜型|
|a8|原std HashSet<[u8;2]>，RandomState|main reset回传该引用；dirty-cell callback6物理1402582d0/2774B在0..63域打包u16并调用1402ca470，精确PDB107:40800为HashMap<[u8;2],(),RandomState>::insert；drop实际stride2|
|d8/e8|状态Brick/Wooden两allocation槽|reset三资源输入与原typename/XXH key直接锁定，见下一节|
|f8|原qword，reset置0；业务命名尚未知|本轮只恢复该真实存储与原clear，不把未知word编成新业务条件|
|100|force_rebuild_requested bool|原interpolate读取/原force publisher仅existing写true|
|101|原byte，reset置0；业务命名尚未知|保真实标量，不构造额外门或mask|

MagicSupports本体：Vec<StairsFloatingBrickParams>@00、计数u32@18、Brick allocation@20、Sparkle allocation@30、flagbyte@40。旧count@10实际上是Vec.len，必须移除那个假独立count。Magic是每条edge内联的实际owner，现独立Resource derive可为兼容保留但reset不接外置Magic对象。

StairsFloatingBrickParams原类型由RawVec::grow_one@14078c940/PDB414:9260/185B精确锁定。真实consumer floating_bricks14077a2f0/12847B（本目录magic-support-consumer-native-pack）bd13/1a读取该Vec；record0..28的Quat/translation/scale与现canonical country_core::utils::components::transform::Transform严格同布局：rotation0、translation10、scale1c、size30，28/2c只是Transform尾padding。record30/34/38/3c分别为累计动画progress、相位、速度、幅度门；bd68..88更新progress，be03相位加入正弦输入，be1c幅度乘正弦。恢复`Transform + 4 f32`（size40 align16），无raw padding字段、无Mat4猜测、无Default伪造几何。

原Pillar type继续复用stairs_walls::StairsPillarProposal；其当前[44] wrapper align1与原align4尚有共享owner适配，已报Duck/lead，不在本visual创建第二Proposal或改foreign source。此字段完整语义未声称全恢复，但clone/drop/缓存身份已闭合。

## 三个真实buffer与安全持锁API

原main true先fetch B1 BrickTransformSsbo(key eb575d061b90ebf5)，再B3 WoodenPlankSsbo(key153e2eb3e8651534)，再B2 SparkleSsbo(key2bf3b1a4ac675864)；reset实参顺序固定B1/B2/B3。Duck THREE_SSBO_OWNERS.json逐字读原typename，禁止沿旧实现把两buffer同泛型T。

复用已独审并交root的Sparse candidate6a5c0f15：clear_and_lock_allocation短借buffer，仅返回allocation MutexGuard；writer_for_locked消费已经持有的guard，不relock/clear/分配；原Writer::clear和新方法共享原clear_parts。reset可保state Brick guard横跨Magic对同Brick buffer另一allocation的操作，不产生重叠&mut，也不drop/reacquire改变时序。原writer current_chunk只依byte_map.len设sentinel，Magic不改变其长度；最后重新组装writer无额外写入或分配。

## reset完整顺序与typed返回

拟唯一canonical签名：`reset_edge(edge, &mut Commands, &mut SparseInstanceBuffer<BrickTransformSsbo>, &mut SparseInstanceBuffer<SparkleSsbo>, &mut SparseInstanceBuffer<WoodenPlankSsbo>)`。buffer/self借用统一返回生命周期，Commands不被返回借用。

返回原顺序的type alias tuple：(Brick writer, Wooden writer, &mut Vec<Entity>, &mut HashSet<[u8;2]>, &mut Vec<Vec4>, &mut Vec<(WallId,[f32;4])>)。不把Magic、RT或新生成默认几何塞进tuple。

1. 当前唯一edges表entry；未命中创建全部空Vec/set、None allocation、原零标量的新条目；命中原对象保留allocation/Vec容量。
2. state Brick槽懒初始化，lock+clear，guard保持直到调用者最终drop writer。
3. 内联Magic.clear先Brick槽懒初始化/lock+clear，随后Sparkle槽懒初始化/lock+clear；两guard都持有时清floating Vec.len、u32 count、flag。再逆序释放Sparkle/Brick锁。不能clear后提前释放第一锁。
4. state Wooden槽懒初始化，lock+clear，guard保持到返回writer生命周期。
5. entities drain先置len0，再逐条排队Commands.entity(entity).despawn；原高32为0终止门在typed Entity域不可构造，但保原顺序与队列时点，不同步World.despawn。
6. clear floor Vec4、wall Vec24、pillar Vec44和cell set（保全部容量/hasher），原qword f8与100/101两flag清0。不得把remove的global dirty=true移到reset；原完整reset没有该写。
7. 最后才用已持guard组装两writer，返回上述6项。无RT/physics操作，无空mesh代成功。

## remove与析构顺序

唯一canonical `remove_edge(edge,&mut Commands)`：不存在立即返回、global dirty不变。存在则从map move state，先take出entities Vec；drop剩余state，先两state allocations（Brick后Wooden），再cell set、Vec4、Vec24、内联Magic两allocations(Brick后Sparkle)及Vec64、Vec44；global dirty=true在这些Drop之后；随后原实体序列排队despawn，最后释放entities Vec。Allocation Drop进入原buffer.waste，不直接写GPU或假释放。现代Rust可在Drop中take两个allocation，按声明drop顺序组织其余owner；不制造随机新HashSet来模拟drop，不额外clone/relock。

reset不释放allocation句柄；remove释放；force existing-only只写force byte且既不clear也不新增。每个路径的副作用和缺失边行为分别测试，不合并成一个清空函数。

## floor-chunk、Nani与资源真正消费者

V1把110B注册函数进一步推成marker的结论已被真实consumer否定，正式撤回，不进入源码。PDB562:80008/public_alias只确定原名StairsFloorMeshChunk及无附加required组件，不能据此判断ZST。

真正producer是原gen_stair_floors1407dc170/9864B：dd66b Trimesh::builder→dd68c positions→dd6b0 attrib<Vec3>→dd6d4 build；GPU失败无spawn。成功后一次move GPU Buffer入唯一Arc，dd873/77读取TrimeshChunk32，dd87b/81复制到FloorChunk+8..+40；dd8ce Arc写chunk+0；dd8d5从原Info.u28(arg10)读u32，dd8db写chunk+40。dd8f9 Commands::spawn原tuple<NaniBundle<StairsFloorInstanceData>,StairsFloorMeshChunk>@14126e2d0；每chunk clone同Arc，末尾释放本地Arc，不造独立buffer/registry。证据复用Duck helpers-native-pack/fn-7dc170及helper-7dc170-calls.json。

真正consumer是原14207fcc0：Query<(&NaniActiveInstance,&StairsFloorMeshChunk),(With<Marker<StairsFloorInstanceData>>,Or<(Changed<StairsFloorMeshChunk>,Changed<NaniActiveInstance>)>)>，Res<GpuWorld>。现canonical instance_updates::build_stairs_floor已精确复制chunk+8起9word到实例，末3word0、positionZERO、sort_key1。恢复floor component必须持原Buffer强owner，不能只留地址/vertex_count或虚假Drawable字段。

拟同visual源码恢复`StairsFloorMeshChunk { buffer:Arc<StairsFloorMeshBuffer>, chunk:现TrimeshChunk, info_28:u32 }`。instance_words()逐位返回chunk8words（address两word、index_count/vertex_count合word、attributes_offset、sphere中心三word、radius一word）+info_28，不做浮点转换、默认值或位mask。现代StairsFloorMeshBuffer是必要RAII适配：当前Boundary Buffer为Copy无Drop，包装内持Buffer及创建者Arc<dyn现GraphicsDevice+Send+Sync>，Drop只调同device.release_buffer一次；不新增后端表或GPU创建路径，复用现rhapsody_contract::GraphicsDevice clone携带同原buffer_host Arc（与已集成OwnedSparseBuffer一致）。该新Rust包装并非声称原版存在额外结构，而是保存原Arc<Buffer>析构语义。组件Clone只clone同Arc。构造无Default，必须由真实成功build交入Buffer。Duck负责producer调用，lead/root唯一迁移Nani实际ECS leaf，不并写visual。

原StairsVisualComponent仅留必要type alias指向同canonical组件身份，无第二组件、无新registry。

独立的通用Drawable渲染链已有（不能直接当作本floor Nani consumer）：asset_libraries/runtime.rs::build_missing_vaos读取Entity+Handle<Mesh>→创建实际VAO→Commands插入；游戏game_loop注册并以ApplyDeferred交付；draw_lists::build_draw_lists读取Handle<Vao>+Handle<RasterShaderProgram>+Transform；production_renderer使用此draw list和真实raster资源。不能把通用renderer写成不存在。本楼梯floor实际走上述NaniBundle+FloorMeshChunk，不应把通用DrawableMeshBundle/TransientMesh receipt强套到该分支；其它楼梯几何类型仍由Duck两main逐原bundle恢复，未闭前不计可见渲染完成。

reset/remove真实floor实体despawn后FloorMeshChunk Drop减少共享Buffer强计数，最后一个owner触发同host.release_buffer，不能丢Arc留下GPU地址。其它实际分支若持TransientMesh，则其真实Drop仍入同AssetMeshLibrary回收队列，既有maintenance删CPU mesh和VAO；仅作为该不同owner的边界，不把它当floor必需组件。本visual不另造回收registry、不清整个mesh表。RT{tag1,packed edge}由main140009a02..9a28及invalidate移除；remove/reset原签名均无RtWorld，不把RT物权移到visual。StairsTopDownMesh是所有V18 Vec4 flatten后独立whole-resource GPU owner，非StairsFloorMeshChunk实体，Duck原main10d02..10d8c另锁。

## 必要直接caller与边界

- Water唯一恢复invalidate.rs TG1a71033571847c7c7a，复用原140b37180/4290B（旧header3455错）：现边调用force，缺边remove(edge,Commands)，本consumer不应调用reset。八个原owner、RT只hit才dirty和三history精确删除按其完整合同，禁止保旧无Commands壳。
- Duck唯一writer stairs_assemble.rs：两main调用新6项返回，实际interpolate测试fixture的旧reset/remove需随同转真实Commands/三buffer。不给测试留会默认成功的无资源reset。
- 现visual.rs同源测试全部迁移真实三元素类型和Commands队列，移除旧泛型两buffer+外Magic假模型。force_rebuild_with旧未用壳可移除，保force_rebuild本体；无current production外部caller依赖该旧壳。
- host/unload仍root/decorator_chain唯一writer。卸载原9451B做resource drop，没有显式entity despawn，通用unload实体清理由其原host合同负责，不能把本remove单边API冒作完整场景卸载。

## 有意义验收

同一组真实三Sparse buffer+实际World/CommandQueue测试串行重用，避免每例重复分配Brick约192MiB CPU shadow；使用现TestDevice只记录后端，不计GPU验收。测试覆盖：四allocation真实写入→reset清chunk/active/保dirty/保各allocation身份与scratch容量；held state writer锁跨Magic clear、返回writer可继续push；实体只在queue.apply后消失，多个真实FloorChunk共享同buffer且最后owner只release一次；remove存在释放四slots且global dirty置位，missing remove不dirty；existing-onlyforce保所有buffer/cache副作用；typedparams布局和真实字段位保留；实际Nani floor九word输入逐位匹配chunk+Info.u28，Chunk/ActiveInstance任一Changed门由独立ECS叶验收。tests不造geometry、不加通用renderer替身，不用函数数量或case数代闭包成功。

源码需lead独审后才写候选；最终root唯一Cargo/GPU/production/Git。此合同不宣称原主mesh两个62KB算法、Magic动画完整算法、支柱/拱业务、宿主特化和长稳已经完成。
