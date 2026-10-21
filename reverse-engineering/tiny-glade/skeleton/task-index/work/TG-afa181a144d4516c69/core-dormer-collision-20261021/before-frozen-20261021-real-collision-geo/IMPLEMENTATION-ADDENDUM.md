# 候选实现与复核边界

- 采用独审批准的get_collision_mesh七参数typed API，真正Arc<RaycastMeshData>缓存与同Arc消费者；不以空Vec成功。保原rank/table、同canonical RoofSection/ShapeParameters字段桥、directed circular采样、glam0.29.3 affine inverse、有序24/42点与new_convex.expect("convex hull")。
- 当前utils::inverse_lerp导出确实缺失。lead明确批准在本业务表达式内联原140c904a0/16B `(radius-lower)/(8-lower)`，完整原leaf保在helper-native-pack；没新建API、没并写utils。root以后统一补共享导出可替换这次内联，需复核本依赖差分。
- 原量化三个键是u32，排序surface→height→curvature保原；旧public quantized_from_xz/dist与假的Vec输出builder没有真实caller，按此次同源纠正移除，父visual只接受真实完整输入。
- 4个源码测试覆盖饱和/round、圆矩形×rank1..3真实凸体/Arc复用、命中跳过scaleZERO inverse、rank错误/convex失败不发布，以及24/42点闭合顺序/实际scale inverse。测试未运行；不声称native浮点差分已通过。
- 唯一生产source未写。parentvisual由tree_commands配对Local并按原调用，不由helper增加全局cache/World。前面的只读报告/LLVM成功是原证验真，不等于游戏runtime完成。
