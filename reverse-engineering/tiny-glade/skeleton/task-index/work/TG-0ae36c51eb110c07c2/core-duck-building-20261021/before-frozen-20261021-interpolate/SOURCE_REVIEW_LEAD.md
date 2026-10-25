# 完整 interpolate 源码独审

结论：原完整8622B业务源及本次10个真实World测试通过本审查，可与 canonical Curve3XZ3275955f 增量有序配对交 root 编译和实际测试。App 未挂此系统；不是楼梯或视觉宿主完成声明。

原1196B wrapper 六资源与当前 typed Res/ResMut 相符；UiSignifierStream 虽正文无读仍保只读required资源。原 factor40B复用canonical helper；旧表take、段表extend不clear、arch表保持、逐请求旧identity remove一次、末尾drop余项均保。四次optional lookup依次原raw位，先判 extra_disc<=1 再parser，不让未知disc误panic；required采样下标前门次序保原。

原 old Curve3XZ 与 output Curve3 分开；索引相等直接旧采样，否则按旧XZ参数取segment重采，实际1407d003d..008c为old+(next-old)*t，非weighted形式，不机械套其它cache的加权算法。位置delta、(dx²+dy²)+dz²阈值与tangent负dot翻旧方向/reciprocal有限正门/Xfallback已与主asm核对。门序与rawmetadata、同widthcount/ordered width阈值、真实全局和同owner visual force门相符。

reuse原同位置数clone旧points，异数move当前插值points，OptionCurve3失败仍完整保旧四Vec和dirty；fresh即使XYZ失败仍发新history、清第四Vec并设dirty。Rust owned Vec/HashMap move/drop保持释放，未写unsafe裸指针或默认几何。

10个测试使用真实graph→generate_requests→首帧consumer→下一帧history输入，覆盖XZ重采/前插值motion门、tangent/widthNaN、五元rawmetadata、missing/非stair/未知extra、visual/global三个门、消失identity和段表/arch保留，mandatory NaN-Y XZOk/XYZNone保opaque与dirty。测试修改原全局时本模块共用mutex和RAII还原；当前其它实际源无store/swap并行测试写该全局。

本次重新核实际生产before和全部现依赖SHA，隔离目录patch --fuzz=0无offset/fuzz精确重放67a1ed11。未写生产、未运行Cargo/rustc/GPU；最终可编译性及新增13case行为由root验收。B证据仅证明本范围，不外推C/原注册specialization/fullcache/full62KBmesh/真实UIproducer。
