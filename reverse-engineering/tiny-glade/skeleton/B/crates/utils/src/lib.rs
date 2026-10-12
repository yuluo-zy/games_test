// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b651f1f10fc8f80000183fd76376f226c10e76bd17f7d349c4a9b892a30b2bde
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 60, 104, 160, 184, 187, 246, 390, 394, 397, 401, 406, 407, 408, 415, 417, 419, 430, 432, 434, 437, 441, 443, 444, 453, 458, 465, 476, 481, 497, 498, 499, 512, 513, 515, 519, 526, 527, 551, 553, 556, 558, 561, 564, 565, 566, 567, 576, 578, 579, 583, 759, 761, 767, 768, 1177, 1184, 1407, 1415, 1416, 1417, 1431, 1434, 1435, 1436, 1437, 1443, 1449, 1451, 1454, 1476, 1477, 1479, 1480, 1482, 1503, 1506, 1510, 1522, 1538, 1540, 1543, 1546, 1552, 1570, 1576, 1585, 1606, 1607, 1610, 1620, 1705, 1778, 2073, 2076, 2077, 2081, 2245, 2603, 2629, 2747, 2842, 2843, 3027, 3030, 3031, 3036, 3087, 3157, 3375, 3377, 3432, 3461, 3474, 3475, 3656, 3707, 3740, 3744, 3753, 3810, 3811, 3814, 3877, 3886, 3899, 4071, 4333, 4438
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
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
// 首条位置: {"module":3377,"record_offset":252,"rva":35129072,"va":5403838192}
// 编译实例 RVAs: 0x21806f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1421806f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403838192,"raw_hex":"e85dffffff","size":5,"soft_fail":false,"target_va":5403838192,"va":5403838350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8008866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403838192,"raw_hex":"e8a2347a00","size":5,"soft_fail":false,"target_va":5411847168,"va":5403838297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7846055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403838192,"raw_hex":"e8a7b87700","size":5,"soft_fail":false,"target_va":5411684768,"va":5403838708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::formatting::format_component
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4122130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403838192,"raw_hex":"e812e63e00","size":5,"soft_fail":false,"target_va":5407960704,"va":5403838569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3377,"record_offset":856,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3377,"record_offset":1232,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_direct] time::formatting::formattable::impl$9::format_into
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/formatting/formattable.rs
//     evidence={"module":3377,"record_offset":884,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":12641,"source_line":88,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_direct] time::formatting::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/formatting/mod.rs
//     evidence={"module":3377,"record_offset":444,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":11021,"source_line":44,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3377,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":252,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3377,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":252,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3377,"record_offset":692,"parent_record_offset":656,"ancestor_record_offset":252,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3377,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":252,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3377,"record_offset":556,"parent_record_offset":528,"ancestor_record_offset":252,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3377,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":252,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3377,"record_offset":808,"parent_record_offset":556,"ancestor_record_offset":252,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3377,"record_offset":596,"parent_record_offset":556,"ancestor_record_offset":252,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3377,"record_offset":528,"parent_record_offset":500,"ancestor_record_offset":252,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3377,"record_offset":772,"parent_record_offset":556,"ancestor_record_offset":252,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3377,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":252,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3377,"record_offset":984,"parent_record_offset":932,"ancestor_record_offset":252,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3377,"record_offset":1104,"parent_record_offset":884,"ancestor_record_offset":252,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3377,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":252,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3377,"record_offset":932,"parent_record_offset":884,"ancestor_record_offset":252,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3377,"record_offset":1016,"parent_record_offset":932,"ancestor_record_offset":252,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":3377,"record_offset":472,"parent_record_offset":444,"ancestor_record_offset":252,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/03377.jsonl.gz"}
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
// 首条位置: {"module":3030,"record_offset":252,"rva":33422720,"va":5402131840}
// 编译实例 RVAs: 0x1fdfd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::determinant
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13796349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402131840,"raw_hex":"e8037c2dff","size":5,"soft_fail":false,"target_va":5388336208,"va":5402132552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::triangle_triangle::cross_intersect::cross_intersect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4423716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402131840,"raw_hex":"e824804300","size":5,"soft_fail":false,"target_va":5406556576,"va":5402132855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3030,"record_offset":952,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::is_tri_degenerated
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3030,"record_offset":420,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":106138,"source_line":7,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3030,"record_offset":676,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":106139,"source_line":20,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3030,"record_offset":484,"parent_record_offset":460,"ancestor_record_offset":252,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":3030,"record_offset":708,"parent_record_offset":676,"ancestor_record_offset":252,"inlinee":82955,"source_line":120,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":3030,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":252,"inlinee":106140,"source_line":77,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3030,"record_offset":516,"parent_record_offset":420,"ancestor_record_offset":252,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3030,"record_offset":460,"parent_record_offset":420,"ancestor_record_offset":252,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs
//     evidence={"module":3030,"record_offset":980,"parent_record_offset":952,"ancestor_record_offset":252,"inlinee":106142,"source_line":39,"archive":"_pdb/B/modules/03030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::length
// 记录形式: inline_instance, inlinee_declaration; instances=47
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x255F
// 首条位置: {"module":60,"inlinee":9567}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":60,"record_offset":19876,"parent_record_offset":19852,"ancestor_record_offset":19852,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":60,"record_offset":19900,"parent_record_offset":19876,"ancestor_record_offset":19852,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":60,"record_offset":19932,"parent_record_offset":19900,"ancestor_record_offset":19852,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::length_squared
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17EE4
// 首条位置: {"module":2629,"inlinee":98020}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":32688,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32736,"parent_record_offset":32712,"ancestor_record_offset":32688,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::normalize
// 记录形式: inline_instance, inlinee_declaration; instances=30
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x632B
// 首条位置: {"module":441,"inlinee":25387}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":441,"record_offset":23780,"parent_record_offset":23752,"ancestor_record_offset":23752,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":46556,"parent_record_offset":46428,"ancestor_record_offset":46380,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":441,"record_offset":23872,"parent_record_offset":23836,"ancestor_record_offset":23752,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":441,"record_offset":23836,"parent_record_offset":23808,"ancestor_record_offset":23752,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":441,"record_offset":23808,"parent_record_offset":23780,"ancestor_record_offset":23752,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3461,"record_offset":7104,"parent_record_offset":7076,"ancestor_record_offset":7048,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03461.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":441,"record_offset":23908,"parent_record_offset":23872,"ancestor_record_offset":23752,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=110
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2568
// 首条位置: {"module":60,"inlinee":9576}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=67
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":60,"record_offset":25752,"parent_record_offset":25720,"ancestor_record_offset":25720,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=71
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":60,"record_offset":25892,"parent_record_offset":25868,"ancestor_record_offset":25720,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=164
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":60,"record_offset":25808,"parent_record_offset":25784,"ancestor_record_offset":25720,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=67
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":60,"record_offset":25784,"parent_record_offset":25752,"ancestor_record_offset":25720,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=59
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":60,"record_offset":25868,"parent_record_offset":25752,"ancestor_record_offset":25720,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::dot
// 记录形式: inline_instance, inlinee_declaration; instances=49
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72D4
// 首条位置: {"module":512,"inlinee":29396}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45260,"parent_record_offset":45236,"ancestor_record_offset":45236,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::length
// 记录形式: inline_instance, inlinee_declaration; instances=50
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2547
// 首条位置: {"module":60,"inlinee":9543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":60,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":5900,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":60,"record_offset":5948,"parent_record_offset":5924,"ancestor_record_offset":5900,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":60,"record_offset":5980,"parent_record_offset":5948,"ancestor_record_offset":5900,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00060.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::length_squared
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19EB0
// 首条位置: {"module":3036,"inlinee":106160}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":5652,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":5652,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::normalize
// 记录形式: inline_instance, inlinee_declaration; instances=30
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6074
// 首条位置: {"module":406,"inlinee":24692}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":406,"record_offset":19172,"parent_record_offset":19148,"ancestor_record_offset":19148,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":406,"record_offset":19328,"parent_record_offset":19196,"ancestor_record_offset":19148,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":406,"record_offset":19260,"parent_record_offset":19228,"ancestor_record_offset":19148,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":406,"record_offset":19228,"parent_record_offset":19196,"ancestor_record_offset":19148,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":406,"record_offset":19196,"parent_record_offset":19172,"ancestor_record_offset":19148,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":406,"record_offset":19292,"parent_record_offset":19260,"ancestor_record_offset":19148,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00406.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::to_2d
// 记录形式: inlinee_declaration; instances=5
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11721
// 首条位置: {"module":1506,"inlinee":71457}
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
// 记录形式: inline_instance, inlinee_declaration; instances=92
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F7A
// 首条位置: {"module":390,"inlinee":24442}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=51
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20436,"parent_record_offset":20408,"ancestor_record_offset":20408,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=67
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20656,"parent_record_offset":20624,"ancestor_record_offset":20408,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=169
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20500,"parent_record_offset":20464,"ancestor_record_offset":20408,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=50
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20464,"parent_record_offset":20436,"ancestor_record_offset":20408,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20624,"parent_record_offset":20436,"ancestor_record_offset":20408,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::dot
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F7B
// 首条位置: {"module":390,"inlinee":24443}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20284,"parent_record_offset":20256,"ancestor_record_offset":20256,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$2::length
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x603C
// 首条位置: {"module":401,"inlinee":24636}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":401,"record_offset":14880,"parent_record_offset":14848,"ancestor_record_offset":14848,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$2::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60F3
// 首条位置: {"module":415,"inlinee":24819}
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
// 记录形式: inline_instance, inlinee_declaration; instances=52
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F79
// 首条位置: {"module":390,"inlinee":24441}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":20376,"parent_record_offset":20032,"ancestor_record_offset":20032,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45124,"parent_record_offset":45092,"ancestor_record_offset":45092,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20072,"parent_record_offset":20032,"ancestor_record_offset":20032,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=32
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":45236,"parent_record_offset":45092,"ancestor_record_offset":45092,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":390,"record_offset":20256,"parent_record_offset":20032,"ancestor_record_offset":20032,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20104,"parent_record_offset":20072,"ancestor_record_offset":20032,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45260,"parent_record_offset":45236,"ancestor_record_offset":45092,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20284,"parent_record_offset":20256,"ancestor_record_offset":20032,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_line_point
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EBB
// 首条位置: {"module":567,"inlinee":32443}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5636,"parent_record_offset":5236,"ancestor_record_offset":5236,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4612,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5692,"parent_record_offset":5236,"ancestor_record_offset":5236,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4720,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":9543,"source_line":89,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":5236,"inlinee":24908,"source_line":179,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":5236,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":5236,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":5236,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":5236,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":5236,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":5236,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":5236,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5568,"parent_record_offset":5460,"ancestor_record_offset":5236,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":5236,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":4052,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":4052,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4336,"parent_record_offset":4308,"ancestor_record_offset":4052,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4364,"parent_record_offset":4336,"ancestor_record_offset":4052,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4508,"parent_record_offset":4336,"ancestor_record_offset":4052,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4160,"parent_record_offset":4128,"ancestor_record_offset":4052,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":567,"record_offset":5772,"parent_record_offset":5740,"ancestor_record_offset":5236,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5376,"parent_record_offset":5292,"ancestor_record_offset":5236,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5436,"parent_record_offset":5264,"ancestor_record_offset":5236,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4248,"parent_record_offset":4128,"ancestor_record_offset":4052,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4308,"parent_record_offset":4092,"ancestor_record_offset":4052,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5292,"parent_record_offset":5264,"ancestor_record_offset":5236,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_segment_point
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7430
// 首条位置: {"module":519,"inlinee":29744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":30112,"parent_record_offset":29536,"ancestor_record_offset":29536,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":519,"record_offset":99296,"parent_record_offset":98532,"ancestor_record_offset":98532,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":30168,"parent_record_offset":29536,"ancestor_record_offset":29536,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":519,"record_offset":99388,"parent_record_offset":98532,"ancestor_record_offset":98532,"inlinee":9543,"source_line":89,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":519,"record_offset":98568,"parent_record_offset":98532,"ancestor_record_offset":98532,"inlinee":24440,"source_line":184,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":519,"record_offset":98960,"parent_record_offset":98568,"ancestor_record_offset":98532,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":519,"record_offset":98928,"parent_record_offset":98600,"ancestor_record_offset":98532,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":519,"record_offset":99224,"parent_record_offset":99192,"ancestor_record_offset":98532,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":519,"record_offset":99076,"parent_record_offset":99048,"ancestor_record_offset":98532,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":519,"record_offset":98664,"parent_record_offset":98636,"ancestor_record_offset":98532,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":29536,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":30192,"parent_record_offset":30168,"ancestor_record_offset":29536,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29936,"parent_record_offset":29912,"ancestor_record_offset":29536,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29960,"parent_record_offset":29936,"ancestor_record_offset":29536,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":30044,"parent_record_offset":29936,"ancestor_record_offset":29536,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29624,"parent_record_offset":29592,"ancestor_record_offset":29536,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":519,"record_offset":98840,"parent_record_offset":98816,"ancestor_record_offset":98532,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":519,"record_offset":99412,"parent_record_offset":99388,"ancestor_record_offset":98532,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":519,"record_offset":99020,"parent_record_offset":98992,"ancestor_record_offset":98532,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":519,"record_offset":99048,"parent_record_offset":99020,"ancestor_record_offset":98532,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":519,"record_offset":99192,"parent_record_offset":99020,"ancestor_record_offset":98532,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":519,"record_offset":98636,"parent_record_offset":98600,"ancestor_record_offset":98532,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":519,"record_offset":99468,"parent_record_offset":99436,"ancestor_record_offset":98532,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":29736,"parent_record_offset":29592,"ancestor_record_offset":29536,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":29912,"parent_record_offset":29564,"ancestor_record_offset":29536,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":519,"record_offset":98816,"parent_record_offset":98600,"ancestor_record_offset":98532,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":519,"record_offset":98992,"parent_record_offset":98568,"ancestor_record_offset":98532,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":519,"record_offset":98600,"parent_record_offset":98568,"ancestor_record_offset":98532,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_squared_segment_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17EE5
// 首条位置: {"module":3811,"inlinee":98021}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32076,"parent_record_offset":32040,"ancestor_record_offset":32040,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length_squared
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32688,"parent_record_offset":32040,"ancestor_record_offset":32040,"inlinee":98020,"source_line":63,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32140,"parent_record_offset":32040,"ancestor_record_offset":32040,"inlinee":24440,"source_line":184,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3811,"record_offset":32456,"parent_record_offset":32140,"ancestor_record_offset":32040,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3811,"record_offset":32424,"parent_record_offset":32168,"ancestor_record_offset":32040,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":32644,"parent_record_offset":32620,"ancestor_record_offset":32040,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":32560,"parent_record_offset":32536,"ancestor_record_offset":32040,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":32104,"parent_record_offset":32076,"ancestor_record_offset":32040,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32336,"parent_record_offset":32312,"ancestor_record_offset":32040,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":32040,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32512,"parent_record_offset":32488,"ancestor_record_offset":32040,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32536,"parent_record_offset":32512,"ancestor_record_offset":32040,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32620,"parent_record_offset":32512,"ancestor_record_offset":32040,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32200,"parent_record_offset":32168,"ancestor_record_offset":32040,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32312,"parent_record_offset":32168,"ancestor_record_offset":32040,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32488,"parent_record_offset":32140,"ancestor_record_offset":32040,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32168,"parent_record_offset":32140,"ancestor_record_offset":32040,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::project_point_onto_line
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x614C
// 首条位置: {"module":419,"inlinee":24908}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5436,"parent_record_offset":5264,"ancestor_record_offset":5264,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":419,"record_offset":15092,"parent_record_offset":14656,"ancestor_record_offset":14656,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":419,"record_offset":14692,"parent_record_offset":14656,"ancestor_record_offset":14656,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":419,"record_offset":15060,"parent_record_offset":14692,"ancestor_record_offset":14656,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":419,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":14656,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":419,"record_offset":15172,"parent_record_offset":15140,"ancestor_record_offset":14656,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":419,"record_offset":14776,"parent_record_offset":14740,"ancestor_record_offset":14656,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":5264,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":5264,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":5264,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5568,"parent_record_offset":5460,"ancestor_record_offset":5264,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":5264,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":419,"record_offset":14960,"parent_record_offset":14932,"ancestor_record_offset":14656,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":419,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":14656,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":419,"record_offset":15140,"parent_record_offset":15116,"ancestor_record_offset":14656,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":419,"record_offset":15288,"parent_record_offset":15116,"ancestor_record_offset":14656,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":419,"record_offset":14740,"parent_record_offset":14692,"ancestor_record_offset":14656,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5376,"parent_record_offset":5292,"ancestor_record_offset":5264,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":419,"record_offset":14932,"parent_record_offset":14692,"ancestor_record_offset":14656,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/00419.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::project_point_onto_segment
// 记录形式: inline_instance, inlinee_declaration; instances=25
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F78
// 首条位置: {"module":390,"inlinee":24440}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":20724,"parent_record_offset":19992,"ancestor_record_offset":19992,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":49188,"parent_record_offset":48956,"ancestor_record_offset":48956,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":390,"record_offset":20408,"parent_record_offset":19992,"ancestor_record_offset":19992,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":390,"record_offset":20032,"parent_record_offset":19992,"ancestor_record_offset":19992,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":20376,"parent_record_offset":20032,"ancestor_record_offset":19992,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20656,"parent_record_offset":20624,"ancestor_record_offset":19992,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20500,"parent_record_offset":20464,"ancestor_record_offset":19992,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20104,"parent_record_offset":20072,"ancestor_record_offset":19992,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49092,"parent_record_offset":49068,"ancestor_record_offset":48956,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49212,"parent_record_offset":49188,"ancestor_record_offset":48956,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49236,"parent_record_offset":49212,"ancestor_record_offset":48956,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49320,"parent_record_offset":49212,"ancestor_record_offset":48956,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49012,"parent_record_offset":48984,"ancestor_record_offset":48956,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20284,"parent_record_offset":20256,"ancestor_record_offset":19992,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20436,"parent_record_offset":20408,"ancestor_record_offset":19992,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20464,"parent_record_offset":20436,"ancestor_record_offset":19992,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20624,"parent_record_offset":20436,"ancestor_record_offset":19992,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20072,"parent_record_offset":20032,"ancestor_record_offset":19992,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":49068,"parent_record_offset":48984,"ancestor_record_offset":48956,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":390,"record_offset":20256,"parent_record_offset":20032,"ancestor_record_offset":19992,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/00390.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":904,"rva":13172816,"va":5381881936}
// 编译实例 RVAs: 0xc90050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1247421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381881936,"raw_hex":"e843f7ecff","size":5,"soft_fail":false,"target_va":5380634736,"va":5381882152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29802233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381881936,"raw_hex":"e8f9bec601","size":5,"soft_fail":false,"target_va":5411684768,"va":5381882530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t127356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381881936,"raw_hex":"e87cf10100","size":5,"soft_fail":false,"target_va":5382009728,"va":5381882367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":759,"record_offset":1724,"parent_record_offset":904,"ancestor_record_offset":904,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":759,"record_offset":1024,"parent_record_offset":904,"ancestor_record_offset":904,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":759,"record_offset":1696,"parent_record_offset":904,"ancestor_record_offset":904,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":904,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":904,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":904,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":1220,"parent_record_offset":1184,"ancestor_record_offset":904,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":904,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":904,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":1184,"parent_record_offset":1152,"ancestor_record_offset":904,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":759,"record_offset":1300,"parent_record_offset":1080,"ancestor_record_offset":904,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":759,"record_offset":1080,"parent_record_offset":1052,"ancestor_record_offset":904,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":759,"record_offset":1124,"parent_record_offset":1080,"ancestor_record_offset":904,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":759,"record_offset":1572,"parent_record_offset":1552,"ancestor_record_offset":904,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":759,"record_offset":1656,"parent_record_offset":1080,"ancestor_record_offset":904,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":759,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":904,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":759,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":904,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":759,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":904,"inlinee":9468,"source_line":258,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":759,"record_offset":1528,"parent_record_offset":1508,"ancestor_record_offset":904,"inlinee":9474,"source_line":263,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":759,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":904,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":759,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":904,"inlinee":9470,"source_line":40,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":759,"record_offset":2268,"parent_record_offset":2236,"ancestor_record_offset":904,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":759,"record_offset":2016,"parent_record_offset":1968,"ancestor_record_offset":904,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":759,"record_offset":2436,"parent_record_offset":2300,"ancestor_record_offset":904,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":759,"record_offset":2372,"parent_record_offset":2344,"ancestor_record_offset":904,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":759,"record_offset":2344,"parent_record_offset":2300,"ancestor_record_offset":904,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":759,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":904,"inlinee":10996,"source_line":853,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":759,"record_offset":2300,"parent_record_offset":2268,"ancestor_record_offset":904,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":759,"record_offset":1412,"parent_record_offset":1080,"ancestor_record_offset":904,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":759,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":904,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":759,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":904,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\try_trait.rs
//     evidence={"module":759,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":904,"inlinee":9472,"source_line":395,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":759,"record_offset":2588,"parent_record_offset":2300,"ancestor_record_offset":904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":759,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":904,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] utils::random_splits_into::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":2464,"parent_record_offset":2436,"ancestor_record_offset":904,"inlinee":44710,"source_line":211,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::random_splits_into::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEA6
// 首条位置: {"module":759,"inlinee":44710}
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
// 首条位置: {"module":759,"record_offset":2988,"rva":13173584,"va":5381882704}
// 编译实例 RVAs: 0xc90350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6801342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381882704,"raw_hex":"e8423898ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381882825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6801512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381882704,"raw_hex":"e8983798ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381883011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29658018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381882704,"raw_hex":"e8a28bc401","size":5,"soft_fail":false,"target_va":5411540963,"va":5381882940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29659133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381882704,"raw_hex":"e8fd8fc401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381882926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381882704,"raw_hex":"e856fcffff","size":5,"soft_fail":false,"target_va":5381881936,"va":5381882869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":759,"record_offset":3096,"parent_record_offset":2988,"ancestor_record_offset":2988,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":759,"record_offset":3484,"parent_record_offset":2988,"ancestor_record_offset":2988,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":759,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":2988,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":759,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":2988,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":759,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":2988,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":759,"record_offset":3380,"parent_record_offset":3208,"ancestor_record_offset":2988,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":759,"record_offset":3648,"parent_record_offset":3556,"ancestor_record_offset":2988,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":2988,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":2988,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":2988,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":2988,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":2988,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":2988,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":759,"record_offset":3240,"parent_record_offset":3208,"ancestor_record_offset":2988,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":759,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":2988,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":759,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":2988,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":759,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":2988,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":759,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":2988,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":759,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":2988,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":759,"record_offset":3616,"parent_record_offset":3584,"ancestor_record_offset":2988,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":759,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":2988,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::smoothstep
// 记录形式: inline_instance, inlinee_declaration; instances=73
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3551
// 首条位置: {"module":160,"inlinee":13649}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":160,"record_offset":10796,"parent_record_offset":10768,"ancestor_record_offset":10768,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::inverse_lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 257, 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEA7
// 首条位置: {"module":759,"inlinee":44711}
// 编译实例 RVAs: 0xc904a0
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
// 首条位置: {"module":759,"record_offset":3820,"rva":13173936,"va":5381883056}
// 编译实例 RVAs: 0xc904b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":759,"record_offset":3952,"parent_record_offset":3820,"ancestor_record_offset":3820,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] utils::inverse_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":3928,"parent_record_offset":3820,"ancestor_record_offset":3820,"inlinee":44711,"source_line":257,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=13
// 源码行: 266, 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEA9
// 首条位置: {"module":759,"inlinee":44713}
// 编译实例 RVAs: 0xc904d0
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
// IPI IDs: 0xAEAA
// 首条位置: {"module":759,"inlinee":44714}
// 编译实例 RVAs: 0xc904e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883104,"raw_hex":"e8b939c901","size":5,"soft_fail":false,"target_va":5411847872,"va":5381883138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":4244,"parent_record_offset":4072,"ancestor_record_offset":4072,"inlinee":44713,"source_line":266,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] utils::time_continuous_lerp_factor
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":4188,"parent_record_offset":4072,"ancestor_record_offset":4072,"inlinee":44712,"source_line":275,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":4072,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp_factor
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEA8
// 首条位置: {"module":759,"inlinee":44712}
// 编译实例 RVAs: 0xc90540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883200,"raw_hex":"e86c39c901","size":5,"soft_fail":false,"target_va":5411847872,"va":5381883215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":4188,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":4424,"rva":13174128,"va":5381883248}
// 编译实例 RVAs: 0xc90570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883248,"raw_hex":"e80a39c901","size":5,"soft_fail":false,"target_va":5411847872,"va":5381883313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":4600,"parent_record_offset":4424,"ancestor_record_offset":4424,"inlinee":44713,"source_line":266,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] utils::time_continuous_lerp_factor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":4544,"parent_record_offset":4424,"ancestor_record_offset":4424,"inlinee":44712,"source_line":275,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":4424,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":4660,"rva":13174272,"va":5381883392}
// 编译实例 RVAs: 0xc90600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883392,"raw_hex":"e89b38c901","size":5,"soft_fail":false,"target_va":5411847872,"va":5381883424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":4952,"parent_record_offset":4660,"ancestor_record_offset":4660,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] utils::time_continuous_lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":4780,"parent_record_offset":4660,"ancestor_record_offset":4660,"inlinee":44714,"source_line":270,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":4660,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] utils::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":4864,"parent_record_offset":4780,"ancestor_record_offset":4660,"inlinee":44713,"source_line":266,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] utils::time_continuous_lerp_factor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":4808,"parent_record_offset":4780,"ancestor_record_offset":4660,"inlinee":44712,"source_line":275,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":4984,"rva":13174384,"va":5381883504}
// 编译实例 RVAs: 0xc90670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883504,"raw_hex":"e82438c901","size":5,"soft_fail":false,"target_va":5411847872,"va":5381883543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::time_continuous_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":5100,"parent_record_offset":4984,"ancestor_record_offset":4984,"inlinee":44714,"source_line":270,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":4984,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] utils::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":5184,"parent_record_offset":5100,"ancestor_record_offset":4984,"inlinee":44713,"source_line":266,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] utils::time_continuous_lerp_factor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":5128,"parent_record_offset":5100,"ancestor_record_offset":4984,"inlinee":44712,"source_line":275,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::perspective_lerp
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E85
// 首条位置: {"module":565,"inlinee":32389}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":565,"record_offset":56648,"parent_record_offset":56624,"ancestor_record_offset":56624,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00565.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":5220,"rva":13174480,"va":5381883600}
// 编译实例 RVAs: 0xc906d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":5412,"parent_record_offset":5220,"ancestor_record_offset":5220,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":5336,"parent_record_offset":5220,"ancestor_record_offset":5220,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":5492,"parent_record_offset":5220,"ancestor_record_offset":5220,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":759,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":5220,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":759,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":5220,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":759,"record_offset":5520,"parent_record_offset":5492,"ancestor_record_offset":5220,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":5576,"rva":13174528,"va":5381883648}
// 编译实例 RVAs: 0xc90700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883648,"raw_hex":"e84937c901","size":5,"soft_fail":false,"target_va":5411847808,"va":5381883698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883648,"raw_hex":"e88838c901","size":5,"soft_fail":false,"target_va":5411848144,"va":5381883715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":5724,"parent_record_offset":5576,"ancestor_record_offset":5576,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":5752,"parent_record_offset":5576,"ancestor_record_offset":5576,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":5696,"parent_record_offset":5576,"ancestor_record_offset":5576,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":5784,"rva":13174640,"va":5381883760}
// 编译实例 RVAs: 0xc90770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883760,"raw_hex":"e8e636c901","size":5,"soft_fail":false,"target_va":5411847808,"va":5381883797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883760,"raw_hex":"e82538c901","size":5,"soft_fail":false,"target_va":5411848144,"va":5381883814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":5900,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":5928,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":5960,"rva":13174736,"va":5381883856}
// 编译实例 RVAs: 0xc907d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29963833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883856,"raw_hex":"e83936c901","size":5,"soft_fail":false,"target_va":5411847808,"va":5381883970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29964180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381883856,"raw_hex":"e89437c901","size":5,"soft_fail":false,"target_va":5411848144,"va":5381883959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":759,"record_offset":6088,"parent_record_offset":5960,"ancestor_record_offset":5960,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":6200,"parent_record_offset":5960,"ancestor_record_offset":5960,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":6172,"parent_record_offset":5960,"ancestor_record_offset":5960,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":6144,"parent_record_offset":5960,"ancestor_record_offset":5960,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":6116,"parent_record_offset":5960,"ancestor_record_offset":5960,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":6232,"rva":13174912,"va":5381884032}
// 编译实例 RVAs: 0xc90880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29963663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381884032,"raw_hex":"e88f35c901","size":5,"soft_fail":false,"target_va":5411847808,"va":5381884140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29963987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381884032,"raw_hex":"e8d336c901","size":5,"soft_fail":false,"target_va":5411848144,"va":5381884152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":759,"record_offset":6388,"parent_record_offset":6232,"ancestor_record_offset":6232,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":6472,"parent_record_offset":6232,"ancestor_record_offset":6232,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":6416,"parent_record_offset":6232,"ancestor_record_offset":6232,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":6444,"parent_record_offset":6232,"ancestor_record_offset":6232,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":6360,"parent_record_offset":6232,"ancestor_record_offset":6232,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":6504,"rva":13175104,"va":5381884224}
// 编译实例 RVAs: 0xc90940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29963538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381884224,"raw_hex":"e81235c901","size":5,"soft_fail":false,"target_va":5411847808,"va":5381884265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29963862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381884224,"raw_hex":"e85636c901","size":5,"soft_fail":false,"target_va":5411848144,"va":5381884277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":6672,"parent_record_offset":6504,"ancestor_record_offset":6504,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":6616,"parent_record_offset":6504,"ancestor_record_offset":6504,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":6644,"parent_record_offset":6504,"ancestor_record_offset":6504,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":759,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":6504,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::radical_inverse
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 340
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAECA
// 首条位置: {"module":759,"record_offset":6752,"rva":13175200,"va":5381884320}
// 编译实例 RVAs: 0xc909a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29659111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381884320,"raw_hex":"e8e78fc401","size":5,"soft_fail":false,"target_va":5411543584,"va":5381884468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 首条位置: {"module":759,"record_offset":6852,"rva":13175360,"va":5381884480}
// 编译实例 RVAs: 0xc90a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29656649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381884480,"raw_hex":"e84986c401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381885410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::vec4::Vec4::is_finite_mask
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6451651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381884480,"raw_hex":"e8c3716200","size":5,"soft_fail":false,"target_va":5388336976,"va":5381885320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":7432,"parent_record_offset":6852,"ancestor_record_offset":6852,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":7796,"parent_record_offset":6852,"ancestor_record_offset":6852,"inlinee":25835,"source_line":522,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":7036,"parent_record_offset":6852,"ancestor_record_offset":6852,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":6980,"parent_record_offset":6852,"ancestor_record_offset":6852,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":7072,"parent_record_offset":6852,"ancestor_record_offset":6852,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":759,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":6852,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/sse2/bvec4a.rs
//     evidence={"module":759,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":6852,"inlinee":25838,"source_line":91,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":759,"record_offset":7556,"parent_record_offset":7460,"ancestor_record_offset":6852,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":7460,"parent_record_offset":7432,"ancestor_record_offset":6852,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":759,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":6852,"inlinee":25836,"source_line":421,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":7156,"parent_record_offset":7124,"ancestor_record_offset":6852,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6852,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":7100,"parent_record_offset":7072,"ancestor_record_offset":6852,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":7188,"parent_record_offset":7156,"ancestor_record_offset":6852,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":7884,"rva":13176304,"va":5381885424}
// 编译实例 RVAs: 0xc90df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9f20
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4026082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381885424,"raw_hex":"e81e91c2ff","size":5,"soft_fail":false,"target_va":5377859360,"va":5381885437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 首条位置: {"module":1585,"record_offset":132688,"rva":19894112,"va":5388603232}
// 编译实例 RVAs: 0x12f8f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-14246235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388603232,"raw_hex":"e8a59e26ff","size":5,"soft_fail":false,"target_va":5374357104,"va":5388603334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22743522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388603232,"raw_hex":"e8e2095b01","size":5,"soft_fail":false,"target_va":5411347040,"va":5388603513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22938448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388603232,"raw_hex":"e850035e01","size":5,"soft_fail":false,"target_va":5411542296,"va":5388603843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":133044,"parent_record_offset":132688,"ancestor_record_offset":132688,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":133220,"parent_record_offset":132688,"ancestor_record_offset":132688,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":133384,"parent_record_offset":132688,"ancestor_record_offset":132688,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":132948,"parent_record_offset":132688,"ancestor_record_offset":132688,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":132920,"parent_record_offset":132688,"ancestor_record_offset":132688,"inlinee":7153,"source_line":1836,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":134148,"parent_record_offset":133964,"ancestor_record_offset":132688,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":133100,"parent_record_offset":133076,"ancestor_record_offset":132688,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":133248,"parent_record_offset":133220,"ancestor_record_offset":132688,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":133332,"parent_record_offset":133308,"ancestor_record_offset":132688,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":133308,"parent_record_offset":133276,"ancestor_record_offset":132688,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":133148,"parent_record_offset":133124,"ancestor_record_offset":132688,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":133124,"parent_record_offset":133100,"ancestor_record_offset":132688,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":133276,"parent_record_offset":133248,"ancestor_record_offset":132688,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":133076,"parent_record_offset":133044,"ancestor_record_offset":132688,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":133452,"parent_record_offset":133384,"ancestor_record_offset":132688,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":133424,"parent_record_offset":133384,"ancestor_record_offset":132688,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":133520,"parent_record_offset":133452,"ancestor_record_offset":132688,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1585,"record_offset":133928,"parent_record_offset":133452,"ancestor_record_offset":132688,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":133796,"parent_record_offset":133760,"ancestor_record_offset":132688,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1585,"record_offset":133760,"parent_record_offset":133724,"ancestor_record_offset":132688,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1585,"record_offset":133724,"parent_record_offset":133452,"ancestor_record_offset":132688,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1585,"record_offset":133632,"parent_record_offset":133608,"ancestor_record_offset":132688,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":133676,"parent_record_offset":133452,"ancestor_record_offset":132688,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1585,"record_offset":133188,"parent_record_offset":133044,"ancestor_record_offset":132688,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1585,"record_offset":132976,"parent_record_offset":132948,"ancestor_record_offset":132688,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1585,"record_offset":133004,"parent_record_offset":132976,"ancestor_record_offset":132688,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/01585.jsonl.gz"}
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
// 首条位置: {"module":1778,"record_offset":1848,"rva":25165888,"va":5393875008}
// 编译实例 RVAs: 0x1800040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-19518012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393875008,"raw_hex":"e8c42dd6fe","size":5,"soft_fail":false,"target_va":5374357104,"va":5393875111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17838623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393875008,"raw_hex":"e81f321001","size":5,"soft_fail":false,"target_va":5411713920,"va":5393875292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17666578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393875008,"raw_hex":"e812920d01","size":5,"soft_fail":false,"target_va":5411542296,"va":5393875713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":2132,"parent_record_offset":1848,"ancestor_record_offset":1848,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":2308,"parent_record_offset":1848,"ancestor_record_offset":1848,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":2472,"parent_record_offset":1848,"ancestor_record_offset":1848,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":2036,"parent_record_offset":1848,"ancestor_record_offset":1848,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1778,"record_offset":2008,"parent_record_offset":1848,"ancestor_record_offset":1848,"inlinee":7153,"source_line":1836,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1778,"record_offset":3232,"parent_record_offset":3048,"ancestor_record_offset":1848,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1778,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":1848,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1778,"record_offset":2336,"parent_record_offset":2308,"ancestor_record_offset":1848,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1778,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":1848,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1778,"record_offset":2396,"parent_record_offset":2364,"ancestor_record_offset":1848,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1778,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1848,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1778,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1848,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1778,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":1848,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":2164,"parent_record_offset":2132,"ancestor_record_offset":1848,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":2540,"parent_record_offset":2472,"ancestor_record_offset":1848,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":2512,"parent_record_offset":2472,"ancestor_record_offset":1848,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":2608,"parent_record_offset":2540,"ancestor_record_offset":1848,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1778,"record_offset":3012,"parent_record_offset":2540,"ancestor_record_offset":1848,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1778,"record_offset":2880,"parent_record_offset":2844,"ancestor_record_offset":1848,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1778,"record_offset":2844,"parent_record_offset":2808,"ancestor_record_offset":1848,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1778,"record_offset":2808,"parent_record_offset":2540,"ancestor_record_offset":1848,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1778,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":1848,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1778,"record_offset":2764,"parent_record_offset":2540,"ancestor_record_offset":1848,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1778,"record_offset":2276,"parent_record_offset":2132,"ancestor_record_offset":1848,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1778,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":1848,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1778,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":1848,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/01778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::cartesian_to_polar
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 418
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAED0
// 首条位置: {"module":759,"record_offset":7984,"rva":13176352,"va":5381885472}
// 编译实例 RVAs: 0xc90e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29961905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381885472,"raw_hex":"e8b12ec901","size":5,"soft_fail":false,"target_va":5411847408,"va":5381885498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29962194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381885472,"raw_hex":"e8d22fc901","size":5,"soft_fail":false,"target_va":5411847712,"va":5381885513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":8128,"parent_record_offset":7984,"ancestor_record_offset":7984,"inlinee":14897,"source_line":830,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":8100,"parent_record_offset":7984,"ancestor_record_offset":7984,"inlinee":10999,"source_line":627,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":8160,"rva":13176432,"va":5381885552}
// 编译实例 RVAs: 0xc90e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29962222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381885552,"raw_hex":"e8ee2fc901","size":5,"soft_fail":false,"target_va":5411847808,"va":5381885581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29962541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381885552,"raw_hex":"e82d31c901","size":5,"soft_fail":false,"target_va":5411848144,"va":5381885598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":8276,"parent_record_offset":8160,"ancestor_record_offset":8160,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":8304,"parent_record_offset":8160,"ancestor_record_offset":8160,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":8336,"rva":13176528,"va":5381885648}
// 编译实例 RVAs: 0xc90ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":759,"record_offset":8876,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":14186,"source_line":86,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":8460,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":8604,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":8636,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":759,"record_offset":8808,"parent_record_offset":8660,"ancestor_record_offset":8336,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":8532,"parent_record_offset":8504,"ancestor_record_offset":8336,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":8504,"parent_record_offset":8484,"ancestor_record_offset":8336,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":8484,"parent_record_offset":8460,"ancestor_record_offset":8336,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":8736,"parent_record_offset":8700,"ancestor_record_offset":8336,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":8700,"parent_record_offset":8660,"ancestor_record_offset":8336,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":759,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":8336,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":8560,"parent_record_offset":8532,"ancestor_record_offset":8336,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_mat3a_bad
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 443
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEAE
// 首条位置: {"module":759,"inlinee":44718}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":759,"record_offset":9192,"parent_record_offset":9020,"ancestor_record_offset":9020,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9276,"parent_record_offset":9020,"ancestor_record_offset":9020,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9428,"parent_record_offset":9020,"ancestor_record_offset":9020,"inlinee":44715,"source_line":550,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":9048,"parent_record_offset":9020,"ancestor_record_offset":9020,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9604,"parent_record_offset":9428,"ancestor_record_offset":9020,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9308,"parent_record_offset":9276,"ancestor_record_offset":9020,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9392,"parent_record_offset":9276,"ancestor_record_offset":9020,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9576,"parent_record_offset":9428,"ancestor_record_offset":9020,"inlinee":44716,"source_line":129,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9364,"parent_record_offset":9276,"ancestor_record_offset":9020,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":9020,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":9020,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":9120,"parent_record_offset":9092,"ancestor_record_offset":9020,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":9092,"parent_record_offset":9072,"ancestor_record_offset":9020,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":9020,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":9484,"parent_record_offset":9456,"ancestor_record_offset":9020,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":9456,"parent_record_offset":9428,"ancestor_record_offset":9020,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":9020,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_bad
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 454
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEB0
// 首条位置: {"module":759,"inlinee":44720}
// 编译实例 RVAs: 0xc90fd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":9640,"parent_record_offset":8908,"ancestor_record_offset":8908,"inlinee":44719,"source_line":281,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] utils::xz_rotation_mat3a_bad
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":9020,"parent_record_offset":8908,"ancestor_record_offset":8908,"inlinee":44718,"source_line":443,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":759,"record_offset":9192,"parent_record_offset":9020,"ancestor_record_offset":8908,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9604,"parent_record_offset":9428,"ancestor_record_offset":8908,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9308,"parent_record_offset":9276,"ancestor_record_offset":8908,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9392,"parent_record_offset":9276,"ancestor_record_offset":8908,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9576,"parent_record_offset":9428,"ancestor_record_offset":8908,"inlinee":44716,"source_line":129,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":9364,"parent_record_offset":9276,"ancestor_record_offset":8908,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":759,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":8908,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":759,"record_offset":9852,"parent_record_offset":9748,"ancestor_record_offset":8908,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":9748,"parent_record_offset":9640,"ancestor_record_offset":8908,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":10076,"parent_record_offset":9748,"ancestor_record_offset":8908,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9276,"parent_record_offset":9020,"ancestor_record_offset":8908,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":8908,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9428,"parent_record_offset":9020,"ancestor_record_offset":8908,"inlinee":44715,"source_line":550,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":8908,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":9688,"parent_record_offset":9664,"ancestor_record_offset":8908,"inlinee":11331,"source_line":1875,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":9120,"parent_record_offset":9092,"ancestor_record_offset":8908,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":9092,"parent_record_offset":9072,"ancestor_record_offset":8908,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":8908,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":9048,"parent_record_offset":9020,"ancestor_record_offset":8908,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":9484,"parent_record_offset":9456,"ancestor_record_offset":8908,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":9456,"parent_record_offset":9428,"ancestor_record_offset":8908,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":8908,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":10116,"rva":13177296,"va":5381886416}
// 编译实例 RVAs: 0xc911d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::xz_rotation_bad
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":10236,"parent_record_offset":10116,"ancestor_record_offset":10116,"inlinee":44720,"source_line":454,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":759,"record_offset":10448,"parent_record_offset":10260,"ancestor_record_offset":10116,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":10860,"parent_record_offset":10684,"ancestor_record_offset":10116,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":10564,"parent_record_offset":10532,"ancestor_record_offset":10116,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":10648,"parent_record_offset":10532,"ancestor_record_offset":10116,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":10832,"parent_record_offset":10684,"ancestor_record_offset":10116,"inlinee":44716,"source_line":129,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":10620,"parent_record_offset":10532,"ancestor_record_offset":10116,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":759,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":10116,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":759,"record_offset":11108,"parent_record_offset":11004,"ancestor_record_offset":10116,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":10896,"parent_record_offset":10236,"ancestor_record_offset":10116,"inlinee":44719,"source_line":281,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":11004,"parent_record_offset":10896,"ancestor_record_offset":10116,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":11332,"parent_record_offset":11004,"ancestor_record_offset":10116,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":10532,"parent_record_offset":10260,"ancestor_record_offset":10116,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":10496,"parent_record_offset":10472,"ancestor_record_offset":10116,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":10684,"parent_record_offset":10260,"ancestor_record_offset":10116,"inlinee":44715,"source_line":550,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":10472,"parent_record_offset":10448,"ancestor_record_offset":10116,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":10116,"inlinee":11331,"source_line":1875,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":10372,"parent_record_offset":10340,"ancestor_record_offset":10116,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":10340,"parent_record_offset":10316,"ancestor_record_offset":10116,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":10316,"parent_record_offset":10288,"ancestor_record_offset":10116,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":10288,"parent_record_offset":10260,"ancestor_record_offset":10116,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":10740,"parent_record_offset":10712,"ancestor_record_offset":10116,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":10712,"parent_record_offset":10684,"ancestor_record_offset":10116,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":10404,"parent_record_offset":10372,"ancestor_record_offset":10116,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] utils::xz_rotation_mat3a_bad
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":10260,"parent_record_offset":10236,"ancestor_record_offset":10116,"inlinee":44718,"source_line":443,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_mat3a_from_forward
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEB3
// 首条位置: {"module":759,"inlinee":44723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":759,"record_offset":11872,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":11712,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":11956,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":44715,"source_line":550,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$89::neg
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":12164,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":44722,"source_line":1754,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":11532,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":12132,"parent_record_offset":11956,"ancestor_record_offset":11496,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":11748,"parent_record_offset":11712,"ancestor_record_offset":11496,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":11836,"parent_record_offset":11712,"ancestor_record_offset":11496,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":12104,"parent_record_offset":11956,"ancestor_record_offset":11496,"inlinee":44716,"source_line":129,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":11808,"parent_record_offset":11712,"ancestor_record_offset":11496,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":11496,"inlinee":13674,"source_line":301,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":11496,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":11496,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":11632,"parent_record_offset":11596,"ancestor_record_offset":11496,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":11596,"parent_record_offset":11568,"ancestor_record_offset":11496,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":11568,"parent_record_offset":11532,"ancestor_record_offset":11496,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":12012,"parent_record_offset":11984,"ancestor_record_offset":11496,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":11984,"parent_record_offset":11956,"ancestor_record_offset":11496,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":11668,"parent_record_offset":11632,"ancestor_record_offset":11496,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_from_forward
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 479
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEB4
// 首条位置: {"module":759,"inlinee":44724}
// 编译实例 RVAs: 0xc913e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":12280,"parent_record_offset":11376,"ancestor_record_offset":11376,"inlinee":44719,"source_line":281,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] utils::xz_rotation_mat3a_from_forward
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":11496,"parent_record_offset":11376,"ancestor_record_offset":11376,"inlinee":44723,"source_line":471,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":759,"record_offset":11872,"parent_record_offset":11496,"ancestor_record_offset":11376,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":12132,"parent_record_offset":11956,"ancestor_record_offset":11376,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":11748,"parent_record_offset":11712,"ancestor_record_offset":11376,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":11836,"parent_record_offset":11712,"ancestor_record_offset":11376,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":12104,"parent_record_offset":11956,"ancestor_record_offset":11376,"inlinee":44716,"source_line":129,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":11808,"parent_record_offset":11712,"ancestor_record_offset":11376,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":11376,"inlinee":13674,"source_line":301,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":759,"record_offset":12352,"parent_record_offset":12328,"ancestor_record_offset":11376,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":759,"record_offset":12564,"parent_record_offset":12444,"ancestor_record_offset":11376,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":12444,"parent_record_offset":12280,"ancestor_record_offset":11376,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":12788,"parent_record_offset":12444,"ancestor_record_offset":11376,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":11712,"parent_record_offset":11496,"ancestor_record_offset":11376,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":11376,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":11956,"parent_record_offset":11496,"ancestor_record_offset":11376,"inlinee":44715,"source_line":550,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":11376,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":12328,"parent_record_offset":12304,"ancestor_record_offset":11376,"inlinee":11331,"source_line":1875,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$89::neg
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":12164,"parent_record_offset":11496,"ancestor_record_offset":11376,"inlinee":44722,"source_line":1754,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":11632,"parent_record_offset":11596,"ancestor_record_offset":11376,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":11596,"parent_record_offset":11568,"ancestor_record_offset":11376,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":11568,"parent_record_offset":11532,"ancestor_record_offset":11376,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":11532,"parent_record_offset":11496,"ancestor_record_offset":11376,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":12012,"parent_record_offset":11984,"ancestor_record_offset":11376,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":11984,"parent_record_offset":11956,"ancestor_record_offset":11376,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":11668,"parent_record_offset":11632,"ancestor_record_offset":11376,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":12828,"rva":13178432,"va":5381887552}
// 编译实例 RVAs: 0xc91640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::xz_rotation_from_forward
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":12956,"parent_record_offset":12828,"ancestor_record_offset":12828,"inlinee":44724,"source_line":479,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":759,"record_offset":13356,"parent_record_offset":12980,"ancestor_record_offset":12828,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":13616,"parent_record_offset":13440,"ancestor_record_offset":12828,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":13232,"parent_record_offset":13196,"ancestor_record_offset":12828,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":13320,"parent_record_offset":13196,"ancestor_record_offset":12828,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":13588,"parent_record_offset":13440,"ancestor_record_offset":12828,"inlinee":44716,"source_line":129,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":13292,"parent_record_offset":13196,"ancestor_record_offset":12828,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":12828,"inlinee":13674,"source_line":301,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":759,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":12828,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":759,"record_offset":14048,"parent_record_offset":13928,"ancestor_record_offset":12828,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":13764,"parent_record_offset":12956,"ancestor_record_offset":12828,"inlinee":44719,"source_line":281,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":13928,"parent_record_offset":13764,"ancestor_record_offset":12828,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":14272,"parent_record_offset":13928,"ancestor_record_offset":12828,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":13196,"parent_record_offset":12980,"ancestor_record_offset":12828,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":13404,"parent_record_offset":13380,"ancestor_record_offset":12828,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":13440,"parent_record_offset":12980,"ancestor_record_offset":12828,"inlinee":44715,"source_line":550,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":13380,"parent_record_offset":13356,"ancestor_record_offset":12828,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":12828,"inlinee":11331,"source_line":1875,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$89::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":13648,"parent_record_offset":12980,"ancestor_record_offset":12828,"inlinee":44722,"source_line":1754,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":13116,"parent_record_offset":13080,"ancestor_record_offset":12828,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":13080,"parent_record_offset":13052,"ancestor_record_offset":12828,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":13052,"parent_record_offset":13016,"ancestor_record_offset":12828,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":759,"record_offset":13016,"parent_record_offset":12980,"ancestor_record_offset":12828,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":13496,"parent_record_offset":13468,"ancestor_record_offset":12828,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":13468,"parent_record_offset":13440,"ancestor_record_offset":12828,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":13152,"parent_record_offset":13116,"ancestor_record_offset":12828,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] utils::xz_rotation_mat3a_from_forward
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":12828,"inlinee":44723,"source_line":471,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::cos_angle_between
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 488
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11506
// 首条位置: {"module":1434,"inlinee":70918}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":1112,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1164,"parent_record_offset":1112,"ancestor_record_offset":1112,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1434,"record_offset":1220,"parent_record_offset":1112,"ancestor_record_offset":1112,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":1112,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01434.jsonl.gz"}
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
// 首条位置: {"module":1705,"record_offset":252,"rva":24521584,"va":5393230704}
// 编译实例 RVAs: 0x1762b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$10::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1705,"record_offset":368,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":53589,"source_line":1951,"archive":"_pdb/B/modules/01705.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1705,"record_offset":416,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01705.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1705,"record_offset":388,"parent_record_offset":368,"ancestor_record_offset":252,"inlinee":19940,"source_line":1790,"archive":"_pdb/B/modules/01705.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1705,"record_offset":436,"parent_record_offset":416,"ancestor_record_offset":252,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$6::intersects
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 520, 521
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFBD
// 首条位置: {"module":768,"inlinee":44989}
// 编译实例 RVAs: 0x1762b90
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
// 首条位置: {"module":1606,"record_offset":46852,"rva":21347872,"va":5390056992}
// 编译实例 RVAs: 0x145be20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20274492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390056992,"raw_hex":"e8c4a2cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390057111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":1606,"record_offset":47012,"parent_record_offset":46852,"ancestor_record_offset":46852,"inlinee":15752,"source_line":323,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":46984,"parent_record_offset":46852,"ancestor_record_offset":46852,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":47184,"parent_record_offset":46852,"ancestor_record_offset":46852,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":47064,"parent_record_offset":47036,"ancestor_record_offset":46852,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":47088,"parent_record_offset":47064,"ancestor_record_offset":46852,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":47148,"parent_record_offset":47036,"ancestor_record_offset":46852,"inlinee":32023,"source_line":816,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":47036,"parent_record_offset":47012,"ancestor_record_offset":46852,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":47208,"parent_record_offset":47184,"ancestor_record_offset":46852,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":47232,"parent_record_offset":47208,"ancestor_record_offset":46852,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":47276,"parent_record_offset":47232,"ancestor_record_offset":46852,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":47620,"parent_record_offset":47232,"ancestor_record_offset":46852,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":47112,"parent_record_offset":47088,"ancestor_record_offset":46852,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":47364,"parent_record_offset":47276,"ancestor_record_offset":46852,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":47336,"parent_record_offset":47276,"ancestor_record_offset":46852,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":31872,"rva":21345072,"va":5390054192}
// 编译实例 RVAs: 0x145b330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20271693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390054192,"raw_hex":"e8b3adcafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390054312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a489e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10562033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390054192,"raw_hex":"e80fd65eff","size":5,"soft_fail":false,"target_va":5379492320,"va":5390054348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$8::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1606,"record_offset":32076,"parent_record_offset":31872,"ancestor_record_offset":31872,"inlinee":73713,"source_line":561,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":32048,"parent_record_offset":31872,"ancestor_record_offset":31872,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":32476,"parent_record_offset":31872,"ancestor_record_offset":31872,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1606,"record_offset":32100,"parent_record_offset":32076,"ancestor_record_offset":31872,"inlinee":21873,"source_line":3343,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":32236,"parent_record_offset":32124,"ancestor_record_offset":31872,"inlinee":31385,"source_line":235,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":32152,"parent_record_offset":32124,"ancestor_record_offset":31872,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":32176,"parent_record_offset":32152,"ancestor_record_offset":31872,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":32432,"parent_record_offset":32236,"ancestor_record_offset":31872,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":32124,"parent_record_offset":32100,"ancestor_record_offset":31872,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":32500,"parent_record_offset":32476,"ancestor_record_offset":31872,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":32524,"parent_record_offset":32500,"ancestor_record_offset":31872,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":32568,"parent_record_offset":32524,"ancestor_record_offset":31872,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":32912,"parent_record_offset":32524,"ancestor_record_offset":31872,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":32200,"parent_record_offset":32176,"ancestor_record_offset":31872,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":32656,"parent_record_offset":32568,"ancestor_record_offset":31872,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":32628,"parent_record_offset":32568,"ancestor_record_offset":31872,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1606,"record_offset":32392,"parent_record_offset":32368,"ancestor_record_offset":31872,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1606,"record_offset":32312,"parent_record_offset":32268,"ancestor_record_offset":31872,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":32368,"parent_record_offset":32344,"ancestor_record_offset":31872,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":32268,"parent_record_offset":32236,"ancestor_record_offset":31872,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":32344,"parent_record_offset":32268,"ancestor_record_offset":31872,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":19444,"rva":21342832,"va":5390051952}
// 编译实例 RVAs: 0x145aa70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20269458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390051952,"raw_hex":"e86eb6cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390052077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::impl$47::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1606,"record_offset":19652,"parent_record_offset":19444,"ancestor_record_offset":19444,"inlinee":37048,"source_line":567,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":19624,"parent_record_offset":19444,"ancestor_record_offset":19444,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":20160,"parent_record_offset":19444,"ancestor_record_offset":19444,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":19444,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":19920,"parent_record_offset":19892,"ancestor_record_offset":19444,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":19444,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":19676,"parent_record_offset":19652,"ancestor_record_offset":19444,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":19892,"parent_record_offset":19868,"ancestor_record_offset":19444,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":19844,"parent_record_offset":19652,"ancestor_record_offset":19444,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":20060,"parent_record_offset":19892,"ancestor_record_offset":19444,"inlinee":15760,"source_line":816,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":20184,"parent_record_offset":20160,"ancestor_record_offset":19444,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":19800,"parent_record_offset":19776,"ancestor_record_offset":19444,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":19444,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":20252,"parent_record_offset":20208,"ancestor_record_offset":19444,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":20596,"parent_record_offset":20208,"ancestor_record_offset":19444,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":19444,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":20340,"parent_record_offset":20252,"ancestor_record_offset":19444,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":20312,"parent_record_offset":20252,"ancestor_record_offset":19444,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$51::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1606,"record_offset":19868,"parent_record_offset":19844,"ancestor_record_offset":19444,"inlinee":21887,"source_line":1539,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":19776,"parent_record_offset":19724,"ancestor_record_offset":19444,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":52416,"rva":21348768,"va":5390057888}
// 编译实例 RVAs: 0x145c1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=6
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20275385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390057888,"raw_hex":"e8479fcafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390058004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::i32::ivec3::impl$135::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec3.rs
//     evidence={"module":1606,"record_offset":52584,"parent_record_offset":52416,"ancestor_record_offset":52416,"inlinee":72209,"source_line":19,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":52556,"parent_record_offset":52416,"ancestor_record_offset":52416,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":53124,"parent_record_offset":52416,"ancestor_record_offset":52416,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":52640,"parent_record_offset":52616,"ancestor_record_offset":52416,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":52664,"parent_record_offset":52640,"ancestor_record_offset":52416,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":52616,"parent_record_offset":52584,"ancestor_record_offset":52416,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":53148,"parent_record_offset":53124,"ancestor_record_offset":52416,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":52740,"parent_record_offset":52716,"ancestor_record_offset":52416,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":53172,"parent_record_offset":53148,"ancestor_record_offset":52416,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":53216,"parent_record_offset":53172,"ancestor_record_offset":52416,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":53560,"parent_record_offset":53172,"ancestor_record_offset":52416,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":52688,"parent_record_offset":52664,"ancestor_record_offset":52416,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":53304,"parent_record_offset":53216,"ancestor_record_offset":52416,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":53276,"parent_record_offset":53216,"ancestor_record_offset":52416,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":52716,"parent_record_offset":52664,"ancestor_record_offset":52416,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref$<slice2$<array$<u32,2> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2629,"record_offset":8348,"rva":30372464,"va":5399081584}
// 编译实例 RVAs: 0x1cf7270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..hash..random..DefaultHasher$u20$as$u20$core..hash..Hasher$GT$::write::h2e3b9fcd8b5c3661 (.llvm.123652408711701364)
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6217880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399081584,"raw_hex":"e898e05e00","size":5,"soft_fail":false,"target_va":5405299648,"va":5399081763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29299086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399081584,"raw_hex":"e872ee40fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5399081705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":8524,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":8496,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":9096,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":2629,"record_offset":8912,"parent_record_offset":8716,"ancestor_record_offset":8348,"inlinee":15752,"source_line":323,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":8716,"parent_record_offset":8548,"ancestor_record_offset":8348,"inlinee":31385,"source_line":235,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":8576,"parent_record_offset":8548,"ancestor_record_offset":8348,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":8600,"parent_record_offset":8576,"ancestor_record_offset":8348,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":9048,"parent_record_offset":8936,"ancestor_record_offset":8348,"inlinee":32023,"source_line":816,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":8348,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":8348,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":8676,"parent_record_offset":8652,"ancestor_record_offset":8348,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":9144,"parent_record_offset":9120,"ancestor_record_offset":8348,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":9188,"parent_record_offset":9144,"ancestor_record_offset":8348,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":9532,"parent_record_offset":9144,"ancestor_record_offset":8348,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":8624,"parent_record_offset":8600,"ancestor_record_offset":8348,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":9276,"parent_record_offset":9188,"ancestor_record_offset":8348,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":9248,"parent_record_offset":9188,"ancestor_record_offset":8348,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2629,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":8348,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2629,"record_offset":8792,"parent_record_offset":8748,"ancestor_record_offset":8348,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2629,"record_offset":8848,"parent_record_offset":8824,"ancestor_record_offset":8348,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2629,"record_offset":8748,"parent_record_offset":8716,"ancestor_record_offset":8348,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2629,"record_offset":8824,"parent_record_offset":8748,"ancestor_record_offset":8348,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":8652,"parent_record_offset":8600,"ancestor_record_offset":8348,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":50256,"rva":21348272,"va":5390057392}
// 编译实例 RVAs: 0x145bfb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20274892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390057392,"raw_hex":"e834a1cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390057511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":50440,"parent_record_offset":50256,"ancestor_record_offset":50256,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":50412,"parent_record_offset":50256,"ancestor_record_offset":50256,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":51040,"parent_record_offset":50256,"ancestor_record_offset":50256,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":50632,"parent_record_offset":50464,"ancestor_record_offset":50256,"inlinee":31385,"source_line":235,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":50492,"parent_record_offset":50464,"ancestor_record_offset":50256,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":50796,"parent_record_offset":50772,"ancestor_record_offset":50256,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":50516,"parent_record_offset":50492,"ancestor_record_offset":50256,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":50772,"parent_record_offset":50740,"ancestor_record_offset":50256,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":50464,"parent_record_offset":50440,"ancestor_record_offset":50256,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":51064,"parent_record_offset":51040,"ancestor_record_offset":50256,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":50592,"parent_record_offset":50568,"ancestor_record_offset":50256,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":51088,"parent_record_offset":51064,"ancestor_record_offset":50256,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":51132,"parent_record_offset":51088,"ancestor_record_offset":50256,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":51476,"parent_record_offset":51088,"ancestor_record_offset":50256,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":50540,"parent_record_offset":50516,"ancestor_record_offset":50256,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":50820,"parent_record_offset":50796,"ancestor_record_offset":50256,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":51220,"parent_record_offset":51132,"ancestor_record_offset":50256,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":51192,"parent_record_offset":51132,"ancestor_record_offset":50256,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1606,"record_offset":50704,"parent_record_offset":50664,"ancestor_record_offset":50256,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":50664,"parent_record_offset":50632,"ancestor_record_offset":50256,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$133::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec3.rs
//     evidence={"module":1606,"record_offset":50740,"parent_record_offset":50632,"ancestor_record_offset":50256,"inlinee":73707,"source_line":19,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":50568,"parent_record_offset":50516,"ancestor_record_offset":50256,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":30048,"rva":21344672,"va":5390053792}
// 编译实例 RVAs: 0x145b1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20271293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390053792,"raw_hex":"e843afcafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390053912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":30212,"parent_record_offset":30048,"ancestor_record_offset":30048,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":30184,"parent_record_offset":30048,"ancestor_record_offset":30048,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":30496,"parent_record_offset":30048,"ancestor_record_offset":30048,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":30264,"parent_record_offset":30236,"ancestor_record_offset":30048,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":30288,"parent_record_offset":30264,"ancestor_record_offset":30048,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":30236,"parent_record_offset":30212,"ancestor_record_offset":30048,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":30404,"parent_record_offset":30236,"ancestor_record_offset":30048,"inlinee":15760,"source_line":816,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":30520,"parent_record_offset":30496,"ancestor_record_offset":30048,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":30364,"parent_record_offset":30340,"ancestor_record_offset":30048,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":30544,"parent_record_offset":30520,"ancestor_record_offset":30048,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":30588,"parent_record_offset":30544,"ancestor_record_offset":30048,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":30932,"parent_record_offset":30544,"ancestor_record_offset":30048,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":30312,"parent_record_offset":30288,"ancestor_record_offset":30048,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":30676,"parent_record_offset":30588,"ancestor_record_offset":30048,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":30648,"parent_record_offset":30588,"ancestor_record_offset":30048,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":30340,"parent_record_offset":30288,"ancestor_record_offset":30048,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1184,"record_offset":256,"rva":6020512,"va":5374729632}
// 编译实例 RVAs: 0x5bdda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=12
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4947115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374729632,"raw_hex":"e85583b4ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5374729734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":408,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":384,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":576,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":256,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":432,"parent_record_offset":408,"ancestor_record_offset":256,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":256,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":256,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":668,"parent_record_offset":624,"ancestor_record_offset":256,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":1012,"parent_record_offset":624,"ancestor_record_offset":256,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=184
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":756,"parent_record_offset":668,"ancestor_record_offset":256,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=120
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":728,"parent_record_offset":668,"ancestor_record_offset":256,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":256,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<rhapsody::raw::texture::TextureViewDesc>
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1606,"record_offset":42636,"rva":21347056,"va":5390056176}
// 编译实例 RVAs: 0x145baf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=15
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20273677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390056176,"raw_hex":"e8f3a5cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390056296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$38::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1606,"record_offset":42820,"parent_record_offset":42636,"ancestor_record_offset":42636,"inlinee":73717,"source_line":251,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":42792,"parent_record_offset":42636,"ancestor_record_offset":42636,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":45476,"parent_record_offset":42636,"ancestor_record_offset":42636,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$350::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":1606,"record_offset":43464,"parent_record_offset":43268,"ancestor_record_offset":42636,"inlinee":73721,"source_line":183,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$622::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":1606,"record_offset":44228,"parent_record_offset":44032,"ancestor_record_offset":42636,"inlinee":73723,"source_line":389,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$306::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1606,"record_offset":43068,"parent_record_offset":42876,"ancestor_record_offset":42636,"inlinee":73719,"source_line":110,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$531::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1606,"record_offset":43832,"parent_record_offset":43636,"ancestor_record_offset":42636,"inlinee":32988,"source_line":649,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":43116,"parent_record_offset":43092,"ancestor_record_offset":42636,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":42924,"parent_record_offset":42900,"ancestor_record_offset":42636,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":43140,"parent_record_offset":43116,"ancestor_record_offset":42636,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":45384,"parent_record_offset":45360,"ancestor_record_offset":42636,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":42948,"parent_record_offset":42924,"ancestor_record_offset":42636,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$0::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":45360,"parent_record_offset":42820,"ancestor_record_offset":42636,"inlinee":13870,"source_line":847,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":43488,"parent_record_offset":43464,"ancestor_record_offset":42636,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":43092,"parent_record_offset":43068,"ancestor_record_offset":42636,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":42900,"parent_record_offset":42876,"ancestor_record_offset":42636,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":45500,"parent_record_offset":45476,"ancestor_record_offset":42636,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":43024,"parent_record_offset":43000,"ancestor_record_offset":42636,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":45524,"parent_record_offset":45500,"ancestor_record_offset":42636,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":45568,"parent_record_offset":45524,"ancestor_record_offset":42636,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":45912,"parent_record_offset":45524,"ancestor_record_offset":42636,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":42972,"parent_record_offset":42948,"ancestor_record_offset":42636,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":43164,"parent_record_offset":43140,"ancestor_record_offset":42636,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":45656,"parent_record_offset":45568,"ancestor_record_offset":42636,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":45628,"parent_record_offset":45568,"ancestor_record_offset":42636,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1606,"record_offset":42876,"parent_record_offset":42820,"ancestor_record_offset":42636,"inlinee":37048,"source_line":567,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":43000,"parent_record_offset":42948,"ancestor_record_offset":42636,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":246,"record_offset":7636,"rva":6019744,"va":5374728864}
// 编译实例 RVAs: 0x5bdaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4946355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374728864,"raw_hex":"e84d86b4ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5374728974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":7812,"parent_record_offset":7636,"ancestor_record_offset":7636,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":8028,"parent_record_offset":7636,"ancestor_record_offset":7636,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_tod::do_time_of_day_screen::impl$2::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_tod.rs
//     evidence={"module":246,"record_offset":7836,"parent_record_offset":7636,"ancestor_record_offset":7636,"inlinee":15883,"source_line":91,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":7636,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":7904,"parent_record_offset":7880,"ancestor_record_offset":7636,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":7856,"parent_record_offset":7836,"ancestor_record_offset":7636,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":7636,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":7636,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":7636,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":8120,"parent_record_offset":8076,"ancestor_record_offset":7636,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":8464,"parent_record_offset":8076,"ancestor_record_offset":7636,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":7928,"parent_record_offset":7904,"ancestor_record_offset":7636,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":8208,"parent_record_offset":8120,"ancestor_record_offset":7636,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":8180,"parent_record_offset":8120,"ancestor_record_offset":7636,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":7956,"parent_record_offset":7904,"ancestor_record_offset":7636,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00246.jsonl.gz"}
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
// 首条位置: {"module":2629,"record_offset":4540,"rva":30371632,"va":5399080752}
// 编译实例 RVAs: 0x1cf6f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29298288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399080752,"raw_hex":"e890f140fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5399080907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14228ce80
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5856995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399080752,"raw_hex":"e8e35e5900","size":5,"soft_fail":false,"target_va":5404937856,"va":5399080856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":4748,"parent_record_offset":4540,"ancestor_record_offset":4540,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":4720,"parent_record_offset":4540,"ancestor_record_offset":4540,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":5092,"parent_record_offset":4540,"ancestor_record_offset":4540,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":2629,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":4540,"inlinee":15752,"source_line":323,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":4848,"parent_record_offset":4796,"ancestor_record_offset":4540,"inlinee":31385,"source_line":235,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":4540,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":4540,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":4900,"parent_record_offset":4872,"ancestor_record_offset":4540,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":4540,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":4540,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4540,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4540,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":5184,"parent_record_offset":5140,"ancestor_record_offset":4540,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":5528,"parent_record_offset":5140,"ancestor_record_offset":4540,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":4540,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":5272,"parent_record_offset":5184,"ancestor_record_offset":4540,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":5244,"parent_record_offset":5184,"ancestor_record_offset":4540,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$6::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":2629,"record_offset":4872,"parent_record_offset":4848,"ancestor_record_offset":4540,"inlinee":28927,"source_line":69,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":5024,"parent_record_offset":4972,"ancestor_record_offset":4540,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":40460,"rva":21346576,"va":5390055696}
// 编译实例 RVAs: 0x145b910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=5
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20273201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390055696,"raw_hex":"e8cfa7cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390055820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":40692,"parent_record_offset":40460,"ancestor_record_offset":40460,"inlinee":71777,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":40664,"parent_record_offset":40460,"ancestor_record_offset":40460,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":41260,"parent_record_offset":40460,"ancestor_record_offset":40460,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":1606,"record_offset":40716,"parent_record_offset":40692,"ancestor_record_offset":40460,"inlinee":15752,"source_line":323,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":41108,"parent_record_offset":41084,"ancestor_record_offset":40460,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":40768,"parent_record_offset":40740,"ancestor_record_offset":40460,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":40792,"parent_record_offset":40768,"ancestor_record_offset":40460,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":40852,"parent_record_offset":40740,"ancestor_record_offset":40460,"inlinee":32023,"source_line":816,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":41084,"parent_record_offset":41060,"ancestor_record_offset":40460,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":40740,"parent_record_offset":40716,"ancestor_record_offset":40460,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":41284,"parent_record_offset":41260,"ancestor_record_offset":40460,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":41208,"parent_record_offset":41184,"ancestor_record_offset":40460,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":41308,"parent_record_offset":41284,"ancestor_record_offset":40460,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":41352,"parent_record_offset":41308,"ancestor_record_offset":40460,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":41696,"parent_record_offset":41308,"ancestor_record_offset":40460,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":40816,"parent_record_offset":40792,"ancestor_record_offset":40460,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":41440,"parent_record_offset":41352,"ancestor_record_offset":40460,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":41412,"parent_record_offset":41352,"ancestor_record_offset":40460,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$31::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1606,"record_offset":41060,"parent_record_offset":40692,"ancestor_record_offset":40460,"inlinee":73715,"source_line":445,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":41184,"parent_record_offset":41132,"ancestor_record_offset":40460,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":11200,"rva":21341504,"va":5390050624}
// 编译实例 RVAs: 0x145a540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=5
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20268124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390050624,"raw_hex":"e8a4bbcafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390050743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":11392,"parent_record_offset":11200,"ancestor_record_offset":11200,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":11364,"parent_record_offset":11200,"ancestor_record_offset":11200,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":12048,"parent_record_offset":11200,"ancestor_record_offset":11200,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":1606,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":11200,"inlinee":15752,"source_line":323,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":11468,"parent_record_offset":11440,"ancestor_record_offset":11200,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":11200,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":11200,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":11620,"parent_record_offset":11588,"ancestor_record_offset":11200,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":11552,"parent_record_offset":11440,"ancestor_record_offset":11200,"inlinee":32023,"source_line":816,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":11200,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":12072,"parent_record_offset":12048,"ancestor_record_offset":11200,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":11200,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":11200,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":12140,"parent_record_offset":12096,"ancestor_record_offset":11200,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":12484,"parent_record_offset":12096,"ancestor_record_offset":11200,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":11516,"parent_record_offset":11492,"ancestor_record_offset":11200,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":11668,"parent_record_offset":11644,"ancestor_record_offset":11200,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":12228,"parent_record_offset":12140,"ancestor_record_offset":11200,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":12200,"parent_record_offset":12140,"ancestor_record_offset":11200,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$133::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec3.rs
//     evidence={"module":1606,"record_offset":11588,"parent_record_offset":11392,"ancestor_record_offset":11200,"inlinee":73707,"source_line":19,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":11696,"parent_record_offset":11644,"ancestor_record_offset":11200,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":2629,"record_offset":12560,"rva":30373376,"va":5399082496}
// 编译实例 RVAs: 0x1cf7600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29299995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399082496,"raw_hex":"e8e5ea40fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5399082614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":12772,"parent_record_offset":12560,"ancestor_record_offset":12560,"inlinee":71777,"source_line":892,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":12744,"parent_record_offset":12560,"ancestor_record_offset":12560,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":13280,"parent_record_offset":12560,"ancestor_record_offset":12560,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":13228,"parent_record_offset":13200,"ancestor_record_offset":12560,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":12560,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":12820,"parent_record_offset":12796,"ancestor_record_offset":12560,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":13128,"parent_record_offset":13104,"ancestor_record_offset":12560,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":13104,"parent_record_offset":12772,"ancestor_record_offset":12560,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":12560,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":12920,"parent_record_offset":12896,"ancestor_record_offset":12560,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":13176,"parent_record_offset":13152,"ancestor_record_offset":12560,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":13328,"parent_record_offset":13304,"ancestor_record_offset":12560,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":13200,"parent_record_offset":13176,"ancestor_record_offset":12560,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":13372,"parent_record_offset":13328,"ancestor_record_offset":12560,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":13716,"parent_record_offset":13328,"ancestor_record_offset":12560,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":12560,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":13460,"parent_record_offset":13372,"ancestor_record_offset":12560,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":13432,"parent_record_offset":13372,"ancestor_record_offset":12560,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$12::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":2629,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":12560,"inlinee":72211,"source_line":48,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":12896,"parent_record_offset":12844,"ancestor_record_offset":12560,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":13152,"parent_record_offset":13128,"ancestor_record_offset":12560,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/02629.jsonl.gz"}
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
// 首条位置: {"module":1540,"record_offset":5884,"rva":19450928,"va":5388160048}
// 编译实例 RVAs: 0x128cc30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18377545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388160048,"raw_hex":"e8b794e7fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5388160164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":6104,"parent_record_offset":5884,"ancestor_record_offset":5884,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":6076,"parent_record_offset":5884,"ancestor_record_offset":5884,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":6472,"parent_record_offset":5884,"ancestor_record_offset":5884,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":5884,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":6420,"parent_record_offset":6392,"ancestor_record_offset":5884,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":5884,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":5884,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":6296,"parent_record_offset":6104,"ancestor_record_offset":5884,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":5884,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":5884,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":5884,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":5884,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":5884,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":6564,"parent_record_offset":6520,"ancestor_record_offset":5884,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":6908,"parent_record_offset":6520,"ancestor_record_offset":5884,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":5884,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":6652,"parent_record_offset":6564,"ancestor_record_offset":5884,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":6624,"parent_record_offset":6564,"ancestor_record_offset":5884,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$26::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1540,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":5884,"inlinee":9030,"source_line":155,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":6228,"parent_record_offset":6176,"ancestor_record_offset":5884,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":5884,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01540.jsonl.gz"}
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
// 首条位置: {"module":1540,"record_offset":2080,"rva":19450480,"va":5388159600}
// 编译实例 RVAs: 0x128ca70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18377098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388159600,"raw_hex":"e87696e7fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5388159717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":2304,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":71777,"source_line":892,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":2276,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":2812,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":2080,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":2620,"parent_record_offset":2592,"ancestor_record_offset":2080,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":2080,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":2080,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":2668,"parent_record_offset":2304,"ancestor_record_offset":2080,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":2080,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":2496,"parent_record_offset":2304,"ancestor_record_offset":2080,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":2080,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":2080,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":2080,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2080,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":2080,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":2904,"parent_record_offset":2860,"ancestor_record_offset":2080,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":3248,"parent_record_offset":2860,"ancestor_record_offset":2080,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2080,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":2080,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":2992,"parent_record_offset":2904,"ancestor_record_offset":2080,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":2964,"parent_record_offset":2904,"ancestor_record_offset":2080,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$26::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1540,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":2080,"inlinee":9030,"source_line":155,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":2428,"parent_record_offset":2376,"ancestor_record_offset":2080,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":2080,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01540.jsonl.gz"}
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
// 首条位置: {"module":1184,"record_offset":3796,"rva":16431856,"va":5385140976}
// 编译实例 RVAs: 0xfabaf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15358476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385140976,"raw_hex":"e8f4a515ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5385141095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":4044,"parent_record_offset":3796,"ancestor_record_offset":3796,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":4016,"parent_record_offset":3796,"ancestor_record_offset":3796,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":4432,"parent_record_offset":3796,"ancestor_record_offset":3796,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":4280,"parent_record_offset":4256,"ancestor_record_offset":3796,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":3796,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":3796,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":3796,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":4256,"parent_record_offset":4236,"ancestor_record_offset":3796,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":3796,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":3796,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3796,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":4524,"parent_record_offset":4480,"ancestor_record_offset":3796,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":4868,"parent_record_offset":4480,"ancestor_record_offset":3796,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":3796,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":4612,"parent_record_offset":4524,"ancestor_record_offset":3796,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":3796,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":4584,"parent_record_offset":4524,"ancestor_record_offset":3796,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::action_set::impl$19::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":1184,"record_offset":4236,"parent_record_offset":4044,"ancestor_record_offset":3796,"inlinee":61879,"source_line":148,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$4::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1184,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3796,"inlinee":15746,"source_line":30,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":4168,"parent_record_offset":4116,"ancestor_record_offset":3796,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01184.jsonl.gz"}
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
// 首条位置: {"module":1184,"record_offset":5808,"rva":16432256,"va":5385141376}
// 编译实例 RVAs: 0xfabc80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15358875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385141376,"raw_hex":"e865a415ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5385141494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":6000,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":5972,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":6336,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":5808,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":5808,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":6192,"parent_record_offset":6000,"ancestor_record_offset":5808,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1184,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":5808,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":5808,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":5808,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":5808,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":6428,"parent_record_offset":6384,"ancestor_record_offset":5808,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1184,"record_offset":6772,"parent_record_offset":6384,"ancestor_record_offset":5808,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":5808,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":6516,"parent_record_offset":6428,"ancestor_record_offset":5808,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":5808,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1184,"record_offset":6488,"parent_record_offset":6428,"ancestor_record_offset":5808,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$4::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1184,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":5808,"inlinee":15746,"source_line":30,"archive":"_pdb/B/modules/01184.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1184,"record_offset":6124,"parent_record_offset":6072,"ancestor_record_offset":5808,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01184.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":13424,"rva":21341968,"va":5390051088}
// 编译实例 RVAs: 0x145a710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20268588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390051088,"raw_hex":"e8d4b9cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390051207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":13676,"parent_record_offset":13424,"ancestor_record_offset":13424,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":13648,"parent_record_offset":13424,"ancestor_record_offset":13424,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":14244,"parent_record_offset":13424,"ancestor_record_offset":13424,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":13424,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":13748,"parent_record_offset":13724,"ancestor_record_offset":13424,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":14188,"parent_record_offset":14160,"ancestor_record_offset":13424,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":13424,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":13724,"parent_record_offset":13700,"ancestor_record_offset":13424,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":13896,"parent_record_offset":13868,"ancestor_record_offset":13424,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":14088,"parent_record_offset":14064,"ancestor_record_offset":13424,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":14064,"parent_record_offset":13868,"ancestor_record_offset":13424,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":13424,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":13424,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":14136,"parent_record_offset":14112,"ancestor_record_offset":13424,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":14292,"parent_record_offset":14268,"ancestor_record_offset":13424,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":14160,"parent_record_offset":14136,"ancestor_record_offset":13424,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":14336,"parent_record_offset":14292,"ancestor_record_offset":13424,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":14680,"parent_record_offset":14292,"ancestor_record_offset":13424,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":13968,"parent_record_offset":13944,"ancestor_record_offset":13424,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":14424,"parent_record_offset":14336,"ancestor_record_offset":13424,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":13424,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":14396,"parent_record_offset":14336,"ancestor_record_offset":13424,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1606,"record_offset":13868,"parent_record_offset":13676,"ancestor_record_offset":13424,"inlinee":37048,"source_line":567,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1606,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":13424,"inlinee":70625,"source_line":11,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":13800,"parent_record_offset":13748,"ancestor_record_offset":13424,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":14112,"parent_record_offset":14088,"ancestor_record_offset":13424,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":246,"record_offset":9404,"rva":6020112,"va":5374729232}
// 编译实例 RVAs: 0x5bdc10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4946730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374729232,"raw_hex":"e8d684b4ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5374729349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":9620,"parent_record_offset":9404,"ancestor_record_offset":9404,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":9592,"parent_record_offset":9404,"ancestor_record_offset":9404,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":10008,"parent_record_offset":9404,"ancestor_record_offset":9404,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":9668,"parent_record_offset":9644,"ancestor_record_offset":9404,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":9404,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":9404,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":9404,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":9404,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":9832,"parent_record_offset":9812,"ancestor_record_offset":9404,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":9404,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":9768,"parent_record_offset":9744,"ancestor_record_offset":9404,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":9404,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":10100,"parent_record_offset":10056,"ancestor_record_offset":9404,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":10444,"parent_record_offset":10056,"ancestor_record_offset":9404,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":9404,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":9404,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":10188,"parent_record_offset":10100,"ancestor_record_offset":9404,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":10160,"parent_record_offset":10100,"ancestor_record_offset":9404,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":9744,"parent_record_offset":9692,"ancestor_record_offset":9404,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_tod::do_time_of_day_screen::impl$2::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_tod.rs
//     evidence={"module":246,"record_offset":9812,"parent_record_offset":9620,"ancestor_record_offset":9404,"inlinee":15883,"source_line":91,"archive":"_pdb/B/modules/00246.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":37660,"rva":21345952,"va":5390055072}
// 编译实例 RVAs: 0x145b6a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=7
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20272579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390055072,"raw_hex":"e83daacafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390055198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":37884,"parent_record_offset":37660,"ancestor_record_offset":37660,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":37856,"parent_record_offset":37660,"ancestor_record_offset":37660,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":39084,"parent_record_offset":37660,"ancestor_record_offset":37660,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":1606,"record_offset":38296,"parent_record_offset":38100,"ancestor_record_offset":37660,"inlinee":15752,"source_line":323,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":38100,"parent_record_offset":37932,"ancestor_record_offset":37660,"inlinee":31385,"source_line":235,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":37960,"parent_record_offset":37932,"ancestor_record_offset":37660,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":38836,"parent_record_offset":38812,"ancestor_record_offset":37660,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":37984,"parent_record_offset":37960,"ancestor_record_offset":37660,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":38812,"parent_record_offset":38780,"ancestor_record_offset":37660,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":38432,"parent_record_offset":38320,"ancestor_record_offset":37660,"inlinee":32023,"source_line":816,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":37932,"parent_record_offset":37908,"ancestor_record_offset":37660,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":37908,"parent_record_offset":37884,"ancestor_record_offset":37660,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":39108,"parent_record_offset":39084,"ancestor_record_offset":37660,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":38060,"parent_record_offset":38036,"ancestor_record_offset":37660,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":39132,"parent_record_offset":39108,"ancestor_record_offset":37660,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":39176,"parent_record_offset":39132,"ancestor_record_offset":37660,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":39520,"parent_record_offset":39132,"ancestor_record_offset":37660,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":38008,"parent_record_offset":37984,"ancestor_record_offset":37660,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":38860,"parent_record_offset":38836,"ancestor_record_offset":37660,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":39264,"parent_record_offset":39176,"ancestor_record_offset":37660,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":39236,"parent_record_offset":39176,"ancestor_record_offset":37660,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1606,"record_offset":38256,"parent_record_offset":38232,"ancestor_record_offset":37660,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1606,"record_offset":38176,"parent_record_offset":38132,"ancestor_record_offset":37660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":38232,"parent_record_offset":38208,"ancestor_record_offset":37660,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":38132,"parent_record_offset":38100,"ancestor_record_offset":37660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":38208,"parent_record_offset":38132,"ancestor_record_offset":37660,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$133::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec3.rs
//     evidence={"module":1606,"record_offset":38780,"parent_record_offset":38672,"ancestor_record_offset":37660,"inlinee":73707,"source_line":19,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":38036,"parent_record_offset":37984,"ancestor_record_offset":37660,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":33852,"rva":21345504,"va":5390054624}
// 编译实例 RVAs: 0x145b4e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20272130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390054624,"raw_hex":"e8feabcafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390054749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141620c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1857264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390054624,"raw_hex":"e8f0561c00","size":5,"soft_fail":false,"target_va":5391912032,"va":5390054763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141620dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1857570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390054624,"raw_hex":"e822581c00","size":5,"soft_fail":false,"target_va":5391912384,"va":5390054809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":34108,"parent_record_offset":33852,"ancestor_record_offset":33852,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":34080,"parent_record_offset":33852,"ancestor_record_offset":33852,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":34536,"parent_record_offset":33852,"ancestor_record_offset":33852,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":34184,"parent_record_offset":34156,"ancestor_record_offset":33852,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":34208,"parent_record_offset":34184,"ancestor_record_offset":33852,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":34156,"parent_record_offset":34132,"ancestor_record_offset":33852,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":34132,"parent_record_offset":34108,"ancestor_record_offset":33852,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":34560,"parent_record_offset":34536,"ancestor_record_offset":33852,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":34284,"parent_record_offset":34260,"ancestor_record_offset":33852,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":34584,"parent_record_offset":34560,"ancestor_record_offset":33852,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":34628,"parent_record_offset":34584,"ancestor_record_offset":33852,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":34972,"parent_record_offset":34584,"ancestor_record_offset":33852,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":34232,"parent_record_offset":34208,"ancestor_record_offset":33852,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":34716,"parent_record_offset":34628,"ancestor_record_offset":33852,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":34688,"parent_record_offset":34628,"ancestor_record_offset":33852,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":34260,"parent_record_offset":34208,"ancestor_record_offset":33852,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":2629,"record_offset":260,"rva":30371168,"va":5399080288}
// 编译实例 RVAs: 0x1cf6d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=5
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29297775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399080288,"raw_hex":"e891f340fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5399080394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$25::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":436,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":63049,"source_line":892,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":408,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":1080,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":260,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":260,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":260,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":936,"parent_record_offset":436,"ancestor_record_offset":260,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":796,"parent_record_offset":436,"ancestor_record_offset":260,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":628,"parent_record_offset":436,"ancestor_record_offset":260,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":484,"parent_record_offset":460,"ancestor_record_offset":260,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":460,"parent_record_offset":436,"ancestor_record_offset":260,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":260,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":260,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":260,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":260,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":260,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":1172,"parent_record_offset":1128,"ancestor_record_offset":260,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":1516,"parent_record_offset":1128,"ancestor_record_offset":260,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":260,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":1260,"parent_record_offset":1172,"ancestor_record_offset":260,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":1232,"parent_record_offset":1172,"ancestor_record_offset":260,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":728,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":260,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1606,"record_offset":4652,"rva":21340096,"va":5390049216}
// 编译实例 RVAs: 0x1459fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=5
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20266703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390049216,"raw_hex":"e831c1cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390049322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":4912,"parent_record_offset":4652,"ancestor_record_offset":4652,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":4884,"parent_record_offset":4652,"ancestor_record_offset":4652,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":5532,"parent_record_offset":4652,"ancestor_record_offset":4652,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4652,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5056,"parent_record_offset":5032,"ancestor_record_offset":4652,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":4652,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":4652,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":4652,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":4652,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":4652,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":4652,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":4652,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":4652,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5032,"parent_record_offset":5008,"ancestor_record_offset":4652,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5624,"parent_record_offset":5580,"ancestor_record_offset":4652,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5968,"parent_record_offset":5580,"ancestor_record_offset":4652,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":4652,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":5712,"parent_record_offset":5624,"ancestor_record_offset":4652,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":5684,"parent_record_offset":5624,"ancestor_record_offset":4652,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$5::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs
//     evidence={"module":1606,"record_offset":5324,"parent_record_offset":5128,"ancestor_record_offset":4652,"inlinee":73705,"source_line":35,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$9::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1606,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":4652,"inlinee":73703,"source_line":116,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":5448,"parent_record_offset":5396,"ancestor_record_offset":4652,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":4652,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::taa::TgsrMain> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1606,"record_offset":23416,"rva":21343264,"va":5390052384}
// 编译实例 RVAs: 0x145ac20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20269871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390052384,"raw_hex":"e8d1b4cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390052490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23652,"parent_record_offset":23416,"ancestor_record_offset":23416,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":23624,"parent_record_offset":23416,"ancestor_record_offset":23416,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":24240,"parent_record_offset":23416,"ancestor_record_offset":23416,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23916,"parent_record_offset":23892,"ancestor_record_offset":23416,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23940,"parent_record_offset":23916,"ancestor_record_offset":23416,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":23416,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23892,"parent_record_offset":23868,"ancestor_record_offset":23416,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23700,"parent_record_offset":23676,"ancestor_record_offset":23416,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23676,"parent_record_offset":23652,"ancestor_record_offset":23416,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":23416,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":23416,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":23416,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":24288,"parent_record_offset":24264,"ancestor_record_offset":23416,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":23772,"parent_record_offset":23748,"ancestor_record_offset":23416,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":24332,"parent_record_offset":24288,"ancestor_record_offset":23416,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":24676,"parent_record_offset":24288,"ancestor_record_offset":23416,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":23964,"parent_record_offset":23940,"ancestor_record_offset":23416,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":24420,"parent_record_offset":24332,"ancestor_record_offset":23416,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":24392,"parent_record_offset":24332,"ancestor_record_offset":23416,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::taa::impl$2::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1606,"record_offset":23868,"parent_record_offset":23844,"ancestor_record_offset":23416,"inlinee":73711,"source_line":116,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":23992,"parent_record_offset":23940,"ancestor_record_offset":23416,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":23416,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::vermeer::GenerateStrokes> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1606,"record_offset":2624,"rva":21339680,"va":5390048800}
// 编译实例 RVAs: 0x1459e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20266287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390048800,"raw_hex":"e8d1c2cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390048906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":2856,"parent_record_offset":2624,"ancestor_record_offset":2624,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":2828,"parent_record_offset":2624,"ancestor_record_offset":2624,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":3276,"parent_record_offset":2624,"ancestor_record_offset":2624,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":2624,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":2624,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":2624,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":2624,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":2904,"parent_record_offset":2880,"ancestor_record_offset":2624,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2624,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":2624,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":2624,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":2624,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":2624,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":2624,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":3368,"parent_record_offset":3324,"ancestor_record_offset":2624,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":3712,"parent_record_offset":3324,"ancestor_record_offset":2624,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":2624,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":3456,"parent_record_offset":3368,"ancestor_record_offset":2624,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":3428,"parent_record_offset":3368,"ancestor_record_offset":2624,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::vermeer::impl$8::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1606,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":2624,"inlinee":73701,"source_line":116,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":3196,"parent_record_offset":3144,"ancestor_record_offset":2624,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":2624,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,ref$<tuple$<> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":11384,"rva":6020512,"va":5374729632}
// 编译实例 RVAs: 0x5bdda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4947115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374729632,"raw_hex":"e85583b4ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5374729734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":11560,"parent_record_offset":11384,"ancestor_record_offset":11384,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":11536,"parent_record_offset":11384,"ancestor_record_offset":11384,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":11756,"parent_record_offset":11384,"ancestor_record_offset":11384,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":11384,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":11608,"parent_record_offset":11584,"ancestor_record_offset":11384,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":11584,"parent_record_offset":11560,"ancestor_record_offset":11384,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":11384,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":11384,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":11804,"parent_record_offset":11780,"ancestor_record_offset":11384,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":11384,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":11848,"parent_record_offset":11804,"ancestor_record_offset":11384,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":12192,"parent_record_offset":11804,"ancestor_record_offset":11384,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":11936,"parent_record_offset":11848,"ancestor_record_offset":11384,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":11908,"parent_record_offset":11848,"ancestor_record_offset":11384,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":11632,"parent_record_offset":11608,"ancestor_record_offset":11384,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/00246.jsonl.gz"}
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
// 首条位置: {"module":2629,"record_offset":10472,"rva":30372928,"va":5399082048}
// 编译实例 RVAs: 0x1cf7440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29299535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399082048,"raw_hex":"e8b1ec40fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5399082154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":10680,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":71777,"source_line":892,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":10652,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":11184,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":10472,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":10824,"parent_record_offset":10800,"ancestor_record_offset":10472,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":11060,"parent_record_offset":11036,"ancestor_record_offset":10472,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":10872,"parent_record_offset":10680,"ancestor_record_offset":10472,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":10472,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":10472,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":10704,"parent_record_offset":10680,"ancestor_record_offset":10472,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":10472,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":10972,"parent_record_offset":10948,"ancestor_record_offset":10472,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":10472,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":11232,"parent_record_offset":11208,"ancestor_record_offset":10472,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":10472,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":11276,"parent_record_offset":11232,"ancestor_record_offset":10472,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":11620,"parent_record_offset":11232,"ancestor_record_offset":10472,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":10472,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":10472,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":11364,"parent_record_offset":11276,"ancestor_record_offset":10472,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":11336,"parent_record_offset":11276,"ancestor_record_offset":10472,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$6::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":2629,"record_offset":11012,"parent_record_offset":10680,"ancestor_record_offset":10472,"inlinee":28927,"source_line":69,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":10948,"parent_record_offset":10896,"ancestor_record_offset":10472,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":10472,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/02629.jsonl.gz"}
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
// 首条位置: {"module":2629,"record_offset":16596,"rva":30373824,"va":5399082944}
// 编译实例 RVAs: 0x1cf77c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29300431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399082944,"raw_hex":"e831e940fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5399083050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":16768,"parent_record_offset":16596,"ancestor_record_offset":16596,"inlinee":71777,"source_line":892,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":16740,"parent_record_offset":16596,"ancestor_record_offset":16596,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":17244,"parent_record_offset":16596,"ancestor_record_offset":16596,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":17124,"parent_record_offset":17100,"ancestor_record_offset":16596,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":16912,"parent_record_offset":16888,"ancestor_record_offset":16596,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":16984,"parent_record_offset":16960,"ancestor_record_offset":16596,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":17100,"parent_record_offset":16768,"ancestor_record_offset":16596,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":16960,"parent_record_offset":16768,"ancestor_record_offset":16596,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":16596,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":16792,"parent_record_offset":16768,"ancestor_record_offset":16596,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":17268,"parent_record_offset":17244,"ancestor_record_offset":16596,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":16596,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":16864,"parent_record_offset":16840,"ancestor_record_offset":16596,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":17292,"parent_record_offset":17268,"ancestor_record_offset":16596,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":16888,"parent_record_offset":16864,"ancestor_record_offset":16596,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":17336,"parent_record_offset":17292,"ancestor_record_offset":16596,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":17680,"parent_record_offset":17292,"ancestor_record_offset":16596,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":17008,"parent_record_offset":16984,"ancestor_record_offset":16596,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":17148,"parent_record_offset":17124,"ancestor_record_offset":16596,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":17424,"parent_record_offset":17336,"ancestor_record_offset":16596,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":17396,"parent_record_offset":17336,"ancestor_record_offset":16596,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":17036,"parent_record_offset":16984,"ancestor_record_offset":16596,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":16840,"parent_record_offset":16816,"ancestor_record_offset":16596,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/02629.jsonl.gz"}
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
// 首条位置: {"module":2629,"record_offset":6468,"rva":30372048,"va":5399081168}
// 编译实例 RVAs: 0x1cf70d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29298655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399081168,"raw_hex":"e821f040fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5399081274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":6636,"parent_record_offset":6468,"ancestor_record_offset":6468,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":6608,"parent_record_offset":6468,"ancestor_record_offset":6468,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":6972,"parent_record_offset":6468,"ancestor_record_offset":6468,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":6468,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":6468,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":6828,"parent_record_offset":6636,"ancestor_record_offset":6468,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":6468,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":6468,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":6996,"parent_record_offset":6972,"ancestor_record_offset":6468,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":6468,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":6468,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":7020,"parent_record_offset":6996,"ancestor_record_offset":6468,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":6468,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":7064,"parent_record_offset":7020,"ancestor_record_offset":6468,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":7408,"parent_record_offset":7020,"ancestor_record_offset":6468,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":6468,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":7152,"parent_record_offset":7064,"ancestor_record_offset":6468,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":7124,"parent_record_offset":7064,"ancestor_record_offset":6468,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":6904,"parent_record_offset":6852,"ancestor_record_offset":6468,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":6468,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/02629.jsonl.gz"}
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
// 首条位置: {"module":2629,"record_offset":14656,"rva":16432256,"va":5385141376}
// 编译实例 RVAs: 0xfabc80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15358875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385141376,"raw_hex":"e865a415ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5385141494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":14856,"parent_record_offset":14656,"ancestor_record_offset":14656,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":14828,"parent_record_offset":14656,"ancestor_record_offset":14656,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":15220,"parent_record_offset":14656,"ancestor_record_offset":14656,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":15072,"parent_record_offset":15048,"ancestor_record_offset":14656,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":14656,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":14928,"parent_record_offset":14904,"ancestor_record_offset":14656,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":14904,"parent_record_offset":14880,"ancestor_record_offset":14656,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":15048,"parent_record_offset":14856,"ancestor_record_offset":14656,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":15244,"parent_record_offset":15220,"ancestor_record_offset":14656,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":15004,"parent_record_offset":14980,"ancestor_record_offset":14656,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":15268,"parent_record_offset":15244,"ancestor_record_offset":14656,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":15312,"parent_record_offset":15268,"ancestor_record_offset":14656,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":15656,"parent_record_offset":15268,"ancestor_record_offset":14656,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":14656,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":15400,"parent_record_offset":15312,"ancestor_record_offset":14656,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":14952,"parent_record_offset":14928,"ancestor_record_offset":14656,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":15372,"parent_record_offset":15312,"ancestor_record_offset":14656,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":14980,"parent_record_offset":14928,"ancestor_record_offset":14656,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::impl$22::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":2629,"record_offset":14880,"parent_record_offset":14856,"ancestor_record_offset":14656,"inlinee":98019,"source_line":322,"archive":"_pdb/B/modules/02629.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":27868,"rva":21344224,"va":5390053344}
// 编译实例 RVAs: 0x145afe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20270841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390053344,"raw_hex":"e807b1cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390053460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":28048,"parent_record_offset":27868,"ancestor_record_offset":27868,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":28020,"parent_record_offset":27868,"ancestor_record_offset":27868,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":28672,"parent_record_offset":27868,"ancestor_record_offset":27868,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":28096,"parent_record_offset":28072,"ancestor_record_offset":27868,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":28072,"parent_record_offset":28048,"ancestor_record_offset":27868,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":28696,"parent_record_offset":28672,"ancestor_record_offset":27868,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":28172,"parent_record_offset":28148,"ancestor_record_offset":27868,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":28720,"parent_record_offset":28696,"ancestor_record_offset":27868,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":28764,"parent_record_offset":28720,"ancestor_record_offset":27868,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":29108,"parent_record_offset":28720,"ancestor_record_offset":27868,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":28120,"parent_record_offset":28096,"ancestor_record_offset":27868,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":28852,"parent_record_offset":28764,"ancestor_record_offset":27868,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":28824,"parent_record_offset":28764,"ancestor_record_offset":27868,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$133::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec3.rs
//     evidence={"module":1606,"record_offset":28212,"parent_record_offset":28048,"ancestor_record_offset":27868,"inlinee":73707,"source_line":19,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":28148,"parent_record_offset":28096,"ancestor_record_offset":27868,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":2629,"record_offset":18620,"rva":30374272,"va":5399083392}
// 编译实例 RVAs: 0x1cf7980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29300890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399083392,"raw_hex":"e866e740fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5399083509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":18796,"parent_record_offset":18620,"ancestor_record_offset":18620,"inlinee":71777,"source_line":892,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":18768,"parent_record_offset":18620,"ancestor_record_offset":18620,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":19276,"parent_record_offset":18620,"ancestor_record_offset":18620,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":18620,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":19224,"parent_record_offset":19196,"ancestor_record_offset":18620,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":18984,"parent_record_offset":18960,"ancestor_record_offset":18620,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":18820,"parent_record_offset":18796,"ancestor_record_offset":18620,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":18960,"parent_record_offset":18796,"ancestor_record_offset":18620,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":19124,"parent_record_offset":19100,"ancestor_record_offset":18620,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":2629,"record_offset":19100,"parent_record_offset":18796,"ancestor_record_offset":18620,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":19300,"parent_record_offset":19276,"ancestor_record_offset":18620,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":18920,"parent_record_offset":18896,"ancestor_record_offset":18620,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":19172,"parent_record_offset":19148,"ancestor_record_offset":18620,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":19324,"parent_record_offset":19300,"ancestor_record_offset":18620,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":19196,"parent_record_offset":19172,"ancestor_record_offset":18620,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":19368,"parent_record_offset":19324,"ancestor_record_offset":18620,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":2629,"record_offset":19712,"parent_record_offset":19324,"ancestor_record_offset":18620,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":19008,"parent_record_offset":18984,"ancestor_record_offset":18620,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":18620,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":19456,"parent_record_offset":19368,"ancestor_record_offset":18620,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2629,"record_offset":19428,"parent_record_offset":19368,"ancestor_record_offset":18620,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":18896,"parent_record_offset":18844,"ancestor_record_offset":18620,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/02629.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2629,"record_offset":19148,"parent_record_offset":19124,"ancestor_record_offset":18620,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/02629.jsonl.gz"}
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
// 首条位置: {"module":246,"record_offset":13132,"rva":6020896,"va":5374730016}
// 编译实例 RVAs: 0x5bdf20, 0x228d080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4947516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374730016,"raw_hex":"e8c481b4ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5374730135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14228ce80
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404938368,"raw_hex":"e894fdffff","size":5,"soft_fail":false,"target_va":5404937856,"va":5404938471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":13308,"parent_record_offset":13132,"ancestor_record_offset":13132,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":13280,"parent_record_offset":13132,"ancestor_record_offset":13132,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":13644,"parent_record_offset":13132,"ancestor_record_offset":13132,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":246,"record_offset":13472,"parent_record_offset":13308,"ancestor_record_offset":13132,"inlinee":10546,"source_line":758,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":13356,"parent_record_offset":13332,"ancestor_record_offset":13132,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":13132,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":13132,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":13432,"parent_record_offset":13408,"ancestor_record_offset":13132,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":13132,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":13736,"parent_record_offset":13692,"ancestor_record_offset":13132,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":14080,"parent_record_offset":13692,"ancestor_record_offset":13132,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":13824,"parent_record_offset":13736,"ancestor_record_offset":13132,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":13380,"parent_record_offset":13356,"ancestor_record_offset":13132,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":13796,"parent_record_offset":13736,"ancestor_record_offset":13132,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":13408,"parent_record_offset":13356,"ancestor_record_offset":13132,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00246.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":9248,"rva":21341088,"va":5390050208}
// 编译实例 RVAs: 0x145a3a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20267708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390050208,"raw_hex":"e844bdcafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390050327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0d1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9753178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390050208,"raw_hex":"e8a62d6bff","size":5,"soft_fail":false,"target_va":5380297200,"va":5390050373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":9456,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":9428,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":9824,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":9672,"parent_record_offset":9644,"ancestor_record_offset":9248,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":9248,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9248,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":9248,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":9248,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":9620,"parent_record_offset":9456,"ancestor_record_offset":9248,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":9248,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":9580,"parent_record_offset":9556,"ancestor_record_offset":9248,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":9872,"parent_record_offset":9848,"ancestor_record_offset":9248,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":9916,"parent_record_offset":9872,"ancestor_record_offset":9248,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":10260,"parent_record_offset":9872,"ancestor_record_offset":9248,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":9248,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":10004,"parent_record_offset":9916,"ancestor_record_offset":9248,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":9248,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":9976,"parent_record_offset":9916,"ancestor_record_offset":9248,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":9556,"parent_record_offset":9504,"ancestor_record_offset":9248,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":25616,"rva":21343712,"va":5390052832}
// 编译实例 RVAs: 0x145ade0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20270335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390052832,"raw_hex":"e801b3cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390052954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":25812,"parent_record_offset":25616,"ancestor_record_offset":25616,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":25784,"parent_record_offset":25616,"ancestor_record_offset":25616,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":26492,"parent_record_offset":25616,"ancestor_record_offset":25616,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":26168,"parent_record_offset":26000,"ancestor_record_offset":25616,"inlinee":31385,"source_line":235,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":26028,"parent_record_offset":26000,"ancestor_record_offset":25616,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":26328,"parent_record_offset":26304,"ancestor_record_offset":25616,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":25860,"parent_record_offset":25836,"ancestor_record_offset":25616,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":26052,"parent_record_offset":26028,"ancestor_record_offset":25616,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":26304,"parent_record_offset":26276,"ancestor_record_offset":25616,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":25836,"parent_record_offset":25812,"ancestor_record_offset":25616,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":26000,"parent_record_offset":25976,"ancestor_record_offset":25616,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":25976,"parent_record_offset":25812,"ancestor_record_offset":25616,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":26516,"parent_record_offset":26492,"ancestor_record_offset":25616,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":25936,"parent_record_offset":25912,"ancestor_record_offset":25616,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":26540,"parent_record_offset":26516,"ancestor_record_offset":25616,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":26584,"parent_record_offset":26540,"ancestor_record_offset":25616,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":26928,"parent_record_offset":26540,"ancestor_record_offset":25616,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":26076,"parent_record_offset":26052,"ancestor_record_offset":25616,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":26352,"parent_record_offset":26328,"ancestor_record_offset":25616,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":26672,"parent_record_offset":26584,"ancestor_record_offset":25616,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":25884,"parent_record_offset":25860,"ancestor_record_offset":25616,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":26644,"parent_record_offset":26584,"ancestor_record_offset":25616,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1606,"record_offset":26240,"parent_record_offset":26200,"ancestor_record_offset":25616,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":26200,"parent_record_offset":26168,"ancestor_record_offset":25616,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$131::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":1606,"record_offset":26276,"parent_record_offset":26168,"ancestor_record_offset":25616,"inlinee":32984,"source_line":17,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":25912,"parent_record_offset":25860,"ancestor_record_offset":25616,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":6908,"rva":21340560,"va":5390049680}
// 编译实例 RVAs: 0x145a190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=5
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20267182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390049680,"raw_hex":"e852bfcafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390049801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7104,"parent_record_offset":6908,"ancestor_record_offset":6908,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":7076,"parent_record_offset":6908,"ancestor_record_offset":6908,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":7872,"parent_record_offset":6908,"ancestor_record_offset":6908,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7460,"parent_record_offset":7292,"ancestor_record_offset":6908,"inlinee":31385,"source_line":235,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7320,"parent_record_offset":7292,"ancestor_record_offset":6908,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7624,"parent_record_offset":7600,"ancestor_record_offset":6908,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":6908,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":6908,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7600,"parent_record_offset":7568,"ancestor_record_offset":6908,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":6908,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7292,"parent_record_offset":7268,"ancestor_record_offset":6908,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":7268,"parent_record_offset":7104,"ancestor_record_offset":6908,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":6908,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":6908,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":6908,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":7964,"parent_record_offset":7920,"ancestor_record_offset":6908,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":8308,"parent_record_offset":7920,"ancestor_record_offset":6908,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":6908,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":7648,"parent_record_offset":7624,"ancestor_record_offset":6908,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":8052,"parent_record_offset":7964,"ancestor_record_offset":6908,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":6908,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":8024,"parent_record_offset":7964,"ancestor_record_offset":6908,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1606,"record_offset":7532,"parent_record_offset":7492,"ancestor_record_offset":6908,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1606,"record_offset":7492,"parent_record_offset":7460,"ancestor_record_offset":6908,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$133::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec3.rs
//     evidence={"module":1606,"record_offset":7568,"parent_record_offset":7460,"ancestor_record_offset":6908,"inlinee":73707,"source_line":19,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":7204,"parent_record_offset":7152,"ancestor_record_offset":6908,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":17464,"rva":21342432,"va":5390051552}
// 编译实例 RVAs: 0x145a8e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20269052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390051552,"raw_hex":"e804b8cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390051671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":17648,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":17620,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":18068,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":17908,"parent_record_offset":17884,"ancestor_record_offset":17464,"inlinee":60626,"source_line":418,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":17464,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":17672,"parent_record_offset":17648,"ancestor_record_offset":17464,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$17::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":17884,"parent_record_offset":17860,"ancestor_record_offset":17464,"inlinee":73709,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":17812,"parent_record_offset":17648,"ancestor_record_offset":17464,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":18092,"parent_record_offset":18068,"ancestor_record_offset":17464,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":17772,"parent_record_offset":17748,"ancestor_record_offset":17464,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":17464,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":18160,"parent_record_offset":18116,"ancestor_record_offset":17464,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":18504,"parent_record_offset":18116,"ancestor_record_offset":17464,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":18248,"parent_record_offset":18160,"ancestor_record_offset":17464,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":17464,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":18220,"parent_record_offset":18160,"ancestor_record_offset":17464,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":17748,"parent_record_offset":17696,"ancestor_record_offset":17464,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":1606,"record_offset":17860,"parent_record_offset":17836,"ancestor_record_offset":17464,"inlinee":60625,"source_line":148,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$33::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":1606,"record_offset":17836,"parent_record_offset":17812,"ancestor_record_offset":17464,"inlinee":60623,"source_line":242,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":1606,"record_offset":21536,"rva":21342432,"va":5390051552}
// 编译实例 RVAs: 0x145a8e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20269052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390051552,"raw_hex":"e804b8cafe","size":5,"soft_fail":false,"target_va":5369782624,"va":5390051671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":21704,"parent_record_offset":21536,"ancestor_record_offset":21536,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":21676,"parent_record_offset":21536,"ancestor_record_offset":21536,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":22040,"parent_record_offset":21536,"ancestor_record_offset":21536,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":21752,"parent_record_offset":21728,"ancestor_record_offset":21536,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":21728,"parent_record_offset":21704,"ancestor_record_offset":21536,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":21868,"parent_record_offset":21704,"ancestor_record_offset":21536,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":22064,"parent_record_offset":22040,"ancestor_record_offset":21536,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":21828,"parent_record_offset":21804,"ancestor_record_offset":21536,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":22088,"parent_record_offset":22064,"ancestor_record_offset":21536,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":22132,"parent_record_offset":22088,"ancestor_record_offset":21536,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":22476,"parent_record_offset":22088,"ancestor_record_offset":21536,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":22220,"parent_record_offset":22132,"ancestor_record_offset":21536,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":21776,"parent_record_offset":21752,"ancestor_record_offset":21536,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":22192,"parent_record_offset":22132,"ancestor_record_offset":21536,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":21804,"parent_record_offset":21752,"ancestor_record_offset":21536,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 首条位置: {"module":246,"record_offset":5792,"rva":6019344,"va":5374728464}
// 编译实例 RVAs: 0x5bd910, 0x228d1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=6
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4945964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374728464,"raw_hex":"e8d487b4ff","size":5,"soft_fail":false,"target_va":5369782624,"va":5374728583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14228ce80
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404938736,"raw_hex":"e824fcffff","size":5,"soft_fail":false,"target_va":5404937856,"va":5404938839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":5952,"parent_record_offset":5792,"ancestor_record_offset":5792,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":5924,"parent_record_offset":5792,"ancestor_record_offset":5792,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":6260,"parent_record_offset":5792,"ancestor_record_offset":5792,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":6000,"parent_record_offset":5976,"ancestor_record_offset":5792,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":246,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":5792,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":6284,"parent_record_offset":6260,"ancestor_record_offset":5792,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":5792,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":5792,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":6352,"parent_record_offset":6308,"ancestor_record_offset":5792,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":246,"record_offset":6696,"parent_record_offset":6308,"ancestor_record_offset":5792,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=92
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":6440,"parent_record_offset":6352,"ancestor_record_offset":5792,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":5792,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=60
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":6412,"parent_record_offset":6352,"ancestor_record_offset":5792,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":246,"record_offset":6052,"parent_record_offset":6000,"ancestor_record_offset":5792,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00246.jsonl.gz"}
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
// 首条位置: {"module":1540,"record_offset":264,"rva":19450080,"va":5388159200}
// 编译实例 RVAs: 0x128c8e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18376700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388159200,"raw_hex":"e80498e7fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5388159319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":424,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":396,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":704,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":264,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":264,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":448,"parent_record_offset":424,"ancestor_record_offset":264,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1540,"record_offset":588,"parent_record_offset":424,"ancestor_record_offset":264,"inlinee":21871,"source_line":811,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":264,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":264,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":264,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":796,"parent_record_offset":752,"ancestor_record_offset":264,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1540,"record_offset":1140,"parent_record_offset":752,"ancestor_record_offset":264,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":264,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":884,"parent_record_offset":796,"ancestor_record_offset":264,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1540,"record_offset":856,"parent_record_offset":796,"ancestor_record_offset":264,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01540.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1540,"record_offset":524,"parent_record_offset":472,"ancestor_record_offset":264,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01540.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":14316,"rva":13179056,"va":5381888176}
// 编译实例 RVAs: 0xc918b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":14412,"parent_record_offset":14316,"ancestor_record_offset":14316,"inlinee":44713,"source_line":266,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::srgb_oetf
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 560
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEBB
// 首条位置: {"module":759,"inlinee":44731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":14764,"parent_record_offset":14732,"ancestor_record_offset":14732,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::srgb_eotf
// 记录形式: inlinee_declaration; instances=1
// 源码行: 568
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEBA
// 首条位置: {"module":759,"inlinee":44730}
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
// 首条位置: {"module":759,"record_offset":14444,"rva":13179088,"va":5381888208}
// 编译实例 RVAs: 0xc918d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::scale_u32_packed_srgba_color::scale_linear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":14564,"parent_record_offset":14444,"ancestor_record_offset":14444,"inlinee":44727,"source_line":581,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":759,"record_offset":14584,"parent_record_offset":14564,"ancestor_record_offset":14444,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color::scale_linear
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 581
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEB7
// 首条位置: {"module":759,"inlinee":44727}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":759,"record_offset":14584,"parent_record_offset":14564,"ancestor_record_offset":14564,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color_premultiplied::scale_srgb
// 记录形式: inlinee_declaration; instances=1
// 源码行: 598
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAEB9
// 首条位置: {"module":759,"inlinee":44729}
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
// IPI IDs: 0xAEBC
// 首条位置: {"module":759,"inlinee":44732}
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
// 首条位置: {"module":759,"record_offset":14620,"rva":13179184,"va":5381888304}
// 编译实例 RVAs: 0xc91930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f70
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29959662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381888304,"raw_hex":"e8ee25c901","size":5,"soft_fail":false,"target_va":5411848048,"va":5381888381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":759,"record_offset":14924,"parent_record_offset":14620,"ancestor_record_offset":14620,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] utils::srgb_oetf
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":759,"record_offset":14732,"parent_record_offset":14620,"ancestor_record_offset":14620,"inlinee":44731,"source_line":560,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":14764,"parent_record_offset":14732,"ancestor_record_offset":14620,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$9::iterator_slice_range
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 661
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11554
// 首条位置: {"module":1449,"inlinee":70996}
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
// IPI IDs: 0x11552
// 首条位置: {"module":1449,"inlinee":70994}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$9::iterator_slice_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1449,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":2472,"inlinee":70996,"source_line":661,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$19::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 682
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x621C
// 首条位置: {"module":430,"inlinee":25116}
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
// 首条位置: {"module":759,"record_offset":15024,"rva":13179696,"va":5381888816}
// 编译实例 RVAs: 0xc91b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":759,"record_offset":15148,"parent_record_offset":15024,"ancestor_record_offset":15024,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":15188,"parent_record_offset":15168,"ancestor_record_offset":15024,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":15168,"parent_record_offset":15148,"ancestor_record_offset":15024,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":15224,"rva":13179744,"va":5381888864}
// 编译实例 RVAs: 0xc91b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":759,"record_offset":15356,"parent_record_offset":15224,"ancestor_record_offset":15224,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":15396,"parent_record_offset":15376,"ancestor_record_offset":15224,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":15376,"parent_record_offset":15356,"ancestor_record_offset":15224,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":15432,"rva":13179776,"va":5381888896}
// 编译实例 RVAs: 0xc91b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":15556,"parent_record_offset":15432,"ancestor_record_offset":15432,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":16360,"parent_record_offset":15432,"ancestor_record_offset":15432,"inlinee":13688,"source_line":894,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":16304,"parent_record_offset":15432,"ancestor_record_offset":15432,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":15972,"parent_record_offset":15556,"ancestor_record_offset":15432,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":15880,"parent_record_offset":15824,"ancestor_record_offset":15432,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":15648,"parent_record_offset":15624,"ancestor_record_offset":15432,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":15680,"parent_record_offset":15600,"ancestor_record_offset":15432,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":759,"record_offset":15744,"parent_record_offset":15556,"ancestor_record_offset":15432,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":16384,"parent_record_offset":16360,"ancestor_record_offset":15432,"inlinee":13682,"source_line":727,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":759,"record_offset":16000,"parent_record_offset":15556,"ancestor_record_offset":15432,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":15432,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":759,"record_offset":15600,"parent_record_offset":15556,"ancestor_record_offset":15432,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 首条位置: {"module":759,"record_offset":16964,"rva":13180016,"va":5381889136}
// 编译实例 RVAs: 0xc91c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29958548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381889136,"raw_hex":"e89421c901","size":5,"soft_fail":false,"target_va":5411847808,"va":5381889255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29958872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381889136,"raw_hex":"e8d822c901","size":5,"soft_fail":false,"target_va":5411848144,"va":5381889267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t120535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381889136,"raw_hex":"e8d7d60100","size":5,"soft_fail":false,"target_va":5382009728,"va":5381889188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":759,"record_offset":17232,"parent_record_offset":16964,"ancestor_record_offset":16964,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":17092,"parent_record_offset":16964,"ancestor_record_offset":16964,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":17120,"parent_record_offset":16964,"ancestor_record_offset":16964,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00759.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":759,"record_offset":17204,"parent_record_offset":16964,"ancestor_record_offset":16964,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00759.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "constructor": "tuple$", "arguments": ["utils::IsoTransform", "ref$<country_core::systems::collision::world::RaycastMeshData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> >", "constructor": "country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >", "constructor": "country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> >", "constructor": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure$0", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >", "constructor": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >,tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >", "tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >", "constructor": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > >", "constructor": "rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > > >", "constructor": "ref_mut$", "arguments": ["rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > > > >", "constructor": "ref_mut$", "arguments": ["ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > > > > >", "constructor": "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor", "arguments": ["rapier3d::geometry::collider_components::ColliderHandle", "ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node<rapier3d::geometry::collider_components::ColliderHandle,parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > > > > > >", "constructor": "parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node", "arguments": ["rapier3d::geometry::collider_components::ColliderHandle", "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> >", "constructor": "country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1", "arguments": ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > >", "constructor": "rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > >", "constructor": "ref_mut$", "arguments": ["rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > > >", "constructor": "ref_mut$", "arguments": ["ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > > > >", "constructor": "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor", "arguments": ["rapier3d::geometry::collider_components::ColliderHandle", "ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node<rapier3d::geometry::collider_components::ColliderHandle,parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > > > > >", "constructor": "parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node", "arguments": ["rapier3d::geometry::collider_components::ColliderHandle", "parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > > >", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >", "ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rapier3d::pipeline::query_pipeline::QueryPipeline::intersections_with_shape<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > >", "constructor": "rapier3d::pipeline::query_pipeline::QueryPipeline::intersections_with_shape", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rapier3d::pipeline::query_pipeline::QueryPipeline::intersections_with_shape<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> > >", "constructor": "rapier3d::pipeline::query_pipeline::QueryPipeline::intersections_with_shape", "arguments": ["country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform>", "constructor": "country_core::systems::collision::world::RaycastWorld::upsert_collider", "arguments": ["country_core::resources::stairs::stairs_state::StairGraphEdge", "utils::IsoTransform"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform>", "constructor": "country_core::systems::collision::world::RaycastWorld::upsert_collider", "arguments": ["tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>", "utils::IsoTransform"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [已有模块证据关联；不自动成为字段事实] evidence/building-platform-stairs/B-raycast-extra/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::IsoTransform", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 53; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::reader::EventReader<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_raise_shape::RaiseShape>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::reader::EventReader<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_raise_shape::RaiseShape>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_raise_shape::RaiseShape>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::writer::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: time::formatting::formattable::impl$8::format_into<alloc::vec::Vec<u8,alloc::alloc::Global> >
// 信息 ID: 68f97fed70177301069f032e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] time::formatting::formattable::impl$8::format_into -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] time::formatting::formattable::sealed::Sealed::format<alloc::vec::Vec<enum2$<time::format_description::borrowed_format_item::BorrowedFormatItem>,alloc::alloc::Global> > @ 0x1941896 -> 0x21806f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::IsoTransform::from_translation
// 信息 ID: 4232de54d5746bd4ce95361f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xe1f5 -> 0xc91b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1d62b -> 0xc91b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::create_structural_colliders_and_initial_graph @ 0x7d2b44 -> 0xc91b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::create_structural_colliders_and_initial_graph @ 0x7d2eac -> 0xc91b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs::generate_playermade_stair_decorators @ 0x804da0 -> 0xc91b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_cuboid @ 0x8d0073 -> 0xc91b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_sphere @ 0x8d0181 -> 0xc91b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d0c4e -> 0xc91b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_subtype::DecoratorCollision::empty @ 0xb25d6d -> 0xc91b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::IsoTransform::from_translation_rotation
// 信息 ID: ed5ee5a7e3fd13bf85084a04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::roofs::roof_shape::roof_uv_and_point_at_ws_pos @ 0x8e4358 -> 0xc91b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$21::to_offset_collider_builder @ 0xb26ac0 -> 0xc91b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$22::to_offset_collider_builder @ 0xb26c14 -> 0xc91b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xb40bf1 -> 0xc91b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::IsoTransform::nested_in
// 信息 ID: 205caa4d1c1c863c34ebaab8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::impl$21::to_offset_collider_builder @ 0xb26ad0 -> 0xc91b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$22::to_offset_collider_builder @ 0xb26c24 -> 0xc91b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$45::into_transformed_shape @ 0xb26d36 -> 0xc91b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xb40c0f -> 0xc91b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<array$<u32,8> >
// 信息 ID: 682b02629634bb46b47d6b04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "8"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["array$<u32,8>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_compound @ 0x8d025e -> 0x145be20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<country_core::resources::stairs::stairs_state::StairsSnapshot>
// 信息 ID: 8b8ea482a84a3c852367bb2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["country_core::resources::stairs::stairs_state::StairsSnapshot"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::stairs::stairs_state::StairsState::compute_checksum @ 0x8ff4eb -> 0x145b330 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::stairs::stairs_state::StairsState::restore_snapshot @ 0x8ff5da -> 0x145b330 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::stairs::stairs_state::StairsState::extract_diff_and_make_backup @ 0x9003fe -> 0x145b330 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::stairs::stairs_state::StairsSnapshot::new @ 0x90116b -> 0x145b330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<enum2$<core::option::Option<ref$<std::ffi::os_str::OsStr> > > >
// 信息 ID: 1627681dd9a3dc1abc1290f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::ffi::os_str::OsStr"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<std::ffi::os_str::OsStr>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<std::ffi::os_str::OsStr> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["enum2$<core::option::Option<ref$<std::ffi::os_str::OsStr> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x1673175 -> 0x145aa70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<glam::i32::ivec3::IVec3>
// 信息 ID: dc19271cb1b59564cecf191b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["glam::i32::ivec3::IVec3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections @ 0xa991e7 -> 0x145c1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d786f -> 0x145c1a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<slice2$<array$<u32,2> > > >
// 信息 ID: e71b811a572aa8abd7e6f415
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<u32,2>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<u32,2> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<slice2$<array$<u32,2> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0xeebbfa -> 0x1cf7270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0xeebc5d -> 0x1cf7270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<slice2$<glam::u32::uvec3::UVec3> > >
// 信息 ID: 7440d92ccfe9259b3f418bfe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<slice2$<glam::u32::uvec3::UVec3> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_convex @ 0x8cec45 -> 0x145bfb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<slice2$<u8> > >
// 信息 ID: 08ad603a7bd56600f17f6b7e
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
// 信息 ID: 34c51f696abf4f639e32e572
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 201 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<alloc::string::String> @ 0x587723 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x587f32 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::impl$0::as_id @ 0x683c10 -> 0x5bdda0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::label @ 0x68bf26 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::menu_button @ 0x68bfe6 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::menu_button_no_nav @ 0x68c286 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::settings_button @ 0x68c7b6 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$15::toggle_icon @ 0x68cc21 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::instantiate_ui_template_impl @ 0x68ebb3 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::instantiate_ui_template_impl @ 0x68f638 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> > @ 0x8a431a -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String> | country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String> | country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String> @ 0x923238 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > | country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > | country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > @ 0x923705 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::WidgetId::from_name | toolbar::mei::core::impl$1::as_id @ 0x92df42 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound @ 0x92dff8 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::mode_manager::mode_manager @ 0xa86b9a -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<rhapsody::raw::texture::TextureViewDesc>
// 信息 ID: a26158110c6cd15752aa4f39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["rhapsody::raw::texture::TextureViewDesc"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::TextureBindingKey::with_view @ 0x867950 -> 0x145baf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x8eea35 -> 0x145baf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset> @ 0x908de6 -> 0x145baf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > > @ 0x96f885 -> 0x145baf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType>
// 信息 ID: 5a6d1c6e21d1361afe901710
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x505ae8 -> 0x5bdaa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<country_core::resources::walls::WallId,2>,usize> >
// 信息 ID: c8726823c672fbe2406920ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["country_core::resources::walls::WallId", "2"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<country_core::resources::walls::WallId,2>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<country_core::resources::walls::WallId,2>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitch_bricks @ 0x1ce76e6 -> 0x1cf6f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<u32,2>,array$<u32,2>,country_core::resources::walls::wall_holes::HoleType> >
// 信息 ID: b8adadfe05775c8cbbea485c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<u32,2>", "array$<u32,2>", "country_core::resources::walls::wall_holes::HoleType"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<u32,2>,array$<u32,2>,country_core::resources::walls::wall_holes::HoleType>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoles::add @ 0xf865f2 -> 0x145b910 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_holes::InProgressHoleStorage::add @ 0xf87884 -> 0x145b910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<u32,3>,glam::u32::uvec3::UVec3> >
// 信息 ID: f802bffad759e9c79b45028f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "3"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<u32,3>", "glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<u32,3>,glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_cuboid @ 0x8d002f -> 0x145a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d0c02 -> 0x145a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_subtype::DecoratorCollision::empty @ 0xb25d3a -> 0x145a540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::components::TreeStorageIndex,usize,ref$<str$> > >
// 信息 ID: d5038cbc496105f6d15bc6ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::components::TreeStorageIndex", "usize", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::components::TreeStorageIndex,usize,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x22d3f9d -> 0x1cf7600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
// 信息 ID: e4dfb9c36ce7dfecfcc88947
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::resources::walls::decorator::DecoratorId", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ee723 -> 0x128cc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ee8c4 -> 0x128cc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f4f21 -> 0x128cc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f5069 -> 0x128cc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter @ 0x7f5faf -> 0x128cc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter @ 0x7f60f3 -> 0x128cc30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize> >
// 信息 ID: 9837158ee2fdeff64e957307
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::resources::walls::decorator::DecoratorId", "ref$<str$>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c9835 -> 0x128ca70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ee561 -> 0x128ca70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f8171 -> 0x128ca70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType> >
// 信息 ID: 4dd35c85cade3a82f685bd71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["country_core::systems::ui::GlobalId", "country_core::input::action_set::DigitalActionSetActionType"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::MenuButtonView::hotkey @ 0x67e716 -> 0xfabaf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::PillButtonView::hotkey @ 0x681703 -> 0xfabaf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,u32> >
// 信息 ID: 1d24ada3c4c385624a54af34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["country_core::systems::ui::GlobalId", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::systems::ui::GlobalId,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$0::decorate_hovered @ 0x67e020 -> 0xfabc80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d7dc8 -> 0xfabc80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$> > > > >
// 信息 ID: b80aa36cb1c1c5ec112681e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<str$>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<str$> >"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::systems::ui::resources::ui_state::WidgetId", "enum2$<core::option::Option<ref$<str$> > >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete @ 0x92945c -> 0x145a710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::signifier_create::signifier_create @ 0xb2d699 -> 0x145a710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<i32,tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType> >
// 信息 ID: 903234db91049c3f8c7ac065
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["i32", "tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<i32,tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x505d07 -> 0x5bdc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x506693 -> 0x5bdc10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<slice2$<array$<u32,3> > >,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 信息 ID: e19885542c0dc1326bbfc3ad
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
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x8ceebd -> 0x145b6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > > >
// 信息 ID: 668b53be2f744b84a9c78290
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
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_unindexed @ 0x8cf855 -> 0x145b4e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,i32,usize,u32> >
// 信息 ID: 135b6dc9316e38bc3186a310
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "i32", "usize", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,i32,usize,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows @ 0x22cf671 -> 0x1cf6d60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur> > >
// 信息 ID: 8515435074d1d3842d11b948
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "ref$<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur> @ 0x1602a04 -> 0x1459fc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::taa::TgsrMain> > >
// 信息 ID: b35d368af5394c1d7a667942
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["country_core::systems::render_loop::compute_passes::taa::TgsrMain"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "ref$<country_core::systems::render_loop::compute_passes::taa::TgsrMain>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::taa::TgsrMain> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::taa::TgsrMain> @ 0x1600eeb -> 0x145ac20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::vermeer::GenerateStrokes> > >
// 信息 ID: 520689a28b8c26fd61ded363
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["country_core::systems::render_loop::vermeer::GenerateStrokes"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "ref$<country_core::systems::render_loop::vermeer::GenerateStrokes>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,ref$<country_core::systems::render_loop::vermeer::GenerateStrokes> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash @ 0xa5d4e3 -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash @ 0xa5d983 -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash @ 0xa5de23 -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketFlowers> @ 0x15fcf8f -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0x15fd89f -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::bin_lights::BinLights> @ 0x15fe1af -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0x15feabf -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketMain> @ 0x15ff3cf -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0x15ffcdf -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketBush> @ 0x16017ef -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank> @ 0x16020ff -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> @ 0x1603c2f -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$30::source_hash @ 0x1624793 -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$34::source_hash @ 0x1624c33 -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$38::source_hash @ 0x16250e3 -> 0x1459e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$42::source_hash @ 0x1625583 -> 0x1459e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,ref$<tuple$<> > > >
// 信息 ID: f422c16954197f5c4f706838
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] ref$ -> ["tuple$<>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "ref$<tuple$<> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,ref$<tuple$<> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 201 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<alloc::string::String> @ 0x587723 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x587f32 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::impl$0::as_id @ 0x683c10 -> 0x5bdda0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::label @ 0x68bf26 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::menu_button @ 0x68bfe6 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::menu_button_no_nav @ 0x68c286 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::settings_button @ 0x68c7b6 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$15::toggle_icon @ 0x68cc21 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::instantiate_ui_template_impl @ 0x68ebb3 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::instantiate_ui_template_impl @ 0x68f638 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> > @ 0x8a431a -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String> | country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String> | country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String> @ 0x923238 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > | country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > | country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > @ 0x923705 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::WidgetId::from_name | toolbar::mei::core::impl$1::as_id @ 0x92df42 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound @ 0x92dff8 -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::mode_manager::mode_manager @ 0xa86b9a -> 0x5bdda0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,u32,country_core::resources::walls::WallId> >
// 信息 ID: 68cf0b77e116bc5abe773976
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "u32", "country_core::resources::walls::WallId"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,u32,country_core::resources::walls::WallId>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates @ 0x22cc99c -> 0x1cf7440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,usize,u32> >
// 信息 ID: a1461fe0c40f63c39077403f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "usize", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,usize,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows @ 0x22cf961 -> 0x1cf77c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows @ 0x22cfb6b -> 0x1cf77c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,usize> >
// 信息 ID: 706e18c6c105a15ba2226f16
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter @ 0x1ce5d8a -> 0x1cf70d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<system_clutter::autoclutter::ClutterRegionId,i32> >
// 信息 ID: d40785fbbe212e0d9cbda7e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["system_clutter::autoclutter::ClutterRegionId", "i32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<system_clutter::autoclutter::ClutterRegionId,i32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$0::decorate_hovered @ 0x67e020 -> 0xfabc80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d7dc8 -> 0xfabc80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u32,glam::u32::uvec3::UVec3> >
// 信息 ID: ba284802ce08bd7b2fd4f811
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u32", "glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u32,glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_sphere @ 0x8d015b -> 0x145afe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u32,usize,ref$<str$> > >
// 信息 ID: 90cad6eef85de6af627a72ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u32", "usize", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u32,usize,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing @ 0x22d3644 -> 0x1cf7980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,core::any::TypeId> >
// 信息 ID: c3efb6d9b5305fcf43bc21e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u64", "core::any::TypeId"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,core::any::TypeId>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 326 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x86ab7 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x150f63 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151056 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151136 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x15121e -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x24671c -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_controls @ 0x24b1a7 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::child<ref$<str$> > @ 0x32db3d -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::tab_switcher<ref$<str$> > @ 0x32dd3c -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::image<u64> @ 0x32dfb9 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::image<ref$<str$> > @ 0x32e119 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::impl$18::template_with_id<ref$<str$> > @ 0x32e3b1 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$52::pill_button<ref$<str$> > @ 0x32e62e -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> > @ 0x32e973 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x32ed06 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f391 -> 0x5bdf20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > > > >
// 信息 ID: 270d63e0e2ddb61eed58b82f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["glam::u32::uvec2::UVec2", "3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<glam::u32::uvec2::UVec2,3>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<glam::u32::uvec2::UVec2,3> >"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_unindexed @ 0x8cf894 -> 0x145a3a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec2::UVec2> > > >
// 信息 ID: 3e1954a23eeee89e083d5b5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec2::UVec2"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec2::UVec2>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<glam::u32::uvec2::UVec2> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<glam::u32::uvec2::UVec2> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x8cef46 -> 0x145ade0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 信息 ID: b0251a670ed56060cea33d41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<glam::u32::uvec3::UVec3> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<glam::u32::uvec3::UVec3> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x8cef00 -> 0x145a190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<std::time::SystemTime> > >
// 信息 ID: 1d0a3e8654f39be11801a074
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::time::SystemTime"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<std::time::SystemTime>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<std::time::SystemTime> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x167321f -> 0x145a8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x167328d -> 0x145a8e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<u64> > >
// 信息 ID: 937c99cf55c7fbe3cad3b948
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["u64"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<u64>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<u64> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x167321f -> 0x145a8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity @ 0x167328d -> 0x145a8e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,u64> >
// 信息 ID: 764e34ec44f3dd1cac8926c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u64", "u64"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,u64>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 330 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x86ad4 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x150f77 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x15106a -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x15114a -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151232 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x246736 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_controls @ 0x24b1b8 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::child<ref$<str$> > @ 0x32db51 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::tab_switcher<ref$<str$> > @ 0x32dd54 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::image<u64> @ 0x32dfd1 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::image<ref$<str$> > @ 0x32e131 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::impl$18::template_with_id<ref$<str$> > @ 0x32e3c2 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$52::pill_button<ref$<str$> > @ 0x32e643 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> > @ 0x32e98b -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x32ed17 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f3a2 -> 0x5bd910 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<usize,u8> >
// 信息 ID: d6a7823ae05739fccc57a889
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["usize", "u8"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<usize,u8>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<system_decorator::decorator_resources::DecoratorHandleName> @ 0x125d1b9 -> 0x128c8e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::cartesian_to_polar
// 信息 ID: 2f3ef1bee2c0cb91b83294b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad140 -> 0xc90e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad15d -> 0xc90e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad666 -> 0xc90e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad206 -> 0xc90e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad90c -> 0xc90e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::color_menu::color_menu @ 0x2294f8d -> 0xc90e20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_closest_u_from_pos; module=767, inline_record=1808; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::cos_angle_between
// 信息 ID: cb080893c23f3b522f713dc9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1434, inline_record=1112; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1434, inline_record=2484; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle; module=3027, inline_record=1916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_line_point
// 信息 ID: ffee52333e70a8e69d805e9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=5236; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=567, inline_record=90916; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3036, inline_record=4052; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3036, inline_record=4888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_segment_point
// 信息 ID: d1e057bef81a6599acad0135
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=519, inline_record=98532; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=768, inline_record=29536; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=6256; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::is_on_right_side<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1607, inline_record=48076; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>; module=4438, inline_record=3340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_squared_segment_point
// 信息 ID: 846970da7b7bfbc53ee3da67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions; module=3811, inline_record=32040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::fit
// 信息 ID: 170a39b4c4b9ad70ff5acb71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x506624 -> 0xc918b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x5063e9 -> 0xc918b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x505c7d -> 0xc918b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x5059b5 -> 0xc918b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x505a71 -> 0xc918b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::remap_hour_to_fmod_hour @ 0xa74a84 -> 0xc918b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect
// 信息 ID: 284ad07436da87895061d865
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::intersect::Triangle::intersect_triangle @ 0x123af88 -> 0x1fdfd80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::golden_spiral
// 信息 ID: ea772885e54b283f0b1635c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::spawn_ducks::spawn_ducks @ 0xf8f5a1 -> 0xc90940 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x2310329 -> 0xc90940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::dot
// 信息 ID: d96e79c5d02381c96ab6a6e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 32 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::closest_segment_index; module=512, inline_record=45236; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::closest_segment_index; module=512, inline_record=45292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment; module=512, inline_record=49068; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=512, inline_record=52064; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=512, inline_record=52128; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=25284; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=25340; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=26196; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::length
// 信息 ID: 638e371a92ad310ee8c79cba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 894525e2ee643a10ae49e3ac: glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=60, inline_record=19852; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=60, inline_record=20360; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=60, inline_record=20636; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=60, inline_record=21152; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=4212; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=5692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=567, inline_record=91372; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=768, inline_record=30168; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::length_squared
// 信息 ID: 07017ff61dac9ad7197cf035
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions; module=3811, inline_record=32688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::lerp
// 信息 ID: 498d2b7f57ef015040849fc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 67 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::Curve::try_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=60, inline_record=25720; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=397, inline_record=4284; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=23124; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=23400; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited; module=465, inline_record=4904; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=512, inline_record=37640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal; module=512, inline_record=46932; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment; module=512, inline_record=49188; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::normalize
// 信息 ID: 10eb3aa5939c9f5cfd968573
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones; module=441, inline_record=23752; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones; module=441, inline_record=24252; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers; module=497, inline_record=54588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal; module=512, inline_record=46380; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=512, inline_record=56360; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls; module=519, inline_record=26348; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; module=578, inline_record=151504; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ivy::wall_coord::WallCoord::into_world_space; module=579, inline_record=34928; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$19::clone
// 信息 ID: 7c7dea413fdbbd26efb5d610
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::trap_doors::generate_trap_doors; module=430, inline_record=11704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::dot
// 信息 ID: 3cf417c53b2ec58616e37149
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=20256; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=20320; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_stairs_viz::insert_point_ui; module=417, inline_record=88920; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_stairs_viz::insert_point_ui; module=417, inline_record=88976; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=419, inline_record=14932; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=419, inline_record=14996; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=519, inline_record=98816; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=519, inline_record=98872; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::length
// 信息 ID: 9cac7be2b9ec709013e45dcf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=60, inline_record=5900; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=60, inline_record=6440; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=60, inline_record=6748; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=60, inline_record=7296; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=60, inline_record=12212; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::smooth<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=60, inline_record=23652; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=519, inline_record=99388; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::crashy_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=1407, inline_record=17948; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::length_squared
// 信息 ID: ab7618384b857c69c3a4e05d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3036, inline_record=5652; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3036, inline_record=5824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::lerp
// 信息 ID: 06c4c1f55515a0b6ee7312af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 51 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=20408; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=406, inline_record=18684; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=406, inline_record=20536; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs; module=417, inline_record=57356; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=419, inline_record=15092; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=10292; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=10536; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=11320; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::normalize
// 信息 ID: 1401f5427a9b5e28f38513be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 17 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=406, inline_record=19148; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=406, inline_record=20948; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=406, inline_record=21440; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks; module=407, inline_record=18120; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=11852; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=12696; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=16864; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=17708; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::to_2d
// 信息 ID: a9dc46488277fc9b410b4d64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$2::length
// 信息 ID: df20f040b6ce9d0ca5137151
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=14848; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=17372; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=18912; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=14616; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1436, inline_record=3412; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1436, inline_record=5728; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1436, inline_record=7252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$2::lerp
// 信息 ID: 42944ffcc55a2ab5a0b07e40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::maintain_lantern_lights; module=415, inline_record=33584; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::brush_preview::brush_preview; module=481, inline_record=41120; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::time_of_day::TodSequence::snapshot_at_tod; module=576, inline_record=12760; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::time_of_day::TodSequence::snapshot_at_tod; module=576, inline_record=12788; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::time_of_day::TodSequence::snapshot_at_tod; module=576, inline_record=13084; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::time_of_day::TodSequence::snapshot_at_tod; module=576, inline_record=13112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$4::assign_auto_resize<enum2$<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex> > >
// 信息 ID: 3885f8b0a70e83c04395cf12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["country_core::asset_libraries::texture_library::DescriptorIndex"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex>"]; 字段布局 = unknown。
// [类型关系] utils::impl$4::assign_auto_resize -> ["enum2$<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index @ 0x93d46e -> 0x12f8f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$4::assign_auto_resize<tg_text::AtlasAllocUsage>
// 信息 ID: 5efb154cfd0e66b506801bed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::impl$4::assign_auto_resize -> ["tg_text::AtlasAllocUsage"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextRenderer::get_glyph @ 0xb903f5 -> 0x1800040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$5::intersects
// 信息 ID: 92882a25f7c5f9581c86608a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121af2a -> 0x1762b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$6::intersects
// 信息 ID: b7d3d097f2fd847eb78df0d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97c575 -> 0x1762b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x12368fc -> 0x1762b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x12369cd -> 0x1762b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x12365be -> 0x1762b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd2bb0 -> 0x1762b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22dae58 -> 0x1762b90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d; module=768, inline_record=1476; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$9::iterator_slice_range
// 信息 ID: 21ea4efc22ff10bb3e9cf1cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1449, inline_record=2496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::inverse_lerp
// 信息 ID: 363fa1e80771803fc9894a95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 894525e2ee643a10ae49e3ac: glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 引用方: 原程序直接转移 101 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D> @ 0x152e75 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::Curve::get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > | utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > @ 0x153462 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::Curve::try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> @ 0x154025 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x74fce3 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x7590ee -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x759162 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t @ 0x7653ac -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76a6b8 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77e661 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78bfdf -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x797deb -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs::closure$12 @ 0x79b80c -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7aaa32 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7aaa5c -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::construct_stair_steps @ 0x7da886 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones @ 0x7eb5c3 -> 0xc904a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::linearstep; module=759, inline_record=3928; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=16184; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >; module=2076, inline_record=7136; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::lerp
// 信息 ID: a422bc5b9b3eba5856dae262
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9e286f0216e1fbaff1175eba: array$<glam::f32::sse2::vec3a::Vec3A,32> (*)()
// 引用方: 原程序直接转移 48 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> @ 0x257e66 -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> @ 0x257e7a -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid> @ 0x257eb9 -> 0xc904d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b878e -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x3d08af -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::insert_point_ui @ 0x79c0df -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs::walk_shape @ 0x80aa48 -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x81d2a0 -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::main_camera::camera_snapshot @ 0x912227 -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_grower::closest_segment_index @ 0x94c9d8 -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x9e99cf -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x9e99e3 -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x9e9a12 -> 0xc904d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | 其余 4 个同地址别名见共享索引 @ 0xa150eb -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | 其余 4 个同地址别名见共享索引 @ 0xa150ff -> 0xc904d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | 其余 4 个同地址别名见共享索引 @ 0xa1512e -> 0xc904d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [PDB 内联宿主] utils::time_continuous_lerp; module=759, inline_record=4244; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec2; module=759, inline_record=4600; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec2; module=759, inline_record=4628; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=759, inline_record=4864; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=759, inline_record=4920; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_mut; module=759, inline_record=5184; 不是独立调用边。
// [PDB 内联宿主] utils::fit; module=759, inline_record=14412; 不是独立调用边。
// [PDB 内联宿主] utils::wall_space::WallSpaceCoord::from_curve_internal_coord; module=761, inline_record=2312; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::linearstep
// 信息 ID: 8280865a515d55ef10669cb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x3d0897 -> 0xc904b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::mouse_ss_to_cs
// 信息 ID: 69ce57e8f1e6515efade6d06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x9a648f -> 0xc906d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line @ 0xa9bd9a -> 0xc906d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode @ 0xac7002 -> 0xc906d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space @ 0xac847a -> 0xc906d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0> @ 0xb1cd33 -> 0xc906d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0> @ 0xb1ce96 -> 0xc906d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::pack_2x16f_uint
// 信息 ID: a7f7e9def785b4084657b0e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c781a -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor> @ 0x1430227 -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor> @ 0x14300a7 -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x1bf2373 -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > > >,system_critters::duck::update_duck_ripples::closure_env$0> @ 0x1d902e9 -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > > >,system_critters::duck::update_duck_ripples::closure_env$0> @ 0x1d902fa -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_solid_vertex_color_bricks @ 0x2076820 -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x2077634 -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x2077644 -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x2077654 -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x207766c -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_lantern_emissive @ 0x207adac -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_lantern_emissive @ 0x207adbd -> 0xc90df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::tree_animation::handplaced_tree_animation @ 0x2279ddb -> 0xc90df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::pack_srgba_u32
// 信息 ID: a3b3b0fa9839d444cea1a644
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x689fca -> 0xc91930 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1bd89 -> 0xc91930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::perspective_lerp
// 信息 ID: b5fbcc76d03fa8a5f9d0b19e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::get_closest_point_u_to_cursor; module=565, inline_record=56624; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray; module=565, inline_record=68564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::polar_to_cartesian
// 信息 ID: f0fbb8278d17d3d06a44c2cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad96a -> 0xc90e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::color_menu::color_menu @ 0x2294fda -> 0xc90e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::project_point_onto_line
// 信息 ID: 1d6e2af7713a0a5db7c2db1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=419, inline_record=14656; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=5264; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims; module=567, inline_record=61764; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=567, inline_record=90944; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3036, inline_record=4092; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3036, inline_record=4924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::project_point_onto_segment
// 信息 ID: 04f0423e817c1af79b5f43d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=19992; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment; module=512, inline_record=48956; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=512, inline_record=51892; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=519, inline_record=98568; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=768, inline_record=29564; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=6284; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=7012; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::is_on_right_side<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1607, inline_record=48168; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::radical_inverse
// 信息 ID: a43bff26d3c2ea64959bab5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0x9d4ee7 -> 0xc909a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0> > > >; module=3375, inline_record=2924; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0> > > >; module=3375, inline_record=2972; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0> > > >; module=3375, inline_record=3220; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0> > > >; module=3375, inline_record=3268; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_orthogonal_quat
// 信息 ID: be83d0c4d9ceed9b885738dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks @ 0x77b897 -> 0xc91c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets @ 0x785150 -> 0xc91c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits
// 信息 ID: c2f81ab59ed754f62cb8ec60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x77564e -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77e90c -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77eea9 -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::construct_stair_steps @ 0x7daa2d -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x7e6b65 -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings @ 0x7fa29b -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings @ 0x7fb98a -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips @ 0x7fd176 -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0x120d89a -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1215cbd -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows @ 0x123e4db -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e0ee -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e497 -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1290e40 -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1291286 -> 0xc90350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1291403 -> 0xc90350 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits_into
// 信息 ID: eb6248158775e78e62b1216d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true> @ 0x48b4e8 -> 0xc90050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76a985 -> 0xc90050 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::random_splits @ 0xc903f5 -> 0xc90050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0x1222688 -> 0xc90050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0x12247e4 -> 0xc90050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0x124b5b1 -> 0xc90050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x124d6e3 -> 0xc90050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0x124f6ee -> 0xc90050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x22d51ad -> 0xc90050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits_into::closure$0
// 信息 ID: 941c3aa02f508168ade61028
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::random_splits_into", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::random_splits_into; module=759, inline_record=2464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rotation_from_normal
// 信息 ID: 5e84be062a73a815b5433937
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 23 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x83db4f -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x83de33 -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow @ 0xb35171 -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow @ 0xb353bc -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow @ 0xb3550a -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x121847e -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121b33a -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x125519f -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128f605 -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128f8d3 -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > > @ 0x1293e4d -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::impl$1::signifier::closure$0 @ 0x155f7ae -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget @ 0x15600b4 -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a42ad6 -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a42d86 -> 0xc90a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a43171 -> 0xc90a40 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color
// 信息 ID: cb8440690cd40a5a429d78c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$3::render @ 0x67e582 -> 0xc918d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$6::render @ 0x67ea86 -> 0xc918d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextRenderer::rect @ 0xb8f550 -> 0xc918d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$6::render @ 0x1e1e94c -> 0xc918d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color::scale_linear
// 信息 ID: 9e5f70f6fd680e6762ea6294
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::scale_u32_packed_srgba_color; module=759, inline_record=14564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color_premultiplied::scale_linear
// 信息 ID: 089ce96c9312cfa8e28ca3ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color_premultiplied::scale_srgb
// 信息 ID: d36bc85fccd4d21398a0c4d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::slice_iter
// 信息 ID: 9080d97121029c9476483569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1449, inline_record=2472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::smoothstep
// 信息 ID: f1da047970a6d070b03192ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 0 条；内联宿主 47 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::rollercoaster::rollercoaster_update; module=160, inline_record=10768; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true>; module=187, inline_record=104668; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)>; module=187, inline_record=162376; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=20720; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals; module=408, inline_record=15212; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph; module=437, inline_record=104896; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_assemble::construct_stair_steps; module=437, inline_record=127952; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=498, inline_record=16316; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::srgb_eotf
// 信息 ID: 8287ec4a98f5b2a058237d36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::srgb_oetf
// 信息 ID: 2ea17b1bb82299f42e3f6cc9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::pack_srgba_u32; module=759, inline_record=14732; 不是独立调用边。
// [PDB 内联宿主] utils::pack_srgba_u32; module=759, inline_record=14796; 不是独立调用边。
// [PDB 内联宿主] utils::pack_srgba_u32; module=759, inline_record=14860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::t_of_point_projection_onto_segment
// 信息 ID: b02ecb952d457a77c23cc3c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=20032; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_stairs_viz::insert_point_ui; module=417, inline_record=88708; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs; module=419, inline_record=14692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::closest_segment_index; module=512, inline_record=45092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment; module=512, inline_record=48984; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=512, inline_record=51920; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=519, inline_record=98600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=25140; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp
// 信息 ID: 3653535914a24d92cee53a3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 23 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b8690 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b87c3 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x75c157 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x794431 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x794bc1 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x7953b1 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x796351 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x796683 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x79696b -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x796c68 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x7972e8 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x795b3a -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::window_lights::window_lights @ 0x7b63e6 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain @ 0x99463b -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xa72b6f -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_mode::replay_mode @ 0xa7c9a3 -> 0xc904e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=759, inline_record=4780; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=759, inline_record=4896; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_mut; module=759, inline_record=5100; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_factor
// 信息 ID: c3b56ef15be395f0ad4ec82a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_interpolate_and_compare @ 0x7cf7ea -> 0xc90540 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::time_continuous_lerp; module=759, inline_record=4188; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec2; module=759, inline_record=4544; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=759, inline_record=4808; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_mut; module=759, inline_record=5128; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_mut
// 信息 ID: fcf6cfa191cb04682b168a21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0xa73f52 -> 0xc90670 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::prepare_layout @ 0x1dfb76a -> 0xc90670 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::prepare_layout @ 0x1dfb78c -> 0xc90670 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::prepare_layout @ 0x1dfb7ad -> 0xc90670 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::impl$3::render @ 0x1e1f58d -> 0xc90670 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::impl$3::render @ 0x1e1f746 -> 0xc90670 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::impl$3::render @ 0x1e1f788 -> 0xc90670 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::impl$3::render @ 0x1e1f84e -> 0xc90670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x2329b88 -> 0xc90670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_vec2
// 信息 ID: ee1b2f9a893ea6bedcde38d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97bff3 -> 0xc90570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97bf30 -> 0xc90570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_vec3
// 信息 ID: d6b52b63c5a16a78f9f2e8e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x796d49 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x796a4d -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x79676c -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x7964ac -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height @ 0xa629a6 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf9192f -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf91ce7 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf91e87 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf92006 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf925d1 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::random_vel_change @ 0x1dc2249 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0> @ 0x1dc2da7 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x1dc3bb4 -> 0xc90600 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x1eace12 -> 0xc90600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_circle
// 信息 ID: bf68fdbeaf233b68204dc197
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::spawn_ducks::spawn_ducks @ 0xf8f652 -> 0xc90770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::spawn_ducks::despawn_ducks @ 0xf9007c -> 0xc90770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::spawn_ducks::despawn_ducks @ 0xf90373 -> 0xc90770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x23103d7 -> 0xc90770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x2310bd7 -> 0xc90770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x2311421 -> 0xc90770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_cone
// 信息 ID: c6a42167da93bbcd2e78e298
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::boids::Boid::random_vel_change @ 0x1dc21d5 -> 0xc90880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_disk
// 信息 ID: 7c5d023f253023a576417983
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0x9d4f42 -> 0xc90700 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::update_duck_goals @ 0x231408f -> 0xc90700 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_behavior::update_sheep_goals @ 0x2315944 -> 0xc90700 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_behavior::update_sheep_goals @ 0x2315b63 -> 0xc90700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_sphere
// 信息 ID: 069dc693215b9daeea13b7ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::butterfly::update_butterfly_goals @ 0x1d86584 -> 0xc907d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::firefly::update_firefly_goals @ 0x2316e60 -> 0xc907d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x2327933 -> 0xc907d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_projective_rotation_bad
// 信息 ID: 59c60f21815a758c51cce1ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x7bc7d4 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c8775 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c880c -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c8d8e -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c8da5 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x7e8f95 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7eccd3 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f113d -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f4643 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f6e7b -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f6f22 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f7020 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls @ 0x9785a9 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_splitter::wall_splitter @ 0xa7589b -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::move_ops::show_decorator_remove_signifier @ 0x127fea3 -> 0xc911d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x1a38cae -> 0xc911d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_projective_rotation_from_forward
// 信息 ID: cedd5a2c49e8f56bf5ff4082
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a64df -> 0xc91640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7abe18 -> 0xc91640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_bad
// 信息 ID: 73d0863d2badd977ed3b6d4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0ec546db294e804e3e737063: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 52 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x17924e -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x17aec7 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::systems::construct_gates @ 0x756b7a -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77efa8 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77f2c6 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs::closure$10 @ 0x79b441 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79d7b4 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79e550 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79ead6 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79f572 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::show_signifiers @ 0x7ae97f -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_collision_as_bbx @ 0x7bb5c4 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x7e6dd9 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x815ee8 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x815e19 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x815a47 -> 0xc90fd0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::xz_projective_rotation_bad; module=759, inline_record=10236; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_from_forward
// 信息 ID: de4ec016c79529ce71fbabba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8166a6 -> 0xc913e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::camera::CameraMatrices::xz_projective_rotation @ 0xc96fa1 -> 0xc913e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::xz_projective_rotation_from_forward; module=759, inline_record=12956; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_mat3_bad
// 信息 ID: c70146a3f8a28be79de11dd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::construct_stair_steps @ 0x7dad1b -> 0xc90ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings @ 0x7fba10 -> 0xc90ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips @ 0x7fd050 -> 0xc90ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216b8d -> 0xc90ed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_mat3a_bad
// 信息 ID: c4a2b2e70eee08262936c144
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::xz_rotation_bad; module=759, inline_record=9020; 不是独立调用边。
// [PDB 内联宿主] utils::xz_projective_rotation_bad; module=759, inline_record=10260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_mat3a_from_forward
// 信息 ID: ec96c62910c036287642c2a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::xz_rotation_from_forward; module=759, inline_record=11496; 不是独立调用边。
// [PDB 内联宿主] utils::xz_projective_rotation_from_forward; module=759, inline_record=12980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
