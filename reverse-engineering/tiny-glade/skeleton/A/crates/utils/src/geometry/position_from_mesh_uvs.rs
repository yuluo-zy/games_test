// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/geometry/position_from_mesh_uvs.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ee22037edc03103b3bd116c53ef39995b4756b06303a54f48583f2dcedf957b1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: position_from_mesh_uvs.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1343
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1343,"record_offset":244,"rva":15588224,"va":5384297344}
// 编译实例 RVAs: 0xeddb80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18681573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384297344,"raw_hex":"e8e50e1d01","size":5,"soft_fail":false,"target_va":5402979344,"va":5384297766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1343,"record_offset":1096,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1343,"record_offset":908,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::position_from_mesh_uvs::get_triangle_coords_from_mesh_uv
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/position_from_mesh_uvs.rs
//     evidence={"module":1343,"record_offset":404,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":62400,"source_line":25,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1343,"record_offset":840,"parent_record_offset":808,"ancestor_record_offset":244,"inlinee":17312,"source_line":1519,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1343,"record_offset":652,"parent_record_offset":404,"ancestor_record_offset":244,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1343,"record_offset":940,"parent_record_offset":908,"ancestor_record_offset":244,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1343,"record_offset":1128,"parent_record_offset":1096,"ancestor_record_offset":244,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1343,"record_offset":524,"parent_record_offset":496,"ancestor_record_offset":244,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1343,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":244,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1343,"record_offset":716,"parent_record_offset":680,"ancestor_record_offset":244,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1343,"record_offset":744,"parent_record_offset":680,"ancestor_record_offset":244,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1343,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":244,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/bvec3.rs
//     evidence={"module":1343,"record_offset":876,"parent_record_offset":404,"ancestor_record_offset":244,"inlinee":25013,"source_line":66,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1343,"record_offset":496,"parent_record_offset":448,"ancestor_record_offset":244,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1343,"record_offset":808,"parent_record_offset":404,"ancestor_record_offset":244,"inlinee":62403,"source_line":322,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::barycentric::barycentric2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/barycentric.rs
//     evidence={"module":1343,"record_offset":448,"parent_record_offset":404,"ancestor_record_offset":244,"inlinee":62402,"source_line":27,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::position_from_mesh_uvs::get_triangle_coords_from_mesh_uv
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF3C0
// 首条位置: {"module":1343,"inlinee":62400}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1343,"record_offset":652,"parent_record_offset":404,"ancestor_record_offset":404,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/bvec3.rs
//     evidence={"module":1343,"record_offset":876,"parent_record_offset":404,"ancestor_record_offset":404,"inlinee":25013,"source_line":66,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1343,"record_offset":808,"parent_record_offset":404,"ancestor_record_offset":404,"inlinee":62403,"source_line":322,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::barycentric::barycentric2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/barycentric.rs
//     evidence={"module":1343,"record_offset":448,"parent_record_offset":404,"ancestor_record_offset":404,"inlinee":62402,"source_line":27,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1343,"record_offset":840,"parent_record_offset":808,"ancestor_record_offset":404,"inlinee":17312,"source_line":1519,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1343,"record_offset":524,"parent_record_offset":496,"ancestor_record_offset":404,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1343,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":404,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1343,"record_offset":716,"parent_record_offset":680,"ancestor_record_offset":404,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1343,"record_offset":744,"parent_record_offset":680,"ancestor_record_offset":404,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1343,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":404,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01343.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1343,"record_offset":496,"parent_record_offset":448,"ancestor_record_offset":404,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01343.jsonl.gz"}
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
// 完整结构及原始类型记录: position_from_mesh_uvs.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: position_from_mesh_uvs.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: position_from_mesh_uvs.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv
// 信息 ID: c5ec8d37724a5663cc23a05a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::get_position_from_mesh_uv @ 0x51a9b1 -> 0xeddb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_position_from_mesh_uv @ 0xce07af -> 0xeddb80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::position_from_mesh_uvs::get_triangle_coords_from_mesh_uv
// 信息 ID: 520eabf8643c22c973fe5556
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv; module=1343, inline_record=404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
