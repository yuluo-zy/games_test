# TG609 三源可执行 schema 合同续约

本合同继承canonical-collision-schema-20261020/SCHEMA-CONTRACT-V1.md，并以其field-order-erratum-v1/ADDENDUM.md纠正bounds轴/中心偏移。既有12个schema设计包引用哈希本次已全量重验，三个原native/getters/field-order report与outline display report均ok。旧world.new_convex输入UVec3结论已过期，现为真实Vec3输入，本次不重复改world。

## 所有权与授权

root经core_business_lead明确窄委派TG609、TGda90、TG450；2026-10-21 live show均保持旧owner clutter-startup-20261020与旧work，不改claim。只生成各自唯一候选，生产由root在coherent caller组齐备后应用。TG609 before=7506ddc7b4c6b0dbe61429bbaaa27c9a02fb90a8b58224ab5c49b022ca4a793d；TGda90原映射decorator_outline.rs生产确实缺文件；TG450 before=ea2ef2119b5e8aaf340dd523278ff8b79c9e0a1ce2baacfc7837c9a6de300990。

## 恢复的值、API与失败

- TGda90原映射文件仅恢复真实枚举载荷：DecoratorOutline::{Mesh(String),Lines(Vec<Vec3>),Sphere(f32),None}；DecoratorCollisionOutline::{Lines(Vec<Vec3>),Sphere(f32),None}。Clone/Debug/PartialEq是Rust能力，不当业务。没有Default把缺失伪成None，没有空display；完整display、回调、动画仍是显式后续边界。TG450只增加pub mod decorator_outline。
- TG609碰撞真实值为Clone非Copy的DecoratorCollision{triangles:Vec<utils::geometry::Triangle>,mesh:Arc<systems::collision::world::RaycastMeshData>,bounds:Rectangle3d}。Debug若保留只打印实际triangle/bounds/mesh.hash；不发明物理shape相等性或派生PartialEq。四表按原类型直接持有collision、interaction、outline、collision_outline；不加第二registry，不保bbox投影当canonical值。
- empty()只在native-physics域可用：真实new_cuboid_at(ZERO,ZERO)经Arc共享，空triangles，Rectangle3d::new(ZERO,ONE,X)。非native域没有假shape或成功fallback，不能假称资产已加载。现system-decorator已有native-physics feature；root控制最终编译功能组。
- logicalkey保DecoratorLibKey::new(subtype:u8,rank:u8)，现variant字段名称保留但文档明其rank含义。
- insert_collision_and_interaction_as_same(key,collision,outline,collision_outline)：先clone triangles与同meshArc；clone入collision并drop旧；原值入interaction并drop旧；outline、collision_outline依次移入/drop旧。insert(key,collision,interaction,outline,collision_outline)：同样clone第一个collision入map0，移动独立interaction与两outline后，最后drop原第一个collision。无事务回滚、无静默move优化。
- get_outline(key)->&DecoratorOutline、get_collision_outline(key)->&DecoratorCollisionOutline必须required借用；缺exact key panic，显式存储None可正常借出。collision_record/interaction_record为现安全host required借用适配，不冒充原函数拼写；不逐帧clone Vec。失败保当前已证“The decorator mesh library doesn't contain”前缀，并包含exact key身份。
- collision dims_wall_space若保同名，仅真实width=footprint.size.x、height=top-bottom，不用旧X/Z bbox差值。
- DerivedDecoratorSubtypes替换同一map为HashMap<u32,DecoratorSubtype>；get(id)->DecoratorSubtype required Copy，insert(id,subtype)；View.try_get_subtype直接借同一entry，View其它真实owner API保持。
- LegacyDerivedDecoratorInfoRecord可临时保留显式legacy元数据字段及纯bbox查询供foreign迁移；删除其bbox伪造collision_mesh/interaction_mesh。旧lib Option collision_mesh/interaction_mesh/get_collision_bbx_ls不继续维持错误返回语义，实际调用方改为已证required record +真实bounds或Arc。无caller使用bbox生成Arc来躲编译错。

## 原证与实施边界

empty140b25ca0/583B、sharedinsert140b25ef0/942B、fourinsert140b262a0/996B、requiredoutline140b26690/352B、requiredcollisionoutline140b267f0/378B，以及primarydisplay140b33b00/2876B，均复用旧完整原PE+PDB包。原原始Vec36-byte Triangle、Arc+18、bounds+20逻辑值已由producer与consumer两侧证明。Rectangle3d field-order原140c98380/305B直接证明width与height字段。不开新原扫描/新工具worker。

这一组schema不能单独当可编译业务闭包应用：tree_commands负责TG4b6d真实26分支loader，lead负责TGa790 singular+per-frame真实producer与foreigncaller组协调，root/decorator_chain负责其已委派windows/UI消费者，root负责共享Cargo/lib/启动/测试。已冻结151引用/27源清单是迁移导航而非完成统计。待全部真实caller转换后再生产集成；不因尚有foreign旧API就伪造兼容geometry。

## 计划的源码验证

无feature基本typed subtype/View借用与requiredmissing测试；native-physics下用真实RaycastMeshData mesh及Triangle验证clone后distinctVec+sameArc、same-key替换释放旧owner、碰撞/交互不同geometry、outline/collisionoutline的显式None与缺失失败、logicalrank精确匹配、empty真实zero cuboid及unit bounds。保留旧测试实际语义，错误旧bbox/Option断言按真API重写，不删除历史覆盖。worker不执行Cargo/rustc/GPU；root编译验证coherent组。
