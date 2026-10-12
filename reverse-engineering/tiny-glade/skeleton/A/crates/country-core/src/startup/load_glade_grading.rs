// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/load_glade_grading.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6756ef98f4e25edb347117fc5e5242a6b04aff662ba517bd1fbb98cdf182d708
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_glade_grading.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1447
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_grading::load_glade_grading
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1447,"record_offset":11640,"rva":17858384,"va":5386567504}
// 编译实例 RVAs: 0x1107f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16935561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e87795fdfe","size":5,"soft_fail":false,"target_va":5369632448,"va":5386568004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f2020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14771112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e8589c1eff","size":5,"soft_fail":false,"target_va":5371797536,"va":5386568643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14253592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e8e88126ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5386567795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14248216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e8e89626ff","size":5,"soft_fail":false,"target_va":5372320512,"va":5386568723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13818034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e84e272dff","size":5,"soft_fail":false,"target_va":5372749696,"va":5386567725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13817977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e887272dff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386567684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11396554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e8361a52ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5386567605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d42aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3954163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e80daac3ff","size":5,"soft_fail":false,"target_va":5382613664,"va":5386567822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16410019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e8a365fa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386568120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16411953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e8316dfa00","size":5,"soft_fail":false,"target_va":5402980128,"va":5386568170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9162049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e8bf3274ff","size":5,"soft_fail":false,"target_va":5377405616,"va":5386567660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9127826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386567504,"raw_hex":"e86eb874ff","size":5,"soft_fail":false,"target_va":5377439776,"va":5386567597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1447,"record_offset":14020,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":13448,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1447,"record_offset":13476,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1447,"record_offset":14080,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":1447,"record_offset":12912,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":22117,"source_line":35,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":1447,"record_offset":14144,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":22119,"source_line":24,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1447,"record_offset":11824,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":21871,"source_line":200,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1447,"record_offset":12108,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1447,"record_offset":12076,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1447,"record_offset":11936,"parent_record_offset":11640,"ancestor_record_offset":11640,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":13232,"parent_record_offset":13208,"ancestor_record_offset":11640,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":11640,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":11640,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":13208,"parent_record_offset":13180,"ancestor_record_offset":11640,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":12524,"parent_record_offset":12468,"ancestor_record_offset":11640,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":12496,"parent_record_offset":12468,"ancestor_record_offset":11640,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":13180,"parent_record_offset":13152,"ancestor_record_offset":11640,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":13152,"parent_record_offset":13124,"ancestor_record_offset":11640,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":13328,"parent_record_offset":13152,"ancestor_record_offset":11640,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":12468,"parent_record_offset":12440,"ancestor_record_offset":11640,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1447,"record_offset":13088,"parent_record_offset":13060,"ancestor_record_offset":11640,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1447,"record_offset":13060,"parent_record_offset":13032,"ancestor_record_offset":11640,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1447,"record_offset":13004,"parent_record_offset":12976,"ancestor_record_offset":11640,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1447,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":11640,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1447,"record_offset":12976,"parent_record_offset":12948,"ancestor_record_offset":11640,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1447,"record_offset":12948,"parent_record_offset":12912,"ancestor_record_offset":11640,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1447,"record_offset":13124,"parent_record_offset":13088,"ancestor_record_offset":11640,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1447,"record_offset":14048,"parent_record_offset":14020,"ancestor_record_offset":11640,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1447,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":11640,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1447,"record_offset":13364,"parent_record_offset":13088,"ancestor_record_offset":11640,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":12308,"parent_record_offset":12108,"ancestor_record_offset":11640,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1447,"record_offset":14112,"parent_record_offset":14080,"ancestor_record_offset":11640,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":13284,"parent_record_offset":13256,"ancestor_record_offset":11640,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1447,"record_offset":11872,"parent_record_offset":11848,"ancestor_record_offset":11640,"inlinee":21875,"source_line":44,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1447,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":11640,"inlinee":21873,"source_line":153,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1447,"record_offset":11984,"parent_record_offset":11960,"ancestor_record_offset":11640,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1447,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":11640,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1447,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":11640,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1447,"record_offset":11900,"parent_record_offset":11848,"ancestor_record_offset":11640,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1447,"record_offset":11960,"parent_record_offset":11936,"ancestor_record_offset":11640,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1447,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11640,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1447,"record_offset":12140,"parent_record_offset":12108,"ancestor_record_offset":11640,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/01447.jsonl.gz"}
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
// 完整结构及原始类型记录: load_glade_grading.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_glade_grading.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_glade_grading.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_grading::load_glade_grading
// 信息 ID: 7d5ce68e97aef38740089c7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c1db00b8934df51f17603d18: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0xd8755f -> 0x1107f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0xe504d1 -> 0x1107f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
