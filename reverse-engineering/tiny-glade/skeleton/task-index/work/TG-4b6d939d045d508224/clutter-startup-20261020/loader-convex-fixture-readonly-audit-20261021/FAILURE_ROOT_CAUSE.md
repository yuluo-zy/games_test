# Loader invalid-convex 失败独立审核

结论：**负例 fixture 的几何分类假设错误，需要只修测试输入；loader / RaycastMeshData / pinned Parry 算法无须因此改变。** 原测试三点构成非零面积的平面三角形，而锁定 Parry3D 接受二维闭合双面凸包。不能增加“必须有三维体积”判据、默认bbox或删除断言来让测试通过。

## 实际失败和代码路径

root日志 `/private/tmp/tg-canonical28-decorator-tests-20261021.log` 第425..428行：`load_decorators`正常返回后，测试第1239行的 `assert!(result.is_err())` 失败。没有“Convex mesh”构造panic。同期 full/demo 四表52/34键成功测试通过。日志整体275/280，另4失败不在本审核范围。

冻结loader after7eac与当前production逐字相同。本函数先register全部JSON，随后subtype0..5各rank1..3写入18项；6/rank1读取 `decorators/window_gothic_1x1_collision`，经convex_collision→真实new_convex。现坏输入是positions[(0,0,0),(1,0,0),(0,1,0)]，index[0,1,2]合法，文件/资产/索引没有制造早期错误。该fixture恰好能构造库支持的平面凸包，因此继续正常注册不是bbox回退的证据。

## 原证没有额外三维体积门

原B closure3 `1407bb160/833B`：7bb19e取真实未索引Vec3位置，7bb1d5调用原new_convex，7bb1da/1df只判断Option判别，None才去7bb34d真实expect panic。成功后才取三角形/bounds并发布record，未检查点云必须满三维。

原new_convex `1408cec10/557B`：先按相同float bits求hash，再调用真实Parry try_convex_hull1416c4880；成功后调用ConvexPolyhedron::from_convex_mesh1416b3760，两个失败返回None。无额外volumetric gate或bbox兜底。本次独立重核上述共1390B、336条指令连续原字节，NATIVE-BYTES-RECHECK.json绑定RNE f507；未执行原target。

## 锁定依赖的真实分类

Cargo.lock固定Parry3D0.17.0与PounceLight Rapier0.22.0 commit7b5ece73c45a4d39bb983c48c283e773d7241050，未用本机另一个0.17.6的行为代替。主源码hash在AUDIT-MANIFEST.json。

`parry3d-0.17.0/src/transformation/convex_hull3/initial_mesh.rs` 第103..139行的dimension2分支先求二维凸包，再分别三角剖分正反两面；npoints3得到3个顶点、两张相反三角面。`convex_hull.rs` 第34..40行直接返回该ResultMesh。`shape/convex_polyhedron.rs` 第132..380行要求闭合manifold/不重复面内索引，但没有非零三维体积门；两张相反三角面共享三条边、无t-junction，所以返回Some。root真实执行记录也已证实该输入被接受。

能保留“合法JSON/索引，但真实凸形构造失败”的最窄反例是三点共线[(0,0,0),(1,0,0),(2,0,0)]。规范化后方差只在X轴非零，dimension1分支用build_degenerate_mesh_segment生成两顶点与索引[0,1,0]/[1,0,0]；from_convex_mesh在第164..166行首次重复索引检查返回None。这里不依赖调大容差/NaN不确定行为/空文件或非法索引。原loader的expect继而真实panic，之前18个key保留、key(6,1)未发布的原断言仍应成立。

## 最窄修复交付与边界

`ROOT_INVALID_CONVEX_FIXTURE_TEST_HUNK.patch` 只改此测试的中文注释和第三个顶点，原函数/所有失败断言/18项保留断言完全不变。基于7eac的独立完整after会是da73d06c，但这不是让另一个writer覆盖source的许可。root peer另有真实B资产测试候选0ce，唯一writer必须只吸收该两行hunk后重新冻结组合hash；本worker不改该候选、不写production、不执行Cargo。

建议root对组合源重跑此负例及full/demo正例。后续可将原平面三角形保留为明确正例，防止新worker又错误增加体积门；本次只交必要负例fixture hunk，不扩生产算法。Curve3XZ合同断点已保留，当前按lead优先级等待本失败闭环。
