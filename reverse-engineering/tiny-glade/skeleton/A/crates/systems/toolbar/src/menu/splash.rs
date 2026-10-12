// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/menu/splash.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 56e52bb5565dd0c55ba22562d3748b9e9ff06d30a78d9242c2efcb85031cd785
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: splash.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1018, 2507
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::splash::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2507,"record_offset":244,"rva":23495024,"va":5392204144}
// 编译实例 RVAs: 0x1668170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: splash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22590849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392204144,"raw_hex":"e87f4aa7fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5392204364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: splash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22586608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392204144,"raw_hex":"e8105ba7fe","size":5,"soft_fail":false,"target_va":5369617648,"va":5392204251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a572e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: splash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12652636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392204144,"raw_hex":"e8a4ef3eff","size":5,"soft_fail":false,"target_va":5379551968,"va":5392204599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20895506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392204144,"raw_hex":"e8ee28c1fe","size":5,"soft_fail":false,"target_va":5371308800,"va":5392204301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10774237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392204144,"raw_hex":"e8dd66a400","size":5,"soft_fail":false,"target_va":5402978880,"va":5392204638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19044888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392204144,"raw_hex":"e8e865ddfe","size":5,"soft_fail":false,"target_va":5373159392,"va":5392204275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2507,"record_offset":396,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2507,"record_offset":368,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen_without_block_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2507,"record_offset":452,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":50926,"source_line":343,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2507,"record_offset":676,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2507,"record_offset":604,"parent_record_offset":572,"ancestor_record_offset":244,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2507,"record_offset":572,"parent_record_offset":540,"ancestor_record_offset":244,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2507,"record_offset":540,"parent_record_offset":476,"ancestor_record_offset":244,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2507,"record_offset":420,"parent_record_offset":396,"ancestor_record_offset":244,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2507,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":244,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2507,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":244,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2507,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":244,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2507,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2507,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":244,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2507,"record_offset":1704,"parent_record_offset":676,"ancestor_record_offset":244,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2507,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":244,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2507,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":244,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2507,"record_offset":636,"parent_record_offset":540,"ancestor_record_offset":244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2507,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":244,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2507,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":244,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2507,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":244,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2507,"record_offset":1440,"parent_record_offset":1220,"ancestor_record_offset":244,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2507,"record_offset":784,"parent_record_offset":752,"ancestor_record_offset":244,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":244,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":1328,"parent_record_offset":1220,"ancestor_record_offset":244,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":244,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":1648,"parent_record_offset":860,"ancestor_record_offset":244,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":244,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":244,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":812,"parent_record_offset":752,"ancestor_record_offset":244,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":244,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2507,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":244,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2507,"record_offset":1128,"parent_record_offset":1100,"ancestor_record_offset":244,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2507,"record_offset":1188,"parent_record_offset":1072,"ancestor_record_offset":244,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2507,"record_offset":1072,"parent_record_offset":860,"ancestor_record_offset":244,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":904,"parent_record_offset":860,"ancestor_record_offset":244,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2507,"record_offset":1220,"parent_record_offset":860,"ancestor_record_offset":244,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2507,"record_offset":932,"parent_record_offset":860,"ancestor_record_offset":244,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2507,"record_offset":1016,"parent_record_offset":860,"ancestor_record_offset":244,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2507,"record_offset":1556,"parent_record_offset":860,"ancestor_record_offset":244,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2507,"record_offset":704,"parent_record_offset":676,"ancestor_record_offset":244,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2507,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":244,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2507,"record_offset":512,"parent_record_offset":476,"ancestor_record_offset":244,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2507,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":244,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::splash::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6CA
// 首条位置: {"module":1018,"inlinee":50890}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":67096,"parent_record_offset":57964,"ancestor_record_offset":57964,"inlinee":17396,"source_line":238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":59048,"parent_record_offset":57964,"ancestor_record_offset":57964,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":59192,"parent_record_offset":57964,"ancestor_record_offset":57964,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":58008,"parent_record_offset":57964,"ancestor_record_offset":57964,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":60220,"parent_record_offset":57964,"ancestor_record_offset":57964,"inlinee":8031,"source_line":727,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":66024,"parent_record_offset":57964,"ancestor_record_offset":57964,"inlinee":50894,"source_line":138,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":58828,"parent_record_offset":58804,"ancestor_record_offset":57964,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":58296,"parent_record_offset":58272,"ancestor_record_offset":57964,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":58380,"parent_record_offset":58356,"ancestor_record_offset":57964,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":58320,"parent_record_offset":58296,"ancestor_record_offset":57964,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":58492,"parent_record_offset":58468,"ancestor_record_offset":57964,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":58740,"parent_record_offset":58716,"ancestor_record_offset":57964,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":58632,"parent_record_offset":58608,"ancestor_record_offset":57964,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":58804,"parent_record_offset":58780,"ancestor_record_offset":57964,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":58092,"parent_record_offset":58064,"ancestor_record_offset":57964,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":58064,"parent_record_offset":58036,"ancestor_record_offset":57964,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":58780,"parent_record_offset":58560,"ancestor_record_offset":57964,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":58124,"parent_record_offset":58092,"ancestor_record_offset":57964,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58692,"parent_record_offset":58668,"ancestor_record_offset":57964,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58668,"parent_record_offset":58560,"ancestor_record_offset":57964,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58608,"parent_record_offset":58584,"ancestor_record_offset":57964,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58988,"parent_record_offset":58200,"ancestor_record_offset":57964,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58584,"parent_record_offset":58560,"ancestor_record_offset":57964,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58176,"parent_record_offset":58152,"ancestor_record_offset":57964,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58152,"parent_record_offset":58092,"ancestor_record_offset":57964,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58200,"parent_record_offset":58176,"ancestor_record_offset":57964,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":58440,"parent_record_offset":58412,"ancestor_record_offset":57964,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":58468,"parent_record_offset":58440,"ancestor_record_offset":57964,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":58528,"parent_record_offset":58412,"ancestor_record_offset":57964,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":58412,"parent_record_offset":58200,"ancestor_record_offset":57964,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58244,"parent_record_offset":58200,"ancestor_record_offset":57964,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":58560,"parent_record_offset":58200,"ancestor_record_offset":57964,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":58272,"parent_record_offset":58200,"ancestor_record_offset":57964,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=16
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":58356,"parent_record_offset":58200,"ancestor_record_offset":57964,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":58896,"parent_record_offset":58200,"ancestor_record_offset":57964,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":58036,"parent_record_offset":58008,"ancestor_record_offset":57964,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61360,"parent_record_offset":60244,"ancestor_record_offset":57964,"inlinee":17396,"source_line":238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61416,"parent_record_offset":60244,"ancestor_record_offset":57964,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61560,"parent_record_offset":60244,"ancestor_record_offset":57964,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":60316,"parent_record_offset":60288,"ancestor_record_offset":57964,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":61384,"parent_record_offset":61360,"ancestor_record_offset":57964,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":59124,"parent_record_offset":59096,"ancestor_record_offset":57964,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":58852,"parent_record_offset":58828,"ancestor_record_offset":57964,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":59096,"parent_record_offset":59072,"ancestor_record_offset":57964,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":59156,"parent_record_offset":59072,"ancestor_record_offset":57964,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":59072,"parent_record_offset":59048,"ancestor_record_offset":57964,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":60288,"parent_record_offset":60244,"ancestor_record_offset":57964,"inlinee":50894,"source_line":138,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::splash::ui::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/splash.rs
//     evidence={"module":1018,"record_offset":60244,"parent_record_offset":60220,"ancestor_record_offset":57964,"inlinee":50892,"source_line":26,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::splash::ui::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6CC
// 首条位置: {"module":1018,"inlinee":50892}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61360,"parent_record_offset":60244,"ancestor_record_offset":60244,"inlinee":17396,"source_line":238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61416,"parent_record_offset":60244,"ancestor_record_offset":60244,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61560,"parent_record_offset":60244,"ancestor_record_offset":60244,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":60288,"parent_record_offset":60244,"ancestor_record_offset":60244,"inlinee":50894,"source_line":138,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":61136,"parent_record_offset":61112,"ancestor_record_offset":60244,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":60604,"parent_record_offset":60580,"ancestor_record_offset":60244,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":60688,"parent_record_offset":60664,"ancestor_record_offset":60244,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":60628,"parent_record_offset":60604,"ancestor_record_offset":60244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":60800,"parent_record_offset":60776,"ancestor_record_offset":60244,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":61048,"parent_record_offset":61024,"ancestor_record_offset":60244,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":60940,"parent_record_offset":60916,"ancestor_record_offset":60244,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":61112,"parent_record_offset":61088,"ancestor_record_offset":60244,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":60400,"parent_record_offset":60372,"ancestor_record_offset":60244,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":60372,"parent_record_offset":60344,"ancestor_record_offset":60244,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":61088,"parent_record_offset":60868,"ancestor_record_offset":60244,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":60432,"parent_record_offset":60400,"ancestor_record_offset":60244,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":61000,"parent_record_offset":60976,"ancestor_record_offset":60244,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60976,"parent_record_offset":60868,"ancestor_record_offset":60244,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60916,"parent_record_offset":60892,"ancestor_record_offset":60244,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":61296,"parent_record_offset":60508,"ancestor_record_offset":60244,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60892,"parent_record_offset":60868,"ancestor_record_offset":60244,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60484,"parent_record_offset":60460,"ancestor_record_offset":60244,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60460,"parent_record_offset":60400,"ancestor_record_offset":60244,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60508,"parent_record_offset":60484,"ancestor_record_offset":60244,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":60748,"parent_record_offset":60720,"ancestor_record_offset":60244,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":60776,"parent_record_offset":60748,"ancestor_record_offset":60244,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":60836,"parent_record_offset":60720,"ancestor_record_offset":60244,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":60720,"parent_record_offset":60508,"ancestor_record_offset":60244,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60552,"parent_record_offset":60508,"ancestor_record_offset":60244,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60868,"parent_record_offset":60508,"ancestor_record_offset":60244,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":60580,"parent_record_offset":60508,"ancestor_record_offset":60244,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=10
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":60664,"parent_record_offset":60508,"ancestor_record_offset":60244,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":61204,"parent_record_offset":60508,"ancestor_record_offset":60244,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":60344,"parent_record_offset":60316,"ancestor_record_offset":60244,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":60316,"parent_record_offset":60288,"ancestor_record_offset":60244,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":61384,"parent_record_offset":61360,"ancestor_record_offset":60244,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61492,"parent_record_offset":61464,"ancestor_record_offset":60244,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61160,"parent_record_offset":61136,"ancestor_record_offset":60244,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61464,"parent_record_offset":61440,"ancestor_record_offset":60244,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61524,"parent_record_offset":61440,"ancestor_record_offset":60244,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61440,"parent_record_offset":61416,"ancestor_record_offset":60244,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/01018.jsonl.gz"}
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
// 完整结构及原始类型记录: splash.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: splash.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: splash.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::system::system_param::Local<f32>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<f32>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>

// ------------------------------------------------------------------
// 函数: toolbar::menu::splash::ui
// 信息 ID: da5c5d7796de8908c5d50c2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f8b5c9b4c3ff32504746c808: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>)> @ 0xa976bd -> 0x1668170 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>)> > @ 0x15e1208 -> 0x1668170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::splash::ui::closure$0
// 信息 ID: 24cb55f6718f77d4eb42a49b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::splash::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::splash::ui::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0>; module=1018, inline_record=57964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::splash::ui::closure$0::closure$0
// 信息 ID: e89a618076180038f989955f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::splash::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0>; module=1018, inline_record=60244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
