// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 176c7ffd2db632fc0295d5993335bac196afe317a76e17a78aa58795a80c4a16
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: billboard_forest.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 435
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":435,"record_offset":7248,"rva":6113376,"va":5374822496}
// 编译实例 RVAs: 0x5d4860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t8222520","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374822496,"raw_hex":"e938777d00","size":5,"soft_fail":false,"target_va":5383045024,"va":5374822499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":435,"record_offset":7428,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::startup_billboard_forest
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":435,"record_offset":7460,"rva":6113392,"va":5374822512}
// 编译实例 RVAs: 0x5d4870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2197632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e88077deff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374822651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2073060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e81c5ee0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374822751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2073444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e89c5ce0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374823151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t908513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e8e1dc0d00","size":5,"soft_fail":false,"target_va":5375731504,"va":5374822986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1165215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e89fc71100","size":5,"soft_fail":false,"target_va":5375988432,"va":5374823212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7997563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e87b087a00","size":5,"soft_fail":false,"target_va":5382820192,"va":5374822624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28155050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e8aa9cad01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374823089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9552890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e8fac39100","size":5,"soft_fail":false,"target_va":5384375952,"va":5374823057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-256848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e8b014fcff","size":5,"soft_fail":false,"target_va":5374566080,"va":5374822923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2543590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374822512,"raw_hex":"e8e6cf2600","size":5,"soft_fail":false,"target_va":5377366224,"va":5374822629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":435,"record_offset":7672,"parent_record_offset":7460,"ancestor_record_offset":7460,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":435,"record_offset":8940,"parent_record_offset":7460,"ancestor_record_offset":7460,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":435,"record_offset":8856,"parent_record_offset":7460,"ancestor_record_offset":7460,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":435,"record_offset":7756,"parent_record_offset":7460,"ancestor_record_offset":7460,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":435,"record_offset":8212,"parent_record_offset":7460,"ancestor_record_offset":7460,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":435,"record_offset":8912,"parent_record_offset":7460,"ancestor_record_offset":7460,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":7460,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":8656,"parent_record_offset":8632,"ancestor_record_offset":7460,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":7460,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":8608,"parent_record_offset":8580,"ancestor_record_offset":7460,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":9092,"parent_record_offset":8992,"ancestor_record_offset":7460,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":435,"record_offset":9024,"parent_record_offset":8992,"ancestor_record_offset":7460,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":435,"record_offset":8580,"parent_record_offset":8552,"ancestor_record_offset":7460,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":435,"record_offset":8552,"parent_record_offset":8524,"ancestor_record_offset":7460,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":435,"record_offset":8728,"parent_record_offset":8552,"ancestor_record_offset":7460,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":435,"record_offset":8992,"parent_record_offset":8964,"ancestor_record_offset":7460,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":435,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":7460,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":435,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":7460,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":435,"record_offset":8404,"parent_record_offset":8376,"ancestor_record_offset":7460,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":435,"record_offset":8432,"parent_record_offset":8404,"ancestor_record_offset":7460,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":435,"record_offset":8376,"parent_record_offset":8348,"ancestor_record_offset":7460,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":435,"record_offset":8348,"parent_record_offset":8320,"ancestor_record_offset":7460,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":435,"record_offset":8320,"parent_record_offset":8276,"ancestor_record_offset":7460,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":435,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":7460,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":435,"record_offset":8524,"parent_record_offset":8488,"ancestor_record_offset":7460,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":435,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":7460,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":435,"record_offset":8044,"parent_record_offset":8016,"ancestor_record_offset":7460,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":435,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7460,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":435,"record_offset":8072,"parent_record_offset":8016,"ancestor_record_offset":7460,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":435,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":7460,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":435,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":7460,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":435,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":7460,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":435,"record_offset":9056,"parent_record_offset":9024,"ancestor_record_offset":7460,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":435,"record_offset":7976,"parent_record_offset":7952,"ancestor_record_offset":7460,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":435,"record_offset":8764,"parent_record_offset":8488,"ancestor_record_offset":7460,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":435,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":7460,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":435,"record_offset":8684,"parent_record_offset":8656,"ancestor_record_offset":7460,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":435,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":7460,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":435,"record_offset":8276,"parent_record_offset":8212,"ancestor_record_offset":7460,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":435,"record_offset":8248,"parent_record_offset":8212,"ancestor_record_offset":7460,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":435,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":7460,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":435,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":7460,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":435,"record_offset":7904,"parent_record_offset":7876,"ancestor_record_offset":7460,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":435,"record_offset":8016,"parent_record_offset":7876,"ancestor_record_offset":7460,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":435,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":7460,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":435,"record_offset":7928,"parent_record_offset":7904,"ancestor_record_offset":7460,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":435,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":7460,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":435,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":7460,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":435,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7460,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00435.jsonl.gz"}
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
// 完整结构及原始类型记录: billboard_forest.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "feedback_reporter::FeedbackReporterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::menu_background::MenuBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::paths::PathPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_water::WaterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::garden::GardenPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_tree::TreePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::prefab::PrefabPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::photo_mode::PhotoModePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: billboard_forest.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: billboard_forest.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// [Shader 映射] {"shader_id":"3b002a2b2051c05b59b05e63","relative_path":"_billboard_forest.raster.hlsl_b903e43ffb3da915.bin","sha256":"a4c451bbf567a369d411f9555e839736fd9084108589bce9a4753b171eefeeef","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"billboard_forest","prefix_candidates":["billboard_forest"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"44695d4c7b3152022af7ab1f","relative_path":"_billboard_forest.raster.hlsl_f0adff76827e73ff.bin","sha256":"426ad6878e928f878686f97d615ac7093a0e3b46bb7b98840b0d9643b7eb5642","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"billboard_forest","prefix_candidates":["billboard_forest"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _billboard_forest.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"frame_constants","storage_class":"Uniform","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[39,39,39,39,39,39,39,39,22,20,20,22,30,22,20,24,20,24,22,24,22,39,16]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":39,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":39,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":39,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":22,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":20,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":20,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":22,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":30,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":22,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":20,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":24,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":20,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":24,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":22,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":24,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":22,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":39,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":16,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[22,30,30,30,30,30]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"framebuffer_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"cloud1_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"cloud2_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"cloud3_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"cloud4_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,22]},"decorations":[],"pointee":{"id":22,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,22]},"decorations":[],"pointee":{"id":22,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,39]},"decorations":[],"pointee":{"id":39,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.C0","storage_class":"Output","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,21]},"decorations":[],"pointee":{"id":21,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C1","storage_class":"Output","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,22]},"decorations":[],"pointee":{"id":22,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Flat","operands":[]},{"decoration":"Location","operands":[1]}],"access_qualifiers":["Flat","Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C2","storage_class":"Output","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C3","storage_class":"Output","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C4","storage_class":"Output","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":4,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[4]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5,6,7]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,8]},"decorations":[],"pointee":{"id":8,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[36,36,36,36,36,36,36,36,27,25,25,27,33,27,25,28,25,28,27,28,27,36,21]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":36,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":36,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":36,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":36,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":36,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":36,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":36,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":36,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":27,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":25,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":25,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":27,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":33,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":27,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":25,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":28,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":25,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":28,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":27,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":28,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":27,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":36,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":21,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"cloud1","storage_class":"UniformConstant","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"cloud2","storage_class":"UniformConstant","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"cloud3","storage_class":"UniformConstant","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"cloud4","storage_class":"UniformConstant","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"color_library","storage_class":"Uniform","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,18]},"decorations":[],"pointee":{"id":18,"name":"type.ConstantBuffer.ColorLibrary","record":{"opcode":"OpTypeStruct","operands":[36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":36,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":36,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":36,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":36,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":36,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":36,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":36,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":36,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":36,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":36,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":36,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":36,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":36,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":36,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":36,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":36,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":36,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":36,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":36,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,26]},"decorations":[],"pointee":{"id":26,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,27]},"decorations":[],"pointee":{"id":27,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Flat","operands":[]},{"decoration":"Location","operands":[1]}],"access_qualifiers":["Flat","Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.C3","storage_class":"Input","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,28]},"decorations":[],"pointee":{"id":28,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"in.var.C4","storage_class":"Input","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,28]},"decorations":[],"pointee":{"id":28,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":4,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[4]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,33]},"decorations":[],"pointee":{"id":33,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[27,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _billboard_forest.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"frame_constants","storage_class":"Uniform","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[39,39,39,39,39,39,39,39,22,20,20,22,30,22,20,24,20,24,22,24,22,39,16]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":39,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":39,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":39,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":22,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":20,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":20,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":22,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":30,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":22,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":20,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":24,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":20,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":24,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":22,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":24,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":22,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":39,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":16,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[22,30,30,30,30,30]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"framebuffer_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"cloud1_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"cloud2_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"cloud3_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"cloud4_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,22]},"decorations":[],"pointee":{"id":22,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,22]},"decorations":[],"pointee":{"id":22,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,39]},"decorations":[],"pointee":{"id":39,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.C0","storage_class":"Output","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,21]},"decorations":[],"pointee":{"id":21,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C1","storage_class":"Output","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,22]},"decorations":[],"pointee":{"id":22,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Flat","operands":[]},{"decoration":"Location","operands":[1]}],"access_qualifiers":["Flat","Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C2","storage_class":"Output","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C3","storage_class":"Output","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C4","storage_class":"Output","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":4,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[4]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":25,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"cloud1","storage_class":"UniformConstant","type":{"id":26,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"cloud2","storage_class":"UniformConstant","type":{"id":26,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"cloud3","storage_class":"UniformConstant","type":{"id":26,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"cloud4","storage_class":"UniformConstant","type":{"id":26,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":27,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,17]},"decorations":[],"pointee":{"id":17,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[16,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":28,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,18]},"decorations":[],"pointee":{"id":18,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Flat","operands":[]},{"decoration":"Location","operands":[1]}],"access_qualifiers":["Flat","Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":14988912,"caller_aliases":{"alias_group_rva":14988912,"count":1,"display_examples":["bevy_ecs::system::system::System::run"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":14989232,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":15814400,"caller_aliases":{"alias_group_rva":15814400,"count":1,"display_examples":["bevy_ecs::system::function_system::impl$7::run_unsafe"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":15814705,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: billboard_forest.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::impl$0::build
// 信息 ID: 98368c52b305692c68be95f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::startup_billboard_forest
// 信息 ID: e2c2af00707850a7824b2d81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cc1c15e21a217d5b987dc287: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > @ 0xe4b7b0 -> 0x5d4870 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> @ 0xf15031 -> 0x5d4870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
