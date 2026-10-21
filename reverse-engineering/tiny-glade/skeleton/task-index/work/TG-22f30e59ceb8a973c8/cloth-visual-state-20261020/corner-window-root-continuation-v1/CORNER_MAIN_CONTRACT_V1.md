# 农舍角窗完整B合同与当前安全Rust投影

完整原函数1407f66b0/9983B，PDB445:1924。1878条指令已逐字节对实际绑定RNE核对并证实范围连续。字面量84次RIP使用及6个rank mesh名称直接读原PE，见CORNER_RANGE_AND_NAMES.json与CORNER_LITERAL_USES.json。此合同不是完整场景/运行通过声明。

参数是DerivedDecoratorInfoResources、DecoratorStorage、CachedDecoratorTransforms、PublicWalls、DecoratorBlueprints、AssetMeshLibrary。唯一ID subtype必须等于3，不能扩大为can_be_on_corner()。必查outer PublicWall和modified key的真实cached transform。

矩形：required wall dst tag<2，Info.u20低byte attachment=0返回None并在这里unwrap失败，其余取attachment-1，角序由同Rectangle2d guard；原get_corner_normal和get_edge_dirs_from_corner_id提供主法线及前/后两方向。后者598B恢复在TG6e76原owner旧work，由root窄委派，独审已通过。不得改成轴符号表。

非矩形：dst_as_wall，真实coord中心、coord.u加原±.1各自project_clamp再用同simplified_curve_2d映射；前/后减中心各自try_normalize，再(right-left).try_normalize，法线(-dy,dx)。任一失败原跳到829e释放临时向量并继续，在blueprint.insert前没有输出；保留真实None/continue，不拿默认几何代替。

共同：required collision key(3,rank) bounds.width/SQRT2(bits3fb504f3)=span；cachedXZ=P，A=left*span+P、B=right*span+P。主位姿位置XZ=B*.5+A*.5，Y=cachedY，原xz_projective_rotation_bad实际XZ复用同utils::xz_rotation_bad；scaleONE。先校验rank1..3，再生成恒2侧Transform：位置A*.35+P*.65与B*.35+P*.65（bits3eb33333/3f266666），YcachedY；旋转法线分别(-side.y,side.x)，scaleONE。原临时Vec容量rank实际恒push2；当前使用[Transform;2]供同borrowed slice，避免无业务可见用途的临时堆分配，不声称内存布局/分配失败行为与原ABI一致。

随后insert_for(modified,Wall(outer),Info.state)。主框mesh按rank选decorators/window_cottage_corner_{1x1,2x1,3x1}，shader nani_solid_vertex_color_window，colorInfo.u28，semi state!=0；主位姿再加rotation*Vec3(0,0,bits3eb677ae=.35638183355)。这个偏移所有shape都有，不能照搬墙窗.14/-.07。先push主框，再设置真实outline transform，随后required collision和interaction表查询/Arcclone，使用同主位置/旋转set_positioned_colliders。缺interaction时已发布框/outline，不事务回滚掩盖原顺序。

仅shape<2追加对应*_glass，位置与主框完全同一份，color required window_extra原+30u32/tag+41==6，lightkey=1000u64*rank+actualModifiedId；shader nani_solid_vertex_color_window_glass。主shader/mesh列表原名称及长度由PE表确定。

再次查required collision，height=top-bottom，span=width/SQRT2；通过dst_as_wall读取原coord并追加RawWallHole{center:[u,y],dimensions:[(span+span)+bitsbf010625(-.504),height],kind0}。接着完整add_lintels用恒2侧位姿调用：y=0、flags=1、height_parameter=span。原调用栈50为1、58为span（callee栈270/278），不能把墙窗flags0复制到角窗。callee会另加自己的墙洞，保留两条洞的原顺序。

shape>=2在此结束，不能追加普通玻璃、布或花槽。premade条件：stored Info.decorator_id wrapping_mul(aaaaaab6)+5556884b<=55555555，且rank<3时Cloth；rank1宽span，rank2宽width，height原bounds高；位置=主位姿+rotation*(0,0,.3 bits3e99999a)，Box内decorator_id用actualModifiedId、wall用outerWall，保留替换旧Box语义。此支不消耗随机。

否则从stored ID init_rng建立同fastrand1.8局部流，完整u32高2位等于1才进入。flowerbed_xforms(rank,3,主位姿,真AssetMeshLibrary)，每个实际采样index的key使用原calculate_hash(actualModifiedId,"flower_bed",usizeindex)，同流usize(0..6)选mesh_index，发布同canonical WindowAutoClutterCandidate::WindowFlowerbed。原角窗不发布shutter候选。

当前墙窗/角窗源候选必须配对：flowerbed_xforms/add_lintels恢复为真实canonical输入输出，无旧record/visual镜像。subtype四表由core lead TG609提供，ID map变为DecoratorSubtype直接值，未配对不能编译集成。当前仅rustfmt emit解析，作者未Cargo/rustc/GPU/生产修改。实际set25宿主尚未注册两窗，后续按原constructor证据另交窄patch；UI闪亮事件writer、编辑与完整卸载仍是独立缺口。
