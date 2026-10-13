// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5856f9e90d255282af565fd5b9127e3bacb9227f2f007f0a0ea45b531ae1d026
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reset_indirect_args.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 402, 1385
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
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
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":402,"record_offset":20936,"rva":5583360,"va":5374292480}
// 编译实例 RVAs: 0x553200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=4
//     target_aliases=1889 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5573135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8f1f5aaff","size":5,"soft_fail":false,"target_va":5368720672,"va":5374293802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064bc90
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1017148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e83c850f00","size":5,"soft_fail":false,"target_va":5375310992,"va":5374293839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064c140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1017858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e802880f00","size":5,"soft_fail":false,"target_va":5375312192,"va":5374294329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1607166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8fe851800","size":5,"soft_fail":false,"target_va":5375902656,"va":5374295485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1945377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e821af1d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5374294538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1949846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e896c01d00","size":5,"soft_fail":false,"target_va":5376242976,"va":5374293125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d336c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8258151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e867027e00","size":5,"soft_fail":false,"target_va":5382551232,"va":5374293076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d37040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8272891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8fb3b7e00","size":5,"soft_fail":false,"target_va":5382565952,"va":5374293056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d38460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8277983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8df4f7e00","size":5,"soft_fail":false,"target_va":5382571104,"va":5374293116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d3e060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8301555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8f3ab7e00","size":5,"soft_fail":false,"target_va":5382594656,"va":5374293096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29299889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8b114bf01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374293466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28684513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8e1b0b501","size":5,"soft_fail":false,"target_va":5402978880,"va":5374294362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28686068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8f4b6b501","size":5,"soft_fail":false,"target_va":5402980464,"va":5374294391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1969127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374292480,"raw_hex":"e8e70b1e00","size":5,"soft_fail":false,"target_va":5376262400,"va":5374293268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":402,"record_offset":24548,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":402,"record_offset":21140,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":26796,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":26852,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::pass_resource_access::ssbo_to_rg_handle
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":402,"record_offset":24604,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":22846,"source_line":206,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":402,"record_offset":25000,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":23008,"source_line":150,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":402,"record_offset":26556,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":402,"record_offset":24884,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":9083,"source_line":27,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":402,"record_offset":24572,"parent_record_offset":24548,"ancestor_record_offset":20936,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":402,"record_offset":21168,"parent_record_offset":21140,"ancestor_record_offset":20936,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":402,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":20936,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":402,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":20936,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":402,"record_offset":22604,"parent_record_offset":22572,"ancestor_record_offset":20936,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":402,"record_offset":22548,"parent_record_offset":22156,"ancestor_record_offset":20936,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":402,"record_offset":22572,"parent_record_offset":22548,"ancestor_record_offset":20936,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":402,"record_offset":22188,"parent_record_offset":22156,"ancestor_record_offset":20936,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":402,"record_offset":22632,"parent_record_offset":22572,"ancestor_record_offset":20936,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":402,"record_offset":22236,"parent_record_offset":22212,"ancestor_record_offset":20936,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":402,"record_offset":22212,"parent_record_offset":22188,"ancestor_record_offset":20936,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":402,"record_offset":22328,"parent_record_offset":22264,"ancestor_record_offset":20936,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":402,"record_offset":22416,"parent_record_offset":22392,"ancestor_record_offset":20936,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":402,"record_offset":22748,"parent_record_offset":22724,"ancestor_record_offset":20936,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":402,"record_offset":22724,"parent_record_offset":22156,"ancestor_record_offset":20936,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":402,"record_offset":22156,"parent_record_offset":21140,"ancestor_record_offset":20936,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":402,"record_offset":21888,"parent_record_offset":21864,"ancestor_record_offset":20936,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":402,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":20936,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":402,"record_offset":21864,"parent_record_offset":21840,"ancestor_record_offset":20936,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":402,"record_offset":24912,"parent_record_offset":24884,"ancestor_record_offset":20936,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":402,"record_offset":21356,"parent_record_offset":21332,"ancestor_record_offset":20936,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":402,"record_offset":21440,"parent_record_offset":21416,"ancestor_record_offset":20936,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":402,"record_offset":21380,"parent_record_offset":21356,"ancestor_record_offset":20936,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":402,"record_offset":25188,"parent_record_offset":25160,"ancestor_record_offset":20936,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":402,"record_offset":21552,"parent_record_offset":21528,"ancestor_record_offset":20936,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":20936,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":22124,"parent_record_offset":21168,"ancestor_record_offset":20936,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":22392,"parent_record_offset":22212,"ancestor_record_offset":20936,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":402,"record_offset":22496,"parent_record_offset":22472,"ancestor_record_offset":20936,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":25112,"parent_record_offset":25000,"ancestor_record_offset":20936,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":402,"record_offset":21776,"parent_record_offset":21752,"ancestor_record_offset":20936,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":402,"record_offset":21696,"parent_record_offset":21672,"ancestor_record_offset":20936,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":25212,"parent_record_offset":25188,"ancestor_record_offset":20936,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":402,"record_offset":25044,"parent_record_offset":25000,"ancestor_record_offset":20936,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":402,"record_offset":22264,"parent_record_offset":22236,"ancestor_record_offset":20936,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":402,"record_offset":22300,"parent_record_offset":22264,"ancestor_record_offset":20936,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":402,"record_offset":22472,"parent_record_offset":22440,"ancestor_record_offset":20936,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":402,"record_offset":22772,"parent_record_offset":22748,"ancestor_record_offset":20936,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":402,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":20936,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":402,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":20936,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":402,"record_offset":21840,"parent_record_offset":21816,"ancestor_record_offset":20936,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":402,"record_offset":21192,"parent_record_offset":21168,"ancestor_record_offset":20936,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":402,"record_offset":21216,"parent_record_offset":21192,"ancestor_record_offset":20936,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":402,"record_offset":21816,"parent_record_offset":21620,"ancestor_record_offset":20936,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":21728,"parent_record_offset":21644,"ancestor_record_offset":20936,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":21672,"parent_record_offset":21644,"ancestor_record_offset":20936,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":22072,"parent_record_offset":21292,"ancestor_record_offset":20936,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":21644,"parent_record_offset":21620,"ancestor_record_offset":20936,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":21268,"parent_record_offset":21244,"ancestor_record_offset":20936,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":21244,"parent_record_offset":21216,"ancestor_record_offset":20936,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":21292,"parent_record_offset":21268,"ancestor_record_offset":20936,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":402,"record_offset":21500,"parent_record_offset":21472,"ancestor_record_offset":20936,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":402,"record_offset":21528,"parent_record_offset":21500,"ancestor_record_offset":20936,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":402,"record_offset":21588,"parent_record_offset":21472,"ancestor_record_offset":20936,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":402,"record_offset":21472,"parent_record_offset":21292,"ancestor_record_offset":20936,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":21620,"parent_record_offset":21292,"ancestor_record_offset":20936,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":402,"record_offset":21332,"parent_record_offset":21292,"ancestor_record_offset":20936,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":402,"record_offset":21416,"parent_record_offset":21292,"ancestor_record_offset":20936,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":402,"record_offset":21988,"parent_record_offset":21292,"ancestor_record_offset":20936,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":402,"record_offset":22440,"parent_record_offset":22416,"ancestor_record_offset":20936,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":402,"record_offset":25160,"parent_record_offset":25136,"ancestor_record_offset":20936,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":402,"record_offset":26584,"parent_record_offset":26556,"ancestor_record_offset":20936,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCC5
// 首条位置: {"module":1385,"inlinee":64709}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":24904,"parent_record_offset":24336,"ancestor_record_offset":24336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::track_unwrap
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":24608,"parent_record_offset":24336,"ancestor_record_offset":24336,"inlinee":62065,"source_line":424,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Ref::raw
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1385,"record_offset":24496,"parent_record_offset":24336,"ancestor_record_offset":24336,"inlinee":26479,"source_line":355,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1385,"record_offset":24752,"parent_record_offset":24724,"ancestor_record_offset":24336,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":24700,"parent_record_offset":24608,"ancestor_record_offset":24336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":24776,"parent_record_offset":24752,"ancestor_record_offset":24336,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":24668,"parent_record_offset":24644,"ancestor_record_offset":24336,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$22::track
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":24644,"parent_record_offset":24608,"ancestor_record_offset":24336,"inlinee":61716,"source_line":397,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":24724,"parent_record_offset":24700,"ancestor_record_offset":24336,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/01385.jsonl.gz"}
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
// 完整结构及原始类型记录: reset_indirect_args.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: reset_indirect_args.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: reset_indirect_args.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_direct","evidence":{"module":402,"record_offset":26556,"parent_record_offset":20936,"ancestor_record_offset":20936,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00402.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3135385,"position_among_mapped_render_targets":1,"mapped_render_target_count":70,"previous_callsite":null,"next_callsite":3137068,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reset_indirect_args.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>", "bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
// 信息 ID: 2456e460e286b8fde4ca1d52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fd799 -> 0x553200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0
// 信息 ID: ab66e12137e50bfcb9d08dd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0 <- alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=24336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
