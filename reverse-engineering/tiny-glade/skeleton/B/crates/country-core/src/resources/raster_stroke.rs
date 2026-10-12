// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/raster_stroke.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a370f620b5aed7f9f18c58812a0248e6ab40c38972a94bf052c3f00c09d04704
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: raster_stroke.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 181, 482, 513, 536, 559, 563, 568, 579, 1626, 1654, 1673, 1682, 3641, 4422
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3725
// 首条位置: {"module":181,"inlinee":14117}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":181,"record_offset":60744,"parent_record_offset":60708,"ancestor_record_offset":60708,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":60780,"parent_record_offset":60744,"ancestor_record_offset":60708,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":60804,"parent_record_offset":60780,"ancestor_record_offset":60708,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::impl$0::fmt<alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3>,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":536,"record_offset":6532,"rva":10427536,"va":5379136656}
// 编译实例 RVAs: 0x9f1c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3755773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136656,"raw_hex":"e8fd4e3900","size":5,"soft_fail":false,"target_va":5382892464,"va":5379136686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::builders::DebugStruct::field
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3741269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136656,"raw_hex":"e855163900","size":5,"soft_fail":false,"target_va":5382878000,"va":5379136726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::builders::DebugStruct::finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3741869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136656,"raw_hex":"e8ad183900","size":5,"soft_fail":false,"target_va":5382878608,"va":5379136734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::radius
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7860
// 首条位置: {"module":563,"inlinee":30816}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::add_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x687A
// 首条位置: {"module":482,"inlinee":26746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":482,"record_offset":19152,"parent_record_offset":19128,"ancestor_record_offset":19128,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":482,"record_offset":19220,"parent_record_offset":19188,"ancestor_record_offset":19128,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":482,"record_offset":19268,"parent_record_offset":19244,"ancestor_record_offset":19128,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":482,"record_offset":19244,"parent_record_offset":19220,"ancestor_record_offset":19128,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":482,"record_offset":19188,"parent_record_offset":19152,"ancestor_record_offset":19128,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":19308,"parent_record_offset":19152,"ancestor_record_offset":19128,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::iter_points_xz
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=54
// 源码行: 59, 60, 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7832
// 首条位置: {"module":536,"inlinee":30770}
// 编译实例 RVAs: 0xa97860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":9548,"parent_record_offset":9440,"ancestor_record_offset":9440,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":9440,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":9512,"parent_record_offset":9488,"ancestor_record_offset":9440,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":9440,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::impl$2::iter_points_xz::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F08
// 首条位置: {"module":568,"inlinee":32520}
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
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x687B
// 首条位置: {"module":482,"inlinee":26747}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::RasterStroke::length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D36
// 首条位置: {"module":1682,"inlinee":32054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1682,"record_offset":159248,"parent_record_offset":159192,"ancestor_record_offset":159192,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":159336,"parent_record_offset":159192,"ancestor_record_offset":159192,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1682,"record_offset":159304,"parent_record_offset":159276,"ancestor_record_offset":159192,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1682,"record_offset":159276,"parent_record_offset":159248,"ancestor_record_offset":159192,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":159444,"parent_record_offset":159380,"ancestor_record_offset":159192,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":159516,"parent_record_offset":159380,"ancestor_record_offset":159192,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":159380,"parent_record_offset":159336,"ancestor_record_offset":159192,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1682,"record_offset":159580,"parent_record_offset":159516,"ancestor_record_offset":159192,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::impl$3::iter_points_xz::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F06
// 首条位置: {"module":568,"inlinee":32518}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::raster_stroke::impl$4::from
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7312
// 首条位置: {"module":513,"inlinee":29458}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":513,"record_offset":85712,"parent_record_offset":85676,"ancestor_record_offset":85676,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":85960,"parent_record_offset":85936,"ancestor_record_offset":85676,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":85988,"parent_record_offset":85960,"ancestor_record_offset":85676,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":86512,"parent_record_offset":86488,"ancestor_record_offset":85676,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":85936,"parent_record_offset":85912,"ancestor_record_offset":85676,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":86488,"parent_record_offset":86464,"ancestor_record_offset":85676,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":86404,"parent_record_offset":86376,"ancestor_record_offset":85676,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":86376,"parent_record_offset":86352,"ancestor_record_offset":85676,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":86352,"parent_record_offset":86328,"ancestor_record_offset":85676,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":85872,"parent_record_offset":85784,"ancestor_record_offset":85676,"inlinee":7968,"source_line":1945,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":85784,"parent_record_offset":85744,"ancestor_record_offset":85676,"inlinee":29461,"source_line":2113,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":86124,"parent_record_offset":85872,"ancestor_record_offset":85676,"inlinee":7971,"source_line":1977,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$22::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":86092,"parent_record_offset":86060,"ancestor_record_offset":85676,"inlinee":7975,"source_line":1952,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$25::allocate_for_slice_in::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":85912,"parent_record_offset":85872,"ancestor_record_offset":85676,"inlinee":29463,"source_line":2117,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$69::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":85744,"parent_record_offset":85712,"ancestor_record_offset":85676,"inlinee":29460,"source_line":3746,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::impl$2::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\mod.rs
//     evidence={"module":513,"record_offset":86464,"parent_record_offset":86376,"ancestor_record_offset":85676,"inlinee":9414,"source_line":384,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":85816,"parent_record_offset":85784,"ancestor_record_offset":85676,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":85840,"parent_record_offset":85816,"ancestor_record_offset":85676,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":86276,"parent_record_offset":85744,"ancestor_record_offset":85676,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":86432,"parent_record_offset":86404,"ancestor_record_offset":85676,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":86304,"parent_record_offset":85744,"ancestor_record_offset":85676,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":513,"record_offset":86152,"parent_record_offset":86124,"ancestor_record_offset":85676,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":86016,"parent_record_offset":85988,"ancestor_record_offset":85676,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":86176,"parent_record_offset":86152,"ancestor_record_offset":85676,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":513,"record_offset":86060,"parent_record_offset":85872,"ancestor_record_offset":85676,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00513.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::raster_stroke::RasterStroke<alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3>,alloc::alloc::Global> >", "constructor": "country_core::resources::raster_stroke::RasterStroke", "arguments": ["alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::raster_stroke::RasterStroke<alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::raster_stroke::RasterStroke<alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::add_point
// 信息 ID: 59a0c5fa33f4cc6a0ad4d1a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::active_raster_stroke::ActiveRasterStroke::start_new; module=482, inline_record=19128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::iter_points_xz
// 信息 ID: 1b66c32d1862be5ffe0457de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 50 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x2260bf4 -> 0xa97860 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x2260fb2 -> 0xa97860 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=9440; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=9580; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=9776; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=17652; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=18588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=25180; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=25320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=25488; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::length
// 信息 ID: e621058081d5fda465957b4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::forward; module=1682, inline_record=159192; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::backward; module=1682, inline_record=160040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::radius
// 信息 ID: 5c0e4ac5a3d5c0c37e93f13d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8>; module=563, inline_record=64660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8>; module=563, inline_record=65656; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >; module=563, inline_record=70592; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >; module=563, inline_record=71588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> >; module=563, inline_record=75756; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> >; module=563, inline_record=76632; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14>; module=563, inline_record=80660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14>; module=563, inline_record=81536; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::RasterStroke::smooth
// 信息 ID: b7e1d7dfae37db592df9d4bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::active_raster_stroke::ActiveRasterStroke::resample_and_smooth; module=482, inline_record=20440; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8>; module=563, inline_record=62872; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >; module=563, inline_record=69232; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> >; module=563, inline_record=74396; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14>; module=563, inline_record=78860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>; module=563, inline_record=84152; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster> >; module=563, inline_record=90228; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >; module=563, inline_record=95440; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::impl$0::fmt<alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3>,alloc::alloc::Global> >
// 信息 ID: 71fd1106e297de1ed411432e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["slice2$<glam::f32::vec3::Vec3>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::resources::raster_stroke::impl$0::fmt -> ["alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3>,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::impl$2::iter_points_xz::closure$0
// 信息 ID: bd9bceae5e838944afcebfbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::raster_stroke::impl$2::iter_points_xz", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0 <- country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 78 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=568, inline_record=49740; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=568, inline_record=50028; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=4422, inline_record=920; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=4422, inline_record=1208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::impl$3::iter_points_xz::closure$0
// 信息 ID: 066407126844374ca7270bda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::raster_stroke::impl$3::iter_points_xz", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0 <- country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 35 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; module=568, inline_record=37856; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; module=568, inline_record=38144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::impl$4::from
// 信息 ID: 9d9e3a3394bab20e13240610
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::world_raster::record_raster_history; module=513, inline_record=85676; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::record_path_history::record_path_history; module=579, inline_record=61000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::record_path_history::record_path_history; module=579, inline_record=63492; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::record_path_history::record_path_history; module=579, inline_record=65988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::record_path_history::record_path_history; module=579, inline_record=68480; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::garden::record_garden_history::record_garden_history; module=1654, inline_record=38388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::raster_stroke::impl$5::clone
// 信息 ID: ca05802f03266d7da9ba81c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=181, inline_record=60708; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1673, inline_record=19284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
