// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/menu/welcome.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a47a11a69889eb2afcb3acad271d5dbb8058f6d3343794efff4b2d3ce1864734
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: welcome.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2468, 2503
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
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::welcome::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2503,"record_offset":244,"rva":23492096,"va":5392201216}
// 编译实例 RVAs: 0x1667600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19047138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392201216,"raw_hex":"e81e5dddfe","size":5,"soft_fail":false,"target_va":5373154400,"va":5392201533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cac20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14273490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392201216,"raw_hex":"e82e3426ff","size":5,"soft_fail":false,"target_va":5377928224,"va":5392201709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a56ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12650262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392201216,"raw_hex":"e8eaf83eff","size":5,"soft_fail":false,"target_va":5379551216,"va":5392201473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161e500
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-299733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392201216,"raw_hex":"e82b6dfbff","size":5,"soft_fail":false,"target_va":5391901952,"va":5392201680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::is_active
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16952582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392201216,"raw_hex":"e8fa52fdfe","size":5,"soft_fail":false,"target_va":5375248800,"va":5392201377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::config::TemplateView::node_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12578629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392201216,"raw_hex":"e8bb1040ff","size":5,"soft_fail":false,"target_va":5379622992,"va":5392201616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2503,"record_offset":360,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02503.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2503,"record_offset":536,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/02503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2503,"record_offset":384,"parent_record_offset":360,"ancestor_record_offset":244,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02503.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2503,"record_offset":564,"parent_record_offset":536,"ancestor_record_offset":244,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::welcome::ui::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2468,"record_offset":123428,"rva":23206048,"va":5391915168}
// 编译实例 RVAs: 0x16218a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22301890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e83eb3abfe","size":5,"soft_fail":false,"target_va":5369613520,"va":5391915405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19165885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e8438ddbfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391915592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18735901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e8e31ce2fe","size":5,"soft_fail":false,"target_va":5373179424,"va":5391915320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12348935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e8f99143ff","size":5,"soft_fail":false,"target_va":5379566416,"va":5391915346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: welcome.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12343194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e866a843ff","size":5,"soft_fail":false,"target_va":5379572176,"va":5391915365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14385417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e8f77e24ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5391915300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20610608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e8d081c5fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5391915659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11063166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e87ecfa800","size":5,"soft_fail":false,"target_va":5402978912,"va":5391915741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15762342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391915168,"raw_hex":"e85a7c0fff","size":5,"soft_fail":false,"target_va":5376152880,"va":5391915217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2468,"record_offset":123576,"parent_record_offset":123428,"ancestor_record_offset":123428,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2468,"record_offset":125120,"parent_record_offset":123428,"ancestor_record_offset":123428,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2468,"record_offset":125180,"parent_record_offset":123428,"ancestor_record_offset":123428,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":124920,"parent_record_offset":123428,"ancestor_record_offset":123428,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":124860,"parent_record_offset":123428,"ancestor_record_offset":123428,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":123656,"parent_record_offset":123428,"ancestor_record_offset":123428,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":123720,"parent_record_offset":123428,"ancestor_record_offset":123428,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":125072,"parent_record_offset":125048,"ancestor_record_offset":123428,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":125048,"parent_record_offset":124992,"ancestor_record_offset":123428,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2468,"record_offset":123624,"parent_record_offset":123600,"ancestor_record_offset":123428,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":125020,"parent_record_offset":124992,"ancestor_record_offset":123428,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":125364,"parent_record_offset":125340,"ancestor_record_offset":123428,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":124992,"parent_record_offset":124968,"ancestor_record_offset":123428,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2468,"record_offset":125340,"parent_record_offset":125300,"ancestor_record_offset":123428,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2468,"record_offset":125300,"parent_record_offset":125264,"ancestor_record_offset":123428,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2468,"record_offset":125148,"parent_record_offset":125120,"ancestor_record_offset":123428,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2468,"record_offset":125204,"parent_record_offset":125180,"ancestor_record_offset":123428,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2468,"record_offset":125264,"parent_record_offset":125180,"ancestor_record_offset":123428,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":124632,"parent_record_offset":124608,"ancestor_record_offset":123428,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":124100,"parent_record_offset":124076,"ancestor_record_offset":123428,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":124184,"parent_record_offset":124160,"ancestor_record_offset":123428,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":124124,"parent_record_offset":124100,"ancestor_record_offset":123428,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":124296,"parent_record_offset":124272,"ancestor_record_offset":123428,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":123600,"parent_record_offset":123576,"ancestor_record_offset":123428,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":124944,"parent_record_offset":124920,"ancestor_record_offset":123428,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":124544,"parent_record_offset":124520,"ancestor_record_offset":123428,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":124436,"parent_record_offset":124412,"ancestor_record_offset":123428,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":125396,"parent_record_offset":125300,"ancestor_record_offset":123428,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":124608,"parent_record_offset":124584,"ancestor_record_offset":123428,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":123868,"parent_record_offset":123828,"ancestor_record_offset":123428,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":123896,"parent_record_offset":123868,"ancestor_record_offset":123428,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":124584,"parent_record_offset":124364,"ancestor_record_offset":123428,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":123928,"parent_record_offset":123896,"ancestor_record_offset":123428,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":124496,"parent_record_offset":124472,"ancestor_record_offset":123428,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":124472,"parent_record_offset":124364,"ancestor_record_offset":123428,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":124412,"parent_record_offset":124388,"ancestor_record_offset":123428,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":124792,"parent_record_offset":124004,"ancestor_record_offset":123428,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":124388,"parent_record_offset":124364,"ancestor_record_offset":123428,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":123980,"parent_record_offset":123956,"ancestor_record_offset":123428,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":123956,"parent_record_offset":123896,"ancestor_record_offset":123428,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":124004,"parent_record_offset":123980,"ancestor_record_offset":123428,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":124244,"parent_record_offset":124216,"ancestor_record_offset":123428,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":124272,"parent_record_offset":124244,"ancestor_record_offset":123428,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":124332,"parent_record_offset":124216,"ancestor_record_offset":123428,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":124216,"parent_record_offset":124004,"ancestor_record_offset":123428,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":124048,"parent_record_offset":124004,"ancestor_record_offset":123428,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":124364,"parent_record_offset":124004,"ancestor_record_offset":123428,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":124076,"parent_record_offset":124004,"ancestor_record_offset":123428,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":124160,"parent_record_offset":124004,"ancestor_record_offset":123428,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":124700,"parent_record_offset":124004,"ancestor_record_offset":123428,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":123828,"parent_record_offset":123792,"ancestor_record_offset":123428,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":123792,"parent_record_offset":123756,"ancestor_record_offset":123428,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":123688,"parent_record_offset":123656,"ancestor_record_offset":123428,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":123756,"parent_record_offset":123720,"ancestor_record_offset":123428,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2468,"record_offset":124888,"parent_record_offset":124860,"ancestor_record_offset":123428,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":124656,"parent_record_offset":124632,"ancestor_record_offset":123428,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
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
// 完整结构及原始类型记录: welcome.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: welcome.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: welcome.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>

// ------------------------------------------------------------------
// 函数: toolbar::menu::welcome::ui
// 信息 ID: 80c25dcc120e97199f2c22b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ce997180a93a7c48554e9dd3: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> @ 0xa9513c -> 0x1667600 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> > @ 0x15e0a10 -> 0x1667600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::welcome::ui::closure$0
// 信息 ID: 9289411e5873cc838c239412
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::welcome::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::welcome::ui::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::welcome::ui::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::welcome::ui::closure_env$0,ref$<str$> > @ 0x161e735 -> 0x16218a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
