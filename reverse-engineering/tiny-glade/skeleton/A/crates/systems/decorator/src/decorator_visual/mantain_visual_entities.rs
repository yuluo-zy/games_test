// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/mantain_visual_entities.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7baf7a7e8a78e805272c02070db998d57dde1a30615893c253b8d2023bd42dd5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mantain_visual_entities.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1293, 2760, 2789
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
// use system_decorator; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::mantain_visual_entities::clear_out_visual_entities
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2789,"record_offset":256,"rva":25045520,"va":5393754640}
// 编译实例 RVAs: 0x17e2a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mantain_visual_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21005197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393754640,"raw_hex":"e8737cbffe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393754904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d02620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mantain_visual_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11405006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393754640,"raw_hex":"e832f951ff","size":5,"soft_fail":false,"target_va":5382350368,"va":5393755369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16509951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393754640,"raw_hex":"e8011404ff","size":5,"soft_fail":false,"target_va":5377245296,"va":5393755242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9216670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393754640,"raw_hex":"e89ea28c00","size":5,"soft_fail":false,"target_va":5402972080,"va":5393755405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16449732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393754640,"raw_hex":"e83cff04ff","size":5,"soft_fail":false,"target_va":5377305552,"va":5393755279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2789,"record_offset":440,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2789,"record_offset":3580,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2789,"record_offset":1152,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual_state::DecoratorTransientVisualState::drain
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":2789,"record_offset":500,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":90004,"source_line":30,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2789,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2789,"record_offset":1824,"parent_record_offset":1732,"ancestor_record_offset":256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2789,"record_offset":1760,"parent_record_offset":1732,"ancestor_record_offset":256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2789,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2789,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":256,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2789,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":256,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2789,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":256,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2789,"record_offset":1368,"parent_record_offset":1340,"ancestor_record_offset":256,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2789,"record_offset":468,"parent_record_offset":440,"ancestor_record_offset":256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2789,"record_offset":3616,"parent_record_offset":3580,"ancestor_record_offset":256,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2789,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":256,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2789,"record_offset":820,"parent_record_offset":792,"ancestor_record_offset":256,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2789,"record_offset":880,"parent_record_offset":848,"ancestor_record_offset":256,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::FlatMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2789,"record_offset":1112,"parent_record_offset":1084,"ancestor_record_offset":256,"inlinee":16081,"source_line":24,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2789,"record_offset":1240,"parent_record_offset":1184,"ancestor_record_offset":256,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2789,"record_offset":1184,"parent_record_offset":1152,"ancestor_record_offset":256,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":2789,"record_offset":1948,"parent_record_offset":1916,"ancestor_record_offset":256,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":2789,"record_offset":1916,"parent_record_offset":1184,"ancestor_record_offset":256,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2789,"record_offset":1340,"parent_record_offset":1312,"ancestor_record_offset":256,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::flat_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2789,"record_offset":1084,"parent_record_offset":500,"ancestor_record_offset":256,"inlinee":16079,"source_line":1456,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2789,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":256,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2789,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2789,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":256,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2789,"record_offset":1312,"parent_record_offset":1240,"ancestor_record_offset":256,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2789,"record_offset":2796,"parent_record_offset":2764,"ancestor_record_offset":256,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2789,"record_offset":1280,"parent_record_offset":1240,"ancestor_record_offset":256,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2789,"record_offset":1528,"parent_record_offset":1340,"ancestor_record_offset":256,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2789,"record_offset":764,"parent_record_offset":732,"ancestor_record_offset":256,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2789,"record_offset":1564,"parent_record_offset":1240,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2789,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2789,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":256,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2789,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":256,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2789,"record_offset":1440,"parent_record_offset":1368,"ancestor_record_offset":256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2789,"record_offset":1468,"parent_record_offset":1368,"ancestor_record_offset":256,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2789,"record_offset":1404,"parent_record_offset":1368,"ancestor_record_offset":256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2789,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":256,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2789,"record_offset":1036,"parent_record_offset":980,"ancestor_record_offset":256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::drain
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2789,"record_offset":552,"parent_record_offset":524,"ancestor_record_offset":256,"inlinee":24733,"source_line":890,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$81::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2789,"record_offset":2072,"parent_record_offset":2040,"ancestor_record_offset":256,"inlinee":24739,"source_line":4952,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":2548,"parent_record_offset":2176,"ancestor_record_offset":256,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":732,"parent_record_offset":640,"ancestor_record_offset":256,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":2176,"parent_record_offset":2140,"ancestor_record_offset":256,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":256,"inlinee":24734,"source_line":1622,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain_iter_from
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":956,"parent_record_offset":580,"ancestor_record_offset":256,"inlinee":24735,"source_line":1637,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":256,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":704,"parent_record_offset":640,"ancestor_record_offset":256,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":676,"parent_record_offset":640,"ancestor_record_offset":256,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":640,"parent_record_offset":612,"ancestor_record_offset":256,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2789,"record_offset":2280,"parent_record_offset":2236,"ancestor_record_offset":256,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2789,"record_offset":2316,"parent_record_offset":2280,"ancestor_record_offset":256,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2789,"record_offset":2376,"parent_record_offset":2236,"ancestor_record_offset":256,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2789,"record_offset":2236,"parent_record_offset":2176,"ancestor_record_offset":256,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":2140,"parent_record_offset":2104,"ancestor_record_offset":256,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$46::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2789,"record_offset":2104,"parent_record_offset":2072,"ancestor_record_offset":256,"inlinee":24741,"source_line":4415,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2789,"record_offset":792,"parent_record_offset":732,"ancestor_record_offset":256,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2789,"record_offset":848,"parent_record_offset":792,"ancestor_record_offset":256,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2789,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":256,"inlinee":24732,"source_line":618,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$68::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2789,"record_offset":2040,"parent_record_offset":2008,"ancestor_record_offset":256,"inlinee":24737,"source_line":2553,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$1::drain::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":2789,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":256,"inlinee":90006,"source_line":33,"archive":"_pdb/A/modules/02789.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::mantain_visual_entities::clear_out_wall_holes
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB96
// 首条位置: {"module":1293,"inlinee":60310}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1293,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":5672,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01293.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1293,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":5672,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01293.jsonl.gz"}
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
// 完整结构及原始类型记录: mantain_visual_entities.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mantain_visual_entities.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mantain_visual_entities.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 46; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::CacheLights>,bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::event::EventReader<country_core::resources::events::LightLanternCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::CacheLights>", "bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::event::EventReader<country_core::resources::events::LightLanternCmd>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::CacheLights>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::LightLanternCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::system_param::Local<system_decorator::ui_systems::ui_rotate_lantern::EditingState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::system_param::Local<system_decorator::ui_systems::ui_rotate_lantern::EditingState>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents> >", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::ui_systems::ui_rotate_lantern::EditingState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::mantain_visual_entities::clear_out_visual_entities
// 信息 ID: 44e5d9fa5160e0fbbc79cc15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c02953a16fade6322fbb9ac7: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands)> @ 0xd14ef4 -> 0x17e2a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands)> > @ 0x17cae2e -> 0x17e2a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::mantain_visual_entities::clear_out_wall_holes
// 信息 ID: 84e5e26f5fa31c78be35a91e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ce40887b725066b6674dbc04: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>)>; module=1293, inline_record=5672; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>)> >; module=2760, inline_record=579268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
