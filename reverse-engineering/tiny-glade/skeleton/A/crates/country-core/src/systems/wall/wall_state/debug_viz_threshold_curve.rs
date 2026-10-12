// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/wall_state/debug_viz_threshold_curve.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 154bd979373b629eb3103c6903a7cba6488b8cfef2f7e949fc565ece60b93801
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: debug_viz_threshold_curve.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 429, 453
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
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
// 函数符号: country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":30876,"rva":6415648,"va":5375124768}
// 编译实例 RVAs: 0x61e520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: debug_viz_threshold_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5511507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375124768,"raw_hex":"e8ade6abff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375125022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz_threshold_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2376001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375124768,"raw_hex":"e8bfbedbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375125708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7e580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz_threshold_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7733202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375124768,"raw_hex":"e8d2ff7500","size":5,"soft_fail":false,"target_va":5382858112,"va":5375124905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27853117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375124768,"raw_hex":"e83d01a901","size":5,"soft_fail":false,"target_va":5402978880,"va":5375125758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27853363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375124768,"raw_hex":"e83302a901","size":5,"soft_fail":false,"target_va":5402979152,"va":5375125784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::show
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t9064814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375124768,"raw_hex":"e86e518a00","size":5,"soft_fail":false,"target_va":5384190224,"va":5375125405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":31616,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":31704,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":453,"record_offset":32920,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":32884,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":33784,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":31820,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":31100,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":22378,"source_line":54,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":453,"record_offset":33548,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":26006,"source_line":91,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::line_width
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":453,"record_offset":33368,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":26005,"source_line":108,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":453,"record_offset":33192,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":26004,"source_line":81,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":33276,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":33220,"parent_record_offset":30876,"ancestor_record_offset":30876,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":34020,"parent_record_offset":33996,"ancestor_record_offset":30876,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":33996,"parent_record_offset":33904,"ancestor_record_offset":30876,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":33932,"parent_record_offset":33904,"ancestor_record_offset":30876,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":33904,"parent_record_offset":33880,"ancestor_record_offset":30876,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":31580,"parent_record_offset":31556,"ancestor_record_offset":30876,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":33808,"parent_record_offset":33784,"ancestor_record_offset":30876,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":33856,"parent_record_offset":33832,"ancestor_record_offset":30876,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":32636,"parent_record_offset":32612,"ancestor_record_offset":30876,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":31408,"parent_record_offset":31380,"ancestor_record_offset":30876,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":32104,"parent_record_offset":32080,"ancestor_record_offset":30876,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":31468,"parent_record_offset":31436,"ancestor_record_offset":30876,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":453,"record_offset":32128,"parent_record_offset":32104,"ancestor_record_offset":30876,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":453,"record_offset":33012,"parent_record_offset":32976,"ancestor_record_offset":30876,"inlinee":21434,"source_line":107,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":453,"record_offset":32948,"parent_record_offset":32920,"ancestor_record_offset":30876,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":453,"record_offset":33036,"parent_record_offset":33012,"ancestor_record_offset":30876,"inlinee":6062,"source_line":268,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":453,"record_offset":32976,"parent_record_offset":32948,"ancestor_record_offset":30876,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":31556,"parent_record_offset":31100,"ancestor_record_offset":30876,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":453,"record_offset":31644,"parent_record_offset":31616,"ancestor_record_offset":30876,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":33960,"parent_record_offset":33932,"ancestor_record_offset":30876,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":32300,"parent_record_offset":32276,"ancestor_record_offset":30876,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":33244,"parent_record_offset":33220,"ancestor_record_offset":30876,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":33308,"parent_record_offset":33276,"ancestor_record_offset":30876,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":31352,"parent_record_offset":31320,"ancestor_record_offset":30876,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":33832,"parent_record_offset":33808,"ancestor_record_offset":30876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":31672,"parent_record_offset":31616,"ancestor_record_offset":30876,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":32548,"parent_record_offset":32524,"ancestor_record_offset":30876,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":32440,"parent_record_offset":32416,"ancestor_record_offset":30876,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":31760,"parent_record_offset":31704,"ancestor_record_offset":30876,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":31732,"parent_record_offset":31704,"ancestor_record_offset":30876,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":31784,"parent_record_offset":31760,"ancestor_record_offset":30876,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":33108,"parent_record_offset":33084,"ancestor_record_offset":30876,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":453,"record_offset":32660,"parent_record_offset":32636,"ancestor_record_offset":30876,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":453,"record_offset":32612,"parent_record_offset":32588,"ancestor_record_offset":30876,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":31880,"parent_record_offset":31856,"ancestor_record_offset":30876,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":31904,"parent_record_offset":31880,"ancestor_record_offset":30876,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":31176,"parent_record_offset":31148,"ancestor_record_offset":30876,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":32588,"parent_record_offset":32368,"ancestor_record_offset":30876,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":31932,"parent_record_offset":31904,"ancestor_record_offset":30876,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":32500,"parent_record_offset":32476,"ancestor_record_offset":30876,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":32476,"parent_record_offset":32368,"ancestor_record_offset":30876,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":32416,"parent_record_offset":32392,"ancestor_record_offset":30876,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":32796,"parent_record_offset":32008,"ancestor_record_offset":30876,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":31320,"parent_record_offset":31232,"ancestor_record_offset":30876,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":32392,"parent_record_offset":32368,"ancestor_record_offset":30876,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":31984,"parent_record_offset":31960,"ancestor_record_offset":30876,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":31960,"parent_record_offset":31904,"ancestor_record_offset":30876,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":31204,"parent_record_offset":31176,"ancestor_record_offset":30876,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":31292,"parent_record_offset":31232,"ancestor_record_offset":30876,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":31264,"parent_record_offset":31232,"ancestor_record_offset":30876,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":30876,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":31232,"parent_record_offset":31204,"ancestor_record_offset":30876,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":32248,"parent_record_offset":32220,"ancestor_record_offset":30876,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":32276,"parent_record_offset":32248,"ancestor_record_offset":30876,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":32336,"parent_record_offset":32220,"ancestor_record_offset":30876,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":32220,"parent_record_offset":32008,"ancestor_record_offset":30876,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":32052,"parent_record_offset":32008,"ancestor_record_offset":30876,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":32368,"parent_record_offset":32008,"ancestor_record_offset":30876,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":32080,"parent_record_offset":32008,"ancestor_record_offset":30876,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":31380,"parent_record_offset":31320,"ancestor_record_offset":30876,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":32164,"parent_record_offset":32008,"ancestor_record_offset":30876,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":32704,"parent_record_offset":32008,"ancestor_record_offset":30876,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":31436,"parent_record_offset":31380,"ancestor_record_offset":30876,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":31856,"parent_record_offset":31820,"ancestor_record_offset":30876,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":31148,"parent_record_offset":31124,"ancestor_record_offset":30876,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":31124,"parent_record_offset":31100,"ancestor_record_offset":30876,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":32852,"parent_record_offset":31820,"ancestor_record_offset":30876,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00453.jsonl.gz"}
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
// 完整结构及原始类型记录: debug_viz_threshold_curve.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: debug_viz_threshold_curve.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: debug_viz_threshold_curve.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve
// 信息 ID: 7ef7e0541f2bb66e16ed23de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] eb6c113aa2c4f003a1890920: void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0xc02a1 -> 0x61e520 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x10c8f6 -> 0x61e520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
