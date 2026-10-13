// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/walls/decorator_outline.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a1af0d9a59dded4f699fa53520315f312a52fc5a38a8cdb567d39044415092e2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_outline.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 461, 1308, 1321
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":461,"record_offset":40312,"rva":6509152,"va":5375218272}
// 编译实例 RVAs: 0x635260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2468674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218272,"raw_hex":"e8be54daff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375218365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27759557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218272,"raw_hex":"e8c593a701","size":5,"soft_fail":false,"target_va":5402978144,"va":5375218582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2308546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218272,"raw_hex":"e8c2392300","size":5,"soft_fail":false,"target_va":5377527120,"va":5375218569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8982109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218272,"raw_hex":"e85d0e8900","size":5,"soft_fail":false,"target_va":5384200656,"va":5375218542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":461,"record_offset":40488,"parent_record_offset":40312,"ancestor_record_offset":40312,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::line_width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":461,"record_offset":41660,"parent_record_offset":40312,"ancestor_record_offset":40312,"inlinee":24410,"source_line":400,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":40804,"parent_record_offset":40780,"ancestor_record_offset":40312,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":40828,"parent_record_offset":40804,"ancestor_record_offset":40312,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":40780,"parent_record_offset":40688,"ancestor_record_offset":40312,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":40688,"parent_record_offset":40660,"ancestor_record_offset":40312,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":40660,"parent_record_offset":40632,"ancestor_record_offset":40312,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":40900,"parent_record_offset":40660,"ancestor_record_offset":40312,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":40968,"parent_record_offset":40944,"ancestor_record_offset":40312,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":40604,"parent_record_offset":40572,"ancestor_record_offset":40312,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":40632,"parent_record_offset":40604,"ancestor_record_offset":40312,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":40516,"parent_record_offset":40488,"ancestor_record_offset":40312,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":41560,"parent_record_offset":41536,"ancestor_record_offset":40312,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":461,"record_offset":40944,"parent_record_offset":40572,"ancestor_record_offset":40312,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":461,"record_offset":40544,"parent_record_offset":40516,"ancestor_record_offset":40312,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":461,"record_offset":40572,"parent_record_offset":40544,"ancestor_record_offset":40312,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":461,"record_offset":40724,"parent_record_offset":40688,"ancestor_record_offset":40312,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":461,"record_offset":40748,"parent_record_offset":40724,"ancestor_record_offset":40312,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":461,"record_offset":41256,"parent_record_offset":41156,"ancestor_record_offset":40312,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":461,"record_offset":41228,"parent_record_offset":41156,"ancestor_record_offset":40312,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":461,"record_offset":41196,"parent_record_offset":41156,"ancestor_record_offset":40312,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":461,"record_offset":41016,"parent_record_offset":40992,"ancestor_record_offset":40312,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":461,"record_offset":41068,"parent_record_offset":41040,"ancestor_record_offset":40312,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":461,"record_offset":40992,"parent_record_offset":40968,"ancestor_record_offset":40312,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":461,"record_offset":41536,"parent_record_offset":41068,"ancestor_record_offset":40312,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":40856,"parent_record_offset":40828,"ancestor_record_offset":40312,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":41584,"parent_record_offset":41560,"ancestor_record_offset":40312,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":461,"record_offset":41040,"parent_record_offset":41016,"ancestor_record_offset":40312,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_outline::impl$0::display::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_outline.rs
//     evidence={"module":461,"record_offset":41092,"parent_record_offset":41068,"ancestor_record_offset":40312,"inlinee":26365,"source_line":31,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":461,"record_offset":41116,"parent_record_offset":41092,"ancestor_record_offset":40312,"inlinee":26366,"source_line":370,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":461,"record_offset":41156,"parent_record_offset":41116,"ancestor_record_offset":40312,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$0::display::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66FD
// 首条位置: {"module":461,"inlinee":26365}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":461,"record_offset":41116,"parent_record_offset":41092,"ancestor_record_offset":41092,"inlinee":26366,"source_line":370,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":461,"record_offset":41256,"parent_record_offset":41156,"ancestor_record_offset":41092,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":461,"record_offset":41228,"parent_record_offset":41156,"ancestor_record_offset":41092,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":461,"record_offset":41196,"parent_record_offset":41156,"ancestor_record_offset":41092,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":461,"record_offset":41156,"parent_record_offset":41116,"ancestor_record_offset":41092,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::DecoratorOutline::display
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":461,"record_offset":41692,"rva":6509472,"va":5375218592}
// 编译实例 RVAs: 0x6353a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5586917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e81bc0aaff","size":5,"soft_fail":false,"target_va":5369632448,"va":5375219360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3570493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e8c384c9ff","size":5,"soft_fail":false,"target_va":5371650384,"va":5375220872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2469725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e8a350daff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375219416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2470460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e8c44ddaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375220167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-438056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e8d850f9ff","size":5,"soft_fail":false,"target_va":5374781984,"va":5375220035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28373540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e824f2b001","size":5,"soft_fail":false,"target_va":5403593360,"va":5375219815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t27757892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e8448da701","size":5,"soft_fail":false,"target_va":5402978144,"va":5375220247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2310557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e89d412300","size":5,"soft_fail":false,"target_va":5377529888,"va":5375219326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2306805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375218592,"raw_hex":"e8f5322300","size":5,"soft_fail":false,"target_va":5377527120,"va":5375220310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":461,"record_offset":42852,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":461,"record_offset":42936,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":44256,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":461,"record_offset":41888,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":47212,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":43424,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":461,"record_offset":47128,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_affine3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":461,"record_offset":41972,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":26367,"source_line":719,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":42764,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":42676,"parent_record_offset":41692,"ancestor_record_offset":41692,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":43064,"parent_record_offset":43040,"ancestor_record_offset":41692,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":43088,"parent_record_offset":43064,"ancestor_record_offset":41692,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":43360,"parent_record_offset":43336,"ancestor_record_offset":41692,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":43008,"parent_record_offset":42972,"ancestor_record_offset":41692,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":43040,"parent_record_offset":43008,"ancestor_record_offset":41692,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":43336,"parent_record_offset":43280,"ancestor_record_offset":41692,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":461,"record_offset":42972,"parent_record_offset":42936,"ancestor_record_offset":41692,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":461,"record_offset":42900,"parent_record_offset":42876,"ancestor_record_offset":41692,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":43308,"parent_record_offset":43280,"ancestor_record_offset":41692,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":43820,"parent_record_offset":43788,"ancestor_record_offset":41692,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":43788,"parent_record_offset":43756,"ancestor_record_offset":41692,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":43976,"parent_record_offset":43788,"ancestor_record_offset":41692,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":43280,"parent_record_offset":43256,"ancestor_record_offset":41692,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":43712,"parent_record_offset":43672,"ancestor_record_offset":41692,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":43672,"parent_record_offset":43632,"ancestor_record_offset":41692,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":43592,"parent_record_offset":43552,"ancestor_record_offset":41692,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":43632,"parent_record_offset":43592,"ancestor_record_offset":41692,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":43552,"parent_record_offset":43512,"ancestor_record_offset":41692,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":461,"record_offset":43512,"parent_record_offset":43472,"ancestor_record_offset":41692,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":461,"record_offset":43472,"parent_record_offset":43424,"ancestor_record_offset":41692,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":461,"record_offset":44124,"parent_record_offset":44100,"ancestor_record_offset":41692,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":43756,"parent_record_offset":43712,"ancestor_record_offset":41692,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":461,"record_offset":44536,"parent_record_offset":44492,"ancestor_record_offset":41692,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":461,"record_offset":44560,"parent_record_offset":44536,"ancestor_record_offset":41692,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":461,"record_offset":42020,"parent_record_offset":41972,"ancestor_record_offset":41692,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":461,"record_offset":44040,"parent_record_offset":44016,"ancestor_record_offset":41692,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":461,"record_offset":41936,"parent_record_offset":41912,"ancestor_record_offset":41692,"inlinee":17599,"source_line":627,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":461,"record_offset":46204,"parent_record_offset":46176,"ancestor_record_offset":41692,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":461,"record_offset":42704,"parent_record_offset":42676,"ancestor_record_offset":41692,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":461,"record_offset":42792,"parent_record_offset":42764,"ancestor_record_offset":41692,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":461,"record_offset":42876,"parent_record_offset":42852,"ancestor_record_offset":41692,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":44016,"parent_record_offset":43712,"ancestor_record_offset":41692,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":43160,"parent_record_offset":42972,"ancestor_record_offset":41692,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":43116,"parent_record_offset":43088,"ancestor_record_offset":41692,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":461,"record_offset":44160,"parent_record_offset":44124,"ancestor_record_offset":41692,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":461,"record_offset":44184,"parent_record_offset":44160,"ancestor_record_offset":41692,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":461,"record_offset":44216,"parent_record_offset":44124,"ancestor_record_offset":41692,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":461,"record_offset":42404,"parent_record_offset":42272,"ancestor_record_offset":41692,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":461,"record_offset":42272,"parent_record_offset":41972,"ancestor_record_offset":41692,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":461,"record_offset":42460,"parent_record_offset":42272,"ancestor_record_offset":41692,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$47::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":461,"record_offset":41912,"parent_record_offset":41888,"ancestor_record_offset":41692,"inlinee":23171,"source_line":1375,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":461,"record_offset":42428,"parent_record_offset":42404,"ancestor_record_offset":41692,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":461,"record_offset":47652,"rva":6512320,"va":5375221440}
// 编译实例 RVAs: 0x635ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5589087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375221440,"raw_hex":"e8a1b7aaff","size":5,"soft_fail":false,"target_va":5369632448,"va":5375221530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2471810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375221440,"raw_hex":"e87e48daff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375221501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2472381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375221440,"raw_hex":"e84346daff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375222088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t118401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375221440,"raw_hex":"e881ce0100","size":5,"soft_fail":false,"target_va":5375340432,"va":5375222026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27756164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375221440,"raw_hex":"e88486a701","size":5,"soft_fail":false,"target_va":5402978144,"va":5375221975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27697248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375221440,"raw_hex":"e860a0a601","size":5,"soft_fail":false,"target_va":5402919072,"va":5375221819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2024689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375221440,"raw_hex":"e8f1e41e00","size":5,"soft_fail":false,"target_va":5377246528,"va":5375221834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t135118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375221440,"raw_hex":"e8ce0f0200","size":5,"soft_fail":false,"target_va":5375356896,"va":5375221773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":47836,"parent_record_offset":47652,"ancestor_record_offset":47652,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":49136,"parent_record_offset":47652,"ancestor_record_offset":47652,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":48580,"parent_record_offset":47652,"ancestor_record_offset":47652,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":49220,"parent_record_offset":47652,"ancestor_record_offset":47652,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":47984,"parent_record_offset":47652,"ancestor_record_offset":47652,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":48552,"parent_record_offset":47652,"ancestor_record_offset":47652,"inlinee":26369,"source_line":184,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":47892,"parent_record_offset":47868,"ancestor_record_offset":47652,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":47916,"parent_record_offset":47892,"ancestor_record_offset":47652,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":49292,"parent_record_offset":49268,"ancestor_record_offset":47652,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":47868,"parent_record_offset":47836,"ancestor_record_offset":47652,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":49268,"parent_record_offset":49244,"ancestor_record_offset":47652,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":461,"record_offset":49244,"parent_record_offset":49220,"ancestor_record_offset":47652,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":48808,"parent_record_offset":48780,"ancestor_record_offset":47652,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":48780,"parent_record_offset":48748,"ancestor_record_offset":47652,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":48748,"parent_record_offset":48720,"ancestor_record_offset":47652,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":48284,"parent_record_offset":48256,"ancestor_record_offset":47652,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":48256,"parent_record_offset":48228,"ancestor_record_offset":47652,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":48428,"parent_record_offset":48256,"ancestor_record_offset":47652,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":48192,"parent_record_offset":48164,"ancestor_record_offset":47652,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":49160,"parent_record_offset":49136,"ancestor_record_offset":47652,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":48164,"parent_record_offset":48136,"ancestor_record_offset":47652,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":48108,"parent_record_offset":48080,"ancestor_record_offset":47652,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":48136,"parent_record_offset":48108,"ancestor_record_offset":47652,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":48080,"parent_record_offset":48052,"ancestor_record_offset":47652,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":461,"record_offset":48052,"parent_record_offset":48024,"ancestor_record_offset":47652,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":461,"record_offset":48024,"parent_record_offset":47984,"ancestor_record_offset":47652,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":49184,"parent_record_offset":49160,"ancestor_record_offset":47652,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":48860,"parent_record_offset":48692,"ancestor_record_offset":47652,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":48720,"parent_record_offset":48692,"ancestor_record_offset":47652,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":48228,"parent_record_offset":48192,"ancestor_record_offset":47652,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":48628,"parent_record_offset":48604,"ancestor_record_offset":47652,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":48656,"parent_record_offset":48628,"ancestor_record_offset":47652,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":48604,"parent_record_offset":48580,"ancestor_record_offset":47652,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":461,"record_offset":48892,"parent_record_offset":48656,"ancestor_record_offset":47652,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":461,"record_offset":48692,"parent_record_offset":48656,"ancestor_record_offset":47652,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":461,"record_offset":48488,"parent_record_offset":48464,"ancestor_record_offset":47652,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":48464,"parent_record_offset":48192,"ancestor_record_offset":47652,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":47940,"parent_record_offset":47916,"ancestor_record_offset":47652,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":461,"record_offset":49336,"rva":6512992,"va":5375222112}
// 编译实例 RVAs: 0x636160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2472488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e8d845daff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375222179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2473139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e84d43daff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375222846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t117515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e80bcb0100","size":5,"soft_fail":false,"target_va":5375340432,"va":5375222912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28371055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e86fe8b001","size":5,"soft_fail":false,"target_va":5403593360,"va":5375222300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27755385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e87983a701","size":5,"soft_fail":false,"target_va":5402978144,"va":5375222754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27696475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e85b9da601","size":5,"soft_fail":false,"target_va":5402919072,"va":5375222592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2304339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e853292300","size":5,"soft_fail":false,"target_va":5377527120,"va":5375222776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2023916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e8ece11e00","size":5,"soft_fail":false,"target_va":5377246528,"va":5375222607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t134345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222112,"raw_hex":"e8c90c0200","size":5,"soft_fail":false,"target_va":5375356896,"va":5375222546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":49524,"parent_record_offset":49336,"ancestor_record_offset":49336,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":51332,"parent_record_offset":49336,"ancestor_record_offset":49336,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":49672,"parent_record_offset":49336,"ancestor_record_offset":49336,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":50776,"parent_record_offset":49336,"ancestor_record_offset":49336,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":51416,"parent_record_offset":49336,"ancestor_record_offset":49336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":50172,"parent_record_offset":49336,"ancestor_record_offset":49336,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":50748,"parent_record_offset":49336,"ancestor_record_offset":49336,"inlinee":26369,"source_line":184,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":49580,"parent_record_offset":49556,"ancestor_record_offset":49336,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":49604,"parent_record_offset":49580,"ancestor_record_offset":49336,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":51488,"parent_record_offset":51464,"ancestor_record_offset":49336,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":49556,"parent_record_offset":49524,"ancestor_record_offset":49336,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":51464,"parent_record_offset":51440,"ancestor_record_offset":49336,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":461,"record_offset":51440,"parent_record_offset":51416,"ancestor_record_offset":49336,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":51004,"parent_record_offset":50976,"ancestor_record_offset":49336,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":50976,"parent_record_offset":50944,"ancestor_record_offset":49336,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":50944,"parent_record_offset":50916,"ancestor_record_offset":49336,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":49848,"parent_record_offset":49820,"ancestor_record_offset":49336,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":49820,"parent_record_offset":49792,"ancestor_record_offset":49336,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":50060,"parent_record_offset":49820,"ancestor_record_offset":49336,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":49756,"parent_record_offset":49728,"ancestor_record_offset":49336,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":51356,"parent_record_offset":51332,"ancestor_record_offset":49336,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":49728,"parent_record_offset":49700,"ancestor_record_offset":49336,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":50300,"parent_record_offset":50272,"ancestor_record_offset":49336,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":49700,"parent_record_offset":49672,"ancestor_record_offset":49336,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":50272,"parent_record_offset":50244,"ancestor_record_offset":49336,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":461,"record_offset":50244,"parent_record_offset":50216,"ancestor_record_offset":49336,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":461,"record_offset":50216,"parent_record_offset":50172,"ancestor_record_offset":49336,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":51380,"parent_record_offset":51356,"ancestor_record_offset":49336,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":51056,"parent_record_offset":50888,"ancestor_record_offset":49336,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":50916,"parent_record_offset":50888,"ancestor_record_offset":49336,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":49792,"parent_record_offset":49756,"ancestor_record_offset":49336,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":50824,"parent_record_offset":50800,"ancestor_record_offset":49336,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":50852,"parent_record_offset":50824,"ancestor_record_offset":49336,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":50800,"parent_record_offset":50776,"ancestor_record_offset":49336,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":461,"record_offset":51088,"parent_record_offset":50852,"ancestor_record_offset":49336,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":461,"record_offset":50888,"parent_record_offset":50852,"ancestor_record_offset":49336,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":461,"record_offset":49884,"parent_record_offset":49848,"ancestor_record_offset":49336,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":461,"record_offset":49908,"parent_record_offset":49884,"ancestor_record_offset":49336,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":461,"record_offset":50124,"parent_record_offset":50100,"ancestor_record_offset":49336,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":50100,"parent_record_offset":49756,"ancestor_record_offset":49336,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":49628,"parent_record_offset":49604,"ancestor_record_offset":49336,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":461,"record_offset":51532,"rva":6513824,"va":5375222944}
// 编译实例 RVAs: 0x6364a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2473316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222944,"raw_hex":"e89c42daff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375223007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2473772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222944,"raw_hex":"e8d440daff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375223479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t116895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222944,"raw_hex":"e89fc80100","size":5,"soft_fail":false,"target_va":5375340432,"va":5375223532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27754741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222944,"raw_hex":"e8f580a701","size":5,"soft_fail":false,"target_va":5402978144,"va":5375223398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t133557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375222944,"raw_hex":"e8b5090200","size":5,"soft_fail":false,"target_va":5375356896,"va":5375223334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":51716,"parent_record_offset":51532,"ancestor_record_offset":51532,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":53020,"parent_record_offset":51532,"ancestor_record_offset":51532,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":461,"record_offset":51864,"parent_record_offset":51532,"ancestor_record_offset":51532,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":53104,"parent_record_offset":51532,"ancestor_record_offset":51532,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":52420,"parent_record_offset":51532,"ancestor_record_offset":51532,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":52992,"parent_record_offset":51532,"ancestor_record_offset":51532,"inlinee":26370,"source_line":179,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":51772,"parent_record_offset":51748,"ancestor_record_offset":51532,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":51796,"parent_record_offset":51772,"ancestor_record_offset":51532,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":53176,"parent_record_offset":53152,"ancestor_record_offset":51532,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":51748,"parent_record_offset":51716,"ancestor_record_offset":51532,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":53152,"parent_record_offset":53128,"ancestor_record_offset":51532,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":461,"record_offset":53128,"parent_record_offset":53104,"ancestor_record_offset":51532,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":52152,"parent_record_offset":52124,"ancestor_record_offset":51532,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":52124,"parent_record_offset":52096,"ancestor_record_offset":51532,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":52296,"parent_record_offset":52124,"ancestor_record_offset":51532,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":52060,"parent_record_offset":52032,"ancestor_record_offset":51532,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":53044,"parent_record_offset":53020,"ancestor_record_offset":51532,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":52032,"parent_record_offset":52004,"ancestor_record_offset":51532,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":51976,"parent_record_offset":51948,"ancestor_record_offset":51532,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":52004,"parent_record_offset":51976,"ancestor_record_offset":51532,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":51948,"parent_record_offset":51920,"ancestor_record_offset":51532,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":461,"record_offset":51920,"parent_record_offset":51892,"ancestor_record_offset":51532,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":461,"record_offset":51892,"parent_record_offset":51864,"ancestor_record_offset":51532,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":53068,"parent_record_offset":53044,"ancestor_record_offset":51532,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":52096,"parent_record_offset":52060,"ancestor_record_offset":51532,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":461,"record_offset":52356,"parent_record_offset":52332,"ancestor_record_offset":51532,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":52332,"parent_record_offset":52060,"ancestor_record_offset":51532,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":51820,"parent_record_offset":51796,"ancestor_record_offset":51532,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
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
// 完整结构及原始类型记录: decorator_outline.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "country_core::resources::walls::decorator_outline::DecoratorCollisionOutline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "country_core::resources::walls::decorator_outline::DecoratorCollisionOutline", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_outline::DecoratorCollisionOutline,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "country_core::resources::walls::decorator_outline::DecoratorCollisionOutline", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::decorator_outline::DecoratorOutline
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>", "constructor": "enum2$", "arguments": ["country_core::resources::walls::decorator_outline::DecoratorOutline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_outline.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_outline.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display
// 信息 ID: 11796dde58626b02f717135c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1c35325 -> 0x635260 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1c36432 -> 0x635260 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1c35613 -> 0x635260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::DecoratorOutline::display
// 信息 ID: 064cbf8919c8aec0c2cf4040
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze @ 0x69bb34 -> 0x6353a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_color_decorator::ui_color_windows_and_flags @ 0x1ac15c9 -> 0x6353a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c76c73 -> 0x6353a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$0::display::closure$0
// 信息 ID: 187896c17d076fd346c8b9a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$0::display", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display; module=461, inline_record=41092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display::closure$0
// 信息 ID: edc92a4c5c9dc0f9ccda2ea0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0 <- core::ptr::drop_in_place<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x634953 -> 0x635ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display::closure$1
// 信息 ID: a014abf5470d611ac913a215
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1 <- core::ptr::drop_in_place<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x634a03 -> 0x636160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display::closure$2
// 信息 ID: 38c22b1f016c8b19e41c3140
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$2 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x634922 -> 0x6364a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
