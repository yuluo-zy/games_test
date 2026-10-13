// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/history/replay_terrain.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 35e93833cab54ec6a40685de9511830b5a69e8011348d6afbb4942b92fa3f68b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 416, 484
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
// 函数符号: country_core::systems::history::replay_terrain::replay_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":416,"record_offset":15000,"rva":5787168,"va":5374496288}
// 编译实例 RVAs: 0x584e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1747275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e8b556e5ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374496966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1747449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e80756e5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374497156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f32940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10147818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e8ead79a00","size":5,"soft_fail":false,"target_va":5384644928,"va":5374497105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29096352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e8a0f9bb01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374497003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29095834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e89af7bb01","size":5,"soft_fail":false,"target_va":5403593376,"va":5374497537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28480429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e8ad93b201","size":5,"soft_fail":false,"target_va":5402978144,"va":5374497710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3191807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e8014ccfff","size":5,"soft_fail":false,"target_va":5371305056,"va":5374496858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3029420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e8ac392e00","size":5,"soft_fail":false,"target_va":5377527120,"va":5374497695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28480765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e8fd94b201","size":5,"soft_fail":false,"target_va":5402978432,"va":5374497662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28481459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e8b397b201","size":5,"soft_fail":false,"target_va":5402979152,"va":5374497688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e82beaffff","size":5,"soft_fail":false,"target_va":5374491968,"va":5374497552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::reapply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1263887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e80f491300","size":5,"soft_fail":false,"target_va":5375760704,"va":5374496812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::revert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1263003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374496288,"raw_hex":"e89b451300","size":5,"soft_fail":false,"target_va":5375759792,"va":5374496784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":416,"record_offset":16012,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":15508,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":15620,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":17652,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":16136,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":416,"record_offset":17516,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":416,"record_offset":15204,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":416,"record_offset":15924,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":416,"record_offset":15748,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":16956,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":416,"record_offset":15536,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":416,"record_offset":17228,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":416,"record_offset":15268,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":23761,"source_line":165,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::set_stroke
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":416,"record_offset":16644,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":23766,"source_line":93,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_terrain::ease_out
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_terrain.rs
//     evidence={"module":416,"record_offset":15956,"parent_record_offset":15000,"ancestor_record_offset":15000,"inlinee":23764,"source_line":162,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":15000,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":15000,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":16904,"parent_record_offset":16880,"ancestor_record_offset":15000,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":16412,"parent_record_offset":16312,"ancestor_record_offset":15000,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":16880,"parent_record_offset":16792,"ancestor_record_offset":15000,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":15292,"parent_record_offset":15268,"ancestor_record_offset":15000,"inlinee":23762,"source_line":1544,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":15000,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":15348,"parent_record_offset":15316,"ancestor_record_offset":15000,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":15412,"parent_record_offset":15316,"ancestor_record_offset":15000,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":15436,"parent_record_offset":15412,"ancestor_record_offset":15000,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":15000,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":16820,"parent_record_offset":16792,"ancestor_record_offset":15000,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":15380,"parent_record_offset":15348,"ancestor_record_offset":15000,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":16312,"parent_record_offset":16284,"ancestor_record_offset":15000,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":16284,"parent_record_offset":16256,"ancestor_record_offset":15000,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":16532,"parent_record_offset":16284,"ancestor_record_offset":15000,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":16792,"parent_record_offset":16768,"ancestor_record_offset":15000,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":416,"record_offset":16220,"parent_record_offset":16192,"ancestor_record_offset":15000,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":416,"record_offset":16192,"parent_record_offset":16164,"ancestor_record_offset":15000,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":416,"record_offset":16164,"parent_record_offset":16136,"ancestor_record_offset":15000,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":15660,"parent_record_offset":15620,"ancestor_record_offset":15000,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":17700,"parent_record_offset":17652,"ancestor_record_offset":15000,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":17860,"parent_record_offset":17652,"ancestor_record_offset":15000,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":16256,"parent_record_offset":16220,"ancestor_record_offset":15000,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":416,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":15000,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":416,"record_offset":17964,"parent_record_offset":17940,"ancestor_record_offset":15000,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":416,"record_offset":15232,"parent_record_offset":15204,"ancestor_record_offset":15000,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":416,"record_offset":16348,"parent_record_offset":16312,"ancestor_record_offset":15000,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":416,"record_offset":16376,"parent_record_offset":16348,"ancestor_record_offset":15000,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":416,"record_offset":16596,"parent_record_offset":16572,"ancestor_record_offset":15000,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":416,"record_offset":17300,"parent_record_offset":17256,"ancestor_record_offset":15000,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":416,"record_offset":17328,"parent_record_offset":17300,"ancestor_record_offset":15000,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":416,"record_offset":16844,"parent_record_offset":16820,"ancestor_record_offset":15000,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":416,"record_offset":16572,"parent_record_offset":16220,"ancestor_record_offset":15000,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":16720,"parent_record_offset":16644,"ancestor_record_offset":15000,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":416,"record_offset":15584,"parent_record_offset":15560,"ancestor_record_offset":15000,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":416,"record_offset":15884,"parent_record_offset":15860,"ancestor_record_offset":15000,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":416,"record_offset":17592,"parent_record_offset":17516,"ancestor_record_offset":15000,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":416,"record_offset":15828,"parent_record_offset":15784,"ancestor_record_offset":15000,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":17372,"parent_record_offset":17328,"ancestor_record_offset":15000,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":16488,"parent_record_offset":16460,"ancestor_record_offset":15000,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":15716,"parent_record_offset":15620,"ancestor_record_offset":15000,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":416,"record_offset":17256,"parent_record_offset":17228,"ancestor_record_offset":15000,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":416,"record_offset":15560,"parent_record_offset":15536,"ancestor_record_offset":15000,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":416,"record_offset":15860,"parent_record_offset":15784,"ancestor_record_offset":15000,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":416,"record_offset":15784,"parent_record_offset":15748,"ancestor_record_offset":15000,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":416,"record_offset":16692,"parent_record_offset":16644,"ancestor_record_offset":15000,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":416,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":15000,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::forward
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CFA
// 首条位置: {"module":484,"inlinee":23802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":20760,"parent_record_offset":20396,"ancestor_record_offset":20396,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::TerrainEdit::duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":484,"record_offset":20432,"parent_record_offset":20396,"ancestor_record_offset":20396,"inlinee":23758,"source_line":58,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":484,"record_offset":20784,"parent_record_offset":20760,"ancestor_record_offset":20396,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":20696,"parent_record_offset":20672,"ancestor_record_offset":20396,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":20672,"parent_record_offset":20432,"ancestor_record_offset":20396,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":20728,"parent_record_offset":20432,"ancestor_record_offset":20396,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":20584,"parent_record_offset":20560,"ancestor_record_offset":20396,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":484,"record_offset":20640,"parent_record_offset":20616,"ancestor_record_offset":20396,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":20560,"parent_record_offset":20432,"ancestor_record_offset":20396,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":20616,"parent_record_offset":20432,"ancestor_record_offset":20396,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":20524,"parent_record_offset":20500,"ancestor_record_offset":20396,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":484,"record_offset":20476,"parent_record_offset":20432,"ancestor_record_offset":20396,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":484,"record_offset":20500,"parent_record_offset":20476,"ancestor_record_offset":20396,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":484,"record_offset":20820,"parent_record_offset":20784,"ancestor_record_offset":20396,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":484,"record_offset":20844,"parent_record_offset":20820,"ancestor_record_offset":20396,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":484,"record_offset":20876,"parent_record_offset":20784,"ancestor_record_offset":20396,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::backward
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CFC
// 首条位置: {"module":484,"inlinee":23804}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":20056,"parent_record_offset":19692,"ancestor_record_offset":19692,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::TerrainEdit::duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":484,"record_offset":19728,"parent_record_offset":19692,"ancestor_record_offset":19692,"inlinee":23758,"source_line":58,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":484,"record_offset":20080,"parent_record_offset":20056,"ancestor_record_offset":19692,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":19992,"parent_record_offset":19968,"ancestor_record_offset":19692,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":19968,"parent_record_offset":19728,"ancestor_record_offset":19692,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":20024,"parent_record_offset":19728,"ancestor_record_offset":19692,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":19880,"parent_record_offset":19856,"ancestor_record_offset":19692,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":484,"record_offset":19936,"parent_record_offset":19912,"ancestor_record_offset":19692,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":19856,"parent_record_offset":19728,"ancestor_record_offset":19692,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":19912,"parent_record_offset":19728,"ancestor_record_offset":19692,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":19692,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":484,"record_offset":19772,"parent_record_offset":19728,"ancestor_record_offset":19692,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":484,"record_offset":19796,"parent_record_offset":19772,"ancestor_record_offset":19692,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":484,"record_offset":20116,"parent_record_offset":20080,"ancestor_record_offset":19692,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":484,"record_offset":20140,"parent_record_offset":20116,"ancestor_record_offset":19692,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":484,"record_offset":20172,"parent_record_offset":20080,"ancestor_record_offset":19692,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::new
// 记录形式: inlinee_declaration; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CD0
// 首条位置: {"module":416,"inlinee":23760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::ease_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CD4
// 首条位置: {"module":416,"inlinee":23764}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":416,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":15956,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/00416.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_terrain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_terrain::TerrainReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_terrain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_terrain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::backward
// 信息 ID: 52932fd51ab3c9ce5c676c8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=19692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::forward
// 信息 ID: 2ede648fcbfc5296bf1d7fd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=20396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::new
// 信息 ID: c80f691100b7a23817251e62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::ease_out
// 信息 ID: 67a751a3ae5206bcbf518ed0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::replay_terrain; module=416, inline_record=15956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::replay_terrain
// 信息 ID: 09edc07baee0a055d276e81a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ce35ececc32f2f3b5ecad055: void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)> > @ 0xe3801a -> 0x584e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)> @ 0x108142c -> 0x584e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
