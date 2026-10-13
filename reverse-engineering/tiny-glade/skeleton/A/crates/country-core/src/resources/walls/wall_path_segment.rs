// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/walls/wall_path_segment.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d40ae4b030fb5d2eac50a767cbe25adf95aff8ae8e4e46900913af3e95ca6569
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_path_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 361, 453
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5376
// 首条位置: {"module":361,"inlinee":21366}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":361,"record_offset":20852,"parent_record_offset":20772,"ancestor_record_offset":20772,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":361,"record_offset":22564,"parent_record_offset":20772,"ancestor_record_offset":20772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":361,"record_offset":21228,"parent_record_offset":21204,"ancestor_record_offset":20772,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":361,"record_offset":21252,"parent_record_offset":21228,"ancestor_record_offset":20772,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":361,"record_offset":22744,"parent_record_offset":22716,"ancestor_record_offset":20772,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":361,"record_offset":21204,"parent_record_offset":21096,"ancestor_record_offset":20772,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":361,"record_offset":22716,"parent_record_offset":22612,"ancestor_record_offset":20772,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":361,"record_offset":22644,"parent_record_offset":22612,"ancestor_record_offset":20772,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":361,"record_offset":21096,"parent_record_offset":21056,"ancestor_record_offset":20772,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":361,"record_offset":21056,"parent_record_offset":21016,"ancestor_record_offset":20772,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":361,"record_offset":21324,"parent_record_offset":21056,"ancestor_record_offset":20772,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":361,"record_offset":22612,"parent_record_offset":22588,"ancestor_record_offset":20772,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":361,"record_offset":20972,"parent_record_offset":20932,"ancestor_record_offset":20772,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":361,"record_offset":20932,"parent_record_offset":20892,"ancestor_record_offset":20772,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":361,"record_offset":20892,"parent_record_offset":20852,"ancestor_record_offset":20772,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":361,"record_offset":21016,"parent_record_offset":20972,"ancestor_record_offset":20772,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":361,"record_offset":21140,"parent_record_offset":21096,"ancestor_record_offset":20772,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":361,"record_offset":21168,"parent_record_offset":21140,"ancestor_record_offset":20772,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":361,"record_offset":21392,"parent_record_offset":21368,"ancestor_record_offset":20772,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":361,"record_offset":22676,"parent_record_offset":22644,"ancestor_record_offset":20772,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":361,"record_offset":21368,"parent_record_offset":20972,"ancestor_record_offset":20772,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":361,"record_offset":22588,"parent_record_offset":22564,"ancestor_record_offset":20772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":361,"record_offset":21280,"parent_record_offset":21252,"ancestor_record_offset":20772,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":10956,"rva":6408992,"va":5375118112}
// 编译实例 RVAs: 0x61cb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::loop_range::LoopRange::len
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t6910183","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375118112,"raw_hex":"e9e7706900","size":5,"soft_fail":false,"target_va":5382028304,"va":5375118116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":11108,"rva":6409008,"va":5375118128}
// 编译实例 RVAs: 0x61cb30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceda30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7147173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118128,"raw_hex":"e8a50e6d00","size":5,"soft_fail":false,"target_va":5382265392,"va":5375118214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":11284,"parent_record_offset":11108,"ancestor_record_offset":11108,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":11788,"parent_record_offset":11108,"ancestor_record_offset":11108,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":453,"record_offset":11364,"parent_record_offset":11108,"ancestor_record_offset":11108,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":453,"record_offset":11608,"parent_record_offset":11108,"ancestor_record_offset":11108,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":11108,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":11544,"parent_record_offset":11388,"ancestor_record_offset":11108,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":11860,"parent_record_offset":11816,"ancestor_record_offset":11108,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":11328,"parent_record_offset":11304,"ancestor_record_offset":11108,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":11508,"parent_record_offset":11484,"ancestor_record_offset":11108,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":11456,"parent_record_offset":11420,"ancestor_record_offset":11108,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":453,"record_offset":11304,"parent_record_offset":11284,"ancestor_record_offset":11108,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":11484,"parent_record_offset":11420,"ancestor_record_offset":11108,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":11420,"parent_record_offset":11388,"ancestor_record_offset":11108,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":11920,"parent_record_offset":11816,"ancestor_record_offset":11108,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":11816,"parent_record_offset":11788,"ancestor_record_offset":11108,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":453,"record_offset":11388,"parent_record_offset":11364,"ancestor_record_offset":11108,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":453,"record_offset":11956,"parent_record_offset":11920,"ancestor_record_offset":11108,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":12000,"rva":6409232,"va":5375118352}
// 编译实例 RVAs: 0x61cc10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceda30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7146949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118352,"raw_hex":"e8c50d6d00","size":5,"soft_fail":false,"target_va":5382265392,"va":5375118438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":12176,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":12680,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":453,"record_offset":12256,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":453,"record_offset":12500,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":12460,"parent_record_offset":12436,"ancestor_record_offset":12000,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":12436,"parent_record_offset":12280,"ancestor_record_offset":12000,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":12740,"parent_record_offset":12708,"ancestor_record_offset":12000,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":12000,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":12000,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":12348,"parent_record_offset":12312,"ancestor_record_offset":12000,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":453,"record_offset":12196,"parent_record_offset":12176,"ancestor_record_offset":12000,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":12376,"parent_record_offset":12312,"ancestor_record_offset":12000,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":12312,"parent_record_offset":12280,"ancestor_record_offset":12000,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":12772,"parent_record_offset":12708,"ancestor_record_offset":12000,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":12708,"parent_record_offset":12680,"ancestor_record_offset":12000,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":453,"record_offset":12280,"parent_record_offset":12256,"ancestor_record_offset":12000,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":453,"record_offset":12804,"parent_record_offset":12772,"ancestor_record_offset":12000,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00453.jsonl.gz"}
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
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_path_segment::WallPathSegment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::wall_path_segment::WallPathSegment>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_path_segment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
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
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
// 信息 ID: 9651f005bdf5b6d9c466f260
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8d89 -> 0x61cb30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4efd05 -> 0x61cb30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4efd73 -> 0x61cb30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
// 信息 ID: c6da5c665111e2e5704afed7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4efd7f -> 0x61cc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1e4d -> 0x61cc10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count
// 信息 ID: c7283f5854a441cb5a4cc964
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d7d0a -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8d4e -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8dd7 -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8e13 -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8e3e -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8e72 -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8ea6 -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4deaa3 -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4deadf -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4deb0f -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4deb43 -> 0x61cb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4deb78 -> 0x61cb20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::impl$2::clone
// 信息 ID: 3267d1d2d56231e2948ad13c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=20772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
