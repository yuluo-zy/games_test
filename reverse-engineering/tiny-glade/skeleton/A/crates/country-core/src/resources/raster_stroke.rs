// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/raster_stroke.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 36eb5d474bb31cbe972dc037e13a946e4ec68929f19e7f50c36f47a202619de7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: raster_stroke.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 167, 421, 432, 453, 485, 499, 1321, 1339, 1429, 4205
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EC8
// 首条位置: {"module":167,"inlinee":11976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":167,"record_offset":6672,"parent_record_offset":6636,"ancestor_record_offset":6636,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6636,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":6636,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6940,"parent_record_offset":6848,"ancestor_record_offset":6636,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":6848,"parent_record_offset":6820,"ancestor_record_offset":6636,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":6820,"parent_record_offset":6792,"ancestor_record_offset":6636,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":7060,"parent_record_offset":6820,"ancestor_record_offset":6636,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":6636,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":6636,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":6636,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":167,"record_offset":6792,"parent_record_offset":6756,"ancestor_record_offset":6636,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":167,"record_offset":6884,"parent_record_offset":6848,"ancestor_record_offset":6636,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":167,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":6636,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":167,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6636,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":167,"record_offset":7100,"parent_record_offset":6756,"ancestor_record_offset":6636,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7016,"parent_record_offset":6988,"ancestor_record_offset":6636,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":10456,"rva":6408848,"va":5375117968}
// 编译实例 RVAs: 0x61ca90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2465949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375117968,"raw_hex":"e89da02500","size":5,"soft_fail":false,"target_va":5377583952,"va":5375117998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::builders::DebugStruct::field
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2452373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375117968,"raw_hex":"e8956b2500","size":5,"soft_fail":false,"target_va":5377570416,"va":5375118038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::builders::DebugStruct::finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2453021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375117968,"raw_hex":"e81d6e2500","size":5,"soft_fail":false,"target_va":5377571072,"va":5375118046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::add_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6583
// 首条位置: {"module":453,"inlinee":25987}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":4348,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":4436,"parent_record_offset":4404,"ancestor_record_offset":4348,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":4404,"parent_record_offset":4372,"ancestor_record_offset":4348,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":4468,"parent_record_offset":4372,"ancestor_record_offset":4348,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65AA
// 首条位置: {"module":485,"inlinee":26026}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":552,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":485,"record_offset":456,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":744,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":416,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":485,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":416,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":808,"parent_record_offset":768,"ancestor_record_offset":416,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":416,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":416,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":836,"parent_record_offset":768,"ancestor_record_offset":416,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":416,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":485,"record_offset":872,"parent_record_offset":836,"ancestor_record_offset":416,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::radius
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=21
// 源码行: 46, 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65AB
// 首条位置: {"module":453,"record_offset":10584,"rva":6408944,"va":5375118064}
// 编译实例 RVAs: 0x61caf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::iter_points_xz
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=21
// 源码行: 50, 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65AC
// 首条位置: {"module":453,"record_offset":10720,"rva":6408960,"va":5375118080}
// 编译实例 RVAs: 0x61cb00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":10872,"parent_record_offset":10720,"ancestor_record_offset":10720,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":10916,"parent_record_offset":10892,"ancestor_record_offset":10720,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":453,"record_offset":10892,"parent_record_offset":10872,"ancestor_record_offset":10720,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::impl$1::iter_points_xz::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F46
// 首条位置: {"module":421,"inlinee":24390}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::smooth
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6584
// 首条位置: {"module":453,"inlinee":25988}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: raster_stroke.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::raster_stroke::RasterStroke
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::raster_stroke::RasterStroke>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::raster_stroke::RasterStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: raster_stroke.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: raster_stroke.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(ref_mut$<bevy_ecs::world::World>)
// 候选参数类型: ["ref_mut$<bevy_ecs::world::World>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>", "bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>,bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>", "bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::add_point
// 信息 ID: 8f8da779a44cdaf50022fa5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::active_raster_stroke::ActiveRasterStroke::start_new; module=453, inline_record=4348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::iter_points_xz
// 信息 ID: bfd19102b2298246285c19ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 19 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e28f -> 0x61cb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e65b -> 0x61cb00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10140; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10696; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=17312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=18524; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=26352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=26492; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=27568; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::length
// 信息 ID: 3a56c7d934f60b337ded6d5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::forward; module=485, inline_record=416; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::backward; module=485, inline_record=1436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::radius
// 信息 ID: 72542a8c21c456a0429d3497
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 19 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdb2fe -> 0x61caf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e29d -> 0x61caf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e669 -> 0x61caf0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10252; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10808; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=17200; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=17424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=26464; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=26604; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=27680; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::smooth
// 信息 ID: e1561d17a64ee247ad5f2c51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::active_raster_stroke::ActiveRasterStroke::resample_and_smooth; module=453, inline_record=5504; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>; module=1339, inline_record=57716; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2>; module=1339, inline_record=62936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::impl$0::fmt
// 信息 ID: a6ad6c8b4b701a945e87e4e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::impl$1::iter_points_xz::closure$0
// 信息 ID: 613dfaa6a309ed42dbe735a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::raster_stroke::impl$1::iter_points_xz", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0 <- country_core::resources::world_raster::impl$1::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 108 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; module=421, inline_record=12792; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; module=421, inline_record=13080; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; module=4205, inline_record=840; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; module=4205, inline_record=1128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::impl$2::clone
// 信息 ID: 7612ca2d60f29018bc0f68e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=167, inline_record=6636; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=432, inline_record=98668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
