// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 946e2a4d2384a64ef81105605a05ca7fe863c22e50f08926359b7b4f52dd79e7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_draw.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 485
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":47044,"rva":6906960,"va":5375616080}
// 编译实例 RVAs: 0x696450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1399496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e838a5eaff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375616291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1398029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e8f3aaeaff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375616440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t350676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e8d4590500","size":5,"soft_fail":false,"target_va":5375967552,"va":5375616871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7733069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e84dff7500","size":5,"soft_fail":false,"target_va":5383349424,"va":5375616350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7733430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e8b6007600","size":5,"soft_fail":false,"target_va":5383349824,"va":5375616389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df9ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7748357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e8053b7600","size":5,"soft_fail":false,"target_va":5383364592,"va":5375616230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27977157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e8c5e5aa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375616198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27362053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e80583a101","size":5,"soft_fail":false,"target_va":5402978880,"va":5375616822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27362538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e8ea84a101","size":5,"soft_fail":false,"target_va":5402979344,"va":5375616801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t91088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375616080,"raw_hex":"e8d0630100","size":5,"soft_fail":false,"target_va":5375707584,"va":5375616491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":485,"record_offset":47228,"parent_record_offset":47044,"ancestor_record_offset":47044,"inlinee":27353,"source_line":547,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":47404,"parent_record_offset":47044,"ancestor_record_offset":47044,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":485,"record_offset":49272,"parent_record_offset":47044,"ancestor_record_offset":47044,"inlinee":22561,"source_line":345,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":485,"record_offset":49184,"parent_record_offset":47044,"ancestor_record_offset":47044,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":48640,"parent_record_offset":48600,"ancestor_record_offset":47044,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":485,"record_offset":47544,"parent_record_offset":47508,"ancestor_record_offset":47044,"inlinee":22529,"source_line":713,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":485,"record_offset":47252,"parent_record_offset":47228,"ancestor_record_offset":47044,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":49104,"parent_record_offset":49080,"ancestor_record_offset":47044,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":48764,"parent_record_offset":48736,"ancestor_record_offset":47044,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":48736,"parent_record_offset":48600,"ancestor_record_offset":47044,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":48856,"parent_record_offset":48792,"ancestor_record_offset":47044,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":48944,"parent_record_offset":48920,"ancestor_record_offset":47044,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":485,"record_offset":49080,"parent_record_offset":48528,"ancestor_record_offset":47044,"inlinee":22535,"source_line":383,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$7::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":485,"record_offset":48600,"parent_record_offset":48568,"ancestor_record_offset":47044,"inlinee":22534,"source_line":904,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::entity_ref::EntityRef::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/entity_ref.rs
//     evidence={"module":485,"record_offset":47476,"parent_record_offset":47444,"ancestor_record_offset":47044,"inlinee":27357,"source_line":111,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":47508,"parent_record_offset":47476,"ancestor_record_offset":47044,"inlinee":22528,"source_line":691,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::fetch_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":48568,"parent_record_offset":48528,"ancestor_record_offset":47044,"inlinee":22532,"source_line":910,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::get_component
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":48528,"parent_record_offset":47508,"ancestor_record_offset":47044,"inlinee":22531,"source_line":942,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":485,"record_offset":48288,"parent_record_offset":48264,"ancestor_record_offset":47044,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":47364,"parent_record_offset":47340,"ancestor_record_offset":47044,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":48336,"parent_record_offset":48312,"ancestor_record_offset":47044,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":48264,"parent_record_offset":48240,"ancestor_record_offset":47044,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":47752,"parent_record_offset":47728,"ancestor_record_offset":47044,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":47836,"parent_record_offset":47812,"ancestor_record_offset":47044,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":485,"record_offset":47776,"parent_record_offset":47752,"ancestor_record_offset":47044,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":47948,"parent_record_offset":47924,"ancestor_record_offset":47044,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":47340,"parent_record_offset":47252,"ancestor_record_offset":47044,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":48880,"parent_record_offset":48856,"ancestor_record_offset":47044,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":48920,"parent_record_offset":48736,"ancestor_record_offset":47044,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":49148,"parent_record_offset":47444,"ancestor_record_offset":47044,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":485,"record_offset":49024,"parent_record_offset":49000,"ancestor_record_offset":47044,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":48176,"parent_record_offset":48152,"ancestor_record_offset":47044,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":48096,"parent_record_offset":48072,"ancestor_record_offset":47044,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":485,"record_offset":47284,"parent_record_offset":47252,"ancestor_record_offset":47044,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":48668,"parent_record_offset":48640,"ancestor_record_offset":47044,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":47308,"parent_record_offset":47284,"ancestor_record_offset":47044,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":49000,"parent_record_offset":48968,"ancestor_record_offset":47044,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":48696,"parent_record_offset":48668,"ancestor_record_offset":47044,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":485,"record_offset":48312,"parent_record_offset":48288,"ancestor_record_offset":47044,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs
//     evidence={"module":485,"record_offset":47444,"parent_record_offset":47404,"ancestor_record_offset":47044,"inlinee":27355,"source_line":23,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":485,"record_offset":48240,"parent_record_offset":48216,"ancestor_record_offset":47044,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":47572,"parent_record_offset":47544,"ancestor_record_offset":47044,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":47600,"parent_record_offset":47572,"ancestor_record_offset":47044,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":48216,"parent_record_offset":48016,"ancestor_record_offset":47044,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48128,"parent_record_offset":48044,"ancestor_record_offset":47044,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48072,"parent_record_offset":48044,"ancestor_record_offset":47044,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48472,"parent_record_offset":47680,"ancestor_record_offset":47044,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48044,"parent_record_offset":48016,"ancestor_record_offset":47044,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":47656,"parent_record_offset":47632,"ancestor_record_offset":47044,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":47632,"parent_record_offset":47600,"ancestor_record_offset":47044,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":47680,"parent_record_offset":47656,"ancestor_record_offset":47044,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":47896,"parent_record_offset":47868,"ancestor_record_offset":47044,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":47924,"parent_record_offset":47896,"ancestor_record_offset":47044,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":47984,"parent_record_offset":47868,"ancestor_record_offset":47044,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":47868,"parent_record_offset":47680,"ancestor_record_offset":47044,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48016,"parent_record_offset":47680,"ancestor_record_offset":47044,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":47728,"parent_record_offset":47680,"ancestor_record_offset":47044,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":47812,"parent_record_offset":47680,"ancestor_record_offset":47044,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":48388,"parent_record_offset":47680,"ancestor_record_offset":47044,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":485,"record_offset":48968,"parent_record_offset":48944,"ancestor_record_offset":47044,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6ADB
// 首条位置: {"module":485,"inlinee":27355}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::entity_ref::EntityRef::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/entity_ref.rs
//     evidence={"module":485,"record_offset":47476,"parent_record_offset":47444,"ancestor_record_offset":47444,"inlinee":27357,"source_line":111,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":49148,"parent_record_offset":47444,"ancestor_record_offset":47444,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":48640,"parent_record_offset":48600,"ancestor_record_offset":47444,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":485,"record_offset":47544,"parent_record_offset":47508,"ancestor_record_offset":47444,"inlinee":22529,"source_line":713,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":49104,"parent_record_offset":49080,"ancestor_record_offset":47444,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":48764,"parent_record_offset":48736,"ancestor_record_offset":47444,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":48736,"parent_record_offset":48600,"ancestor_record_offset":47444,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":48856,"parent_record_offset":48792,"ancestor_record_offset":47444,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":48944,"parent_record_offset":48920,"ancestor_record_offset":47444,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":485,"record_offset":49080,"parent_record_offset":48528,"ancestor_record_offset":47444,"inlinee":22535,"source_line":383,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$7::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":485,"record_offset":48600,"parent_record_offset":48568,"ancestor_record_offset":47444,"inlinee":22534,"source_line":904,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":47508,"parent_record_offset":47476,"ancestor_record_offset":47444,"inlinee":22528,"source_line":691,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::fetch_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":48568,"parent_record_offset":48528,"ancestor_record_offset":47444,"inlinee":22532,"source_line":910,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::get_component
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":48528,"parent_record_offset":47508,"ancestor_record_offset":47444,"inlinee":22531,"source_line":942,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":485,"record_offset":48288,"parent_record_offset":48264,"ancestor_record_offset":47444,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":48336,"parent_record_offset":48312,"ancestor_record_offset":47444,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":48264,"parent_record_offset":48240,"ancestor_record_offset":47444,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":47752,"parent_record_offset":47728,"ancestor_record_offset":47444,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":47836,"parent_record_offset":47812,"ancestor_record_offset":47444,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":485,"record_offset":47776,"parent_record_offset":47752,"ancestor_record_offset":47444,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":47948,"parent_record_offset":47924,"ancestor_record_offset":47444,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":48880,"parent_record_offset":48856,"ancestor_record_offset":47444,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":48920,"parent_record_offset":48736,"ancestor_record_offset":47444,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":485,"record_offset":49024,"parent_record_offset":49000,"ancestor_record_offset":47444,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":48176,"parent_record_offset":48152,"ancestor_record_offset":47444,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":48096,"parent_record_offset":48072,"ancestor_record_offset":47444,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":485,"record_offset":48792,"parent_record_offset":48764,"ancestor_record_offset":47444,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":48668,"parent_record_offset":48640,"ancestor_record_offset":47444,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":48828,"parent_record_offset":48792,"ancestor_record_offset":47444,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":49000,"parent_record_offset":48968,"ancestor_record_offset":47444,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":48696,"parent_record_offset":48668,"ancestor_record_offset":47444,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":485,"record_offset":48312,"parent_record_offset":48288,"ancestor_record_offset":47444,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":485,"record_offset":48240,"parent_record_offset":48216,"ancestor_record_offset":47444,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":47572,"parent_record_offset":47544,"ancestor_record_offset":47444,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":47600,"parent_record_offset":47572,"ancestor_record_offset":47444,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":48216,"parent_record_offset":48016,"ancestor_record_offset":47444,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48128,"parent_record_offset":48044,"ancestor_record_offset":47444,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48072,"parent_record_offset":48044,"ancestor_record_offset":47444,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48472,"parent_record_offset":47680,"ancestor_record_offset":47444,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48044,"parent_record_offset":48016,"ancestor_record_offset":47444,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":47656,"parent_record_offset":47632,"ancestor_record_offset":47444,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":47632,"parent_record_offset":47600,"ancestor_record_offset":47444,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":47680,"parent_record_offset":47656,"ancestor_record_offset":47444,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":47896,"parent_record_offset":47868,"ancestor_record_offset":47444,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":47924,"parent_record_offset":47896,"ancestor_record_offset":47444,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":47984,"parent_record_offset":47868,"ancestor_record_offset":47444,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":47868,"parent_record_offset":47680,"ancestor_record_offset":47444,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":48016,"parent_record_offset":47680,"ancestor_record_offset":47444,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":47728,"parent_record_offset":47680,"ancestor_record_offset":47444,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":47812,"parent_record_offset":47680,"ancestor_record_offset":47444,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":48388,"parent_record_offset":47680,"ancestor_record_offset":47444,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":485,"record_offset":48968,"parent_record_offset":48944,"ancestor_record_offset":47444,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00485.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_draw.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_draw.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: stairs_draw.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":485,"record_offset":49184,"parent_record_offset":47044,"ancestor_record_offset":47044,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_draw.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw
// 信息 ID: 70898d599a1316797feb19b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw::closure$0
// 信息 ID: b64f2f51c6dc899ea6264d19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw; module=485, inline_record=47444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
