# 楼梯 owner/node 独立源码复核（lead）

结论：本包最窄源码范围通过；完整原合同已先独审，现逐项读完整候选与增量 patch，隔离 `patch -p1 -F0` 从真实生产 before 重放，退出0且字节完全等于冻结候选。未 Cargo/rustc/GPU，未改生产；不能因此称完整楼梯链完成。

同 owner 保真实 Curve2/Option<Curve3>、两有向 GraphMap、前帧三表，删除无实际使用的整体 PartialEq，未复制 positions 权威。Params+80 Vec<f32> 有实际装配器 MOVSS/UCOMISS 读零证据；其专名仍未知。Sample第四Vec未证明转接此向量，保 Vec<u32> 观测 words，不做同size类型传染。元数据按 u32 bits 比较保证 NaN payload，不把漂浮相等替原位相等。AdjustedStairEdge 真实16B含 position/outgoing_dir，非临时closure28B。prev_requests 返回唯一 previous 子对象。

锁定依赖 API 已从本机实际源码复核：country-core petgraph=0.8.1 的 DiGraphMap 支持第三 RandomState 类型参数（0.6.5 alias不同不可混用）；utils Curve2/3 可 Clone/Debug，无 PartialEq；bitvec1.0.1 已 pin。smallvec1.13.2窄依赖仍由root统一。

外部唯一 Params struct literal 与四直接positions读取在 foreign TG-d11abc 的 structure_outline；附ROOT_STRUCTURE_OUTLINE.patch仅作root可评审最窄提案，未生产写、未擅改foreign claim。该调用适配与依赖必须成组整合。只含权威值与数学，不以新增struct当完成双30KB请求/compute/历史/mesh业务。

生产before：`bf72c02a4aed2cd5356e666bb6c6652b43fee72b5d225a556dbb0352f466d18e`；候选：`2a046da8ee6e909304adc12dc4901a64e1bd4a44cb4aa506b7b4085ce29f5e1c`；patch：`3d744be256566589c2615beb764d9cd6737895162067dcb96add6e4c00bd7325`。
