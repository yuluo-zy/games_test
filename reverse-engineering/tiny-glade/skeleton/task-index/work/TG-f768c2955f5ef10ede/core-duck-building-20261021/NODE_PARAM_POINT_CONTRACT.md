# request 节点 producer 与 derive 必要闭包合同（待独审）

既有stairs.rs同claim，继续完整request闭包。本叶仅替换旧derive返回跳表index的伪入口，并恢复同源码原StairPoint构造；不新权威资源、不建立墙组/transform镜像。full14080c640/455B、helpers node-fold14048ce90/840B的game closure均已由RNE/PDB identity-bound完整pack保存。

## StairsPoint 真实构造

原48B记录：DecoratorStorageAddr@0..10，Option墙normal标签@10（墙0/non墙1），normal@14/18，cached translation XYZ@1c/20/24，DecoratorId@28，StairExtra第2f32 width@2c。现代结构在本stairs.rs存addr/id/Option<Vec2>wall_normal/position/width，按语义传递，不保raw-layout镜像。

按StairsState.graph().nodes原顺序，依次：

1. actual id→storage.try_lookup_storage(id) required；
2. storage.get(addr) required；bucket.get(id) required；
3. cache.transform(id) required；
4. info.extra().as_stair() required；读raw[4..8]的f32为width；
5. 若 info.dst_tag()<=1：addr.as_wall()→walls.get(wall) required（含原assert_valid）；info.wall_space_coord().get_tangent(wall.simplified_curve_2d())返回(tx,ty)，side0 normal=(ty,-tx)，side1 normal=(-ty,tx)。否则OptionNone，inactive payload不填0方向。
6. 写实际point到唯一临时HashMap<DecoratorId,StairPoint,RandomState>。这是原本图到几何的本轮scratch map，不额外插World资源。Lookup/缺失失败顺序不得前移；不跳过孤点、不默认transform/extra/墙高。

正常原体的地图next/drop由std/petgraph依赖负责；不逆向重写HashMap/Vec逻辑。

## derive_stair_node_params 真实输入输出

原参数是(info,walls,stairs,DecoratorLocator(addr,id))；当前手工Locator只addr（缺id），为保已有重构不扩foreign类型，现代函数显式传info,walls,stairs,addr,id。addr和id来自同真实lookup producer，不是从info.dst_wall猜WallId。

输出typed `(StairNodeAnchor, neighbor_count:u8)`；canonical enum/WallId type为已审TG280候选。先读取原graph节点邻接len，原无节点或空度=0；长度as u8（原LEA/SHR/INC低位），不能饱和为255。然后原跳表规范化：dst2→roof；dst3→terrain；dst4→unreachable panic；其他先走wall并由info.dst_as_wall()断言tag<2。

- wall：先info.dst_as_wall；addr.as_wall；walls.get required；上述同normal计算，再canonical DecoratorRotationXZ::from_xz_forward_vector(normal).0（原140833a00/18B，atan2(-x,-y)），StairWallAnchor::new(actualWallId,angle)。不能用xz_rotation_bad/Quaternion yaw近似替代此angle leaf。
- roof：required extra.as_stair，首f32→StairNodeAnchor::Roof{height}。
- terrain：required extra.as_stair，首f32→Terrain{height}。
- dst4：原14080c7e2是40Bunreachable字符串panic，不返回index3或零锚点。

原WallAnchor首qword WallId高位不能经过Vec2/f32转换。输出inactivepadding不声称原初始化；typed enum不会读取inactive payload。

## 当前API可复用面

PublicWalls::get保原validation；PublicWallState::simplified_curve_2d返回原sourceCurve2（surface.rs真实f32/raw normalize），不要用mappedCurve3或prepared_path代替。WallSpaceCoord::get_tangent已接受其CurveSampling借用。DecoratorRotationXZ原18B已恢复。同Point字段必须命名normal而非tangent；原d0ab..13b翻转后交换由水worker独审命名addendum确认。

## 验收（未运行）

actual InnerWalls→PublicWalls.consume发布墙，side0/1分别得到原normal和angle；图含真实墙/terrain/roof节点、cache中各自translation，collect得到完整points而不造新World owner；required缺cache/extra/addr/invalidation保panic；roof/terrain exactf32含NaN位载荷保留；dst4panic；孤点degree0；256实际邻边degreeas u8回0而非clamp。旧“derive返回dispatchindex”测试必须改为真实值与失败分支，不能保旧测试掩盖新业务类型。

完整requests/compute/interpolate/cache仍另行实施；这个候选不冒替两30KB系统完成。

## 同源 determine_segment_type 数值与真实输入补充（2026-10-21）

PDB451:79228/185B全函数 `determine-segment-native-pack/determine-segment-type-full.*` 已绑定原RNE/PDB。原参数不是现代旧接口的两端tuple，而是 `points:&[Vec2]` 与 `heights:&[f32]`。原814先height.len==0 panic，再823检查points.len==0 panic。pointer provenance合法Rust输入不可能产生的尾地址null分支对应编译器unwrap约束，不人为引入可构造空成功。只读首尾points与首尾height，无要求两slice等长；两个都单元素亦执行除法。

弦长按原 `(first.y-last.y)^2 + (first.x-last.x)^2` 的f32顺序sqrt；高度绝对差除弦长。原86b seta `.25`，875 inc；877 ucomiss `2.5`，886 cmovbe：实现应为`if slope>2.5{3}else if slope>.25{2}else{1}`，NaN为kind1（现生产旧`if slope<=2.5`外门会错为3）。分母0且差非0→Inf kind3，0/0→NaN kind1；不做epsilon/finite修复。

原返回AL=kind且DL=1；真实孤点caller140281e34/e3a分别保存二者，后者最终入SubSegment flag。现代可使用观测typed二元`(u8,u8)`并在caller创建count1，具体原返回类型名称尚未证明，不谎称DL是Option成功tag。此flag恒1属于原body明确赋值，不是补默认成功。后续正式request消费者须同时接管二值；既有无外部真实caller的假tuple API可据真实slice修正。

测试覆盖height空先失败、points空失败、长度不等只取首尾、单点0/0、Inf、NaN以及0.25/2.5边界，返二元第二值1。此补充仍待parent独审后写candidate，不因pack成功计恢复。
