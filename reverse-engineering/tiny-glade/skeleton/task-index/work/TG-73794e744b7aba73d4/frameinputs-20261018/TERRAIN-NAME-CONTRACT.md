# B terrain readback NAME修复

原 ctor14161b870 NAME="TerrainHeight"13B@142c7fcd3；14161c540 NAME="VisualTerrainHeight"19B@142c7fb62，完整原范围/原bytes在 `TG-81dc0cd75bc98816f4/os-populate-review-20261018/constructor-pack`。两个都是u32元素×0x20000，size0x80000、memory4、align4，None实际zero upload。原同namespace Rust类型含Ssbo后缀，但NAME/hash key不含。生产此次只窄改macro显式NAME，保持现CPU采样/回读算法与consumer identity。没有从统计表推完成率。原CPU constructor@1409457c0由boot另行独立核到zero0x80000/min-2.5/max24；本次未动该CPU构造及其旧注释。

Source SHA见本目录applied-source-manifest.json；不包含未执行Cargo/GPU声明。
