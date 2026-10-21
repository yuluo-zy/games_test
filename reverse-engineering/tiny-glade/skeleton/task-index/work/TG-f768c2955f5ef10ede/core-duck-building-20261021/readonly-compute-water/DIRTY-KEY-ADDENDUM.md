# dirty key与sample packed Identity不可混同

本补充优先解释COMPUTE-CONTRACT中“sorted identity”的泛称，不能映射为sample-map的packed u64类型。duck补充true原14027e808/80e直接从Params+128/+12c两个u32分别传1402c8640，原typedset元素是StairGraphEdge或tuple<DecoratorId,DecoratorId>（RandomState）；compute1405b1be0逐两个u32字段Hash，原比较low a/high b。

正确现代桥是同真实两endpoint ID unsigned min/max后用StairGraphEdge::new(a,b)或StairSegment的两个u32逐字段Hash；不是sample-map StairGraphEdgeIdentity的packed high-a/low-b u64。二者虽描述同无向边，Hash输入与native表示不同，不可把一个map/set当另一个owner。

true原426/427/428 source行收集旧dirtyset，false相应输入empty；compute仍执行完整计算和浮点分支，不凭source行集合/同checksum推导其它差分完成。此补充仅闭合dirty key，不修改原candidate/source/claim。
