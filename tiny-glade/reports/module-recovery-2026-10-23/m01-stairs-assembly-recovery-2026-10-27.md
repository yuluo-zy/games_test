# M01 楼梯装配主链调查与必要叶恢复（2026-10-27）

## 后续检查点：梯步、Mat3与安全三缓冲借用

### 缓存与资源边界复核

墙切孔24B缓存已恢复为 `StairsWallCutout { wall, center:Vec2, dimensions:Vec2 }`，边缓存同时补原Vec<Vec4>顶面。`playermade_stairs_add_wall_holes`原实际只有两个资源参数，已替换旧6参误注释/空实现；遍历所有边与缓存Vec，四f32原样发布kind4，不去重、不替换NaN/负零。原完整1407d1f80在显式hashbrown16槽夹具上执行28例/156记录，唯一业务边界桥为WallHoles.add采集；Rust真World验证重复记录、同墙多孔、帧间重新发布与删除边停止发布。全own `original_stair_`当前9/9通过（含步骤928case、真实Terrain/BrickSparse编辑/三稳定帧/owner释放），日志 `m01-stair-own-final-tests.log`。Sparse最后一个byte_map槽是原32满块哨兵，卸载验收应检查真实CHUNK_COUNT槽全空并保留哨兵，不能错误要求哨兵也清零。

`MagicSupports` 首字段已从错误的u64计数修成真实 `Vec<StairsFloatingBrickParams>`。原grow `14078c940` 确定元素身份；原consumer `14077a2f0`，尤其 `14077bd63..be20` 和第二次循环，交叉确认64B记录为canonical Transform48B以及出现进度、相位、频率、幅度四个f32。进度按delta/.3增长，phase/frequency进入time*(frequency+2)+phase，amplitude参与原浮动幅度插值。此处只恢复typed缓存/clear，不声称整个floating_bricks系统已恢复。完整fresh C与有界ASM保存 `m01-stair-floating-params-contract-native.c`、`m01-stair-floating-consumer.asm`。

`clear`清Vec元素且保留容量；force_rebuild继续保留缓存和真实allocation。core visual当前8/8通过，日志 `m01-stair-magic-vector-tests.log`。旧报告下文“64B身份未确证”由此后续证据取代。

真实步骤资源测试揭示两项必须分别记录的边界：请求采样使用 `TerrainHeightsData<VisualTerrainHeightSsbo>`，装配数值读取 `TerrainHeightsData<TerrainHeightSsbo>`，二者TypeId不同，不能共用前者假装原参数齐全；已恢复generate_requests真实把aux留空，当前interpolate亦未填充，因此直接执行steps产生零砖。该结果是主链缺口，不能用生产默认1填补。新增leaf资源测试仅显式给定aux夹具，验证实际Terrain/Sparse编辑/上传/卸载；不把该夹具当完整请求链证据。原distortion独立阶段已交同组owner恢复，仍需确认实际写面并完成联合主入口验收。

五叶检查点之后，`construct_stair_steps` 已按原完整4617B恢复数值与96B writer内核；928组原执行/6,460条记录的24个word和最终RNG均与Rust逐位一致。覆盖不同点数、弯曲切线、升降/等高、四端标志、减宽、窄/中/宽随机分割及aux门；原最后bool只已证减半厚度且过滤aux均值<.1的内部层，**不能称UI预览模式**。原terrain采样与random_splits在原侧均真实执行，Rust逐字测试只在既有terrain采样接口读回已记录高度，生产adapter仍调用canonical TerrainHeightsData，实际World/terrain/Sparse生命周期另外验收。

原 `utils::xz_rotation_mat3_bad` @140c90ed0 已回到实际B归属 `crates/utils/src/lib.rs`，169组原无桥输入（负零、subnormal、极值、Inf、NaN）逐位通过。矩阵不能经Quat往返；梯步caller的预归一化与Mat3内部再次归一化都保留。随机宽度高低分支同样不能将原u64高位采样换成Rng::bool。

主装配还需同时借用canonical Brick/Sparkle/Wooden三缓冲。经中央owner确认，原库没有现成安全多借API；本轮窄补 `FrozenMap::get_disjoint_mut`（标准HashMap互斥键借用，无新unsafe）与 `AssetSsboLibrary::get3_mut<A,B,C>`。重复键明确panic（缺失键同样拒绝）；每个已存在entry均核完整type_name；全部downcast成功才记录三路changes。CPU影子剥离后的render快照仍返回None，不能恢复游戏写权限。原始Box owner不移动、不复制缓冲，不重新实现渲染器。alias/missing/type冲突/change与snapshot六项测试通过。最新分层收据追加在 `m01-stairs-assembly-leaf-validation.json`，没有覆盖先前历史基线。

`gen_stair_floors` 已由中央分配给hints独占函数与真实48B chunk owner，rails四叶由circle负责；本owner继续steps/reset/主装配/支持请求。双方只在同文件各自函数分段apply_patch，不能整文件重写。主装配依然未完成，本文下文的五叶表为先前检查点，不把新局部记录视为整链验收。

## 验收边界

本轮恢复并验证五个必要算法叶；`playermade_stairs_assemble<true/false>` **仍未恢复**，不能注册其空体或宣称楼梯主装配完成。本文与 `m01-stair-dirty-structural-recovery-2026-10-27.md` 的插值、懒订阅、dirty、结构阶段分开记录；后者已验收部分未修改。当前源码里主装配、梯步、楼层、栏杆和部分支持资源仍明确是缺口。

原 B 源归属已实际核对：`skeleton/B/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs`，以及 core 的 `resources/stairs/visual.rs`、`resources/stairs_walls.rs`。未新建业务目录，未迁移 rhapsody 结构。

## 身份、控制流修复与真实参数

原 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；B PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。

两个单态物理范围分别为 `140008c90..140018036`、`140018040..1400275a6`，PDB 物理字节为 62,374 / 62,822。不能拿这些物理字节当恢复比例：范围内含大量独立 EH 函数。共享 Ghidra 原 body 截断并超时；独占 ReVa worker `m01-freehand-assemble-20261027` 仅修此两函数，排除范围内其他独立函数并补原四张相对跳表：

| 单态 | jump VA | 原 table | 分支数 |
|---|---|---|---:|
| true | 140009a4f | 142927880 | 12 |
| true | 14000a784 | 1429278b0 | 4 |
| false | 140018d5b | 1429278c0 | 12 |
| false | 140019ba3 | 1429278f0 | 4 |

结果分别为 36,901 / 37,387 owner body bytes、282,970 / 248,332 字符可用 C。完整原 ASM/C 和元数据已存 `m01-stairs-assemble-{true,false}-native.{asm,c}`、`m01-stairs-assemble-{true,false}-cfg-metadata.json`、`m01-stairs-assemble-cfg-prepare.json`。不再以旧工具 timeout 解释主实现缺口。

原 FunctionSystem 是 11 个顶层 SystemParam：AssetSsboLibrary、GraphicsDeviceResource、RaycastWorld、StairColliderRegistry；六资源 tuple（AssemblyRequests、VisualState、ArchSupportRequests、BracketsRequests、TerrainHeightsData、RemovedSupports）；UiSignifierStream；Commands；RtWorld；StairsState；DecoratorStorage；最后 PublicWalls/Query<&Roof> tuple。原必须先取得 BrickTransformSsbo、WoodenPlankSsbo、SparkleSsbo；dirty 精确等于 1 且 interpolated_curve 为 Some 才进入。main 完整分支仍需继续恢复，不能用少量样例拼装代替。

## 五个已恢复算法叶

| 原函数 | 已恢复合同 | 原执行 / Rust 验证 |
|---|---|---|
| determine_ladder_support @1407de800 /167B | 按上下端锚点、邻接度、下端对齐及 .13 高度门生成全部五态；NaN 保原 setbe 选端 | 1,440 原完整无桥例，Rust 差分通过 |
| gen_stairs_collider_and_rt_geo @1407de8b0，闭包141fe5f70 | 最短 zip、step_by、相邻样本；每端左法线半宽；8 点凸包、24 顶点 RT 与 6 顶点 Vec4 顶面；先追加几何再判凸包 None | 68 原 wrapper 例、101 段；逐字比较输入8点/RT/顶面、step=0 panic、负零、长度不等、append；真实 Parry 单独验证 |
| construct_ladder @1407db3e0 /3435B | 双主梁、状态1/2/4竖撑、坡度叉积厚度、宽度扣 .593 与横档最小 .4、ceil(3D长/.65)至少3、跳过端点、fastrand(-100..100)、24/25 flags、真实木板 writer | 180 原完整例、1,512 条64B、最终 RNG；有限字逐位相同，NaN只比分类；真实Sparse编辑/dirty上传/稳定帧/drop回收通过 |
| split_segment_into_supported_and_unsupported_subsegments @1407d94a0 /1886B | 墙法线择侧、正有限倒数门、原 origin.y-1.17、mask1/maxTOI1.78；原共享交界采样下标、有序支持/无支持区间、独立逐点距离Vec | 192 原完整例枚举 n1..7 内部命中组合，774 条ray逐字；真实World加入/三稳定帧/移出TOI/删除通过 |
| floor_tri_count_to_atlas_idx 原PDB inline @1407dd80c..832 +930..949 | 两次饱和减法与 leading_zeros；index_count/3由caller负责，低计数保持0 | 253 原无桥边界例，覆盖0..129及32位各2次幂两侧，Rust通过 |

调用实现沿现有手工 Rust 架构：梯子直接经泛型发射回调进入现有 Sparse writer，不为生产构造临时实例 Vec；碰撞对三路借用采样迭代，不复制输入；支撑查询生产 adapter 消费 canonical RaycastWorld。未改 frozen interpolate/propagate/structural 三块。

原凸包差分桥仅在已解析的 `new_convex` 边界捕获8点并返回None，因此**没有**声称原 Parry 内核逐位一致。原分段差分明确解析 wall_normal、World查询和标准 Vec 分配；实际 Rust World 另测真实几何。梯子仅解析 CRT ceilf 与真实64B writer 输出边界，RNG与全部几何指令均执行。

测试曾错误断言零宽凸包必定None；真实 Parry 可接受部分退化多面体。已修验收假设，生产几何未改，未添加球体 fallback。原等高输入且要求竖撑时产生NaN，保留该结果，不用零矩阵遮盖；跨ISA不要求NaN载荷/符号一致。

最新收据见 `m01-stairs-assembly-leaf-validation.json`：新增六项 own tests通过；五态独立一项通过；core visual七项通过。新增分段前的48项 stair 联合收据作为历史基线单独标注，不能替代新增部分的最新收据。未做OS输入或真实GPU本轮楼梯验收。

## 原资源合同与未完成缺口

1. `StairsArchSupportRequests` 原是 Vec<0xa0>，`StairsBracketsRequests` 原是 Vec<0xa8>，**不是**现有 HashMap<u64,raw16>。保序且允许重复。生产与两个独立Drop证明共同头：Curve3@0（Vec<Vec3>+Vec<f32>u+length）、Vec<Vec2>@38、Vec<f32>@50、Vec<f32>@68；Bracket另有Option<Vec>@80。Arch尾的identity/segment/color/flags和Bracket尾必须继续由真实producer/consumer解码，不能塞opaque大数组。
2. 原 reset_edge @140aa8a60 有8实参（含sret与两edge寄存器）：self、edge两端、Commands、Brick、Sparkle、Wooden。先清Brick，清内嵌MagicSupports的Brick/Sparkle，再清Wooden；保留分配锁返回两writer以及实际数据缓存引用，随后主装配继续写入。同T的 MagicSupports.clear约束已窄修成独立T/U；现兼容reset入口仍不是原完整生产入口。
3. 原 value Drop @140893be0 与main写面证明：+18是Vec<Vec4>顶面；+30是24B墙切孔记录Vec（WallId+四f32）；+90是44B记录Vec；内嵌MagicSupports首字段是64B/align16元素Vec，clear写+10是清Vec.len，不能只当抽象count。**64B身份尚未确证，不能误当96B BrickTransformSsbo**。这些需要typed恢复再补完整reset；当前不新增假缓存字段。
4. 主装配仍缺 `construct_stair_steps`、`gen_stair_floors`（包括真实MeshAtlas/实体/RT）、四个栏杆算法和Arch/Bracket有序记录。已取得 steps/floors/split/reset 的原完整C；不能依赖现空栏杆去注册装配。
5. 尚无本轮 true→结构distortion→false 完整World/资产/实体/GPU/编辑/卸载联合证据。原18阶段其余叶的状态由各owner单独验收，不由本文代称。

## 可复用逆向经验

- 大函数必须按PDB完整物理范围核独立函数/EH后修body；排除真实独立函数，再用身份绑定PE原相对table补CFG。不能把相邻几万字节强行塞给body、按反编译体字数或stub减少数报进度。
- 批量工具 stdout 64KiB 截断会使含两份压缩C的JSON整体无效。本轮保留该失败产物，先用 JSONDecoder.raw_decode取第一完整对象，再单独读第二函数；检查 completed/error/truncated 和解压后原长度。
- ICF符号别名不能决定具体SSBO类型：reset的第一个host是Brick，内嵌第二host是Sparkle，末host是Wooden。必须沿原调用寄存器/stack实参到真实AssetSsbo类型字符串反查。
- Win64栈参数必须从callee入口偏移计算，caller的20/28/30对应callee28/30/38；不能套现有错误Rust壳签名。叶的独立差分在正确参数之前没有意义。
- 本机沙箱下Uc.mem_map直接SIGILL，连旧绿脚本与最小映射也复现；普通进程权限下同脚本正常。先做旧绿与最小映射交叉诊断，别把工具权限异常误判为原算法失败，更不能放宽浮点容差。普通进程调用经过自动审批，脚本只读RNE并写/tmp/现报告证据。
- 梯梁Y列原为 normalize(delta)*length，直接赋delta会出现1ULP；原叉积不再次归一化。应修求值次序而不是提高阈值。
- Native边界桥、真实Rust World、Sparse dirty上传与真实GPU验收是不同证据；分别记录。零宽、等高、NaN、空输入、短zip及缺资源不能用默认几何掩盖。

## 2026-10-29 碰撞 helper 与跨帧复核

`create_stairs_collider@1407deb60` 只读 C/ASM 保存在 `m01-collider-1407deb60-read-2026-10-29.json`、`m01-collider-contract-2026-10-29.json`。调用者 `14001d44d..d530` 将凸包子项装入 compound，构造 source tag 13 和 edge，`RaycastMesh::new@1408ceb10` 写24B source 与 Arc；空子项走 niche 17。helper 按 Option 移除或 upsert，RNE 常量 `142a88760` 为零平移单位旋转。本轮替换伪签名与空体；真实 World 创建、保持、替换、重复移除 1/1 通过：`CARGO_INCREMENTAL=0 cargo test --offline -p system-decorator collider_owner_creates_keeps_replaces_and_removes -- --nocapture`。此为叶级测试，主装配未接线，未做整函数原侧差分。

结构变形原 tail C 2885–2935 是 resize：只初始化新增元素，既有值保留。生产 `generate_playermade_stair_requests` 每轮清请求并新建空 auxiliary Vec；单独连续调用 distortion 与生产调度不同。删除旧 World 测试中的人工 clear，改为检验同长旧 factor 保留并再变换；不据此修改生产算法。无 clear 的连续叶测试 1/1 通过（`CARGO_INCREMENTAL=0 cargo test --offline -p system-decorator distortion_consumes_real_world_support_add_remove_and_keeps_same_edge_unconnected -- --nocapture`）；真实 generate→distortion 跨帧组合仍待验证。图查询 snapshot `053060f0c5d388b353567d8b` 的双向 CALLS 回执 `m01-collider-code-map-neighbors-2026-10-29.json`，limit 200、未截断、无下一页；静态图不能证明运行时分派，collision/world 源绑定标记 stale。

2026-10-29 续：真实请求生成→插值→结构变形二帧 World 测试 `generated_requests_replace_auxiliary_before_distortion_next_frame` 通过；第二帧由真实 producer 重建 Vec，未人工清空。命令 `CARGO_INCREMENTAL=0 cargo test --offline -p system-decorator generated_requests_replace_auxiliary_before_distortion_next_frame -- --nocapture`。该测试不覆盖主装配。

主装配接口复核：两个单态都含碰撞/梯步/楼板/栏杆/arch/bracket，不可把 false 当独立简化系统。false C 1203 reset、1333 RT、2041 bracket、2050 ladder collider、2589 常规 collider、3012–3168 railing/floor/steps、4319–4324 wall projection、4370 collider、4450–5354 arch/bracket队列；true C 对应 956、1246、1659–1801、2198、3150–3159、3605、3814、4394、4440、4684–5399。现 visual.rs 的 reset_edge_with 是旧兼容接口，未能返回真实三缓冲 writer 和边缓存；必须补全该共享接口及剩余分支字段，才可安全接主装配。墙投影独立审查发现双向射线和严格 entry+0.1<exit 门，详细字段仍待核。没有把这些定位或叶测试视为主函数恢复。

2026-10-29 reset：`reset_edge_native` 使用既有 clear_and_lock_allocation，依次清 Brick、内嵌 MagicSupports 的 Brick/Sparkle、Wooden，保留两分配锁生成写器；清实体、顶面、墙洞、瓦片和原+90记录。`native_reset_preserves_allocations_clears_writers_and_despawns` 实际三缓冲写入→重置→再次写入、allocation身份与 Commands 卸载测试 1/1 通过；`CARGO_INCREMENTAL=0 cargo test --offline -p country-core native_reset_preserves_allocations_clears_writers_and_despawns -- --nocapture`。未做原侧完整reset差分或主装配接线。44B字段暂为明确未知的 owned 字节记录，只恢复reset清长度保容量，不声明语义；原drop只证明allocation释放，结果 `m01-stair-visual-drop-2026-10-29.json`。图查询 `m01-visual-code-map-2026-10-29.json` limit100未截断、无下一页，runtime dispatch未知，world源码绑定stale。

2026-10-29 补核 `+0x90`：B patched EXE 只读 Ghidra `reset_edge@140aa8a60` 和音频 `op_stairs_audio@14081d9b0` 结果保存在 `m01-reset-audio-read-2026-10-29.json`（SHA256 `ebaf818fb6a6f7edd79d929ec776ac7f22149ba4233464c6bce885829396c2f9`）；原 RNE/PDB 身份同上。以 C 中 `pauVar33[-0x11]+8` 为 value 基址，尾部清 `pauVar33[-7]+8` 即 +0xa0 的 Vec len；返回 sret 四个缓存引用分别指向 +0、+0xa8、+0x18、+0x30，**不包含 +0x90**。因此从 reset 返回的缓存引用不能作为主装配写入 +0x90 的依据。既有 drop 只确定 44B 元素分配释放；元素语义及其他直接或间接 writer/consumer 未证实。保留未知完整字节记录和清空，不宣称类型完成。

代码图 `m01-visual-producer-search-2026-10-29.json` 保存三项查询（`StairsEdgeVisualState` 30 条、`op_stairs_audio` 69 条、`stairs_assemble` 527 条，后者 offsets 0/200/400）；均无下一页且最终未截断，snapshot `053060f0c5d388b353567d8b`；查询返回 runtime dispatch 未知、world 源码绑定 stale。命令为 `code_map.py query search --args` 各使用 text、limit 200 和相应 offset；本机 socket 在沙箱内拒绝，受审查权限调用成功。该集合是符号检索而非所有字段引用证明，尚不能断言主装配或其他间接路径从未读写该字段。当前 visual.rs SHA256 `78448f246d0fbfdadfbc93694a3788cc22671552e25c3d70b1c62aaf512d574c`；未做代码修改或新测试。
