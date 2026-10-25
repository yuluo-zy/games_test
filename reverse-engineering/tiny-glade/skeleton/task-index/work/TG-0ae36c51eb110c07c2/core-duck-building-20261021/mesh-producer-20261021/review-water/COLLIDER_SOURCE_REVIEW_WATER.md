# gen_stairs_collider_and_rt_geo 冻结源码独立审查

**通过，无静态阻断。** 本SOURCE审绑定after d68991adb17df76fc0d8096cefa29a2949edb03f60ce903bf3838222572057b0、几何increment patchc6056d3c3e404543d72acafa5d02cf3808d90d08e002f09be9bf1b66635ff7c5、whole patch25200c6bf0c77e14ddd64818d596f1d21cc5410dbc6143c233c85bf83bcbe205，合同ed54e99e+必需Y勘误15040f54、前Water原证独审af17fe7a。我不是source作者，独立读了新增完整函数/纯八点helper/6测试，与完整679B/2763B及迭代原证逐项对照。

- 七参slice/stride/三个mutable Vec对应已证10 ABI借用，未增加owner/Arc或图镜像；现真实主caller仍待作者完整恢复，当前公开旧壳无实际旧签名调用，API迁移不伪兼容。
- stride0首门保原文本；zip最短，step_by首0以后步进，previous/current流式形成相邻窗口，无collect/额外尾端。输出原有前缀保持。固定24indices和6floorindices逐项与原局部表一致；RT extend→floor extend→canonical new_convex→Some时pushCollider次序正确，None不回滚两路输出、不以sphere代替。
- 纯geometry保(-t.y*w)*.5/0*w*.5/t.x*w*.5；端0/端1 base Y按勘误有序相加，right=dy+base、下右literal-.5+right、上部XZ显式+0均已入source，不沿旧C错误关联。负width/NaN/Inf无预归一化/finite guard，dy没有被省成0。只复制每窗口两个Vec3/Vec2/f32值，原引用语义不需要frame所有权复制或分配。
- 6新测试含真实SharedShape/convex AABB、明确八点与24+6写序/前缀、多次追加、zip短支/stride不补尾、空输入stride0不写、degenerateNone仍保输出、转弯负宽及NaN/Inf/±0。逐读pinned nativephysics RaycastMeshData/SharedShape API无静态错配；尚未执行，不能把测试源码当通过。原167leaf及其6测试是本bundle前置，increment仅两处几何hunk，不重复宣称我本次独审其所有业务。
- 从d4d26402旧167候选应用increment与从67a1ed11生产before应用whole均独立正反F0/after hash完全一致；rustfmt edition2024 skip_children解析rc0。未修改candidate/source/生产，未Cargo/rustc/GPU。

此通过只针对本几何闭包与机械组合，不证明62KB main/RT宿主/最终画面全链完成；NaN payload与跨ISA逐位全等无动态oracle不作承诺。
