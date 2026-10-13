// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/critters/src/sheep/sheep_input_record_replay.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c2e996fcc4549241b06574114f050eee78a12d3b409b1a6e8ce4e37d5a67eb02
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sheep_input_record_replay.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2391, 2410, 3614
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3614,"record_offset":252,"rva":28542528,"va":5397251648}
// 编译实例 RVAs: 0x1b38640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24502920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397251648,"raw_hex":"e8781d8afe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397252627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a0300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21595164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397251648,"raw_hex":"e8e47bb6fe","size":5,"soft_fail":false,"target_va":5375656704,"va":5397251863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a5d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5844181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397251648,"raw_hex":"e82bd3a6ff","size":5,"soft_fail":false,"target_va":5391408400,"va":5397252576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a60e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5843361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397251648,"raw_hex":"e85fd6a6ff","size":5,"soft_fail":false,"target_va":5391409376,"va":5397252732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6341733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397251648,"raw_hex":"e865c46000","size":5,"soft_fail":false,"target_va":5403594000,"va":5397252262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6342080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397251648,"raw_hex":"e8c0c56000","size":5,"soft_fail":false,"target_va":5403594336,"va":5397252251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5725969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397251648,"raw_hex":"e8115f5700","size":5,"soft_fail":false,"target_va":5402978880,"va":5397252906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::input_stream::InputStream::last_frame_mut
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21597149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397251648,"raw_hex":"e82374b6fe","size":5,"soft_fail":false,"target_va":5375655440,"va":5397252584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3614,"record_offset":448,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3614,"record_offset":1852,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3614,"record_offset":3244,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3614,"record_offset":508,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3614,"record_offset":988,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3614,"record_offset":3452,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3614,"record_offset":708,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3614,"record_offset":3536,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3614,"record_offset":3572,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3614,"record_offset":1908,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":22233,"source_line":185,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3614,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":252,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3614,"record_offset":3720,"parent_record_offset":3620,"ancestor_record_offset":252,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3614,"record_offset":3652,"parent_record_offset":3620,"ancestor_record_offset":252,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3614,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":252,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3614,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":252,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3614,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":252,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3614,"record_offset":476,"parent_record_offset":448,"ancestor_record_offset":252,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3614,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":252,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3614,"record_offset":1496,"parent_record_offset":1208,"ancestor_record_offset":252,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3614,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":252,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3614,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":252,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3614,"record_offset":1208,"parent_record_offset":1176,"ancestor_record_offset":252,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3614,"record_offset":1176,"parent_record_offset":1136,"ancestor_record_offset":252,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3614,"record_offset":536,"parent_record_offset":508,"ancestor_record_offset":252,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3614,"record_offset":1468,"parent_record_offset":1208,"ancestor_record_offset":252,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3614,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":252,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3614,"record_offset":1384,"parent_record_offset":1208,"ancestor_record_offset":252,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3614,"record_offset":2256,"parent_record_offset":2156,"ancestor_record_offset":252,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3614,"record_offset":3144,"parent_record_offset":2912,"ancestor_record_offset":252,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3614,"record_offset":2200,"parent_record_offset":2156,"ancestor_record_offset":252,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3614,"record_offset":2488,"parent_record_offset":2428,"ancestor_record_offset":252,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3614,"record_offset":3116,"parent_record_offset":2912,"ancestor_record_offset":252,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3614,"record_offset":1024,"parent_record_offset":988,"ancestor_record_offset":252,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3614,"record_offset":1060,"parent_record_offset":1024,"ancestor_record_offset":252,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3614,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":252,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3614,"record_offset":3684,"parent_record_offset":3652,"ancestor_record_offset":252,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3614,"record_offset":1728,"parent_record_offset":1060,"ancestor_record_offset":252,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3614,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3614,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3614,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":252,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3614,"record_offset":1296,"parent_record_offset":1260,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3614,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":252,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3614,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":252,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3614,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":252,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3614,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":252,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3614,"record_offset":1324,"parent_record_offset":1260,"ancestor_record_offset":252,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3614,"record_offset":1260,"parent_record_offset":1208,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::euler::impl$10::new_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/euler.rs
//     evidence={"module":3614,"record_offset":1936,"parent_record_offset":1908,"ancestor_record_offset":252,"inlinee":22235,"source_line":252,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::euler::impl$10::new_quat::rot_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/euler.rs
//     evidence={"module":3614,"record_offset":1968,"parent_record_offset":1936,"ancestor_record_offset":252,"inlinee":22237,"source_line":259,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3614,"record_offset":2036,"parent_record_offset":2004,"ancestor_record_offset":252,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3614,"record_offset":2004,"parent_record_offset":1968,"ancestor_record_offset":252,"inlinee":21673,"source_line":169,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3614,"record_offset":2124,"parent_record_offset":2004,"ancestor_record_offset":252,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3614,"record_offset":2156,"parent_record_offset":1968,"ancestor_record_offset":252,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3614,"record_offset":2888,"parent_record_offset":1936,"ancestor_record_offset":252,"inlinee":22240,"source_line":485,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3614,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":252,"inlinee":22241,"source_line":522,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3614,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":252,"inlinee":22243,"source_line":52,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3614,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":252,"inlinee":22242,"source_line":77,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3614,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":252,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3614,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":252,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/03614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14414
// 首条位置: {"module":2391,"inlinee":82964}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14413
// 首条位置: {"module":2391,"inlinee":82963}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::to_euler
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2410,"record_offset":28624,"parent_record_offset":28600,"ancestor_record_offset":28600,"inlinee":25930,"source_line":397,"archive":"_pdb/A/modules/02410.jsonl.gz"}
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
// 完整结构及原始类型记录: sheep_input_record_replay.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sheep_input_record_replay.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sheep_input_record_replay.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay
// 信息 ID: db1684ce64682a17929cc5a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1fcca224b2574abbeaac05ae: void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0x158a851 -> 0x1b38640 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> @ 0x1597cec -> 0x1b38640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$0
// 信息 ID: 3e1df7e761ebd38109b8e3d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay", "captures": "unknown"}
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0> >; module=2410, inline_record=24472; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0> >; module=2410, inline_record=26208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$1
// 信息 ID: 147c831d42e8c8b38d9c95bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay", "captures": "unknown"}
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1> >; module=2410, inline_record=28600; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1> >; module=2410, inline_record=31140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
