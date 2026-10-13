// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/roofs/roof_visual_shape.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2c99000d50dd971a1f6459c4ea846d252580c5dc889a2363012305a3e5039b53
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_visual_shape.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 411, 492, 493
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":20324,"rva":7057792,"va":5375766912}
// 编译实例 RVAs: 0x6bb180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406bb250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_visual_shape.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375766912,"raw_hex":"e824000000","size":5,"soft_fail":false,"target_va":5375767120,"va":5375767079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_visual_shape.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1172849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375766912,"raw_hex":"e871e51100","size":5,"soft_fail":false,"target_va":5376939904,"va":5375767050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_noise::gpunoise::noise
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t40812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375766912,"raw_hex":"e86c9f0000","size":5,"soft_fail":false,"target_va":5375807792,"va":5375766975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375766912,"raw_hex":"e801faffff","size":5,"soft_fail":false,"target_va":5375765488,"va":5375767018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":493,"record_offset":20716,"parent_record_offset":20324,"ancestor_record_offset":20324,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::get_height_01_unclamped_from_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_visual_shape.rs
//     evidence={"module":493,"record_offset":20496,"parent_record_offset":20324,"ancestor_record_offset":20324,"inlinee":27686,"source_line":39,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":20592,"parent_record_offset":20568,"ancestor_record_offset":20324,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_noise::revert_roof_noise
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":493,"record_offset":20540,"parent_record_offset":20496,"ancestor_record_offset":20324,"inlinee":27049,"source_line":12,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":493,"record_offset":20656,"parent_record_offset":20496,"ancestor_record_offset":20324,"inlinee":22264,"source_line":111,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":493,"record_offset":20680,"parent_record_offset":20656,"ancestor_record_offset":20324,"inlinee":22265,"source_line":102,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":493,"record_offset":20568,"parent_record_offset":20540,"ancestor_record_offset":20324,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_shape::Roof::get_height_01_unclamped_from_ws
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C26
// 首条位置: {"module":493,"inlinee":27686}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::revert_roof_noise
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":493,"record_offset":20540,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":27049,"source_line":12,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":493,"record_offset":20656,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":22264,"source_line":111,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":20592,"parent_record_offset":20568,"ancestor_record_offset":20496,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":493,"record_offset":20680,"parent_record_offset":20656,"ancestor_record_offset":20496,"inlinee":22265,"source_line":102,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":493,"record_offset":20568,"parent_record_offset":20540,"ancestor_record_offset":20496,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_shape::Roof::get_roof_shape
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":20752,"rva":7058000,"va":5375767120}
// 编译实例 RVAs: 0x6bb250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: roof_visual_shape.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27827033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767120,"raw_hex":"e8599ba801","size":5,"soft_fail":false,"target_va":5403594240,"va":5375767202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_visual_shape.rs
//     evidence={"module":493,"record_offset":21060,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":27650,"source_line":78,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":493,"record_offset":20908,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":493,"record_offset":21032,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":20956,"parent_record_offset":20908,"ancestor_record_offset":20752,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":21004,"parent_record_offset":20908,"ancestor_record_offset":20752,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":20932,"parent_record_offset":20908,"ancestor_record_offset":20752,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":493,"record_offset":21152,"parent_record_offset":21116,"ancestor_record_offset":20752,"inlinee":22269,"source_line":42,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":493,"record_offset":21116,"parent_record_offset":21060,"ancestor_record_offset":20752,"inlinee":22267,"source_line":31,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::RoofShapeParams::eave_length_ws
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":493,"record_offset":21340,"parent_record_offset":21060,"ancestor_record_offset":20752,"inlinee":27633,"source_line":335,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":493,"record_offset":21196,"parent_record_offset":21152,"ancestor_record_offset":20752,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":493,"record_offset":21428,"parent_record_offset":21340,"ancestor_record_offset":20752,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C02
// 首条位置: {"module":492,"record_offset":15268,"rva":7021024,"va":5375730144}
// 编译实例 RVAs: 0x6b21e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":492,"record_offset":15440,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":22267,"source_line":31,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::RoofShapeParams::eave_length_ws
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":492,"record_offset":15652,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":27633,"source_line":335,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":492,"record_offset":15568,"parent_record_offset":15520,"ancestor_record_offset":15268,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":492,"record_offset":15616,"parent_record_offset":15520,"ancestor_record_offset":15268,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":492,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":15268,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":492,"record_offset":15476,"parent_record_offset":15440,"ancestor_record_offset":15268,"inlinee":22269,"source_line":42,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":492,"record_offset":15520,"parent_record_offset":15476,"ancestor_record_offset":15268,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":492,"record_offset":15816,"parent_record_offset":15652,"ancestor_record_offset":15268,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":492,"record_offset":16060,"rva":7021344,"va":5375730464}
// 编译实例 RVAs: 0x6b2320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27248640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730464,"raw_hex":"e800c89f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5375730507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_rectangle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C23
// 首条位置: {"module":411,"inlinee":23587}
// 编译实例 RVAs: 0x6b2360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27248576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730528,"raw_hex":"e8c0c79f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5375730571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: roof_visual_shape.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::roofs::roof_visual_shape::RoofBoundsDims
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_visual_shape.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_visual_shape.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>)
// 候选参数类型: ["bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_shape::Roof::get_height_01_unclamped_from_ws
// 信息 ID: 4f96597218118b43cd97b82b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped; module=493, inline_record=20496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_shape::Roof::get_roof_shape
// 信息 ID: e8759afa09bb947408c275a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped @ 0x6bb227 -> 0x6bb250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccfaeb -> 0x6bb250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccfc79 -> 0x6bb250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd1f1d -> 0x6bb250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped
// 信息 ID: b3577a480c9fd28d9ee64d42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd370 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf48f4 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0xcf6a42 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0xcf6c3f -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17386 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17442 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd177e3 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17859 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a5cd1 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a5f4a -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a6525 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a65ca -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x17d5804 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x17d586a -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x17d9ba8 -> 0x6bb180 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x17d9c1b -> 0x6bb180 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_circle
// 信息 ID: d4b099ce3befc0bd648b5492
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e04a1 -> 0x6b2320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x19839cc -> 0x6b2320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_rectangle
// 信息 ID: 35b3605f723132cdf5df0641
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dbe69 -> 0x6b2360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xcccfbc -> 0x6b2360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x198388a -> 0x6b2360 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=27796; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
// 信息 ID: d47bc63125433ec6aa1e57dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x57208a -> 0x6b21e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dbe5a -> 0x6b21e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e0499 -> 0x6b21e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xcccfa9 -> 0x6b21e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x1983862 -> 0x6b21e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x19839ab -> 0x6b21e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape; module=493, inline_record=21060; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
