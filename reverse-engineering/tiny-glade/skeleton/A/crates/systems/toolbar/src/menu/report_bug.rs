// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/menu/report_bug.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 78b23256f709ffd983de09ef27a438b25a5b666f6e3a88ed7c99cafc3e91674c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: report_bug.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 319, 1018, 1020, 1061, 2425, 2468
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_input; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$5
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2468,"record_offset":116616,"rva":23203056,"va":5391912176}
// 编译实例 RVAs: 0x1620cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22298747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912176,"raw_hex":"e885bfabfe","size":5,"soft_fail":false,"target_va":5369613520,"va":5391912262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18732799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912176,"raw_hex":"e80129e2fe","size":5,"soft_fail":false,"target_va":5373179424,"va":5391912218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11066195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912176,"raw_hex":"e853dba800","size":5,"soft_fail":false,"target_va":5402978912,"va":5391912712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-15759329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912176,"raw_hex":"e81f880fff","size":5,"soft_fail":false,"target_va":5376152880,"va":5391912204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":117848,"parent_record_offset":116616,"ancestor_record_offset":116616,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":116744,"parent_record_offset":116616,"ancestor_record_offset":116616,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":117620,"parent_record_offset":117596,"ancestor_record_offset":116616,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":117088,"parent_record_offset":117064,"ancestor_record_offset":116616,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":117172,"parent_record_offset":117148,"ancestor_record_offset":116616,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":117112,"parent_record_offset":117088,"ancestor_record_offset":116616,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":117284,"parent_record_offset":117260,"ancestor_record_offset":116616,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":119136,"parent_record_offset":119112,"ancestor_record_offset":116616,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":117532,"parent_record_offset":117508,"ancestor_record_offset":116616,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":117424,"parent_record_offset":117400,"ancestor_record_offset":116616,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":117596,"parent_record_offset":117572,"ancestor_record_offset":116616,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":116856,"parent_record_offset":116828,"ancestor_record_offset":116616,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":116884,"parent_record_offset":116856,"ancestor_record_offset":116616,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":117572,"parent_record_offset":117352,"ancestor_record_offset":116616,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":116916,"parent_record_offset":116884,"ancestor_record_offset":116616,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":117484,"parent_record_offset":117460,"ancestor_record_offset":116616,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":117460,"parent_record_offset":117352,"ancestor_record_offset":116616,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":117400,"parent_record_offset":117376,"ancestor_record_offset":116616,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":117780,"parent_record_offset":116992,"ancestor_record_offset":116616,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":117376,"parent_record_offset":117352,"ancestor_record_offset":116616,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":116968,"parent_record_offset":116944,"ancestor_record_offset":116616,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":116944,"parent_record_offset":116884,"ancestor_record_offset":116616,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":116992,"parent_record_offset":116968,"ancestor_record_offset":116616,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":117232,"parent_record_offset":117204,"ancestor_record_offset":116616,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":117260,"parent_record_offset":117232,"ancestor_record_offset":116616,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":117320,"parent_record_offset":117204,"ancestor_record_offset":116616,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":117204,"parent_record_offset":116992,"ancestor_record_offset":116616,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":117036,"parent_record_offset":116992,"ancestor_record_offset":116616,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":117352,"parent_record_offset":116992,"ancestor_record_offset":116616,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":117064,"parent_record_offset":116992,"ancestor_record_offset":116616,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":117148,"parent_record_offset":116992,"ancestor_record_offset":116616,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":117688,"parent_record_offset":116992,"ancestor_record_offset":116616,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":116828,"parent_record_offset":116800,"ancestor_record_offset":116616,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":116800,"parent_record_offset":116772,"ancestor_record_offset":116616,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":116772,"parent_record_offset":116744,"ancestor_record_offset":116616,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2468,"record_offset":117872,"parent_record_offset":117848,"ancestor_record_offset":116616,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":117644,"parent_record_offset":117620,"ancestor_record_offset":116616,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$11::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC8D1
// 首条位置: {"module":1061,"inlinee":51409}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC8CF
// 首条位置: {"module":1061,"inlinee":51407}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1061,"record_offset":2516,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2484,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":2484,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":2484,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":2612,"parent_record_offset":2580,"ancestor_record_offset":2484,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1061,"record_offset":2548,"parent_record_offset":2516,"ancestor_record_offset":2484,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1061,"record_offset":2688,"parent_record_offset":2660,"ancestor_record_offset":2484,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::FeedbackReportEditorMemory::clear
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14493
// 首条位置: {"module":2425,"inlinee":83091}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2425,"record_offset":2440,"parent_record_offset":2396,"ancestor_record_offset":2396,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::report_bug::TextEditMemory::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":2468,"parent_record_offset":2396,"ancestor_record_offset":2396,"inlinee":83093,"source_line":354,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2425,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2396,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":3100,"parent_record_offset":2468,"ancestor_record_offset":2396,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":2640,"parent_record_offset":2580,"ancestor_record_offset":2396,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":3004,"parent_record_offset":2468,"ancestor_record_offset":2396,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2924,"parent_record_offset":2896,"ancestor_record_offset":2396,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3388,"parent_record_offset":3200,"ancestor_record_offset":2396,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":2396,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3480,"parent_record_offset":3448,"ancestor_record_offset":2396,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":2396,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2612,"parent_record_offset":2580,"ancestor_record_offset":2396,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2396,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":2396,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":2396,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":2425,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":2396,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":2425,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":2396,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":2512,"parent_record_offset":2468,"ancestor_record_offset":2396,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":2676,"parent_record_offset":2512,"ancestor_record_offset":2396,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":3044,"parent_record_offset":2468,"ancestor_record_offset":2396,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":3160,"parent_record_offset":3128,"ancestor_record_offset":2396,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":3200,"parent_record_offset":3160,"ancestor_record_offset":2396,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":2896,"parent_record_offset":2704,"ancestor_record_offset":2396,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":2396,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":2425,"record_offset":2548,"parent_record_offset":2512,"ancestor_record_offset":2396,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":2425,"record_offset":3448,"parent_record_offset":3160,"ancestor_record_offset":2396,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":2425,"record_offset":2740,"parent_record_offset":2704,"ancestor_record_offset":2396,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2425,"record_offset":812,"rva":22983552,"va":5391692672}
// 编译实例 RVAs: 0x15eb380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22075885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e81326affe","size":5,"soft_fail":false,"target_va":5369617648,"va":5391693528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22060912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e89060affe","size":5,"soft_fail":false,"target_va":5369632448,"va":5391693355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22023388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e824f3affe","size":5,"soft_fail":false,"target_va":5369673744,"va":5391697127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19447896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8a83fd7fe","size":5,"soft_fail":false,"target_va":5372249392,"va":5391697283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19376440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8c856d8fe","size":5,"soft_fail":false,"target_va":5372320512,"va":5391696947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18945653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e88be9defe","size":5,"soft_fail":false,"target_va":5372749696,"va":5391695344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=19
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18943347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e88df2defe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391693054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18539252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e80c1de5fe","size":5,"soft_fail":false,"target_va":5373154400,"va":5391693647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17297456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8d00ff8fe","size":5,"soft_fail":false,"target_va":5374398944,"va":5391696395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f2bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15702499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e81d6610ff","size":5,"soft_fail":false,"target_va":5375994864,"va":5391697358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a62320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12096166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e85a6d47ff","size":5,"soft_fail":false,"target_va":5379597088,"va":5391693249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a63a90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12093528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8a87747ff","size":5,"soft_fail":false,"target_va":5379603088,"va":5391696611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a64e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12089380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8dc8747ff","size":5,"soft_fail":false,"target_va":5379608096,"va":5391697471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a94a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11891854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8728b4aff","size":5,"soft_fail":false,"target_va":5379803680,"va":5391695529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141619150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t186775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e897d90200","size":5,"soft_fail":false,"target_va":5391880528,"va":5391693748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161aa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t193027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e803f20200","size":5,"soft_fail":false,"target_va":5391886976,"va":5391693944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161acd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t192953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8b9f10200","size":5,"soft_fail":false,"target_va":5391887568,"va":5391694610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161ba40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t196144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e830fe0200","size":5,"soft_fail":false,"target_va":5391891008,"va":5391694859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t198033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e891050300","size":5,"soft_fail":false,"target_va":5391891840,"va":5391693802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161c260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t199217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8310a0300","size":5,"soft_fail":false,"target_va":5391893088,"va":5391693866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161cd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t202033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e831150300","size":5,"soft_fail":false,"target_va":5391895872,"va":5391693834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161d260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t203511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8f71a0300","size":5,"soft_fail":false,"target_va":5391897184,"va":5391693668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161dd20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t205942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e876240300","size":5,"soft_fail":false,"target_va":5391899936,"va":5391693989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161df70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t206182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e866250300","size":5,"soft_fail":false,"target_va":5391900528,"va":5391694341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11897534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8be8ab500","size":5,"soft_fail":false,"target_va":5403593360,"va":5391695821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11281686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e81625ac00","size":5,"soft_fail":false,"target_va":5402978144,"va":5391696453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-20384790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8eaf3c8fe","size":5,"soft_fail":false,"target_va":5371308800,"va":5391693585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11283853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e88d2dac00","size":5,"soft_fail":false,"target_va":5402980128,"va":5391696270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::feedback_report_resources::FeedbackReportBgTexture::is_available
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17501880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e848f1f4fe","size":5,"soft_fail":false,"target_va":5374191552,"va":5391693427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::feedback_report_resources::FeedbackReportResources::clear_screenshot
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17501385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e837f3f4fe","size":5,"soft_fail":false,"target_va":5374191760,"va":5391693140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::feedback_report_resources::FeedbackReportTags::to_list
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17503832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8a8e9f4fe","size":5,"soft_fail":false,"target_va":5374191568,"va":5391695395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::PrevEscMenuState::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17295768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e86816f8fe","size":5,"soft_fail":false,"target_va":5374399136,"va":5391694899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17440031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8e1e2f5fe","size":5,"soft_fail":false,"target_va":5374255184,"va":5391695210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::session_file_list
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17439658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e856e4f5fe","size":5,"soft_fail":false,"target_va":5374255568,"va":5391695221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14341394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8ee2a25ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5391695165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11274926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8ae0aac00","size":5,"soft_fail":false,"target_va":5402968992,"va":5391694061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11276669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e87d11ac00","size":5,"soft_fail":false,"target_va":5402970800,"va":5391694126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11279142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8261bac00","size":5,"soft_fail":false,"target_va":5402975328,"va":5391696181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11279328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8e01bac00","size":5,"soft_fail":false,"target_va":5402975504,"va":5391696171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11279411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8331cac00","size":5,"soft_fail":false,"target_va":5402975696,"va":5391696280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-18534165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391692672,"raw_hex":"e8eb30e5fe","size":5,"soft_fail":false,"target_va":5373159392,"va":5391693552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] country_core::resources::feedback_report_resources::FeedbackReportResources::reset_choices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-17501604","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5391692672,"raw_hex":"e95cf2f4fe","size":5,"soft_fail":false,"target_va":5374191744,"va":5391693343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":2425,"record_offset":6876,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2425,"record_offset":4132,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2425,"record_offset":2336,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2425,"record_offset":1008,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2425,"record_offset":8416,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2425,"record_offset":1624,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2425,"record_offset":6500,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2425,"record_offset":6416,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2425,"record_offset":6444,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":4188,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":4160,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":5856,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":8101,"source_line":1889,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":8176,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":12684,"source_line":1444,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":6092,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":1872,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":9644,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2425,"record_offset":6472,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2425,"record_offset":6332,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":2425,"record_offset":7372,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":10619,"source_line":2217,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2425,"record_offset":3908,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":7454,"source_line":349,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::report_bug::FeedbackReportEditorMemory::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":2396,"parent_record_offset":812,"ancestor_record_offset":812,"inlinee":83091,"source_line":56,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":812,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":812,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":812,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":7136,"parent_record_offset":7108,"ancestor_record_offset":812,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":2028,"parent_record_offset":1972,"ancestor_record_offset":812,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":812,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":4060,"parent_record_offset":4028,"ancestor_record_offset":812,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":812,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":7080,"parent_record_offset":7052,"ancestor_record_offset":812,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":7256,"parent_record_offset":7080,"ancestor_record_offset":812,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":1972,"parent_record_offset":1944,"ancestor_record_offset":812,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2425,"record_offset":7016,"parent_record_offset":6988,"ancestor_record_offset":812,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2425,"record_offset":6988,"parent_record_offset":6960,"ancestor_record_offset":812,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2425,"record_offset":6932,"parent_record_offset":6904,"ancestor_record_offset":812,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2425,"record_offset":6960,"parent_record_offset":6932,"ancestor_record_offset":812,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2425,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":812,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2425,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":812,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2425,"record_offset":2440,"parent_record_offset":2396,"ancestor_record_offset":812,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2425,"record_offset":4028,"parent_record_offset":3996,"ancestor_record_offset":812,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2425,"record_offset":3996,"parent_record_offset":3932,"ancestor_record_offset":812,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2425,"record_offset":7456,"parent_record_offset":7408,"ancestor_record_offset":812,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2425,"record_offset":7052,"parent_record_offset":7016,"ancestor_record_offset":812,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2425,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":812,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2425,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":812,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2425,"record_offset":8448,"parent_record_offset":8416,"ancestor_record_offset":812,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2425,"record_offset":1132,"parent_record_offset":1096,"ancestor_record_offset":812,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2425,"record_offset":8508,"parent_record_offset":8416,"ancestor_record_offset":812,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2425,"record_offset":1096,"parent_record_offset":1044,"ancestor_record_offset":812,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2425,"record_offset":1664,"parent_record_offset":1624,"ancestor_record_offset":812,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":2425,"record_offset":1044,"parent_record_offset":1008,"ancestor_record_offset":812,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2425,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":812,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2425,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":812,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2425,"record_offset":1704,"parent_record_offset":1664,"ancestor_record_offset":812,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2425,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":812,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2425,"record_offset":1200,"parent_record_offset":1132,"ancestor_record_offset":812,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2425,"record_offset":9304,"parent_record_offset":9276,"ancestor_record_offset":812,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":1740,"parent_record_offset":1704,"ancestor_record_offset":812,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":1796,"parent_record_offset":1704,"ancestor_record_offset":812,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":4440,"parent_record_offset":4216,"ancestor_record_offset":812,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":1828,"parent_record_offset":1664,"ancestor_record_offset":812,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":6048,"parent_record_offset":5932,"ancestor_record_offset":812,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2425,"record_offset":7292,"parent_record_offset":7016,"ancestor_record_offset":812,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2425,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":812,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":812,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":7212,"parent_record_offset":7184,"ancestor_record_offset":812,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":4092,"parent_record_offset":3996,"ancestor_record_offset":812,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":4376,"parent_record_offset":4292,"ancestor_record_offset":812,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":2640,"parent_record_offset":2580,"ancestor_record_offset":812,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":3004,"parent_record_offset":2468,"ancestor_record_offset":812,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2425,"record_offset":1232,"parent_record_offset":1132,"ancestor_record_offset":812,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2425,"record_offset":1332,"parent_record_offset":1132,"ancestor_record_offset":812,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2425,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":812,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2425,"record_offset":1272,"parent_record_offset":1232,"ancestor_record_offset":812,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2425,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":812,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2924,"parent_record_offset":2896,"ancestor_record_offset":812,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3388,"parent_record_offset":3200,"ancestor_record_offset":812,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":812,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":4320,"parent_record_offset":4292,"ancestor_record_offset":812,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3480,"parent_record_offset":3448,"ancestor_record_offset":812,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":5988,"parent_record_offset":5960,"ancestor_record_offset":812,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":812,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":812,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2612,"parent_record_offset":2580,"ancestor_record_offset":812,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":812,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":812,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":812,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":812,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":2425,"record_offset":5892,"parent_record_offset":5856,"ancestor_record_offset":812,"inlinee":51315,"source_line":92,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$29::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":2425,"record_offset":5932,"parent_record_offset":5892,"ancestor_record_offset":812,"inlinee":51317,"source_line":211,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":2425,"record_offset":4268,"parent_record_offset":4216,"ancestor_record_offset":812,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":2425,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":812,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2425,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":812,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2425,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":812,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":2425,"record_offset":7756,"parent_record_offset":7728,"ancestor_record_offset":812,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":2425,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":812,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":2425,"record_offset":7408,"parent_record_offset":7372,"ancestor_record_offset":812,"inlinee":10184,"source_line":2185,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":2425,"record_offset":7728,"parent_record_offset":7408,"ancestor_record_offset":812,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":2425,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":812,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":2425,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":812,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":2512,"parent_record_offset":2468,"ancestor_record_offset":812,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":2676,"parent_record_offset":2512,"ancestor_record_offset":812,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":3044,"parent_record_offset":2468,"ancestor_record_offset":812,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":3160,"parent_record_offset":3128,"ancestor_record_offset":812,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":3200,"parent_record_offset":3160,"ancestor_record_offset":812,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":2896,"parent_record_offset":2704,"ancestor_record_offset":812,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":812,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":2425,"record_offset":2548,"parent_record_offset":2512,"ancestor_record_offset":812,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":2425,"record_offset":3448,"parent_record_offset":3160,"ancestor_record_offset":812,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":2425,"record_offset":2740,"parent_record_offset":2704,"ancestor_record_offset":812,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2425,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":812,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2425,"record_offset":3968,"parent_record_offset":3932,"ancestor_record_offset":812,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::TextEditMemory::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":2468,"parent_record_offset":2396,"ancestor_record_offset":812,"inlinee":83093,"source_line":354,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::ui::closure$7
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":4216,"parent_record_offset":4188,"ancestor_record_offset":812,"inlinee":83095,"source_line":195,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::ui::closure$8
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":6532,"parent_record_offset":6500,"ancestor_record_offset":812,"inlinee":83096,"source_line":203,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::ui::closure$9
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":8212,"parent_record_offset":8176,"ancestor_record_offset":812,"inlinee":83097,"source_line":218,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14694
// 首条位置: {"module":2468,"inlinee":83604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":71240,"parent_record_offset":70884,"ancestor_record_offset":70884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":71120,"parent_record_offset":70884,"ancestor_record_offset":70884,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":71180,"parent_record_offset":70884,"ancestor_record_offset":70884,"inlinee":17225,"source_line":220,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":70920,"parent_record_offset":70884,"ancestor_record_offset":70884,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":71392,"parent_record_offset":71368,"ancestor_record_offset":70884,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":71368,"parent_record_offset":71312,"ancestor_record_offset":70884,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":71340,"parent_record_offset":71312,"ancestor_record_offset":70884,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":71312,"parent_record_offset":71288,"ancestor_record_offset":70884,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":71264,"parent_record_offset":71240,"ancestor_record_offset":70884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":71028,"parent_record_offset":70972,"ancestor_record_offset":70884,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":70972,"parent_record_offset":70920,"ancestor_record_offset":70884,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":71148,"parent_record_offset":71120,"ancestor_record_offset":70884,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":70944,"parent_record_offset":70920,"ancestor_record_offset":70884,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":71000,"parent_record_offset":70972,"ancestor_record_offset":70884,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":71052,"parent_record_offset":71028,"ancestor_record_offset":70884,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":71084,"parent_record_offset":70972,"ancestor_record_offset":70884,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14684
// 首条位置: {"module":2468,"inlinee":83588}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2468,"record_offset":21744,"parent_record_offset":21704,"ancestor_record_offset":21704,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::report_bug::TextEditView::text
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2468,"record_offset":21840,"parent_record_offset":21704,"ancestor_record_offset":21704,"inlinee":83107,"source_line":337,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":22008,"parent_record_offset":21984,"ancestor_record_offset":21704,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":21984,"parent_record_offset":21928,"ancestor_record_offset":21704,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":21956,"parent_record_offset":21928,"ancestor_record_offset":21704,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":21928,"parent_record_offset":21900,"ancestor_record_offset":21704,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":21876,"parent_record_offset":21840,"ancestor_record_offset":21704,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2468,"record_offset":21780,"parent_record_offset":21744,"ancestor_record_offset":21704,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2468,"record_offset":21804,"parent_record_offset":21780,"ancestor_record_offset":21704,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14690
// 首条位置: {"module":2468,"inlinee":83600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2468,"record_offset":53908,"parent_record_offset":53872,"ancestor_record_offset":53872,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":54192,"parent_record_offset":53872,"ancestor_record_offset":53872,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":55500,"parent_record_offset":55236,"ancestor_record_offset":55236,"inlinee":17226,"source_line":226,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":53992,"parent_record_offset":53872,"ancestor_record_offset":53872,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":54344,"parent_record_offset":54320,"ancestor_record_offset":53872,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":54320,"parent_record_offset":54264,"ancestor_record_offset":53872,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2468,"record_offset":53956,"parent_record_offset":53932,"ancestor_record_offset":53872,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":54292,"parent_record_offset":54264,"ancestor_record_offset":53872,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":54264,"parent_record_offset":54240,"ancestor_record_offset":53872,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":53932,"parent_record_offset":53908,"ancestor_record_offset":53872,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":54216,"parent_record_offset":54192,"ancestor_record_offset":53872,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":54100,"parent_record_offset":54044,"ancestor_record_offset":53872,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":54044,"parent_record_offset":53992,"ancestor_record_offset":53872,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":55528,"parent_record_offset":55500,"ancestor_record_offset":55236,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":54016,"parent_record_offset":53992,"ancestor_record_offset":53872,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":54072,"parent_record_offset":54044,"ancestor_record_offset":53872,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":54124,"parent_record_offset":54100,"ancestor_record_offset":53872,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":54156,"parent_record_offset":54044,"ancestor_record_offset":53872,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14693
// 首条位置: {"module":2468,"inlinee":83603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2468,"record_offset":67480,"parent_record_offset":67444,"ancestor_record_offset":67444,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2468,"record_offset":67508,"parent_record_offset":67480,"ancestor_record_offset":67444,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2468,"record_offset":115820,"rva":23202720,"va":5391911840}
// 编译实例 RVAs: 0x1620ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19162375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391911840,"raw_hex":"e8f99adbfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391912082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b88800
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5667785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391911840,"raw_hex":"e8c97b5600","size":5,"soft_fail":false,"target_va":5397579776,"va":5391911986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-15759034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391911840,"raw_hex":"e846890fff","size":5,"soft_fail":false,"target_va":5376152880,"va":5391911909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2468,"record_offset":116008,"parent_record_offset":115820,"ancestor_record_offset":115820,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":116148,"parent_record_offset":115820,"ancestor_record_offset":115820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::HintTracker::text
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":2468,"record_offset":115980,"parent_record_offset":115820,"ancestor_record_offset":115820,"inlinee":17504,"source_line":158,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":2468,"record_offset":115956,"parent_record_offset":115820,"ancestor_record_offset":115820,"inlinee":12065,"source_line":152,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":116328,"parent_record_offset":116304,"ancestor_record_offset":115820,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":116304,"parent_record_offset":116248,"ancestor_record_offset":115820,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":116276,"parent_record_offset":116248,"ancestor_record_offset":115820,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":116248,"parent_record_offset":116220,"ancestor_record_offset":115820,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2468,"record_offset":116032,"parent_record_offset":116008,"ancestor_record_offset":115820,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":116172,"parent_record_offset":116148,"ancestor_record_offset":115820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$6
// 记录形式: compiled_procedure; instances=1
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2468,"record_offset":119180,"rva":23203600,"va":5391912720}
// 编译实例 RVAs: 0x1620f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22298592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e820c0abfe","size":5,"soft_fail":false,"target_va":5369616080,"va":5391914667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22298213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e89bc1abfe","size":5,"soft_fail":false,"target_va":5369616480,"va":5391914688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22296979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e86dc6abfe","size":5,"soft_fail":false,"target_va":5369617648,"va":5391914622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19165155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e81d90dbfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391914862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12351475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e80d8843ff","size":5,"soft_fail":false,"target_va":5379563248,"va":5391914718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12342592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e8c0aa43ff","size":5,"soft_fail":false,"target_va":5379572176,"va":5391914763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12341701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e83bae43ff","size":5,"soft_fail":false,"target_va":5379573040,"va":5391914736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11680239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e8ef39b200","size":5,"soft_fail":false,"target_va":5403593360,"va":5391913116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11005101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e8adeca700","size":5,"soft_fail":false,"target_va":5402919072,"va":5391913966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11064870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e826d6a800","size":5,"soft_fail":false,"target_va":5402979344,"va":5391914469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::StrSearcher::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14319291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e8458125ff","size":5,"soft_fail":false,"target_va":5377593648,"va":5391912934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11065679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e84fd9a800","size":5,"soft_fail":false,"target_va":5402980656,"va":5391914972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::keycode_to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17826494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e842fdeffe","size":5,"soft_fail":false,"target_va":5374086352,"va":5391912841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15759902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e8e2850fff","size":5,"soft_fail":false,"target_va":5376152880,"va":5391912777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15870144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e840d70dff","size":5,"soft_fail":false,"target_va":5376044672,"va":5391914811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15870251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e8d5d60dff","size":5,"soft_fail":false,"target_va":5376044544,"va":5391914790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18753840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391912720,"raw_hex":"e8d0d6e1fe","size":5,"soft_fail":false,"target_va":5373160864,"va":5391914699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":2468,"record_offset":119420,"parent_record_offset":119180,"ancestor_record_offset":119180,"inlinee":14514,"source_line":271,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":119332,"parent_record_offset":119180,"ancestor_record_offset":119180,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":123024,"parent_record_offset":119180,"ancestor_record_offset":119180,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":122904,"parent_record_offset":119180,"ancestor_record_offset":119180,"inlinee":17223,"source_line":208,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":122964,"parent_record_offset":119180,"ancestor_record_offset":119180,"inlinee":17226,"source_line":226,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":122844,"parent_record_offset":119180,"ancestor_record_offset":119180,"inlinee":17222,"source_line":202,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":122784,"parent_record_offset":119180,"ancestor_record_offset":119180,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":122584,"parent_record_offset":119180,"ancestor_record_offset":119180,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":122532,"parent_record_offset":122508,"ancestor_record_offset":119180,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":122508,"parent_record_offset":122452,"ancestor_record_offset":119180,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":119892,"parent_record_offset":119864,"ancestor_record_offset":119180,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":122480,"parent_record_offset":122452,"ancestor_record_offset":119180,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":119864,"parent_record_offset":119832,"ancestor_record_offset":119180,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":119772,"parent_record_offset":119728,"ancestor_record_offset":119180,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":119832,"parent_record_offset":119804,"ancestor_record_offset":119180,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":122452,"parent_record_offset":122424,"ancestor_record_offset":119180,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2468,"record_offset":119504,"parent_record_offset":119420,"ancestor_record_offset":119180,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2468,"record_offset":119588,"parent_record_offset":119420,"ancestor_record_offset":119180,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2468,"record_offset":119684,"parent_record_offset":119652,"ancestor_record_offset":119180,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2468,"record_offset":121848,"parent_record_offset":121656,"ancestor_record_offset":119180,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2468,"record_offset":119620,"parent_record_offset":119588,"ancestor_record_offset":119180,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2468,"record_offset":119944,"parent_record_offset":119684,"ancestor_record_offset":119180,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2468,"record_offset":119804,"parent_record_offset":119684,"ancestor_record_offset":119180,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2468,"record_offset":119652,"parent_record_offset":119620,"ancestor_record_offset":119180,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\methods.rs
//     evidence={"module":2468,"record_offset":121428,"parent_record_offset":120764,"ancestor_record_offset":119180,"inlinee":5382,"source_line":612,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\methods.rs
//     evidence={"module":2468,"record_offset":121452,"parent_record_offset":121428,"ancestor_record_offset":119180,"inlinee":5096,"source_line":1741,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":120436,"parent_record_offset":120400,"ancestor_record_offset":119180,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2468,"record_offset":120540,"parent_record_offset":120140,"ancestor_record_offset":119180,"inlinee":7276,"source_line":33,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2468,"record_offset":120688,"parent_record_offset":120612,"ancestor_record_offset":119180,"inlinee":14451,"source_line":213,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2468,"record_offset":120472,"parent_record_offset":120400,"ancestor_record_offset":119180,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2468,"record_offset":120400,"parent_record_offset":120368,"ancestor_record_offset":119180,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2468,"record_offset":120612,"parent_record_offset":120576,"ancestor_record_offset":119180,"inlinee":7279,"source_line":798,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2468,"record_offset":120368,"parent_record_offset":120140,"ancestor_record_offset":119180,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2468,"record_offset":120576,"parent_record_offset":120540,"ancestor_record_offset":119180,"inlinee":7278,"source_line":967,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2468,"record_offset":120496,"parent_record_offset":120472,"ancestor_record_offset":119180,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2468,"record_offset":120712,"parent_record_offset":120688,"ancestor_record_offset":119180,"inlinee":14452,"source_line":664,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":2468,"record_offset":121068,"parent_record_offset":120956,"ancestor_record_offset":119180,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":119384,"parent_record_offset":119360,"ancestor_record_offset":119180,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":122376,"parent_record_offset":119420,"ancestor_record_offset":119180,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":119728,"parent_record_offset":119684,"ancestor_record_offset":119180,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2468,"record_offset":121272,"parent_record_offset":121228,"ancestor_record_offset":119180,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2468,"record_offset":120280,"parent_record_offset":120140,"ancestor_record_offset":119180,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2468,"record_offset":120320,"parent_record_offset":120280,"ancestor_record_offset":119180,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2468,"record_offset":121228,"parent_record_offset":121156,"ancestor_record_offset":119180,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2468,"record_offset":121504,"parent_record_offset":119420,"ancestor_record_offset":119180,"inlinee":12626,"source_line":507,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2468,"record_offset":120956,"parent_record_offset":120928,"ancestor_record_offset":119180,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2468,"record_offset":119532,"parent_record_offset":119420,"ancestor_record_offset":119180,"inlinee":14515,"source_line":1882,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2468,"record_offset":120028,"parent_record_offset":119988,"ancestor_record_offset":119180,"inlinee":14519,"source_line":1054,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2468,"record_offset":121116,"parent_record_offset":120764,"ancestor_record_offset":119180,"inlinee":4928,"source_line":42,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2468,"record_offset":119988,"parent_record_offset":119420,"ancestor_record_offset":119180,"inlinee":14517,"source_line":506,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2468,"record_offset":120140,"parent_record_offset":120064,"ancestor_record_offset":119180,"inlinee":14545,"source_line":1427,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2468,"record_offset":119556,"parent_record_offset":119532,"ancestor_record_offset":119180,"inlinee":7267,"source_line":956,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2468,"record_offset":120764,"parent_record_offset":120064,"ancestor_record_offset":119180,"inlinee":14454,"source_line":1085,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2468,"record_offset":120064,"parent_record_offset":120028,"ancestor_record_offset":119180,"inlinee":14546,"source_line":1137,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2468,"record_offset":120928,"parent_record_offset":120892,"ancestor_record_offset":119180,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2468,"record_offset":120892,"parent_record_offset":120856,"ancestor_record_offset":119180,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2468,"record_offset":120856,"parent_record_offset":120764,"ancestor_record_offset":119180,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2468,"record_offset":121528,"parent_record_offset":121504,"ancestor_record_offset":119180,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2468,"record_offset":121156,"parent_record_offset":121116,"ancestor_record_offset":119180,"inlinee":4930,"source_line":36,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2468,"record_offset":121336,"parent_record_offset":121156,"ancestor_record_offset":119180,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2468,"record_offset":121308,"parent_record_offset":121156,"ancestor_record_offset":119180,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":122692,"parent_record_offset":122636,"ancestor_record_offset":119180,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":122636,"parent_record_offset":122584,"ancestor_record_offset":119180,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":122812,"parent_record_offset":122784,"ancestor_record_offset":119180,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":122608,"parent_record_offset":122584,"ancestor_record_offset":119180,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":122664,"parent_record_offset":122636,"ancestor_record_offset":119180,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":122716,"parent_record_offset":122692,"ancestor_record_offset":119180,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":122748,"parent_record_offset":122636,"ancestor_record_offset":119180,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::ui::closure$6::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2468,"record_offset":119360,"parent_record_offset":119332,"ancestor_record_offset":119180,"inlinee":83615,"source_line":178,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$6::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 178
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1469F
// 首条位置: {"module":2468,"inlinee":83615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":119384,"parent_record_offset":119360,"ancestor_record_offset":119360,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14497
// 首条位置: {"module":2425,"inlinee":83095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2425,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":4440,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":4556,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":2425,"record_offset":4268,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2425,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":4216,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":4216,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":4520,"parent_record_offset":4464,"ancestor_record_offset":4216,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":4376,"parent_record_offset":4292,"ancestor_record_offset":4216,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":4400,"parent_record_offset":4376,"ancestor_record_offset":4216,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":4636,"parent_record_offset":4604,"ancestor_record_offset":4216,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":4320,"parent_record_offset":4292,"ancestor_record_offset":4216,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":4660,"parent_record_offset":4636,"ancestor_record_offset":4216,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":4216,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":2425,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":4216,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2425,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":4216,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2425,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":4216,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14498
// 首条位置: {"module":2425,"inlinee":83096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":6568,"parent_record_offset":6532,"ancestor_record_offset":6532,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":6532,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":6784,"parent_record_offset":6728,"ancestor_record_offset":6532,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":6532,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":6532,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":6532,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14499
// 首条位置: {"module":2425,"inlinee":83097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1468F
// 首条位置: {"module":2468,"inlinee":83599}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":51588,"parent_record_offset":51564,"ancestor_record_offset":51564,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":51692,"parent_record_offset":51640,"ancestor_record_offset":51564,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":51640,"parent_record_offset":51588,"ancestor_record_offset":51564,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":51612,"parent_record_offset":51588,"ancestor_record_offset":51564,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":51664,"parent_record_offset":51640,"ancestor_record_offset":51564,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":51716,"parent_record_offset":51692,"ancestor_record_offset":51564,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":52404,"parent_record_offset":52320,"ancestor_record_offset":52272,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14696
// 首条位置: {"module":2468,"inlinee":83606}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":82944,"parent_record_offset":82912,"ancestor_record_offset":82912,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":79948,"parent_record_offset":79916,"ancestor_record_offset":79916,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":80148,"parent_record_offset":79916,"ancestor_record_offset":79916,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":81024,"parent_record_offset":81000,"ancestor_record_offset":79916,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":80492,"parent_record_offset":80468,"ancestor_record_offset":79916,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":80576,"parent_record_offset":80552,"ancestor_record_offset":79916,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":80516,"parent_record_offset":80492,"ancestor_record_offset":79916,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":80688,"parent_record_offset":80664,"ancestor_record_offset":79916,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":83056,"parent_record_offset":83032,"ancestor_record_offset":82912,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":80936,"parent_record_offset":80912,"ancestor_record_offset":79916,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":80828,"parent_record_offset":80804,"ancestor_record_offset":79916,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":81000,"parent_record_offset":80976,"ancestor_record_offset":79916,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":80260,"parent_record_offset":80232,"ancestor_record_offset":79916,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":80288,"parent_record_offset":80260,"ancestor_record_offset":79916,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":80976,"parent_record_offset":80756,"ancestor_record_offset":79916,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":80320,"parent_record_offset":80288,"ancestor_record_offset":79916,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80888,"parent_record_offset":80864,"ancestor_record_offset":79916,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80864,"parent_record_offset":80756,"ancestor_record_offset":79916,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80804,"parent_record_offset":80780,"ancestor_record_offset":79916,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":81184,"parent_record_offset":80396,"ancestor_record_offset":79916,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80780,"parent_record_offset":80756,"ancestor_record_offset":79916,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80372,"parent_record_offset":80348,"ancestor_record_offset":79916,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80348,"parent_record_offset":80288,"ancestor_record_offset":79916,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80396,"parent_record_offset":80372,"ancestor_record_offset":79916,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":80636,"parent_record_offset":80608,"ancestor_record_offset":79916,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":80664,"parent_record_offset":80636,"ancestor_record_offset":79916,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":80724,"parent_record_offset":80608,"ancestor_record_offset":79916,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":80608,"parent_record_offset":80396,"ancestor_record_offset":79916,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80440,"parent_record_offset":80396,"ancestor_record_offset":79916,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":80756,"parent_record_offset":80396,"ancestor_record_offset":79916,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":80468,"parent_record_offset":80396,"ancestor_record_offset":79916,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":80552,"parent_record_offset":80396,"ancestor_record_offset":79916,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":81092,"parent_record_offset":80396,"ancestor_record_offset":79916,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":80232,"parent_record_offset":80204,"ancestor_record_offset":79916,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":80204,"parent_record_offset":80176,"ancestor_record_offset":79916,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":80056,"parent_record_offset":80000,"ancestor_record_offset":79916,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":80000,"parent_record_offset":79948,"ancestor_record_offset":79916,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":80176,"parent_record_offset":80148,"ancestor_record_offset":79916,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2468,"record_offset":82980,"parent_record_offset":82944,"ancestor_record_offset":82912,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":79972,"parent_record_offset":79948,"ancestor_record_offset":79916,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":81048,"parent_record_offset":81024,"ancestor_record_offset":79916,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":80028,"parent_record_offset":80000,"ancestor_record_offset":79916,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":80080,"parent_record_offset":80056,"ancestor_record_offset":79916,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":80112,"parent_record_offset":80000,"ancestor_record_offset":79916,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$12
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1468D
// 首条位置: {"module":2468,"inlinee":83597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":42264,"parent_record_offset":41828,"ancestor_record_offset":41828,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":42376,"parent_record_offset":41828,"ancestor_record_offset":41828,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":41864,"parent_record_offset":41828,"ancestor_record_offset":41828,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":43184,"parent_record_offset":43160,"ancestor_record_offset":41828,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":42652,"parent_record_offset":42628,"ancestor_record_offset":41828,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":42736,"parent_record_offset":42712,"ancestor_record_offset":41828,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":42676,"parent_record_offset":42652,"ancestor_record_offset":41828,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":41828,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":45356,"parent_record_offset":45332,"ancestor_record_offset":45304,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":43096,"parent_record_offset":43072,"ancestor_record_offset":41828,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":42988,"parent_record_offset":42964,"ancestor_record_offset":41828,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":43160,"parent_record_offset":43136,"ancestor_record_offset":41828,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":42448,"parent_record_offset":42424,"ancestor_record_offset":41828,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":42424,"parent_record_offset":42400,"ancestor_record_offset":41828,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":43136,"parent_record_offset":42916,"ancestor_record_offset":41828,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":42480,"parent_record_offset":42448,"ancestor_record_offset":41828,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":43048,"parent_record_offset":43024,"ancestor_record_offset":41828,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":43024,"parent_record_offset":42916,"ancestor_record_offset":41828,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":42964,"parent_record_offset":42940,"ancestor_record_offset":41828,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":43344,"parent_record_offset":42556,"ancestor_record_offset":41828,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":42940,"parent_record_offset":42916,"ancestor_record_offset":41828,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":42532,"parent_record_offset":42508,"ancestor_record_offset":41828,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":42508,"parent_record_offset":42448,"ancestor_record_offset":41828,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":42556,"parent_record_offset":42532,"ancestor_record_offset":41828,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":42796,"parent_record_offset":42768,"ancestor_record_offset":41828,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":42824,"parent_record_offset":42796,"ancestor_record_offset":41828,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":42884,"parent_record_offset":42768,"ancestor_record_offset":41828,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":42768,"parent_record_offset":42556,"ancestor_record_offset":41828,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":42600,"parent_record_offset":42556,"ancestor_record_offset":41828,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":42916,"parent_record_offset":42556,"ancestor_record_offset":41828,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":42628,"parent_record_offset":42556,"ancestor_record_offset":41828,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":42712,"parent_record_offset":42556,"ancestor_record_offset":41828,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":43252,"parent_record_offset":42556,"ancestor_record_offset":41828,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":42400,"parent_record_offset":42376,"ancestor_record_offset":41828,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":41972,"parent_record_offset":41916,"ancestor_record_offset":41828,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":41916,"parent_record_offset":41864,"ancestor_record_offset":41828,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":41888,"parent_record_offset":41864,"ancestor_record_offset":41828,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":43208,"parent_record_offset":43184,"ancestor_record_offset":41828,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":41944,"parent_record_offset":41916,"ancestor_record_offset":41828,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":41996,"parent_record_offset":41972,"ancestor_record_offset":41828,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":42288,"parent_record_offset":42264,"ancestor_record_offset":41828,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":42028,"parent_record_offset":41916,"ancestor_record_offset":41828,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::ui::closure$12::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6C8
// 首条位置: {"module":1018,"inlinee":50888}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":57448,"parent_record_offset":55084,"ancestor_record_offset":55084,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":56232,"parent_record_offset":55084,"ancestor_record_offset":55084,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":55128,"parent_record_offset":55084,"ancestor_record_offset":55084,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":57628,"parent_record_offset":57604,"ancestor_record_offset":55084,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":57604,"parent_record_offset":57548,"ancestor_record_offset":55084,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":57576,"parent_record_offset":57548,"ancestor_record_offset":55084,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":57548,"parent_record_offset":57520,"ancestor_record_offset":55084,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":56004,"parent_record_offset":55980,"ancestor_record_offset":55084,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":55472,"parent_record_offset":55448,"ancestor_record_offset":55084,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":55556,"parent_record_offset":55532,"ancestor_record_offset":55084,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":55496,"parent_record_offset":55472,"ancestor_record_offset":55084,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":55668,"parent_record_offset":55644,"ancestor_record_offset":55084,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":57752,"parent_record_offset":57728,"ancestor_record_offset":55084,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":57472,"parent_record_offset":57448,"ancestor_record_offset":55084,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":55916,"parent_record_offset":55892,"ancestor_record_offset":55084,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":55808,"parent_record_offset":55784,"ancestor_record_offset":55084,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":55980,"parent_record_offset":55956,"ancestor_record_offset":55084,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":55240,"parent_record_offset":55212,"ancestor_record_offset":55084,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":55268,"parent_record_offset":55240,"ancestor_record_offset":55084,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":55956,"parent_record_offset":55736,"ancestor_record_offset":55084,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":55300,"parent_record_offset":55268,"ancestor_record_offset":55084,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55868,"parent_record_offset":55844,"ancestor_record_offset":55084,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55844,"parent_record_offset":55736,"ancestor_record_offset":55084,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55784,"parent_record_offset":55760,"ancestor_record_offset":55084,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":56164,"parent_record_offset":55376,"ancestor_record_offset":55084,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55760,"parent_record_offset":55736,"ancestor_record_offset":55084,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55352,"parent_record_offset":55328,"ancestor_record_offset":55084,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55328,"parent_record_offset":55268,"ancestor_record_offset":55084,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55376,"parent_record_offset":55352,"ancestor_record_offset":55084,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":55616,"parent_record_offset":55588,"ancestor_record_offset":55084,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":55644,"parent_record_offset":55616,"ancestor_record_offset":55084,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":55704,"parent_record_offset":55588,"ancestor_record_offset":55084,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":55588,"parent_record_offset":55376,"ancestor_record_offset":55084,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55420,"parent_record_offset":55376,"ancestor_record_offset":55084,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":55736,"parent_record_offset":55376,"ancestor_record_offset":55084,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":55448,"parent_record_offset":55376,"ancestor_record_offset":55084,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":55532,"parent_record_offset":55376,"ancestor_record_offset":55084,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":56072,"parent_record_offset":55376,"ancestor_record_offset":55084,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":55212,"parent_record_offset":55184,"ancestor_record_offset":55084,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":55184,"parent_record_offset":55156,"ancestor_record_offset":55084,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":55156,"parent_record_offset":55128,"ancestor_record_offset":55084,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":56256,"parent_record_offset":56232,"ancestor_record_offset":55084,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":56028,"parent_record_offset":56004,"ancestor_record_offset":55084,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1061,"record_offset":43776,"rva":11094560,"va":5379803680}
// 编译实例 RVAs: 0xa94a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23115133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379803680,"raw_hex":"e87db56001","size":5,"soft_fail":false,"target_va":5402919072,"va":5379803934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::ser::SerializeMap::serialize_entry
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t12396502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379803680,"raw_hex":"e8d627bd00","size":5,"soft_fail":false,"target_va":5392200336,"va":5379803829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::format_escaped_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12395231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379803680,"raw_hex":"e8df22bd00","size":5,"soft_fail":false,"target_va":5392198992,"va":5379803756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::external::serde_support::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$uuid..Uuid$GT$::serialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12396741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379803680,"raw_hex":"e8c528bd00","size":5,"soft_fail":false,"target_va":5392200544,"va":5379803798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44044,"parent_record_offset":43776,"ancestor_record_offset":43776,"inlinee":5677,"source_line":409,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":45184,"parent_record_offset":43776,"ancestor_record_offset":43776,"inlinee":5699,"source_line":774,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44496,"parent_record_offset":43776,"ancestor_record_offset":43776,"inlinee":5682,"source_line":746,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1061,"record_offset":44336,"parent_record_offset":44304,"ancestor_record_offset":43776,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1061,"record_offset":44404,"parent_record_offset":44372,"ancestor_record_offset":43776,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1061,"record_offset":44304,"parent_record_offset":44276,"ancestor_record_offset":43776,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1061,"record_offset":44240,"parent_record_offset":44212,"ancestor_record_offset":43776,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1061,"record_offset":44372,"parent_record_offset":44240,"ancestor_record_offset":43776,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1061,"record_offset":44184,"parent_record_offset":44156,"ancestor_record_offset":43776,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1061,"record_offset":44436,"parent_record_offset":44240,"ancestor_record_offset":43776,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1061,"record_offset":44276,"parent_record_offset":44240,"ancestor_record_offset":43776,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1061,"record_offset":44212,"parent_record_offset":44184,"ancestor_record_offset":43776,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_profile::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_profile.rs
//     evidence={"module":1061,"record_offset":45092,"parent_record_offset":44668,"ancestor_record_offset":43776,"inlinee":51413,"source_line":12,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::SerializeMap::serialize_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/mod.rs
//     evidence={"module":1061,"record_offset":44524,"parent_record_offset":44496,"ancestor_record_offset":43776,"inlinee":5684,"source_line":1804,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$0::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":1061,"record_offset":44580,"parent_record_offset":44556,"ancestor_record_offset":43776,"inlinee":5689,"source_line":42,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44100,"parent_record_offset":44072,"ancestor_record_offset":43776,"inlinee":5679,"source_line":1849,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44704,"parent_record_offset":44668,"ancestor_record_offset":43776,"inlinee":5694,"source_line":1894,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":45248,"parent_record_offset":45216,"ancestor_record_offset":43776,"inlinee":5680,"source_line":1859,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$11::serialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44604,"parent_record_offset":44580,"ancestor_record_offset":43776,"inlinee":5691,"source_line":857,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44072,"parent_record_offset":44044,"ancestor_record_offset":43776,"inlinee":5678,"source_line":386,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44628,"parent_record_offset":44604,"ancestor_record_offset":43776,"inlinee":5692,"source_line":219,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":45216,"parent_record_offset":45184,"ancestor_record_offset":43776,"inlinee":5700,"source_line":720,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::serialize_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44556,"parent_record_offset":44524,"ancestor_record_offset":43776,"inlinee":5686,"source_line":667,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::serialize_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1061,"record_offset":44668,"parent_record_offset":44524,"ancestor_record_offset":43776,"inlinee":5693,"source_line":695,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":1061,"record_offset":44156,"parent_record_offset":44128,"ancestor_record_offset":43776,"inlinee":4269,"source_line":417,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":1061,"record_offset":44128,"parent_record_offset":44100,"ancestor_record_offset":43776,"inlinee":4267,"source_line":77,"archive":"_pdb/A/modules/01061.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$2::text_edit<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1018,"record_offset":119196,"rva":10861392,"va":5379570512}
// 编译实例 RVAs: 0xa5bb50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9957582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8320f68ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5379571097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9954549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e80b1b68ff","size":5,"soft_fail":false,"target_va":5369616080,"va":5379570624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9954179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e87d1c68ff","size":5,"soft_fail":false,"target_va":5369616480,"va":5379570654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9952928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8602168ff","size":5,"soft_fail":false,"target_va":5369617648,"va":5379570571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9897754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8e6f868ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5379571493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7322388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8ec4490ff","size":5,"soft_fail":false,"target_va":5372249392,"va":5379571775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5a150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e840e5ffff","size":5,"soft_fail":false,"target_va":5379563856,"va":5379570699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e822050000","size":5,"soft_fail":false,"target_va":5379572176,"va":5379570857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e870080000","size":5,"soft_fail":false,"target_va":5379573040,"va":5379570875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a62320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e851660000","size":5,"soft_fail":false,"target_va":5379597088,"va":5379570890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a67750
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t47614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8feb90000","size":5,"soft_fail":false,"target_va":5379618640,"va":5379571021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24022542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e80e8e6e01","size":5,"soft_fail":false,"target_va":5403593360,"va":5379570813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23407441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8512b6501","size":5,"soft_fail":false,"target_va":5402978880,"va":5379571434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23408741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e865306501","size":5,"soft_fail":false,"target_va":5402980128,"va":5379571382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23404557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e80d206501","size":5,"soft_fail":false,"target_va":5402975328,"va":5379570766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23404760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8d8206501","size":5,"soft_fail":false,"target_va":5402975504,"va":5379570739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23404727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8b7206501","size":5,"soft_fail":false,"target_va":5402975696,"va":5379570964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::replace_editor
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3528453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8fb28caff","size":5,"soft_fail":false,"target_va":5376042480,"va":5379570928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6409806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379570512,"raw_hex":"e8b2319eff","size":5,"soft_fail":false,"target_va":5373160864,"va":5379570665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":122084,"parent_record_offset":119196,"ancestor_record_offset":119196,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":119348,"parent_record_offset":119196,"ancestor_record_offset":119196,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::grab_focus
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":120992,"parent_record_offset":119196,"ancestor_record_offset":119196,"inlinee":17412,"source_line":839,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":122144,"parent_record_offset":122108,"ancestor_record_offset":119196,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":120708,"parent_record_offset":119508,"ancestor_record_offset":119196,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":119604,"parent_record_offset":119508,"ancestor_record_offset":119196,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":120852,"parent_record_offset":120824,"ancestor_record_offset":119196,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1018,"record_offset":120164,"parent_record_offset":120140,"ancestor_record_offset":119196,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":121808,"parent_record_offset":121784,"ancestor_record_offset":119196,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":121276,"parent_record_offset":121252,"ancestor_record_offset":119196,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":121360,"parent_record_offset":121336,"ancestor_record_offset":119196,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":121300,"parent_record_offset":121276,"ancestor_record_offset":119196,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1018,"record_offset":120220,"parent_record_offset":120196,"ancestor_record_offset":119196,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":121472,"parent_record_offset":121448,"ancestor_record_offset":119196,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":120196,"parent_record_offset":119508,"ancestor_record_offset":119196,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":122052,"parent_record_offset":120992,"ancestor_record_offset":119196,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":120312,"parent_record_offset":119508,"ancestor_record_offset":119196,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":121720,"parent_record_offset":121696,"ancestor_record_offset":119196,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":121612,"parent_record_offset":121588,"ancestor_record_offset":119196,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1018,"record_offset":122176,"parent_record_offset":122144,"ancestor_record_offset":119196,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":120248,"parent_record_offset":120220,"ancestor_record_offset":119196,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":120276,"parent_record_offset":120220,"ancestor_record_offset":119196,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1018,"record_offset":119752,"parent_record_offset":119692,"ancestor_record_offset":119196,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1018,"record_offset":120100,"parent_record_offset":119508,"ancestor_record_offset":119196,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120020,"parent_record_offset":119992,"ancestor_record_offset":119196,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120552,"parent_record_offset":120376,"ancestor_record_offset":119196,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":119692,"parent_record_offset":119664,"ancestor_record_offset":119196,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120640,"parent_record_offset":120612,"ancestor_record_offset":119196,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120736,"parent_record_offset":120708,"ancestor_record_offset":119196,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120888,"parent_record_offset":120852,"ancestor_record_offset":119196,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":119924,"parent_record_offset":119896,"ancestor_record_offset":119196,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120760,"parent_record_offset":120736,"ancestor_record_offset":119196,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":119724,"parent_record_offset":119692,"ancestor_record_offset":119196,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":119948,"parent_record_offset":119924,"ancestor_record_offset":119196,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120576,"parent_record_offset":120552,"ancestor_record_offset":119196,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120912,"parent_record_offset":120888,"ancestor_record_offset":119196,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120664,"parent_record_offset":120640,"ancestor_record_offset":119196,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120944,"parent_record_offset":120852,"ancestor_record_offset":119196,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":121784,"parent_record_offset":121760,"ancestor_record_offset":119196,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":121072,"parent_record_offset":121048,"ancestor_record_offset":119196,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":121048,"parent_record_offset":121024,"ancestor_record_offset":119196,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":121760,"parent_record_offset":121540,"ancestor_record_offset":119196,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":121104,"parent_record_offset":121072,"ancestor_record_offset":119196,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121672,"parent_record_offset":121648,"ancestor_record_offset":119196,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121648,"parent_record_offset":121540,"ancestor_record_offset":119196,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121588,"parent_record_offset":121564,"ancestor_record_offset":119196,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121968,"parent_record_offset":121180,"ancestor_record_offset":119196,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121564,"parent_record_offset":121540,"ancestor_record_offset":119196,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121156,"parent_record_offset":121132,"ancestor_record_offset":119196,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121132,"parent_record_offset":121072,"ancestor_record_offset":119196,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121180,"parent_record_offset":121156,"ancestor_record_offset":119196,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":121420,"parent_record_offset":121392,"ancestor_record_offset":119196,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":121448,"parent_record_offset":121420,"ancestor_record_offset":119196,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":121508,"parent_record_offset":121392,"ancestor_record_offset":119196,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":121392,"parent_record_offset":121180,"ancestor_record_offset":119196,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121224,"parent_record_offset":121180,"ancestor_record_offset":119196,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":121540,"parent_record_offset":121180,"ancestor_record_offset":119196,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":121252,"parent_record_offset":121180,"ancestor_record_offset":119196,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":121336,"parent_record_offset":121180,"ancestor_record_offset":119196,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":121876,"parent_record_offset":121180,"ancestor_record_offset":119196,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":121024,"parent_record_offset":120992,"ancestor_record_offset":119196,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1018,"record_offset":119896,"parent_record_offset":119872,"ancestor_record_offset":119196,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1018,"record_offset":119872,"parent_record_offset":119848,"ancestor_record_offset":119196,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1018,"record_offset":119632,"parent_record_offset":119508,"ancestor_record_offset":119196,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1018,"record_offset":119788,"parent_record_offset":119632,"ancestor_record_offset":119196,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1018,"record_offset":120140,"parent_record_offset":119508,"ancestor_record_offset":119196,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1018,"record_offset":120340,"parent_record_offset":120312,"ancestor_record_offset":119196,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1018,"record_offset":120376,"parent_record_offset":120340,"ancestor_record_offset":119196,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1018,"record_offset":119992,"parent_record_offset":119816,"ancestor_record_offset":119196,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1018,"record_offset":119816,"parent_record_offset":119788,"ancestor_record_offset":119196,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1018,"record_offset":119664,"parent_record_offset":119632,"ancestor_record_offset":119196,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1018,"record_offset":120612,"parent_record_offset":120340,"ancestor_record_offset":119196,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":1018,"record_offset":119848,"parent_record_offset":119816,"ancestor_record_offset":119196,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":119452,"parent_record_offset":119348,"ancestor_record_offset":119196,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":122024,"parent_record_offset":120992,"ancestor_record_offset":119196,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":119420,"parent_record_offset":119396,"ancestor_record_offset":119196,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":121832,"parent_record_offset":121808,"ancestor_record_offset":119196,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":119396,"parent_record_offset":119348,"ancestor_record_offset":119196,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":119476,"parent_record_offset":119452,"ancestor_record_offset":119196,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::impl$2::text_edit::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":1018,"record_offset":119508,"parent_record_offset":119348,"ancestor_record_offset":119196,"inlinee":50925,"source_line":304,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$2::text_edit::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 304
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6ED
// 首条位置: {"module":1018,"inlinee":50925}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":120708,"parent_record_offset":119508,"ancestor_record_offset":119508,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":119604,"parent_record_offset":119508,"ancestor_record_offset":119508,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":120196,"parent_record_offset":119508,"ancestor_record_offset":119508,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":120312,"parent_record_offset":119508,"ancestor_record_offset":119508,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1018,"record_offset":120100,"parent_record_offset":119508,"ancestor_record_offset":119508,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1018,"record_offset":119632,"parent_record_offset":119508,"ancestor_record_offset":119508,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1018,"record_offset":120140,"parent_record_offset":119508,"ancestor_record_offset":119508,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":120852,"parent_record_offset":120824,"ancestor_record_offset":119508,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1018,"record_offset":120164,"parent_record_offset":120140,"ancestor_record_offset":119508,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1018,"record_offset":120220,"parent_record_offset":120196,"ancestor_record_offset":119508,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":120824,"parent_record_offset":120796,"ancestor_record_offset":119508,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":120248,"parent_record_offset":120220,"ancestor_record_offset":119508,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":120276,"parent_record_offset":120220,"ancestor_record_offset":119508,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1018,"record_offset":119752,"parent_record_offset":119692,"ancestor_record_offset":119508,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120020,"parent_record_offset":119992,"ancestor_record_offset":119508,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120552,"parent_record_offset":120376,"ancestor_record_offset":119508,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":119692,"parent_record_offset":119664,"ancestor_record_offset":119508,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120640,"parent_record_offset":120612,"ancestor_record_offset":119508,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120736,"parent_record_offset":120708,"ancestor_record_offset":119508,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120888,"parent_record_offset":120852,"ancestor_record_offset":119508,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":119924,"parent_record_offset":119896,"ancestor_record_offset":119508,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120760,"parent_record_offset":120736,"ancestor_record_offset":119508,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":119724,"parent_record_offset":119692,"ancestor_record_offset":119508,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":119948,"parent_record_offset":119924,"ancestor_record_offset":119508,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120576,"parent_record_offset":120552,"ancestor_record_offset":119508,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120912,"parent_record_offset":120888,"ancestor_record_offset":119508,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120664,"parent_record_offset":120640,"ancestor_record_offset":119508,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":120944,"parent_record_offset":120852,"ancestor_record_offset":119508,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1018,"record_offset":119896,"parent_record_offset":119872,"ancestor_record_offset":119508,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1018,"record_offset":119872,"parent_record_offset":119848,"ancestor_record_offset":119508,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1018,"record_offset":119788,"parent_record_offset":119632,"ancestor_record_offset":119508,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1018,"record_offset":120340,"parent_record_offset":120312,"ancestor_record_offset":119508,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1018,"record_offset":120376,"parent_record_offset":120340,"ancestor_record_offset":119508,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1018,"record_offset":119992,"parent_record_offset":119816,"ancestor_record_offset":119508,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1018,"record_offset":119816,"parent_record_offset":119788,"ancestor_record_offset":119508,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1018,"record_offset":119664,"parent_record_offset":119632,"ancestor_record_offset":119508,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1018,"record_offset":120612,"parent_record_offset":120340,"ancestor_record_offset":119508,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":1018,"record_offset":119848,"parent_record_offset":119816,"ancestor_record_offset":119508,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::TextEditView::size
// 记录形式: compiled_procedure; instances=1
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2425,"record_offset":10908,"rva":22988736,"va":5391697856}
// 编译实例 RVAs: 0x15ec7c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22084450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391697856,"raw_hex":"e89e04affe","size":5,"soft_fail":false,"target_va":5369613520,"va":5391697965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5a150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12134046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391697856,"raw_hex":"e862d946ff","size":5,"soft_fail":false,"target_va":5379563856,"va":5391697897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11280686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391697856,"raw_hex":"e82e21ac00","size":5,"soft_fail":false,"target_va":5402978880,"va":5391698189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15653373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391697856,"raw_hex":"e8032611ff","size":5,"soft_fail":false,"target_va":5376044544,"va":5391697912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2425,"record_offset":11044,"parent_record_offset":10908,"ancestor_record_offset":10908,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2425,"record_offset":11072,"parent_record_offset":10908,"ancestor_record_offset":10908,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2425,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":10908,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2425,"record_offset":11352,"parent_record_offset":11328,"ancestor_record_offset":10908,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2425,"record_offset":11436,"parent_record_offset":11412,"ancestor_record_offset":10908,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2425,"record_offset":11376,"parent_record_offset":11352,"ancestor_record_offset":10908,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2425,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":10908,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2425,"record_offset":12128,"parent_record_offset":11072,"ancestor_record_offset":10908,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2425,"record_offset":11796,"parent_record_offset":11772,"ancestor_record_offset":10908,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2425,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":10908,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2425,"record_offset":11860,"parent_record_offset":11836,"ancestor_record_offset":10908,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2425,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":10908,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2425,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":10908,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2425,"record_offset":11836,"parent_record_offset":11616,"ancestor_record_offset":10908,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2425,"record_offset":11180,"parent_record_offset":11152,"ancestor_record_offset":10908,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11748,"parent_record_offset":11724,"ancestor_record_offset":10908,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11724,"parent_record_offset":11616,"ancestor_record_offset":10908,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11664,"parent_record_offset":11640,"ancestor_record_offset":10908,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":12044,"parent_record_offset":11256,"ancestor_record_offset":10908,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":10908,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11232,"parent_record_offset":11208,"ancestor_record_offset":10908,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11208,"parent_record_offset":11152,"ancestor_record_offset":10908,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11256,"parent_record_offset":11232,"ancestor_record_offset":10908,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2425,"record_offset":11496,"parent_record_offset":11468,"ancestor_record_offset":10908,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2425,"record_offset":11524,"parent_record_offset":11496,"ancestor_record_offset":10908,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2425,"record_offset":11584,"parent_record_offset":11468,"ancestor_record_offset":10908,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2425,"record_offset":11468,"parent_record_offset":11256,"ancestor_record_offset":10908,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11300,"parent_record_offset":11256,"ancestor_record_offset":10908,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2425,"record_offset":11616,"parent_record_offset":11256,"ancestor_record_offset":10908,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2425,"record_offset":11328,"parent_record_offset":11256,"ancestor_record_offset":10908,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2425,"record_offset":11412,"parent_record_offset":11256,"ancestor_record_offset":10908,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2425,"record_offset":11952,"parent_record_offset":11256,"ancestor_record_offset":10908,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2425,"record_offset":11104,"parent_record_offset":11072,"ancestor_record_offset":10908,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2425,"record_offset":11908,"parent_record_offset":11884,"ancestor_record_offset":10908,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::impl$3::size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":12100,"parent_record_offset":11072,"ancestor_record_offset":10908,"inlinee":50908,"source_line":329,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$3::size::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6DC
// 首条位置: {"module":1018,"inlinee":50908}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::TextEditView::text
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 337
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144A3
// 首条位置: {"module":2468,"inlinee":83107}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":21876,"parent_record_offset":21840,"ancestor_record_offset":21840,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":22008,"parent_record_offset":21984,"ancestor_record_offset":21840,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":21984,"parent_record_offset":21928,"ancestor_record_offset":21840,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":21956,"parent_record_offset":21928,"ancestor_record_offset":21840,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":21928,"parent_record_offset":21900,"ancestor_record_offset":21840,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":21840,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::TextEditMemory::clear
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14495
// 首条位置: {"module":2425,"inlinee":83093}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":3100,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":3004,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":2512,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":3044,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2425,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2468,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2425,"record_offset":2640,"parent_record_offset":2580,"ancestor_record_offset":2468,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2924,"parent_record_offset":2896,"ancestor_record_offset":2468,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3388,"parent_record_offset":3200,"ancestor_record_offset":2468,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":2468,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3480,"parent_record_offset":3448,"ancestor_record_offset":2468,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":2468,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2612,"parent_record_offset":2580,"ancestor_record_offset":2468,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2468,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":2468,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2425,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":2468,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":2425,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":2468,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":2425,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":2468,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":2676,"parent_record_offset":2512,"ancestor_record_offset":2468,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":2425,"record_offset":3160,"parent_record_offset":3128,"ancestor_record_offset":2468,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":3200,"parent_record_offset":3160,"ancestor_record_offset":2468,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":2896,"parent_record_offset":2704,"ancestor_record_offset":2468,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":2425,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":2468,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":2425,"record_offset":2548,"parent_record_offset":2512,"ancestor_record_offset":2468,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":2425,"record_offset":3448,"parent_record_offset":3160,"ancestor_record_offset":2468,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":2425,"record_offset":2740,"parent_record_offset":2704,"ancestor_record_offset":2468,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$5::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 369
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6E7
// 首条位置: {"module":1018,"inlinee":50919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$6::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 381
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1020,"record_offset":315360,"rva":10908256,"va":5379617376}
// 编译实例 RVAs: 0xa67260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9944300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e8144368ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5379618039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7368576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e880908fff","size":5,"soft_fail":false,"target_va":5372249392,"va":5379617963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a62320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e8f1afffff","size":5,"soft_fail":false,"target_va":5379597088,"va":5379617578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a67750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e8ce030000","size":5,"soft_fail":false,"target_va":5379618640,"va":5379617661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23975883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e8cbd76d01","size":5,"soft_fail":false,"target_va":5403593360,"va":5379617472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23362351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e82f7b6401","size":5,"soft_fail":false,"target_va":5402980128,"va":5379617772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23357801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e869696401","size":5,"soft_fail":false,"target_va":5402975328,"va":5379617522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23358004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e8346a6401","size":5,"soft_fail":false,"target_va":5402975504,"va":5379617495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23358055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e8676a6401","size":5,"soft_fail":false,"target_va":5402975696,"va":5379617636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::take_editor
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3571151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379617376,"raw_hex":"e83182c9ff","size":5,"soft_fail":false,"target_va":5376046304,"va":5379617450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1020,"record_offset":315620,"parent_record_offset":315360,"ancestor_record_offset":315360,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1020,"record_offset":315504,"parent_record_offset":315360,"ancestor_record_offset":315360,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1020,"record_offset":316208,"parent_record_offset":315360,"ancestor_record_offset":315360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1020,"record_offset":316112,"parent_record_offset":315360,"ancestor_record_offset":315360,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1020,"record_offset":315648,"parent_record_offset":315360,"ancestor_record_offset":315360,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1020,"record_offset":316152,"parent_record_offset":315360,"ancestor_record_offset":315360,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1020,"record_offset":316692,"parent_record_offset":316656,"ancestor_record_offset":315360,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1020,"record_offset":316656,"parent_record_offset":316632,"ancestor_record_offset":315360,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1020,"record_offset":316176,"parent_record_offset":316152,"ancestor_record_offset":315360,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1020,"record_offset":315528,"parent_record_offset":315504,"ancestor_record_offset":315360,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1020,"record_offset":316724,"parent_record_offset":316692,"ancestor_record_offset":315360,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1020,"record_offset":315556,"parent_record_offset":315528,"ancestor_record_offset":315360,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1020,"record_offset":315584,"parent_record_offset":315528,"ancestor_record_offset":315360,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1020,"record_offset":315764,"parent_record_offset":315704,"ancestor_record_offset":315360,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":316032,"parent_record_offset":316004,"ancestor_record_offset":315360,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":316476,"parent_record_offset":316300,"ancestor_record_offset":315360,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":315704,"parent_record_offset":315680,"ancestor_record_offset":315360,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":316564,"parent_record_offset":316536,"ancestor_record_offset":315360,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":315936,"parent_record_offset":315908,"ancestor_record_offset":315360,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":315736,"parent_record_offset":315704,"ancestor_record_offset":315360,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":315960,"parent_record_offset":315936,"ancestor_record_offset":315360,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":316500,"parent_record_offset":316476,"ancestor_record_offset":315360,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":316752,"parent_record_offset":316692,"ancestor_record_offset":315360,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":316588,"parent_record_offset":316564,"ancestor_record_offset":315360,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1020,"record_offset":316784,"parent_record_offset":316656,"ancestor_record_offset":315360,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1020,"record_offset":315908,"parent_record_offset":315884,"ancestor_record_offset":315360,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1020,"record_offset":315884,"parent_record_offset":315860,"ancestor_record_offset":315360,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1020,"record_offset":315800,"parent_record_offset":315648,"ancestor_record_offset":315360,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1020,"record_offset":316264,"parent_record_offset":316236,"ancestor_record_offset":315360,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1020,"record_offset":316300,"parent_record_offset":316264,"ancestor_record_offset":315360,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1020,"record_offset":316004,"parent_record_offset":315828,"ancestor_record_offset":315360,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1020,"record_offset":315828,"parent_record_offset":315800,"ancestor_record_offset":315360,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1020,"record_offset":315680,"parent_record_offset":315648,"ancestor_record_offset":315360,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1020,"record_offset":316536,"parent_record_offset":316264,"ancestor_record_offset":315360,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":1020,"record_offset":315860,"parent_record_offset":315828,"ancestor_record_offset":315360,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/01020.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$7::prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 390
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2425,"record_offset":12164,"rva":22989088,"va":5391698208}
// 编译实例 RVAs: 0x15ec920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::bounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15653558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391698208,"raw_hex":"e84a2511ff","size":5,"soft_fail":false,"target_va":5376044688,"va":5391698241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::prepare
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15653534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391698208,"raw_hex":"e8622511ff","size":5,"soft_fail":false,"target_va":5376044704,"va":5391698233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$7::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2425,"record_offset":12284,"rva":22989184,"va":5391698304}
// 编译实例 RVAs: 0x15ec980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::bounds
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-15653680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391698304,"raw_hex":"e8d02411ff","size":5,"soft_fail":false,"target_va":5376044688,"va":5391698363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15653322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391698304,"raw_hex":"e8362611ff","size":5,"soft_fail":false,"target_va":5376045152,"va":5391698469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::scale_u32_packed_srgba_color
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14754577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391698304,"raw_hex":"e8efdc1eff","size":5,"soft_fail":false,"target_va":5376943872,"va":5391698444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2425,"record_offset":12684,"parent_record_offset":12284,"ancestor_record_offset":12284,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":12424,"parent_record_offset":12284,"ancestor_record_offset":12284,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":12596,"parent_record_offset":12284,"ancestor_record_offset":12284,"inlinee":25007,"source_line":1061,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":12508,"parent_record_offset":12284,"ancestor_record_offset":12284,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2425,"record_offset":12536,"parent_record_offset":12508,"ancestor_record_offset":12284,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2425,"record_offset":12624,"parent_record_offset":12596,"ancestor_record_offset":12284,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2425,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":12284,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":12284,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$7::on_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 419
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2425,"record_offset":12744,"rva":22989392,"va":5391698512}
// 编译实例 RVAs: 0x15eca50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$smol_str..SmolStr$u20$as$u20$core..ops..deref..Deref$GT$::deref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12258150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391698512,"raw_hex":"e89af444ff","size":5,"soft_fail":false,"target_va":5379440880,"va":5391699025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::unicode::unicode_data::cc::lookup
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-14083422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391698512,"raw_hex":"e8a21a29ff","size":5,"soft_fail":false,"target_va":5377615248,"va":5391698665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::editor_action
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-15652703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391698512,"raw_hex":"e8a12811ff","size":5,"soft_fail":false,"target_va":5376046176,"va":5391698874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_input::impl$15::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/lib.rs
//     evidence={"module":2425,"record_offset":12884,"parent_record_offset":12744,"ancestor_record_offset":12744,"inlinee":29668,"source_line":132,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":2425,"record_offset":13020,"parent_record_offset":12744,"ancestor_record_offset":12744,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::chars
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2425,"record_offset":12908,"parent_record_offset":12744,"ancestor_record_offset":12744,"inlinee":4970,"source_line":865,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::InputModifiers::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2425,"record_offset":13540,"parent_record_offset":12744,"ancestor_record_offset":12744,"inlinee":83100,"source_line":603,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_control
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\methods.rs
//     evidence={"module":2425,"record_offset":13476,"parent_record_offset":13440,"ancestor_record_offset":12744,"inlinee":41857,"source_line":914,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2425,"record_offset":13044,"parent_record_offset":13020,"ancestor_record_offset":12744,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2425,"record_offset":13368,"parent_record_offset":13068,"ancestor_record_offset":12744,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2425,"record_offset":13068,"parent_record_offset":13044,"ancestor_record_offset":12744,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2425,"record_offset":13404,"parent_record_offset":13368,"ancestor_record_offset":12744,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2425,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":12744,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2425,"record_offset":13216,"parent_record_offset":13184,"ancestor_record_offset":12744,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2425,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":12744,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2425,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":12744,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2425,"record_offset":13184,"parent_record_offset":13132,"ancestor_record_offset":12744,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2425,"record_offset":13104,"parent_record_offset":13068,"ancestor_record_offset":12744,"inlinee":4928,"source_line":42,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2425,"record_offset":13132,"parent_record_offset":13104,"ancestor_record_offset":12744,"inlinee":4930,"source_line":36,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2425,"record_offset":13276,"parent_record_offset":13132,"ancestor_record_offset":12744,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2425,"record_offset":13248,"parent_record_offset":13132,"ancestor_record_offset":12744,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::impl$7::on_input::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":14184,"parent_record_offset":14148,"ancestor_record_offset":12744,"inlinee":51411,"source_line":468,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::impl$7::on_input::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":2425,"record_offset":13440,"parent_record_offset":13404,"ancestor_record_offset":12744,"inlinee":83098,"source_line":474,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$7::on_input::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 468
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC8D3
// 首条位置: {"module":1061,"inlinee":51411}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::char::methods::impl$0::is_control
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\methods.rs
//     evidence={"module":2425,"record_offset":14220,"parent_record_offset":14184,"ancestor_record_offset":14184,"inlinee":41857,"source_line":914,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$7::on_input::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 474
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1449A
// 首条位置: {"module":2425,"inlinee":83098}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::char::methods::impl$0::is_control
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\methods.rs
//     evidence={"module":2425,"record_offset":13476,"parent_record_offset":13440,"ancestor_record_offset":13440,"inlinee":41857,"source_line":914,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$8::toggle_icon
// 记录形式: compiled_procedure; instances=1
// 源码行: 488
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":319,"record_offset":76556,"rva":4471312,"va":5373180432}
// 编译实例 RVAs: 0x443a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3567121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180432,"raw_hex":"e8ef91c9ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373180636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3564441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180432,"raw_hex":"e8679cc9ff","size":5,"soft_fail":false,"target_va":5369616080,"va":5373180516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3564065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180432,"raw_hex":"e8df9dc9ff","size":5,"soft_fail":false,"target_va":5369616480,"va":5373180540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3562828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180432,"raw_hex":"e8b4a2c9ff","size":5,"soft_fail":false,"target_va":5369617648,"va":5373180471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5a910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6385262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180432,"raw_hex":"e86e6e6100","size":5,"soft_fail":false,"target_va":5379565840,"va":5373180573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29797545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180432,"raw_hex":"e8a9acc601","size":5,"soft_fail":false,"target_va":5402978912,"va":5373181362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29797549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180432,"raw_hex":"e8adacc601","size":5,"soft_fail":false,"target_va":5402978880,"va":5373181326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180432,"raw_hex":"e814b3ffff","size":5,"soft_fail":false,"target_va":5373160864,"va":5373180551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":80148,"parent_record_offset":76556,"ancestor_record_offset":76556,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":76708,"parent_record_offset":76556,"ancestor_record_offset":76556,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":80204,"parent_record_offset":76556,"ancestor_record_offset":76556,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":76888,"parent_record_offset":76556,"ancestor_record_offset":76556,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":79076,"parent_record_offset":76556,"ancestor_record_offset":76556,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":319,"record_offset":77952,"parent_record_offset":76556,"ancestor_record_offset":76556,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":319,"record_offset":77704,"parent_record_offset":77680,"ancestor_record_offset":76556,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":319,"record_offset":77172,"parent_record_offset":77148,"ancestor_record_offset":76556,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":319,"record_offset":77256,"parent_record_offset":77232,"ancestor_record_offset":76556,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":319,"record_offset":77196,"parent_record_offset":77172,"ancestor_record_offset":76556,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":319,"record_offset":77368,"parent_record_offset":77344,"ancestor_record_offset":76556,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":319,"record_offset":77920,"parent_record_offset":76888,"ancestor_record_offset":76556,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":319,"record_offset":77616,"parent_record_offset":77592,"ancestor_record_offset":76556,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":319,"record_offset":77508,"parent_record_offset":77484,"ancestor_record_offset":76556,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":319,"record_offset":77680,"parent_record_offset":77656,"ancestor_record_offset":76556,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":79168,"parent_record_offset":79132,"ancestor_record_offset":76556,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":79192,"parent_record_offset":79168,"ancestor_record_offset":76556,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":76968,"parent_record_offset":76944,"ancestor_record_offset":76556,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":76944,"parent_record_offset":76920,"ancestor_record_offset":76556,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":77656,"parent_record_offset":77436,"ancestor_record_offset":76556,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":77000,"parent_record_offset":76968,"ancestor_record_offset":76556,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77568,"parent_record_offset":77544,"ancestor_record_offset":76556,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77544,"parent_record_offset":77436,"ancestor_record_offset":76556,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77484,"parent_record_offset":77460,"ancestor_record_offset":76556,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77864,"parent_record_offset":77076,"ancestor_record_offset":76556,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77460,"parent_record_offset":77436,"ancestor_record_offset":76556,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77052,"parent_record_offset":77028,"ancestor_record_offset":76556,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":79248,"parent_record_offset":79192,"ancestor_record_offset":76556,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77028,"parent_record_offset":76968,"ancestor_record_offset":76556,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77076,"parent_record_offset":77052,"ancestor_record_offset":76556,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":77316,"parent_record_offset":77288,"ancestor_record_offset":76556,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":77344,"parent_record_offset":77316,"ancestor_record_offset":76556,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":77404,"parent_record_offset":77288,"ancestor_record_offset":76556,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":77288,"parent_record_offset":77076,"ancestor_record_offset":76556,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77120,"parent_record_offset":77076,"ancestor_record_offset":76556,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":77436,"parent_record_offset":77076,"ancestor_record_offset":76556,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":77148,"parent_record_offset":77076,"ancestor_record_offset":76556,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":77232,"parent_record_offset":77076,"ancestor_record_offset":76556,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":77772,"parent_record_offset":77076,"ancestor_record_offset":76556,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":319,"record_offset":79132,"parent_record_offset":79104,"ancestor_record_offset":76556,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":319,"record_offset":76920,"parent_record_offset":76888,"ancestor_record_offset":76556,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":319,"record_offset":79104,"parent_record_offset":79076,"ancestor_record_offset":76556,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":79044,"parent_record_offset":77952,"ancestor_record_offset":76556,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":76796,"parent_record_offset":76708,"ancestor_record_offset":76556,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":77984,"parent_record_offset":77952,"ancestor_record_offset":76556,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":319,"record_offset":80172,"parent_record_offset":80148,"ancestor_record_offset":76556,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":76764,"parent_record_offset":76740,"ancestor_record_offset":76556,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":77728,"parent_record_offset":77704,"ancestor_record_offset":76556,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":76740,"parent_record_offset":76708,"ancestor_record_offset":76556,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":76820,"parent_record_offset":76796,"ancestor_record_offset":76556,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::report_bug::impl$8::toggle_icon::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":319,"record_offset":76852,"parent_record_offset":76708,"ancestor_record_offset":76556,"inlinee":17371,"source_line":490,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$8::toggle_icon::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 490
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43DB
// 首条位置: {"module":319,"inlinee":17371}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$18::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 516
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1449E
// 首条位置: {"module":2425,"inlinee":83102}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$9::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 524
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6E9
// 首条位置: {"module":1018,"inlinee":50921}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":105280,"parent_record_offset":105256,"ancestor_record_offset":105256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":105328,"parent_record_offset":105304,"ancestor_record_offset":105256,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":105304,"parent_record_offset":105280,"ancestor_record_offset":105256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":105364,"parent_record_offset":105328,"ancestor_record_offset":105256,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":105388,"parent_record_offset":105364,"ancestor_record_offset":105256,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":105420,"parent_record_offset":105328,"ancestor_record_offset":105256,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$10::prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 531
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2425,"record_offset":14316,"rva":22990160,"va":5391699280}
// 编译实例 RVAs: 0x15ecd50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18949829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391699280,"raw_hex":"e83bd9defe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391699536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056c370
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17304130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391699280,"raw_hex":"e8bef5f7fe","size":5,"soft_fail":false,"target_va":5374395248,"va":5391699373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407bf410
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14866838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391699280,"raw_hex":"e86a261dff","size":5,"soft_fail":false,"target_va":5376832528,"va":5391699361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12102144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391699280,"raw_hex":"e8005647ff","size":5,"soft_fail":false,"target_va":5379597488,"va":5391699627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a68060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: report_bug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12078446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391699280,"raw_hex":"e892b247ff","size":5,"soft_fail":false,"target_va":5379620960,"va":5391699401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12079650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391699280,"raw_hex":"e8dead47ff","size":5,"soft_fail":false,"target_va":5379619792,"va":5391699437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::fill_color
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12080156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391699280,"raw_hex":"e8e4ab47ff","size":5,"soft_fail":false,"target_va":5379619264,"va":5391699415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":14480,"parent_record_offset":14316,"ancestor_record_offset":14316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2425,"record_offset":14452,"parent_record_offset":14316,"ancestor_record_offset":14316,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2425,"record_offset":15028,"parent_record_offset":14316,"ancestor_record_offset":14316,"inlinee":17206,"source_line":977,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":14316,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2425,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":14316,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":14892,"parent_record_offset":14864,"ancestor_record_offset":14316,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2425,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":14316,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2425,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":14316,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2425,"record_offset":14920,"parent_record_offset":14892,"ancestor_record_offset":14316,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2425,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":14316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::report_bug::impl$10::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 559
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2425,"record_offset":15060,"rva":22990528,"va":5391699648}
// 编译实例 RVAs: 0x15ecec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::rect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15662268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391699648,"raw_hex":"e8440311ff","size":5,"soft_fail":false,"target_va":5376037504,"va":5391699767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":15368,"parent_record_offset":15060,"ancestor_record_offset":15060,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":15284,"parent_record_offset":15060,"ancestor_record_offset":15060,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":15200,"parent_record_offset":15060,"ancestor_record_offset":15060,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::RenderContext::draw_rect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2425,"record_offset":15452,"parent_record_offset":15060,"ancestor_record_offset":15060,"inlinee":17200,"source_line":1035,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2425,"record_offset":15480,"parent_record_offset":15452,"ancestor_record_offset":15060,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2425,"record_offset":15224,"parent_record_offset":15200,"ancestor_record_offset":15060,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2425,"record_offset":15392,"parent_record_offset":15368,"ancestor_record_offset":15060,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2425,"record_offset":15308,"parent_record_offset":15284,"ancestor_record_offset":15060,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":15536,"parent_record_offset":15452,"ancestor_record_offset":15060,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2425,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":15060,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/02425.jsonl.gz"}
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
// 完整结构及原始类型记录: report_bug.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::report_bug::FeedbackReportEditorMemory
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 31
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["toolbar::menu::report_bug::FeedbackReportEditorMemory"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>", "constructor": "bevy_utils::synccell::SyncCell", "arguments": ["toolbar::menu::report_bug::FeedbackReportEditorMemory"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > >", "constructor": "tuple$", "arguments": ["bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >", "bevy_ecs::component::ComponentId", "bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>", "bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > >", "constructor": "core::option::Option", "arguments": ["tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>,bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState> >", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>", "bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::report_bug::FeedbackReportUiState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 32
// [PDB 泛型关系，非字段关系] {"type": "enum2$<toolbar::menu::report_bug::FeedbackReportUiState>", "constructor": "enum2$", "arguments": ["toolbar::menu::report_bug::FeedbackReportUiState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >", "constructor": "bevy_utils::synccell::SyncCell", "arguments": ["enum2$<toolbar::menu::report_bug::FeedbackReportUiState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["enum2$<toolbar::menu::report_bug::FeedbackReportUiState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > >", "constructor": "tuple$", "arguments": ["bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_ecs::component::ComponentId", "bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >", "bevy_ecs::component::ComponentId", "bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>", "bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > >", "constructor": "core::option::Option", "arguments": ["tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<tuple$<bevy_utils::synccell::SyncCell<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_ecs::component::ComponentId,bevy_utils::synccell::SyncCell<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::component::ComponentId,bevy_ecs::event::_::FetchState<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::_::FetchState<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>,bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState> >", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>", "bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)", "void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::report_bug::MetaJson
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::report_bug::MetaJson>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::report_bug::MetaJson"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::report_bug::TextEditContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content<toolbar::menu::report_bug::TextEditContent>", "constructor": "toolbar::mei::core::NodeContext::content", "arguments": ["toolbar::menu::report_bug::TextEditContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::report_bug::TextEditContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::menu::report_bug::TextEditContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::report_bug::TextEditContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::report_bug::TextEditContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::report_bug::TextEditContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::report_bug::TextEditContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::menu::report_bug::TextEditContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::menu::report_bug::TextEditContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::menu::report_bug::TextEditContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::menu::report_bug::TextEditContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::menu::report_bug::TextEditContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::menu::report_bug::TextEditContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::menu::report_bug::TextEditContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::menu::report_bug::TextEditContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::report_bug::TextEditMemory
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<toolbar::menu::report_bug::TextEditMemory,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["toolbar::menu::report_bug::TextEditMemory", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::report_bug::TextEditView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::report_bug::ToggleIconContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::report_bug::ToggleIconContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::menu::report_bug::ToggleIconContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::report_bug::ToggleIconContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::report_bug::ToggleIconContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::report_bug::ToggleIconContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::report_bug::ToggleIconContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::menu::report_bug::ToggleIconContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::menu::report_bug::ToggleIconContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::on_input<toolbar::menu::report_bug::ToggleIconContent>", "constructor": "toolbar::mei::core::NodeContent::on_input", "arguments": ["toolbar::menu::report_bug::ToggleIconContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::menu::report_bug::ToggleIconContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::menu::report_bug::ToggleIconContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::menu::report_bug::ToggleIconContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::menu::report_bug::ToggleIconContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::menu::report_bug::ToggleIconContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::menu::report_bug::ToggleIconContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: report_bug.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: report_bug.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 47; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>", "bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::system::system_param::Local<toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::system::system_param::Local<toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::FeedbackReportEditorMemory::clear
// 信息 ID: b4e6235f15a2d1da48b47c4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::ui; module=2425, inline_record=2396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::TextEditMemory::clear
// 信息 ID: 373cf59eecadb5243c400ccf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::ui; module=2425, inline_record=2468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::TextEditView::size
// 信息 ID: 8cdd1bca3735d0ea8a10377a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$1,ref$<str$> > @ 0x16193f1 -> 0x15ec7c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::TextEditView::text
// 信息 ID: cc0d9d988d5c28b9b7989d90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=21840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: ac22a7c3ca3f3c798f7852bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] toolbar::menu::report_bug::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x15ebea9 -> 0xa94a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$0::default
// 信息 ID: 4b3e5f3f303e75a48f6fa026
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 75cc74cc49c22312fbad1ef6: void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)>; module=1061, inline_record=2484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$10::prepare
// 信息 ID: f1b0130fb4d031fe776d052b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$10::render
// 信息 ID: 8e359db943904cdabf44d416
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$11::default
// 信息 ID: ae4cd4cf9f3a5dfa0633948d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$18::default
// 信息 ID: 3c7c5552cd7ff08f63ea680d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$2::text_edit::closure$0
// 信息 ID: dd3df514b2f236d8c3d8d46b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::report_bug::impl$2::text_edit", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::impl$2::text_edit<ref$<str$> >; module=1018, inline_record=119508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$2::text_edit<ref$<str$> >
// 信息 ID: 9c12a9c8fa64379670af6a33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] toolbar::menu::report_bug::impl$2::text_edit -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$1,ref$<str$> > @ 0x16193d9 -> 0xa5bb50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$3::size::closure$0
// 信息 ID: 98499b1297e8529688e20d17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::report_bug::impl$3::size", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::TextEditView::size; module=2425, inline_record=12100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$5::create_node_content
// 信息 ID: e98bcbd46bf9545640bc9bfa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::report_bug::TextEditContent>; module=1018, inline_record=99144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$6::drop
// 信息 ID: 385bae9e6f9852af8c95da3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<toolbar::menu::report_bug::TextEditContent> @ 0xa63bb6 -> 0xa67260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$7::on_input
// 信息 ID: 82bf6910186857be8e8fe25c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$7::on_input::closure$0
// 信息 ID: f06f67819ecd3fe92dfcb10a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::report_bug::impl$7::on_input", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::impl$7::on_input; module=2425, inline_record=14184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$7::on_input::closure$1
// 信息 ID: 91b9503a9da06b3a40bdec42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::report_bug::impl$7::on_input", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::impl$7::on_input; module=2425, inline_record=13440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$7::prepare
// 信息 ID: ef20be3c430b8991241fbdd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$7::render
// 信息 ID: 2720dfcda9aaa406d9386c7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$8::toggle_icon
// 信息 ID: a90bde7c39bef80f0a694c68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$3,ref$<str$> > @ 0x161cf93 -> 0x443a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$3,ref$<str$> > @ 0x161cfaf -> 0x443a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$3,ref$<str$> > @ 0x161cfcb -> 0x443a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$3,ref$<str$> > @ 0x161cfea -> 0x443a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$8::toggle_icon::closure$0
// 信息 ID: 0dba1ac595eba9f2c6ec51fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::report_bug::impl$8::toggle_icon", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::impl$8::toggle_icon; module=319, inline_record=76852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::impl$9::create_node_content
// 信息 ID: 66fa1761b6bfe59d0dffb3e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::report_bug::ToggleIconContent>; module=1018, inline_record=105256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui
// 信息 ID: 1971db844537cb67b6b0ef54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 75cc74cc49c22312fbad1ef6: void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> @ 0xa9439f -> 0x15eb380 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >),void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)> > @ 0x15e7282 -> 0x15eb380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$0
// 信息 ID: 320e73eb09e1742b29bf28ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=70884; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=71760; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=72320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$1
// 信息 ID: 538bec2996a789ff3ee1e0a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=21704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$10
// 信息 ID: b44a591140800096d4b5d229
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$10 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$10,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$10,ref$<str$> >; module=2468, inline_record=51564; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$10,ref$<str$> >; module=2468, inline_record=52076; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$10,ref$<str$> >; module=2468, inline_record=52272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$11
// 信息 ID: 117bace276485cb90a8a35d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$11 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$11,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$11,ref$<str$> >; module=2468, inline_record=79916; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$11,ref$<str$> >; module=2468, inline_record=81572; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$11,ref$<str$> >; module=2468, inline_record=82912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$12
// 信息 ID: e282fe1167707f6a68b8fb5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$12 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$12,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$12,ref$<str$> >; module=2468, inline_record=41828; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$12,ref$<str$> >; module=2468, inline_record=43724; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$12,ref$<str$> >; module=2468, inline_record=45304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$12::closure$0
// 信息 ID: d8a22e8c8b26eeb452a95dba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure$12::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::report_bug::ui::closure$12::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::report_bug::ui::closure$12::closure_env$0>; module=1018, inline_record=55084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$2
// 信息 ID: 066b00ce20b4e1634b183a0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=53872; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=54712; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=55236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$3
// 信息 ID: dce70b5a4e9bd1036985f77f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$3,ref$<str$> >; module=2468, inline_record=67444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$4
// 信息 ID: 208f2797781b5c2b03248685
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$4,ref$<str$> > @ 0x161c495 -> 0x1620ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$5
// 信息 ID: 7608301c37056afaf322d5e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$5 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$5,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$5,ref$<str$> > @ 0x161acb5 -> 0x1620cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$6
// 信息 ID: b6f80ae00b78e0934cccce09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::report_bug::ui::closure_env$6 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$6,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$6,ref$<str$> > @ 0x161df55 -> 0x1620f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$6::closure$0
// 信息 ID: 96bdea5af810155901a2e823
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::ui::closure$6; module=2468, inline_record=119360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$7
// 信息 ID: cd93518a1a1717b96dc6740e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::ui; module=2425, inline_record=4216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$8
// 信息 ID: 20c4a865ca6e4fd755abbb45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::ui; module=2425, inline_record=6532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::report_bug::ui::closure$9
// 信息 ID: d9d151826b120bdc8f79295e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "toolbar::menu::report_bug::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::report_bug::ui; module=2425, inline_record=8212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
