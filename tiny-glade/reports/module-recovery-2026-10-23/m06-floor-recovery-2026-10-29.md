# M06 楼板候选恢复

B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；图 snapshot `053060f0c5d388b353567d8b`，PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。consumer 查询回执 `m06-floor-consumer-graph.json` 未截断；静态图不证明调度。

| 函数 | 原证 | 实现与接线 | 验证与未决 |
|---|---|---|---|
| construct_floors 0x1407661e0 | 已存反编译 `crates/country-core/recovery/agent-team/work/restorer-remaining-wallconstructor/evidence/0x1407661e0.c`；原 RNE 140766660–1407668e3 及 14082bd60 指令 | 删除先于变更，变更先撤销再按墙高度/形状创建；shape中心、高度、rank选择AtlasPart。真实 set28 注册由中央处理 | World事件创建、替换、删除通过；极值及原执行差分未验 |
| Floor | writer 栈 +d0 ID 8B，+d8 形状28B；组件对齐后可能40B，末尾padding不作为字段；旧报告+88属命令包装 | 安全字段 WallId、WallShapeSnapshot；渲染临时投影最后字仅为填充 | 原consumer只读取有效字段；PDB类型搜索未返回成员 |
| StoneFloor consumer 0x142069810 系统包装 | PDB Query 包含 Option<&Roof>、Option<&RoofAnimation>、Option<&Floor>，With marker | 维护域包含 marker；Roof优先，Floor-only接入原有build_stone_floor；缺二者保留失败 | MockDevice World 分配、写入、despawn、槽位回收通过；真实GPU未验 |

原 rank 指令 `1407666b7–6f8`：`maxss(radius,1)*3`，log2，max 1、min 5，float转u32后以 byte 传 `DecoratorRank::new`；`1407666fd–707` 结果传 14082bd60，后者写 tag=1 和 part=edx。对 NaN/极值及跨平台 log2 尚无原执行差分。

验证：`CARGO_INCREMENTAL=0 cargo check --offline -p system-wall-constructor` 通过；`cargo test --offline -p system-wall-constructor floor_is_replaced_and_removed_with_wall_events --lib` 1通过；`cargo test --offline -p country-core floor_only_instance_upload_and_retirement --lib` 1通过。日志 `/private/tmp/m06-floor-test.log`、`/private/tmp/m06-floor-gpu-test.log`。第一个测试初次因私有字段编译失败，改用公开更新API后通过。未运行真实GPU或全游戏验收。
7b24fa4a155f97cc9fc3174eeec3da166bed5d4fdf5a37a73a811aa6f201f73e  tiny-glade/crates/systems/wall-constructor/src/construct_floors.rs
ecc57b982ff6ea8b9274db17467c1723188cc423aaa625154b40256ca5653062  tiny-glade/crates/country-core/src/resources/walls/mod.rs
b3b12e8a64be7e2110f079fd4be3471e3af7cf3dfd41241992046c05367ea376  tiny-glade/crates/country-core/src/render/nani/systems.rs

## 追加边界及注册复核

`combined_events_and_ineligible_wall_states` 真实 World 定向测试通过：同帧 delete+change 对仍存在墙先清理旧实体再创建新实体；height 为 0、-1、NaN 时无楼板；非 premade 不创建；墙不存在的 changed 清除旧实体。命令 `CARGO_INCREMENTAL=0 cargo test --offline -p system-wall-constructor combined_events_and_ineligible_wall_states --lib`，日志 `/private/tmp/m06-floor-edges.log`。这些是当前实现 World 测试，并非原侧差分。

原 Update set28 chain_ignore_deferred（既有原注册证据 `tiny-glade/reports/runtime/2026-10-19-main-loop-render-recovery/PLUGIN-WIRING-2026-10-20.md:72`）：update_gates_hashmap → construct_gates → construct_fences → construct_platform_stairs → place_pillars → construct_walls → construct_floors → debug_viz_threshold_curve → structure_outline。当前 `app_systems.rs:103` 已挂 gates两项、fences、额外适配 finalize_wall_holes、construct_walls、debug、outline，缺 stairs、pillars、floors；楼板应插 construct_walls 后 debug 前，由中央处理。静态调用关系不能单独证明调度；以上注册依赖既有构造器原证。

Local 生命周期：原系统参数为 Local<FloorStorage>，构造代码仅在 deleted/changed 消息内查表并用 Commands 删除；暂未发现 Local drop 自动访问 World 的证据。若销毁系统并在保留 World 时重建系统，Local 表丢失后，旧 Floor 无法被新 Local 定位，可能产生残留或重复；这是推论，尚未核原宿主退出/重入调用路径，不能引入无原证的全局清理。
7efc063c3ab786d59510b2fe6140d9bb7094fcba97c3ad31aca2d4817d444f2e  tiny-glade/crates/systems/wall-constructor/src/construct_floors.rs
ecc57b982ff6ea8b9274db17467c1723188cc423aaa625154b40256ca5653062  tiny-glade/crates/country-core/src/resources/walls/mod.rs
b3b12e8a64be7e2110f079fd4be3471e3af7cf3dfd41241992046c05367ea376  tiny-glade/crates/country-core/src/render/nani/systems.rs

退出证据补充：`evidence/round12/independent-startup-reload/REPORT.md` 记录原 140b3f550 OnExit builder 与 140a04980 的 `Query<Entity, Without<KeepOnLevelUnload>>`；当前 `country-core/src/startup/clearing.rs:127` 对该域逐实体 despawn，楼板生成不包含 KeepOnLevelUnload。相同卸载链的 uninit 清 owned GPU instances。`app_systems.rs` 中墙卸载清 PublicWalls/InnerWalls，而 Update 系统自身在普通状态转换中仍保留 Local；旧 map 对已卸载 entity 的条目留存，重入同ID changed 会先 remove，实体存在检查失败后再创建。未验证完整真实 App Reload 跨帧及无同ID事件时 Local 的驻留成本。此结论修正前段“可能残留实体”的推论：正常 Game OnExit 的全局实体清理已覆盖 Floor，系统销毁而无正常 OnExit 仍未验证。

中央已将 construct_floors 插到 `app_systems.rs` set28 construct_walls 后、debug 前；集成 check 正在由中央执行，不将注册等同于原侧和GPU验收。

## 原 rank 差分尝试

`m06-floor-rank-oracle.py` 从身份核对的 RNE 提取映像，计划执行原 1407666b7..6c7 和 1407666cc..6f8，桥接 macOS libSystem log2f，保留参数位型、桥返回位型及原整数结果；桥接不证明 Windows UCRT 等价。实际 `tools/venv/bin/python -X faulthandler ...` 在首次 Unicorn `mem_map` 处 SIGILL / exit 132，未生成 fixture；Rosetta x86_64 Python 无法加载该 Unicorn dylib。原执行未通过，不能把计划或本地 Rust 测试当差分。可按已存在 `probe_m01_expand_x86.c` 的 Rosetta 原指令执行方式，建立有界段和明确 libm 桥接后再比较。

真实 App Reload 仍未完成：`tests/original_session_control.rs` 的现有 app 使用 StatesPlugin、真实 session 支架，但仅注册 begin_glade_unload，没有实例化 GpuWorld / GraphicsDeviceResource 或完整 uninit 所需 RT/audio 资源。`tests/original_nani_gpu.rs` 提供物理设备 bridge 模式；这些需要在原会话支架上整合并实际运行，不能将 World 简易模拟标为 App 验收。

## rank 原指令差分（续）

相同 `m06-floor-rank-oracle.py` 在受审查执行权限下成功；执行原 PE 区间 `1407666b7..6c7` 与 `1407666cc..6f8`，生成 `m06-floor-rank-oracle.json`，31个半径位型含 ±0、无穷、qNaN/sNaN、次正规及rank边界邻值。`log2f` 外部调用桥接 macOS libSystem，不能外推 Windows UCRT 全域等价。生产 `floor_circle_rank` 对这些原指令结果逐项比较通过；命令 `CARGO_INCREMENTAL=0 cargo test --offline -p system-wall-constructor construct_floors::tests --lib`，三项通过，日志 `/private/tmp/m06-floor-rank-tests.log`。上节沙箱 SIGILL 为真实保留的首次失败。

## 同 App 重载测试

`tests/original_session_control.rs::floor_owner_survives_schedule_across_real_game_reload_transition` 在一个真实 Bevy App 状态机上进入 Game→Reload→Game，保留同一 Update schedule 的 Floor Local；通过原 begin_glade_unload、uninit_clearing、despawn_glade_entities，并检查旧实体消失、测试 GPU owned subset 槽归零、同墙变更重建后得到不同实体及新 key。`CARGO_INCREMENTAL=0 cargo test --offline -p tiny-glade --test original_session_control floor_owner_survives_schedule_across_real_game_reload_transition` 通过，日志 `/private/tmp/m06-floor-app-reload-retry.log`。首轮 `/private/tmp/m06-floor-app-reload.log` 因 fixture 未附 save sender 而 panic；补建实际 channel 后通过。范围：TestDevice、disabled audio、保留同一 PublicWalls 行，未执行完整 `walls_on_glade_unload`、真实 GPU 或整个游戏场景。
b43d07bec86ceb0f7d3b6d24858ab8174a7f29d91e1fd5e8ae8e8f9bdd7902ce  tiny-glade/crates/systems/wall-constructor/src/construct_floors.rs
ecc57b982ff6ea8b9274db17467c1723188cc423aaa625154b40256ca5653062  tiny-glade/crates/country-core/src/resources/walls/mod.rs
b3b12e8a64be7e2110f079fd4be3471e3af7cf3dfd41241992046c05367ea376  tiny-glade/crates/country-core/src/render/nani/systems.rs
08178209a56d683c3050875c858b940ed8bcfef983d815705e17e9c7c7aa2ccc  tiny-glade/crates/tiny-glade/tests/original_session_control.rs

## 支撑链取证起点

图 search `construct_elevation_supports` 两页388条：`m06-support-graph-page0.json` 200条、truncated=true/next_offset=200；page1 188条、truncated=false；均 snapshot `053060f0c5d388b353567d8b`。初始函数清单20个非closure候选仍需逐项确认，不等于完成。

复用 `evidence/country-core-deep/systems-recovery/groupC/wall-constructor/decomp/decomp-0x141248300.txt`：原 finder cache 0x141248300 创建两个Vec，16B元素 `(WallId, 引用)`，第一表为 `PublicWalls::roof_entity` 及ECS generation/occupancy验证后的屋顶记录，第二表为形状tag<2的公共墙记录；现行 HashMap<WallId, Vec<[u8;40]>> 和零参 new 是未完成的错误替身。应保留 PublicWalls 原遍历顺序，是否影响可观察结果需对 finder 判定。原 finder `decomp-0x141248620.txt` 45598B、9个原参数，返回动态多项结果；现签名单个Option不符。两者的字段语义、所有分支、失败路径与消费侧尚未闭合，不改空接口替代完整实现。

## 石柱碰撞链进展

原 0x14124fcb0 在 Rectangle3d::new 后将32B对象写入每墙柱Vec；0x1412006f0 消费闭包按32B读取，变换位置与方向后经原 box_trigger_volume 展平三角形；本轮 Ghidra 只读 `m06-column-closure-result.json` 成功。原 0x14078a6e0 使用存储+0x48墙ID→碰撞实例表；先对已删除墙清空列并remove，后排空脏墙，对空列remove、非空列三角化new_indexed并upsert，最后本轮发生工作时发送一次零大小StoneColumnsChanged。当前已写候选typed Rectangle3d、零大小事件、真实碰撞维护和World测试；尚待编译、原侧差分及detect/geometry生产链接入。

退出证据：root核 `run_game.rs:1027–1038` 的 OnExit(Game) 普通tuple包含 MeshIntersectionGraph::on_exit_game_state 与 RaycastWorld::on_level_reload；后者 `world.rs:3188` 重建整个物理世界。decorator unload 同阶段重置石柱storage是独立原owner清理，不应增加逐柱删除以改写失败顺序。完整同App退出与顺序验证尚未运行。

石柱碰撞候选现已落库：`wall_stone_columns.rs` 的 StoneColumn 包含 Rectangle3d，事件零大小，按原 storage +0x48 引入独占墙ID→ColliderMeshInstance map；删除旧Clone避免复制碰撞所有权。wall-constructor 使用 country-core 的 native-physics feature。`maintain_stone_column_colliders.rs` 用原1408cee40索引mesh（双None：原14078af13–2f两处Option判别为MIN）、source tag6与墙ID、identity pose。原1412007da–89d读取 footprint axis/center/size、bottom/top，center_y=(top+bottom)*0.5，高=top-bottom，旋转输入=(-axis.y,axis.x)，经现存原 box 三角化。

`CARGO_INCREMENTAL=0 cargo test --offline -p system-wall-constructor construct_elevation_supports::maintain_stone_column_colliders::tests --lib` 2通过，日志 `/private/tmp/m06-column-world.log`：真实Rapier World创建、静帧、相同dirty保留句柄、几何变更替换、删除及事件次数；候选盒12三角形边界通过。尚未覆盖 native 原侧三角形逐位差分、生产者detect→柱或实际完整App unload，因此不宣称整条支撑链完成。
b80f3cf9250f275635c7bdb9b4fe5b2bb0087ee2296b9057ccb270a35be9bc7a  tiny-glade/crates/country-core/src/resources/walls/wall_stone_columns.rs
3ec4254b79107f50ae16e1e938f169de9ac4ce8bdc483b310c83a3dfe104772c  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs
72dcc40c9011b2d9badc7985858862348787370598f3d3d27f1fb34f8feb4dbe  tiny-glade/crates/systems/wall-constructor/Cargo.toml

Finder cache 候选已替换假HashMap：`OverhangSupportFinderCache::new` 遍历 PublicWalls，独立收集存活 Roof entity 和形状tag<2的墙，Rust安全存墙ID/实体而不保存私有ECS指针；必须在同一不可变业务阶段消费。`cache_collects_live_roofs_and_premade_walls_independently` World测试覆盖活屋顶、失效entity和非预制墙，通过（`/private/tmp/m06-support-cache.log`），仅是候选合同测试。原调用点1407874d7矩形使用flag1、0.6，1407875de圆形flag0、0；finder仍todo且其80B动态记录与全部几何分支未还原，不接生产。
a8c3a31c26bb4dd66df3a2606ee2ccc0ec04ed1a3939432e7fb1f0ce659daef0  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs

独立复核修正：原 `clear_and_get_mut` 0x140a78880 先无条件追加墙ID，再清空该墙列；原实现不去重。旧 `contains` 已移除；同墙连续两次清理保留两个dirty ID，第二次清除第一次柱，维护重复消费但只发一次ZST事件。`construct_elevation_supports::maintain_stone_column_colliders::tests` 两项通过 `/private/tmp/m06-column-repeat.log`。发现测试消费者 `systems/decorator/tests/decorator_unload_lifecycle.rs` 曾用假handle，已迁移成非零真实Rectangle3d fixture，目标卸载测试通过 `/private/tmp/m06-decorator-adapter.log`。本轮无全workspace回归。
df49aef9d7b0c3b8963488ad39f83e6f01f668dde71127ac5af704205eb30c79  tiny-glade/crates/country-core/src/resources/walls/wall_stone_columns.rs
3d8ad5a9a07c59154e7a0a2aea5d282ea9e76cae87b507997cdeee343df9f619  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs
a8c3a31c26bb4dd66df3a2606ee2ccc0ec04ed1a3939432e7fb1f0ce659daef0  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
23a1ff77f65515902b515d29deb5db887541c134064c4448cc0b50659db5b885  tiny-glade/crates/systems/decorator/tests/decorator_unload_lifecycle.rs

缓存后续复核：移除对现代 Entity 高位私有位型的检查，使用 `roof_entity` Option 与 `Query<&Roof>::get` 验证实体有效性和组件存在。World 测试追加存活但无Roof组件反例，连同失效entity、有效Roof、非premade通过；`/private/tmp/m06-support-cache-final.log`。原侧执行差分仍未做。
dd255f8b3c57a81fbc383e44c8447ba7d512ff592360571a8dbd61c1760130c9  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs

支撑finder的线段×形状依赖 0x14175ada0：按B原归属新增 country-core `systems/utils_intersect.rs` 并在实际lib.rs内联systems入口声明。原PE函数及原几何callee在Unicorn中执行，fmodf桥接macOS libSystem，84组有限输入含旋转/平移矩形、退化线、padding负/过大等，夹具 `m06-line-overlap-oracle.json`；Rust逐位对照测试尚待运行。NaN、无穷、跨环、端点ULP、原panic顺序及真实UCRT未完成，候选helper未接生产finder。finder 80B记录结构修正为 Curve2（两Vec及length）、WallId@+38、参数@+40/+44、flag@+48；候选安全字段位于find_overlap_segments.rs，不表述原Rust ABI。

线段helper原PE差分扩大到264条：输入线段用u32位型，含qNaN/sNaN/Inf、有限端点相邻ULP、旋转/平移矩形、倒序参数和负/大padding；原panic入口`0x1428d9430`与fmod桥实参记录在`m06-line-overlap-oracle.json`。预先规则：有限结果逐bit、无命中与panic分别分类；不存在以宽容误差替换位型比较。首轮 `/private/tmp/m06-line-264-failure.log` 暴露共享 `Circle2d::intersect_ray` unordered判别错误：原0x140c97980只对有序d<0返回None。候选修复有序<0拒绝、NaN继续；264上层复验 `/private/tmp/m06-line-264-retry.log` 通过。macOS fmodf桥与非穷举输入不证明Windows UCRT全域等价。

圆射线无桥原执行 `m06-circle-ray-oracle.json`：q/sNaN样本两根0x7fc12345，Inf新生invalid两根0xffc00000；首轮Rust `/private/tmp/m06-circle-ray-test.log` 首根NaN符号与原不符。已尝试遵循原SUBSS源操作数选择，但尚未验证多个位置与多负载，49组无桥原执行 `m06-circle-ray-nan-oracle.json` 和严格测试待运行。未证明跨架构全域语义；不得将首个样本通过当最终修复。

49组多位置/双NaN逐位对照 `/private/tmp/m06-circle-multipayload.log` 失败：负qNaN圆心原首根0x7fc54321、次根0xffc54321，候选首根0xffc54321；先前按root直接替换的单样本临时候选已撤除。保留有序判别修复，但NaN负载传播仍未闭合，严格测试暂为失败。由于Unicorn双NaN传播已知可能与真实x86不一致，必须通过Rosetta直接原执行交叉验证后才确定通用规则，不把某模拟器输出当最终权威。

对49个多位置NaN原输入进行独立Rosetta x86_64直接原函数执行，RNE按PE sections映射，入口字节校验，无几何或libm桥（`m06-circle-rosetta.c`）。结果写入 `m06-circle-ray-nan-oracle.json` 的 `rosetta_words`，与Unicorn在17/49例不一致；包括单q/sNaN原首根负NaN、次根正NaN，否定Unicorn先前的双正NaN结果。候选首根强制替换已撤销；最终对照使用Rosetta结果，尚待Rust重新执行49例。原首次单样本测试使用Unicorn错误预期，失败日志保留，不代表Rust与真实x86不同。

49组 q/sNaN 与双负载，Rust对照 `rosetta_words` 全部逐bit通过 `/private/tmp/m06-circle-rosetta-comparison.log`；撤销单样本首根修饰，仅保留有序判别式修复。Rosetta 是 x86 翻译执行，不是物理 Intel 或 Windows；与Unicorn冲突的17条保留两种结果，不能将冲突位型扩称原设备全域真值。264上层有限值逐bit与panic/None分类单独记录；非穷举输入、Windows UCRT及真实设备仍未验证。
60a49a294f9cad90096e672e4ef4ed7a944701711de61b78ea06710194e7f7b9  tiny-glade/crates/utils/src/geometry/circle.rs
0031b83384e98ea98a316e77315bf9d2c6ec19e860a4d03d6c5a732ea88a6b55  tiny-glade/crates/country-core/src/systems/utils_intersect.rs
0f7bbc2685081f584143996148a32cf84318c0c154e52c5042478f235c868218  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs

`m06-line-overlap-oracle.json`扩展462条：padding按u32覆盖q/sNaN和Inf，原返回420条、断言panic 42条。首轮严格测试 `/private/tmp/m06-line-462-failure.log` 发现qNaN padding：原fmod桥返回NaN后直接写Some(NaN,NaN)，候选CurveUCircular::new误panic；非有限值按类别、有限值逐bit，panic独立分类。该路径尚待共享CurveU的原未校验结果表示，不宣称候选完成。

462原执行样本首轮确认：对NaN padding，原fmod桥NaN结果在原SUBSS/UCOMISS控制流中直接写入Some(NaN,NaN)；候选调用`CurveUCircular::new`提前panic（`/private/tmp/m06-line-462-failure.log`）。当前共享`CurveU::new`保证区间有效且无法表达原未校验返回；已向中央提出仅供原版未校验产出的明确构造API，待批准后恢复并重测。当前候选 `utils_intersect.rs` 在非有限padding的测试失败，尚无生产调用，不能报完成。有限结果逐bit，非有限结果按NaN类别与原panic独立分类；原桥为macOS fmodf。
fe4355e8922193a3105504acf527822eb7320fd36bd62f1726e733eef86da8a9  tiny-glade/crates/country-core/src/systems/utils_intersect.rs
60a49a294f9cad90096e672e4ef4ed7a944701711de61b78ea06710194e7f7b9  tiny-glade/crates/utils/src/geometry/circle.rs
ad92c92fe7f5397a711cd14f6e7096daefd15911a47b46b20661bcbb6385d559  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs

**重要修正（待重验）**：`PublicWallState::building_projection` 原publisher证据+0x280为 `authority.elevation()`，不是`authority.height()`；先前楼板候选将这两个字段混淆，先前World/App测试未区分，因此不证明原字段选择。现构造门控和Transform.y、finder候选高度门控均改用elevation，新增不同于高度的fixture及高度0/高程正反例；这些当前更改尚未重新运行测试。报告先前将其称height的说法均被此条取代。

finder交点子分支候选：以原ShapeIntersections两墙键筛选当前墙，按原`PublicWalls::get`必有语义、source高程与max_y门控，转换source预制形状曲线，投影交点、按需附矩形五角参数、排序环形配对、中点目标形状包含、最短长度与0.3距离收缩、circular_slice输出。此子分支及本次elevation字段更正尚待编译与原行为差分；屋顶截面、包含形状全分支及完整finder仍未实现/未接线，不能把局部helper当整链完成。

高程修正重验：floor World 3/3 `/private/tmp/m06-floor-elevation-test.log`、同App Reload 1/1 `/private/tmp/m06-floor-elevation-app.log`；finder交点局部5/5 `/private/tmp/m06-finder-intersection-test.log`。墙高7、高程4且零墙高、正高程的反例覆盖。原publisher佐证 `reports/business/2026-10-23-module-recovery/toolbar-focus-height-review.md`、`toolbar-shape-placement-review.md`：source+58 经140a8d825→d904写栈+780，dd16以栈+500为起点复制0x2a0，得到PublicWallState+280；source+5c经d82b→d914写+288。原线段helper原528条 `/private/tmp/m06-line-528-final.log` 分类对照通过，utils曲线37项 `/private/tmp/m06-curve-regression.log` 通过；参数0和0.6为原调用输入，其余padding是IEEE探针，不声称游戏可达。fmod使用macOS桥，非物理Windows或全域证明。finder当前只有交点子分支候选，未执行原侧整函数差分。
99700e9fe3977b64c9156d880ed4d96250f219780617f1b51847327ce6501531  tiny-glade/crates/systems/wall-constructor/src/construct_floors.rs
22704036d07021c66af22178c055d2371a309f08905c6129a443b7e1717c546c  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
b51a51148b58632a30a68cd82459359db6d8b120b0dfed0234cbf13bdd710677  tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs
e5b5d7a22070ac43debb98c38e65d8b75dca3a52b4c4fce0d6639825a20ac778  tiny-glade/crates/country-core/src/systems/utils_intersect.rs
d408141c2212eefe21a8c0bdc10726ab0e4231c09f405dfa541a69f8f3d81a25  tiny-glade/crates/utils/src/curve/curve_u.rs

finder候选分支续：原1412497e3/7ec为UCOMISS+JB/JBE，无序时均跳过；先前只看C反向比较得出“包含分支NaN通过”是错误推论，现已纠正。交点、roof gable、完整包含和缓存7项本地测试通过 `/private/tmp/m06-finder-contained-final.log`，含圆环及四矩形边，尚无原侧完整函数差分。屋顶闭合索引4的原get_point 140c99604上界5、140c99644–4a index4转0；表142aa5d60为[-0.5,-0.5,+0.5,+0.5]，修共享方法并通过旋转矩形index4闭合、index5 panic `/private/tmp/m06-rectangle-point4.log`。gable样本首次错误预期两段，`/private/tmp/m06-gable-branch-test.log` 失败；原线段末端回绕及逆序门控说明该输入只产1段，改正预期后 `/private/tmp/m06-gable-branch-retry.log` 通过。这是候选回归而非整函数原侧验证。

缓存失效安全合同复核：`PublicWalls::get_unchecked` 的“unchecked”只表示不执行 `PublicWallState::assert_valid`，实现是安全的 `HashMap::get` 并返回 `Option`，不是 Rust `unsafe` 或无检查内存读取。原 0x141248300 的缓存保留同一轮不可变公共墙借用的引用；当前候选以 ID 表示，调用者必须在同次借用建立并消费。人为构造失效 ID 的 `stale_wall_cache_fails_safely` 以明确 panic 终止，定向 1/1 通过 `/private/tmp/m06-finder-stale-wall-test.log`；这是当前实现的保护反例，不主张原程序存在可达的过期缓存 panic。当前 finder 仍未执行原侧整函数差分，detector/生产者/原顺序接线与卸载尚未完成。源 SHA-256：`find_overlap_segments.rs` 83ba92e2cf3a1591bc63f38bb774f02afe7e615cfe3cd5332d55b39edc0d3a96；`public_walls.rs` 81c65533047fd65118694714cb3f738b46837d0039edbcb67b7e6bb699d2f89f。

Finder 原入口有界复核：`m06-finder-height-oracle.py` 校验 B 映像 SHA 后通过 Unicorn 自原入口 0x141248620 执行；关闭 puffin，无算法桥，原 `PublicWallState::max_y` 直接执行；到 0x1412497ee 截获进入包含分支或正常返回判为跳过。33 例覆盖两门限邻 ULP、Inf、qNaN，原进入11/跳过22；Rust `containment_height_matches_native_finder_branch` 全部一致，`/private/tmp/m06-finder-native-height.log` 1/1。另以同一原入口继续到完整包含几何接受处 0x141249d4a/0x1412498df，九例覆盖圆圆、矩圆、圆矩、矩矩及部分边界，原结果已在 JSON，Rust 对照新增尚待运行。这些只是分支探针：无段输出、排序、分配、屋顶或交点的完整原侧差分，不证明 finder 完成。Finder 源 SHA 3b9b7a51c606268cf6c53a32c86be33ec82260b23a2b550097a8f33d023cc9c6；探针 SHA 9ffce332b3f5dc1f60f9fd4e3782553ec1e8815c08341cadcc01375b593d93ff；夹具 SHA 41eedce9b5746a8f781dae0e43e3e4a4429bcc76dc5b10946f2a05ac2a6645b8。

九例圆/矩包含分支 Rust 对照 `shape_containment_matches_native_finder_branch` 已通过 `/private/tmp/m06-finder-native-shape.log`（1/1）。试行继续至完整圆环输出时，原 `Circle2d::as_curve2` 首个 CRT 导入处（0x142923e60）因原 PE IAT 未装载而停在未映射地址；这是原输出探针尚未完成，不能当原函数差分通过。需确认导入、桥接语义和原分配器后继续，探针初始实验保留 `/private/tmp/m06-finder-output-probe.py`。

原 finder 输出探针已推进：绕过 puffin，原入口正常返回圆包含1段（27点）和矩形包含4段（各2点，原顺序）；使用有界分配/重分配、延迟释放桥及 macOS `ceilf/cosf/sinf/fmodf` 桥，不能推断 Windows CRT 全域位型。对应 `m06-finder-circle-output-oracle.{py,json}` 与 `m06-finder-rectangle-output-oracle.{py,json}`；原始字段及全部点/参数位型已保存，现有真实墙测试新增逐位对照，尚待运行。仅这两样本，交点/屋顶/trim、边界、全部输出和整链仍未验收。当前 finder 源 SHA `706ed30199328ca6a46ef7f38dbf0d03ff83c4a356d960539546fe24f5fb70a4`；circle 脚本/数据 SHA `ed72642f1710711b8a5c9b5d9b6784aeb4ffe84180f225349c16570a7fbb8cdf`/`7a6c86865b49bfa82c9de23dd07d78bc72ea3013a9b477e681267a2b04a99351`；rectangle 脚本/数据 SHA `d99f685c45d1eca691df3c0a49d211d09ac8a67dfdb686ae316fdf70e004728b`/`9dd22ff48b6406f745b899a3ebaf2c0b3e5bc3c610f6cadf7a00ea033063f7b5`。

输出探针的结果状态已由易误解的 `skipped` 改为 `returned`（指正常返回，并非遗漏段），复跑输出数不变。更新 SHA：circle 脚本 `3e72b22e9359d343d8839d82a5d0af2f101d743ff01488d50078b4e27da4f89b`、数据 `f7cd816e10b509dbdbea5bc879290d718b34ea5ea4e7c4896a851ad042b3ac93`；rectangle 脚本 `204cacf8ae9bd54092e3917355318773349eafadc85cebcafb3927a48643b599`、数据 `b74dd0a4a3185b564c6cd12a60aa33d36c154ff98657072a1b62d27dfa93dd1a`。

两个原包含输出的逐位测试 `/private/tmp/m06-finder-native-output.log` 通过（1/1，圆27点和矩形4段完整数值与输出顺序）。随后原入口另执行 `trim=true` 的同一矩形场景并正常返回4段，脚本/数据为 `m06-finder-rectangle-trim-output-oracle.{py,json}`，SHA 分别 `af2ed93fc609ccd677e6e4ee5a12b4394c8db94b0c67fe6d390dfcb2552540dc`、`86e0c2d462fd509d7c66f00c6cffa0b66c2a4891c71a4c3e6bb859b9ab0d3b2d`。现有 World 测试已加入该样本全部输出逐位对照，尚未重跑；当前 finder 源 SHA `5b4e1b21ad664e7cde11bfdff99bbe82cfd3f964673dae9dac2cf16713b7b161`。原执行桥和覆盖限制同上。

山墙原输出：以 roof 原88字节样本、目标墙高程4、目标形状10×10执行原 finder 到正常返回，输出1段2点；`m06-finder-gable-output-oracle.{py,json}` 已保存，Rust World 同样本逐位断言待测试。最初夹具遗漏目标墙的 +0x258 形状字段导致0段，不作为算法证据；补齐后才为同输入。四个输出脚本均增加“正常返回”断言，杜绝将部分输出当成功。当前源 SHA `a672d951373c37955b00f7acdcecf21fd88321840ae0b737b6706340548fe01c`；circle/rectangle/trim/gable 脚本 SHA 分别 `b60384a36eecaad7007f336de91ed16240af448e5fb209bb011ad59c3549e77a`、`43dbcd11324a445aadb10fd2651654b82cb5009849449dcbdca67298f5a815fe`、`91955dea6c3f355f94a360ea35d1d98be2e51782cd8e46a70bc791334a7eddeb`、`2b6d64c8a800ea589cbaae38f6b299c20798c81daa53299ca15e8e39906d9145`；gable 数据 SHA `37c15e4d687c856344bd0d80e77b33b212c1a517265f6a967b68d91ab0dc311b`。

交点原输出：原哈希表构造一对墙与一个交点，目标矩形 5×5、高程4，源圆半径2、高程1；原入口正常返回1段27点。`PublicWalls::get` 以已知有效墙地址桥接，使用有界 allocator/reallocator、memcpy 和 macOS libm 桥；不声称验证墙查找或 UCRT。原交点输出数据 `m06-finder-intersection-output-oracle.json` 与脚本 SHA 分别 `0007acfe8a9c80e9323b3cf34c5e0867a7e4bb01d03ec245f65c7590b963d2c6`、`aa080e7b19c5720c2e788962160c4e5270d31f093964b9bad381d8680d5ff400`。已向现有 World 测试加入全字段逐位断言，待共享编译批次；当前源码 SHA `5b0527f5d7c0e4b859efeeb6974bb9997b511c1da1150ed6af20609392413424`。交点多段、更多屋顶和真实完整检测器仍未验收。

共享编译批次 `find_overlap_segments::tests` 10/10 通过，日志 `/private/tmp/m06-finder-shared-epoch.log`：包含原单交点、山墙、矩形 trim 全段输出逐位对照，和先前圆/矩形包含与缓存反例。源指纹仍为 `5b0527f5d7c0e4b859efeeb6974bb9997b511c1da1150ed6af20609392413424`；上述是少量有界样本，不证明整个 finder 或 detector/producer 全链。

Detector 0x1407867d0 候选已换为原15参数面，另有当前实现显式可变 SSBO 与 ConstructorGpuOutput owner 适配；会话事件全排空、墙变化与砖变化各自收集、合并脏订阅、圆/矩形门控、对应 finder 和事件已写入。零高程统计重置保留明确 todo，未生产注册，未编译/未运行 World 验证；当前源 SHA `d84f73f5b781f6257c97fa81756964987de4e53b25c1957c1e9995b40239f813`。原 1407884b5 返回 `StoneColumnCollisionStorage::clear_and_get_mut` 的 Vec 指针，1407884ba 保存栈+100，1407888ca 读取该 Vec 并按32字节复制到 `ElevationSupports` 首个 Vec，元素对应已证 `StoneColumn`（Rectangle3d）；其余字段语义仍核。图查询 `m06-elevation-types-query.json` 非截断、无下一页；`m06-elevation-stats-query-page0.json` 与 `m06-elevation-audio-query-page0.json` 截断且 next_offset=100，不将首屏当全部消费者。

统计图查询后续 `m06-elevation-stats-query-page1.json` 为63项，`next_offset=null` 且未截断（两页合计163项）；音频查询仍仅保存第一页，尚未完成分页。`0x1408c5670` 图查询命名 `WallStyle::is_halftimbered`，结合原 1407888c6 写入取反字节可确认圆 producer 的该标志来源。Detector 候选需修正一次调用时序：当前即使墙事件命中也提前计算 AABB/脏相交；原仅在未命中变化事件时计算，下一源码批次按原短路修复并验证。

更正上段标志推论：原 1407884ff 的 `circle_categorize` 返回值写 rsp+67，14078866b 对其 XOR 1，140788753/78878c 取回并于 1407888c6 写入 value +48，故该字节是圆形分类取反（turret 分支为1）。14078883e 的 `WallStyle::is_halftimbered` 是构造音频事件 kind，不是该字节来源；上段旧判断作废。此校正不改已证首个 StoneColumn Vec。

Stats 另外三项原 writer/reader：矩形 bracket 宿主 `140789c3e/c77` 向 small helper 传栈+98 输出地址，`140789c8d/ccf` 向 large helper 传栈+90；原 `140789dcb/dd5` 比较并写 map value +24，`140789de0` 比较 value +32，变化触发声音。圆宿主以栈+b8 比较并写 +40（`140788812/8c2`）。三者为 qword 值而不是 Vec；尚需确认各 helper 的数值定义与失败行为。

小托架计数得到独立 writer 佐证：原 `elev-0x141250da0.c:751` 在生成条目后递增输出指针，宿主将其结果与旧 +24 值比较。large/circle 数值的更具体单位仍未断言。

Detector/stats 下一批：原零高程分支 1407867d0 的 stores 将首 Vec 初始化 cap=0/指针=4/len=0，再写三个 qword 0 和布尔字节0；Rust 用安全 `Vec<StoneColumn>`、small/large bracket 数量、`circle_output`（单位未知）和 `is_turret` 表达业务字段，不复制布局。原操作顺序为 clear_and_get_mut→插入上述空 stats→清 plank 再 brick SSBO→发空木塔事件。事件命中时跳过 AABB，未命中才按原圆/矩门控检测脏区。两项 World 测试 `/private/tmp/m06-detector-world.log` 2/2 通过，覆盖变墙、砖变化形状差异、订阅脏区与跨帧、零高程清理和空木塔消息；之后仅将未证语义的 `circle_pillars` 字段改名为 `circle_output`，该重命名后的源尚待重跑。当前 `mod.rs` SHA `29b15edd92f0748752d2cf5d4d8f6ef84cdd90a3ca549b785aa25d972af4a149`；`elevation_support_stats.rs` SHA `93d79fa357a853e4d391e0f98093a06f95d9a692a863e855b0dfb6a03b598656`。已实现的是 detector 有界行为；生产注册、circle/rectangle producer、完整 finder 差分与实际 App 支撑卸载仍未完成。

圆形分类 `circle_categorize` 迁移 typed 输入：原只读取段记录 +0x38 WallId，现直接从 `OverhangSegment::wall` 读取，删去旧私人80字节 `WallEntry50` 镜像；候选未在消费者接线，原有测试尚待新批次重跑。文件 SHA `2a21fce306f1c3ced14e60f95d1717f5cf4ba9b77c2db8ea567b3aeb77778966`。大托架原构造器 `decomp-0x141253fc0.txt:1901` 对输出指针逐项加1，确证 stats +32 为对应数量；小托架同理，circle_output 仍保留未定单位。

当前源码重验：中央 `/private/tmp/root-m6-detector-current-epoch-20261029.log` 为 circle_output 改名后 detector World 2/2；`/private/tmp/m06-circle-categorize-typed.log` typed 分类 3/3。原资源生命周期：PDB enrichment 指向 `run_game_thread` 0x14059d450 内联 `ElevationSupportStats::default`；图 `m06-stats-insert-callers.json`（snapshot 053060f0c5d388b353567d8b，未截断）证实 `App::insert_resource<ElevationSupportStats>` 的调用点 0x1405a1e45。原 1405a1de0..df5 准备资源、1e3b 传地址后 1e45 插入。静态资源插入只证初始化，未证明 Game 退出时是否清理；原 `walls_on_glade_unload` 未发现该资源不是完整反证，继续查其他清理 owner。未挂生产。

新增待验 World 断言：零高程检测前先向实际砖/木 SSBO owner 各写一项，之后断言两缓冲均清空；圆分类 radius=2 对缺失段墙应 panic。前述通过日志不覆盖新增断言；待下个 Cargo 批次运行。当前 mod.rs SHA `851f4480e1336f11e6bc85a12b35ece6ef4fcb0ee542be3852c5e0773e07848a`，circle_categorize.rs SHA `cef9b766c0eb7b0f7c967eb227c2ca3b56e36f0e7b9818b57cc5160de11fd02a`。

`construct_elevation_supports` 过滤组 `/private/tmp/m06-support-group-epoch.log` 21/21 通过，包括新零高程实际 SSBO owner 清空与缺失段墙 panic；中央 `/private/tmp/root-m6-stats-registration-integration-20261029.log` 集成 check 通过并已初始化 stats。此组不执行未实现 producer，不能视为完整注册、实际画面或卸载验收；继续恢复圆/矩 producer 和依赖。

`add_bottom_brick` 0x141fe1de0 已按原指令恢复 typed 单砖候选：一次随机采样、随机高度限制、端点长度/方向及垂直向构造仿射矩阵、原 ColorId→BrickSourceId、seed/距离与默认材质字段，逐项写入并返回失败供原循环置脏。原指令完整执行，桥接单次 rng=0.25 与成功 push，捕获96B `m06-bottom-brick-oracle.json`，逐位测试待运行；只覆盖该一例，未证明 RNG 输入全域或圆底层循环。源 SHA `afab9900f7c787749e160db98f7e9dbf70c7f7862ff8e876dd8371cf954cdb4f`，脚本/数据 SHA `665ae1e207ebac74154372563c5aba4658234ae09f9f23ceef743655d4e2902e`/`e1095c28258ac74251d8cddb7727761bfbfb126c183538af969c7d815e51490c`。

底砖原96B样本对照 `/private/tmp/m06-bottom-brick-native.log` 1/1 通过。原圆底层依赖 `PublicWallState+284`：publisher 140a8ce65 从 source+65 取 flag，140a8d7ac/c0/c6 依该 flag 选 2.5（非零）或14.0（零），140a8d90c 写栈+784，后续整体复制为 public+284；圆底层 14124b6a2 读取。此前误称投影未写此字段；后续原指令完整复核确认投影使用 constrained_extra，见下方更正。原底层还读取墙 ID 与圆周长*1000 之和作为 fastrand 原状态，随机切分 count=ceil(perimeter/0.69)+1、jitter=0.32/perimeter，当前正在核完整循环/写入合同，不以单砖对照宣称底层完成。

原圆底层完整循环探针从 0x14124b2c0 到正常返回，实际运行原 RNG/split/单砖计算，桥接 SSBO 查找/锁/逐砖写、原分配器及 macOS libm；原输出21块96B，脚本/数据 `m06-circle-bottom-oracle.py`/`m06-circle-bottom-native-output.json`。首次差分 `/private/tmp/m06-circle-bottom-native-sequence.log` 失败，查明原夹具错误地把 RCX 设为 WallId 指针而非值，导致原 seed/material 变成0x30001000；现修正 RCX=77 并重生成，候选待复验。不能将首次错误当原算法差异或宣称已通过。当前圆底层源 SHA `d3e1a79b675454e4c750dad1b707cc1672d2669000f11cb12c10073e76a8994a`、脚本 `fd691fca75b6f0430fdd2e4e37cfc85f107222e064a7da9834a0fbbb7c9fe604`、夹具 `d1766577043e2589a4f7190b12e117744bd8bda2eb8825baa76abb35a09b2b54`。

修正原夹具 WallId 值后 `/private/tmp/m06-circle-bottom-native-sequence-retry.log` 重跑通过：原/Rust 21砖全部96字节逐位一致（1/1），包含原 RNG、random_splits、曲线与单砖序列；仍限定该场景和 macOS libm 桥，SSBO库获取/锁/写及分配为本地桥。候选未连接实际 consumer；生产 owner、其他形状/失败/卸载未验；+284 投影保持既有 constrained_extra。

更正前述“当前 +284 默认零”：中央复核 `WallPublicationCore::write_projection` 当前将 `metrics.constrained_extra` 写到该偏移，不能简单称一律为0。原 publisher 的 2.5/14 选择仍由指令证据支持；中央正在对照该写入与现有消费者后协调修正。

最终纠正 +284：连续指令 `m06-publisher-284-native.asm` 显示选下界 .3/.5、上界2.5/14 后，140a8d8ae 以 source+54 对下界 `MAXSS`、d8b3 对上界 `MINSS`，才于 d90c 写回；现 `publication_policy::publication_metrics` 的 constrained_extra 正对应该夹取（包括无序时保留 source）。因此前述将该字段说成直接2.5/14的推断错误，现有投影不应按该误判覆盖。

本轮圆底层已接 `WallConstructorOutput.gpu.brick_writer`：原 +284 通过现有 `building_projection().float_0x284()` 取得，同一 writer 锁内随机切分及全部逐砖 push，失败次数在解锁后记录，保留成功前缀。detector 与 World fixture 改用正式 `WallConstructorOutput` 唯一资源。新增隔离 World 测试 `detector_circle_event_builds_bottom_bricks_in_formal_owner`，以真实 detector 事件调用实际底层并检查砖 owner；尚待本轮编译，不是完整 circle producer。当前 `mod.rs` SHA `f06ea5b70bb3b09bf97a30245f5644d8e8e2fc0de2b6697fcd220005d921dafc`，`circle_bottom_layer.rs` SHA `23af2a547b25cfad35fa95cb1e61944004d4e4db458c75612394be340cf1f8ad`，publisher指令回执 SHA `f069cb4c620a5277f16e5c9247cce7af14d64eb5bde33cce47a3c5b57ead1412`。

本轮 support 过滤组 `/private/tmp/m06-support-owner-and-exclude.log` 25/25：同 World 原检测事件→测试消费者调用真实圆底层→正式 `WallConstructorOutput.gpu` owner 生成21砖，静帧不重复；该测试消费者不是完整圆 producer。原 `find_stone_column_start_y` 141250785/78a 传 Some(WallId) 到 1408ccc50；后者 ccc58/5d/7b 将选项和墙ID转发到 1408ce180，ccc20 则 r9=0，旧“忽略excluded”描述错误。已将本目录柱起点改用现存 typed 排除墙射线，真实 Rapier 两墙重叠射线反例分别排除一墙并命中另一墙，结果0.5/4.5，通过。指令 `m06-stone-exclude-{caller,wrapper}.asm` SHA 分别 `e8a29837ebc05b0956063b1c2cbfc9a7ec29c3ad6a9b1d2cd444654abc6d9d59`/`be3a53dec918e1751f56b44057d5fdbc056ee57862cc127499f218ef5aec63fc`；源码 `rectangle_pillars.rs` SHA `c79f93274a9be835ca2a36f53d3c8848efeb2da090dd49f61f84c5ad14f49d06`。圆/矩生产者及真实App卸载仍未完成。

圆柱数量 helper 候选按原周长 20..50 对 4..7 插值、round 和饱和转换，当前仅小段，未接柱 producer；`circle_pillars.rs` SHA `c8f3626c30ed02a428454e0ed96deac946ffb9aadf88712832c8618bb4f4e73a`。新一轮 `cargo check` `/private/tmp/m06-circle-pillar-count-check.log` 被共享 `construct_wall_bricks.rs:329` 两处 Query 生命周期错误阻断，已通知中央/对应 owner；不可把旧编译当新源码通过。柱禁用位原 140a8d841 读取 source+67，经 140a8db90 写 public+298；现安全可读 `authority.removed_supports()`，不使用未投影的零值。

十字砖柱 `0x142418f50` typed 候选已恢复随机分层、双端平滑与四边位移/旋转/逐砖载荷，writer 失败逐项计数；原入口实际执行输出36块96字节，原 RNG、几何和分层直接执行，仅桥接分配/释放、SSBO写和 macOS ceil，已写逐位测试待集中 Cargo。当前尚未接柱 producer，不证明全部边界。候选源码 SHA `f38fd6a8b064bb44a235407acba13fd88de5add8e555ed9e3364acaf5100724c`；脚本 SHA `27ba8fd5ff1962eec23e7fcb30dc3d0126d39bd2a5bd50609419a75587ad038a`；数据 SHA `5790bd8cba9c0046ed21f63e881641fa5e7ac2658b6f527430a731cd5778db45`。

共享编译暴露 circle_pillar_count 引用路径错误 `utils::inverse_lerp`（失败 `/private/tmp/m6-plaster-side-current-20261029.log`），已改为已有实现 `utils::scalar::inverse_lerp`，未改变运算顺序；当前源 SHA `bd74a42584b9ee707744b458109aaad12bcee7b1190fdcce39dddbdb6bcb423d`，修复后尚待下一轮编译。

### 矩形柱候选阶段（本轮）

- 原证：B `0x14124fcb0` 的空交集、轴对齐 4×4 矩形场景执行结果，保存在 `m06-rectangle-pillar-native-output.json`；脚本只桥接分配、地形采样、柱起点及消费者捕获，不代表完整原进程。
- 算法：`collect_rectangle_pillars` 实现四角候选、地形和相邻墙过滤、三方向碰撞排除自身墙、起点和宽度；其中仅上述空交集场景完成原输出比对，非方形、旋转、相交与异常分支仍须验证。
- 接线：候选函数尚未接入正式事件消费者；矩形木柱、碰撞输出、托架和生产卸载链仍未完成。
- 验证：`rectangle_pillar_proposals_match_native_empty_intersections` 通过，4 条候选各 15 个有效字段逐位比较，日志 `/private/tmp/m06-rectangle-proposals-test.log`。源码 SHA-256 `ec1d4f7b9aee7c8d1b3309873152b437cbb7d17e4f1d432b7b8163f0ed92ab4e`；原输出 SHA-256 `3f1dd7d58cf2352c30727d0aaed0d79858c5df207b40142f254de861c6eefc71`。
- 随后仅修正候选阶段的过期说明，源码 SHA-256 更新为 `e7dd750b5892dd4ba09c00d47a9d9d20d0480b7e0e6e82a57ff7648950bf7788`，测试时业务代码未变化。
- 新增旋转平移非方形原样本（轴 0.6/0.8，中心 3/-2，尺寸 4/6）：首次对照发现偶数角相邻尺寸次序颠倒，失败 `/private/tmp/m06-rectangle-rotated-test.log`；按四角原输出修正后，两样本 2/2 逐位通过 `/private/tmp/m06-rectangle-rotated-retry.log`。本次源码 SHA-256 `7dfe828c4f84f597a2d9c2562864ccb2263399883e0c0e44d26ccea661f43064`；旋转 fixture SHA-256 `e229039eb0d1180ba640906087ff63eb7be54f69ebe29bca230346a16778c17b`。桥接边界与上一样本相同，非空交集和完整生产链仍未验证。
- 继续捕获旋转非方形原候选对应的 4 条 `Rectangle3d` 原输出（fixture SHA-256 `6c8fb3cb76575370fa01ad39fe44ca83df4affca7ba97d95ec203c42d81295d9`），新增 typed 候选到碰撞边界转换；逐位测试待当前 Cargo 轮转后运行。原消费者在柱生成之前还调用 0x14124eb80 矩形底层，当前仍未实现；不得把柱候选登记为完整消费者。
- 矩形普通石墙底层 0x14124eb80 建立有界原探针：旋转、平移、非方形样本捕获 34 条 96B 砖载荷，日志 `/private/tmp/m06-rectangle-bottom-probe.log`；桥接 SSBO 查询、锁/分配、push 和 macOS libm。已写隔离候选及逐位断言，尚待 Cargo 验证；半木分支未恢复，不接生产。
- 普通石墙底层样本对照最终 34×96B 逐位通过 `/private/tmp/m06-rectangle-bottom-rust-retry3.log`。先前失败保留 `/private/tmp/m06-rectangle-bottom-rust.log`、`...-retry.log`、`...-retry2.log`：分别揭露砖抖动使用边随机流、砖 seed 为墙 id 加边内序号、原 `0x140c904d0` 插值是 `a+(b-a)*t`。源码 SHA-256 `b612a5f52b74286550e041fab31a6d6bf3a45db83d62df229a57836cadeea73b`，fixture SHA-256 `14f7debb61def378f7d3509498d43b41e4a087063beb191d27f981c000824b38`。此限于一个有界样本，尚未接线或恢复半木底层。
- 继续核 0x1400885d0 为 `mov rax,[rcx]; ret`，确认底砖 seed 来自 WallId 低位而非下一随机数。已新增正式 `WallConstructorOutput.gpu` 单锁写入 wrapper；该 wrapper 尚未进行真实 World 测试，也尚未由消费者调用。
- 同探针半木材质分支获得 4 条 64B 木板，日志 `/private/tmp/m06-rectangle-wood-bottom-probe.log`，fixture SHA-256 `19cc1e274ea19546cf23508e08ad445fbabe3d6ca96d3cd7084870fd9dd71de5`；仍待实现矩阵及 World 对照。矩形碰撞 bounds 测试尝试因共享 build 锁占用而取消，日志 `/private/tmp/m06-rectangle-bounds-test.log` 不算测试结果。
- 增加轴对齐方形普通石墙原底砖序列 28 条，fixture SHA-256 `9425ef202b5c0f62ff9164be3b8cf0bd11f53b96ceab5051e6812a1112c65514`；`/private/tmp/m06-rectangle-square-bottom-probe.log`，Rust 对照待下轮。
- 本轮实际编译完成：矩形候选及旋转场景四个完整 Rectangle3d 逐位断言 2/2 `/private/tmp/m06-rectangle-bounds-retry.log`；矩形石底旋转载荷 1/1 `/private/tmp/m06-rectangle-bottom-epoch.log`。当前 `rectangle_pillars.rs` SHA-256 `e107e819e842a5bb10a343faadd462aa2dd3f598bc33994c63b93cc5adcf1744`、`rectangle_bottom_layer.rs` `bb6ad02cef06398bb91ecc17d9a5f7dd196051a6e196366d7c78f5c9642d1c57`、`util_pillar_construction.rs` `9be83f435b860e696718de4c83160025ccd150e07799d7df312ba5c8c60fd7df`。正式 owner wrapper 已编译但仍无 World 调用链测试。
- 恢复半木矩形底层四边位置、原四元数乘法与矩阵列运算：旋转非方形场景 4×64B 全字段逐位通过 `/private/tmp/m06-rectangle-wood-rust.log`；源码 SHA-256 `a774ef59bd8c3e888fd94eae88e8a54930959e3fa5be7a3f52609ab7dc9ae34f`。仍未接正式 plank owner/consumer，其他几何及边界未验。
- 原木柱 0x141fe2150 的单条高度 4 旋转提案产生 3 条 64B 木板，保存 `m06-wood-pillar-native-output.json`（SHA-256 `eda27a928b2d9117d9e95b316aa22960bee994b878699fa562af497ce837e9ff`），原探针桥接 lock、SSBO、alloc 和后续 crossbrick，尚非完整木柱原差分。前两次模拟锁释放失败日志 `/private/tmp/m06-wood-pillar-probe.log`、`/private/tmp/m06-wood-pillar-probe-retry.log`；桥接后原函数返回日志 `/private/tmp/m06-wood-pillar-probe-retry2.log`。p5 是 wall identity、p6 为 RNG 引用；此结论由 p5=123/p6 指向 77 时木板 wall_id=123 核得。
- 原木柱低高度 1.0 对照输出仅主体 1 条木板 `/private/tmp/m06-wood-pillar-short-probe.log`，fixture SHA-256 `f38c2a5d01db6edd9735af96eca29ef7a2a14e37eeb86b513c446d8794321465`。新增独立主体候选和逐字段测试，源码 SHA-256 `37c029423be70032786eecd1bd60cccfc0fad21fe5414f23c1a7a1f81e81d2ec`；测试待中央统一批次，斜撑/石基与生产接线仍缺。
- 木柱石基调用 0x141fe2c5d..2cd3 已确认：宽 `min(w,1.3)-0.5`，上端 `bottom + ((w*w)*w)*0.5`，下端为原 bottom，flag=true；调用截图文本 `/private/tmp/m06-wood-pillar-cross-call.log`。斜撑高度门为 >1.25，其插值用原 inverse_lerp(1.25,3,height)，端点水平偏移取 min(w,相邻边长×0.25)。尚待实现及原侧完整差分。
- 取消原 crossbrick 桥接后，同一木柱原函数执行输出 3×64B 木板及 8×96B 石基砖，日志 `/private/tmp/m06-wood-pillar-full-probe.log`，砖 fixture SHA-256 `293ee48ab4f7e103ce3fe895824ab95ea365e11d921f88a6c3dbeffc69e99f21`；保留 SSBO/分配/锁及 macOS libm 桥接边界。中央报告当前 wall-constructor lib 107/107 通过，包含短主体测试；完整木柱仍未实现。
- 木柱主体、按高度门控的两斜撑和石基顺序候选已写；高柱 3 木板+8 砖、低柱 1 木板+8 砖逐位断言尚待运行。当前 `util_pillar_construction.rs` SHA-256 `44c5bd6d8d6d8934ebfa5c488de9822db9a9ad2a866bcf68e6081d03ecd48d7d`；callback 版本尚未接正式两个 writer，不登记生产完成。
- 矩形消费者缺墙规则更正：0x140788a80 lookup 失败调用 panic；找到墙但非矩形则跳过。此前向中央发送的“缺墙跳过”属于误读，不得用于实现。
- 矩形主消费者首轮已接真实检测事件、底层两材质、柱候选、正式木/砖 owner、碰撞存储、stats 和音效；缺墙 panic，非矩形跳过。真实 World 验收候选串接检测→消费者→碰撞维护，检查重算不累积、删除碰撞；尚待本轮编译和原非空交集验证。当前 `mod.rs` SHA-256 `fa4e38ea80f0b450f6b644df328962d6003e14b5761fee284cad88a3c94f20f9`，底层 `e74173db57b9dfbe07f90c1899beda2bce45d07274b92ca431ab2c89b8573723`，柱工具 `0841e4b7aec4a5de3bdb3f2a39c00dcf7d2b498c42ba4239e88b52b19f84a31e`。不得据旧批次测试把新接线宣称通过。
- 中央本轮 wall-constructor 121/121 通过（`/private/tmp/root-m6-support-half-plaster-round3-20261029.log`），包括矩形半木双 owner World。此后圆主入口已接类型化分类、底层/木塔/圆柱、stats、碰撞和音效，新增圆真实 World 反例，当前 mod SHA-256 `3a673d0cd2c66a8fee1982646b5a8cb760fc7ba750522f85ba2ee3743dafa343`，尚待本轮编译。圆柱非空邻墙及塔楼边界另由圆依赖 owner 继续验证。
- 矩形小托架非空单段原探针以 finder 真实 Curve2、圆墙半径2/高程1、矩形5×5/高程4 执行；原函数返回 count=27、54×96B 砖，fixture SHA-256 `5792a3f357c29913e300c8ded69a18af47344eea5449472c5af6717bacdc4cdf`，日志 `/private/tmp/m06-small-bracket-valid-probe.log`。桥接 PublicWalls lookup、TLS、内存、SSBO/锁和 macOS libm；前四次缺桥接的失败日志保留。小/大托架算法及主消费者尚未恢复。
- 同一小托架非空场景半木分支原返回 count=27、27×64B 木板，日志 `/private/tmp/m06-small-bracket-wood-probe.log`，fixture SHA-256 `1eeb07b9d86dddba3791eeb0c6509324495c7894391c0f1329c0c0e31edb5a17`。石/木两材质原结果完整，桥接范围同前；几何裁剪和生产代码仍待恢复。
- 小托架原写入前中间几何提案石材54条、半木27条已捕获；新增 typed 转换到砖/木板并分别加入逐位测试，当前源码 SHA-256 `e81a2ad616b21bd675d2aeec3eb5fd6cc12aeee6d5662fe391b929e5dae88f97`，半木中间数据 SHA-256 `e2e85fb30fa4cebf9f46e9e4d91e784852700a43058fe094cc756e2affca455e`。两项尚未编译，主裁剪/采样和生产接线仍缺。
- 大托架非空原探针：finder 矩形 segment，模拟 raycast 命中 tag6/位置(0,0,0)、地形0；原返回 count=7、7×64B 木板，日志 `/private/tmp/m06-large-bracket-hit-probe.log`，fixture SHA-256 `a7d10c5cb5a1a52e8040f1df790b60d9bae49fc80434ba79672faa0e6e0cdf35`。射线与地形明确为桥接，仍须真实 World 校验；前两个无命中样本输出0，不能作为生成算法通过。
- 小托架另一 flag=true 矩形邻墙段原探针返回8支撑/16砖及16条原中间提案（日志 `/private/tmp/m06-small-bracket-rect-probe.log`，输出 SHA-256 `995fa76a57a66e510e4c31ea34d73a14426cd3fa6d2c2f96e413a2b7e740e7f6`，中间 SHA-256 `2b4e91db7cd2adb1b3e06286d799ff75fa9ba155beb9189375d9ac34cc8dc7ab`）；尚待 Rust 对照。
- 图数据库 snapshot `053060f0c5d388b353567d8b` 确认小托架调用 `Curve::crashy_resample` 0x1411f97b0（圆邻墙，spacing 原浮点0.480000019）和 `Curve::crashy_resample_per_segment` 0x1411fa6b0（矩形邻墙，spacing 0.600000024）；第二查询首10行截断，仍需分页完成。首次原采样已捕获27点及原参数 `m06-small-bracket-resampled-native.json`，SHA-256 `2f83cf096413514c13e7dd898ed2a42c8acd45efbb1b4d83afca3fb30a413200`。现有 `utils::curve` 有对应实现，需逐位复验后接入。
- 0x1411fa6b0 图查询第二页 `/private/tmp/m06-resample-per-segment-graph-page1.json` 返回 PDB 对应完整符号且不再截断；静态关系仍不能证明调度。
- 小托架矩形邻墙的原 `crashy_resample_per_segment` 产生4个采样点，fixture `m06-small-bracket-rect-resampled-native.json` SHA-256 `ae48191091afa14073d484b0c9fd11b7995d9615b9266e7646182d5d6f2a21dc`；同场景总支撑计数8，说明不能将重采样点数直接当完整托架数，仍有角点分支须追踪。
- 小托架按邻墙类型选择已有重采样函数，并按邻墙去重、顺序收集目标矩形内角点的 typed 阶段已写；两个原采样及四角原输出断言尚待本轮编译。`rectangle_small_brackets.rs` SHA-256 `03203d8cf4a0ac91e470add3fd0eda7b679d5bcfef994e5126c1a9ae0d70a772`。矩形角点谓词图查询 `/private/tmp/m06-small-corner-containment-graph.json` 确认为 `Rectangle2d::contains_point`，角法线查询 `/private/tmp/m06-small-corner-normal-graph.json` 确认为 `get_corner_normal`。
- 小托架现已写出采样与额外角点投射为原射线输入阶段：圆邻墙27条、矩形邻墙8条（含4个额外角点）原射线逐位样本和测试均已加入，当前源码 SHA-256 `dfb9aaa257418f8634a1c2e70564d87d6fa39c5c85791237211fbd722000661a`，尚待运行；Raycast 过滤、几何与最终系统仍未完成。圆半木 World 反例源码另已加入，mod SHA-256 `87ef25dbe9d841c226dd0f45cb6e3ab159cda4e18b23b50d7d82e7ff61a59f62`，尚待运行。
- 小托架上游进一步接入有界射线门、原 AABB near/far 和出界距离、原随机厚度及石两件/木一件几何；新增圆邻墙54/27条原中间几何逐位断言，35条射线 AABB 位型及真实 Rapier 近远遮挡测试。源码 SHA-256 `442f1cdb7a9c7d06db9260caf10bc2ff6d993f0599595130fb2d12fbe81cd0cb`，尚未编译，且多墙遮挡/完整消费者未接。
- 小托架新增 typed 候选入口：逐段重采样/首次角点、其他墙遮挡、0.3 射线、AABB 出界、材料几何、正式 owner 写入，源码 SHA-256 `5af9f644f5552db54526e6780e5ed854a5064fb677678dd7348c125c4aa2b995`。测试仍待集中编译；多墙 HashSet 次序/异常与完整 World 未验证，不得注册或称完成。
- 独立原指令复核推翻旧 detector 会话逻辑：任一会话 reader 非空则该 reader 消费至末尾、置 Local 并立即返回；先后两个 reader 可连续推迟，两者均空后 Local 只使外层扫描，不直接选择非零墙。已修改并新增双会话、墙/砖事件保留、dirty 保留及 Local 单独不选墙的真实 World 测试；尚待编译，mod SHA-256 `cefa6a3b2622ba6433501324e5e345d35899617578dc1e12db771600c30893eb`。旧 detector World 测试通过不能代表新会话合同。
- 小托架旋转平移目标墙（轴0.6/0.8、中心1/-2、尺寸5×6）原探针：27条射线经目标 AABB 判定后产生13支撑/26砖，日志 `/private/tmp/m06-small-rotated-probe.log`；输出 SHA-256 `ad74b8c147d32a5612de7ad1fc9e5ffd444c641dba96e6ce60eb4657b87d5597`，中间几何 `705ab5406bd57844eee7813175bec1bb339615b65a55febcdd4ff0810638dbda`，射线 `cbe73e7951d7b0d828d4ba3c42d612b99cba10308f5b7871abeb69e6cc2ab087`，局部 AABB `6e18bc5e98f06d053c2ca1ecb53b51ed2c8ea3d77513a61bf3d9da4e03164d21`。桥接仍为 lookup、TLS/分配、SSBO 和 macOS libm；此样本覆盖目标逆矩阵与部分未命中，不等于真实多墙遮挡。
- 中央 round6 实测154通过4失败 `/private/tmp/root-m6-wall-parent-algorithms-round6-20261029.log`。已依据原证分别修正：会话测试断言推迟至第二帧；小托架距离用 segment.curve.length；木斜撑归一化 `(y²+x²)+z²`；偏移圆石砖52的原 `0x14241940f..426` 先 `t*t` 再乘 `(3-2t)`，旧运算导致宽 `0x3f1c2750` 对原 `0x3f1c274f`，独立 f32 探针修后匹配。完整测试仍待重跑，不视为已通过。
- round9-v2 `/private/tmp/root-m6-three-support-half-parent-round9-v2-20261029.log` 166通过3失败，其中托架 World 删除后 active 仍54：定位为原 owner drop 只入 waste，`gpu_flush` 的 `take_out_waste` 才回收，非新 reader 重读。已修测试在同一持久 Schedule 运行原 `delete_constructor_output`→`flush_constructor_output`，由 TestDevice 的真实接口执行回收，待重测；并不声称真实 GPU。第三入口 `construct_rectangle_brackets` 及小/大 typed 候选均已接，宿主 set36 尚未注册；当前 mod SHA-256 `b1ca0816d43e605dd3886f11013336d6577cc43877b0a981b26e9731725a135d`。

- 删除反例 fixture 后续修正：country-core 依赖的 test_device 不对本 crate 测试开放；改由现有本地 TestDevice 显式实现纹理接口并在未使用时返回错误，仍经正式 flush 回收 waste。当前 mod SHA-256 `f9c97169c0f358f0c9a7d0d1d9a2fe461326a12c3d0fb5c7181f0dc40676cc92`；编译与删除断言等待中央本轮验证。
- 大托架 0x1412550dd 起非 miss 命中原先采地形再判 tag；修正候选失败顺序，当前 rectangle_large_brackets.rs SHA-256 `adbe58e0f7ed4b6c8b1dbb6d75821de8ecb85e24fb74d6197365c1c2c10770b8`，尚待本轮回归。
- 中央 round10 `/private/tmp/root-m6-support-and-half-full-round10-20261029.log`：托架真实持久 Schedule 删除和 flush 已通过；完整 lib 167 通过、2 项半木失败。新增 `large_brackets_real_rapier_hit_writes_and_retires_owner`，覆盖真实 Rapier 屋顶命中、正式木板 owner 和删除退役；该新用例尚未编译，原侧射线命中位置由桥接提供，不能称同输入逐字段差分。mod.rs SHA-256 `0d80351ad34ed26050e4a666d15eeb8696c4dec95e2958fe73e39f648785d4f4`。
- round11-v2 支撑 67/67（含真实 Rapier 大托架和删除 flush）及 set36 已由中央接入；非空多墙、容量失败、完整历史/卸载仍独立验收。审查发现 Startup 一次订阅而 RaycastWorld::on_level_reload 重建发布器，添加真实 collider 的新旧订阅反例 `reload_replaces_collision_publishers_and_leaves_old_subscribers_detached`，尚待编译；原重订阅合同和 stats 清理 owner 尚未确证，不作推测性修复。mod.rs SHA-256 `896cbcc0ca63efea925c366b8f0a4d1bf4a3ef7d6b4684deb1cef10cc71d30dd`，rectangle_large_brackets.rs SHA-256 `dae806018de96093e2d76c0c7c520c6af083024d3878696d073f45e47ce184ef`。

### 三支撑入口独立复核（2026-10-29，当前源码未重新编译）

原 B Ghidra shared 只读、程序 `tiny-glade.exe`，父体反编译保留 `/private/tmp/m06-audit/0x140788160.c`、`0x140788a80.c`、`0x1407896b0.c`。下表的原算法列仅表示所列分支或样本，不表示所有输入、失败和跨平台数值完成；round11-v2 67/67 日志 `/private/tmp/root-m6-three-main-supports-rapier-round11-v2-20261029.log` 属当时源码，不自动覆盖之后源码。

| 原函数与归属 | 原算法及具体验证边界 | 生产接线 | 生命周期与未闭合边界 |
| --- | --- | --- | --- |
| `construct_circle_supports` 140788160；`construct_elevation_supports/mod.rs` | 父体确认缺墙 unwrap、非圆跳过、清柱→分类；柱模式底层→空木塔事件→圆柱，塔模式直接构造塔。圆柱有空及已发布邻墙样本，塔有石132条、半木5条及偏心邻墙样本；局部 World 覆盖柱的正式 owner、stats 和音效。不同几何及完整邻墙组合、容量失败未证全域等价。 | set24 中 detector→本函数→矩形柱→柱碰撞→flush，`chain_ignore_deferred`。 | 石柱删除由维护系统处理，墙卸载退役 GPU sidecar；真实 App 圆塔事件实体与重复 Reload 联合链仍未证。 |
| `construct_rectangle_columns` 140788a80；同上 | 父体确认缺墙 unwrap、非矩形跳过、清柱→底层→柱；矩形空交集与旋转非方形候选、石／木底层及木柱石基有原记录样本；真实 World 覆盖两材质 owner、柱与音效。真实非空多墙完整父体差分及跨材质变更未证。 | 同 set24，柱碰撞维护接真实 Rapier。 | 删除柱碰撞与 GPU waste flush 有持久 Schedule 验证，不代表完整会话卸载或实际 GPU lease。 |
| `construct_rectangle_brackets` 1407896b0；同上 | 父体确认缺墙 unwrap、非矩形跳过、小先大后，墙字段 +0x284 ≥1.6 方执行大；小托架有圆／矩邻墙采样、石54条和木27条原样本；大托架有桥接射线样本及真实 Rapier 命中 World 测试。多墙遮挡／去重顺序、门槛边界、分配容量失败与完整父体同输入差分未闭合。 | 独立 set36，晚于 set24；两个 reader 独立消费同一矩形事件。 | 实际删除经 owner 退役及 flush 的局部测试已通过；真实会话重入及完整 GPU 释放尚未验。 |

**订阅结论更正**：先前将原 Reload 判为替换全部发布器是错误的。原 1408d8b48–6a 保存旧世界并写入新世界，1408d8c69–d20 保留旧槽的 dirty 标志、0x200 字节未发布脏区和 7 qword 发布器，1408d8dbb–e41 保留旧 ColliderDiff 发布器；临时新发布器的 Vec 被析构。当前实现已按此保留旧订阅、待处理脏区与通知并重建物理数据；此前 OnEnter 重绑是基于误判的适配，应撤销。新增 world 与 `original_session_control::m06_collision_subscribers_follow_real_game_reload_and_wall_producer` 回归等待中央运行，后者仍不是完整游戏入口。地形 publisher 当前只在宿主启动 `insert_terrain_resources` 创建；原 1409457c0 的唯一已知直接 caller 是 run_game_thread，间接写入未知。

当前 `walls_on_glade_unload` 重置墙及 WallId 生成器，decorator unload 清理 StoneColumnCollisionStorage，然而 `ElevationSupportStats` 跨会话保留。三父入口将新计数与同 WallId 的旧计数比较后决定是否发音效：新会话复用 ID 时可能抑制声音。此为可定位的生命周期风险，是否与原版一致、原 stats owner 的清理时点和跨会话音频行为均未证，不应先推测性清理。

补充原侧复核：`walls_on_glade_unload@1402b1cb0` 完整体及其 `run_without_applying_deferred@140476f00` 的 PDB 泛型参数核对，13 个参数为设备、SSBO、PublicWalls、WallPlasters、WallEntities、InnerWalls、WallSpatialHash、三种颜色、WallHighlight、PlatformStairsStorage、Gates，不含 ElevationSupportStats；原函数体没有统计资源清理。`detect_overhang_support_segments@1407867d0` 的两个会话 reader 命中均进入 `140786bbb`，设置 Local 后返回，不进入后续按墙统计写入。图 snapshot `053060f0c5d388b353567d8b` 的 ElevationSupportStats 查询已读两页（100+54，末页无下一页），未见 OnExit、clear_canvas 或 remove_resource 命名特化，但类型擦除及间接 Commands 不在静态名称搜索证明范围。不得据此宣称不存在其他清理；资源类型旧注释“卸载时随资源所有者重置”已更正。

### 通用柱放置原始入口（进行中）

原 B `place_pillars@140753670`（完整反编译 `/private/tmp/m06-place-original.c`）系统参数为墙变化、砖重算、创建屋顶三 reader，`PublicWalls`、SSBO 库和 `WallPathSegmentationMasksMinusStairs` 三 Res；当前占位的全局 `ResMut<PillarProposals>` 不符合原接口。图 snapshot `053060f0c5d388b353567d8b` 的双向关系 158 行未截断，B EXE `f507a6ce…b7d03`、PDB `2d9686b8…d8911`。父体合并三 reader 的墙 ID 后无符号排序去重；现有占位已补此步骤，但仍不是完整恢复或生产接线。

原每个 0x88 掩码段的 +0x38/+0x50/+0x68 分别承载 28B proposal、f32 和 u64 索引数组，按三长度最小值遍历；索引分别检查旗标和坐标数组，再经 +0x78 LoopRange 映射实际准备曲线。自由墙端点有 CurveSlice 延展，不能替换为等间距。当前 `WallPathSegmentationMask` 仅保留 flags/coords/range/length，丢失这三组数组；尚须恢复真实上游写入及所有权后才能闭合生产路径。Ghidra xrefs 显示 `PillarProposals::add_proposal@1407534d0` 仅有 `construct_arches@1407610f3` 直接调用，place 父体没有该调用；过去将提案视为 place 全局系统资源的假设不成立。石木装配、材质统计、实际 SSBO owner、原输出差分与 App/历史/卸载均尚未验，不得注册占位入口或声称 M06 完成。

后续增量：core 掩码已增三数组，arches 与 lintels 上游实现由独立 owner 恢复；lintels 原函数也内联写入三数组，不能从 add_proposal 的唯一直接调用推断它只有一个生产者。原 place 的 writer 取 PublicWall+0x120 砖 / +0x130 木（1407544bb / 1407543c2），与支撑 +0x150 砖 / +0x140 木不同；中央已拆对应 sidecar。当前 place 模块新增三数组 zip / key 检查 / LoopRange 插值、自由墙 XZ 两端 0.1875 延伸和木记录构造的候选 helper，尚未进入生产入口或运行。石柱 14120d670 复核纠正既有候选：proposal 最大高度与 score 最小高度分别是 XMM2/XMM3，双段为 `[0,score+0.2]`、`[score+0.2,proposal+0.2]`，段数取 `ceil(span/0.56)` 而非 round。闭包 14120d340 另要求按提案方向定向、单柱计数 seed、BrickSourceId 和 `proposal.u * Public+0x28c ±0.23`；旧 `PillarEmitContext` 固定原点轴向数据不可作为生产恢复。完整原侧逐记录输出和父入口接线仍未完成。

本轮父入口候选已消费真实掩码、逐项检查并追加通用砖/木 owner、末尾写 MaterialStats；原入口 set28 挂载和真实 App 仍由中央验收。`/private/tmp/root-m6-place-streaming-round56-20261029.log` 的 wall-constructor 189/189 包含实际 lintel→木柱 World（含无效 key 时已写前缀）、石柱原28组96B、木柱原6组64B及自由墙5组12B逐字节比较。石柱仅桥 allocator/ceilf/writer，木柱/自由墙为原指令有界片段，不等于完整父体原差分。后续加入短段4组原样本断言等待本轮执行；另一 owner 已增加真实 arches→石柱 World 测试，尚待串行运行。最终接线、编辑/历史/删除和 Reload 仍不由此证明。

`/private/tmp/root-m6-place-arch-short-round57-20261029.log`：wall-constructor 191/191，新增真实 arches 段→石柱与短段4组原坐标均通过；同批仍含真实 lintel→木柱、原石28、原木6。此为当前有限原输入与 World 验证，不覆盖完整父体、真实 App 或历史与卸载。

原重算事件与清理关系：B `switch_roof_type@1421ac4f0` 每个类型切换事件发送 `RecalculateWallBricks`；原同一 `SwitchRoofTypeCmd` 亦被 set22 arches 接收，arches 清通用+0x120 砖 owner。`gpu_clear_wall_gate_pieces@140773c00` 的 typed 参数只读取 `OnWallChanged` 并清+0x130 木 owner；屋顶转平时只有特定来源且高度不等1.6才改 InnerWalls，尚不能证明任意切换都同时产生墙变化。`CreateRoofCmd` 来源还包括形状创建、历史重放和快照恢复，是否在所有场景伴随墙变化仍未闭合。当前宿主额外的 `finalize_wall_holes` 可单独发重算；新增真实洞口→finalize→place World 断言记录此宿主单独事件的追加行为，等待执行；不据此宣称原版正确或作去重修复。图中 `RecalculateWallBricks` 两页 200+44，末页无续页，静态名称不能覆盖类型擦除与间接生产者。

2026-10-29 功能验收增量：在既有 `original_session_control.rs` 增加矩形正式命令→InnerWalls→`publish_public_wall_state` 真实事件→`construct_floors` 的候选测试，墙高 7.25、海拔 1.75，检查楼板和 Nani 描述及位置；尚待中央编译运行。原 `AdjustWallHeightCmd` consumer 自身不写历史；抬升由 `ui_raise_shape::impl_raise_shape` 的交互阶段修改权威墙并写 ChangeElevation，不能把手工改 InnerWalls 或伪造历史称为完整编辑链。跨进程 saveworker、新进程重建、历史重放、删除和 Reload 的该组合尚未完成；此前单独楼板 Reload 和屋顶快照测试不能代替它。

本批追加 `m06_floor_edit_save_restart` 跨进程候选：CreateRectangle 以 height 7.25/elevation 0 创建，合成 UI phase 3/4/0 经过原 `ui_raise_shape`/`impl_raise_shape` 改海拔并生成真实历史，发布墙后生成楼板；撤销/重做检查楼板消失和再生成，真实 saveworker 写快照，新进程读取后重新生产楼板，随后撤销抬升与创建以检查墙删除及楼板消失。此新增测试尚未编译或运行，合成 UI 输入不代表 OS 输入；同 App Reload 和 Nani 实例资源退役仍须与既有实际 Reload 测试组合，不能将本候选当成完整功能验收。
