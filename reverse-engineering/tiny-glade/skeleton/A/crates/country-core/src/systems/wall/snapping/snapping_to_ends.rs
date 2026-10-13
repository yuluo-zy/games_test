// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/snapping/snapping_to_ends.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 196e1fe9b1c5175b117dced06c4db8d2085e4ec569b8fc23a23c88d4d833bda6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapping_to_ends.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 453
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":24424,"rva":6413648,"va":5375122768}
// 编译实例 RVAs: 0x61dd50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: snapping_to_ends.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5509607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122768,"raw_hex":"e819eeabff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375123122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: snapping_to_ends.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2374246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122768,"raw_hex":"e89ac5dbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375123953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7e580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_ends.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7735110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122768,"raw_hex":"e846077600","size":5,"soft_fail":false,"target_va":5382858112,"va":5375122997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88710
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: snapping_to_ends.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8824242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122768,"raw_hex":"e8b2a58600","size":5,"soft_fail":false,"target_va":5383948048,"va":5375123801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t27854504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122768,"raw_hex":"e8a806a901","size":5,"soft_fail":false,"target_va":5402978880,"va":5375124371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27854747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122768,"raw_hex":"e89b07a901","size":5,"soft_fail":false,"target_va":5402979152,"va":5375124400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27854922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122768,"raw_hex":"e84a08a901","size":5,"soft_fail":false,"target_va":5402979344,"va":5375124417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":30816,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":24604,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":24632,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":453,"record_offset":24668,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":29712,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::snapping::snapping_to_ends::find_closest_end_to_snap_to
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/snapping/snapping_to_ends.rs
//     evidence={"module":453,"record_offset":24724,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":26001,"source_line":36,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":29332,"parent_record_offset":29308,"ancestor_record_offset":24424,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":29308,"parent_record_offset":29216,"ancestor_record_offset":24424,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":29244,"parent_record_offset":29216,"ancestor_record_offset":24424,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":29216,"parent_record_offset":29192,"ancestor_record_offset":24424,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":28568,"parent_record_offset":28512,"ancestor_record_offset":24424,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":25316,"parent_record_offset":25292,"ancestor_record_offset":24424,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":25352,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":29120,"parent_record_offset":29096,"ancestor_record_offset":24424,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":29168,"parent_record_offset":29144,"ancestor_record_offset":24424,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":25440,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":30840,"parent_record_offset":30816,"ancestor_record_offset":24424,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":26684,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":26372,"parent_record_offset":26348,"ancestor_record_offset":24424,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":25144,"parent_record_offset":25116,"ancestor_record_offset":24424,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":25840,"parent_record_offset":25816,"ancestor_record_offset":24424,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":25204,"parent_record_offset":25172,"ancestor_record_offset":24424,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":453,"record_offset":25864,"parent_record_offset":25840,"ancestor_record_offset":24424,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":25292,"parent_record_offset":24832,"ancestor_record_offset":24424,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":453,"record_offset":25380,"parent_record_offset":25352,"ancestor_record_offset":24424,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":29272,"parent_record_offset":29244,"ancestor_record_offset":24424,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":26036,"parent_record_offset":26012,"ancestor_record_offset":24424,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":28964,"parent_record_offset":28868,"ancestor_record_offset":24424,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":29020,"parent_record_offset":28868,"ancestor_record_offset":24424,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":26976,"parent_record_offset":26948,"ancestor_record_offset":24424,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":26824,"parent_record_offset":26740,"ancestor_record_offset":24424,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":26620,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":24692,"parent_record_offset":24668,"ancestor_record_offset":24424,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":25088,"parent_record_offset":25056,"ancestor_record_offset":24424,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":29096,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":25408,"parent_record_offset":25352,"ancestor_record_offset":24424,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":24424,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":26176,"parent_record_offset":26152,"ancestor_record_offset":24424,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":25496,"parent_record_offset":25440,"ancestor_record_offset":24424,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":25468,"parent_record_offset":25440,"ancestor_record_offset":24424,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":25520,"parent_record_offset":25496,"ancestor_record_offset":24424,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":26792,"parent_record_offset":26764,"ancestor_record_offset":24424,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":28600,"parent_record_offset":28568,"ancestor_record_offset":24424,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":28632,"parent_record_offset":28600,"ancestor_record_offset":24424,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/in_progress_curve.rs
//     evidence={"module":453,"record_offset":26740,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":26002,"source_line":130,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":453,"record_offset":26396,"parent_record_offset":26372,"ancestor_record_offset":24424,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":453,"record_offset":26656,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":25556,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":24832,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":22378,"source_line":54,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":26924,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":27004,"parent_record_offset":26948,"ancestor_record_offset":24424,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":26948,"parent_record_offset":26924,"ancestor_record_offset":24424,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":28840,"parent_record_offset":28812,"ancestor_record_offset":24424,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":28868,"parent_record_offset":28840,"ancestor_record_offset":24424,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":453,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":24424,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":25616,"parent_record_offset":25592,"ancestor_record_offset":24424,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":25640,"parent_record_offset":25616,"ancestor_record_offset":24424,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":24908,"parent_record_offset":24880,"ancestor_record_offset":24424,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":26324,"parent_record_offset":26104,"ancestor_record_offset":24424,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":25668,"parent_record_offset":25640,"ancestor_record_offset":24424,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":24424,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26212,"parent_record_offset":26104,"ancestor_record_offset":24424,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26152,"parent_record_offset":26128,"ancestor_record_offset":24424,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26532,"parent_record_offset":25744,"ancestor_record_offset":24424,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25056,"parent_record_offset":24964,"ancestor_record_offset":24424,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26128,"parent_record_offset":26104,"ancestor_record_offset":24424,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25720,"parent_record_offset":25696,"ancestor_record_offset":24424,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25696,"parent_record_offset":25640,"ancestor_record_offset":24424,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":24936,"parent_record_offset":24908,"ancestor_record_offset":24424,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25028,"parent_record_offset":24964,"ancestor_record_offset":24424,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25000,"parent_record_offset":24964,"ancestor_record_offset":24424,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25744,"parent_record_offset":25720,"ancestor_record_offset":24424,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":24964,"parent_record_offset":24936,"ancestor_record_offset":24424,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":25984,"parent_record_offset":25956,"ancestor_record_offset":24424,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":26012,"parent_record_offset":25984,"ancestor_record_offset":24424,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":26072,"parent_record_offset":25956,"ancestor_record_offset":24424,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":25956,"parent_record_offset":25744,"ancestor_record_offset":24424,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25788,"parent_record_offset":25744,"ancestor_record_offset":24424,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26104,"parent_record_offset":25744,"ancestor_record_offset":24424,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":25816,"parent_record_offset":25744,"ancestor_record_offset":24424,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":25116,"parent_record_offset":25056,"ancestor_record_offset":24424,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":25900,"parent_record_offset":25744,"ancestor_record_offset":24424,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":26440,"parent_record_offset":25744,"ancestor_record_offset":24424,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":25172,"parent_record_offset":25116,"ancestor_record_offset":24424,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":25592,"parent_record_offset":25556,"ancestor_record_offset":24424,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":24880,"parent_record_offset":24856,"ancestor_record_offset":24424,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":24856,"parent_record_offset":24832,"ancestor_record_offset":24424,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":453,"record_offset":27036,"parent_record_offset":27004,"ancestor_record_offset":24424,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":28512,"parent_record_offset":28480,"ancestor_record_offset":24424,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":28480,"parent_record_offset":24724,"ancestor_record_offset":24424,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":26588,"parent_record_offset":25556,"ancestor_record_offset":24424,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::start_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":26764,"parent_record_offset":26740,"ancestor_record_offset":24424,"inlinee":26003,"source_line":863,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":28540,"parent_record_offset":28512,"ancestor_record_offset":24424,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":28676,"parent_record_offset":28512,"ancestor_record_offset":24424,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":453,"record_offset":28812,"parent_record_offset":28512,"ancestor_record_offset":24424,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_ends::find_closest_end_to_snap_to
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6591
// 首条位置: {"module":453,"inlinee":26001}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":25352,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":25440,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":26684,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":26620,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":29096,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":26860,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/in_progress_curve.rs
//     evidence={"module":453,"record_offset":26740,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":26002,"source_line":130,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":453,"record_offset":26656,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":25556,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":24832,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":22378,"source_line":54,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":26924,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":28480,"parent_record_offset":24724,"ancestor_record_offset":24724,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":29332,"parent_record_offset":29308,"ancestor_record_offset":24724,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":29308,"parent_record_offset":29216,"ancestor_record_offset":24724,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":29244,"parent_record_offset":29216,"ancestor_record_offset":24724,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":29216,"parent_record_offset":29192,"ancestor_record_offset":24724,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":28568,"parent_record_offset":28512,"ancestor_record_offset":24724,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":25316,"parent_record_offset":25292,"ancestor_record_offset":24724,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":29120,"parent_record_offset":29096,"ancestor_record_offset":24724,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":29168,"parent_record_offset":29144,"ancestor_record_offset":24724,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":26372,"parent_record_offset":26348,"ancestor_record_offset":24724,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":25144,"parent_record_offset":25116,"ancestor_record_offset":24724,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":25840,"parent_record_offset":25816,"ancestor_record_offset":24724,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":25204,"parent_record_offset":25172,"ancestor_record_offset":24724,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":453,"record_offset":25864,"parent_record_offset":25840,"ancestor_record_offset":24724,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":25292,"parent_record_offset":24832,"ancestor_record_offset":24724,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":453,"record_offset":25380,"parent_record_offset":25352,"ancestor_record_offset":24724,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":29272,"parent_record_offset":29244,"ancestor_record_offset":24724,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":26036,"parent_record_offset":26012,"ancestor_record_offset":24724,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":28964,"parent_record_offset":28868,"ancestor_record_offset":24724,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":29020,"parent_record_offset":28868,"ancestor_record_offset":24724,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":26976,"parent_record_offset":26948,"ancestor_record_offset":24724,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":26824,"parent_record_offset":26740,"ancestor_record_offset":24724,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":25088,"parent_record_offset":25056,"ancestor_record_offset":24724,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":29144,"parent_record_offset":29120,"ancestor_record_offset":24724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":25408,"parent_record_offset":25352,"ancestor_record_offset":24724,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":24724,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":26176,"parent_record_offset":26152,"ancestor_record_offset":24724,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":25496,"parent_record_offset":25440,"ancestor_record_offset":24724,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":25468,"parent_record_offset":25440,"ancestor_record_offset":24724,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":25520,"parent_record_offset":25496,"ancestor_record_offset":24724,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":26792,"parent_record_offset":26764,"ancestor_record_offset":24724,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":28600,"parent_record_offset":28568,"ancestor_record_offset":24724,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":28632,"parent_record_offset":28600,"ancestor_record_offset":24724,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":453,"record_offset":26396,"parent_record_offset":26372,"ancestor_record_offset":24724,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":27004,"parent_record_offset":26948,"ancestor_record_offset":24724,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":26948,"parent_record_offset":26924,"ancestor_record_offset":24724,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":28840,"parent_record_offset":28812,"ancestor_record_offset":24724,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":28868,"parent_record_offset":28840,"ancestor_record_offset":24724,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":453,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":24724,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":25616,"parent_record_offset":25592,"ancestor_record_offset":24724,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":25640,"parent_record_offset":25616,"ancestor_record_offset":24724,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":24908,"parent_record_offset":24880,"ancestor_record_offset":24724,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":26324,"parent_record_offset":26104,"ancestor_record_offset":24724,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":25668,"parent_record_offset":25640,"ancestor_record_offset":24724,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":24724,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26212,"parent_record_offset":26104,"ancestor_record_offset":24724,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26152,"parent_record_offset":26128,"ancestor_record_offset":24724,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26532,"parent_record_offset":25744,"ancestor_record_offset":24724,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25056,"parent_record_offset":24964,"ancestor_record_offset":24724,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26128,"parent_record_offset":26104,"ancestor_record_offset":24724,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25720,"parent_record_offset":25696,"ancestor_record_offset":24724,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25696,"parent_record_offset":25640,"ancestor_record_offset":24724,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":24936,"parent_record_offset":24908,"ancestor_record_offset":24724,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25028,"parent_record_offset":24964,"ancestor_record_offset":24724,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25000,"parent_record_offset":24964,"ancestor_record_offset":24724,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25744,"parent_record_offset":25720,"ancestor_record_offset":24724,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":24964,"parent_record_offset":24936,"ancestor_record_offset":24724,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":25984,"parent_record_offset":25956,"ancestor_record_offset":24724,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":26012,"parent_record_offset":25984,"ancestor_record_offset":24724,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":26072,"parent_record_offset":25956,"ancestor_record_offset":24724,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":25956,"parent_record_offset":25744,"ancestor_record_offset":24724,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":25788,"parent_record_offset":25744,"ancestor_record_offset":24724,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":26104,"parent_record_offset":25744,"ancestor_record_offset":24724,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":25816,"parent_record_offset":25744,"ancestor_record_offset":24724,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":25116,"parent_record_offset":25056,"ancestor_record_offset":24724,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":25900,"parent_record_offset":25744,"ancestor_record_offset":24724,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":26440,"parent_record_offset":25744,"ancestor_record_offset":24724,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":25172,"parent_record_offset":25116,"ancestor_record_offset":24724,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":25592,"parent_record_offset":25556,"ancestor_record_offset":24724,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":24880,"parent_record_offset":24856,"ancestor_record_offset":24724,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":24856,"parent_record_offset":24832,"ancestor_record_offset":24724,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":453,"record_offset":27036,"parent_record_offset":27004,"ancestor_record_offset":24724,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":28512,"parent_record_offset":28480,"ancestor_record_offset":24724,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":26588,"parent_record_offset":25556,"ancestor_record_offset":24724,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::start_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":26764,"parent_record_offset":26740,"ancestor_record_offset":24724,"inlinee":26003,"source_line":863,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":28540,"parent_record_offset":28512,"ancestor_record_offset":24724,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":28676,"parent_record_offset":28512,"ancestor_record_offset":24724,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":453,"record_offset":28812,"parent_record_offset":28512,"ancestor_record_offset":24724,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00453.jsonl.gz"}
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
// 完整结构及原始类型记录: snapping_to_ends.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapping_to_ends.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapping_to_ends.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_ends::find_closest_end_to_snap_to
// 信息 ID: a66cba1376ba138eef46b1b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends; module=453, inline_record=24724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends
// 信息 ID: 21846a7f48d781dd4fae6a99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 066094a18fcdab94d0f5c9c5: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > @ 0xcf188 -> 0x61dd50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> @ 0x2fb5a6 -> 0x61dd50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
