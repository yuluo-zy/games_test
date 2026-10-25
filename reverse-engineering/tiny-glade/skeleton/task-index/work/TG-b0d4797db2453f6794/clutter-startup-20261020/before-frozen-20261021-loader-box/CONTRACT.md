# Loader硬依赖的原box入口续接合同

沿用 ../box-trigger-volume-20261020/CONTRACT-V1.md 的完整1302B outer/690B map/558B collect 与24点36索引lift，当前before单独冻结。唯一生产源仍原ui_edit_rectangle_orientation.rs，only现错误public box_trigger_volume→真实 width:f32,height:f32,depth:f32,&country_core::utils::components::transform::Transform->Vec<Triangle>。原任意indices/Mat4/positions map改私有box_trigger_triangles，保全部原测试域并按原算序校正。原3处测试call迁移到private mapper；不删历史测试，不改skewed_box/UI。

局部24points依原width*-.5/width*.5等乘法各lane，再literal36 indices；Transform.compute_matrix复用pinnedglam29 SRT；map按y*Y+x*X然后z*Z+prev，再加translation；第三点translation在前。完整原无finite/negative/zero/normalize保护，不添加。私有map不改变原chunk短尾/越界panic。新增真实12三角拓扑、SRT非均匀/负维度/零维度/非有限传播验例，均在原文件cfg(test)下，不新建生产文件。

共享依赖原矩阵构造与算式已在旧合同绑定；新候选需独立复核后给root。原版本仅B，无C同等性结论，未运行原程序/Cargo/GPU。
