# 楼梯唯一请求 owner 必要 schema 合同（待独审，尚未写完整owner候选）

旧sourcebefore bf72...；已交小叶fullsource after2be2ff...仅canonical基础/数学，保完整原手工owner尚未扩展。本合同不把4B辅助向量未知语义伪装已解。

## 完整数据依赖

已有依赖utils Curve2/Curve3、petgraph=0.8.1、hashbrown=0.14.5、bitvec=1.0.1；smallvec=1.13.2 root已收到窄hunk。现代owner仅在既有 `stairs_assembly_params.rs`，不保 positions mirror，不另建query映射资源。

Params唯一字段拟按真业务值：

|原offset|现代字段|证明/状态|
|---|---|---|
|0..38|curve:utils::curve::Curve2|完整points/points_u/length；不是只取points|
|38..50|heights:Vec<f32>|compute+outline双证|
|50..68|widths:Vec<f32>|compute+outline双证|
|68..80|tangents:Vec<Vec2>|compute default callback+outline|
|80..98|auxiliary_words:Vec<u32>|**只证明4B trivial元素，语义/实际f32或u32未知**；这是无损观测word表示，不做数值计算。原compute/init明确empty，后续不据此宣称完整mesh消费者恢复|
|98..d0|interpolated_curve:Option<utils::curve::Curve3>|interpolate真try_new(true)→Some/None，不用空curve替None|
|d0..110|nodes:[(StairNodeAnchor,u8);2]|anchor variants真实payload+neighbor degree，保持几何顺序|
|110..128|terrain_contacts:BitVec<usize,Lsb0>|名称为观测业务含义：callback写floor+.1>height，Some heights为全false；paddingstartfalse/endtrue|
|128..130|segment:StairSegment|无向min/max两u32；几何from/to身份顺序不能因此排序|
|130..148|sub_segments:SmallVec<[StairSubSegment;1]>|kind/flag/count，flag真实businessbyte|
|148|dirty:bool|old dirty HashSet<StairGraphEdge>contains；不可常true/defaultfalse|

原Debug/Clone/PartialEq derive不是业务，当前通用Curve不实现PartialEq。若扩展owner移除未被consumer使用的PartialEq derive，不能为了继续eq而复制curve.points镜像或把原Curve改简化Vec；本source只读外部引用清单未找到requests整体eq调用。

## Request owner 与前帧子对象

Requests包含current `requests Vec`、两个真实 `DiGraphMap<DecoratorId, ..., RandomState>`、一个 `PreviousStairsAssemblyRequests` 子对象。两个graph分别weights AlignKind与AdjustedStairEdge；AdjustedStairEdge=16B {position:Vec2,outgoing_dir:Vec2}，source14028000b..82已有双向producer；不得把closure1临时28B当weight。

Previous子对象包含三HashMap<packedIdentity,...,RandomState>：sampled_points、arch_wall_supports、sub_segments。类型分别SampledStairsPoints、StairArchWallSupportType、SmallVec<SubSegment,1>；原prev_requests返回self+138此子对象引用，不是currentVec。cache只借canonical子对象内table，不另造本地“previous请求”镜像。

StairArchWallSupportType原typedinsert1402c8dc0仅本必要retained assemble两组写0/1，现代可保 `Kind0/Kind1` 明确观测variant，不猜业务名；真实support生成完整恢复前不宣称其所有业务闭合。

SampledStairsPoints原value **0x70**，RawTable bucket0x78含packedkey8，不能把0x78再用于value。原interpolate insert141266df0四Vec依次：points Vec<Vec3>、tangents Vec<Vec2>、widths Vec<f32>、auxiliary_words Vec<u32>(同4B观测，语义未知)。后16B metadata分别当前endpoint0 info.u28、extra+8、extra+c（均按u32 bits相等）、removed_support bool低bit、extra+10 byte，剩余inactivepadding不作为字段。

Metadata现代用明确观测名 `info_28/extra_08/extra_0c/removed_support/extra_10`；源由当前actualinfo读取，非硬连字符串/默认值。extra的f32/u32语义未证时保持bits做原相等比较，不用f32 PartialEq错误处理NaN payload。

## 外部调用适配与共享边界

唯一当前直接Params读取者是structure_outline源TG-d11abc3329e6f85879，live owner frameinputs-20261018，actualbefore da0baedb81fefce6ab536867f9c5c87ece5637e638bd0d1a2c3688813d75414d。最窄hunk只有原343/346/347/383 request.positions→request.curve.points及709测试fixture建立完整真实Curve/其余显式数据；不动其它prepared_path.positions，不加生产for_outline fake constructor。等待root确认，不能写foreign。

Utils canonical构造器极端float/错误顺序真实差异见TGf768/curve-constructor-readonly/CURVE_CONSTRUCTOR_CONTRACT.md；同wall周期差见readonly-preferences-water required-leaves原证。不能为避共享缺口复制算法/造默认values。

StairsVisualState lazy每边flag还未有真实producer/accessor；interpolate不能用固定false补。全局USE_STAIRS_LAZINESS身份已锁（slot14362c018→14362c010初值1），state源foreign frameinputs owner须root协调。完整mesh assemble62KB两个原函数仍未恢复，不通过owner类型扩展冒充业务完成。

## 源码落地准则

必须先由lead独立复核上述字段绑定，明确4B观测word未知边界后才写同source候选。compute原明确empty/None初始化是producer实际行为，允许忠实保留；其它字段必须来自真实输入/计算，不通过Default补失数据。完整request算法、interpolate删除、graph同步、cache所有分支仍按各自原contract实现与测试。
