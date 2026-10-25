# 楼梯 floor mesh 完整合同独立复核

结论：按完整合同cf7f03cd与日志勘误908aed78成组批准源码准入。旧静默build Err条款失效，必须恢复Error日志后才drop。源码及测试尚未写/运行，此结论不代表Cargo、GPU、两个62KB主body或宿主接线完成。

完整原9864B/1917指令与raise callback1454B/336指令均已读并逐条对同RNE连续PE字节，原size/地址和所有合同常量一致；另独立读取-0.2原bitsbe4ccccd。原cleanup展开与CPU buffers/ArcDrop无缺范围。结构/Arc事实也与已审visual262f合同相互约束。

核对通过的业务：
- 13个原参数及reset+b0实体Vec的真实借用，未把writer/dirtycell/RT/floorVertices作为本helper产物；两Option屋顶高度MAXSS unordered归0。raise全部点先算，X/Z仍+0*height；正常/大usize转f32、完整enumerate和归一化的运算关联保留。
- 0/1点在p1读取失败，早于aux；随后aux[0]在最短zip前必读，不能对空width/tangent先return或加等长校验。头、中、尾三法线delta方向以及失败负零皆与原一致。
- 上下双strip没有侧封口。四顶点与法线顺序、每strip六索引、u32截断、交叉条件有序gap>0、前后记录在关闭行仍更新、插值curr*t+prev*(1-t)且不归一化normal均成立。交叉先top后bottom，两段原active分别接indices，只有current active保存cross pair，之后当前原顶点才emit；没有擅加caps/filter/clamp。
- canonical Trimesh builder顺序indices→positions→attrib<Vec3>→build、原17字节名称、build失败无实体、成功一个Arc底层Buffer按chunk克隆并通过真正NaniBundle+StairsFloorMeshChunk发布。index_count/3和已审atlas bucket入口、subset26、u28、identity transform、每spawn立即push预留Entity、末尾本地Arc release均可实现。
- 唯一发现的真实合同错误已纠正：dd949..dda68在Error全局level允许时通过log::__private_api::log写原target/prefix+Display，之后drop Err与临时Vec；不得静默、不得Debug或panic。独立proof记录target与prefix真实PE字节。

源码验收必须保留合同计划的真实builder/Commands/CPU recording backend测试，检查packed顶点和法线/索引、短zip与失败序、NaN交叉、真实Nani实体及同Arc最后一次release；GPU仅root验收。尤其日志失败测试不能因root测试logger全局静默便删除副作用。
