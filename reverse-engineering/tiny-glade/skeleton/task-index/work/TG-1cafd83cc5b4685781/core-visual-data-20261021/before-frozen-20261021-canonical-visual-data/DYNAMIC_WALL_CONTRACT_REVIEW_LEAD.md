# 墙烟囱动态collision合同独立复核

core_business_lead，2026-10-21。已独立读取完整原包的collision所需entry/几何/overlap/common indexed输出分支，以及本合同、全常量bit表。允许进入候选。首次合同错误的arch clearance +.1已被原1407c4289→1429258d8实bytes9a99993e纠正为+.3；不得将旧合同值带进源码/测试。

2a19 entry在2a6b as_chimney之前，2a9a..2aba原缓存变换槽在两支之前；19不能保外层dst>=2静默跳过而20不强读墙目标。19先required真实wall/max_y/raw outline，两个原q偏移分开相加，fullheight及diff使用对应MAXSS operand/ordered门。+78是实际relative_heights已有同publisher闭合。全邻居含self先required subtype/rank查表再排self，height/width来自同record.bounds；不clamp rank或默认几何。arch strict>带+.3，path恰0且NaN拒绝，lower先于body，有效lower和fallback分支顺序应以完整点/面核，不能只断24Triangle。

20 required vent_pipe_collision真实AssetMeshLibrary→Mesh.as_triangles→原Affine3A逐点，保重复点/顺序。common416b真new_indexed需要sequentialindices/None normals UV；4270同一Arc双槽且world ZERO/IDENTITY，别二次乘cached SRT。借用既有CurveSlice/remap/approx与canonical box入口，避免额外曲线clone和第二registry。

测试必须使用真实publication/prepared path/height与不规则mesh到system/physics consumer，覆盖entry后的required失败和相对高度/arch/overlap边界，不照实现重算oracle。该批准不代表视觉装配盒/烟雾/terrain-stair遍历全部恢复，也未运行Cargo或原程序。
