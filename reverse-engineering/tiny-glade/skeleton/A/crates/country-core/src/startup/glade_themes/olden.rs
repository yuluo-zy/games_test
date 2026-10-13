// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/glade_themes/olden.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 16cb0b53522eb48d7f5cd8c4a7704c08c552e2673fef3abba2a14319043a1c76
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: olden.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1403
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::glade_themes::olden::startup_olden
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1403,"record_offset":4112,"rva":17053712,"va":5385762832}
// 编译实例 RVAs: 0x1043810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13014540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e8f46939ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5385764231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13014307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e8dd6a39ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385764014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b6e80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11062914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e87e3157ff","size":5,"soft_fail":false,"target_va":5374701184,"va":5385764093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10592402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e86e5f5eff","size":5,"soft_fail":false,"target_va":5375171056,"va":5385763453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9777348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e83ccf6aff","size":5,"soft_fail":false,"target_va":5375988432,"va":5385765775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141086720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t273521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e8712c0400","size":5,"soft_fail":false,"target_va":5386037024,"va":5385763498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17830344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e8c8111001","size":5,"soft_fail":false,"target_va":5403594000,"va":5385763651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17830692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e824131001","size":5,"soft_fail":false,"target_va":5403594336,"va":5385763639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17212832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e8a0a50601","size":5,"soft_fail":false,"target_va":5402978144,"va":5385765307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3898026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e85685c4ff","size":5,"soft_fail":false,"target_va":5381865936,"va":5385763957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8402580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e86cc97fff","size":5,"soft_fail":false,"target_va":5377364336,"va":5385766911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1382252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e894e8eaff","size":5,"soft_fail":false,"target_va":5384382368,"va":5385764615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17213554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e872a80601","size":5,"soft_fail":false,"target_va":5402978880,"va":5385765321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17214833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e871ad0601","size":5,"soft_fail":false,"target_va":5402980128,"va":5385765290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11198353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e86f2055ff","size":5,"soft_fail":false,"target_va":5374566080,"va":5385764428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10072045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e8135066ff","size":5,"soft_fail":false,"target_va":5375692416,"va":5385764456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8323674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e8a6fd80ff","size":5,"soft_fail":false,"target_va":5377439776,"va":5385763445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7810762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385762832,"raw_hex":"e836d188ff","size":5,"soft_fail":false,"target_va":5377953168,"va":5385763925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1403,"record_offset":4912,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1403,"record_offset":4344,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1403,"record_offset":7852,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1403,"record_offset":6300,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1403,"record_offset":7136,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1403,"record_offset":4832,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1403,"record_offset":5052,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":23061,"source_line":73,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::preplaced_clutter::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/preplaced_clutter.rs
//     evidence={"module":1403,"record_offset":4316,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":65026,"source_line":5,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1403,"record_offset":4552,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":21871,"source_line":200,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":1403,"record_offset":7208,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":1403,"record_offset":5784,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":15693,"source_line":1157,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1403,"record_offset":6080,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1403,"record_offset":4804,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1403,"record_offset":4664,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1403,"record_offset":5268,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":27549,"source_line":23,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1403,"record_offset":7888,"parent_record_offset":4112,"ancestor_record_offset":4112,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1403,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":4112,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1403,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":4112,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1403,"record_offset":6488,"parent_record_offset":6464,"ancestor_record_offset":4112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1403,"record_offset":7604,"parent_record_offset":7576,"ancestor_record_offset":4112,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1403,"record_offset":6464,"parent_record_offset":6372,"ancestor_record_offset":4112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1403,"record_offset":6400,"parent_record_offset":6372,"ancestor_record_offset":4112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1403,"record_offset":4988,"parent_record_offset":4956,"ancestor_record_offset":4112,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1403,"record_offset":7576,"parent_record_offset":7548,"ancestor_record_offset":4112,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1403,"record_offset":7548,"parent_record_offset":7520,"ancestor_record_offset":4112,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1403,"record_offset":7724,"parent_record_offset":7548,"ancestor_record_offset":4112,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1403,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":4112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1403,"record_offset":7484,"parent_record_offset":7456,"ancestor_record_offset":4112,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1403,"record_offset":7456,"parent_record_offset":7428,"ancestor_record_offset":4112,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1403,"record_offset":7400,"parent_record_offset":7372,"ancestor_record_offset":4112,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1403,"record_offset":7428,"parent_record_offset":7400,"ancestor_record_offset":4112,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1403,"record_offset":7372,"parent_record_offset":7344,"ancestor_record_offset":4112,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1403,"record_offset":7344,"parent_record_offset":7316,"ancestor_record_offset":4112,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1403,"record_offset":7316,"parent_record_offset":7272,"ancestor_record_offset":4112,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1403,"record_offset":4956,"parent_record_offset":4912,"ancestor_record_offset":4112,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1403,"record_offset":7520,"parent_record_offset":7484,"ancestor_record_offset":4112,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1403,"record_offset":4432,"parent_record_offset":4368,"ancestor_record_offset":4112,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1403,"record_offset":5868,"parent_record_offset":5812,"ancestor_record_offset":4112,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1403,"record_offset":5520,"parent_record_offset":5468,"ancestor_record_offset":4112,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1403,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":4112,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1403,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":4112,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1403,"record_offset":6428,"parent_record_offset":6400,"ancestor_record_offset":4112,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1403,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4112,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1403,"record_offset":4404,"parent_record_offset":4368,"ancestor_record_offset":4112,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1403,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":4112,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1403,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":4112,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1403,"record_offset":7760,"parent_record_offset":7484,"ancestor_record_offset":4112,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1403,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":4112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1403,"record_offset":4876,"parent_record_offset":4832,"ancestor_record_offset":4112,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1403,"record_offset":7680,"parent_record_offset":7652,"ancestor_record_offset":4112,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1403,"record_offset":5020,"parent_record_offset":4912,"ancestor_record_offset":4112,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1403,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":4112,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1403,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":4112,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1403,"record_offset":7272,"parent_record_offset":7208,"ancestor_record_offset":4112,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1403,"record_offset":7244,"parent_record_offset":7208,"ancestor_record_offset":4112,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1403,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":4112,"inlinee":21875,"source_line":44,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1403,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":4112,"inlinee":21873,"source_line":153,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1403,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":4112,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":1403,"record_offset":5296,"parent_record_offset":5268,"ancestor_record_offset":4112,"inlinee":27550,"source_line":215,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":1403,"record_offset":5332,"parent_record_offset":5296,"ancestor_record_offset":4112,"inlinee":27551,"source_line":181,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1403,"record_offset":5088,"parent_record_offset":5052,"ancestor_record_offset":4112,"inlinee":14670,"source_line":137,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1403,"record_offset":5380,"parent_record_offset":5332,"ancestor_record_offset":4112,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1403,"record_offset":5468,"parent_record_offset":5296,"ancestor_record_offset":4112,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$47::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1403,"record_offset":5720,"parent_record_offset":5296,"ancestor_record_offset":4112,"inlinee":27557,"source_line":1300,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$8::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1403,"record_offset":5812,"parent_record_offset":5784,"ancestor_record_offset":4112,"inlinee":15703,"source_line":960,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1403,"record_offset":6168,"parent_record_offset":6136,"ancestor_record_offset":4112,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1403,"record_offset":6136,"parent_record_offset":6104,"ancestor_record_offset":4112,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1403,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":4112,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1403,"record_offset":5204,"parent_record_offset":5088,"ancestor_record_offset":4112,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1403,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":4112,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1403,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4112,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1403,"record_offset":6200,"parent_record_offset":6168,"ancestor_record_offset":4112,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1403,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":4112,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1403,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":4112,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1403,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":4112,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1403,"record_offset":4628,"parent_record_offset":4576,"ancestor_record_offset":4112,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1403,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":4112,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01403.jsonl.gz"}
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
// 完整结构及原始类型记录: olden.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: olden.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: olden.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::startup::glade_themes::olden::startup_olden
// 信息 ID: 0c49511c065f6c241e443368
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0x6c888e -> 0x1043810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
