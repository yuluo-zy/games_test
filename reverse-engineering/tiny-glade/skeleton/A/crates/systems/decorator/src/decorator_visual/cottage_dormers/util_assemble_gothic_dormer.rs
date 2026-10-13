// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/cottage_dormers/util_assemble_gothic_dormer.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1e4fa1386f310e1417cc6f024085ff702df7d2359a3d34663f7bb29453908e65
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: util_assemble_gothic_dormer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3843
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"PDB_registry_path_only","version":"1.8.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6>
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3843,"record_offset":256,"rva":29626256,"va":5398335376}
// 编译实例 RVAs: 0x1c40f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25587283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8ad9179fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398336990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21391687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8b996b9fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398335682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf2c40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16051742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8e2110bff","size":5,"soft_fail":false,"target_va":5382286400,"va":5398338137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e7af0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10852776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e858665aff","size":5,"soft_fail":false,"target_va":5387483888,"va":5398336659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c8de0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9930365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8837968ff","size":5,"soft_fail":false,"target_va":5388406240,"va":5398336600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e7670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4562756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8bc60baff","size":5,"soft_fail":false,"target_va":5393774192,"va":5398336943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5256949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8f5365000","size":5,"soft_fail":false,"target_va":5403593968,"va":5398337014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5255019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e86b2f5000","size":5,"soft_fail":false,"target_va":5403594000,"va":5398338976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5258323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8533c5000","size":5,"soft_fail":false,"target_va":5403594288,"va":5398335960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5255389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8dd305000","size":5,"soft_fail":false,"target_va":5403594336,"va":5398338942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4638767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e82fc84600","size":5,"soft_fail":false,"target_va":5402978880,"va":5398340108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4639305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e849ca4600","size":5,"soft_fail":false,"target_va":5402979152,"va":5398339842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4638955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8ebc84600","size":5,"soft_fail":false,"target_va":5402978944,"va":5398339984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22570823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8b998a7fe","size":5,"soft_fail":false,"target_va":5375764720,"va":5398335538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23973133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8f33292fe","size":5,"soft_fail":false,"target_va":5374362576,"va":5398335704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12488834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e87e6f41ff","size":5,"soft_fail":false,"target_va":5385850032,"va":5398338861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::closest_point_on_perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20366720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8803ac9fe","size":5,"soft_fail":false,"target_va":5377968928,"va":5398335643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20374105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e8a71dc9fe","size":5,"soft_fail":false,"target_va":5377961504,"va":5398335604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21396457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398335376,"raw_hex":"e81784b9fe","size":5,"soft_fail":false,"target_va":5376939616,"va":5398336068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3843,"record_offset":1320,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3843,"record_offset":4384,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":22232,"source_line":903,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3843,"record_offset":2200,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3843,"record_offset":696,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3843,"record_offset":1964,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3843,"record_offset":668,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3843,"record_offset":1464,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":60316,"source_line":237,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3843,"record_offset":4412,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14670,"source_line":137,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3843,"record_offset":3280,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3843,"record_offset":3652,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":59541,"source_line":846,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":3248,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":912,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":820,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":3012,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":1204,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":2920,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":732,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":3843,"record_offset":1380,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3843,"record_offset":1348,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3843,"record_offset":4244,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3843,"record_offset":2172,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3843,"record_offset":1292,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure$6
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs
//     evidence={"module":3843,"record_offset":1936,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":111761,"source_line":330,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3843,"record_offset":2292,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3843,"record_offset":2500,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3843,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3843,"record_offset":2104,"parent_record_offset":2012,"ancestor_record_offset":256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3843,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3843,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3843,"record_offset":1584,"parent_record_offset":1552,"ancestor_record_offset":256,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3843,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":256,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3843,"record_offset":1620,"parent_record_offset":1552,"ancestor_record_offset":256,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3843,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":256,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3843,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":256,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3843,"record_offset":3928,"parent_record_offset":3680,"ancestor_record_offset":256,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3843,"record_offset":3788,"parent_record_offset":3680,"ancestor_record_offset":256,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3843,"record_offset":3760,"parent_record_offset":3680,"ancestor_record_offset":256,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3843,"record_offset":2652,"parent_record_offset":2500,"ancestor_record_offset":256,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3843,"record_offset":2680,"parent_record_offset":2500,"ancestor_record_offset":256,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3843,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":256,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3843,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":3843,"record_offset":1900,"parent_record_offset":1464,"ancestor_record_offset":256,"inlinee":7987,"source_line":1749,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3843,"record_offset":1740,"parent_record_offset":1488,"ancestor_record_offset":256,"inlinee":27853,"source_line":1703,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3843,"record_offset":1824,"parent_record_offset":1796,"ancestor_record_offset":256,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3843,"record_offset":760,"parent_record_offset":732,"ancestor_record_offset":256,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3843,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":256,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3843,"record_offset":980,"parent_record_offset":940,"ancestor_record_offset":256,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3843,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":256,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3843,"record_offset":2540,"parent_record_offset":2500,"ancestor_record_offset":256,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3843,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3843,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3843,"record_offset":2256,"parent_record_offset":2224,"ancestor_record_offset":256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3843,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3843,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":256,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3843,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3843,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":256,"inlinee":60166,"source_line":143,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3843,"record_offset":1528,"parent_record_offset":1488,"ancestor_record_offset":256,"inlinee":60167,"source_line":122,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3843,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":256,"inlinee":26812,"source_line":128,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3843,"record_offset":4448,"parent_record_offset":4412,"ancestor_record_offset":256,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3843,"record_offset":3420,"parent_record_offset":3304,"ancestor_record_offset":256,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3843,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":256,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3843,"record_offset":4604,"parent_record_offset":4412,"ancestor_record_offset":256,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3843,"record_offset":3680,"parent_record_offset":3652,"ancestor_record_offset":256,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":256,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":3076,"parent_record_offset":3040,"ancestor_record_offset":256,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":256,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":940,"parent_record_offset":912,"ancestor_record_offset":256,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3843,"record_offset":4548,"parent_record_offset":4412,"ancestor_record_offset":256,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3843,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":256,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3843,"record_offset":4504,"parent_record_offset":4476,"ancestor_record_offset":256,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3843,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":256,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/03843.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3843,"record_offset":876,"parent_record_offset":844,"ancestor_record_offset":256,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03843.jsonl.gz"}
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
// 完整结构及原始类型记录: util_assemble_gothic_dormer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: util_assemble_gothic_dormer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: util_assemble_gothic_dormer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6>
// 信息 ID: 112a3debfac197c25122e293
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles -> ["system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x17aa643 -> 0x1c40f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
