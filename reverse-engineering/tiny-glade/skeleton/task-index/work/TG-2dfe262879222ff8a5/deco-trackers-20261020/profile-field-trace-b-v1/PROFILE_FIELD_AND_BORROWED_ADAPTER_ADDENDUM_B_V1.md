# Anchor profile 字段追踪及借用适配 addendum B V1

本包追加到 ANCHOR_CONTRACT_WITH_ADDENDUM_MANIFEST_B_V1.json，不修改旧合同。它收窄旧 V1 中「+40/+48 profile 尚未映射」这一门：原 publication 的相同字段和当前 canonical geometry.outline 已可对齐。九参数系统、Storage mutable/immutable 合同、普通重锚先写后 optional lookup、真实 collision loader 及原事件流仍须分别闭合。没有 source candidate、注册、Cargo/rustc/tests、App/GPU 或性能实测。

## 原 B 字段 trace

原 publisher 140a8b530/18515B 已在现有 owner pack 保存。本包只复制依此 consumer 所需的有限指令，并直接对 B PE 比较；不修改 foreign source 或原证。

1. 140a8cd59 将 sret RCX 指向 RBP+260，cd60 传原 Curve2，cd6a 调 14146bb70。匹配 PDB module1607/1555B 的完整函数是 `Curve::crashy_resample_remap_per_segment_2<Vec2,Semantic2D,Vec3,Semantic3D,publisher::process::closure0>`。它逐点调用实际 terrain sample 1409e9640（bc85、bee3 等），返回 mapped Curve3；当前 publication_source::collect_raw_wall_publication 已恢复这条 policy 并保留 Arc<Vec<[f32;3]>> outline。本 addendum 不再重建共享 resample 算法。
2. publisher d7d9 固定 R8=RBP+708；record base 为 RBP+500（同 publisher reuse pack 和末尾整 record copy）。d8b7/d8be/d8c6 将 sret260/270/280 的三个16B块读出，d8de/d8d6/d8ce 分别存 R8−1d0/R8−1c0/R8−1b0，即 record+38/+48/+58。所以原 Curve3 的首 Vec3 点集 header 位于 Public value+38，点指针+40，长度+48。这里是 B 原 record 的偏移身份，不声称现 Rust PublicWallState 具有同物理布局。
3. anchor d30d 读 bucket-end−258，对2a0B value 即 +48 length；d330 读 end−260 即 +40 ptr。它不是后续 path-intersection/prepared path。d33f 指向 141206c40，完整10B remap getter 从每点12B取 X/Z；d359 调 1412898e0/921B 的共享 CurveSlice::extend_right_until，mapped output 是 Vec2。PDB 的 Semantic3D/Thresholded3D 两个别名落在同函数，不据 alias 给不同算法。
4. 因而当前消费者应借 `wall.geometry.outline.as_slice()`。`wall.height_outline()` 在 prepared_path 有值时优先返回 prepared.positions；直接用它会改换原 profile，不能因两者同为3D点而替代。anchor 同时从原 profile 的两点 Y 作原 `y0+t*(y1−y0)`，不能从 prepared heights 或 bbox 补值。

新增 bounded pack 为 1555+10+921B；每段 collector status ok/overhang0，PDB procs 对地址和长度另交叉绑定。collector 的 function_boundary_verified 仍原样 false；不将此字段伪改 true。选中 publisher/anchor82条和三段共619指令/2943B直接 PE 比较，0 mismatch。此数是取证范围，不是游戏吞吐或性能验收。

## 当前无分配适配边界

当前 outline 点是 `[f32;3]`，shared CurveSlice 默认 identity 构造和一般 remap impl 仍共同要求输入 T:CurvePoint（当前 trait 只实现 glam Vec2/Vec3）。所以仅把 constructor bound 改为 T:Copy **不足够**：identity 产生 R=T=[f32;3]，随后 `.remap` 仍受 R:CurvePoint 的 impl 门约束。不能写 `collect::<Vec<Vec3>>()` 当长期适配，也不能 unsafe 重解释数组点引用。

建议给现共享 owner 一个最窄安全 API 方案（未授权、未写源）：

- 增加 `CurveSlice::from_coord_range_with_remap(points, first, last, remap)`，输入 T 不要求 CurvePoint，输出 R:CurvePoint、F:Fn(usize,&T)->R；保现 from_coord_range 的全部 coord 边界/顺序检查。现 identity constructor 可委托同验证路径，原 API 不必改名。
- 当前一般 mapped impl 的输入 T:CurvePoint 约束可去掉，实际 mapped arithmetic 只作用于 R；`remap_2d` 单独加 `where T:CurvePoint` 保 projected 所需约束。其余 f32 extend 算法、panic 时已完成 endpoint 提交、stop predicate 及采样顺序逐字保持。
- anchor 已依原分支显式处理 profile len0/1 的错误，不用空 profile fallback。len>=2 时以 `{segment:0,t:0.0}` 的 first/last 构造借用 view，mapper `|_,p:&[f32;3]| Vec2::new(p[0],p[2])`；随后调用现 `extend_right(native_maxss(dst_x,0))` 与 `.end()`，用同借用原点 Y 插值。

这个方案不新增点集、registry 或整 wall clone。每次 bottom branch 仍按原需要遍历从起点到目标距离的点段；不宣称它是O(1)、不缓存未经生命周期证明的坐标，也不改变多个 modified decorations 各自调用的原频率。额外全点 Vec3 collect 则会引入每次调用的全 profile 转换/分配，当前 API 可避免这份不必要工作；没有实测倍数或回归结论。

## 所有权与剩余门

utils/curve/curve_slice.rs 目前不是本任务已授权 companion。这里只交 API 设计；需 root live task/owner 协调后由合法 owner 写一次。04 当前仅 owned utils/lib.rs 的三 helpers，不能从同 crate 推全曲线文件权限。Storage/publication/Derived 也不因此授权修改。

Derived 双 ref View 的候选由01唯一 owner 恢复，其 try_get_subtype 可空查和 mesh_library 不是完整 collision schema/loader。bottom 原 b25c90 的 (lookup-subtype,rank) 逻辑与真实 collision@38/@3c 仍须同 canonical backing 配合；不能用 Legacy bbox 或另建 map 闭门。普通重锚 prefix 当前 core helper 的 bottom 提前返回也仍需合法 owner 窄适配。

待 source 方案准入后，中央执行的限定检查应覆盖同一实际 CurveSlice 源：数组借用与 Vec3 原 API 在相同点/target 下 coord bits 相同；XZ metric 不含 Y；零距离 f32 cancellation、零段、NaN/Inf、越长目标和 late-remap panic 的 endpoint 提交保持；len0/1 由真实 anchor 原分支处理。这里均未执行，不称完整 utils/core 或 App 通过。
