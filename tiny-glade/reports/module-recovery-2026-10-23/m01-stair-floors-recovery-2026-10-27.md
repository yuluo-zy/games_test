# M01 原楼梯楼板几何与共享chunk生命周期（2026-10-27）

## 原边界和资源合同

原 B 同文件 `systems/decorator/src/decorator_visual/stairs_assemble.rs::gen_stair_floors` 为1407dc170；PDB物理9864B，Ghidra body6991B。匹配 RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`、PDB SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。完整原指令在 `m01-stair-floors-1407dc170-full.asm`；原callee/consumer和141262530 map、spawn等记录在两个 `m01-stair-floors-*-functions.json`。未照旧签名的空Vec投影继续造假输出。

原13参数由14000b3b8/14000dbcc调用pack及callee复核：设备、widths ptr/len、tangents ptr/len、positions ptr/len、原reset context、Commands、source u32、两Option<f32>端点修正、auxiliary ptr/len。原context+0xb0持真正Vec<Entity>，当前API直接接该借面，不造第二个实体owner；freehand负责主assemble/reset调度。

原1407dd873..8db证明 `StairsFloorMeshChunk` 为48B：Arc Buffer@0、既有32B TrimeshChunk@8、source u32@40。原14207fcc0复制+8起36B，最后一个字正是source。沿当前架构在原core/resources/stairs/visual.rs恢复同一canonical组件，保留raw来源字，不根据猜测改成别的材质默认值。

## 完整几何与现代资源复用

保留原全位置端点修正：两端max(0.15-height,+0)、按i/(n-1)线性混合，再显式保留height*0参与XZ运算；首/末相反差分和内部中心差分计算坡面法线。原末端反向影响有符号零，未强改向前差分；无效归一化输出原负零。

width/tangent/position/auxiliary按最短输入zip；逐点左右半宽展开，沿auxiliary==0.15交点裁剪。上下面各有独立前一对索引、共享交点边界，交点法线仅线性混合不归一化；底面减0.2、反绕向/反法线。没有固定quad、flatbox、补端盖/侧墙或忽略阈值分支。原仅positions和Vec3法线属性，没有独立UV流，未按任务措辞凭空加UV。原函数没有独立物理producer；对应collider/RT由原另一个gen_stairs_collider_and_rt_geo及main/reset负责，不重复造碰撞体。

复用当前Trimesh::builder/positions/attrib/build做原64三角meshlet打包；复用既有floor_tri_count_to_atlas_idx，传index_count/3，只除一次。每块spawn真实NaniBundle<StairsFloorInstanceData>和同一canonical chunk，单位Transform，source由真实caller提供，实体追加原reset Vec。

现Buffer为Copy回执、Trimesh本体没有物理Drop，因此仅Arc<Buffer>不能释放宿主分配。新增局部StairsFloorBuffer lease持同一设备Arc和真实Buffer；所有chunk共享，最后Arc Drop调用该设备release_buffer一次。没有创建新GPU host或重建rhapsody。只读核当前BufferHost.freeze_frame_resources的FrameBufferSnapshot持有物理version pins，以及release/write_or_defer，现宿主负责在飞命令资源延迟释放。

新增原canonical ECS floor Query，保留With<marker>及Or<Changed<chunk>,Changed<active>>，先锁kind26再枚举；逐位传9字+3零到现typed updater，原position=0/sort_key=1。共享Nani guard基于现16marker尾部tuple扩为17，保留brick/gothic/Smoke/Lantern等已有身份，不覆盖他人块。已加入现unordered updater tuple，未另造行镜像或每帧scratch Vec。

## 独立原指令与World验收

`m01-stair-floors-rosetta.c` 按PE section/RVA映射原RNE，真实执行1407dc170入口直到原Trimesh::builder边界，只有allocator/reallocator/deallocator/panic桥；几何/map/x0y/分支均原码。该边界捕获原indices/positions/normals，不把GPU模拟当作原渲染执行。388组输入含斜坡/反坡、7种长度至129、交替和穿越阈值、多种端点修正、独立短输入、±0/∞/NaN/subnormal/maxfinite；几何阶段3个原拒绝，所有三数组全bit匹配。夹具 `m01-stair-floors-rosetta.json`，输入生成脚本/原probe均保留。

初次比较只两个NaN端点失败。回1407dc218/241发现MAXSS第二操作数+0：unordered及tie必须取+0，伪C的<=判断会漏NaN。按原ASM修正，未放宽NaN测试。有限结果使用直接const路径；仅非有限输出局部重演原SSE NaN来源/payload及新生invalid负位型，不在末端盲目归一化所有NaN。

最终 `native_stair_floor_recovery_tests` 4/4，日志 `m01-stair-floors-world-tests-final.log`：

- 上述388原几何全bit及失败差分。
- 真实Trimesh生成跨64三角的多个chunk→Commands真实实体→同一World的Nani maintain/ApplyDeferred/floor ECS updater→真实typed payload与记录上传；静止帧不重复上传，source Changed保留完整u32位型；逐块despawn保共享owner，最后一块删除后物理release恰1次，Nani实例数归零。
- 明确设备创建错误：原log后不spawn，不制造成功buffer/owner。
- 全部裁剪为空时，几何阶段可空，但后续真实Trimesh::positions/attrib保留原空属性assert失败，没有改成假成功。388/3只是原几何边界范围，不代表完整genfloor所有失败总数。

共享 `country-core render::nani::systems` 32/32回归通过，日志 `m01-stair-floors-nani-regression.log`；使用同target、CARGO_INCREMENTAL=0、offline。serde_json仅加既有锁的dev dependency，经root授权，不进入生产依赖。

记录设备是明确CPU边界模型，只检查实际build/upload/release调用，未将模拟BDA称真实GPU，也未测量原性能。真正本机GPU、生产主装配/dirty/reset及主窗口联合验收仍由root/freehand继续；本文件不宣称全楼梯业务已完成。

## 复用经验增量

- 非系统purefn的void调试签名无效，应从两处真实caller pack恢复slice数量和所有者；同一个source字必须沿spawn到实际ECS/GPU consumer复核。
- 共享资源恢复必须先看现代Buffer是否真实拥有分配，不能把Copy回执套Arc冒充原Arc<raw::Buffer>。复用现host的release与frame pins，新增最小业务lease即可。
- 位型差分优先定位首个完整失败的分支：本轮386/388直接同位，两个NaN指向原MAXSS，不能从漂亮的C条件继续推断unordered语义。
- 原几何phase oracle与完整渲染/错误入口需明确区分，特别是全裁空还能被原下游builder拒绝；不能把边界前成功当整函数成功。
