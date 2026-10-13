// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/menu/corrupted_save_file.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: bc912047eeb645adb60cd469c4cda833ee71a2442fd3bb532e49ea4118b91fff
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: corrupted_save_file.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1018, 2468, 2505
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
// 函数符号: toolbar::menu::corrupted_save_file::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2505,"record_offset":244,"rva":23493424,"va":5392202544}
// 编译实例 RVAs: 0x1667b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: corrupted_save_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22585056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392202544,"raw_hex":"e82061a7fe","size":5,"soft_fail":false,"target_va":5369617648,"va":5392202699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: corrupted_save_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19048463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392202544,"raw_hex":"e8f157ddfe","size":5,"soft_fail":false,"target_va":5373154400,"va":5392202858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a700b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: corrupted_save_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12549005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392202544,"raw_hex":"e8738440ff","size":5,"soft_fail":false,"target_va":5379653808,"va":5392202808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161eae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: corrupted_save_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-299433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392202544,"raw_hex":"e8576efbff","size":5,"soft_fail":false,"target_va":5391903456,"va":5392202884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20893961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392202544,"raw_hex":"e8f72ec1fe","size":5,"soft_fail":false,"target_va":5371308800,"va":5392202756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10775968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392202544,"raw_hex":"e8a06da400","size":5,"soft_fail":false,"target_va":5402978880,"va":5392202907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19043336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392202544,"raw_hex":"e8f86bddfe","size":5,"soft_fail":false,"target_va":5373159392,"va":5392202723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2505,"record_offset":744,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2505,"record_offset":384,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2505,"record_offset":652,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2505,"record_offset":800,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7454,"source_line":349,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2505,"record_offset":1024,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8031,"source_line":727,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2505,"record_offset":952,"parent_record_offset":920,"ancestor_record_offset":244,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2505,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":244,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2505,"record_offset":888,"parent_record_offset":824,"ancestor_record_offset":244,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2505,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":244,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2505,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":244,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2505,"record_offset":440,"parent_record_offset":404,"ancestor_record_offset":244,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2505,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":244,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":2505,"record_offset":404,"parent_record_offset":384,"ancestor_record_offset":244,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2505,"record_offset":512,"parent_record_offset":464,"ancestor_record_offset":244,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2505,"record_offset":1136,"parent_record_offset":1080,"ancestor_record_offset":244,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2505,"record_offset":576,"parent_record_offset":464,"ancestor_record_offset":244,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2505,"record_offset":708,"parent_record_offset":680,"ancestor_record_offset":244,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2505,"record_offset":984,"parent_record_offset":888,"ancestor_record_offset":244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2505,"record_offset":1108,"parent_record_offset":1080,"ancestor_record_offset":244,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2505,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":244,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2505,"record_offset":1080,"parent_record_offset":1052,"ancestor_record_offset":244,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2505,"record_offset":860,"parent_record_offset":824,"ancestor_record_offset":244,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::corrupted_save_file::ui::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/corrupted_save_file.rs
//     evidence={"module":2505,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":244,"inlinee":50898,"source_line":29,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::corrupted_save_file::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6D2
// 首条位置: {"module":1018,"inlinee":50898}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2505,"record_offset":1080,"parent_record_offset":1052,"ancestor_record_offset":1052,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2505,"record_offset":1136,"parent_record_offset":1080,"ancestor_record_offset":1052,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2505,"record_offset":1108,"parent_record_offset":1080,"ancestor_record_offset":1052,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::corrupted_save_file::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14698
// 首条位置: {"module":2468,"inlinee":83608}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":91432,"parent_record_offset":91400,"ancestor_record_offset":91400,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":88724,"parent_record_offset":88700,"ancestor_record_offset":88700,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":88784,"parent_record_offset":88700,"ancestor_record_offset":88700,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":89660,"parent_record_offset":89636,"ancestor_record_offset":88700,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":89128,"parent_record_offset":89104,"ancestor_record_offset":88700,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":89212,"parent_record_offset":89188,"ancestor_record_offset":88700,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":89152,"parent_record_offset":89128,"ancestor_record_offset":88700,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":89324,"parent_record_offset":89300,"ancestor_record_offset":88700,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":91544,"parent_record_offset":91520,"ancestor_record_offset":91400,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":89572,"parent_record_offset":89548,"ancestor_record_offset":88700,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":89464,"parent_record_offset":89440,"ancestor_record_offset":88700,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":89636,"parent_record_offset":89612,"ancestor_record_offset":88700,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":88896,"parent_record_offset":88868,"ancestor_record_offset":88700,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":88924,"parent_record_offset":88896,"ancestor_record_offset":88700,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":89612,"parent_record_offset":89392,"ancestor_record_offset":88700,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":88956,"parent_record_offset":88924,"ancestor_record_offset":88700,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89524,"parent_record_offset":89500,"ancestor_record_offset":88700,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89500,"parent_record_offset":89392,"ancestor_record_offset":88700,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89440,"parent_record_offset":89416,"ancestor_record_offset":88700,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89820,"parent_record_offset":89032,"ancestor_record_offset":88700,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89416,"parent_record_offset":89392,"ancestor_record_offset":88700,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89008,"parent_record_offset":88984,"ancestor_record_offset":88700,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":88984,"parent_record_offset":88924,"ancestor_record_offset":88700,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89032,"parent_record_offset":89008,"ancestor_record_offset":88700,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":89272,"parent_record_offset":89244,"ancestor_record_offset":88700,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":89300,"parent_record_offset":89272,"ancestor_record_offset":88700,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":89360,"parent_record_offset":89244,"ancestor_record_offset":88700,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":89244,"parent_record_offset":89032,"ancestor_record_offset":88700,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89076,"parent_record_offset":89032,"ancestor_record_offset":88700,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":89392,"parent_record_offset":89032,"ancestor_record_offset":88700,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":89104,"parent_record_offset":89032,"ancestor_record_offset":88700,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":89188,"parent_record_offset":89032,"ancestor_record_offset":88700,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":89728,"parent_record_offset":89032,"ancestor_record_offset":88700,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":88868,"parent_record_offset":88840,"ancestor_record_offset":88700,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":88840,"parent_record_offset":88812,"ancestor_record_offset":88700,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":88752,"parent_record_offset":88724,"ancestor_record_offset":88700,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":88812,"parent_record_offset":88784,"ancestor_record_offset":88700,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2468,"record_offset":91468,"parent_record_offset":91432,"ancestor_record_offset":91400,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":89684,"parent_record_offset":89660,"ancestor_record_offset":88700,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
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
// 完整结构及原始类型记录: corrupted_save_file.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::corrupted_save_file::MessageScreenState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["toolbar::menu::corrupted_save_file::MessageScreenState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>,bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState> >", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>", "bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: corrupted_save_file.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: corrupted_save_file.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>", "bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>

// ------------------------------------------------------------------
// 函数: toolbar::menu::corrupted_save_file::ui
// 信息 ID: a9d425402e18d55c6c31d247
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 55824a71d7712d429ee5a608: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> @ 0xa96039 -> 0x1667b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > @ 0x15e97d2 -> 0x1667b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::corrupted_save_file::ui::closure$0
// 信息 ID: a59cec13f49511e7aff08aff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::corrupted_save_file::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::corrupted_save_file::ui; module=2505, inline_record=1052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::corrupted_save_file::ui::closure$1
// 信息 ID: 4574b881275a9ac828327edd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::corrupted_save_file::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::corrupted_save_file::ui::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::corrupted_save_file::ui::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::corrupted_save_file::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=88700; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::corrupted_save_file::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=90208; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::corrupted_save_file::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=91400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
