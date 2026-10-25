# TG69 完整蓝图验证消费者候选 V1

仅以 B 发行 RNE 为行为证据；同组 C 库未作差分，本包不声称 C 等价。原 claim 保持 scene-clutter-producer-20261020，root 延续委派 candidate 唯一 writer；不写生产、不执行 Cargo/rustc/GPU。

## 原证与真实输入输出

主入口140827130/539B、实际 HashSet ordered short-circuit callback1412b84f0/8161B、删除提示工厂1412ba810/473B、最大墙高140827350/434B及其 map141261700/445B、DecoratorId格式器1412ba7d0/56B。合同目录 NATIVE_BYTE_PROOF_V1.json 对六范围逐指令核发行 PE 的连续全部字节，另锁11项分发表、类型投影6项表、格式片段/.5/1/3常量。

14项 SystemParams 与实际原调用顺序对应：Blueprints、PublicWalls、Storage(mut)、DeferredHistory、RaycastWorld、Roof Query、canonical DerivedResources、Backup(mut)、Dispatcher、UiSignifierStream(mut)、CachedTransforms、OutlineWallCmd writer、CursorPositionSS、单位 Mute writer。全都由同一个 World 的唯一 owner 提供，不创建第二状态图/网格库/物理世界。

## 主入口和回调副作用顺序

1. 无条件 mut-deref Storage 并调用顶层 clear_modification_tracking（只清 modified/removed 元数据，保全部 live map/d2s）；然后检查 records 是否为空，绝不以 pending 代替。
2. 直接迭代原 potentially_surviving_affected_decorators HashSet 的实际迭代序，第一拒绝即停止。Locator 使用显式 addr+decorator_id，实际140a1c5f0不是 d2s 重定向；不存在对象继续，缺 blueprint 拒绝。
3. 完整 location validator 后，仍 required 读取 subtype 与当前缓存变换，再按 Valid/Invalid 分派。Invalid 先 intended outline（非 dashed），成功入队后写 specific=true，再执行原 reason 反馈并拒绝。
4. Valid 仅 Wall 且实际 collision mesh 时调用同 RaycastWorld slot2 真实 overlap，按当前id/同墙/ordered=false。必需物理shape，adapter严格 external*mesh.local，复用同一 Arc<dynShape>；不可用相反专化 into_transformed_shape_iso。
5. 命中后先写 specific=true。tag3 源按原24B typed source 的 addr/id required 取对象。原8936/898c..89a1勘误：ty范围56..63之外或62直接 continue 下个 locator，保留已写specific；旧 investigate 合同称总拒绝不正确，不再作oracle。允许的建筑装饰源先 required subtype→collision_outline→cache；subtype3允许实际其他 blueprint.transform 覆盖当前cache。先 blocking dashed outline，再 intended非dashed，拒绝。其它hit tag保原 intended+拒绝。
6. 仅有拒绝且 backup.created，才在没有specific反馈时取 Dispatcher 实际type发图标；随后 backup.state=2，然后一次单位静音事件。验证器不直接 restore、不清 pending、不 drain history、不清已有反馈；后续恢复系统仍是原独立 consumer。

## 全11类反馈

0 Cutout：真实墙 prepared path，segment±4 wrapping signed gate 使用独立 segment_heights.len；两表相邻采样均 required，最后点规约prev/t1，pathY加真实 cut height。无 clamp/缺省高。
1 RoofBounds：position 到 position-direction*2 的两点线（direction=0仍保重复），随后发 OutlineWallCmd{wall,strength:2,kind:0}。
2 Ends：分别处理两侧；取原 geometry.outline 的首/尾与真实 arch heights 首/尾，上端Y是绝对高，不额外加bottomY。
3 Turn：segment±4门使用 prepared path.len，path XZ，所有点Y为当前cache.translation.y。
4 Top：show_max_wall_y(padding)。
5 Corner：Rectangle2d::get_point→唯一 approx_curve_coord 原语（595B）找原geometry XZ最近坐标→原加权Y→wall.max_y垂线；然后真实 named decorator_delete_scribble 网格工厂。工厂name是 despawn_decorator_{DecoratorId:?}_{Vec3:?}，duration0、position/rotation取cache、scale.5；沿共享builder的初始ZERO动画，不自造实体/mesh owner。
6 Upper：show_max_wall_y(0)。
7 Flat、8 Gable、9 MissingRoof：原无额外反馈，前面的 intended outline仍保留。
10 Lower：复制原 geometry.outline 全部点的线。
所有线均 canonical DebugLine 白色、dashed、width3；真实 show/factory 依赖已独审 companion。show_max_wall_y 按原 prepared XZ zip arch heights 最短长度、h-padding，shape.tag()!=2才required首点闭合；不扩长两张表。

## 测试与边界

新增14项源码测试，未执行：records空仍清tracking且保authority；同墙实际2id；缺blueprint不造backup；Valid仍required subtype/cache；addr优先于错误d2s；cut两张表不齐保原panic+正常6/6精确点值，Turn保cacheY；Roof重复点与真实event；Ends/Lower真原轮廓；maxY zip/闭合；Corner真工厂与动画；7/8/9无额外分支；tuple同Arc/乘法顺序；真实slot2 type62等继续并保specific；真实建筑overlap的corner override、先阻挡后意图、最终拒绝。碰撞测试用真实 canonical upsert_collider/flush/query，未mock query返回值。测试显式构造几何输入不表示游戏producer已闭合。

## 必要依赖与未挂载门

配对 Locator/Op 明确ID（root已集成canonical子组），location V4 + Circle84增量，canonical CollisionOutline::display + DebugLine/Sphere，Mute单位事件及两个入口reader，图标三函数，Dispatcher最窄语义type accessor，startup同App add_event。所有候选的真实路径/哈希由本轮单入口manifest指定。

不得 mount 整个历史 build_recovered_visual_systems：原set25的cache→derive→clear→generators→collider→flush→validate→history→fullvisual→flush前提、全部相关prepared/arch/cache producer、opaque0/1 stairs泛型caller身份及完整编辑入口仍未一并证明。Dispatcher已恢复的Move/Rotate从owned request取得真实type；其它legacy place/height变体 writer 必须提供真实投影，不能视为本包已恢复。旧cull归档入口仍含独立未闭consumer，不在本包挂载。完整卸载/删除生命周期、完整gothic producer、其它UI编辑族不是本轮完成声明。
