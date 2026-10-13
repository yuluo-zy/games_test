// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b651f1f10fc8f80000183fd76376f226c10e76bd17f7d349c4a9b892a30b2bde
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 81, 94, 97, 148, 157, 196, 228, 249, 399, 403, 405, 416, 417, 418, 420, 423, 431, 434, 436, 447, 449, 451, 459, 461, 468, 470, 478, 484, 491, 507, 535, 549, 550, 551, 557, 561, 567, 569, 570, 571, 575, 576, 577, 579, 580, 583, 584, 585, 589, 754, 756, 763, 764, 1237, 1455, 1459, 1460, 1472, 1473, 1478, 1482, 1487, 1490, 1491, 1492, 1493, 1499, 1504, 1505, 1525, 1526, 1528, 1529, 1531, 1535, 1536, 1541, 1545, 1559, 1563, 1579, 1593, 1596, 1600, 1608, 1631, 1643, 1682, 1687, 1699, 1701, 1705, 1706, 1713, 1762, 1879, 2178, 2179, 2181, 2186, 2192, 2416, 2746, 2959, 2960, 2961, 3144, 3152, 3161, 3164, 3167, 3173, 3230, 3631, 3637, 3752, 3777, 3785, 3786, 3935, 3979, 4030, 4031, 4054, 4061, 4066, 4078, 4084, 4155, 4165, 4172, 4377, 4660, 4746, 4777, 4841, 5004, 5040
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use half; // namespace candidate
//   package identities: [{"name":"half","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.4.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: time::formatting::formattable::impl$8::format_into<alloc::vec::Vec<u8,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3637,"record_offset":260,"rva":41250720,"va":5409959840}
// 编译实例 RVAs: 0x2756fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142756fa0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409959840,"raw_hex":"e858ffffff","size":5,"soft_fail":false,"target_va":5409959840,"va":5409960003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6392746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409959840,"raw_hex":"e8aa8b6100","size":5,"soft_fail":false,"target_va":5416353040,"va":5409960289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7097101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409959840,"raw_hex":"e80d4b6c00","size":5,"soft_fail":false,"target_va":5417057056,"va":5409959950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::formatting::format_component
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3635371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409959840,"raw_hex":"e8ab783700","size":5,"soft_fail":false,"target_va":5413595536,"va":5409960160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3637,"record_offset":868,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3637,"record_offset":1148,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_direct] time::formatting::formattable::impl$9::format_into
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/formatting/formattable.rs
//     evidence={"module":3637,"record_offset":936,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17184,"source_line":88,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_direct] time::formatting::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/formatting/mod.rs
//     evidence={"module":3637,"record_offset":452,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9166,"source_line":44,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3637,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":260,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3637,"record_offset":732,"parent_record_offset":704,"ancestor_record_offset":260,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3637,"record_offset":704,"parent_record_offset":668,"ancestor_record_offset":260,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3637,"record_offset":668,"parent_record_offset":636,"ancestor_record_offset":260,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3637,"record_offset":564,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3637,"record_offset":508,"parent_record_offset":480,"ancestor_record_offset":260,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3637,"record_offset":820,"parent_record_offset":564,"ancestor_record_offset":260,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3637,"record_offset":608,"parent_record_offset":564,"ancestor_record_offset":260,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3637,"record_offset":536,"parent_record_offset":508,"ancestor_record_offset":260,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3637,"record_offset":784,"parent_record_offset":564,"ancestor_record_offset":260,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3637,"record_offset":1048,"parent_record_offset":980,"ancestor_record_offset":260,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3637,"record_offset":1016,"parent_record_offset":980,"ancestor_record_offset":260,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3637,"record_offset":1080,"parent_record_offset":936,"ancestor_record_offset":260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3637,"record_offset":980,"parent_record_offset":936,"ancestor_record_offset":260,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":3637,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":260,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/03637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3173,"record_offset":260,"rva":38712512,"va":5407421632}
// 编译实例 RVAs: 0x24eb4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::determinant
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8965058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407421632,"raw_hex":"e83e3477ff","size":5,"soft_fail":false,"target_va":5398457248,"va":5407422301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::triangle_triangle::cross_intersect::cross_intersect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5471057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407421632,"raw_hex":"e8517b5300","size":5,"soft_fail":false,"target_va":5412893648,"va":5407422586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3173,"record_offset":1084,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::is_tri_degenerated
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3173,"record_offset":428,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":117855,"source_line":7,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3173,"record_offset":808,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":117856,"source_line":20,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3173,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":260,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":3173,"record_offset":840,"parent_record_offset":808,"ancestor_record_offset":260,"inlinee":117857,"source_line":128,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":3173,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":117858,"source_line":85,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3173,"record_offset":524,"parent_record_offset":428,"ancestor_record_offset":260,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3173,"record_offset":468,"parent_record_offset":428,"ancestor_record_offset":260,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs
//     evidence={"module":3173,"record_offset":1112,"parent_record_offset":1084,"ancestor_record_offset":260,"inlinee":117860,"source_line":39,"archive":"_pdb/C/modules/03173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::length
// 记录形式: inline_instance, inlinee_declaration; instances=55
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3980
// 首条位置: {"module":157,"inlinee":14720}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":157,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":24132,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":157,"record_offset":24236,"parent_record_offset":24212,"ancestor_record_offset":24132,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":157,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":24132,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":157,"record_offset":24212,"parent_record_offset":24180,"ancestor_record_offset":24132,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::length_squared
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F97
// 首条位置: {"module":4054,"inlinee":135063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27892,"parent_record_offset":27868,"ancestor_record_offset":27868,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27916,"parent_record_offset":27892,"ancestor_record_offset":27868,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::normalize
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BC4
// 首条位置: {"module":468,"inlinee":27588}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":1228,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":468,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":1228,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":551,"record_offset":76116,"parent_record_offset":75960,"ancestor_record_offset":75912,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1332,"parent_record_offset":1300,"ancestor_record_offset":1228,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":1228,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":1228,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":468,"record_offset":1364,"parent_record_offset":1332,"ancestor_record_offset":1228,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=123
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x397F
// 首条位置: {"module":157,"inlinee":14719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=80
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":157,"record_offset":29932,"parent_record_offset":29904,"ancestor_record_offset":29904,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":157,"record_offset":30068,"parent_record_offset":30044,"ancestor_record_offset":29904,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=188
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":157,"record_offset":29984,"parent_record_offset":29960,"ancestor_record_offset":29904,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=80
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":157,"record_offset":29960,"parent_record_offset":29932,"ancestor_record_offset":29904,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=72
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":157,"record_offset":30044,"parent_record_offset":29932,"ancestor_record_offset":29904,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::dot
// 记录形式: inline_instance, inlinee_declaration; instances=60
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x85B1
// 首条位置: {"module":550,"inlinee":34225}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":550,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":12980,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::length
// 记录形式: inline_instance, inlinee_declaration; instances=53
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ABB
// 首条位置: {"module":81,"inlinee":10939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":157,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":5048,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":157,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":5048,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":157,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":5048,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":157,"record_offset":5128,"parent_record_offset":5096,"ancestor_record_offset":5048,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::length_squared
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC50
// 首条位置: {"module":3167,"inlinee":117840}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":5148,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":5148,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::normalize
// 记录形式: inline_instance, inlinee_declaration; instances=36
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6732
// 首条位置: {"module":416,"inlinee":26418}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":416,"record_offset":13924,"parent_record_offset":13896,"ancestor_record_offset":13896,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":416,"record_offset":14076,"parent_record_offset":14052,"ancestor_record_offset":13896,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":20456,"parent_record_offset":20296,"ancestor_record_offset":20248,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":416,"record_offset":14016,"parent_record_offset":13980,"ancestor_record_offset":13896,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":416,"record_offset":13980,"parent_record_offset":13952,"ancestor_record_offset":13896,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":416,"record_offset":13952,"parent_record_offset":13924,"ancestor_record_offset":13896,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":416,"record_offset":14052,"parent_record_offset":14016,"ancestor_record_offset":13896,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::to_2d
// 记录形式: inlinee_declaration; instances=5
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13964
// 首条位置: {"module":1541,"inlinee":80228}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=105
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6659
// 首条位置: {"module":399,"inlinee":26201}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=60
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6916,"parent_record_offset":6888,"ancestor_record_offset":6888,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=81
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":399,"record_offset":7136,"parent_record_offset":7104,"ancestor_record_offset":6888,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=197
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":399,"record_offset":6980,"parent_record_offset":6944,"ancestor_record_offset":6888,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=59
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":6888,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=43
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":7104,"parent_record_offset":6916,"ancestor_record_offset":6888,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::dot
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x665A
// 首条位置: {"module":399,"inlinee":26202}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6764,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$2::length
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E4A
// 首条位置: {"module":196,"inlinee":15946}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":53904,"parent_record_offset":53876,"ancestor_record_offset":53876,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$2::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6854
// 首条位置: {"module":431,"inlinee":26708}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::t_of_point_projection_onto_segment
// 记录形式: inline_instance, inlinee_declaration; instances=57
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6658
// 首条位置: {"module":399,"inlinee":26200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":399,"record_offset":6856,"parent_record_offset":6512,"ancestor_record_offset":6512,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":550,"record_offset":12872,"parent_record_offset":12840,"ancestor_record_offset":12840,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6552,"parent_record_offset":6512,"ancestor_record_offset":6512,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=42
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":550,"record_offset":12980,"parent_record_offset":12840,"ancestor_record_offset":12840,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":399,"record_offset":6736,"parent_record_offset":6512,"ancestor_record_offset":6512,"inlinee":26202,"source_line":114,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=68
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":399,"record_offset":6584,"parent_record_offset":6552,"ancestor_record_offset":6512,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":550,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":12840,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6764,"parent_record_offset":6736,"ancestor_record_offset":6512,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_line_point
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89C6
// 首条位置: {"module":561,"inlinee":35270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81880,"parent_record_offset":81480,"ancestor_record_offset":81480,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":4036,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":561,"record_offset":81936,"parent_record_offset":81480,"ancestor_record_offset":81480,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":4144,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":10939,"source_line":89,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":561,"record_offset":81508,"parent_record_offset":81480,"ancestor_record_offset":81480,"inlinee":26846,"source_line":179,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":82040,"parent_record_offset":82016,"ancestor_record_offset":81480,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":81836,"parent_record_offset":81812,"ancestor_record_offset":81480,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":81752,"parent_record_offset":81728,"ancestor_record_offset":81480,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":81588,"parent_record_offset":81564,"ancestor_record_offset":81480,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81644,"parent_record_offset":81620,"ancestor_record_offset":81480,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81960,"parent_record_offset":81936,"ancestor_record_offset":81480,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81704,"parent_record_offset":81680,"ancestor_record_offset":81480,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81728,"parent_record_offset":81704,"ancestor_record_offset":81480,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81812,"parent_record_offset":81704,"ancestor_record_offset":81480,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81564,"parent_record_offset":81536,"ancestor_record_offset":81480,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":3476,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":4172,"parent_record_offset":4144,"ancestor_record_offset":3476,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3760,"parent_record_offset":3732,"ancestor_record_offset":3476,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":3476,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3932,"parent_record_offset":3760,"ancestor_record_offset":3476,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3167,"record_offset":3584,"parent_record_offset":3552,"ancestor_record_offset":3476,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":82016,"parent_record_offset":81984,"ancestor_record_offset":81480,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":561,"record_offset":81620,"parent_record_offset":81536,"ancestor_record_offset":81480,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":561,"record_offset":81680,"parent_record_offset":81508,"ancestor_record_offset":81480,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3672,"parent_record_offset":3552,"ancestor_record_offset":3476,"inlinee":26202,"source_line":114,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3167,"record_offset":3732,"parent_record_offset":3516,"ancestor_record_offset":3476,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":561,"record_offset":81536,"parent_record_offset":81508,"ancestor_record_offset":81480,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_segment_point
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8D33
// 首条位置: {"module":570,"inlinee":36147}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34792,"parent_record_offset":33836,"ancestor_record_offset":33836,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":570,"record_offset":75140,"parent_record_offset":74312,"ancestor_record_offset":74312,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":34876,"parent_record_offset":33836,"ancestor_record_offset":33836,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":570,"record_offset":75240,"parent_record_offset":74312,"ancestor_record_offset":74312,"inlinee":10939,"source_line":89,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":570,"record_offset":74352,"parent_record_offset":74312,"ancestor_record_offset":74312,"inlinee":26199,"source_line":184,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":570,"record_offset":75104,"parent_record_offset":74352,"ancestor_record_offset":74312,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":570,"record_offset":74756,"parent_record_offset":74392,"ancestor_record_offset":74312,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":570,"record_offset":75356,"parent_record_offset":75332,"ancestor_record_offset":74312,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":570,"record_offset":75036,"parent_record_offset":75004,"ancestor_record_offset":74312,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":570,"record_offset":74884,"parent_record_offset":74852,"ancestor_record_offset":74312,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":570,"record_offset":74476,"parent_record_offset":74440,"ancestor_record_offset":74312,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34244,"parent_record_offset":34204,"ancestor_record_offset":33836,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34916,"parent_record_offset":34876,"ancestor_record_offset":33836,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34528,"parent_record_offset":34488,"ancestor_record_offset":33836,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34568,"parent_record_offset":34528,"ancestor_record_offset":33836,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34696,"parent_record_offset":34528,"ancestor_record_offset":33836,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34028,"parent_record_offset":33956,"ancestor_record_offset":33836,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":570,"record_offset":74660,"parent_record_offset":74628,"ancestor_record_offset":74312,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":570,"record_offset":75268,"parent_record_offset":75240,"ancestor_record_offset":74312,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":570,"record_offset":74820,"parent_record_offset":74788,"ancestor_record_offset":74312,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":570,"record_offset":74852,"parent_record_offset":74820,"ancestor_record_offset":74312,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":570,"record_offset":75004,"parent_record_offset":74820,"ancestor_record_offset":74312,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":570,"record_offset":74440,"parent_record_offset":74392,"ancestor_record_offset":74312,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":570,"record_offset":75332,"parent_record_offset":75296,"ancestor_record_offset":74312,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":34204,"parent_record_offset":33956,"ancestor_record_offset":33836,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":34488,"parent_record_offset":33896,"ancestor_record_offset":33836,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":570,"record_offset":74628,"parent_record_offset":74392,"ancestor_record_offset":74312,"inlinee":26202,"source_line":114,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":570,"record_offset":74788,"parent_record_offset":74352,"ancestor_record_offset":74312,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":570,"record_offset":74392,"parent_record_offset":74352,"ancestor_record_offset":74312,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_squared_segment_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F96
// 首条位置: {"module":4054,"inlinee":135062}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27256,"parent_record_offset":27220,"ancestor_record_offset":27220,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length_squared
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4054,"record_offset":27868,"parent_record_offset":27220,"ancestor_record_offset":27220,"inlinee":135063,"source_line":63,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4054,"record_offset":27320,"parent_record_offset":27220,"ancestor_record_offset":27220,"inlinee":26199,"source_line":184,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4054,"record_offset":27636,"parent_record_offset":27320,"ancestor_record_offset":27220,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4054,"record_offset":27604,"parent_record_offset":27348,"ancestor_record_offset":27220,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4054,"record_offset":27824,"parent_record_offset":27800,"ancestor_record_offset":27220,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4054,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":27220,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4054,"record_offset":27284,"parent_record_offset":27256,"ancestor_record_offset":27220,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27516,"parent_record_offset":27492,"ancestor_record_offset":27220,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27892,"parent_record_offset":27868,"ancestor_record_offset":27220,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27692,"parent_record_offset":27668,"ancestor_record_offset":27220,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27716,"parent_record_offset":27692,"ancestor_record_offset":27220,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27800,"parent_record_offset":27692,"ancestor_record_offset":27220,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4054,"record_offset":27380,"parent_record_offset":27348,"ancestor_record_offset":27220,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4054,"record_offset":27492,"parent_record_offset":27348,"ancestor_record_offset":27220,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4054,"record_offset":27668,"parent_record_offset":27320,"ancestor_record_offset":27220,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4054,"record_offset":27348,"parent_record_offset":27320,"ancestor_record_offset":27220,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/04054.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::project_point_onto_line
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68DE
// 首条位置: {"module":436,"inlinee":26846}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":561,"record_offset":81680,"parent_record_offset":81508,"ancestor_record_offset":81508,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":436,"record_offset":11952,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":436,"record_offset":11552,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":436,"record_offset":11920,"parent_record_offset":11552,"ancestor_record_offset":11516,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":436,"record_offset":12176,"parent_record_offset":12148,"ancestor_record_offset":11516,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":436,"record_offset":12032,"parent_record_offset":12000,"ancestor_record_offset":11516,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":436,"record_offset":11636,"parent_record_offset":11600,"ancestor_record_offset":11516,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81644,"parent_record_offset":81620,"ancestor_record_offset":81508,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81704,"parent_record_offset":81680,"ancestor_record_offset":81508,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81728,"parent_record_offset":81704,"ancestor_record_offset":81508,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81812,"parent_record_offset":81704,"ancestor_record_offset":81508,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":81564,"parent_record_offset":81536,"ancestor_record_offset":81508,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":436,"record_offset":11820,"parent_record_offset":11792,"ancestor_record_offset":11516,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":436,"record_offset":11976,"parent_record_offset":11952,"ancestor_record_offset":11516,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":436,"record_offset":12000,"parent_record_offset":11976,"ancestor_record_offset":11516,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":436,"record_offset":12148,"parent_record_offset":11976,"ancestor_record_offset":11516,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":436,"record_offset":11600,"parent_record_offset":11552,"ancestor_record_offset":11516,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":561,"record_offset":81620,"parent_record_offset":81536,"ancestor_record_offset":81508,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":436,"record_offset":11792,"parent_record_offset":11552,"ancestor_record_offset":11516,"inlinee":26202,"source_line":114,"archive":"_pdb/C/modules/00436.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::project_point_onto_segment
// 记录形式: inline_instance, inlinee_declaration; instances=25
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6657
// 首条位置: {"module":399,"inlinee":26199}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":399,"record_offset":7204,"parent_record_offset":6472,"ancestor_record_offset":6472,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":31340,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":399,"record_offset":6888,"parent_record_offset":6472,"ancestor_record_offset":6472,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":399,"record_offset":6512,"parent_record_offset":6472,"ancestor_record_offset":6472,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":399,"record_offset":6856,"parent_record_offset":6512,"ancestor_record_offset":6472,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":399,"record_offset":7136,"parent_record_offset":7104,"ancestor_record_offset":6472,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":399,"record_offset":6980,"parent_record_offset":6944,"ancestor_record_offset":6472,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":399,"record_offset":6584,"parent_record_offset":6552,"ancestor_record_offset":6472,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31244,"parent_record_offset":31220,"ancestor_record_offset":31108,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31364,"parent_record_offset":31340,"ancestor_record_offset":31108,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31388,"parent_record_offset":31364,"ancestor_record_offset":31108,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31472,"parent_record_offset":31364,"ancestor_record_offset":31108,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31164,"parent_record_offset":31136,"ancestor_record_offset":31108,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6764,"parent_record_offset":6736,"ancestor_record_offset":6472,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6916,"parent_record_offset":6888,"ancestor_record_offset":6472,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":6472,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":7104,"parent_record_offset":6916,"ancestor_record_offset":6472,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":399,"record_offset":6552,"parent_record_offset":6512,"ancestor_record_offset":6472,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":31220,"parent_record_offset":31136,"ancestor_record_offset":31108,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":399,"record_offset":6736,"parent_record_offset":6512,"ancestor_record_offset":6472,"inlinee":26202,"source_line":114,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::random_splits_into
// 记录形式: compiled_procedure; instances=1
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":7316,"rva":14567296,"va":5383276416}
// 编译实例 RVAs: 0xde4780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33076045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383276416,"raw_hex":"e84db3f801","size":5,"soft_fail":false,"target_va":5416353040,"va":5383276990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33105165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383276416,"raw_hex":"e80d25f901","size":5,"soft_fail":false,"target_va":5416381792,"va":5383276622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t106317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383276416,"raw_hex":"e84d9f0100","size":5,"soft_fail":false,"target_va":5383383152,"va":5383276830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":8312,"parent_record_offset":7316,"ancestor_record_offset":7316,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":7436,"parent_record_offset":7316,"ancestor_record_offset":7316,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":8284,"parent_record_offset":7316,"ancestor_record_offset":7316,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":7732,"parent_record_offset":7708,"ancestor_record_offset":7316,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":7316,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":7316,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":7628,"parent_record_offset":7592,"ancestor_record_offset":7316,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":7316,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":7756,"parent_record_offset":7732,"ancestor_record_offset":7316,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":7592,"parent_record_offset":7560,"ancestor_record_offset":7316,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":7708,"parent_record_offset":7492,"ancestor_record_offset":7316,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":7492,"parent_record_offset":7464,"ancestor_record_offset":7316,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":7316,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":7532,"parent_record_offset":7492,"ancestor_record_offset":7316,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":8008,"parent_record_offset":7988,"ancestor_record_offset":7316,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":754,"record_offset":8244,"parent_record_offset":7492,"ancestor_record_offset":7316,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":754,"record_offset":8176,"parent_record_offset":8156,"ancestor_record_offset":7316,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":754,"record_offset":7464,"parent_record_offset":7436,"ancestor_record_offset":7316,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":754,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":7316,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":754,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":7316,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":754,"record_offset":7892,"parent_record_offset":7868,"ancestor_record_offset":7316,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":754,"record_offset":7944,"parent_record_offset":7916,"ancestor_record_offset":7316,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":754,"record_offset":9016,"parent_record_offset":8988,"ancestor_record_offset":7316,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":754,"record_offset":8884,"parent_record_offset":8852,"ancestor_record_offset":7316,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":754,"record_offset":8632,"parent_record_offset":8584,"ancestor_record_offset":7316,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":754,"record_offset":9052,"parent_record_offset":8916,"ancestor_record_offset":7316,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":754,"record_offset":8988,"parent_record_offset":8960,"ancestor_record_offset":7316,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":754,"record_offset":8960,"parent_record_offset":8916,"ancestor_record_offset":7316,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":754,"record_offset":8656,"parent_record_offset":8632,"ancestor_record_offset":7316,"inlinee":10947,"source_line":861,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":754,"record_offset":8916,"parent_record_offset":8884,"ancestor_record_offset":7316,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":754,"record_offset":7820,"parent_record_offset":7492,"ancestor_record_offset":7316,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":754,"record_offset":7988,"parent_record_offset":7968,"ancestor_record_offset":7316,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":754,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":7316,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":754,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":7316,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":8076,"parent_record_offset":7916,"ancestor_record_offset":7316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":7316,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::random_splits_into::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":9080,"parent_record_offset":9052,"ancestor_record_offset":7316,"inlinee":48852,"source_line":211,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::random_splits_into::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBED4
// 首条位置: {"module":754,"inlinee":48852}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::random_splits
// 记录形式: compiled_procedure; instances=1
// 源码行: 225
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":2872,"rva":14564960,"va":5383274080}
// 编译实例 RVAs: 0xde3e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7595519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274080,"raw_hex":"e8011a8cff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383274202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7595654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274080,"raw_hex":"e87a198cff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383274353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14551996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274080,"raw_hex":"e844f421ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383274183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33346617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274080,"raw_hex":"e839d4fc01","size":5,"soft_fail":false,"target_va":5416620794,"va":5383274172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33347905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274080,"raw_hex":"e841d9fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383274298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274080,"raw_hex":"e87a080000","size":5,"soft_fail":false,"target_va":5383276416,"va":5383274241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":2980,"parent_record_offset":2872,"ancestor_record_offset":2872,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":3392,"parent_record_offset":2872,"ancestor_record_offset":2872,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":2872,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":3308,"parent_record_offset":3284,"ancestor_record_offset":2872,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2872,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":2872,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":2872,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":3628,"parent_record_offset":3604,"ancestor_record_offset":2872,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":3260,"parent_record_offset":3084,"ancestor_record_offset":2872,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":3556,"parent_record_offset":3464,"ancestor_record_offset":2872,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2872,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":3492,"parent_record_offset":3464,"ancestor_record_offset":2872,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":3464,"parent_record_offset":3440,"ancestor_record_offset":2872,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":3084,"parent_record_offset":3052,"ancestor_record_offset":2872,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2872,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":3440,"parent_record_offset":3416,"ancestor_record_offset":2872,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":754,"record_offset":3116,"parent_record_offset":3084,"ancestor_record_offset":2872,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2872,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":754,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2872,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":754,"record_offset":3220,"parent_record_offset":3164,"ancestor_record_offset":2872,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":754,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":2872,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":754,"record_offset":3192,"parent_record_offset":3164,"ancestor_record_offset":2872,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":754,"record_offset":3524,"parent_record_offset":3492,"ancestor_record_offset":2872,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":2872,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::smoothstep
// 记录形式: inline_instance, inlinee_declaration; instances=79
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D99
// 首条位置: {"module":94,"inlinee":11673}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":94,"record_offset":91584,"parent_record_offset":91556,"ancestor_record_offset":91556,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00094.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::inverse_lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 257, 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBECE
// 首条位置: {"module":754,"inlinee":48846}
// 编译实例 RVAs: 0xde3df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::linearstep
// 记录形式: compiled_procedure; instances=1
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":424,"rva":14564480,"va":5383273600}
// 编译实例 RVAs: 0xde3c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":556,"parent_record_offset":424,"ancestor_record_offset":424,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::inverse_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":532,"parent_record_offset":424,"ancestor_record_offset":424,"inlinee":48846,"source_line":257,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=13
// 源码行: 266, 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBED6
// 首条位置: {"module":754,"inlinee":48854}
// 编译实例 RVAs: 0xde5ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBED9
// 首条位置: {"module":754,"inlinee":48857}
// 编译实例 RVAs: 0xde4f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33779257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383278464,"raw_hex":"e8396e0302","size":5,"soft_fail":false,"target_va":5417057760,"va":5383278498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":11676,"parent_record_offset":11504,"ancestor_record_offset":11504,"inlinee":48854,"source_line":266,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::time_continuous_lerp_factor
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":11620,"parent_record_offset":11504,"ancestor_record_offset":11504,"inlinee":48853,"source_line":275,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":11504,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp_factor
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBED5
// 首条位置: {"module":754,"inlinee":48853}
// 编译实例 RVAs: 0xde58a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33776940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383280800,"raw_hex":"e82c650302","size":5,"soft_fail":false,"target_va":5417057760,"va":5383280815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":11620,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp_vec2
// 记录形式: compiled_procedure; instances=1
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":15072,"rva":14570880,"va":5383280000}
// 编译实例 RVAs: 0xde5580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33777690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383280000,"raw_hex":"e81a680302","size":5,"soft_fail":false,"target_va":5417057760,"va":5383280065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":15248,"parent_record_offset":15072,"ancestor_record_offset":15072,"inlinee":48854,"source_line":266,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::time_continuous_lerp_factor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":15192,"parent_record_offset":15072,"ancestor_record_offset":15072,"inlinee":48853,"source_line":275,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":15216,"parent_record_offset":15192,"ancestor_record_offset":15072,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp_vec3
// 记录形式: compiled_procedure; instances=1
// 源码行: 284
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":15308,"rva":14571024,"va":5383280144}
// 编译实例 RVAs: 0xde5610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33777579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383280144,"raw_hex":"e8ab670302","size":5,"soft_fail":false,"target_va":5417057760,"va":5383280176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":15600,"parent_record_offset":15308,"ancestor_record_offset":15308,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::time_continuous_lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":15428,"parent_record_offset":15308,"ancestor_record_offset":15308,"inlinee":48857,"source_line":270,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":15480,"parent_record_offset":15456,"ancestor_record_offset":15308,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":15512,"parent_record_offset":15428,"ancestor_record_offset":15308,"inlinee":48854,"source_line":266,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::time_continuous_lerp_factor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":15456,"parent_record_offset":15428,"ancestor_record_offset":15308,"inlinee":48853,"source_line":275,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":13140,"rva":14570208,"va":5383279328}
// 编译实例 RVAs: 0xde52e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33778388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383279328,"raw_hex":"e8d46a0302","size":5,"soft_fail":false,"target_va":5417057760,"va":5383279367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::time_continuous_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":13256,"parent_record_offset":13140,"ancestor_record_offset":13140,"inlinee":48857,"source_line":270,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":13308,"parent_record_offset":13284,"ancestor_record_offset":13140,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":13340,"parent_record_offset":13256,"ancestor_record_offset":13140,"inlinee":48854,"source_line":266,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::time_continuous_lerp_factor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":13284,"parent_record_offset":13256,"ancestor_record_offset":13140,"inlinee":48853,"source_line":275,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::perspective_lerp
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x91BC
// 首条位置: {"module":585,"inlinee":37308}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":585,"record_offset":6464,"parent_record_offset":6440,"ancestor_record_offset":6440,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::mouse_ss_to_cs
// 记录形式: compiled_procedure; instances=1
// 源码行: 304
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":3688,"rva":14565264,"va":5383274384}
// 编译实例 RVAs: 0xde3f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":3880,"parent_record_offset":3688,"ancestor_record_offset":3688,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":3804,"parent_record_offset":3688,"ancestor_record_offset":3688,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":3960,"parent_record_offset":3688,"ancestor_record_offset":3688,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":3908,"parent_record_offset":3880,"ancestor_record_offset":3688,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":3688,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":3688,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::uniform_sample_disk
// 记录形式: compiled_procedure; instances=1
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":9932,"rva":14568240,"va":5383277360}
// 编译实例 RVAs: 0xde4b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33780281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383277360,"raw_hex":"e839720302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383277410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33780600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383277360,"raw_hex":"e878730302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383277427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":10108,"parent_record_offset":9932,"ancestor_record_offset":9932,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":10136,"parent_record_offset":9932,"ancestor_record_offset":9932,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":10052,"parent_record_offset":9932,"ancestor_record_offset":9932,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":9932,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::uniform_sample_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":12336,"rva":14569696,"va":5383278816}
// 编译实例 RVAs: 0xde50e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33778838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383278816,"raw_hex":"e8966c0302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383278853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33779157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383278816,"raw_hex":"e8d56d0302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383278870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":12452,"parent_record_offset":12336,"ancestor_record_offset":12336,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":12480,"parent_record_offset":12336,"ancestor_record_offset":12336,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::uniform_sample_sphere
// 记录形式: compiled_procedure; instances=1
// 源码行: 320
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":12512,"rva":14569792,"va":5383278912}
// 编译实例 RVAs: 0xde5140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33778665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383278912,"raw_hex":"e8e96b0302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383279026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33779012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383278912,"raw_hex":"e8446d0302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383279015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":12640,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":12780,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":12752,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":12724,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":12668,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":12692,"parent_record_offset":12668,"ancestor_record_offset":12512,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::uniform_sample_cone
// 记录形式: compiled_procedure; instances=1
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":9632,"rva":14568048,"va":5383277168}
// 编译实例 RVAs: 0xde4a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33780415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383277168,"raw_hex":"e8bf720302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383277276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33780739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383277168,"raw_hex":"e803740302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383277288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":9816,"parent_record_offset":9632,"ancestor_record_offset":9632,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":9900,"parent_record_offset":9632,"ancestor_record_offset":9632,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":9844,"parent_record_offset":9632,"ancestor_record_offset":9632,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":9872,"parent_record_offset":9632,"ancestor_record_offset":9632,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":9760,"parent_record_offset":9632,"ancestor_record_offset":9632,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":9784,"parent_record_offset":9760,"ancestor_record_offset":9632,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::golden_spiral
// 记录形式: compiled_procedure; instances=1
// 源码行: 335
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":2624,"rva":14564864,"va":5383273984}
// 编译实例 RVAs: 0xde3e00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33783666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383273984,"raw_hex":"e8727f0302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383274025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33783990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383273984,"raw_hex":"e8b6800302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383274037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":2792,"parent_record_offset":2624,"ancestor_record_offset":2624,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":2736,"parent_record_offset":2624,"ancestor_record_offset":2624,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":2764,"parent_record_offset":2624,"ancestor_record_offset":2624,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":2624,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::radical_inverse
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 340
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEF4
// 首条位置: {"module":754,"record_offset":5384,"rva":14566432,"va":5383275552}
// 编译实例 RVAs: 0xde4420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33346855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383275552,"raw_hex":"e827d5fc01","size":5,"soft_fail":false,"target_va":5416622560,"va":5383275700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rotation_from_normal
// 记录形式: compiled_procedure; instances=1
// 源码行: 355
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":10168,"rva":14568352,"va":5383277472}
// 编译实例 RVAs: 0xde4ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33343759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383277472,"raw_hex":"e80fc9fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383278444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":10828,"parent_record_offset":10168,"ancestor_record_offset":10168,"inlinee":16667,"source_line":277,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":11304,"parent_record_offset":10168,"ancestor_record_offset":10168,"inlinee":27906,"source_line":596,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":10356,"parent_record_offset":10168,"ancestor_record_offset":10168,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":10296,"parent_record_offset":10168,"ancestor_record_offset":10168,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":10392,"parent_record_offset":10168,"ancestor_record_offset":10168,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_and_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":11400,"parent_record_offset":11376,"ancestor_record_offset":10168,"inlinee":27910,"source_line":253,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmplt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":11432,"parent_record_offset":11352,"ancestor_record_offset":10168,"inlinee":27911,"source_line":516,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":10324,"parent_record_offset":10296,"ancestor_record_offset":10168,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":10532,"parent_record_offset":10508,"ancestor_record_offset":10168,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/sse2/bvec4a.rs
//     evidence={"module":754,"record_offset":11464,"parent_record_offset":11328,"ancestor_record_offset":10168,"inlinee":27913,"source_line":91,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":754,"record_offset":10948,"parent_record_offset":10856,"ancestor_record_offset":10168,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":10856,"parent_record_offset":10828,"ancestor_record_offset":10168,"inlinee":16668,"source_line":208,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":754,"record_offset":11328,"parent_record_offset":11304,"ancestor_record_offset":10168,"inlinee":27907,"source_line":513,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::is_finite_mask
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":754,"record_offset":11352,"parent_record_offset":11328,"ancestor_record_offset":10168,"inlinee":27908,"source_line":522,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":10476,"parent_record_offset":10444,"ancestor_record_offset":10168,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":10168,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":10168,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::m128_abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":11376,"parent_record_offset":11352,"ancestor_record_offset":10168,"inlinee":27909,"source_line":123,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":10508,"parent_record_offset":10476,"ancestor_record_offset":10168,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::pack_2x16f_uint
// 记录形式: compiled_procedure; instances=1
// 源码行: 369
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":4384,"rva":14565808,"va":5383274928}
// 编译实例 RVAs: 0xde41b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140954d20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4781507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274928,"raw_hex":"e83d0ab7ff","size":5,"soft_fail":false,"target_va":5378493728,"va":5383275230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std_detect::detect::cache::detect_and_initialize
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t33355425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274928,"raw_hex":"e8a1f6fc01","size":5,"soft_fail":false,"target_va":5416630640,"va":5383275210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] half::binary16::f16::from_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16.rs
//     evidence={"module":754,"record_offset":4492,"parent_record_offset":4384,"ancestor_record_offset":4384,"inlinee":19898,"source_line":65,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":754,"record_offset":4672,"parent_record_offset":4636,"ancestor_record_offset":4384,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":754,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":4384,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":754,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":4384,"inlinee":19900,"source_line":64,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16_fallback
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":754,"record_offset":4828,"parent_record_offset":4516,"ancestor_record_offset":4384,"inlinee":19924,"source_line":482,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::arch::x86::__is_feature_detected::f16c
//     owner=unresolved_namespace; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\macros.rs
//     evidence={"module":754,"record_offset":4552,"parent_record_offset":4516,"ancestor_record_offset":4384,"inlinee":19902,"source_line":197,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::Cache::test
//     owner=unresolved_namespace; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":754,"record_offset":4636,"parent_record_offset":4608,"ancestor_record_offset":4384,"inlinee":19908,"source_line":90,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::Initializer::test
//     owner=unresolved_namespace; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":754,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":4384,"inlinee":19912,"source_line":40,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test
//     owner=unresolved_namespace; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":754,"record_offset":4608,"parent_record_offset":4580,"ancestor_record_offset":4384,"inlinee":19906,"source_line":194,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":754,"record_offset":4728,"parent_record_offset":4636,"ancestor_record_offset":4384,"inlinee":19910,"source_line":202,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":754,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":4384,"inlinee":19913,"source_line":16,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::check_for
//     owner=unresolved_namespace; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\mod.rs
//     evidence={"module":754,"record_offset":4580,"parent_record_offset":4552,"ancestor_record_offset":4384,"inlinee":19904,"source_line":89,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$4::assign_auto_resize<enum2$<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 399
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1643,"record_offset":278628,"rva":23042816,"va":5391751936}
// 编译实例 RVAs: 0x15f9b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24600849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391751936,"raw_hex":"e811617701","size":5,"soft_fail":false,"target_va":5416353040,"va":5391752186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24628208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391751936,"raw_hex":"e8f0cb7701","size":5,"soft_fail":false,"target_va":5416380224,"va":5391752011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24869242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391751936,"raw_hex":"e87a797b01","size":5,"soft_fail":false,"target_va":5416621762,"va":5391752515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":278984,"parent_record_offset":278628,"ancestor_record_offset":278628,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":279188,"parent_record_offset":278628,"ancestor_record_offset":278628,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":279348,"parent_record_offset":278628,"ancestor_record_offset":278628,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":278888,"parent_record_offset":278628,"ancestor_record_offset":278628,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":278860,"parent_record_offset":278628,"ancestor_record_offset":278628,"inlinee":14729,"source_line":2010,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":280144,"parent_record_offset":279960,"ancestor_record_offset":278628,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":279064,"parent_record_offset":279040,"ancestor_record_offset":278628,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":279216,"parent_record_offset":279188,"ancestor_record_offset":278628,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":279296,"parent_record_offset":279272,"ancestor_record_offset":278628,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":279272,"parent_record_offset":279244,"ancestor_record_offset":278628,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":279112,"parent_record_offset":279088,"ancestor_record_offset":278628,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":279088,"parent_record_offset":279064,"ancestor_record_offset":278628,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":279244,"parent_record_offset":279216,"ancestor_record_offset":278628,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":279040,"parent_record_offset":279008,"ancestor_record_offset":278628,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":279416,"parent_record_offset":279348,"ancestor_record_offset":278628,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":279388,"parent_record_offset":279348,"ancestor_record_offset":278628,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":279008,"parent_record_offset":278984,"ancestor_record_offset":278628,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":279484,"parent_record_offset":279416,"ancestor_record_offset":278628,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1643,"record_offset":279892,"parent_record_offset":279416,"ancestor_record_offset":278628,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":279760,"parent_record_offset":279724,"ancestor_record_offset":278628,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1643,"record_offset":279724,"parent_record_offset":279688,"ancestor_record_offset":278628,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1643,"record_offset":279688,"parent_record_offset":279416,"ancestor_record_offset":278628,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":279596,"parent_record_offset":279572,"ancestor_record_offset":278628,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1643,"record_offset":279640,"parent_record_offset":279416,"ancestor_record_offset":278628,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":279152,"parent_record_offset":279008,"ancestor_record_offset":278628,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":278916,"parent_record_offset":278888,"ancestor_record_offset":278628,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":278944,"parent_record_offset":278916,"ancestor_record_offset":278628,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$4::assign_auto_resize<tg_text::AtlasAllocUsage>
// 记录形式: compiled_procedure; instances=1
// 源码行: 399
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1879,"record_offset":608,"rva":30146352,"va":5398855472}
// 编译实例 RVAs: 0x1cbff30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17497310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398855472,"raw_hex":"e8defc0a01","size":5,"soft_fail":false,"target_va":5416353040,"va":5398855725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17524671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398855472,"raw_hex":"e8bf670b01","size":5,"soft_fail":false,"target_va":5416380224,"va":5398855548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17765612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398855472,"raw_hex":"e8ec140f01","size":5,"soft_fail":false,"target_va":5416621762,"va":5398856145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":892,"parent_record_offset":608,"ancestor_record_offset":608,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":1096,"parent_record_offset":608,"ancestor_record_offset":608,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":1256,"parent_record_offset":608,"ancestor_record_offset":608,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":796,"parent_record_offset":608,"ancestor_record_offset":608,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1879,"record_offset":768,"parent_record_offset":608,"ancestor_record_offset":608,"inlinee":14729,"source_line":2010,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1879,"record_offset":2052,"parent_record_offset":1868,"ancestor_record_offset":608,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1879,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":608,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1879,"record_offset":1124,"parent_record_offset":1096,"ancestor_record_offset":608,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1879,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":608,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1879,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":608,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1879,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":608,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1879,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":608,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1879,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":608,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":948,"parent_record_offset":916,"ancestor_record_offset":608,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":1324,"parent_record_offset":1256,"ancestor_record_offset":608,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":1296,"parent_record_offset":1256,"ancestor_record_offset":608,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":608,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1879,"record_offset":1392,"parent_record_offset":1324,"ancestor_record_offset":608,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1879,"record_offset":1800,"parent_record_offset":1324,"ancestor_record_offset":608,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1879,"record_offset":1668,"parent_record_offset":1632,"ancestor_record_offset":608,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1879,"record_offset":1632,"parent_record_offset":1596,"ancestor_record_offset":608,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1879,"record_offset":1596,"parent_record_offset":1324,"ancestor_record_offset":608,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1879,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":608,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1879,"record_offset":1548,"parent_record_offset":1324,"ancestor_record_offset":608,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1879,"record_offset":1060,"parent_record_offset":916,"ancestor_record_offset":608,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1879,"record_offset":824,"parent_record_offset":796,"ancestor_record_offset":608,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1879,"record_offset":852,"parent_record_offset":824,"ancestor_record_offset":608,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/01879.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::cartesian_to_polar
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 418
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEF5
// 首条位置: {"module":754,"record_offset":6964,"rva":14567120,"va":5383276240}
// 编译实例 RVAs: 0xde46d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bc10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33781025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383276240,"raw_hex":"e821750302","size":5,"soft_fail":false,"target_va":5417057296,"va":5383276266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33781314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383276240,"raw_hex":"e842760302","size":5,"soft_fail":false,"target_va":5417057600,"va":5383276281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":7108,"parent_record_offset":6964,"ancestor_record_offset":6964,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":7080,"parent_record_offset":6964,"ancestor_record_offset":6964,"inlinee":14783,"source_line":675,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::polar_to_cartesian
// 记录形式: compiled_procedure; instances=1
// 源码行: 425
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":7140,"rva":14567200,"va":5383276320}
// 编译实例 RVAs: 0xde4720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33781342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383276320,"raw_hex":"e85e760302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383276349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33781661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383276320,"raw_hex":"e89d770302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383276366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":7256,"parent_record_offset":7140,"ancestor_record_offset":7140,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":7284,"parent_record_offset":7140,"ancestor_record_offset":7140,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_mat3_bad
// 记录形式: compiled_procedure; instances=1
// 源码行: 432
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":11708,"rva":14569440,"va":5383278560}
// 编译实例 RVAs: 0xde4fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":754,"record_offset":12304,"parent_record_offset":11708,"ancestor_record_offset":11708,"inlinee":48855,"source_line":94,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":11832,"parent_record_offset":11708,"ancestor_record_offset":11708,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":12004,"parent_record_offset":11708,"ancestor_record_offset":11708,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":12036,"parent_record_offset":11708,"ancestor_record_offset":11708,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":11708,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":12236,"parent_record_offset":12060,"ancestor_record_offset":11708,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":11904,"parent_record_offset":11876,"ancestor_record_offset":11708,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":11876,"parent_record_offset":11856,"ancestor_record_offset":11708,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":11708,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":12136,"parent_record_offset":12100,"ancestor_record_offset":11708,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":12100,"parent_record_offset":12060,"ancestor_record_offset":11708,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":11708,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":11932,"parent_record_offset":11904,"ancestor_record_offset":11708,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_mat3a_bad
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 443
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBED0
// 首条位置: {"module":754,"inlinee":48848}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":5836,"parent_record_offset":5596,"ancestor_record_offset":5596,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":5932,"parent_record_offset":5596,"ancestor_record_offset":5596,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":6092,"parent_record_offset":5596,"ancestor_record_offset":5596,"inlinee":48849,"source_line":649,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5632,"parent_record_offset":5596,"ancestor_record_offset":5596,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6200,"parent_record_offset":6144,"ancestor_record_offset":5596,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6264,"parent_record_offset":6092,"ancestor_record_offset":5596,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":5968,"parent_record_offset":5932,"ancestor_record_offset":5596,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6056,"parent_record_offset":5932,"ancestor_record_offset":5596,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6236,"parent_record_offset":6092,"ancestor_record_offset":5596,"inlinee":32232,"source_line":137,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6028,"parent_record_offset":5932,"ancestor_record_offset":5596,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5596,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":5892,"parent_record_offset":5864,"ancestor_record_offset":5596,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":5864,"parent_record_offset":5836,"ancestor_record_offset":5596,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5728,"parent_record_offset":5692,"ancestor_record_offset":5596,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5692,"parent_record_offset":5664,"ancestor_record_offset":5596,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5664,"parent_record_offset":5632,"ancestor_record_offset":5596,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":5596,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":6120,"parent_record_offset":6092,"ancestor_record_offset":5596,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":5764,"parent_record_offset":5728,"ancestor_record_offset":5596,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_bad
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 454
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEDE
// 首条位置: {"module":754,"inlinee":48862}
// 编译实例 RVAs: 0xde44c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":6300,"parent_record_offset":5484,"ancestor_record_offset":5484,"inlinee":48850,"source_line":291,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::xz_rotation_mat3a_bad
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":5596,"parent_record_offset":5484,"ancestor_record_offset":5484,"inlinee":48848,"source_line":443,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":5836,"parent_record_offset":5596,"ancestor_record_offset":5484,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6200,"parent_record_offset":6144,"ancestor_record_offset":5484,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6264,"parent_record_offset":6092,"ancestor_record_offset":5484,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":5968,"parent_record_offset":5932,"ancestor_record_offset":5484,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6056,"parent_record_offset":5932,"ancestor_record_offset":5484,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6236,"parent_record_offset":6092,"ancestor_record_offset":5484,"inlinee":32232,"source_line":137,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":6028,"parent_record_offset":5932,"ancestor_record_offset":5484,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5484,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":754,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":5484,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":754,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":5484,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":5484,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":754,"record_offset":6588,"parent_record_offset":6464,"ancestor_record_offset":5484,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":6464,"parent_record_offset":6300,"ancestor_record_offset":5484,"inlinee":16668,"source_line":208,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":6924,"parent_record_offset":6464,"ancestor_record_offset":5484,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":5932,"parent_record_offset":5596,"ancestor_record_offset":5484,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":5892,"parent_record_offset":5864,"ancestor_record_offset":5484,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":6092,"parent_record_offset":5596,"ancestor_record_offset":5484,"inlinee":48849,"source_line":649,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":5864,"parent_record_offset":5836,"ancestor_record_offset":5484,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":5484,"inlinee":13903,"source_line":2121,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5728,"parent_record_offset":5692,"ancestor_record_offset":5484,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5692,"parent_record_offset":5664,"ancestor_record_offset":5484,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5664,"parent_record_offset":5632,"ancestor_record_offset":5484,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5632,"parent_record_offset":5596,"ancestor_record_offset":5484,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":5484,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":6120,"parent_record_offset":6092,"ancestor_record_offset":5484,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":5764,"parent_record_offset":5728,"ancestor_record_offset":5484,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_projective_rotation_bad
// 记录形式: compiled_procedure; instances=1
// 源码行: 459
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":15632,"rva":14571136,"va":5383280256}
// 编译实例 RVAs: 0xde5680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::xz_rotation_bad
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":15752,"parent_record_offset":15632,"ancestor_record_offset":15632,"inlinee":48862,"source_line":454,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":16016,"parent_record_offset":15776,"ancestor_record_offset":15632,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":16380,"parent_record_offset":16324,"ancestor_record_offset":15632,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":16444,"parent_record_offset":16272,"ancestor_record_offset":15632,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":16148,"parent_record_offset":16112,"ancestor_record_offset":15632,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":16236,"parent_record_offset":16112,"ancestor_record_offset":15632,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":16416,"parent_record_offset":16272,"ancestor_record_offset":15632,"inlinee":32232,"source_line":137,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":16208,"parent_record_offset":16112,"ancestor_record_offset":15632,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":15968,"parent_record_offset":15944,"ancestor_record_offset":15632,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":754,"record_offset":16552,"parent_record_offset":16528,"ancestor_record_offset":15632,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":754,"record_offset":16576,"parent_record_offset":16552,"ancestor_record_offset":15632,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":16600,"parent_record_offset":16576,"ancestor_record_offset":15632,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":754,"record_offset":16768,"parent_record_offset":16644,"ancestor_record_offset":15632,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":16480,"parent_record_offset":15752,"ancestor_record_offset":15632,"inlinee":48850,"source_line":291,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":16644,"parent_record_offset":16480,"ancestor_record_offset":15632,"inlinee":16668,"source_line":208,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":17104,"parent_record_offset":16644,"ancestor_record_offset":15632,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":16112,"parent_record_offset":15776,"ancestor_record_offset":15632,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":16072,"parent_record_offset":16044,"ancestor_record_offset":15632,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":16272,"parent_record_offset":15776,"ancestor_record_offset":15632,"inlinee":48849,"source_line":649,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":16044,"parent_record_offset":16016,"ancestor_record_offset":15632,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":16528,"parent_record_offset":16504,"ancestor_record_offset":15632,"inlinee":13903,"source_line":2121,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":15908,"parent_record_offset":15872,"ancestor_record_offset":15632,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":15872,"parent_record_offset":15844,"ancestor_record_offset":15632,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":15844,"parent_record_offset":15812,"ancestor_record_offset":15632,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":15812,"parent_record_offset":15776,"ancestor_record_offset":15632,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":15632,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":16300,"parent_record_offset":16272,"ancestor_record_offset":15632,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":15944,"parent_record_offset":15908,"ancestor_record_offset":15632,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::xz_rotation_mat3a_bad
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":15776,"parent_record_offset":15752,"ancestor_record_offset":15632,"inlinee":48848,"source_line":443,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_mat3a_from_forward
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEDA
// 首条位置: {"module":754,"inlinee":48858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":13716,"parent_record_offset":13496,"ancestor_record_offset":13496,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":13800,"parent_record_offset":13496,"ancestor_record_offset":13496,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":13952,"parent_record_offset":13496,"ancestor_record_offset":13496,"inlinee":48849,"source_line":649,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$89::neg
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":14156,"parent_record_offset":13496,"ancestor_record_offset":13496,"inlinee":48860,"source_line":2006,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":13528,"parent_record_offset":13496,"ancestor_record_offset":13496,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":14060,"parent_record_offset":14004,"ancestor_record_offset":13496,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":14124,"parent_record_offset":13952,"ancestor_record_offset":13496,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":13832,"parent_record_offset":13800,"ancestor_record_offset":13496,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":13916,"parent_record_offset":13800,"ancestor_record_offset":13496,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":14096,"parent_record_offset":13952,"ancestor_record_offset":13496,"inlinee":32232,"source_line":137,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":13888,"parent_record_offset":13800,"ancestor_record_offset":13496,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":13496,"inlinee":11712,"source_line":319,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":13496,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":13496,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":13496,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":13612,"parent_record_offset":13580,"ancestor_record_offset":13496,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":13496,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":13556,"parent_record_offset":13528,"ancestor_record_offset":13496,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":14004,"parent_record_offset":13980,"ancestor_record_offset":13496,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":13980,"parent_record_offset":13952,"ancestor_record_offset":13496,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":13644,"parent_record_offset":13612,"ancestor_record_offset":13496,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_from_forward
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 479
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEE1
// 首条位置: {"module":754,"inlinee":48865}
// 编译实例 RVAs: 0xde5340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":14272,"parent_record_offset":13376,"ancestor_record_offset":13376,"inlinee":48850,"source_line":291,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::xz_rotation_mat3a_from_forward
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":13496,"parent_record_offset":13376,"ancestor_record_offset":13376,"inlinee":48858,"source_line":471,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":13716,"parent_record_offset":13496,"ancestor_record_offset":13376,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":14060,"parent_record_offset":14004,"ancestor_record_offset":13376,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":14124,"parent_record_offset":13952,"ancestor_record_offset":13376,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":13832,"parent_record_offset":13800,"ancestor_record_offset":13376,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":13916,"parent_record_offset":13800,"ancestor_record_offset":13376,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":14096,"parent_record_offset":13952,"ancestor_record_offset":13376,"inlinee":32232,"source_line":137,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":13888,"parent_record_offset":13800,"ancestor_record_offset":13376,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":13376,"inlinee":11712,"source_line":319,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":13376,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":754,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":13376,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":754,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":13376,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":14400,"parent_record_offset":14376,"ancestor_record_offset":13376,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":754,"record_offset":14676,"parent_record_offset":14556,"ancestor_record_offset":13376,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":14556,"parent_record_offset":14272,"ancestor_record_offset":13376,"inlinee":16668,"source_line":208,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":14760,"parent_record_offset":14556,"ancestor_record_offset":13376,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":13800,"parent_record_offset":13496,"ancestor_record_offset":13376,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":13376,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":13952,"parent_record_offset":13496,"ancestor_record_offset":13376,"inlinee":48849,"source_line":649,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":13376,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":14328,"parent_record_offset":14304,"ancestor_record_offset":13376,"inlinee":13903,"source_line":2121,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$89::neg
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":14156,"parent_record_offset":13496,"ancestor_record_offset":13376,"inlinee":48860,"source_line":2006,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":13612,"parent_record_offset":13580,"ancestor_record_offset":13376,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":13376,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":13556,"parent_record_offset":13528,"ancestor_record_offset":13376,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":13528,"parent_record_offset":13496,"ancestor_record_offset":13376,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":14004,"parent_record_offset":13980,"ancestor_record_offset":13376,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":13980,"parent_record_offset":13952,"ancestor_record_offset":13376,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":13644,"parent_record_offset":13612,"ancestor_record_offset":13376,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_projective_rotation_from_forward
// 记录形式: compiled_procedure; instances=1
// 源码行: 483
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":17472,"rva":14571824,"va":5383280944}
// 编译实例 RVAs: 0xde5930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::xz_rotation_from_forward
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":17600,"parent_record_offset":17472,"ancestor_record_offset":17472,"inlinee":48865,"source_line":479,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":17868,"parent_record_offset":17632,"ancestor_record_offset":17472,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":18212,"parent_record_offset":18156,"ancestor_record_offset":17472,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":18276,"parent_record_offset":18104,"ancestor_record_offset":17472,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":17984,"parent_record_offset":17952,"ancestor_record_offset":17472,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":18068,"parent_record_offset":17952,"ancestor_record_offset":17472,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":18248,"parent_record_offset":18104,"ancestor_record_offset":17472,"inlinee":32232,"source_line":137,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":18040,"parent_record_offset":17952,"ancestor_record_offset":17472,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":18332,"parent_record_offset":18308,"ancestor_record_offset":17472,"inlinee":11712,"source_line":319,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":17820,"parent_record_offset":17796,"ancestor_record_offset":17472,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":754,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":17472,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":754,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":17472,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":18552,"parent_record_offset":18528,"ancestor_record_offset":17472,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":754,"record_offset":18828,"parent_record_offset":18708,"ancestor_record_offset":17472,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":18424,"parent_record_offset":17600,"ancestor_record_offset":17472,"inlinee":48850,"source_line":291,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":18708,"parent_record_offset":18424,"ancestor_record_offset":17472,"inlinee":16668,"source_line":208,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":18912,"parent_record_offset":18708,"ancestor_record_offset":17472,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":17952,"parent_record_offset":17632,"ancestor_record_offset":17472,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":17472,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":18104,"parent_record_offset":17632,"ancestor_record_offset":17472,"inlinee":48849,"source_line":649,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":17472,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":18480,"parent_record_offset":18456,"ancestor_record_offset":17472,"inlinee":13903,"source_line":2121,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$89::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":18308,"parent_record_offset":17632,"ancestor_record_offset":17472,"inlinee":48860,"source_line":2006,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":17760,"parent_record_offset":17724,"ancestor_record_offset":17472,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":17724,"parent_record_offset":17696,"ancestor_record_offset":17472,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":17696,"parent_record_offset":17664,"ancestor_record_offset":17472,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":17664,"parent_record_offset":17632,"ancestor_record_offset":17472,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":18156,"parent_record_offset":18132,"ancestor_record_offset":17472,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":18132,"parent_record_offset":18104,"ancestor_record_offset":17472,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":17796,"parent_record_offset":17760,"ancestor_record_offset":17472,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::xz_rotation_mat3a_from_forward
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":17632,"parent_record_offset":17600,"ancestor_record_offset":17472,"inlinee":48858,"source_line":471,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::cos_angle_between
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 488
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC1E
// 首条位置: {"module":3152,"inlinee":117790}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":676,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":728,"parent_record_offset":676,"ancestor_record_offset":676,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3152,"record_offset":784,"parent_record_offset":676,"ancestor_record_offset":676,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3152,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":676,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":676,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$5::intersects
// 记录形式: compiled_procedure; instances=1
// 源码行: 514
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1706,"record_offset":364,"rva":26865072,"va":5395574192}
// 编译实例 RVAs: 0x199edb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$10::ge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1706,"record_offset":480,"parent_record_offset":364,"ancestor_record_offset":364,"inlinee":64435,"source_line":2153,"archive":"_pdb/C/modules/01706.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1706,"record_offset":528,"parent_record_offset":364,"ancestor_record_offset":364,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01706.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::ge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1706,"record_offset":500,"parent_record_offset":480,"ancestor_record_offset":364,"inlinee":37230,"source_line":1922,"archive":"_pdb/C/modules/01706.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1706,"record_offset":548,"parent_record_offset":528,"ancestor_record_offset":364,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01706.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$6::intersects
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 520, 521
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC047
// 首条位置: {"module":764,"inlinee":49223}
// 编译实例 RVAs: 0x199ed90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<array$<u32,8> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":93496,"rva":12350176,"va":5381059296}
// 编译实例 RVAs: 0xbc72e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7573740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381059296,"raw_hex":"e8146f8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381059415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":580,"record_offset":93656,"parent_record_offset":93496,"ancestor_record_offset":93496,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":93628,"parent_record_offset":93496,"ancestor_record_offset":93496,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":93828,"parent_record_offset":93496,"ancestor_record_offset":93496,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":93708,"parent_record_offset":93680,"ancestor_record_offset":93496,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":93732,"parent_record_offset":93708,"ancestor_record_offset":93496,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":93792,"parent_record_offset":93680,"ancestor_record_offset":93496,"inlinee":28244,"source_line":817,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":93680,"parent_record_offset":93656,"ancestor_record_offset":93496,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":93852,"parent_record_offset":93828,"ancestor_record_offset":93496,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":93876,"parent_record_offset":93852,"ancestor_record_offset":93496,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":93920,"parent_record_offset":93876,"ancestor_record_offset":93496,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":94432,"parent_record_offset":93876,"ancestor_record_offset":93496,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":94032,"parent_record_offset":94008,"ancestor_record_offset":93496,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":93756,"parent_record_offset":93732,"ancestor_record_offset":93496,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":94008,"parent_record_offset":93920,"ancestor_record_offset":93496,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":93980,"parent_record_offset":93920,"ancestor_record_offset":93496,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<country_core::resources::stairs::stairs_state::StairsSnapshot>
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":134624,"rva":12356400,"va":5381065520}
// 编译实例 RVAs: 0xbc8b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=1
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7579965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381065520,"raw_hex":"e8c3568cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381065640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b1dc50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-700289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381065520,"raw_hex":"e87f50f5ff","size":5,"soft_fail":false,"target_va":5380365392,"va":5381065676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$8::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":580,"record_offset":134828,"parent_record_offset":134624,"ancestor_record_offset":134624,"inlinee":37019,"source_line":561,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":134800,"parent_record_offset":134624,"ancestor_record_offset":134624,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":135160,"parent_record_offset":134624,"ancestor_record_offset":134624,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":134852,"parent_record_offset":134828,"ancestor_record_offset":134624,"inlinee":14306,"source_line":3793,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":134988,"parent_record_offset":134876,"ancestor_record_offset":134624,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":134904,"parent_record_offset":134876,"ancestor_record_offset":134624,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":134928,"parent_record_offset":134904,"ancestor_record_offset":134624,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":135116,"parent_record_offset":134988,"ancestor_record_offset":134624,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":134876,"parent_record_offset":134852,"ancestor_record_offset":134624,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":135184,"parent_record_offset":135160,"ancestor_record_offset":134624,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":135208,"parent_record_offset":135184,"ancestor_record_offset":134624,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":135252,"parent_record_offset":135208,"ancestor_record_offset":134624,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":135764,"parent_record_offset":135208,"ancestor_record_offset":134624,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":135364,"parent_record_offset":135340,"ancestor_record_offset":134624,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":134952,"parent_record_offset":134928,"ancestor_record_offset":134624,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":135340,"parent_record_offset":135252,"ancestor_record_offset":134624,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":135312,"parent_record_offset":135252,"ancestor_record_offset":134624,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":135084,"parent_record_offset":135020,"ancestor_record_offset":134624,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":135052,"parent_record_offset":135020,"ancestor_record_offset":134624,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":135020,"parent_record_offset":134988,"ancestor_record_offset":134624,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<enum2$<core::option::Option<ref$<std::ffi::os_str::OsStr> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":137180,"rva":12356832,"va":5381065952}
// 编译实例 RVAs: 0xbc8ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=3
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7580402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381065952,"raw_hex":"e80e558cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381066077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::impl$56::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":137388,"parent_record_offset":137180,"ancestor_record_offset":137180,"inlinee":14031,"source_line":594,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":137360,"parent_record_offset":137180,"ancestor_record_offset":137180,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":137896,"parent_record_offset":137180,"ancestor_record_offset":137180,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":137436,"parent_record_offset":137412,"ancestor_record_offset":137180,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":137656,"parent_record_offset":137628,"ancestor_record_offset":137180,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":137460,"parent_record_offset":137436,"ancestor_record_offset":137180,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":137412,"parent_record_offset":137388,"ancestor_record_offset":137180,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":137628,"parent_record_offset":137604,"ancestor_record_offset":137180,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":137580,"parent_record_offset":137388,"ancestor_record_offset":137180,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":137796,"parent_record_offset":137628,"ancestor_record_offset":137180,"inlinee":14369,"source_line":817,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":137920,"parent_record_offset":137896,"ancestor_record_offset":137180,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":137536,"parent_record_offset":137512,"ancestor_record_offset":137180,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":137944,"parent_record_offset":137920,"ancestor_record_offset":137180,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":137988,"parent_record_offset":137944,"ancestor_record_offset":137180,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":138500,"parent_record_offset":137944,"ancestor_record_offset":137180,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":138100,"parent_record_offset":138076,"ancestor_record_offset":137180,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":137484,"parent_record_offset":137460,"ancestor_record_offset":137180,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":138076,"parent_record_offset":137988,"ancestor_record_offset":137180,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":138048,"parent_record_offset":137988,"ancestor_record_offset":137180,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$51::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":580,"record_offset":137604,"parent_record_offset":137580,"ancestor_record_offset":137180,"inlinee":20271,"source_line":1610,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":137512,"parent_record_offset":137460,"ancestor_record_offset":137180,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<glam::i32::ivec3::IVec3>
// 记录形式: compiled_procedure; instances=2
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":106624,"rva":12352160,"va":5381061280}
// 编译实例 RVAs: 0xbc7aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=6
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7575721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381061280,"raw_hex":"e857678cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381061396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::i32::ivec3::impl$265::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/i32/ivec3.rs
//     evidence={"module":580,"record_offset":106792,"parent_record_offset":106624,"ancestor_record_offset":106624,"inlinee":37007,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":106764,"parent_record_offset":106624,"ancestor_record_offset":106624,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":107332,"parent_record_offset":106624,"ancestor_record_offset":106624,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":106848,"parent_record_offset":106824,"ancestor_record_offset":106624,"inlinee":14049,"source_line":411,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":106872,"parent_record_offset":106848,"ancestor_record_offset":106624,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":106824,"parent_record_offset":106792,"ancestor_record_offset":106624,"inlinee":14048,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":107356,"parent_record_offset":107332,"ancestor_record_offset":106624,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":106948,"parent_record_offset":106924,"ancestor_record_offset":106624,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":107380,"parent_record_offset":107356,"ancestor_record_offset":106624,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":107424,"parent_record_offset":107380,"ancestor_record_offset":106624,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":107936,"parent_record_offset":107380,"ancestor_record_offset":106624,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":107536,"parent_record_offset":107512,"ancestor_record_offset":106624,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":106896,"parent_record_offset":106872,"ancestor_record_offset":106624,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":107512,"parent_record_offset":107424,"ancestor_record_offset":106624,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":107484,"parent_record_offset":107424,"ancestor_record_offset":106624,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":106924,"parent_record_offset":106872,"ancestor_record_offset":106624,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref$<slice2$<glam::u32::uvec3::UVec3> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":109352,"rva":12352592,"va":5381061712}
// 编译实例 RVAs: 0xbc7c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7576156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381061712,"raw_hex":"e8a4658cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381061831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":109536,"parent_record_offset":109352,"ancestor_record_offset":109352,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":109508,"parent_record_offset":109352,"ancestor_record_offset":109352,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":110292,"parent_record_offset":109352,"ancestor_record_offset":109352,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":109728,"parent_record_offset":109560,"ancestor_record_offset":109352,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":109588,"parent_record_offset":109560,"ancestor_record_offset":109352,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":109880,"parent_record_offset":109856,"ancestor_record_offset":109352,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":109612,"parent_record_offset":109588,"ancestor_record_offset":109352,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":109856,"parent_record_offset":109824,"ancestor_record_offset":109352,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":109560,"parent_record_offset":109536,"ancestor_record_offset":109352,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":110316,"parent_record_offset":110292,"ancestor_record_offset":109352,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":109688,"parent_record_offset":109664,"ancestor_record_offset":109352,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":110340,"parent_record_offset":110316,"ancestor_record_offset":109352,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":110384,"parent_record_offset":110340,"ancestor_record_offset":109352,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":110896,"parent_record_offset":110340,"ancestor_record_offset":109352,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":110496,"parent_record_offset":110472,"ancestor_record_offset":109352,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":109636,"parent_record_offset":109612,"ancestor_record_offset":109352,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":109904,"parent_record_offset":109880,"ancestor_record_offset":109352,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":110472,"parent_record_offset":110384,"ancestor_record_offset":109352,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":110444,"parent_record_offset":110384,"ancestor_record_offset":109352,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":109788,"parent_record_offset":109760,"ancestor_record_offset":109352,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":109760,"parent_record_offset":109728,"ancestor_record_offset":109352,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$263::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec3.rs
//     evidence={"module":580,"record_offset":109824,"parent_record_offset":109728,"ancestor_record_offset":109352,"inlinee":37003,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":109664,"parent_record_offset":109612,"ancestor_record_offset":109352,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":112312,"rva":12353088,"va":5381062208}
// 编译实例 RVAs: 0xbc7e40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7576653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381062208,"raw_hex":"e8b3638cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381062328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":112476,"parent_record_offset":112312,"ancestor_record_offset":112312,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":112448,"parent_record_offset":112312,"ancestor_record_offset":112312,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":112760,"parent_record_offset":112312,"ancestor_record_offset":112312,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":112528,"parent_record_offset":112500,"ancestor_record_offset":112312,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":112552,"parent_record_offset":112528,"ancestor_record_offset":112312,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":112500,"parent_record_offset":112476,"ancestor_record_offset":112312,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":112668,"parent_record_offset":112500,"ancestor_record_offset":112312,"inlinee":14369,"source_line":817,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":112784,"parent_record_offset":112760,"ancestor_record_offset":112312,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":112628,"parent_record_offset":112604,"ancestor_record_offset":112312,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":112808,"parent_record_offset":112784,"ancestor_record_offset":112312,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":112852,"parent_record_offset":112808,"ancestor_record_offset":112312,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":113364,"parent_record_offset":112808,"ancestor_record_offset":112312,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":112964,"parent_record_offset":112940,"ancestor_record_offset":112312,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":112576,"parent_record_offset":112552,"ancestor_record_offset":112312,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":112940,"parent_record_offset":112852,"ancestor_record_offset":112312,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":112912,"parent_record_offset":112852,"ancestor_record_offset":112312,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":112604,"parent_record_offset":112552,"ancestor_record_offset":112312,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref$<str$> >
// 记录形式: compiled_procedure; instances=8
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":124004,"rva":12355056,"va":5381064176}
// 编译实例 RVAs: 0xbc85f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=12
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7578603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381064176,"raw_hex":"e8155c8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381064278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":124156,"parent_record_offset":124004,"ancestor_record_offset":124004,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":124132,"parent_record_offset":124004,"ancestor_record_offset":124004,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":124324,"parent_record_offset":124004,"ancestor_record_offset":124004,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":124276,"parent_record_offset":124252,"ancestor_record_offset":124004,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":124180,"parent_record_offset":124156,"ancestor_record_offset":124004,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":124348,"parent_record_offset":124324,"ancestor_record_offset":124004,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":124228,"parent_record_offset":124204,"ancestor_record_offset":124004,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":124372,"parent_record_offset":124348,"ancestor_record_offset":124004,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":124252,"parent_record_offset":124228,"ancestor_record_offset":124004,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":124416,"parent_record_offset":124372,"ancestor_record_offset":124004,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":124928,"parent_record_offset":124372,"ancestor_record_offset":124004,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=184
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":124528,"parent_record_offset":124504,"ancestor_record_offset":124004,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=184
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":124504,"parent_record_offset":124416,"ancestor_record_offset":124004,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=120
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":124476,"parent_record_offset":124416,"ancestor_record_offset":124004,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":124204,"parent_record_offset":124180,"ancestor_record_offset":124004,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref_mut$<slice2$<array$<u32,2> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":5832,"rva":42833104,"va":5411542224}
// 编译实例 RVAs: 0x28d94d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=1
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38056670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411542224,"raw_hex":"e8224dbbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411542345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14218fd70
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7641112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411542224,"raw_hex":"e8e8678bff","size":5,"soft_fail":false,"target_va":5403901296,"va":5411542403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$6::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":6012,"parent_record_offset":5832,"ancestor_record_offset":5832,"inlinee":135178,"source_line":950,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":5984,"parent_record_offset":5832,"ancestor_record_offset":5832,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":6516,"parent_record_offset":5832,"ancestor_record_offset":5832,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":4084,"record_offset":6332,"parent_record_offset":6204,"ancestor_record_offset":5832,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":6204,"parent_record_offset":6036,"ancestor_record_offset":5832,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":5832,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":5832,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":6468,"parent_record_offset":6356,"ancestor_record_offset":5832,"inlinee":28244,"source_line":817,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":5832,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":6540,"parent_record_offset":6516,"ancestor_record_offset":5832,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":5832,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":5832,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":6608,"parent_record_offset":6564,"ancestor_record_offset":5832,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":7120,"parent_record_offset":6564,"ancestor_record_offset":5832,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":5832,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":5832,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":6696,"parent_record_offset":6608,"ancestor_record_offset":5832,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":6668,"parent_record_offset":6608,"ancestor_record_offset":5832,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4084,"record_offset":6300,"parent_record_offset":6236,"ancestor_record_offset":5832,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4084,"record_offset":6268,"parent_record_offset":6236,"ancestor_record_offset":5832,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4084,"record_offset":6236,"parent_record_offset":6204,"ancestor_record_offset":5832,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":6140,"parent_record_offset":6088,"ancestor_record_offset":5832,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tg_mods::identity::RecordedModEntry>
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1237,"record_offset":5468,"rva":18348624,"va":5387057744}
// 编译实例 RVAs: 0x117fa50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13572172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387057744,"raw_hex":"e8b4e730ff","size":5,"soft_fail":false,"target_va":5373485680,"va":5387057847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":5620,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":5840,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::impl$15::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1237,"record_offset":5644,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":70130,"source_line":98,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1237,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":5468,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5468,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":5468,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":5468,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":5468,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":5468,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":5468,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":5932,"parent_record_offset":5888,"ancestor_record_offset":5468,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":6444,"parent_record_offset":5888,"ancestor_record_offset":5468,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1237,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5468,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":6020,"parent_record_offset":5932,"ancestor_record_offset":5468,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":5992,"parent_record_offset":5932,"ancestor_record_offset":5468,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":5468,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType>
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":148,"record_offset":63480,"rva":4776064,"va":5373485184}
// 编译实例 RVAs: 0x48e080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=1
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373485184,"raw_hex":"e87d010000","size":5,"soft_fail":false,"target_va":5373485680,"va":5373485294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":63656,"parent_record_offset":63480,"ancestor_record_offset":63480,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":63872,"parent_record_offset":63480,"ancestor_record_offset":63480,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_tod::do_time_of_day_screen::impl$3::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_tod.rs
//     evidence={"module":148,"record_offset":63680,"parent_record_offset":63480,"ancestor_record_offset":63480,"inlinee":14386,"source_line":99,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":63724,"parent_record_offset":63700,"ancestor_record_offset":63480,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":63748,"parent_record_offset":63724,"ancestor_record_offset":63480,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":63700,"parent_record_offset":63680,"ancestor_record_offset":63480,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":63896,"parent_record_offset":63872,"ancestor_record_offset":63480,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":63824,"parent_record_offset":63800,"ancestor_record_offset":63480,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":63920,"parent_record_offset":63896,"ancestor_record_offset":63480,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":63964,"parent_record_offset":63920,"ancestor_record_offset":63480,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":64476,"parent_record_offset":63920,"ancestor_record_offset":63480,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":148,"record_offset":64076,"parent_record_offset":64052,"ancestor_record_offset":63480,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":63772,"parent_record_offset":63748,"ancestor_record_offset":63480,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":64052,"parent_record_offset":63964,"ancestor_record_offset":63480,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":64024,"parent_record_offset":63964,"ancestor_record_offset":63480,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":63800,"parent_record_offset":63748,"ancestor_record_offset":63480,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<array$<country_core::resources::walls::WallId,2>,usize> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":24520,"rva":42835328,"va":5411544448}
// 编译实例 RVAs: 0x28d9d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=1
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38058928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411544448,"raw_hex":"e85044bbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411544603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a6ae70
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15134589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411544448,"raw_hex":"e8831019ff","size":5,"soft_fail":false,"target_va":5396409968,"va":5411544552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":24728,"parent_record_offset":24520,"ancestor_record_offset":24520,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":24700,"parent_record_offset":24520,"ancestor_record_offset":24520,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":25072,"parent_record_offset":24520,"ancestor_record_offset":24520,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":4084,"record_offset":24752,"parent_record_offset":24728,"ancestor_record_offset":24520,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":24828,"parent_record_offset":24776,"ancestor_record_offset":24520,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":24800,"parent_record_offset":24776,"ancestor_record_offset":24520,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":24952,"parent_record_offset":24928,"ancestor_record_offset":24520,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":24880,"parent_record_offset":24852,"ancestor_record_offset":24520,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":24776,"parent_record_offset":24752,"ancestor_record_offset":24520,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":25096,"parent_record_offset":25072,"ancestor_record_offset":24520,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":25028,"parent_record_offset":25004,"ancestor_record_offset":24520,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":25120,"parent_record_offset":25096,"ancestor_record_offset":24520,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":25164,"parent_record_offset":25120,"ancestor_record_offset":24520,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":25676,"parent_record_offset":25120,"ancestor_record_offset":24520,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":25276,"parent_record_offset":25252,"ancestor_record_offset":24520,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":24976,"parent_record_offset":24952,"ancestor_record_offset":24520,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":25252,"parent_record_offset":25164,"ancestor_record_offset":24520,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":25224,"parent_record_offset":25164,"ancestor_record_offset":24520,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$7::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":4084,"record_offset":24852,"parent_record_offset":24828,"ancestor_record_offset":24520,"inlinee":30594,"source_line":82,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":25004,"parent_record_offset":24952,"ancestor_record_offset":24520,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<array$<u32,2>,array$<u32,2>,country_core::resources::walls::wall_holes::HoleType> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":114780,"rva":12353488,"va":5381062608}
// 编译实例 RVAs: 0xbc7fd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=5
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7577057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381062608,"raw_hex":"e81f628cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381062732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":115012,"parent_record_offset":114780,"ancestor_record_offset":114780,"inlinee":37009,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":114984,"parent_record_offset":114780,"ancestor_record_offset":114780,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":115580,"parent_record_offset":114780,"ancestor_record_offset":114780,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":580,"record_offset":115036,"parent_record_offset":115012,"ancestor_record_offset":114780,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":115428,"parent_record_offset":115404,"ancestor_record_offset":114780,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":115088,"parent_record_offset":115060,"ancestor_record_offset":114780,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":115112,"parent_record_offset":115088,"ancestor_record_offset":114780,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":115172,"parent_record_offset":115060,"ancestor_record_offset":114780,"inlinee":28244,"source_line":817,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":115404,"parent_record_offset":115380,"ancestor_record_offset":114780,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":115060,"parent_record_offset":115036,"ancestor_record_offset":114780,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":115604,"parent_record_offset":115580,"ancestor_record_offset":114780,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":115528,"parent_record_offset":115504,"ancestor_record_offset":114780,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":115628,"parent_record_offset":115604,"ancestor_record_offset":114780,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":115672,"parent_record_offset":115628,"ancestor_record_offset":114780,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":116184,"parent_record_offset":115628,"ancestor_record_offset":114780,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":115784,"parent_record_offset":115760,"ancestor_record_offset":114780,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":115136,"parent_record_offset":115112,"ancestor_record_offset":114780,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":115760,"parent_record_offset":115672,"ancestor_record_offset":114780,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":115732,"parent_record_offset":115672,"ancestor_record_offset":114780,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$34::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":580,"record_offset":115380,"parent_record_offset":115012,"ancestor_record_offset":114780,"inlinee":37011,"source_line":445,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":115504,"parent_record_offset":115452,"ancestor_record_offset":114780,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<array$<u32,3>,glam::u32::uvec3::UVec3> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":117600,"rva":12353968,"va":5381063088}
// 编译实例 RVAs: 0xbc81b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=5
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7577532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381063088,"raw_hex":"e844608cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381063207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":117792,"parent_record_offset":117600,"ancestor_record_offset":117600,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":117764,"parent_record_offset":117600,"ancestor_record_offset":117600,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":118448,"parent_record_offset":117600,"ancestor_record_offset":117600,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":580,"record_offset":117816,"parent_record_offset":117792,"ancestor_record_offset":117600,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":117868,"parent_record_offset":117840,"ancestor_record_offset":117600,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":118044,"parent_record_offset":118020,"ancestor_record_offset":117600,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":117892,"parent_record_offset":117868,"ancestor_record_offset":117600,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":118020,"parent_record_offset":117988,"ancestor_record_offset":117600,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":117952,"parent_record_offset":117840,"ancestor_record_offset":117600,"inlinee":28244,"source_line":817,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":117840,"parent_record_offset":117816,"ancestor_record_offset":117600,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":118472,"parent_record_offset":118448,"ancestor_record_offset":117600,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":118120,"parent_record_offset":118096,"ancestor_record_offset":117600,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":118496,"parent_record_offset":118472,"ancestor_record_offset":117600,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":118540,"parent_record_offset":118496,"ancestor_record_offset":117600,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":119052,"parent_record_offset":118496,"ancestor_record_offset":117600,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":118652,"parent_record_offset":118628,"ancestor_record_offset":117600,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":117916,"parent_record_offset":117892,"ancestor_record_offset":117600,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":118068,"parent_record_offset":118044,"ancestor_record_offset":117600,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":118628,"parent_record_offset":118540,"ancestor_record_offset":117600,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":118600,"parent_record_offset":118540,"ancestor_record_offset":117600,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$263::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec3.rs
//     evidence={"module":580,"record_offset":117988,"parent_record_offset":117792,"ancestor_record_offset":117600,"inlinee":37003,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":118096,"parent_record_offset":118044,"ancestor_record_offset":117600,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::components::TreeStorageIndex,usize,ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":11120,"rva":42833568,"va":5411542688}
// 编译实例 RVAs: 0x28d96a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38057131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411542688,"raw_hex":"e8554bbbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411542806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":11332,"parent_record_offset":11120,"ancestor_record_offset":11120,"inlinee":37009,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":11304,"parent_record_offset":11120,"ancestor_record_offset":11120,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":11840,"parent_record_offset":11120,"ancestor_record_offset":11120,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":11788,"parent_record_offset":11760,"ancestor_record_offset":11120,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":11404,"parent_record_offset":11380,"ancestor_record_offset":11120,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":11380,"parent_record_offset":11356,"ancestor_record_offset":11120,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":11120,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":11664,"parent_record_offset":11332,"ancestor_record_offset":11120,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":11864,"parent_record_offset":11840,"ancestor_record_offset":11120,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":11480,"parent_record_offset":11456,"ancestor_record_offset":11120,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":11120,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":11888,"parent_record_offset":11864,"ancestor_record_offset":11120,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":11120,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":11932,"parent_record_offset":11888,"ancestor_record_offset":11120,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":12444,"parent_record_offset":11888,"ancestor_record_offset":11120,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":11120,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":11120,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":12020,"parent_record_offset":11932,"ancestor_record_offset":11120,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":11992,"parent_record_offset":11932,"ancestor_record_offset":11120,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$14::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":4084,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":11120,"inlinee":135182,"source_line":54,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":11456,"parent_record_offset":11404,"ancestor_record_offset":11120,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":11120,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::resources::walls::WallId,usize,i32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":268,"rva":42832208,"va":5411541328}
// 编译实例 RVAs: 0x28d9150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=3
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38055771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411541328,"raw_hex":"e8a550bbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411541446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":468,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":37009,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":440,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":972,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":268,"inlinee":14049,"source_line":411,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":268,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":268,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":268,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":800,"parent_record_offset":468,"ancestor_record_offset":268,"inlinee":14048,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":268,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":268,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":268,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":1064,"parent_record_offset":1020,"ancestor_record_offset":268,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":1576,"parent_record_offset":1020,"ancestor_record_offset":268,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":268,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":268,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":1152,"parent_record_offset":1064,"ancestor_record_offset":268,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":1124,"parent_record_offset":1064,"ancestor_record_offset":268,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$7::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":4084,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":268,"inlinee":30594,"source_line":82,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":592,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1563,"record_offset":7824,"rva":21741568,"va":5390450688}
// 编译实例 RVAs: 0x14bc000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=3
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16965129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390450688,"raw_hex":"e8f721fdfe","size":5,"soft_fail":false,"target_va":5373485680,"va":5390450804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":8044,"parent_record_offset":7824,"ancestor_record_offset":7824,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":8016,"parent_record_offset":7824,"ancestor_record_offset":7824,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":8412,"parent_record_offset":7824,"ancestor_record_offset":7824,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":8116,"parent_record_offset":8092,"ancestor_record_offset":7824,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":8360,"parent_record_offset":8332,"ancestor_record_offset":7824,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":8092,"parent_record_offset":8068,"ancestor_record_offset":7824,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":7824,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":8236,"parent_record_offset":8044,"ancestor_record_offset":7824,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":7824,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":7824,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":7824,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":7824,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":7824,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":8504,"parent_record_offset":8460,"ancestor_record_offset":7824,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":9016,"parent_record_offset":8460,"ancestor_record_offset":7824,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1563,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":7824,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":8140,"parent_record_offset":8116,"ancestor_record_offset":7824,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":8592,"parent_record_offset":8504,"ancestor_record_offset":7824,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":8564,"parent_record_offset":8504,"ancestor_record_offset":7824,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$30::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1563,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":7824,"inlinee":11788,"source_line":155,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":8168,"parent_record_offset":8116,"ancestor_record_offset":7824,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":7824,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1563,"record_offset":272,"rva":21740720,"va":5390449840}
// 编译实例 RVAs: 0x14bbcb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16964282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390449840,"raw_hex":"e84625fdfe","size":5,"soft_fail":false,"target_va":5373485680,"va":5390449957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":496,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":37009,"source_line":893,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":1004,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":272,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":812,"parent_record_offset":784,"ancestor_record_offset":272,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":272,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":272,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":860,"parent_record_offset":496,"ancestor_record_offset":272,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":272,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":688,"parent_record_offset":496,"ancestor_record_offset":272,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":272,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":272,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":272,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":272,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":272,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":1096,"parent_record_offset":1052,"ancestor_record_offset":272,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":1608,"parent_record_offset":1052,"ancestor_record_offset":272,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1563,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":272,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":272,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":272,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":1184,"parent_record_offset":1096,"ancestor_record_offset":272,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":1156,"parent_record_offset":1096,"ancestor_record_offset":272,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$30::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1563,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":272,"inlinee":11788,"source_line":155,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":620,"parent_record_offset":568,"ancestor_record_offset":272,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":272,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::startup::startup_window_cut_sdf::WindowCutShape,array$<u32,2>,array$<u32,3>,array$<u32,2>,bool> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":128868,"rva":12355840,"va":5381064960}
// 编译实例 RVAs: 0xbc8900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=8
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7579415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381064960,"raw_hex":"e8e9588cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381065090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$26::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129128,"parent_record_offset":128868,"ancestor_record_offset":128868,"inlinee":37013,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":129100,"parent_record_offset":128868,"ancestor_record_offset":128868,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":129980,"parent_record_offset":128868,"ancestor_record_offset":128868,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":580,"record_offset":129348,"parent_record_offset":129128,"ancestor_record_offset":128868,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129200,"parent_record_offset":129176,"ancestor_record_offset":128868,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129400,"parent_record_offset":129372,"ancestor_record_offset":128868,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129888,"parent_record_offset":129864,"ancestor_record_offset":128868,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129224,"parent_record_offset":129200,"ancestor_record_offset":128868,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$0::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129864,"parent_record_offset":129128,"ancestor_record_offset":128868,"inlinee":13427,"source_line":848,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129484,"parent_record_offset":129372,"ancestor_record_offset":128868,"inlinee":28244,"source_line":817,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129176,"parent_record_offset":129156,"ancestor_record_offset":128868,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":129372,"parent_record_offset":129348,"ancestor_record_offset":128868,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":130004,"parent_record_offset":129980,"ancestor_record_offset":128868,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":129300,"parent_record_offset":129276,"ancestor_record_offset":128868,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":130028,"parent_record_offset":130004,"ancestor_record_offset":128868,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":130072,"parent_record_offset":130028,"ancestor_record_offset":128868,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":130584,"parent_record_offset":130028,"ancestor_record_offset":128868,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":130184,"parent_record_offset":130160,"ancestor_record_offset":128868,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":129248,"parent_record_offset":129224,"ancestor_record_offset":128868,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":130160,"parent_record_offset":130072,"ancestor_record_offset":128868,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":130132,"parent_record_offset":130072,"ancestor_record_offset":128868,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_window_cut_sdf::impl$8::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_window_cut_sdf.rs
//     evidence={"module":580,"record_offset":129156,"parent_record_offset":129128,"ancestor_record_offset":128868,"inlinee":37015,"source_line":53,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":129276,"parent_record_offset":129224,"ancestor_record_offset":128868,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1237,"record_offset":2812,"rva":18348224,"va":5387057344}
// 编译实例 RVAs: 0x117f8c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13571788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387057344,"raw_hex":"e834e930ff","size":5,"soft_fail":false,"target_va":5373485680,"va":5387057463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":3060,"parent_record_offset":2812,"ancestor_record_offset":2812,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":3032,"parent_record_offset":2812,"ancestor_record_offset":2812,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":3448,"parent_record_offset":2812,"ancestor_record_offset":2812,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":2812,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":2812,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":2812,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":2812,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":3272,"parent_record_offset":3252,"ancestor_record_offset":2812,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":2812,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":2812,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":3496,"parent_record_offset":3472,"ancestor_record_offset":2812,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":3540,"parent_record_offset":3496,"ancestor_record_offset":2812,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":4052,"parent_record_offset":3496,"ancestor_record_offset":2812,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1237,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":2812,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":2812,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":3628,"parent_record_offset":3540,"ancestor_record_offset":2812,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2812,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":3600,"parent_record_offset":3540,"ancestor_record_offset":2812,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::action_set::impl$20::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":1237,"record_offset":3252,"parent_record_offset":3060,"ancestor_record_offset":2812,"inlinee":70126,"source_line":148,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$4::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1237,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2812,"inlinee":14364,"source_line":30,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":3184,"parent_record_offset":3132,"ancestor_record_offset":2812,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1237,"record_offset":264,"rva":18347824,"va":5387056944}
// 编译实例 RVAs: 0x117f730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13571387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387056944,"raw_hex":"e8c5ea30ff","size":5,"soft_fail":false,"target_va":5373485680,"va":5387057062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":456,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":428,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":792,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":264,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":264,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":648,"parent_record_offset":456,"ancestor_record_offset":264,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":264,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":264,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":264,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":264,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":884,"parent_record_offset":840,"ancestor_record_offset":264,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":1396,"parent_record_offset":840,"ancestor_record_offset":264,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1237,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":264,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":264,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":972,"parent_record_offset":884,"ancestor_record_offset":264,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":264,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1237,"record_offset":944,"parent_record_offset":884,"ancestor_record_offset":264,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$4::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1237,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":264,"inlinee":14364,"source_line":30,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":580,"parent_record_offset":528,"ancestor_record_offset":264,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":149068,"rva":12358848,"va":5381067968}
// 编译实例 RVAs: 0xbc94c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7582412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381067968,"raw_hex":"e8344d8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381068087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149320,"parent_record_offset":149068,"ancestor_record_offset":149068,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":149292,"parent_record_offset":149068,"ancestor_record_offset":149068,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":149888,"parent_record_offset":149068,"ancestor_record_offset":149068,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149564,"parent_record_offset":149540,"ancestor_record_offset":149068,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149392,"parent_record_offset":149368,"ancestor_record_offset":149068,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149832,"parent_record_offset":149804,"ancestor_record_offset":149068,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149588,"parent_record_offset":149564,"ancestor_record_offset":149068,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149368,"parent_record_offset":149344,"ancestor_record_offset":149068,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149540,"parent_record_offset":149512,"ancestor_record_offset":149068,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149732,"parent_record_offset":149708,"ancestor_record_offset":149068,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":149708,"parent_record_offset":149512,"ancestor_record_offset":149068,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":149912,"parent_record_offset":149888,"ancestor_record_offset":149068,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":149468,"parent_record_offset":149444,"ancestor_record_offset":149068,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":149780,"parent_record_offset":149756,"ancestor_record_offset":149068,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":149936,"parent_record_offset":149912,"ancestor_record_offset":149068,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":149804,"parent_record_offset":149780,"ancestor_record_offset":149068,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":149980,"parent_record_offset":149936,"ancestor_record_offset":149068,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":150492,"parent_record_offset":149936,"ancestor_record_offset":149068,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":150092,"parent_record_offset":150068,"ancestor_record_offset":149068,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":149612,"parent_record_offset":149588,"ancestor_record_offset":149068,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":150068,"parent_record_offset":149980,"ancestor_record_offset":149068,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":149416,"parent_record_offset":149392,"ancestor_record_offset":149068,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":150040,"parent_record_offset":149980,"ancestor_record_offset":149068,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$56::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":149512,"parent_record_offset":149320,"ancestor_record_offset":149068,"inlinee":14031,"source_line":594,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$15::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":580,"record_offset":149344,"parent_record_offset":149320,"ancestor_record_offset":149068,"inlinee":37023,"source_line":14,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":149444,"parent_record_offset":149392,"ancestor_record_offset":149068,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":149756,"parent_record_offset":149732,"ancestor_record_offset":149068,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<i32,tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":148,"record_offset":55836,"rva":4774864,"va":5373483984}
// 编译实例 RVAs: 0x48dbd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373483984,"raw_hex":"e826060000","size":5,"soft_fail":false,"target_va":5373485680,"va":5373484101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":56052,"parent_record_offset":55836,"ancestor_record_offset":55836,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":56024,"parent_record_offset":55836,"ancestor_record_offset":55836,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":56440,"parent_record_offset":55836,"ancestor_record_offset":55836,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":56100,"parent_record_offset":56076,"ancestor_record_offset":55836,"inlinee":14049,"source_line":411,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":56288,"parent_record_offset":56264,"ancestor_record_offset":55836,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":56124,"parent_record_offset":56100,"ancestor_record_offset":55836,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":56312,"parent_record_offset":56288,"ancestor_record_offset":55836,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":56076,"parent_record_offset":56052,"ancestor_record_offset":55836,"inlinee":14048,"source_line":812,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":56264,"parent_record_offset":56244,"ancestor_record_offset":55836,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":56464,"parent_record_offset":56440,"ancestor_record_offset":55836,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":56200,"parent_record_offset":56176,"ancestor_record_offset":55836,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":56488,"parent_record_offset":56464,"ancestor_record_offset":55836,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":56532,"parent_record_offset":56488,"ancestor_record_offset":55836,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":57044,"parent_record_offset":56488,"ancestor_record_offset":55836,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":148,"record_offset":56644,"parent_record_offset":56620,"ancestor_record_offset":55836,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":56336,"parent_record_offset":56312,"ancestor_record_offset":55836,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":56148,"parent_record_offset":56124,"ancestor_record_offset":55836,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":56620,"parent_record_offset":56532,"ancestor_record_offset":55836,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":56592,"parent_record_offset":56532,"ancestor_record_offset":55836,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":56176,"parent_record_offset":56124,"ancestor_record_offset":55836,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_tod::do_time_of_day_screen::impl$3::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_tod.rs
//     evidence={"module":148,"record_offset":56244,"parent_record_offset":56052,"ancestor_record_offset":55836,"inlinee":14386,"source_line":99,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<slice2$<array$<u32,3> > >,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":120468,"rva":12354432,"va":5381063552}
// 编译实例 RVAs: 0xbc8380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=7
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7578003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381063552,"raw_hex":"e86d5e8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381063678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":120692,"parent_record_offset":120468,"ancestor_record_offset":120468,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":120664,"parent_record_offset":120468,"ancestor_record_offset":120468,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":121984,"parent_record_offset":120468,"ancestor_record_offset":120468,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":580,"record_offset":121040,"parent_record_offset":120908,"ancestor_record_offset":120468,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":120908,"parent_record_offset":120740,"ancestor_record_offset":120468,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":120768,"parent_record_offset":120740,"ancestor_record_offset":120468,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":121568,"parent_record_offset":121544,"ancestor_record_offset":120468,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":120792,"parent_record_offset":120768,"ancestor_record_offset":120468,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":121544,"parent_record_offset":121512,"ancestor_record_offset":120468,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":121176,"parent_record_offset":121064,"ancestor_record_offset":120468,"inlinee":28244,"source_line":817,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":120740,"parent_record_offset":120716,"ancestor_record_offset":120468,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":120716,"parent_record_offset":120692,"ancestor_record_offset":120468,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":122008,"parent_record_offset":121984,"ancestor_record_offset":120468,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":120868,"parent_record_offset":120844,"ancestor_record_offset":120468,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":122032,"parent_record_offset":122008,"ancestor_record_offset":120468,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":122076,"parent_record_offset":122032,"ancestor_record_offset":120468,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":122588,"parent_record_offset":122032,"ancestor_record_offset":120468,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":122188,"parent_record_offset":122164,"ancestor_record_offset":120468,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":120816,"parent_record_offset":120792,"ancestor_record_offset":120468,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":121592,"parent_record_offset":121568,"ancestor_record_offset":120468,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":122164,"parent_record_offset":122076,"ancestor_record_offset":120468,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":122136,"parent_record_offset":122076,"ancestor_record_offset":120468,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":121008,"parent_record_offset":120940,"ancestor_record_offset":120468,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":120976,"parent_record_offset":120940,"ancestor_record_offset":120468,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":120940,"parent_record_offset":120908,"ancestor_record_offset":120468,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$263::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec3.rs
//     evidence={"module":580,"record_offset":121512,"parent_record_offset":121416,"ancestor_record_offset":120468,"inlinee":37003,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":120844,"parent_record_offset":120792,"ancestor_record_offset":120468,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":101476,"rva":12351104,"va":5381060224}
// 编译实例 RVAs: 0xbc7680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=21
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7574689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381060224,"raw_hex":"e85f6b8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381060364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":101732,"parent_record_offset":101476,"ancestor_record_offset":101476,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":101704,"parent_record_offset":101476,"ancestor_record_offset":101476,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":104600,"parent_record_offset":101476,"ancestor_record_offset":101476,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":580,"record_offset":103756,"parent_record_offset":103660,"ancestor_record_offset":101476,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":101948,"parent_record_offset":101780,"ancestor_record_offset":101476,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":101808,"parent_record_offset":101780,"ancestor_record_offset":101476,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":102140,"parent_record_offset":102116,"ancestor_record_offset":101476,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":101832,"parent_record_offset":101808,"ancestor_record_offset":101476,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":102116,"parent_record_offset":102084,"ancestor_record_offset":101476,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":101780,"parent_record_offset":101756,"ancestor_record_offset":101476,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":101756,"parent_record_offset":101732,"ancestor_record_offset":101476,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":104624,"parent_record_offset":104600,"ancestor_record_offset":101476,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":101908,"parent_record_offset":101884,"ancestor_record_offset":101476,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":104648,"parent_record_offset":104624,"ancestor_record_offset":101476,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":104696,"parent_record_offset":104648,"ancestor_record_offset":101476,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":105208,"parent_record_offset":104648,"ancestor_record_offset":101476,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":104808,"parent_record_offset":104784,"ancestor_record_offset":101476,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":101856,"parent_record_offset":101832,"ancestor_record_offset":101476,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":102164,"parent_record_offset":102140,"ancestor_record_offset":101476,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":104784,"parent_record_offset":104696,"ancestor_record_offset":101476,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":104756,"parent_record_offset":104696,"ancestor_record_offset":101476,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":102008,"parent_record_offset":101980,"ancestor_record_offset":101476,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":101980,"parent_record_offset":101948,"ancestor_record_offset":101476,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$263::hash
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec3.rs
//     evidence={"module":580,"record_offset":102084,"parent_record_offset":102044,"ancestor_record_offset":101476,"inlinee":37003,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":101884,"parent_record_offset":101832,"ancestor_record_offset":101476,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::intersect::impl$1::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/intersect.rs
//     evidence={"module":580,"record_offset":102044,"parent_record_offset":101948,"ancestor_record_offset":101476,"inlinee":37005,"source_line":32,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,i32,usize,u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":2992,"rva":42832640,"va":5411541760}
// 编译实例 RVAs: 0x28d9300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=5
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38056191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411541760,"raw_hex":"e8014fbbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411541866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$25::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3168,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":15793,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":3140,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":3812,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":2992,"inlinee":14049,"source_line":411,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":2992,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":2992,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":2992,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3668,"parent_record_offset":3168,"ancestor_record_offset":2992,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3528,"parent_record_offset":3168,"ancestor_record_offset":2992,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3360,"parent_record_offset":3168,"ancestor_record_offset":2992,"inlinee":14048,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":2992,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":2992,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":2992,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":2992,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":2992,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":2992,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":2992,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":3904,"parent_record_offset":3860,"ancestor_record_offset":2992,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":4416,"parent_record_offset":3860,"ancestor_record_offset":2992,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":2992,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":3576,"parent_record_offset":3552,"ancestor_record_offset":2992,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":2992,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":3992,"parent_record_offset":3904,"ancestor_record_offset":2992,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":3964,"parent_record_offset":3904,"ancestor_record_offset":2992,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":3460,"parent_record_offset":3408,"ancestor_record_offset":2992,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":2992,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,u32,country_core::resources::walls::WallId> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":13860,"rva":42834016,"va":5411543136}
// 编译实例 RVAs: 0x28d9860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38057567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411543136,"raw_hex":"e8a149bbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411543242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":14068,"parent_record_offset":13860,"ancestor_record_offset":13860,"inlinee":37009,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":14040,"parent_record_offset":13860,"ancestor_record_offset":13860,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":14572,"parent_record_offset":13860,"ancestor_record_offset":13860,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":14284,"parent_record_offset":14260,"ancestor_record_offset":13860,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":14212,"parent_record_offset":14188,"ancestor_record_offset":13860,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":13860,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":14260,"parent_record_offset":14068,"ancestor_record_offset":13860,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":13860,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":14116,"parent_record_offset":14092,"ancestor_record_offset":13860,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":14092,"parent_record_offset":14068,"ancestor_record_offset":13860,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":14596,"parent_record_offset":14572,"ancestor_record_offset":13860,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":13860,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":13860,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":14620,"parent_record_offset":14596,"ancestor_record_offset":13860,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":14188,"parent_record_offset":14164,"ancestor_record_offset":13860,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":14664,"parent_record_offset":14620,"ancestor_record_offset":13860,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":15176,"parent_record_offset":14620,"ancestor_record_offset":13860,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":13860,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":13860,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":14308,"parent_record_offset":14284,"ancestor_record_offset":13860,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":14752,"parent_record_offset":14664,"ancestor_record_offset":13860,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":14724,"parent_record_offset":14664,"ancestor_record_offset":13860,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$7::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":4084,"record_offset":14400,"parent_record_offset":14068,"ancestor_record_offset":13860,"inlinee":30594,"source_line":82,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":14336,"parent_record_offset":14284,"ancestor_record_offset":13860,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":13860,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,usize,u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":27092,"rva":42835744,"va":5411544864}
// 编译实例 RVAs: 0x28d9f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38059295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411544864,"raw_hex":"e8e142bbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411544970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":27264,"parent_record_offset":27092,"ancestor_record_offset":27092,"inlinee":37009,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":27236,"parent_record_offset":27092,"ancestor_record_offset":27092,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":27740,"parent_record_offset":27092,"ancestor_record_offset":27092,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":27620,"parent_record_offset":27596,"ancestor_record_offset":27092,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":27408,"parent_record_offset":27384,"ancestor_record_offset":27092,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":27480,"parent_record_offset":27456,"ancestor_record_offset":27092,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":27596,"parent_record_offset":27264,"ancestor_record_offset":27092,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":27456,"parent_record_offset":27264,"ancestor_record_offset":27092,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":27312,"parent_record_offset":27288,"ancestor_record_offset":27092,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":27288,"parent_record_offset":27264,"ancestor_record_offset":27092,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":27764,"parent_record_offset":27740,"ancestor_record_offset":27092,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":27556,"parent_record_offset":27532,"ancestor_record_offset":27092,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":27360,"parent_record_offset":27336,"ancestor_record_offset":27092,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":27788,"parent_record_offset":27764,"ancestor_record_offset":27092,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":27384,"parent_record_offset":27360,"ancestor_record_offset":27092,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":27832,"parent_record_offset":27788,"ancestor_record_offset":27092,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":28344,"parent_record_offset":27788,"ancestor_record_offset":27092,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":27944,"parent_record_offset":27920,"ancestor_record_offset":27092,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":27504,"parent_record_offset":27480,"ancestor_record_offset":27092,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":27644,"parent_record_offset":27620,"ancestor_record_offset":27092,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":27920,"parent_record_offset":27832,"ancestor_record_offset":27092,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":27892,"parent_record_offset":27832,"ancestor_record_offset":27092,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":27532,"parent_record_offset":27480,"ancestor_record_offset":27092,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":27336,"parent_record_offset":27312,"ancestor_record_offset":27092,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,usize> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":21996,"rva":42834912,"va":5411544032}
// 编译实例 RVAs: 0x28d9be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=3
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38058463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411544032,"raw_hex":"e82146bbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411544138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":22164,"parent_record_offset":21996,"ancestor_record_offset":21996,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":22136,"parent_record_offset":21996,"ancestor_record_offset":21996,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":22500,"parent_record_offset":21996,"ancestor_record_offset":21996,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":22308,"parent_record_offset":22284,"ancestor_record_offset":21996,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":22380,"parent_record_offset":22356,"ancestor_record_offset":21996,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":22356,"parent_record_offset":22164,"ancestor_record_offset":21996,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":22212,"parent_record_offset":22188,"ancestor_record_offset":21996,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":22188,"parent_record_offset":22164,"ancestor_record_offset":21996,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":22524,"parent_record_offset":22500,"ancestor_record_offset":21996,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":22456,"parent_record_offset":22432,"ancestor_record_offset":21996,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":22260,"parent_record_offset":22236,"ancestor_record_offset":21996,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":22548,"parent_record_offset":22524,"ancestor_record_offset":21996,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":22284,"parent_record_offset":22260,"ancestor_record_offset":21996,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":22592,"parent_record_offset":22548,"ancestor_record_offset":21996,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":23104,"parent_record_offset":22548,"ancestor_record_offset":21996,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":22704,"parent_record_offset":22680,"ancestor_record_offset":21996,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":22404,"parent_record_offset":22380,"ancestor_record_offset":21996,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":22680,"parent_record_offset":22592,"ancestor_record_offset":21996,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":22652,"parent_record_offset":22592,"ancestor_record_offset":21996,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":22432,"parent_record_offset":22380,"ancestor_record_offset":21996,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":22236,"parent_record_offset":22212,"ancestor_record_offset":21996,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<system_clutter::autoclutter::ClutterRegionId,i32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":8536,"rva":18347824,"va":5387056944}
// 编译实例 RVAs: 0x117f730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13571387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387056944,"raw_hex":"e8c5ea30ff","size":5,"soft_fail":false,"target_va":5373485680,"va":5387057062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":8736,"parent_record_offset":8536,"ancestor_record_offset":8536,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":8708,"parent_record_offset":8536,"ancestor_record_offset":8536,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":9100,"parent_record_offset":8536,"ancestor_record_offset":8536,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":8536,"inlinee":14049,"source_line":411,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":8536,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":8536,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":8536,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":8928,"parent_record_offset":8736,"ancestor_record_offset":8536,"inlinee":14048,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":8536,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":8536,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":9148,"parent_record_offset":9124,"ancestor_record_offset":8536,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":9192,"parent_record_offset":9148,"ancestor_record_offset":8536,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":9704,"parent_record_offset":9148,"ancestor_record_offset":8536,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":9304,"parent_record_offset":9280,"ancestor_record_offset":8536,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":9000,"parent_record_offset":8976,"ancestor_record_offset":8536,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":9280,"parent_record_offset":9192,"ancestor_record_offset":8536,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":8536,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":9252,"parent_record_offset":9192,"ancestor_record_offset":8536,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":8860,"parent_record_offset":8808,"ancestor_record_offset":8536,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::impl$26::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":4084,"record_offset":8760,"parent_record_offset":8736,"ancestor_record_offset":8536,"inlinee":135180,"source_line":322,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u32,glam::u32::uvec3::UVec3> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":143248,"rva":12357888,"va":5381067008}
// 编译实例 RVAs: 0xbc9100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7581449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381067008,"raw_hex":"e8f7508cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381067124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":143428,"parent_record_offset":143248,"ancestor_record_offset":143248,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":143400,"parent_record_offset":143248,"ancestor_record_offset":143248,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":144052,"parent_record_offset":143248,"ancestor_record_offset":143248,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":143476,"parent_record_offset":143452,"ancestor_record_offset":143248,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":143452,"parent_record_offset":143428,"ancestor_record_offset":143248,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":144076,"parent_record_offset":144052,"ancestor_record_offset":143248,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":143552,"parent_record_offset":143528,"ancestor_record_offset":143248,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":144100,"parent_record_offset":144076,"ancestor_record_offset":143248,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":144144,"parent_record_offset":144100,"ancestor_record_offset":143248,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":144656,"parent_record_offset":144100,"ancestor_record_offset":143248,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":144256,"parent_record_offset":144232,"ancestor_record_offset":143248,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":143500,"parent_record_offset":143476,"ancestor_record_offset":143248,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":144232,"parent_record_offset":144144,"ancestor_record_offset":143248,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":144204,"parent_record_offset":144144,"ancestor_record_offset":143248,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$263::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec3.rs
//     evidence={"module":580,"record_offset":143592,"parent_record_offset":143428,"ancestor_record_offset":143248,"inlinee":37003,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":143528,"parent_record_offset":143476,"ancestor_record_offset":143248,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u32,usize,ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4084,"record_offset":19320,"rva":42834464,"va":5411543584}
// 编译实例 RVAs: 0x28d9a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38058026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411543584,"raw_hex":"e8d647bbfd","size":5,"soft_fail":false,"target_va":5373485680,"va":5411543701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":19496,"parent_record_offset":19320,"ancestor_record_offset":19320,"inlinee":37009,"source_line":893,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":19468,"parent_record_offset":19320,"ancestor_record_offset":19320,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":19976,"parent_record_offset":19320,"ancestor_record_offset":19320,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":19544,"parent_record_offset":19520,"ancestor_record_offset":19320,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":19924,"parent_record_offset":19896,"ancestor_record_offset":19320,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":19684,"parent_record_offset":19660,"ancestor_record_offset":19320,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":19520,"parent_record_offset":19496,"ancestor_record_offset":19320,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":19660,"parent_record_offset":19496,"ancestor_record_offset":19320,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":19824,"parent_record_offset":19800,"ancestor_record_offset":19320,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":4084,"record_offset":19800,"parent_record_offset":19496,"ancestor_record_offset":19320,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":20000,"parent_record_offset":19976,"ancestor_record_offset":19320,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":19620,"parent_record_offset":19596,"ancestor_record_offset":19320,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":19872,"parent_record_offset":19848,"ancestor_record_offset":19320,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":20024,"parent_record_offset":20000,"ancestor_record_offset":19320,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":19896,"parent_record_offset":19872,"ancestor_record_offset":19320,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":20068,"parent_record_offset":20024,"ancestor_record_offset":19320,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4084,"record_offset":20580,"parent_record_offset":20024,"ancestor_record_offset":19320,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4084,"record_offset":20180,"parent_record_offset":20156,"ancestor_record_offset":19320,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":19320,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":19320,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":20156,"parent_record_offset":20068,"ancestor_record_offset":19320,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4084,"record_offset":20128,"parent_record_offset":20068,"ancestor_record_offset":19320,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":19596,"parent_record_offset":19544,"ancestor_record_offset":19320,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4084,"record_offset":19848,"parent_record_offset":19824,"ancestor_record_offset":19320,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/04084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,core::any::TypeId> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":148,"record_offset":58460,"rva":4775264,"va":5373484384}
// 编译实例 RVAs: 0x48dd60, 0x2b13d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373484384,"raw_hex":"e894040000","size":5,"soft_fail":false,"target_va":5373485680,"va":5373484503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a6ae70
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17469276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413879136,"raw_hex":"e8a470f5fe","size":5,"soft_fail":false,"target_va":5396409968,"va":5413879239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":58636,"parent_record_offset":58460,"ancestor_record_offset":58460,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":58608,"parent_record_offset":58460,"ancestor_record_offset":58460,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":58972,"parent_record_offset":58460,"ancestor_record_offset":58460,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":148,"record_offset":58800,"parent_record_offset":58636,"ancestor_record_offset":58460,"inlinee":10558,"source_line":871,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":58684,"parent_record_offset":58660,"ancestor_record_offset":58460,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":58660,"parent_record_offset":58636,"ancestor_record_offset":58460,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":58996,"parent_record_offset":58972,"ancestor_record_offset":58460,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":58760,"parent_record_offset":58736,"ancestor_record_offset":58460,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":59020,"parent_record_offset":58996,"ancestor_record_offset":58460,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":59064,"parent_record_offset":59020,"ancestor_record_offset":58460,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":59576,"parent_record_offset":59020,"ancestor_record_offset":58460,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=69
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":148,"record_offset":59176,"parent_record_offset":59152,"ancestor_record_offset":58460,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=69
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":59152,"parent_record_offset":59064,"ancestor_record_offset":58460,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":58708,"parent_record_offset":58684,"ancestor_record_offset":58460,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=45
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":59124,"parent_record_offset":59064,"ancestor_record_offset":58460,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":58736,"parent_record_offset":58684,"ancestor_record_offset":58460,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":139916,"rva":12357264,"va":5381066384}
// 编译实例 RVAs: 0xbc8e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=9
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7580831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381066384,"raw_hex":"e861538cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381066506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140124,"parent_record_offset":139916,"ancestor_record_offset":139916,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":140096,"parent_record_offset":139916,"ancestor_record_offset":139916,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":141228,"parent_record_offset":139916,"ancestor_record_offset":139916,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":580,"record_offset":140576,"parent_record_offset":140480,"ancestor_record_offset":139916,"inlinee":14366,"source_line":348,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140480,"parent_record_offset":140312,"ancestor_record_offset":139916,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140340,"parent_record_offset":140312,"ancestor_record_offset":139916,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140868,"parent_record_offset":140836,"ancestor_record_offset":139916,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140172,"parent_record_offset":140148,"ancestor_record_offset":139916,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140364,"parent_record_offset":140340,"ancestor_record_offset":139916,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140836,"parent_record_offset":140792,"ancestor_record_offset":139916,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140148,"parent_record_offset":140124,"ancestor_record_offset":139916,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140312,"parent_record_offset":140288,"ancestor_record_offset":139916,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":140288,"parent_record_offset":140124,"ancestor_record_offset":139916,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":141252,"parent_record_offset":141228,"ancestor_record_offset":139916,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":140248,"parent_record_offset":140224,"ancestor_record_offset":139916,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":141276,"parent_record_offset":141252,"ancestor_record_offset":139916,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":141320,"parent_record_offset":141276,"ancestor_record_offset":139916,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":141832,"parent_record_offset":141276,"ancestor_record_offset":139916,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":141432,"parent_record_offset":141408,"ancestor_record_offset":139916,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":140388,"parent_record_offset":140364,"ancestor_record_offset":139916,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":140900,"parent_record_offset":140868,"ancestor_record_offset":139916,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":141408,"parent_record_offset":141320,"ancestor_record_offset":139916,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":140196,"parent_record_offset":140172,"ancestor_record_offset":139916,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":141380,"parent_record_offset":141320,"ancestor_record_offset":139916,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":140540,"parent_record_offset":140512,"ancestor_record_offset":139916,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":140512,"parent_record_offset":140480,"ancestor_record_offset":139916,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$261::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":580,"record_offset":140792,"parent_record_offset":140768,"ancestor_record_offset":139916,"inlinee":37021,"source_line":20,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":140224,"parent_record_offset":140172,"ancestor_record_offset":139916,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec2::UVec2> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":146072,"rva":12358336,"va":5381067456}
// 编译实例 RVAs: 0xbc92c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=4
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7581903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381067456,"raw_hex":"e8314f8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381067578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146268,"parent_record_offset":146072,"ancestor_record_offset":146072,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":146240,"parent_record_offset":146072,"ancestor_record_offset":146072,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":147048,"parent_record_offset":146072,"ancestor_record_offset":146072,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146624,"parent_record_offset":146456,"ancestor_record_offset":146072,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146484,"parent_record_offset":146456,"ancestor_record_offset":146072,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146772,"parent_record_offset":146748,"ancestor_record_offset":146072,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146316,"parent_record_offset":146292,"ancestor_record_offset":146072,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146508,"parent_record_offset":146484,"ancestor_record_offset":146072,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146748,"parent_record_offset":146720,"ancestor_record_offset":146072,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146292,"parent_record_offset":146268,"ancestor_record_offset":146072,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146456,"parent_record_offset":146432,"ancestor_record_offset":146072,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":146432,"parent_record_offset":146268,"ancestor_record_offset":146072,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":147072,"parent_record_offset":147048,"ancestor_record_offset":146072,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":146392,"parent_record_offset":146368,"ancestor_record_offset":146072,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":147096,"parent_record_offset":147072,"ancestor_record_offset":146072,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":147140,"parent_record_offset":147096,"ancestor_record_offset":146072,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":147652,"parent_record_offset":147096,"ancestor_record_offset":146072,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":147252,"parent_record_offset":147228,"ancestor_record_offset":146072,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":146532,"parent_record_offset":146508,"ancestor_record_offset":146072,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":146796,"parent_record_offset":146772,"ancestor_record_offset":146072,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":147228,"parent_record_offset":147140,"ancestor_record_offset":146072,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":146340,"parent_record_offset":146316,"ancestor_record_offset":146072,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":147200,"parent_record_offset":147140,"ancestor_record_offset":146072,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":146684,"parent_record_offset":146656,"ancestor_record_offset":146072,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":146656,"parent_record_offset":146624,"ancestor_record_offset":146072,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$261::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":580,"record_offset":146720,"parent_record_offset":146624,"ancestor_record_offset":146072,"inlinee":37021,"source_line":20,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":146368,"parent_record_offset":146316,"ancestor_record_offset":146072,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":98336,"rva":12350576,"va":5381059696}
// 编译实例 RVAs: 0xbc7470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=5
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7574142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381059696,"raw_hex":"e8826d8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381059817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":98532,"parent_record_offset":98336,"ancestor_record_offset":98336,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":98504,"parent_record_offset":98336,"ancestor_record_offset":98336,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":99456,"parent_record_offset":98336,"ancestor_record_offset":98336,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":98888,"parent_record_offset":98720,"ancestor_record_offset":98336,"inlinee":14309,"source_line":234,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":98748,"parent_record_offset":98720,"ancestor_record_offset":98336,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":99040,"parent_record_offset":99016,"ancestor_record_offset":98336,"inlinee":11791,"source_line":374,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":98580,"parent_record_offset":98556,"ancestor_record_offset":98336,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":98772,"parent_record_offset":98748,"ancestor_record_offset":98336,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":99016,"parent_record_offset":98984,"ancestor_record_offset":98336,"inlinee":11790,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":98556,"parent_record_offset":98532,"ancestor_record_offset":98336,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":98720,"parent_record_offset":98696,"ancestor_record_offset":98336,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":98696,"parent_record_offset":98532,"ancestor_record_offset":98336,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":99480,"parent_record_offset":99456,"ancestor_record_offset":98336,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":98656,"parent_record_offset":98632,"ancestor_record_offset":98336,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":99504,"parent_record_offset":99480,"ancestor_record_offset":98336,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":99548,"parent_record_offset":99504,"ancestor_record_offset":98336,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":100060,"parent_record_offset":99504,"ancestor_record_offset":98336,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":99660,"parent_record_offset":99636,"ancestor_record_offset":98336,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":98796,"parent_record_offset":98772,"ancestor_record_offset":98336,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":99064,"parent_record_offset":99040,"ancestor_record_offset":98336,"inlinee":11792,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":99636,"parent_record_offset":99548,"ancestor_record_offset":98336,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":98604,"parent_record_offset":98580,"ancestor_record_offset":98336,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":99608,"parent_record_offset":99548,"ancestor_record_offset":98336,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":98948,"parent_record_offset":98920,"ancestor_record_offset":98336,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":98920,"parent_record_offset":98888,"ancestor_record_offset":98336,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$263::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec3.rs
//     evidence={"module":580,"record_offset":98984,"parent_record_offset":98888,"ancestor_record_offset":98336,"inlinee":37003,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":98632,"parent_record_offset":98580,"ancestor_record_offset":98336,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,ref$<std::time::SystemTime> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":132000,"rva":12355440,"va":5381064560}
// 编译实例 RVAs: 0xbc8770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7579004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381064560,"raw_hex":"e8845a8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381064679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":132184,"parent_record_offset":132000,"ancestor_record_offset":132000,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":132156,"parent_record_offset":132000,"ancestor_record_offset":132000,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":132604,"parent_record_offset":132000,"ancestor_record_offset":132000,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i64
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":132444,"parent_record_offset":132420,"ancestor_record_offset":132000,"inlinee":20850,"source_line":417,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":132232,"parent_record_offset":132208,"ancestor_record_offset":132000,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":132208,"parent_record_offset":132184,"ancestor_record_offset":132000,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$17::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":132420,"parent_record_offset":132396,"ancestor_record_offset":132000,"inlinee":37017,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":132348,"parent_record_offset":132184,"ancestor_record_offset":132000,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":132628,"parent_record_offset":132604,"ancestor_record_offset":132000,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":132308,"parent_record_offset":132284,"ancestor_record_offset":132000,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":132652,"parent_record_offset":132628,"ancestor_record_offset":132000,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":132696,"parent_record_offset":132652,"ancestor_record_offset":132000,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":133208,"parent_record_offset":132652,"ancestor_record_offset":132000,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":132808,"parent_record_offset":132784,"ancestor_record_offset":132000,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":132784,"parent_record_offset":132696,"ancestor_record_offset":132000,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":132256,"parent_record_offset":132232,"ancestor_record_offset":132000,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":132756,"parent_record_offset":132696,"ancestor_record_offset":132000,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":132284,"parent_record_offset":132232,"ancestor_record_offset":132000,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":580,"record_offset":132396,"parent_record_offset":132372,"ancestor_record_offset":132000,"inlinee":20847,"source_line":153,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$35::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":580,"record_offset":132372,"parent_record_offset":132348,"ancestor_record_offset":132000,"inlinee":20845,"source_line":248,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,ref$<u64> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":126344,"rva":12355440,"va":5381064560}
// 编译实例 RVAs: 0xbc8770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7579004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381064560,"raw_hex":"e8845a8cff","size":5,"soft_fail":false,"target_va":5373485680,"va":5381064679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":126512,"parent_record_offset":126344,"ancestor_record_offset":126344,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":126484,"parent_record_offset":126344,"ancestor_record_offset":126344,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":126848,"parent_record_offset":126344,"ancestor_record_offset":126344,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":126560,"parent_record_offset":126536,"ancestor_record_offset":126344,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":126536,"parent_record_offset":126512,"ancestor_record_offset":126344,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":580,"record_offset":126676,"parent_record_offset":126512,"ancestor_record_offset":126344,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":126872,"parent_record_offset":126848,"ancestor_record_offset":126344,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":126636,"parent_record_offset":126612,"ancestor_record_offset":126344,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":126896,"parent_record_offset":126872,"ancestor_record_offset":126344,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":126940,"parent_record_offset":126896,"ancestor_record_offset":126344,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":580,"record_offset":127452,"parent_record_offset":126896,"ancestor_record_offset":126344,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":127052,"parent_record_offset":127028,"ancestor_record_offset":126344,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":127028,"parent_record_offset":126940,"ancestor_record_offset":126344,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":126584,"parent_record_offset":126560,"ancestor_record_offset":126344,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":127000,"parent_record_offset":126940,"ancestor_record_offset":126344,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":580,"record_offset":126612,"parent_record_offset":126560,"ancestor_record_offset":126344,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,u64> >
// 记录形式: compiled_procedure; instances=4
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":148,"record_offset":60992,"rva":4775664,"va":5373484784}
// 编译实例 RVAs: 0x48def0, 0x2b13ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=6
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373484784,"raw_hex":"e804030000","size":5,"soft_fail":false,"target_va":5373485680,"va":5373484903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a6ae70
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17469644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413879504,"raw_hex":"e8346ff5fe","size":5,"soft_fail":false,"target_va":5396409968,"va":5413879607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":61152,"parent_record_offset":60992,"ancestor_record_offset":60992,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":61124,"parent_record_offset":60992,"ancestor_record_offset":60992,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":61460,"parent_record_offset":60992,"ancestor_record_offset":60992,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":61200,"parent_record_offset":61176,"ancestor_record_offset":60992,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":61176,"parent_record_offset":61152,"ancestor_record_offset":60992,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":61484,"parent_record_offset":61460,"ancestor_record_offset":60992,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":61276,"parent_record_offset":61252,"ancestor_record_offset":60992,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":61508,"parent_record_offset":61484,"ancestor_record_offset":60992,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":61552,"parent_record_offset":61508,"ancestor_record_offset":60992,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":62064,"parent_record_offset":61508,"ancestor_record_offset":60992,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=92
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":148,"record_offset":61664,"parent_record_offset":61640,"ancestor_record_offset":60992,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=92
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":61640,"parent_record_offset":61552,"ancestor_record_offset":60992,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":61224,"parent_record_offset":61200,"ancestor_record_offset":60992,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":61612,"parent_record_offset":61552,"ancestor_record_offset":60992,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":61252,"parent_record_offset":61200,"ancestor_record_offset":60992,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<usize,u8> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1563,"record_offset":5364,"rva":21741168,"va":5390450288}
// 编译实例 RVAs: 0x14bbe70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048e270
//     owner=unresolved_namespace; count=2
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16964732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390450288,"raw_hex":"e88423fdfe","size":5,"soft_fail":false,"target_va":5373485680,"va":5390450407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":5524,"parent_record_offset":5364,"ancestor_record_offset":5364,"inlinee":14036,"source_line":893,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":5496,"parent_record_offset":5364,"ancestor_record_offset":5364,"inlinee":14292,"source_line":106,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":5804,"parent_record_offset":5364,"ancestor_record_offset":5364,"inlinee":11793,"source_line":140,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":5364,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":5572,"parent_record_offset":5548,"ancestor_record_offset":5364,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":5548,"parent_record_offset":5524,"ancestor_record_offset":5364,"inlinee":6066,"source_line":812,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1563,"record_offset":5688,"parent_record_offset":5524,"ancestor_record_offset":5364,"inlinee":14042,"source_line":812,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":5364,"inlinee":11794,"source_line":242,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":5364,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":5364,"inlinee":11795,"source_line":311,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":5896,"parent_record_offset":5852,"ancestor_record_offset":5364,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1563,"record_offset":6408,"parent_record_offset":5852,"ancestor_record_offset":5364,"inlinee":11796,"source_line":367,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1563,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":5364,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":5364,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":5984,"parent_record_offset":5896,"ancestor_record_offset":5364,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1563,"record_offset":5956,"parent_record_offset":5896,"ancestor_record_offset":5364,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1563,"record_offset":5624,"parent_record_offset":5572,"ancestor_record_offset":5364,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01563.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::fit
// 记录形式: compiled_procedure; instances=1
// 源码行: 554
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":19228,"rva":14572416,"va":5383281536}
// 编译实例 RVAs: 0xde5b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":19324,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":48854,"source_line":266,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::srgb_oetf
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 560
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBECF
// 首条位置: {"module":754,"inlinee":48847}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":4188,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::srgb_eotf
// 记录形式: inlinee_declaration; instances=1
// 源码行: 568
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEE4
// 首条位置: {"module":754,"inlinee":48868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 579
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":17296,"rva":14571728,"va":5383280848}
// 编译实例 RVAs: 0xde58d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::scale_u32_packed_srgba_color::scale_linear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":17416,"parent_record_offset":17296,"ancestor_record_offset":17296,"inlinee":48864,"source_line":581,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":17436,"parent_record_offset":17416,"ancestor_record_offset":17296,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color::scale_linear
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 581
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEE0
// 首条位置: {"module":754,"inlinee":48864}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":17436,"parent_record_offset":17416,"ancestor_record_offset":17416,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color_premultiplied::scale_srgb
// 记录形式: inlinee_declaration; instances=1
// 源码行: 598
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEE3
// 首条位置: {"module":754,"inlinee":48867}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color_premultiplied::scale_linear
// 记录形式: inlinee_declaration; instances=1
// 源码行: 602
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBEE5
// 首条位置: {"module":754,"inlinee":48869}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::pack_srgba_u32
// 记录形式: compiled_procedure; instances=1
// 源码行: 614
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":4044,"rva":14565312,"va":5383274432}
// 编译实例 RVAs: 0xde3fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33783420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383274432,"raw_hex":"e87c7e0302","size":5,"soft_fail":false,"target_va":5417057936,"va":5383274511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":4348,"parent_record_offset":4044,"ancestor_record_offset":4044,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::srgb_oetf
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":4156,"parent_record_offset":4044,"ancestor_record_offset":4044,"inlinee":48847,"source_line":560,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":4188,"parent_record_offset":4156,"ancestor_record_offset":4044,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$9::iterator_slice_range
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 661
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1365C
// 首条位置: {"module":1460,"inlinee":79452}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::slice_iter
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 674
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1365A
// 首条位置: {"module":1460,"inlinee":79450}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$9::iterator_slice_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1460,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":79452,"source_line":661,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$21::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 682
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69BB
// 首条位置: {"module":447,"inlinee":27067}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::IsoTransform::from_translation
// 记录形式: compiled_procedure; instances=1
// 源码行: 694
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":588,"rva":14564512,"va":5383273632}
// 编译实例 RVAs: 0xde3ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":712,"parent_record_offset":588,"ancestor_record_offset":588,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":752,"parent_record_offset":732,"ancestor_record_offset":588,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":732,"parent_record_offset":712,"ancestor_record_offset":588,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::IsoTransform::from_translation_rotation
// 记录形式: compiled_procedure; instances=1
// 源码行: 701
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":788,"rva":14564560,"va":5383273680}
// 编译实例 RVAs: 0xde3cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":920,"parent_record_offset":788,"ancestor_record_offset":788,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":960,"parent_record_offset":940,"ancestor_record_offset":788,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":940,"parent_record_offset":920,"ancestor_record_offset":788,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::IsoTransform::nested_in
// 记录形式: compiled_procedure; instances=1
// 源码行: 712
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":996,"rva":14564592,"va":5383273712}
// 编译实例 RVAs: 0xde3cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":1120,"parent_record_offset":996,"ancestor_record_offset":996,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$27::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":1924,"parent_record_offset":996,"ancestor_record_offset":996,"inlinee":11714,"source_line":1123,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":1868,"parent_record_offset":996,"ancestor_record_offset":996,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":1536,"parent_record_offset":1120,"ancestor_record_offset":996,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":1444,"parent_record_offset":1388,"ancestor_record_offset":996,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":996,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":1244,"parent_record_offset":1164,"ancestor_record_offset":996,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":1308,"parent_record_offset":1120,"ancestor_record_offset":996,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":996,"inlinee":11715,"source_line":804,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":1564,"parent_record_offset":1120,"ancestor_record_offset":996,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":996,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":754,"record_offset":1164,"parent_record_offset":1120,"ancestor_record_offset":996,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::random_orthogonal_quat
// 记录形式: compiled_procedure; instances=1
// 源码行: 733
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":12812,"rva":14569968,"va":5383279088}
// 编译实例 RVAs: 0xde51f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33778484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383279088,"raw_hex":"e8346b0302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383279207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33778808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383279088,"raw_hex":"e8786c0302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383279219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t104007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383279088,"raw_hex":"e847960100","size":5,"soft_fail":false,"target_va":5383383152,"va":5383279140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":13108,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":12940,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":12968,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":754,"record_offset":13052,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":13076,"parent_record_offset":13052,"ancestor_record_offset":12812,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00754.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::IsoTransform
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 19
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "constructor": "tuple$", "arguments": ["utils::IsoTransform", "ref$<country_core::systems::collision::world::RaycastMeshData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >", "constructor": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > >", "constructor": "rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > >", "constructor": "ref_mut$", "arguments": ["rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > > >", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >", "ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rapier3d::pipeline::query_pipeline::QueryPipeline::intersections_with_shape<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > >", "constructor": "rapier3d::pipeline::query_pipeline::QueryPipeline::intersections_with_shape", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> >", "constructor": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rapier3d::pipeline::query_pipeline::QueryPipeline::intersections_with_shape<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > >", "constructor": "rapier3d::pipeline::query_pipeline::QueryPipeline::intersections_with_shape", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> >", "constructor": "country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >", "constructor": "country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> >", "constructor": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure$0", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >", "constructor": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >,tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >", "tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > >", "constructor": "rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > > >", "constructor": "ref_mut$", "arguments": ["rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > > > >", "constructor": "ref_mut$", "arguments": ["ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > > > > >", "constructor": "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor", "arguments": ["rapier3d::geometry::collider_components::ColliderHandle", "ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node<rapier3d::geometry::collider_components::ColliderHandle,parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > > > > > >", "constructor": "parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node", "arguments": ["rapier3d::geometry::collider_components::ColliderHandle", "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > > >", "constructor": "ref_mut$", "arguments": ["ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > > > >", "constructor": "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor", "arguments": ["rapier3d::geometry::collider_components::ColliderHandle", "ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node<rapier3d::geometry::collider_components::ColliderHandle,parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > > > > >", "constructor": "parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node", "arguments": ["rapier3d::geometry::collider_components::ColliderHandle", "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>", "constructor": "tuple$", "arguments": ["utils::IsoTransform", "country_core::systems::collision::world::RaycastMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform>", "constructor": "country_core::systems::collision::world::RaycastWorld::upsert_collider", "arguments": ["country_core::resources::stairs::stairs_state::StairGraphEdge", "utils::IsoTransform"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform>", "constructor": "country_core::systems::collision::world::RaycastWorld::upsert_collider", "arguments": ["tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>", "utils::IsoTransform"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2> >,system_color::style_picker_target::impl$0::capture::closure_env$3> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2> >,system_color::style_picker_target::impl$0::capture::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >", "constructor": "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>", "ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >", "constructor": "core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0", "arguments": ["core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >", "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >", "constructor": "core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0", "arguments": ["core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >", "core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,system_color::style_picker_target::impl$0::capture::closure_env$2,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >", "core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>", "system_color::style_picker_target::impl$0::capture::closure_env$2", "tuple$<>", "core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >", "constructor": "ref_mut$", "arguments": ["core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<country_core::systems::collision::world::SubworldColliderHandle,core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>", "system_color::style_picker_target::impl$0::capture::closure_env$1", "tuple$<>", "ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > > >", "constructor": "core::iter::adapters::chain::impl$1::try_fold", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::systems::collision::world::ColliderMeshInstance>,system_color::style_picker_target::impl$0::capture::closure_env$0>,system_color::style_picker_target::impl$0::capture::closure_env$1>", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::decorator::DecoratorId>,core::iter::adapters::flatten::Flatten<core::array::iter::IntoIter<enum2$<core::option::Option<country_core::systems::collision::world::SubworldColliderHandle> >,2> >,system_color::style_picker_target::impl$0::capture::closure_env$2>", "tuple$<>", "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,ref_mut$<system_color::style_picker_target::impl$0::capture::closure_env$3> >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<utils::IsoTransform,country_core::systems::collision::world::RaycastMesh>,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::Side
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 65; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bvh::bvh::bvh_node::BvhNodeBuildArgs<mesh_to_sdf::generate::generic::bvh::BvhNode<array$<f32,3> >,f32,3>,bvh::bvh::bvh_node::BvhNodeBuildArgs<mesh_to_sdf::generate::generic::bvh::BvhNode<array$<f32,3> >,f32,3>)
// 候选参数类型: ["bvh::bvh::bvh_node::BvhNodeBuildArgs<mesh_to_sdf::generate::generic::bvh::BvhNode<array$<f32,3> >,f32,3>", "bvh::bvh::bvh_node::BvhNodeBuildArgs<mesh_to_sdf::generate::generic::bvh::BvhNode<array$<f32,3> >,f32,3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::LastKnownGraphicsDevice (*)(ref$<rhapsody::raw::init::AvailableGraphicsDevice>)
// 候选参数类型: ["ref$<rhapsody::raw::init::AvailableGraphicsDevice>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: time::formatting::formattable::impl$8::format_into<alloc::vec::Vec<u8,alloc::alloc::Global> >
// 信息 ID: 709c619c4633b1d8357df855
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] time::formatting::formattable::impl$8::format_into -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] time::formatting::formattable::sealed::Sealed::format<alloc::vec::Vec<enum2$<time::format_description::borrowed_format_item::BorrowedFormatItem>,alloc::alloc::Global> > @ 0x1e34736 -> 0x2756fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::IsoTransform::from_translation
// 信息 ID: 31a9fbe2f6ffebedb97925bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36d2d5 -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37f820 -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::generate_playermade_stair_decorators @ 0x89ab26 -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph @ 0x8a8113 -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph @ 0x8a84ec -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_cuboid @ 0x9f94e3 -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_sphere @ 0x9f95f1 -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fb527 -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_subtype::DecoratorCollision::empty @ 0xc57a9d -> 0xde3ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::IsoTransform::from_translation_rotation
// 信息 ID: 402852b9800a6583142c309b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::impl$22::to_offset_collider_builder @ 0xb99ec4 -> 0xde3cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$15::from @ 0xba0b7a -> 0xde3cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::roof_uv_and_point_at_ws_pos @ 0xbe814f -> 0xde3cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$21::to_offset_collider_builder @ 0xc409a0 -> 0xde3cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xc796b8 -> 0xde3cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::IsoTransform::nested_in
// 信息 ID: 650927b3a23447eea451fc7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::impl$22::to_offset_collider_builder @ 0xb99ed4 -> 0xde3cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$45::into_transformed_shape @ 0xba1856 -> 0xde3cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$21::to_offset_collider_builder @ 0xc409b0 -> 0xde3cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xc796d4 -> 0xde3cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<array$<u32,8> >
// 信息 ID: faabdf1737c9a871a25789d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "8"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["array$<u32,8>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_compound @ 0x9fa01e -> 0xbc72e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<country_core::resources::stairs::stairs_state::StairsSnapshot>
// 信息 ID: 4aa5c08fe312f9cf83f2a42b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["country_core::resources::stairs::stairs_state::StairsSnapshot"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::stairs::stairs_state::StairsState::compute_checksum @ 0xa55fed -> 0xbc8b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::stairs::stairs_state::StairsState::restore_snapshot @ 0xa560e9 -> 0xbc8b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::stairs::stairs_state::StairsState::extract_diff_and_make_backup @ 0xa57095 -> 0xbc8b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::stairs::stairs_state::StairsSnapshot::new @ 0xa5782b -> 0xbc8b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<enum2$<core::option::Option<ref$<std::ffi::os_str::OsStr> > > >
// 信息 ID: be0b1e8ef9a5471618a8740e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::ffi::os_str::OsStr"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<std::ffi::os_str::OsStr>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<std::ffi::os_str::OsStr> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["enum2$<core::option::Option<ref$<std::ffi::os_str::OsStr> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x17ce354 -> 0xbc8ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<glam::i32::ivec3::IVec3>
// 信息 ID: 262dc0f369e31bdc7e8dceee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["glam::i32::ivec3::IVec3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections @ 0xbee655 -> 0xbc7aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bca91 -> 0xbc7aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<slice2$<glam::u32::uvec3::UVec3> > >
// 信息 ID: ba9b297fb3ae566c592fd53c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<slice2$<glam::u32::uvec3::UVec3> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_convex @ 0x9f9265 -> 0xbc7c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<slice2$<u8> > >
// 信息 ID: 6297b97a55d673cd90099d8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<str$> >
// 信息 ID: 4f3d74c171d827eb0d690029
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 305 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$1::decorate_clutter_selector @ 0x71f7ef -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::text_hint_popup @ 0x720bc7 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::text_hint_popup @ 0x720deb -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::impl$0::as_id @ 0x725200 -> 0xbc85f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::menu_button @ 0x72dd66 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::settings_button @ 0x72dfe6 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::menu_button_no_nav @ 0x72e2c6 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::label @ 0x72e7c6 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::impl$1::as_id @ 0x730582 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::instantiate_ui_template_impl @ 0x7324e2 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::instantiate_ui_template_impl @ 0x733058 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x852112 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8528f1 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8529bb -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x852d10 -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x852dda -> 0xbc85f0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref_mut$<slice2$<array$<u32,2> > > >
// 信息 ID: cd0fe80cc30a160e0d5e6318
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<u32,2>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["slice2$<array$<u32,2> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref_mut$<slice2$<array$<u32,2> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x21c81aa -> 0x28d94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x21c820b -> 0x28d94d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tg_mods::identity::RecordedModEntry>
// 信息 ID: 7ed84e6724567365b92efd43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["tg_mods::identity::RecordedModEntry"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items<toolbar::menu::mod_checklist::replacement_mod_checklist::closure_env$0> @ 0x231b581 -> 0x117fa50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType>
// 信息 ID: 672b8e819f657096dcbed5be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311312 -> 0x48e080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<country_core::resources::walls::WallId,2>,usize> >
// 信息 ID: a1b50b41ae4c553002945431
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["country_core::resources::walls::WallId", "2"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<country_core::resources::walls::WallId,2>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<country_core::resources::walls::WallId,2>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitch_bricks @ 0x21c756b -> 0x28d9d80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<u32,2>,array$<u32,2>,country_core::resources::walls::wall_holes::HoleType> >
// 信息 ID: 1ee7bade1ee7fc17580e125c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<u32,2>", "array$<u32,2>", "country_core::resources::walls::wall_holes::HoleType"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<u32,2>,array$<u32,2>,country_core::resources::walls::wall_holes::HoleType>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::wall_holes::InProgressHoleStorage::add @ 0xc4e2d4 -> 0xbc7fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoles::add @ 0xc4e8cd -> 0xbc7fd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<u32,3>,glam::u32::uvec3::UVec3> >
// 信息 ID: bbf7626dcd7251910cb903ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "3"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<u32,3>", "glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<u32,3>,glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_cuboid @ 0x9f949f -> 0xbc81b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fb4da -> 0xbc81b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_subtype::DecoratorCollision::empty @ 0xc57a6a -> 0xbc81b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::components::TreeStorageIndex,usize,ref$<str$> > >
// 信息 ID: b42bf6fea3af60e34334ed8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::components::TreeStorageIndex", "usize", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::components::TreeStorageIndex,usize,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x28cfcae -> 0x28d96a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::resources::walls::WallId,usize,i32> >
// 信息 ID: 0e9672c5963af9ab9db3318e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["country_core::resources::walls::WallId", "usize", "i32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::resources::walls::WallId,usize,i32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitch_beams @ 0x21c6b1a -> 0x28d9150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitch_beams @ 0x21c6b42 -> 0x28d9150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
// 信息 ID: e820cfd06fbfcd4a10247f40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::resources::walls::decorator::DecoratorId", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c418c -> 0x14bc000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c4308 -> 0x14bc000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter @ 0x8c8b9e -> 0x14bc000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter @ 0x8c8cdb -> 0x14bc000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x8c9fea -> 0x14bc000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x8ca151 -> 0x14bc000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize> >
// 信息 ID: 57b0706f42893123c71e96ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::resources::walls::decorator::DecoratorId", "ref$<str$>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x884796 -> 0x14bbcb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c3fe4 -> 0x14bbcb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8ccc8b -> 0x14bbcb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::startup::startup_window_cut_sdf::WindowCutShape,array$<u32,2>,array$<u32,3>,array$<u32,2>,bool> >
// 信息 ID: 5c79bf6225105830a4b81424
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "3"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::startup::startup_window_cut_sdf::WindowCutShape", "array$<u32,2>", "array$<u32,3>", "array$<u32,2>", "bool"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::startup::startup_window_cut_sdf::WindowCutShape,array$<u32,2>,array$<u32,3>,array$<u32,2>,bool>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::window_cuts::InProgressWindowCutStorage::add @ 0xb3bb7b -> 0xbc8900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType> >
// 信息 ID: d54b4e963662342468280211
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["country_core::systems::ui::GlobalId", "country_core::input::action_set::DigitalActionSetActionType"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::MenuButtonView::hotkey @ 0x7202e4 -> 0x117f8c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::PillButtonView::hotkey @ 0x720821 -> 0x117f8c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,u32> >
// 信息 ID: 1de32f12a368a0411be720d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["country_core::systems::ui::GlobalId", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::systems::ui::GlobalId,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$0::decorate_hovered @ 0x71f0de -> 0x117f730 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$1::decorate_clutter_selector @ 0x71f67d -> 0x117f730 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bd2a9 -> 0x117f730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$> > > > >
// 信息 ID: fa575a7de4a7a64bc7b0be70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<str$>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<str$> >"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::systems::ui::resources::ui_state::WidgetId", "enum2$<core::option::Option<ref$<str$> > >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::signifier_create::signifier_create @ 0xaf7cb8 -> 0xbc94c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<i32,tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType> >
// 信息 ID: db8872701dcc9ff65b2123b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["i32", "tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<i32,tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311544 -> 0x48dbd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311eb4 -> 0x48dbd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<slice2$<array$<u32,3> > >,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 信息 ID: 78b1fb948cea849d221bae08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<u32,3>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<u32,3> >"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<slice2$<array$<u32,3> > >", "ref$<slice2$<glam::u32::uvec3::UVec3> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<slice2$<array$<u32,3> > >,ref$<slice2$<glam::u32::uvec3::UVec3> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x9f9711 -> 0xbc8380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > > >
// 信息 ID: 6da6693e77d82c0e00d3a9e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["utils::geometry::intersect::Triangle"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<utils::geometry::intersect::Triangle>"]; 字段布局 = unknown。
// [类型关系] array$ -> ["glam::u32::uvec3::UVec3", "3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<glam::u32::uvec3::UVec3,3>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<glam::u32::uvec3::UVec3,3> >"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<slice2$<utils::geometry::intersect::Triangle> >", "ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_unindexed @ 0x9fa1d5 -> 0xbc7680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,i32,usize,u32> >
// 信息 ID: 95a8e4d427ffbfc0a5082f12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "i32", "usize", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,i32,usize,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows @ 0x28cb704 -> 0x28d9300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,u32,country_core::resources::walls::WallId> >
// 信息 ID: 5d86b267240583e7607d89c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "u32", "country_core::resources::walls::WallId"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,u32,country_core::resources::walls::WallId>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates @ 0x28c8e1a -> 0x28d9860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,usize,u32> >
// 信息 ID: 62be363be3cd3492db874dab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "usize", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,usize,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows @ 0x28cb9ef -> 0x28d9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows @ 0x28cbc20 -> 0x28d9f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,usize> >
// 信息 ID: d79d75b25ab618b5daf45a19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter @ 0x21c4215 -> 0x28d9be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<system_clutter::autoclutter::ClutterRegionId,i32> >
// 信息 ID: 622bd4cda7b79adef4ab6138
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["system_clutter::autoclutter::ClutterRegionId", "i32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<system_clutter::autoclutter::ClutterRegionId,i32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$0::decorate_hovered @ 0x71f0de -> 0x117f730 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$1::decorate_clutter_selector @ 0x71f67d -> 0x117f730 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bd2a9 -> 0x117f730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u32,glam::u32::uvec3::UVec3> >
// 信息 ID: f9b5df0e6bf9d05f8ffb27c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u32", "glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u32,glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_sphere @ 0x9f95cb -> 0xbc9100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u32,usize,ref$<str$> > >
// 信息 ID: 6f49beebde26ccb660fc37b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u32", "usize", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u32,usize,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing @ 0x28d04e0 -> 0x28d9a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,core::any::TypeId> >
// 信息 ID: 45a52c884d1602a5817e1e31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u64", "core::any::TypeId"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,core::any::TypeId>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 433 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186de6 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186ed3 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186f9b -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x187065 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x22208d -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::child_with_content<tiny_glade::gui::gui_fog::FogSliderContent,ref$<str$>,tiny_glade::gui::gui_fog::fill_slider::closure_env$0> @ 0x222689 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::child<alloc::string::String> @ 0x222f61 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::tab_switcher<ref$<str$> > @ 0x223305 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::label_with_id<ref$<str$> > @ 0x22357a -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::frame<ref$<str$> > @ 0x223801 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::image<ref$<str$> > @ 0x223a77 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::image<u64> @ 0x223bc7 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::impl$20::template_with_id<ref$<str$> > @ 0x223e4a -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$53::pill_button<ref$<str$> > @ 0x223f9b -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> > @ 0x2242ae -> 0x48dd60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x224624 -> 0x48dd60 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > > > >
// 信息 ID: f2a792547e522e718cac6a68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["glam::u32::uvec2::UVec2", "3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<glam::u32::uvec2::UVec2,3>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<glam::u32::uvec2::UVec2,3> >"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_unindexed @ 0x9fa215 -> 0xbc8e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec2::UVec2> > > >
// 信息 ID: 61d9ca17808f68e717fe05eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec2::UVec2"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec2::UVec2>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<glam::u32::uvec2::UVec2> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<glam::u32::uvec2::UVec2> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x9f9785 -> 0xbc92c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 信息 ID: 3586ad5f09bd416509ffec6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<glam::u32::uvec3::UVec3> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<glam::u32::uvec3::UVec3> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x9f9749 -> 0xbc7470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<std::time::SystemTime> > >
// 信息 ID: 90d1807cbf11f60993e8d482
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::time::SystemTime"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<std::time::SystemTime>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<std::time::SystemTime> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x17ce417 -> 0xbc8770 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x17ce438 -> 0xbc8770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<u64> > >
// 信息 ID: 5ea386228cc6a421254bdd79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["u64"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<u64>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<u64> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x17ce417 -> 0xbc8770 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x17ce438 -> 0xbc8770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,u64> >
// 信息 ID: 0ce4ee5e2d231c26f358250d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u64", "u64"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,u64>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 437 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186dfa -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186ee7 -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186faf -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x187079 -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x2220a5 -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::child_with_content<tiny_glade::gui::gui_fog::FogSliderContent,ref$<str$>,tiny_glade::gui::gui_fog::fill_slider::closure_env$0> @ 0x2226a1 -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::child<alloc::string::String> @ 0x222f90 -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::tab_switcher<ref$<str$> > @ 0x22331d -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::label_with_id<ref$<str$> > @ 0x22358f -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::frame<ref$<str$> > @ 0x223816 -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::image<ref$<str$> > @ 0x223a8f -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::image<u64> @ 0x223bdf -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::impl$20::template_with_id<ref$<str$> > @ 0x223e5b -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$53::pill_button<ref$<str$> > @ 0x223fb0 -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> > @ 0x2242c6 -> 0x48def0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x224635 -> 0x48def0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<usize,u8> >
// 信息 ID: 2418a765d71d325eac22f7ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["usize", "u8"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<usize,u8>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<system_decorator::decorator_resources::DecoratorHandleName> @ 0x14b84d9 -> 0x14bbe70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::cartesian_to_polar
// 信息 ID: 5044d4644aa863637beacc37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb622bc -> 0xde46d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb622dc -> 0xde46d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb6259a -> 0xde46d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb6294b -> 0xde46d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb62ccb -> 0xde46d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_wall_transform_gizmos::wall_rotation_gizmo @ 0x1aba82c -> 0xde46d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::color_menu::color_menu @ 0x217c4a5 -> 0xde46d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_closest_u_from_pos; module=763, inline_record=5288; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::cos_angle_between
// 信息 ID: 85c4a3d59a98967e81408a05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::find_corners; module=3152, inline_record=676; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::find_corners; module=3152, inline_record=2076; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle; module=3161, inline_record=1764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_line_point
// 信息 ID: b7a1bf36803b6af02febe874
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=561, inline_record=81480; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=89840; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3167, inline_record=3476; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3167, inline_record=4340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_segment_point
// 信息 ID: 84b1e68f90387a06cc1e6003
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=570, inline_record=74312; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=764, inline_record=33836; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::is_on_right_side<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1699, inline_record=52536; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=4165, inline_record=6212; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$2>; module=4746, inline_record=1824; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>; module=4746, inline_record=4212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_squared_segment_point
// 信息 ID: cacdfacfa41f2fa619d7e2c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions; module=4054, inline_record=27220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::fit
// 信息 ID: 312e6ce3f72970ac700ed3ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311e40 -> 0xde5b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311c20 -> 0xde5b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x3114aa -> 0xde5b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x3111d7 -> 0xde5b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311293 -> 0xde5b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::remap_hour_to_fmod_hour @ 0xad9d04 -> 0xde5b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect
// 信息 ID: 88b698c9af3b1708e471a58a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::intersect::Triangle::intersect_triangle @ 0x147fe28 -> 0x24eb4c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::golden_spiral
// 信息 ID: c9794cbfe8f0092bd22c89eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::spawn_ducks::spawn_ducks @ 0x116f499 -> 0xde3e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x292bf16 -> 0xde3e00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::dot
// 信息 ID: 81828725e357994c660a465f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 42 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos; module=550, inline_record=12980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos; module=550, inline_record=13036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=561, inline_record=81620; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims; module=567, inline_record=41684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims; module=567, inline_record=41756; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=89980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::closest_segment_index; module=580, inline_record=27404; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::closest_segment_index; module=580, inline_record=27460; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::length
// 信息 ID: 99255ba5058e49eb468ae941
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 894525e2ee643a10ae49e3ac: glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 引用方: 原程序直接转移 0 条；内联宿主 34 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=157, inline_record=24132; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=157, inline_record=24780; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=157, inline_record=25084; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=157, inline_record=25712; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=561, inline_record=81936; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=88888; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=90296; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=584, inline_record=31120; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::length_squared
// 信息 ID: a836053876d534e031e92513
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions; module=4054, inline_record=27868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::lerp
// 信息 ID: d962915299397195c32a1bb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 80 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::Curve::try_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=157, inline_record=29904; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=17340; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=17616; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=41688; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=41964; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=405, inline_record=9432; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited; module=491, inline_record=4792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=561, inline_record=81680; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::normalize
// 信息 ID: ea29affa4f7db552f90e042c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones; module=468, inline_record=1228; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones; module=468, inline_record=1736; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones; module=468, inline_record=2200; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; module=551, inline_record=75912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers; module=557, inline_record=29580; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ivy::wall_coord::WallCoord::into_world_space; module=577, inline_record=74392; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal; module=580, inline_record=28520; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=580, inline_record=38396; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::dot
// 信息 ID: c93cafc495744a0bfac16e0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6736; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6800; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_stairs_viz::insert_point_ui; module=434, inline_record=3064; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_stairs_viz::insert_point_ui; module=434, inline_record=3120; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=436, inline_record=11792; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=436, inline_record=11856; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=570, inline_record=74628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=570, inline_record=74700; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::length
// 信息 ID: adb3256717b0b7d57b5f8950
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5ac66223e7f01d6a6eceffcb: glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)
// 引用方: 原程序直接转移 0 条；内联宿主 32 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=157, inline_record=5048; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=157, inline_record=5728; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=157, inline_record=6064; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=157, inline_record=6708; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>,core::slice::iter::Iter<glam::f32::vec3::Vec3>,utils::curve::impl$4::calculate_points_u_and_length::closure_env$0<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,utils::curve::impl$4::calculate_points_u_and_length::closure_env$0<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D> > > > >; module=228, inline_record=55832; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=570, inline_record=75240; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample_remap<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)>; module=1459, inline_record=17676; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample_remap<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)>; module=1459, inline_record=18356; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::length_squared
// 信息 ID: 3144e2b2939e12590d1bb277
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3167, inline_record=5148; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3167, inline_record=5320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::lerp
// 信息 ID: 61f6295693fc999bb33522e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 60 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6888; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets; module=420, inline_record=24648; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs; module=434, inline_record=52632; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=436, inline_record=11952; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=7436; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=10772; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=11476; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=14172; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::normalize
// 信息 ID: f3aaddda777540a459b21a77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks; module=416, inline_record=13896; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=20248; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=12008; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=12904; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=17048; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=17960; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=478, inline_record=30624; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$3::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$3::recompute_quads::closure_env$0> > > >; module=557, inline_record=70776; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::to_2d
// 信息 ID: eac556b75da2bae6cfde6546
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$21::clone
// 信息 ID: 82340df65266bebac1e638ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::trap_doors::generate_trap_doors; module=447, inline_record=9180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$2::length
// 信息 ID: ee795f1f823d107bf8702102
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > >; module=196, inline_record=53876; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >; module=196, inline_record=55000; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> > >; module=1478, inline_record=8560; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=14936; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1491, inline_record=3492; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1491, inline_record=4616; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1491, inline_record=5300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$2::lerp
// 信息 ID: ae0ddcd25ab5d21f4ddc234f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::maintain_lantern_lights; module=431, inline_record=32248; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::brush_preview::brush_preview; module=507, inline_record=19336; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::time_of_day::snapshot_lerp; module=571, inline_record=110788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$4::assign_auto_resize<enum2$<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex> > >
// 信息 ID: 52262fb3a63f21470e20efe0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["country_core::asset_libraries::texture_library::DescriptorIndex"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex>"]; 字段布局 = unknown。
// [类型关系] utils::impl$4::assign_auto_resize -> ["enum2$<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index @ 0xa15103 -> 0x15f9b00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$4::assign_auto_resize<tg_text::AtlasAllocUsage>
// 信息 ID: 1dd1fff07498ada80ae7659a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::impl$4::assign_auto_resize -> ["tg_text::AtlasAllocUsage"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextRenderer::get_glyph @ 0xcb29f8 -> 0x1cbff30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$5::intersects
// 信息 ID: 227da4fd83ef7b8111af6692
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143d1bf -> 0x199edb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x1458c40 -> 0x199edb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145a62e -> 0x199edb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145d02a -> 0x199edb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145cbd2 -> 0x199edb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$6::intersects
// 信息 ID: 3e1bc9e178a227b273962c99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x9290c8 -> 0x199ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x928d04 -> 0x199ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x92913a -> 0x199ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5b301 -> 0x199ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x21b9c95 -> 0x199ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bfe9a -> 0x199ed90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d; module=764, inline_record=46144; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$9::iterator_slice_range
// 信息 ID: 7df4c0c86897b0bb34591049
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1460, inline_record=2324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::inverse_lerp
// 信息 ID: 7340926bf9b8f4371ca7c3e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5ac66223e7f01d6a6eceffcb: glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)
// [泛型宿主/闭包/直接引用方参数候选] 894525e2ee643a10ae49e3ac: glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 引用方: 原程序直接转移 134 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_weather_slots::closure$2 @ 0x3104f2 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_weather_slots::closure$2 @ 0x31051b -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D> @ 0x4a35a5 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > @ 0x4a3b95 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveUBoundless<utils::curve::CurveCoordSemantic2D> > | utils::curve::Curve::get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > @ 0x4a3d75 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> @ 0x4a4b0d -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_slider @ 0x673497 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x679c68 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x7fc32f -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x805603 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x8056a1 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t @ 0x80c35c -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81e991 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x8408db -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x854827 -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs::closure$12 @ 0x85813c -> 0xde3df0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::linearstep; module=754, inline_record=532; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=9304; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >; module=2178, inline_record=908; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::lerp
// 信息 ID: 41a1786b659fbe6d7fe98f98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9e286f0216e1fbaff1175eba: array$<glam::f32::sse2::vec3a::Vec3A,32> (*)()
// 引用方: 原程序直接转移 55 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> @ 0x1ab5f9 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> @ 0x1ab60d -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> @ 0x1ab64c -> 0xde5ba0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3526f2 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x670ca0 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::trim_rows::impl$0::above_to_below @ 0x8426e3 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::trim_rows::impl$0::below_to_above @ 0x8427be -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::insert_point_ui @ 0x84e0e8 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape @ 0x89357d -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings @ 0x8b87ce -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings @ 0x8b8820 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings @ 0x8b8921 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x8e4813 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall @ 0x9ccfaf -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall @ 0x9ccfe8 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::main_camera::camera_snapshot @ 0xa20975 -> 0xde5ba0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::time_continuous_lerp; module=754, inline_record=11676; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_mut; module=754, inline_record=13340; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec2; module=754, inline_record=15248; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec2; module=754, inline_record=15276; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=754, inline_record=15512; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=754, inline_record=15568; 不是独立调用边。
// [PDB 内联宿主] utils::fit; module=754, inline_record=19324; 不是独立调用边。
// [PDB 内联宿主] utils::wall_space::WallSpaceCoord::from_curve_internal_coord; module=756, inline_record=5660; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::linearstep
// 信息 ID: f9cee5ca0f487be007937569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x670c87 -> 0xde3c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::mouse_ss_to_cs
// 信息 ID: aaadadb62a2a46ecfd1479cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0> @ 0xab276f -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0> @ 0xab28e9 -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4> @ 0xab2ae8 -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4> @ 0xab2bb1 -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_raise_shape::impl_raise_shape @ 0xb5222c -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line @ 0xb557ea -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7ce72 -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7dc84 -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space @ 0xbe9e1a -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode @ 0xbeb3bc -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x1abd6d5 -> 0xde3f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x1abd9e0 -> 0xde3f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::pack_2x16f_uint
// 信息 ID: 81f18a4ecf8d47f3d2d01004
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x9b7c07 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor> @ 0x167e0d1 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor> @ 0x167d92a -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x20b49bf -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_lantern_emissive @ 0x25b6ec3 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_lantern_emissive @ 0x25b6ed5 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x25ba601 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x25ba612 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x25ba623 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x25ba631 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_solid_vertex_color_bricks @ 0x25bbd6b -> 0xde41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::tree_animation::handplaced_tree_animation @ 0x286eef5 -> 0xde41b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::pack_srgba_u32
// 信息 ID: 76e8ee6911ac4549ed35f760
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x72b352 -> 0xde3fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x2321b62 -> 0xde3fc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::perspective_lerp
// 信息 ID: bf5a6f83448b420ca7d489aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray; module=585, inline_record=6440; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::get_closest_point_u_to_cursor; module=585, inline_record=29276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::polar_to_cartesian
// 信息 ID: 1875e4fdd514027934474374
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb6264d -> 0xde4720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_wall_transform_gizmos::wall_rotation_gizmo @ 0x1abaadc -> 0xde4720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::color_menu::color_menu @ 0x217c4e6 -> 0xde4720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::project_point_onto_line
// 信息 ID: baf448ac9fb781097566c209
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=436, inline_record=11516; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=561, inline_record=81508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims; module=567, inline_record=41512; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=89868; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3167, inline_record=3516; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3167, inline_record=4376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::project_point_onto_segment
// 信息 ID: a37ed12053f64e91699093ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6472; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=570, inline_record=74352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment; module=580, inline_record=31108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=580, inline_record=34088; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=764, inline_record=33896; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::is_on_right_side<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1699, inline_record=52628; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions; module=4054, inline_record=27320; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions; module=4054, inline_record=28196; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::radical_inverse
// 信息 ID: 91605bd4115abe0d9c30a9e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<u64> @ 0x5577a9 -> 0xde4420 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<u64> @ 0x5577c0 -> 0xde4420 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<u64> @ 0x5577cf -> 0xde4420 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> > @ 0x557c11 -> 0xde4420 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> > @ 0x557c28 -> 0xde4420 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> > @ 0x557c37 -> 0xde4420 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0xa83fb8 -> 0xde4420 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$3::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$3::default::closure_env$0> > > >; module=3631, inline_record=1192; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$3::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$3::default::closure_env$0> > > >; module=3631, inline_record=1240; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$3::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$3::default::closure_env$0> > > >; module=3631, inline_record=1372; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$3::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$3::default::closure_env$0> > > >; module=3631, inline_record=1420; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_orthogonal_quat
// 信息 ID: 08fb728cfd0a87102495f2d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks @ 0x81b65a -> 0xde51f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets @ 0x832618 -> 0xde51f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits
// 信息 ID: e79ede6e06cce058f7d7f6a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 29 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81eceb -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81f310 -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows @ 0x8419ce -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::construct_stone_stair_steps @ 0x8a093f -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings @ 0x8afe4b -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings @ 0x8b0feb -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railing_tips @ 0x8b2181 -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x8bbc02 -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141b41a -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e3c9 -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0x142eddc -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x143668b -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1438d34 -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e2e17 -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e31bc -> 0xde3e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x14e5a13 -> 0xde3e60 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits_into
// 信息 ID: 6da668776aef6eb5b730840a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true> @ 0x1f03c8 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67b0d4 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::random_splits @ 0xde3f01 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1444dea -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1447188 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3> @ 0x1449928 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x144d120 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1450999 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> @ 0x1454180 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0x1481efb -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x14839ef -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0x1485fb0 -> 0xde4780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x28d1089 -> 0xde4780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits_into::closure$0
// 信息 ID: 6ec604f786f101148194e328
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::random_splits_into", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::random_splits_into; module=754, inline_record=9080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rotation_from_normal
// 信息 ID: dcb5ec9b800e052e6d3a609c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 23 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0xb561fd -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0xb561be -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget @ 0xb9f473 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::impl$1::signifier::closure$0 @ 0xba06be -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow @ 0xc0ad06 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow @ 0xc0af51 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow @ 0xc0b099 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x143a56e -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143d5e8 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x148b70a -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e4044 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> @ 0x14e431c -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > > @ 0x14efcdd -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::create_ui_widget @ 0x1f1e576 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length::closure$1 @ 0x1f20bc7 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f472d3 -> 0xde4ba0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color
// 信息 ID: 85165330d61ed8a1f39c5b63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$3::render @ 0x722e22 -> 0xde58d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$6::render @ 0x723903 -> 0xde58d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextRenderer::rect @ 0xcb21bb -> 0xde58d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$6::render @ 0x232475c -> 0xde58d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color::scale_linear
// 信息 ID: bb8549a61a9d50ca86521a65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::scale_u32_packed_srgba_color; module=754, inline_record=17416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color_premultiplied::scale_linear
// 信息 ID: 2feb2d6bd427ea475d45ec2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color_premultiplied::scale_srgb
// 信息 ID: e7ec91bd27f63e7be32637d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::slice_iter
// 信息 ID: b5cd7f05589200b8ed3f80de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1460, inline_record=2300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::smoothstep
// 信息 ID: a19a6e0fc8477f70f9cee657
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8e6acdd33b13c966488eca7: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 0 条；内联宿主 49 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)>; module=94, inline_record=91556; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)>; module=94, inline_record=141008; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::rollercoaster::rollercoaster_update; module=97, inline_record=17632; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=403, inline_record=18336; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals; module=417, inline_record=16292; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=75968; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::construct_stone_stair_steps; module=459, inline_record=35668; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::calculate_distortion_from_structural_graph; module=461, inline_record=47868; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::srgb_eotf
// 信息 ID: 4896f8982e6f2d2707d961bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::srgb_oetf
// 信息 ID: 7456d77e5b71637b729e1ed6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::pack_srgba_u32; module=754, inline_record=4156; 不是独立调用边。
// [PDB 内联宿主] utils::pack_srgba_u32; module=754, inline_record=4220; 不是独立调用边。
// [PDB 内联宿主] utils::pack_srgba_u32; module=754, inline_record=4284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::t_of_point_projection_onto_segment
// 信息 ID: a51788e310e7dde01729169b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 30 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6512; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_stairs_viz::insert_point_ui; module=434, inline_record=2852; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=436, inline_record=11552; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos; module=550, inline_record=12840; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=561, inline_record=81536; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims; module=567, inline_record=41540; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=89896; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=570, inline_record=74392; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp
// 信息 ID: 45482f0d17c7708be9d036fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x352604 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x352727 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x809ec9 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x84ef75 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x84f683 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x84fdd6 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x852286 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x852b1b -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x852f3d -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x853312 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8536fa -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x853dad -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::window_lights::window_lights @ 0x871e36 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain @ 0xabfb26 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xadae1b -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls @ 0xb167e2 -> 0xde4f80 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::time_continuous_lerp_mut; module=754, inline_record=13256; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=754, inline_record=15428; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=754, inline_record=15544; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_factor
// 信息 ID: 4ee2ee9420204a734acb19c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_interpolate_and_compare @ 0x88f6ef -> 0xde58a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::time_continuous_lerp; module=754, inline_record=11620; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_mut; module=754, inline_record=13284; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec2; module=754, inline_record=15192; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=754, inline_record=15456; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_mut
// 信息 ID: ee1a43b730b113bd9caf1725
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$19::render @ 0x7242d6 -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0xad90af -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0xad90ce -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0xad90ef -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0xad9179 -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::thumbnail_scroll::impl$3::render @ 0x22d0d4d -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::thumbnail_scroll::impl$3::render @ 0x22d0f06 -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::thumbnail_scroll::impl$3::render @ 0x22d0f49 -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::thumbnail_scroll::impl$3::render @ 0x22d100f -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::prepare_layout @ 0x22f57fa -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::prepare_layout @ 0x22f581c -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::prepare_layout @ 0x22f583d -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items<toolbar::menu::mod_checklist::replacement_mod_checklist::closure_env$0> @ 0x231a8ad -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_workshop_tab::list_slot::closure_env$3> @ 0x231c165 -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_mine_tab::list_slot::closure_env$1> @ 0x231cbf5 -> 0xde52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::missing_mods::ui::closure$3::closure_env$0> @ 0x231d85e -> 0xde52e0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_vec2
// 信息 ID: 1f4f8411821026ee21b10d56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5be8c -> 0xde5580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5bdd0 -> 0xde5580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_vec3
// 信息 ID: b394912b187be4addf4b8501
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8537d4 -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8533fd -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x85302a -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x852c88 -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height @ 0xbd1f3a -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2170323 -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x23b1191 -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x2939227 -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x29395dc -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x293978b -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x2939910 -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x2939f39 -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::random_vel_change @ 0x2b28c59 -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0> @ 0x2b2972d -> 0xde5610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x2b2a589 -> 0xde5610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_circle
// 信息 ID: 8a7a9eb2305757939897de6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::spawn_ducks::spawn_ducks @ 0x116f57e -> 0xde50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::spawn_ducks::despawn_ducks @ 0x116fe45 -> 0xde50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::spawn_ducks::despawn_ducks @ 0x1170274 -> 0xde50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x292c010 -> 0xde50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x292c857 -> 0xde50e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x292d366 -> 0xde50e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_cone
// 信息 ID: 42c5938e31c67380ef52f825
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::boids::Boid::random_vel_change @ 0x2b28be5 -> 0xde4a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_disk
// 信息 ID: fb8bedc9762e38f6d1a435a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0xa84013 -> 0xde4b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_behavior::update_sheep_goals @ 0x2929ca2 -> 0xde4b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_behavior::update_sheep_goals @ 0x2929f54 -> 0xde4b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::update_duck_goals @ 0x292af71 -> 0xde4b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_sphere
// 信息 ID: c56176c25b1fbd4ecc6ce62e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::butterfly::update_butterfly_goals @ 0x2291ea4 -> 0xde5140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::firefly::update_firefly_goals @ 0x2930dfc -> 0xde5140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x2945dee -> 0xde5140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_projective_rotation_bad
// 信息 ID: 05e1a4fd88c15b021687bd24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x877ee0 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x883933 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x8839c3 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x883f47 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x883f5f -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x8bdf88 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c26fb -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x8c6463 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x8c955e -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8cb898 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8cb94a -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8cba44 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls @ 0xb169a8 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_splitter::wall_splitter @ 0xbb5ef4 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::move_ops::show_decorator_remove_signifier @ 0x14d9912 -> 0xde5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x1f389cf -> 0xde5680 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_projective_rotation_from_forward
// 信息 ID: 63ec903f68b51046a30e2bfc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x8622ff -> 0xde5930 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x867911 -> 0xde5930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_bad
// 信息 ID: 1f6e2f45d7323b07162b334e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6111dd30703fa00f801674f1: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 60 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x31b596 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x31d7b1 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::systems::construct_gates @ 0x801dc3 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81f410 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81f6ec -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs::closure$10 @ 0x857d27 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x8596e3 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85a49e -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85b001 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85b537 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::show_signifiers @ 0x86a42e -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_collision_as_bbx @ 0x877154 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x8bbe83 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8dd6ae -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8dd57f -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8dd1a2 -> 0xde44c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::xz_projective_rotation_bad; module=754, inline_record=15752; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_from_forward
// 信息 ID: 26e2dc8982edcd4ed7d55b94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8dde62 -> 0xde5340 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::camera::impl$4::xz_projective_rotation @ 0xdea714 -> 0xde5340 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::xz_projective_rotation_from_forward; module=754, inline_record=17600; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_mat3_bad
// 信息 ID: e24e649abf9d93b197a5afa3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::construct_stone_stair_steps @ 0x8a0c4a -> 0xde4fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings @ 0x8b106d -> 0xde4fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railing_tips @ 0x8b2017 -> 0xde4fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14377ee -> 0xde4fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14393e7 -> 0xde4fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_mat3a_bad
// 信息 ID: 61beda365065d3d941547565
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::xz_rotation_bad; module=754, inline_record=5596; 不是独立调用边。
// [PDB 内联宿主] utils::xz_projective_rotation_bad; module=754, inline_record=15776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_mat3a_from_forward
// 信息 ID: dec2ccd416bb66633faa348c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::xz_rotation_from_forward; module=754, inline_record=13496; 不是独立调用边。
// [PDB 内联宿主] utils::xz_projective_rotation_from_forward; module=754, inline_record=17632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
