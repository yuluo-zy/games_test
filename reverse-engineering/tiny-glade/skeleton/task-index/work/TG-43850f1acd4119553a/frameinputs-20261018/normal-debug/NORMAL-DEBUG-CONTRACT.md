# B NormalDebugSsbo专用构造

完整835B原 ctor14161e5a0保存在 `TG-81dc0cd75bc98816f4/os-populate-review-20261018/constructor-pack/14161e5a0.asm`。元素32B/align4，fixed capacity10000，NAME="NormalDebug"11B@142c7f886，buffer320000B/usage22/memory2/alignment4。Some byte guards: len%32==0、ptr%4==0、len<320032。None@14161e665只是清实际data长度，不分配、不写零，直接把None传create_buffer。这是不能套所有mapped None均zero的反例。

原映射逻辑保留：None map panic，mapped len>=desc，mod32/pointermod4检查，完整mapped range重解释32B元素。生产用单一 `render::normals_debug::NormalDebugSsbo([u32;8])` opaque记录，不猜8个word字段业务；维持当前真实GraphicsDevice/MappedStorageBuffer API，未改rhapsody架构。原debug draw/生产系统仍需原consumer另行闭合。

验证：`original_mapped_ssbo_constructors_keep_distinct_none_policies` 实际Library→两个专用factory，记录None vs Somezero64以及各自descriptor。CPU fixture映射为显式初始化对齐页；不作为GPU成功证据。Source SHA在父目录applied-source-manifest.json；本worker未Cargo。
