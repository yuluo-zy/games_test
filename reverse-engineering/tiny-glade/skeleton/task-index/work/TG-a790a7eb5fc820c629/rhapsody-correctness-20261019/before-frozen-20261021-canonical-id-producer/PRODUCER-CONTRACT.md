# TGa790 完整singular与per-frame唯一ID producer合同

## 身份/源/范围

claim继续rhapsody-correctness-20261019，root经lead窄委派，live show与实际source before在本目录。旧candidate-derive-20261020原1893B singular/998B half/dispatch、exact-kernel-addendum-v1全量复用；本次只新增必要原producer **140812b20/4576B，PDB456:2900**有界原RNE字节包native-pack/per-frame-subtype-producer，report=ok，未执行目标。原RNE f507a6ce...、PDB2d9686b8...身份沿同工程。仅同文件完整producer和singular及其共用内核，绝不展开chimney visual或其它家族。

## 原真实6参数和同owner

PDB候选943e756bb07dafeded25746f由实际原caller1403e914d及140450e64到140812b20共同绑定：ResMut<DerivedDecoratorSubtypes>、Res<DecoratorStorage>、Res<DecoratorMeshLibrary>、Res<PublicWalls>、Query<&Roof>、**Res<CachedDecoratorTransforms>**。最后一个不能可选，不能新镜像cache。现系统缺最后参数且mesh_lib为Option是错误简化，必须恢复required输入。缓存缺exact ID时position()保原失败。

目前system-decorator/lib.rs的现tuple链是derive→cache_decorator_transforms。此合同不凭producer读cache反转顺序；原注册链/主App装配由root协调，需要明确可能读前一缓存时点。至少同一CachedDecoratorTransforms真实资源必须安装；无默认transform绕开缺失。

## 第一遍：完整存储与真实失败

- 原140812e0x..e35直接clear同一HashMap控制字节、len，保capacity/hasher/owner；不可每帧把资源赋新Default重新分配/换seed。
- 原140812e4f调用DecoratorStorage::iter。PDB原Chain<Chain<Map<per_wall Iter,closure0>,Map<Option::Iter,closure1>>,Map<Option::Iter,closure2>>，明确顺序per_wall→terrain→stairs。现core iter()只转iter_walls，属于尚未恢复的全存储API；本窄文件以现iter_walls/iter_terrain_decorators/iter_stair_decorators组成完全相同的**借用链**，不修改foreign storage、不造新表/Vec。
- 每真实bucket全部条目，不只modified_ids或窗TY。原140813149调用同一完整singular，14081316d插入ID→DecoratorSubtype；ID是bucket map key。所有8×dst原分支均执行，不跳uncovered、不吞MissingCollision/wall异常、不生成Legacy bbox记录。
- 原insert返回旧 subtype；0x1a为OptionNone niche。若已有旧值，原14081317x..82c重新全存储find_map找该ID第一记录，失败触发expect；然后受原error log开关输出完整诊断，最后Duplicate decorator ID panic，已经插入的新值不回滚。
- 原日志静态piece数组142a8f3e0：`Duplicate decorator ID: {id}\nInfo: {info}\nStorage addr: {addr}\nPrev Info: {prev_info}\nPrev storage addr: {prev_addr}\nSubtype: {new}, Prev subtype: {old}`；原最终panic前缀同Duplicate decorator ID。使用现类型Debug/Display与log::error!保同实际值/副作用门，不删除重复检测。
- clear天然删除已从任一storage消失的旧ID，不读取旧map bbox、不插默认ID，不额外申请新数组。若中途panic，同一map保之前已插入的部分结果，符合原失败边界。

## 第二遍：原chimney19/20必要业务（完整保留）

原1408131b5开始仅遍历真实per_wall桶，然后全部decorator。原140813367限定ty==0x3b，14081337a限定dst_tag<=1，其它条目不访问墙高、extra、mesh或cache。每符合者：

1. required PublicWalls.get(bucket WallId)，原1408133b6；缺墙panic。读取max_y()1408133ca，required chimney union1408133e2，只读其真实Info+30 f32，tag必须4。使用info.extra().as_chimney().f，不写extra不造字段。
2. 原maxss( max_y - chimney_coord.y, 0.0 )在NaN选择0。令 `span = maxss(...) + 3.0 + extra.f`，f32加法按这个顺序，常量3.0的原bytes在1429cd078。
3. 原140813417/41b确实计算`(span*0.0)*0.5`再加chimney_coord.u，不能把它常量折叠掉掩盖Inf→NaN。用真实WallSpace{is_loop=shape.tag()!=2,length=actual wall_space_length}及共享ComparativeSpace::project_unbounded得u。两次原wall_space函数21B返回tag!=2的loop位和真实length；现core Option<f32>仅借其is_some loop谓词，不把None当缺空间。
4. roof邻近用的世界点是`WallSpaceCoord(u, max_y).to_world_space(actual simplified_curve_2d)`（原140813483），**不是烟囱中点**。墙平面竖向中点另算`center_y = chimney_coord.y + span*0.5`，half_span=span*0.5，顺序保留。
5. 对同桶每个other条目按原dst分派：tag2走roof分支；tag3/4原unreachable panic；其余tag走wall分支。自ID仅在该分支required数据读取和比较之后排除；不能提前continue掩盖自己缺mesh/cache。
6. wall分支：先required subtypes.get(other_id)拿**第一遍或已更新的同一map**值，exactkey(subtype,other.rank) required collision_record；取width=footprint.size.x，height=top-bottom。严格`half_span + height*0.5 > abs(other.y-center_y)`，原UCOMISS unordered亦false；仅成立再调同一个WallSpace.distance(other.u,u)，严格 `< width*0.5 + 0.42500001192092896`（原142a8edf8 bytes9a99d93e）。不新写环距算法或任意bbox重算。
7. roof分支：required真实CachedDecoratorTransforms.position(other_id)（原140813572）。用第4项真实世界点，保运算顺序 `sqrt(dz*dz + (dy*dy + dx*dx)) < 1.0`（原SSE先dy²+dx²，再+dz²，unordered false）。不先平方阈值改算法，不用fallbackposition。
8. 得到任一非自ID的true立即停止内层查找，原1408136e2写subtype20；遍历无命中，原1408132d1写19。原1408132ed覆盖同一ID map，第二遍不再触发duplicate诊断。保原short-circuit所导致的required输入读取范围及失败顺序，不做空间索引近似。

## 同文件singular/half实现方式

完整dispatch与平台+08优先级、缺wall/optionalroof、lazy prepared_path.positions/arch_segment_heights、exact0/6/22+rank遵守旧已审合同与exact-kernel addendum。复用CoreCurve2的CurveSampling、utils::curve::approx_curve_coord、ShapeParameters.as_rectangle与canonical Rectangle2d.get_closest_edge_id。一个私有half控制/数值内核承接懒supplier，旧public helper维持其Result/旧强guard方言，新原路径不引入whole-array有限扫描/长度相等/height_outline fallback；不会复制一套数值算法或新Vec曲线。

## 必要TG609后续hunk

lead已授权base **aa33480b**之后另追加最窄独立hunk，不修改已freeze三源：DerivedDecoratorSubtypes.clear()调用原同map.clear()；insert(id,subtype)->Option<DecoratorSubtype>返回原替换旧值。producer第一遍检查None、第二遍明确忽略返回。现get required保持。测试验证clear复用capacity与去旧ID、insert返回旧值，不新增table。

## 验收和未验证边界

worker只产候选不Cargo/rustc/GPU/生产。root需同schema/loader/callers组编译并执行：全8×dst、平台与角优先级、prepared/arch数组真实lazy读取/selected bounds失败、不同proposed位置与旧ID无关、full wall+terrain+stair全ID/删除后无旧ID/重复ID真实panic、chimney无邻近19/墙宽高严格阈值20/roof1.0严格阈值/自项也required/缺extra mesh cache失败，以及原partial map失败边界。实际数据使用PublicWalls/Query与真实RaycastMeshData owners，不能为使其它visual编译而填默认几何。完整调度顺序与GPU仍由root验收，本合同不宣称全项目已编译或原程序执行对照完成。
