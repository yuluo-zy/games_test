// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/float_grid.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2c00de9a66430be7005aec53777bfefce6b1f7df6363c45d661948983354efd7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: float_grid.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 364, 391, 392, 399, 409, 411, 430, 438, 445, 452, 453, 463, 474, 479, 492, 495, 499, 1011, 1012, 1202, 1321, 1327, 1368, 1390, 1395, 1398, 1461, 2091, 2154, 2374, 2395, 2414, 2830, 3381, 3386, 3513, 3514, 3515, 3607, 3608, 3609, 3613, 3621, 4206
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::float_grid::FloatGrid::load_unchecked
// 记录形式: inline_instance, inlinee_declaration; instances=434
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53B6
// 首条位置: {"module":364,"inlinee":21430}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=128
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":391,"record_offset":6812,"parent_record_offset":6784,"ancestor_record_offset":6784,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=260
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":364,"record_offset":11700,"parent_record_offset":11664,"ancestor_record_offset":11664,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=256
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":6896,"parent_record_offset":6844,"ancestor_record_offset":6784,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=128
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":6948,"parent_record_offset":6844,"ancestor_record_offset":6784,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=128
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":6872,"parent_record_offset":6844,"ancestor_record_offset":6784,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":409,"record_offset":24628,"parent_record_offset":24604,"ancestor_record_offset":24516,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":409,"record_offset":24580,"parent_record_offset":24548,"ancestor_record_offset":24516,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":409,"record_offset":24604,"parent_record_offset":24580,"ancestor_record_offset":24516,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=128
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":391,"record_offset":6844,"parent_record_offset":6812,"ancestor_record_offset":6784,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::float_grid::impl$0::bilinear_sample_at_world_xz
// 记录形式: inline_instance, inlinee_declaration; instances=140
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53AF
// 首条位置: {"module":364,"inlinee":21423}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":399,"record_offset":33384,"parent_record_offset":32988,"ancestor_record_offset":32988,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00399.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=83
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11604,"parent_record_offset":11076,"ancestor_record_offset":11076,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=83
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11336,"parent_record_offset":11076,"ancestor_record_offset":11076,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=83
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11248,"parent_record_offset":11076,"ancestor_record_offset":11076,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=84
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11972,"parent_record_offset":11076,"ancestor_record_offset":11076,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=83
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11160,"parent_record_offset":11076,"ancestor_record_offset":11076,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=83
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11512,"parent_record_offset":11076,"ancestor_record_offset":11076,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=84
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":364,"record_offset":11636,"parent_record_offset":11076,"ancestor_record_offset":11076,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":430,"record_offset":40708,"parent_record_offset":40232,"ancestor_record_offset":40232,"inlinee":24890,"source_line":577,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=388
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":364,"record_offset":11664,"parent_record_offset":11076,"ancestor_record_offset":11076,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=110
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":364,"record_offset":11392,"parent_record_offset":11360,"ancestor_record_offset":11076,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=110
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":364,"record_offset":11448,"parent_record_offset":11360,"ancestor_record_offset":11076,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=366
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":11544,"parent_record_offset":11512,"ancestor_record_offset":11076,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=238
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":11276,"parent_record_offset":11248,"ancestor_record_offset":11076,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=110
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":11188,"parent_record_offset":11160,"ancestor_record_offset":11076,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$3::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":430,"record_offset":40732,"parent_record_offset":40708,"ancestor_record_offset":40232,"inlinee":24892,"source_line":105,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=252
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":11076,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":409,"record_offset":24628,"parent_record_offset":24604,"ancestor_record_offset":24092,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":409,"record_offset":24580,"parent_record_offset":24548,"ancestor_record_offset":24092,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":409,"record_offset":24604,"parent_record_offset":24580,"ancestor_record_offset":24092,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=128
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":391,"record_offset":6812,"parent_record_offset":6784,"ancestor_record_offset":6732,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=260
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":364,"record_offset":11700,"parent_record_offset":11664,"ancestor_record_offset":11076,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=83
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":11076,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":399,"record_offset":33408,"parent_record_offset":33384,"ancestor_record_offset":32988,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=128
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":391,"record_offset":6844,"parent_record_offset":6812,"ancestor_record_offset":6732,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: float_grid.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::float_grid::FloatGrid
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: float_grid.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-production-authority/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/utils/src/float_grid.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-production-authority/three-version-path-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/utils/src/float_grid.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/utils/src/float_grid.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/utils/src/float_grid.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: float_grid.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 30; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::float_grid::FloatGrid::load_unchecked
// 信息 ID: 8c4ccc80d80bf7e3d0e0a3d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 388 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11664; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11740; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11824; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11892; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13700; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13776; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13860; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13928; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz
// 信息 ID: f256ad8a597bdb457d61570f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 97 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11076; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13112; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=19060; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=20240; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=21052; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::MainCamera::init_walk_camera; module=391, inline_record=6732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::garden_audio::garden_audio_system; module=392, inline_record=10720; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::garden_audio::garden_audio_system; module=392, inline_record=11840; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
