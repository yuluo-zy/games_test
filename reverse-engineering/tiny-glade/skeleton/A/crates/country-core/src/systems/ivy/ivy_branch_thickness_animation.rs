// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ivy/ivy_branch_thickness_animation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e1fd9e07f80f76c056567a67b977a0e6a936566ee843cd2625fb88795e773232
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_branch_thickness_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 499
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_branch_thickness_animation::ivy_branch_thickness_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":499,"record_offset":47492,"rva":7165616,"va":5375874736}
// 编译实例 RVAs: 0x6d56b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_branch_thickness_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7432966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e8066b7100","size":5,"soft_fail":false,"target_va":5383308144,"va":5375875173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ivy_branch_thickness_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27718455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e837f3a601","size":5,"soft_fail":false,"target_va":5403594064,"va":5375875604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27102675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e8d38d9d01","size":5,"soft_fail":false,"target_va":5402978688,"va":5375876008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27104901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e885969d01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375875222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-15781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e85bc2ffff","size":5,"soft_fail":false,"target_va":5375859040,"va":5375874816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27105765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e8e5999d01","size":5,"soft_fail":false,"target_va":5402980656,"va":5375874886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t481521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e8f1580700","size":5,"soft_fail":false,"target_va":5376356912,"va":5375875386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t485402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e81a680700","size":5,"soft_fail":false,"target_va":5376360560,"va":5375875153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t481352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375874736,"raw_hex":"e848580700","size":5,"soft_fail":false,"target_va":5376356144,"va":5375874787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-17153","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375874736,"raw_hex":"e9ffbcffff","size":5,"soft_fail":false,"target_va":5375858848,"va":5375875996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":499,"record_offset":51340,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":20372,"source_line":1149,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":499,"record_offset":50328,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":499,"record_offset":51764,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":499,"record_offset":51280,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":23351,"source_line":50,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":499,"record_offset":50388,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":25002,"source_line":159,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":499,"record_offset":51624,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":499,"record_offset":49296,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":499,"record_offset":47708,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":499,"record_offset":48284,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":499,"record_offset":50796,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":499,"record_offset":51308,"parent_record_offset":47492,"ancestor_record_offset":47492,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_mut_slices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":499,"record_offset":51368,"parent_record_offset":51340,"ancestor_record_offset":47492,"inlinee":11897,"source_line":1216,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":499,"record_offset":51532,"parent_record_offset":51368,"ancestor_record_offset":47492,"inlinee":11292,"source_line":180,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":499,"record_offset":51556,"parent_record_offset":51532,"ancestor_record_offset":47492,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":499,"record_offset":51400,"parent_record_offset":51368,"ancestor_record_offset":47492,"inlinee":11289,"source_line":1270,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":499,"record_offset":51444,"parent_record_offset":51400,"ancestor_record_offset":47492,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":499,"record_offset":51468,"parent_record_offset":51444,"ancestor_record_offset":47492,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::iter_mut::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\iter_mut.rs
//     evidence={"module":499,"record_offset":51792,"parent_record_offset":51764,"ancestor_record_offset":47492,"inlinee":20374,"source_line":36,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":499,"record_offset":51492,"parent_record_offset":51468,"ancestor_record_offset":47492,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":499,"record_offset":51580,"parent_record_offset":51556,"ancestor_record_offset":47492,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":499,"record_offset":50356,"parent_record_offset":50328,"ancestor_record_offset":47492,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":499,"record_offset":49680,"parent_record_offset":49652,"ancestor_record_offset":47492,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":499,"record_offset":49732,"parent_record_offset":49708,"ancestor_record_offset":47492,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":499,"record_offset":49708,"parent_record_offset":49680,"ancestor_record_offset":47492,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":499,"record_offset":49620,"parent_record_offset":49592,"ancestor_record_offset":47492,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":499,"record_offset":49652,"parent_record_offset":49620,"ancestor_record_offset":47492,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":499,"record_offset":49824,"parent_record_offset":49652,"ancestor_record_offset":47492,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":499,"record_offset":49936,"parent_record_offset":49912,"ancestor_record_offset":47492,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":499,"record_offset":50672,"parent_record_offset":50644,"ancestor_record_offset":47492,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":499,"record_offset":50732,"parent_record_offset":50700,"ancestor_record_offset":47492,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":499,"record_offset":49756,"parent_record_offset":49732,"ancestor_record_offset":47492,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":499,"record_offset":47920,"parent_record_offset":47824,"ancestor_record_offset":47492,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":499,"record_offset":51676,"parent_record_offset":51624,"ancestor_record_offset":47492,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":499,"record_offset":51732,"parent_record_offset":51624,"ancestor_record_offset":47492,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":499,"record_offset":51648,"parent_record_offset":51624,"ancestor_record_offset":47492,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":499,"record_offset":49516,"parent_record_offset":49488,"ancestor_record_offset":47492,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":499,"record_offset":48880,"parent_record_offset":48420,"ancestor_record_offset":47492,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":499,"record_offset":49564,"parent_record_offset":49392,"ancestor_record_offset":47492,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":499,"record_offset":48232,"parent_record_offset":48204,"ancestor_record_offset":47492,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":499,"record_offset":49888,"parent_record_offset":49592,"ancestor_record_offset":47492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":499,"record_offset":51196,"parent_record_offset":51172,"ancestor_record_offset":47492,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":499,"record_offset":51172,"parent_record_offset":51148,"ancestor_record_offset":47492,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":499,"record_offset":51872,"parent_record_offset":51848,"ancestor_record_offset":47492,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":499,"record_offset":48656,"parent_record_offset":48632,"ancestor_record_offset":47492,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":499,"record_offset":48632,"parent_record_offset":48608,"ancestor_record_offset":47492,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":499,"record_offset":48840,"parent_record_offset":48808,"ancestor_record_offset":47492,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":499,"record_offset":49780,"parent_record_offset":49756,"ancestor_record_offset":47492,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":499,"record_offset":50288,"parent_record_offset":49360,"ancestor_record_offset":47492,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":499,"record_offset":47856,"parent_record_offset":47824,"ancestor_record_offset":47492,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":499,"record_offset":47884,"parent_record_offset":47856,"ancestor_record_offset":47492,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":499,"record_offset":48608,"parent_record_offset":48584,"ancestor_record_offset":47492,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":499,"record_offset":51848,"parent_record_offset":51824,"ancestor_record_offset":47492,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":499,"record_offset":48936,"parent_record_offset":48912,"ancestor_record_offset":47492,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":499,"record_offset":48584,"parent_record_offset":48476,"ancestor_record_offset":47492,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":499,"record_offset":51824,"parent_record_offset":51792,"ancestor_record_offset":47492,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":499,"record_offset":47824,"parent_record_offset":47796,"ancestor_record_offset":47492,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":499,"record_offset":48316,"parent_record_offset":48284,"ancestor_record_offset":47492,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":499,"record_offset":48448,"parent_record_offset":48420,"ancestor_record_offset":47492,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":499,"record_offset":48420,"parent_record_offset":48388,"ancestor_record_offset":47492,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":499,"record_offset":48364,"parent_record_offset":48340,"ancestor_record_offset":47492,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":499,"record_offset":48388,"parent_record_offset":48364,"ancestor_record_offset":47492,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":499,"record_offset":48340,"parent_record_offset":48316,"ancestor_record_offset":47492,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":499,"record_offset":48912,"parent_record_offset":48388,"ancestor_record_offset":47492,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":499,"record_offset":48020,"parent_record_offset":47988,"ancestor_record_offset":47492,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":499,"record_offset":48180,"parent_record_offset":48020,"ancestor_record_offset":47492,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":499,"record_offset":47988,"parent_record_offset":47964,"ancestor_record_offset":47492,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":499,"record_offset":47740,"parent_record_offset":47708,"ancestor_record_offset":47492,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":499,"record_offset":48204,"parent_record_offset":48180,"ancestor_record_offset":47492,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":499,"record_offset":47796,"parent_record_offset":47764,"ancestor_record_offset":47492,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":499,"record_offset":47764,"parent_record_offset":47740,"ancestor_record_offset":47492,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":499,"record_offset":48476,"parent_record_offset":48448,"ancestor_record_offset":47492,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":499,"record_offset":48724,"parent_record_offset":48476,"ancestor_record_offset":47492,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":499,"record_offset":48696,"parent_record_offset":48476,"ancestor_record_offset":47492,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":499,"record_offset":48528,"parent_record_offset":48476,"ancestor_record_offset":47492,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":499,"record_offset":50472,"parent_record_offset":50444,"ancestor_record_offset":47492,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":499,"record_offset":50852,"parent_record_offset":50824,"ancestor_record_offset":47492,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":499,"record_offset":51148,"parent_record_offset":50912,"ancestor_record_offset":47492,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":499,"record_offset":50588,"parent_record_offset":50528,"ancestor_record_offset":47492,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":499,"record_offset":50912,"parent_record_offset":50880,"ancestor_record_offset":47492,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":499,"record_offset":50500,"parent_record_offset":50472,"ancestor_record_offset":47492,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":499,"record_offset":50560,"parent_record_offset":50528,"ancestor_record_offset":47492,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":499,"record_offset":50528,"parent_record_offset":50500,"ancestor_record_offset":47492,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":499,"record_offset":50980,"parent_record_offset":50952,"ancestor_record_offset":47492,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":499,"record_offset":50952,"parent_record_offset":50912,"ancestor_record_offset":47492,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":499,"record_offset":50880,"parent_record_offset":50852,"ancestor_record_offset":47492,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":499,"record_offset":50644,"parent_record_offset":50588,"ancestor_record_offset":47492,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":499,"record_offset":50700,"parent_record_offset":50644,"ancestor_record_offset":47492,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":499,"record_offset":49328,"parent_record_offset":49296,"ancestor_record_offset":47492,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":499,"record_offset":49428,"parent_record_offset":49392,"ancestor_record_offset":47492,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":499,"record_offset":49592,"parent_record_offset":49392,"ancestor_record_offset":47492,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":499,"record_offset":49860,"parent_record_offset":49592,"ancestor_record_offset":47492,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":499,"record_offset":50444,"parent_record_offset":50416,"ancestor_record_offset":47492,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":499,"record_offset":50416,"parent_record_offset":50388,"ancestor_record_offset":47492,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":499,"record_offset":50824,"parent_record_offset":50796,"ancestor_record_offset":47492,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":499,"record_offset":49456,"parent_record_offset":49428,"ancestor_record_offset":47492,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":499,"record_offset":49488,"parent_record_offset":49456,"ancestor_record_offset":47492,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":499,"record_offset":49392,"parent_record_offset":49360,"ancestor_record_offset":47492,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":499,"record_offset":49360,"parent_record_offset":49328,"ancestor_record_offset":47492,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00499.jsonl.gz"}
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
// 完整结构及原始类型记录: ivy_branch_thickness_animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ivy_branch_thickness_animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ivy_branch_thickness_animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_branch_thickness_animation::ivy_branch_thickness_animation
// 信息 ID: 6acdaf8ba5d1df1d6371c734
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 722bfa9098a4fce6ec596e11: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)> > @ 0xe4b527 -> 0x6d56b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)> @ 0xf26511 -> 0x6d56b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
