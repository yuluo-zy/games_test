# 装饰四表与真实调用者同步迁移合同

只读合同，TG609保持clutter-startup-20261020旧claim。未生成schema Rust候选，未写foreign源，未运行Cargo/GPU/原程序。本轮以当前源码逐文件调查，current-caller-inventory.json记录27个直接类型依赖文件、151条词法引用、源码SHA、live源任务及owner；这些数量只是编译迁移定位，不能作为业务完成率。额外真实loader/outline/module owner另列producer-and-outline-owners.json。

## 原证与当前可复用边界

复用同owner canonical-collision-schema-20261020/SCHEMA-CONTRACT-V1.md，并必须同时读field-order-erratum-v1/ADDENDUM.md。原B RNE f507/PDB2d968。TriangleVec、同一mesh Arc与真实Rectangle3d由原empty583B/shared-insert942B/four-insert996B、loader17386B与callbacks绑定；getter352/378B为required借用。字段勘误明确：碰撞record+20为axis，+28为center，+30/+34为size，+38/+3c为bottom/top。

已有可复用当前Rust：utils::geometry::Triangle{points:[Vec3;3]}；Rectangle3d/Rectangle2d；RaycastMeshData真实native-physics构造及实际形状；AssetMeshLibrary、MeshWatch、Mesh positions/normals/uvs/collision_bbx；唯一DerivedDecoratorInfoResources/View；BlueprintMeshSlot+set_positioned_colliders与实际instantiate_decorator_colliders。它们是实现资产，不自动构成整个装饰链验证。

当前world.rs new_convex已经接受 &[Vec3]，通过位视图做原hash并以float坐标构造真实hull，旧SCHEMA合同的numeric UVec3->f32缺口已失效，不能重复修。原same Arc/rigid pose仍须在loader真实输入下验证。

独立检索旧work：TG609只有已整合的DerivedResourceView Rust候选；canonical四表包、TG4b6d真实26loader包、TGda90 outline包、TGa790 singular derive包目前均只有原证/设计/旧before，没有可直接接受的typed四表/26loader/完整derive Rust候选。decorator_chain确认其并未接管这些实现；它的窗数值producer工作只消费本合同API，不重做库。当前源码generic目录循环与None bbox helper不等于原loader。

## 唯一资源应怎样替换

- 保留现DecoratorMeshLibrary资源身份，直接替换四字段，禁止第二registry/cache：collision/interaction的value为DecoratorCollision；outline为DecoratorOutline；collision_outline为DecoratorCollisionOutline。logical key继续new(subtype,rank)，variant是rank，不照native寄存器盲交换。
- DecoratorCollision为Clone、非Copy，字段只用真实triangles:Vec<Triangle>、mesh:Arc<RaycastMeshData>、bounds:Rectangle3d。不得从Legacy bbox造新Arc作为兼容值。Clone复制triangle Vec且共享原mesh Arc；bounds是原读取的真实值。
- shared insertion接受owned collision/outline/collision_outline：先clone triangle Vec+同Arc，再依序insert collision clone、interaction原值、两个outline。每次旧value在该insert后drop。four insertion接受独立collision/interaction/两个outline，先clone collision入表，依序移动其余三值，末尾drop原collision；不得改为回滚事务或略去原clone/drop次序。
- empty仅在原分支显式要求时构造：空triangle Vec、真实零尺寸cuboid Arc、ZERO center/ONE dimensions/axis X的unit Rectangle3d。它不是替代缺资产的default geometry；必须有native-physics能力，不新增形状缺失的fallback。
- 新required借用API collision_record(key)->&DecoratorCollision、interaction_record(key)->&DecoratorCollision，missing exact key panic；get_outline/get_collision_outline返回各自&enum，missing panic，显式存储None变体正常返回。不要逐装饰/逐帧clone Vec。旧collision_mesh/interaction_mesh原签名需独立inline合同；不能将名字转接到bbox几何。
- 唯一DerivedDecoratorSubtypes.map应替换为HashMap<u32,DecoratorSubtype>。producer插入真subtype；View::try_get_subtype直接借用这一map的实际值，View::subtype保持required值查询；保留两owner身份和mesh_library()引用。禁止为兼容旧Arc-record caller新建第二map。Legacy record若暂留只能是明确未迁移的纯数据/test类型；它的伪collision_mesh/interaction_mesh几何方法在四表切换时必须撤销并迁移所有调用者。

## 必须同批配套的当前caller

| 当前真实组 | 必须改的边界 | source owner |
|---|---|---|
| load_decorators.rs TG4b6d | 从原17386B及三callback恢复目录+26分支，发布真实四表；见下文硬门 | clutter-startup-20261020，尚未交给本worker写 |
| derive_decorator_subtype.rs TGa790 | singular计算直接返回Subtype；full每帧producer按原所有分支写唯一ID map。现source仅两窗ty、byte+21门/错误高度输入等不能继承为完整producer | rhapsody-correctness-20261019，需协调 |
| cottage_wall_window TG52/corner TG22/gothic_wall TG73 | 借用View取subtype，以subtype+info.rank读真实collision/interaction；只clone两个.mesh Arc，保持真实cached pose，经set_positioned_colliders。width=size.x,height=top-bottom，不再derived.get(id).collision_mesh或set_colliders(Vec bbox) | root/decorator_chain已明确消费此API，继续其自身原业务，不覆盖在途候选 |
| collision_detection TGe314 + merge wall TGe689/roof TG4f402 | existing ID走View，真key required record；直接读bounds字段，保持原查询/失败顺序。纯metrics数学helper可保留，不用Legacy bbx重建碰撞体 | rhapsody-correctness-20261019，未获写权 |
| UI calculate/snap TG602 | proposed subtype必须用真实storage address的singular算法；half/plain所需record按原读点lazy required。直接Y界/宽度；不以旧ID subtype或默认bbox解锁 | 同上，未获写权 |
| 其余visual：arrow/slit、trapdoor、chimney wall/roof、gothic/cottage dormer、balcony、flag、lantern | inventory逐项记录get/mesh/bounds调用；每个有实际geometry读的caller都切换同View/record，不能仅让签名编译而省掉输出。unused资源参数可保持，不为迁移删除原依赖 | 部分NULL、部分scene-clutter等；必须逐源normal claim或root窄委派 |
| blueprint TG69cc | 真实两个positioned mesh槽/实例化owner已有，直接复用；旧Vec<DecoratorCollision>兼容字段不能取代真实槽。若留字段仅存真record，不能继续接bbox替身 | scene-clutter-producer-20261020，未获写权 |
| dormer_holes TG49d0 | 现empty调用将受真实构造/feature影响；按原业务确认用零形状还是禁用槽，不能把空列表/None当几何完成 | 同owner，未获写权 |
| chimney_assemble_bricks测试TG917、interaction测试TG602、core_paths_integration测试及core subtype测试 | 把旧Arc<LegacyRecord>fixtures改成真实Subtype与独立Triangle/Arc/bounds fixture；保留用例的实际分支/预期，不删除失败或用bbox造Arc | inventory列明各owner；测试改动也属于同源owner |

remove_ivy_under_windows、stairs及assemble_dormer_roof_tiles等仅携带资源/类型的当前点也已列入inventory。按实际所用方法区分需要迁移与仅编译确认，不机械重写全部算法。27文件不是全游戏完整反编译覆盖；实际loader/registration额外显式纳入。

## 真实26 loader是硬依赖

当前load_decorators<H>只做read_dir后裸stem注册，DecoratorMeshHost没有production impl/caller；load_collision_as_bbx始终返回None；system-decorator当前已启用native-physics，因此其旧“feature未启用”注释也不能解释这个空实现。实际App仅init_resource了空/Legacy库，未装原四表producer。

需要恢复到同一真实App的原PreStartup三owner注册：ResMut<DecoratorMeshLibrary>、Res<AssetMeshLibrary>、Res<MeshWatch>。原plugin build140342c10在140344de6调用1132B registration140170fb0；安装合同在TG4b6d/real-collision-producer-20261020/PRESTARTUP-INSTALLATION-CONTRACT-V1.md。不能通过挂整个未恢复plugin绕过这条边界，更不能把Clutter Startup移到PreStartup。

原目录字面是assets/meshes/decorators，实际mesh名含decorators/前缀，outline是前7字节前缀门，watch必须先于AssetMeshLibrary.add。保留目录/UTF8/JSON/required mesh/convex错误，不能continue/default成功。随后完整26 subtype0..25 dispatch（19个不同目标，shared入口不代表同输出），0x1a为终止sentinel；每个分支的rank/shape/真实TriangleVec、interaction差异和两个outline值必须来自完整原body/callback，不只恢复窗0/6就宣布catalog完成。窗口业务本身也会依位置落0..10/22/23，不能只加载plain。

当前helper可复用Mesh数据和现真实RaycastMeshData构造；但Triangle shrink/convex/outline分支仍由原callback决定，禁止从BBox推导TriangleVec、默认cube、伪handle或loaded marker。原完整LoadingGameComplete/warmup仍是额外门，空资源presence不算ready。

## 缺失outline源映射与窄委派请求

原mapped源是country-core/src/resources/walls/decorator_outline.rs（TG-da90ad5ff4af16fcb5），当前生产文件不存在；合法旧owner/work是clutter-startup-20261020。schema必须恢复真实Mesh(String)/Lines(Vec<Vec3>)/Sphere(f32)/None与独立CollisionOutline Lines/Sphere/None；这两个enum不等于显示算法已恢复，不能补No-op display。module注册位于现walls/mod.rs（TG450，同owner），不是另造新目录。请root协调这两个原映射文件与TG4b6d，以及TGa790/metrics/UI/blueprint实际外部caller的窄边界；本合同未取得它们的生产或candidate写权。

## 可执行顺序与接受

先在各合法work完成同一API下的outline、四表、完整loader及subtype producer/callers候选；root集中整合一个可编译整体。不得先只改公共schema再以大量foreign编译断口结束，也不得增加兼容第二registry隐藏断口。可复用当前View、实际physics、资产库和positioned blueprint消费，无须重造这些模块。

实际CPU门包括：Vec clone独立/Arc同owner、替换drop次序、required缺key失败与显式None区别、空collision真实零形状+unit bounds、独立collision/interaction、logical subtype/rank、真实资产26表非空、每帧derive→window/corner→同Arc+pose→实际upsert→编辑→卸载清除→二次会话重建。root唯一Cargo/GPU执行；合同/原证/类型覆盖不算这些已通过。本轮无schema实现与游戏完成度提升声明。
