# B MouseDepthValueSsbo

原完整1139B ctor14161e8f0见 `TG-81dc0cd75bc98816f4/os-populate-review-20261018/constructor-pack/14161e8f0.asm`，manifest绑定B RNE/PDB。输入字节len%16==0，pointer%4==0，len<80（允许64B）；None实际分配64B align4，四次movups写零，Some上传原实际字节。descriptor64B/usage22/memory4/alignment4。映射失败panic；mapped len>=desc，len%16==0，ptr%4==0，完整mapped元素数len/16。旧代码错误地以Vec4的SIMD align16做buffer alignment；原and0xf检查的是长度，不是指针。

唯一canonical identity现在是 `resources::compute_mouse_depth_value::MouseDepthValueSsbo`，16B `[f32;4]`、align4、BUFFER_LEN4；render_ssbo聚合仅reexport。`.as_vec4`是当前架构表示适配，未新造映射owner。专用factory保留原Nonezero64，无rhapsody内部改动。CPU descriptor regression `original_mapped_ssbo_constructors_keep_distinct_none_policies` 与NormalDebug实际两factory/实际library比较。新shader/CPU数值业务更深字段语义仍需各原consumer，不把此载体叫完整mouse-depth系统。

Source SHA见本目录applied-source-manifest.json；rustfmt通过，Cargo/GPU交root，非本worker已验证。
