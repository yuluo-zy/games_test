# 哥特墙窗完整主生成器与实际帽骨闭包

本包只恢复原B `generate_gothic_wall_windows`（1407e87b0/10658B，PDB441:4316）与其必要 `get_gothic_hat_bones`（1407eb500/1182B，PDB441:22896），候选基于当前已推送08ecee生产源。原任务TG-73d8f60997815a3b36、原owner不变；本包为root明确委派的唯一增量。root独占生产整合、Cargo、原程序执行、GPU和Git，本作者未执行这些操作。

## 取证范围和局限

- `GOTHIC_MAIN_AND_ROTATION_BYTE_PROOF_V1.json`：完整10658B主函数1984条指令、513B projective、253B Mat3逐字节与原RNE连续比对。
- `MAIN_REQUIRED_LEAVES_BYTE_PROOF_V1.json`：完整1182B帽骨244条及5B LibKey.new三条指令，与同原RNE逐字节核验。
- `MAIN_FINITE_TABLES_V1.json`：按实际RIP引用直接读取原RNE字符串、相对表、三档宽、三墙洞表、Dormer两尾表及必要向量。
- ReVa当前已分析B提供1544行反编译，但其function body仅8889B；不能将该provider边界冒充PDB10658B完整范围。完整assembly为本包控制流/数值/cleanup权威。共享Ghidra读函数报Corrupted BufferMgr，未重启或修改共享数据库。
- 不主张分配失败、宿主HashMap遍历次序、任意跨ISA NaN payload、原容器ABI或未定义padding逐字节等价。三行常量洞直接使用数组，免去原常量Vec分配；每行append和所有可观察业务失败阶段保留。原BrickColor padding规范为0，两实际consumer只读low byte/bit0。

## 参数、门与实际基址

原函数是DerivedDecoratorInfoResources、DecoratorStorage、PublicWalls、DecoratorBlueprints四参，不读CachedDecoratorTransforms或TransientVisualState。迭代真实storage.iter_walls及每桶iter_modified，required subtype查表先执行。

7e8c5d/8c99确认Info所在bucket stride0x50；Info起点为bucket_end-0x48（8d34），因此：

- 8d15的`[bucket_end-0x26]`是Info+0x22 destination tag，**不是Info.state**。
- 8df6/901e等`end-4`为rank+44，`end-1`为state+47，`end-0x20`为ColorId+28，`end-0x1c`为可变word+2c。
- 原门是 `(subtype & 0x1e)==6 && info.dst_tag()<=1`，高位不人为收窄。state仅传蓝图source flags和半透明；不拒绝state2。

先required墙，再required collision.bounds。LibKey.new原5B把物理EDX返AL、ECX返DL；原物理传(rank,subtype)，host canonical `new(subtype,rank)`恰保持语义，不能反转host实参。

## 主函数数值与副作用顺序

1. 8e3c..8e4a拷贝32B原Rectangle3d；8e55..8e7c按rank覆盖局部size.x：rank3=3ff64e8b、rank2=3fbf3e03，其余=3f738716。非法rank随后仍由帽骨原分支panic，而不是提前改变lookup/write顺序。
2. required dst_as_wall，传真实墙、Info、u、y、上述宽到帽骨。主函数**不调用**半老虎窗profile helper。
3. 两端差dx/dz，r=1/sqrt(dz²+dx²)，原 `(-r)*dz` / `dx*r`，x0y后调用真实513B `utils::xz_projective_rotation_bad(Vec3)`。该函数由Water唯一utils/lib writer恢复；禁止拿607或当前xz_rotation_bad替代。姿态平移为second*.5+first*.5，scale=1。
4. **在上述geometry/rotation完成后**insert_for实际id/外层wallId/state。读实际WallStyle.is_halftimbered，frame色Info+28，Brick word0=Info+2c（不等于真实map id），word1=外层WallId低32位。
5. 帽檐第一个push：subtype==7或半木墙选_hat_full，否则_hat。rank1用nani_solid_vertex_color_bricks，无deformation；rank2/3用nani_gothic_window_bricks，有deformation。原BrickColor word2=(u-width*.5)，word3=(width*.5+u)，word4=width，low byte word5=halftimbered。state!=0才半透明。
6. shape.tag()<2才required window_extra，再NameHash("window_glass")；**然后**平移加rotation*(0,0,.14)，玻璃push。光键=真实modified id+1000*rank，AlbedoMap=同一NameHash，玻璃色=WindowExtra。此平移变更继续影响后续frame、碰撞、transform和Dormer；帽檐保旧平移。freeform不读WindowExtra、不产玻璃、不加该偏移。
7. frame无条件push，用nani_solid_vertex_color_window；frame色999时改nani_solid_vertex_color_bricks。BrickColor同前，但tag byte固定0；没有deformation/光/贴图。
8. 所有视觉push之后才required collision mesh并Arc::clone，再required interaction并Arc::clone。深度取**最初局部collision bounds**的extent.z（ea107 RDX=&bp10），不取interaction bounds。位移为rotation*((depth*0,depth*0,depth)*.5)+最终平移；调用同一finite-check set_positioned_colliders，再一次性set_cached_transform。
9. 原三行常量表依次append三个RawWallHole，每次重新dst_as_wall读实际u/y。center=(u,offset+y)，dimensions=(width,height)，kind0。无需重新近似网格开洞。
10. 全三洞之后，subtype==7且shape<2，再extent.x>0有序门（unordered不进入）。**这个extent.x已由rank常量覆盖**。height=extent.y+0，depth=1.13。midY=(bounds.bottom+bounds.top)*.5。forward=rotation*Z，first=(forward*.5)*.5+(最终平移+(midY*0,midY,midY*0))。DormerHoleSsbo::gothic标量为WallId低32，second=(extent.x,extent.y+0,1.13)，tail=(forward.x,forward.z,A[rank-1],B[rank-1])，A=[2,2.2,1.85]，B=[-.011,-.131,.059]。没有新增视觉builder。

原整个10658B含错误/释放尾部均已读取；Rust RAII处理Arc/借用，业务缺key/非法extra/有限性失败不回滚已发布的帽/玻璃/框。没有默认mesh、fallback姿态或默认成功。

## 帽骨1182B

直接借用`wall.simplified_curve_2d()`的canonical core Curve2，复用已有CurveSampling，不克隆/重算曲线或私建四张表。

- 非circle：rank1不改half_base，rank2加-.05，rank3加-.1，非法panic。
- circle：先inverse_lerp(.95,1.6,真实radius)，rank1加-.05；rank2/3保原MAXSS/MINSS和零乘项，再分别减`(0*c+(1-c)*.21)+.1`及`(0*c+(1-c)*.158)+.15`。非法在本分支原位panic。
- 乘.5后，先project两端u-half、u+half，再先后world转换；先完成first、second的approximate_u→clamp→coord→required pair，再world转换未经project的center并选pair。
- 每段normal=(-dy*r,dx*r)，r=1/sqrt(dy²+dx²)。side[4]为first normal、center normal；edge为second normal。旧source side[2]混取first.dy是错误MOVSHDUP符号推导，原SIMD_SIDE_TRACE已纠正。

## 真实producer→consumer数据合同（交core lead）

原gethat返回布局是side4、first3、center3、second3、edge2；但**主函数写builder时重排**：7e91d6 movlhps把edge2+first.xy写+54，first.z写+64。最终GothicWindowBrickDeformation的15个f32必须为：

`[side0,side1,side2,side3, edge0,edge1, first.x,first.y,first.z, center.x,center.y,center.z, second.x,second.y,second.z]`。

此序恰对应现core `build_gothic_window_bricks`读取def6..8、9..11、12..14及三对pack normal；不能直接memcpy整个GothicHatBones或另建镜像组件。BrickColor六word如上，word5仅low byte定义。

当前system-decorator `visual_entity_builder::pending_core::{GothicWindowBrickDeformation([u8;60]),BrickColor([u8;24])}`仍本地组件；core systems.rs349/363只是BricksRow行视图，真实ECS query adapter及canonical类型缺口已报告root/lead。必要原consumer为B142076230 SolidVertexColorBricks与142076e40 GothicWindowBricks，后者build现core instance_updates.rs477。root委派lead协调唯一writer，不在本包私造GPUWorld或第二组件。

## 本包验证与未完门

单文件rustfmt已解析/格式化候选；未运行Cargo、Rust/native执行或GPU。10个新测试函数（内含多rank/subtype/style分支）验证真实PublicWalls/Storage/Derived/canonical mesh Arcs，不需要缓存或假resource_exists门：三类builder与键、玻璃位置延续、独立interaction bounds、固定27个洞表word、bitmask/destination而非state门、真实折线中心normal、circle rank1、前置lookup/非法rank与中途extra/interaction失败、原Dormer两尾表/宽覆盖。

独立`calc_gothic_half_dormer_profile_ws`当前仍旧简式，仍需原923B+1364B+625B及同Curve3 strict-f32策略恢复；**不是本主函数callee**，本包不宣称它完成。Dormer屋顶caller/完整set10/set25宿主顺序/cache9331/stairs62KB/unload及上述Nani组件链未齐前不mount。本包不注册系统、不扩place/UI族。
