// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/mei/widgets/decorate.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: af29cc7765d2951d95b9ff16a185943a4bedbaf9c77417ec1e29cee16a954fec
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorate.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 318
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::decorate::impl$0::decorate_hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":388,"rva":4435312,"va":5373144432}
// 编译实例 RVAs: 0x43ad70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: decorate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3530962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373144432,"raw_hex":"e82e1fcaff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373144477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a55310
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: decorate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6399150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373144432,"raw_hex":"e8aea46100","size":5,"soft_fail":false,"target_va":5379543824,"va":5373144669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5b0e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6422988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373144432,"raw_hex":"e8cc016200","size":5,"soft_fail":false,"target_va":5379567840,"va":5373144847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1835935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373144432,"raw_hex":"e861fce3ff","size":5,"soft_fail":false,"target_va":5371308800,"va":5373144730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29833434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373144432,"raw_hex":"e8da38c701","size":5,"soft_fail":false,"target_va":5402978912,"va":5373145473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29833390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373144432,"raw_hex":"e8ae38c701","size":5,"soft_fail":false,"target_va":5402978880,"va":5373145485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373144432,"raw_hex":"e85b390000","size":5,"soft_fail":false,"target_va":5373159392,"va":5373144704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_relative
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":1768,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":8027,"source_line":377,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":1656,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":17189,"source_line":252,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::mei
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":1712,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":17190,"source_line":281,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":1968,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":318,"record_offset":548,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":318,"record_offset":1896,"parent_record_offset":1864,"ancestor_record_offset":388,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":1864,"parent_record_offset":1828,"ancestor_record_offset":388,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":1828,"parent_record_offset":1792,"ancestor_record_offset":388,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":388,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":388,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":388,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":388,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":388,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":318,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":388,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":388,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":388,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":318,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":388,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":1928,"parent_record_offset":1828,"ancestor_record_offset":388,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":388,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":668,"parent_record_offset":632,"ancestor_record_offset":388,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":388,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":2052,"parent_record_offset":2024,"ancestor_record_offset":388,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":388,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":1380,"parent_record_offset":1160,"ancestor_record_offset":388,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":724,"parent_record_offset":692,"ancestor_record_offset":388,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":388,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1268,"parent_record_offset":1160,"ancestor_record_offset":388,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":388,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1588,"parent_record_offset":800,"ancestor_record_offset":388,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":388,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":388,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":752,"parent_record_offset":692,"ancestor_record_offset":388,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":2112,"parent_record_offset":2052,"ancestor_record_offset":388,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":388,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":1040,"parent_record_offset":1012,"ancestor_record_offset":388,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":1068,"parent_record_offset":1040,"ancestor_record_offset":388,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":1128,"parent_record_offset":1012,"ancestor_record_offset":388,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":1012,"parent_record_offset":800,"ancestor_record_offset":388,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":844,"parent_record_offset":800,"ancestor_record_offset":388,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1160,"parent_record_offset":800,"ancestor_record_offset":388,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":872,"parent_record_offset":800,"ancestor_record_offset":388,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":956,"parent_record_offset":800,"ancestor_record_offset":388,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":1496,"parent_record_offset":800,"ancestor_record_offset":388,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":632,"parent_record_offset":604,"ancestor_record_offset":388,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":1996,"parent_record_offset":1968,"ancestor_record_offset":388,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":388,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":388,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":388,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":318,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":388,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":388,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
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
// 完整结构及原始类型记录: decorate.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorate.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorate.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::decorate::impl$0::decorate_hovered
// 信息 ID: ffaefa0ef426743c483e6112
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$7 @ 0x33ec23 -> 0x43ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$7 @ 0x33ecf1 -> 0x43ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$7 @ 0x33ed88 -> 0x43ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::new_session::ui @ 0x1664f1e -> 0x43ad70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
