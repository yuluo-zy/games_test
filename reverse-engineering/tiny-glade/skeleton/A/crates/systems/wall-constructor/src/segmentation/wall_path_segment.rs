// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 505ffbd7c4c1d117a9b06fe6b8756c9d6d3fe56bde9731f5a09556f70058732c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_path_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1230
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1230,"record_offset":868,"rva":13334816,"va":5382043936}
// 编译实例 RVAs: 0xcb7920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14035f850
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9798435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e8dd7c6aff","size":5,"soft_fail":false,"target_va":5372246096,"va":5382044526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9294807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e8292c72ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382044514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c974a0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-139042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e8dee0fdff","size":5,"soft_fail":false,"target_va":5381911712,"va":5382050749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cac790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-48117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e80b44ffff","size":5,"soft_fail":false,"target_va":5381998480,"va":5382046592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cad160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e87c4dffff","size":5,"soft_fail":false,"target_va":5382000992,"va":5382046687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cad810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e8d25affff","size":5,"soft_fail":false,"target_va":5382002704,"va":5382044985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb3bb0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e8eabdffff","size":5,"soft_fail":false,"target_va":5382028208,"va":5382045121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc87b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t66421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e875030100","size":5,"soft_fail":false,"target_va":5382113200,"va":5382046774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t208808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e8a82f0300","size":5,"soft_fail":false,"target_va":5382255968,"va":5382047155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t209167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e80f310300","size":5,"soft_fail":false,"target_va":5382256288,"va":5382047116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10739349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e86b215cff","size":5,"soft_fail":false,"target_va":5371305056,"va":5382044400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t20930892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e84c613f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382048255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t20931382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e836633f01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382047957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20930080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e8205e3f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382048859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::u_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-111460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e89c4cfeff","size":5,"soft_fail":false,"target_va":5381933088,"va":5382044543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopIdx::index
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-17090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e83ebdffff","size":5,"soft_fail":false,"target_va":5382028080,"va":5382045165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::is_empty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e89fb4ffff","size":5,"soft_fail":false,"target_va":5382028320,"va":5382047612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::subslice
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043936,"raw_hex":"e869b6ffff","size":5,"soft_fail":false,"target_va":5382028752,"va":5382047586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1230,"record_offset":1128,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1230,"record_offset":1448,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1230,"record_offset":5860,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1230,"record_offset":6052,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1230,"record_offset":2984,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1230,"record_offset":7512,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1230,"record_offset":1296,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1230,"record_offset":3884,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1230,"record_offset":2368,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1230,"record_offset":2340,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1230,"record_offset":2256,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1230,"record_offset":5748,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":34234,"source_line":3416,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1230,"record_offset":3800,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$106::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1230,"record_offset":5552,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":17486,"source_line":35,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1230,"record_offset":1600,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1230,"record_offset":1212,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1230,"record_offset":3716,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1230,"record_offset":3012,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1230,"record_offset":1904,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1230,"record_offset":2172,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
//     evidence={"module":1230,"record_offset":6148,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":59695,"source_line":100,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1230,"record_offset":1156,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":21479,"source_line":174,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] utils::slice_iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1230,"record_offset":2200,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":59691,"source_line":619,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1230,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":868,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1230,"record_offset":1752,"parent_record_offset":1652,"ancestor_record_offset":868,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1230,"record_offset":1684,"parent_record_offset":1652,"ancestor_record_offset":868,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1230,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":868,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1230,"record_offset":1652,"parent_record_offset":1624,"ancestor_record_offset":868,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1230,"record_offset":1484,"parent_record_offset":1448,"ancestor_record_offset":868,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1230,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":868,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1230,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":868,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1230,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":868,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1230,"record_offset":2668,"parent_record_offset":2636,"ancestor_record_offset":868,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1230,"record_offset":6556,"parent_record_offset":6404,"ancestor_record_offset":868,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1230,"record_offset":6584,"parent_record_offset":6404,"ancestor_record_offset":868,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::Copied::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1230,"record_offset":5772,"parent_record_offset":5748,"ancestor_record_offset":868,"inlinee":32552,"source_line":26,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1230,"record_offset":2416,"parent_record_offset":2368,"ancestor_record_offset":868,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1230,"record_offset":4280,"parent_record_offset":4080,"ancestor_record_offset":868,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1230,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":868,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1230,"record_offset":3932,"parent_record_offset":3884,"ancestor_record_offset":868,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1230,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":868,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1230,"record_offset":3980,"parent_record_offset":3932,"ancestor_record_offset":868,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1230,"record_offset":2700,"parent_record_offset":2636,"ancestor_record_offset":868,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1230,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":868,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1230,"record_offset":2608,"parent_record_offset":2572,"ancestor_record_offset":868,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1230,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":868,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1230,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":868,"inlinee":15501,"source_line":305,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1230,"record_offset":2544,"parent_record_offset":2416,"ancestor_record_offset":868,"inlinee":11441,"source_line":357,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1230,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":868,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1230,"record_offset":2460,"parent_record_offset":2416,"ancestor_record_offset":868,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1230,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":868,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1230,"record_offset":1716,"parent_record_offset":1684,"ancestor_record_offset":868,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":1230,"record_offset":6244,"parent_record_offset":6204,"ancestor_record_offset":868,"inlinee":21488,"source_line":2775,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1230,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":868,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1230,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":868,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1230,"record_offset":6444,"parent_record_offset":6404,"ancestor_record_offset":868,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1230,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":868,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1230,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":868,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1230,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":868,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1230,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":868,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1230,"record_offset":1356,"parent_record_offset":1320,"ancestor_record_offset":868,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1230,"record_offset":1540,"parent_record_offset":1448,"ancestor_record_offset":868,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1230,"record_offset":5888,"parent_record_offset":5860,"ancestor_record_offset":868,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1230,"record_offset":6080,"parent_record_offset":6052,"ancestor_record_offset":868,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1230,"record_offset":5916,"parent_record_offset":5888,"ancestor_record_offset":868,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1230,"record_offset":6108,"parent_record_offset":6080,"ancestor_record_offset":868,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1230,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":868,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1230,"record_offset":1384,"parent_record_offset":1320,"ancestor_record_offset":868,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1230,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":868,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1230,"record_offset":4220,"parent_record_offset":4128,"ancestor_record_offset":868,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1230,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":868,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1230,"record_offset":4128,"parent_record_offset":4080,"ancestor_record_offset":868,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1230,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":868,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1230,"record_offset":6376,"parent_record_offset":6148,"ancestor_record_offset":868,"inlinee":59697,"source_line":25,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1230,"record_offset":6404,"parent_record_offset":6148,"ancestor_record_offset":868,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$9::iterator_slice_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1230,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":868,"inlinee":59693,"source_line":606,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$6::next
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1230,"record_offset":2768,"parent_record_offset":2572,"ancestor_record_offset":868,"inlinee":59615,"source_line":211,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$6::next::closure$0
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1230,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":868,"inlinee":59617,"source_line":212,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1230,"record_offset":6204,"parent_record_offset":6148,"ancestor_record_offset":868,"inlinee":21487,"source_line":225,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE92F
// 首条位置: {"module":1230,"inlinee":59695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1230,"record_offset":6376,"parent_record_offset":6148,"ancestor_record_offset":6148,"inlinee":59697,"source_line":25,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1230,"record_offset":6404,"parent_record_offset":6148,"ancestor_record_offset":6148,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1230,"record_offset":6204,"parent_record_offset":6148,"ancestor_record_offset":6148,"inlinee":21487,"source_line":225,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1230,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":6148,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1230,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":6148,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1230,"record_offset":6556,"parent_record_offset":6404,"ancestor_record_offset":6148,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1230,"record_offset":6584,"parent_record_offset":6404,"ancestor_record_offset":6148,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":1230,"record_offset":6244,"parent_record_offset":6204,"ancestor_record_offset":6148,"inlinee":21488,"source_line":2775,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1230,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":6148,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01230.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1230,"record_offset":6444,"parent_record_offset":6404,"ancestor_record_offset":6148,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01230.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_path_segment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_path_segment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_path_segment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
// 信息 ID: 43f192dfea04d3d73b30ca47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d82e0 -> 0xcb7920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs @ 0x4dcc9d -> 0xcb7920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4de22e -> 0xcb7920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments::closure$0
// 信息 ID: 77b49dcb8ed87805960ecf2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments; module=1230, inline_record=6148; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments; module=1230, inline_record=6716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
