# interpolate 同必要链只读断点（合同未闭，不是 candidate）

原8622B@1407cf500/PDB437:5700 native-pack/interpolate-full.asm.txt；旧named C只导航。真实wrapper generic PDB已锁6资源：ResMut<PlayermadeStairsAssemblyRequests>,Res<GameTime>,Res<StairsVisualState>,Res<StairsRemovedSupports>,Res<DecoratorStorage>,Res<UiSignifierStream>（2:34676 run_unsafe140004710/1196B、2:12684 init1400037c0/284B、178:3873472 run140480810/51B）。body优化为5参因为最后UI无读；**不可沿当前生产_old/_new slices→bool假签名**。

入口取 time_continuous_lerp_factor(原constant142a7ba84，值待绑定)；mem::take previous.sampled_points保old表供remove_entry，new表回装；先fold141293420发布本轮sub_segments到同previous子owner。逐actual requests序，packed identity由actual sortedsegment140959e70，不误用dirty2×u32 key。

每条先 zip current.curve.points 与 heights 生成Vec<Vec3>（minlen）；再从segment.first节点四次optional storage lookup，rawInfo+28默认0、StairExtra+8默认0、+10byte默认0、+c默认0，顺序+28→+8→+10→+c。removedsupport用真实StairsRemovedSupports.contains(StairGraphEdge)；不可在缺Info时requiredpanic。然后旧 sampled_points.remove_entry(identity)，保旧四Vec与metadata生命周期。

如果有旧sample，clone其points作Curve3.try_new_from_points(true)；valid后按 min(new points len,new curve coords len)循环，required new tangents[i]。旧samplepoints.len==new.curve.points.len时按index取旧point/旧tangent（仍逐bounds），否则用真实旧3D Curve.get_coord_at_u(new.curve.points_u[i])重采旧point与tangent；不得简单zip不同len截断或按均匀i比例。位置为old+(new-old)*lerpfactor。tangent先按 dot(new,old)<0翻转旧，lerp后safe normalize，失败原取Vec2::X而非ZERO（需ASM独核）；max位置delta²门用于dirty对比。metadata按原rawbits比较；widthlen同且差绝对阈值内，USE_STAIRS_LAZINESS==1且canonical visual entry存在且force_rebuild_requested==false 才进入reuse分支。该canonicalflag/helper由parent负责，本source不得常false替身。

reuse路径写 request.interpolated_curve 为真实Curve3::try_new_from_points(current interpolated points,true)的Option，失败保None；旧samplevalue本身重插新表保四Vec与metadata。重建分支/阈值/Mask/width/history释放及末尾删除尚待读全ASM，**以上只读不宣布整个interpolate合同或实现完成**。当前只有梯位置598+603 candidate已独审；生产其余mesh/collider主系统仍有明确U。不要把新全文包存在或旧timeout报告作实现进度。
