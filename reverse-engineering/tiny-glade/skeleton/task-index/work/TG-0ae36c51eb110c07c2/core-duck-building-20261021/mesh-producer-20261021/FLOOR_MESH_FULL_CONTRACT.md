# gen_stair_floors 完整业务合同（等待独立复核）

原 B 1407dc170/9864B/PDB437:137612，完整 helpers-native-pack/fn-7dc170；实际映射 callback2 在 floor-raise-map-native-pack/floor-raise-map，141262530/1454B。原字节哈希分别由 pack manifest 固定，私有13参反编译只导航，private13-v2-meta 记录保存未验证；C 的 MAXSS NaN 近似不作证据。FLOOR_MESH_CONSTANTS.json 直接读取同 RNE。

## 实际输入与共享 owner
原13平坦参数：device、widths ptr/len、tangents ptr/len、positions ptr/len、真实reset返回元组指针、Commands、Info.u28、两端 Option<f32>屋顶高度、aux ptr/len。只读reset返回+b0的 entities，不写两 writer/dirtycell/floorVertices/wallHoles。现代同源接口借 canonical entities 与Commands；不再保原错误的blueprint/假render resource。
复用 canonical Trimesh builder、NaniBundle<StairsFloorInstanceData>、NaniInstanceDesc、Subset::StairsFloor、Tree真实 StairsFloorMeshChunk(Arc<StairsMeshBuffer>+TrimeshChunk+u28)，设备Arc仍持同一个真实backend host；不另造GPU registry或mesh结构。

## Raise、法线、输入失败
依次算起点和终点屋顶补高：None为+0；Some(h)=(0.15-h).max(+0)，原1407dc218/241 MAXSS第二操作数+0，NaN归+0。inv=1/(positions.len().wrapping_sub(1) as f32)。按位置完整enumerate先产生raised Vec：t=i as f32 * inv；height=(1-t)*start + t*end；每分量执行 p + Vec3(+0,height,+0)，X/Z含原 0*height 运算，不只写Y。callback实际 scalar 141262770..7b6，Vec化路径同数学序；不加finite门。
随后完整生成法线；读点1在原1407ddc8b bounds失败，故positions长度0和1均panic，在访问aux前。first delta=p1-p0；interior delta=p[i+1]-p[i-1]；last delta=p[n-2]-p[n-1]。lateral=Vec3(-delta.z,+0,delta.x)，normal=-delta.cross(lateral).normalize_or_zero()；注意负号在normalize之后，失败返回(-0,-0,-0)，不可用(-cross).normalize_or_zero()返回+0。原倒长度必须finite且>0，否则原零fallback，不能raw normalize。
原预分配后、zip开始前强制aux[0]，空aux即panic，即使widths/tangents空。当前aux值不是Option缺省，不填0。不做“所有slice等长”的前置校验。有效行是widths、tangents、raisedpositions、normals、aux的最短zip。若部分为空，只在已证上面边界后形成空indices，并继续真实builder/build，而非提前return。

## 完整上下双面 strip 与交叉裁切
对每行先gap=0.15-aux，active=(gap>+0)有序比较；NaN为false。先取该行tangent/width/raised point/normal，lateral=Vec3(-ty,+0,tx)，offset=(lateral*width)*0.5；left=point-offset，right=offset+point。width乘+0保NaN/Inf传播。每个emit输出四顶点顺序[topLeft,topRight,bottomRight,bottomLeft]，bottom.Y=top.Y+(-0.2)，X/Z复制；normal顺序[N,N,-N,-N]，无边缘侧封口。
维护两条同active strip上次pair与prev行(即使prev未激活也保原几何)。每条strip连接索引为[prev0,prev1,new0,new0,prev1,new1]。原使用u32索引truncation，不添全域大小校验。
第0行prev gap先被初始化aux[0]同值，故无交叉。相邻两行 (prevGap>0)!=(gap>0) 时，t=prevGap/(prevGap-gap)，oneMinus=1-t；交叉位置每分量 curr*t + prev*oneMinus，法线同线性插值而不normalize。先发交叉top pair，若prevActive先追加top6indices；再发交叉bottom pair及其6indices。新pair仅在curr active时成为lastPair，active改为curr active。NaN参与sign变化可形成NaN交叉；不得过滤。
若当前gap>0，再发当前原四顶点；每strip原active才连接前pair，然后记当前pair、active=true。gap<=0或NaN且没有sign变化不输出当前原顶点。无强加起止caps、无额外尾点、无clamp交叉参数、无按高度排序。

## 真实网格发布与失败副作用
1407dd66b Trimesh::builder(indices) → dd68c.positions(vertices) → dd6b0.attrib(normals) → dd6d4.build(device,"stair floor chunk")，原17字节名称在142a884e0。Trimesh全部CPU packing/meshlet/chunk/address由canonical实现；不画替代box/手造chunks。
build Err：drop原错误并结束，不log/panic、不spawn任何实体，不改entities已有prefix；不是topdown resource的retain逻辑。builder自身bounds/分配panic保原行为。build Ok：原一次Arc包装其唯一Buffer，然后按chunks原序逐chunk clone同Arc。每chunk index_count/3给已审floor_tri_count_to_atlas_idx，再构造NaniInstanceDesc{AtlasPart(bucket),subset StairsFloor(26)}与identity Transform、实际InstanceDataMarker。Commands.spawn((NaniBundle,StairsFloorMeshChunk{sameArc,chunk,info_28}))，立刻将预留Entity push到借用entities。每次spawn成功后push；不批量全部成功才提交、不用marker替代真实chunk。本地Arc最后drop，实际Buffer直到最后chunk卸载才经同host release一次。

## 源码/测试验收计划与边界
待独审后实现同源完整9typed参数入口和私有CPU geometry helper，私有helper即原大函数数值区，不另建生产文件；主入口必须真的builder/Commands发布。测试实际CPU recording GraphicsDevice只替GPU硬件边界，复用真实builder：全开/全闭/起止交叉/多段sign变化/NaN有序门/短zip/aux空与位置0或1失败序；截取真实buffer packedpositions/normals，检查Nani subtype/AtlasPart/u28/同Arc及实体记录；创建buffer失败不得spawn；多chunk移除最后一个才release一次。测试尚未写/运行。
完整两62KB主body、rails、Arch/Bracket typed requests、实际宿主注册仍另有缺口；此完整helper合同不宣称主渲染链已挂载。
