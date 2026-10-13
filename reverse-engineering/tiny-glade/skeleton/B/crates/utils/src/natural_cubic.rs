// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/natural_cubic.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d466725da68f127c4b251f2e672449ec145e6330b0fb944b197116265eddef28
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: natural_cubic.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 762, 2079
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bumpalo; // namespace candidate
//   package identities: [{"name":"bumpalo","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.18.1"}]
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_tridiagonal
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":35112,"rva":13196384,"va":5381905504}
// 编译实例 RVAs: 0xc95c60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29941586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381905504,"raw_hex":"e852dfc801","size":5,"soft_fail":false,"target_va":5411847200,"va":5381905609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29628701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381905504,"raw_hex":"e81d19c401","size":5,"soft_fail":false,"target_va":5411534720,"va":5381906014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29629246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381905504,"raw_hex":"e83e1bc401","size":5,"soft_fail":false,"target_va":5411535312,"va":5381906061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t29636212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381905504,"raw_hex":"e87436c401","size":5,"soft_fail":false,"target_va":5411542296,"va":5381906079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":35244,"parent_record_offset":35112,"ancestor_record_offset":35112,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":762,"record_offset":36536,"parent_record_offset":35112,"ancestor_record_offset":35112,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":36440,"parent_record_offset":35112,"ancestor_record_offset":35112,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":35376,"parent_record_offset":35276,"ancestor_record_offset":35112,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":35276,"parent_record_offset":35244,"ancestor_record_offset":35112,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":35404,"parent_record_offset":35376,"ancestor_record_offset":35112,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":35436,"parent_record_offset":35404,"ancestor_record_offset":35112,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":36744,"parent_record_offset":36720,"ancestor_record_offset":35112,"inlinee":15494,"source_line":1865,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":35528,"parent_record_offset":35436,"ancestor_record_offset":35112,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":35320,"parent_record_offset":35276,"ancestor_record_offset":35112,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":35344,"parent_record_offset":35320,"ancestor_record_offset":35112,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":35472,"parent_record_offset":35436,"ancestor_record_offset":35112,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":35636,"parent_record_offset":35612,"ancestor_record_offset":35112,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":35612,"parent_record_offset":35436,"ancestor_record_offset":35112,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":35820,"parent_record_offset":35792,"ancestor_record_offset":35112,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":35792,"parent_record_offset":35764,"ancestor_record_offset":35112,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":36584,"parent_record_offset":36560,"ancestor_record_offset":35112,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":35764,"parent_record_offset":35276,"ancestor_record_offset":35112,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":36560,"parent_record_offset":36536,"ancestor_record_offset":35112,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":35660,"parent_record_offset":35636,"ancestor_record_offset":35112,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":35728,"parent_record_offset":35404,"ancestor_record_offset":35112,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":35576,"parent_record_offset":35552,"ancestor_record_offset":35112,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":35552,"parent_record_offset":35528,"ancestor_record_offset":35112,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":35684,"parent_record_offset":35660,"ancestor_record_offset":35112,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":762,"record_offset":36720,"parent_record_offset":36696,"ancestor_record_offset":35112,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::SplineSegment2D::position_at_local_t
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF3B
// 首条位置: {"module":762,"inlinee":44859}
// 编译实例 RVAs: 0xc92610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::natural_cubic::eval_cubic
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":396,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44835,"source_line":351,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::SplineSegment2D::tangent_at_local_t
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 29, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF3C
// 首条位置: {"module":762,"inlinee":44860}
// 编译实例 RVAs: 0xc92660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::natural_cubic::eval_cubic_deriv
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":596,"parent_record_offset":452,"ancestor_record_offset":452,"inlinee":44836,"source_line":356,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::SplineSegment3D::position_at_local_t
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF4A
// 首条位置: {"module":762,"inlinee":44874}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":23448,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::eval_cubic
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":23392,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":44835,"source_line":351,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::SplineSegment3D::tangent_at_local_t
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF4B
// 首条位置: {"module":762,"inlinee":44875}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":24272,"parent_record_offset":24184,"ancestor_record_offset":24184,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::eval_cubic_deriv
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":24216,"parent_record_offset":24184,"ancestor_record_offset":24184,"inlinee":44836,"source_line":356,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline2D::from_control_points_with_tangents
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":656,"rva":13182640,"va":5381891760}
// 编译实例 RVAs: 0xc926b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29954814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891760,"raw_hex":"e8fe12c901","size":5,"soft_fail":false,"target_va":5411847200,"va":5381892381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29641001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891760,"raw_hex":"e82949c401","size":5,"soft_fail":false,"target_va":5411534720,"va":5381893714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13296309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891760,"raw_hex":"e8b5e2ca00","size":5,"soft_fail":false,"target_va":5395189216,"va":5381892902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29641221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891760,"raw_hex":"e8054ac401","size":5,"soft_fail":false,"target_va":5411535312,"va":5381894086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29647669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891760,"raw_hex":"e83563c401","size":5,"soft_fail":false,"target_va":5411541808,"va":5381894134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29647618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891760,"raw_hex":"e80263c401","size":5,"soft_fail":false,"target_va":5411541776,"va":5381894153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29647949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891760,"raw_hex":"e84d64c401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381894110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::natural_cubic::solve_clamped_spline
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891760,"raw_hex":"e8e91d0000","size":5,"soft_fail":false,"target_va":5381900816,"va":5381893154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":3280,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::from_iter_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":4404,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":44842,"source_line":604,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":5464,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":5324,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":5868,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":6980,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":7016,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::build_knots_open
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":836,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":44837,"source_line":180,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1044,"parent_record_offset":936,"ancestor_record_offset":656,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":2256,"parent_record_offset":836,"ancestor_record_offset":656,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":908,"parent_record_offset":836,"ancestor_record_offset":656,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":936,"parent_record_offset":908,"ancestor_record_offset":656,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":5064,"parent_record_offset":5008,"ancestor_record_offset":656,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":5096,"parent_record_offset":5064,"ancestor_record_offset":656,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":656,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":656,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":5036,"parent_record_offset":5008,"ancestor_record_offset":656,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":656,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":656,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":4660,"parent_record_offset":4632,"ancestor_record_offset":656,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":656,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":656,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":4448,"parent_record_offset":4404,"ancestor_record_offset":656,"inlinee":19968,"source_line":545,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":4596,"parent_record_offset":4476,"ancestor_record_offset":656,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":4632,"parent_record_offset":4596,"ancestor_record_offset":656,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":4512,"parent_record_offset":4476,"ancestor_record_offset":656,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":4476,"parent_record_offset":4404,"ancestor_record_offset":656,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":6640,"parent_record_offset":6276,"ancestor_record_offset":656,"inlinee":15494,"source_line":1865,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1432,"parent_record_offset":936,"ancestor_record_offset":656,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":6216,"parent_record_offset":6148,"ancestor_record_offset":656,"inlinee":39942,"source_line":1556,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1196,"parent_record_offset":1104,"ancestor_record_offset":656,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":988,"parent_record_offset":936,"ancestor_record_offset":656,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":656,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":1140,"parent_record_offset":1104,"ancestor_record_offset":656,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":656,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":1280,"parent_record_offset":1104,"ancestor_record_offset":656,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":656,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":2152,"parent_record_offset":2120,"ancestor_record_offset":656,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":3124,"parent_record_offset":2924,"ancestor_record_offset":656,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":762,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":656,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":656,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":762,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":656,"inlinee":11851,"source_line":91,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":2952,"parent_record_offset":2924,"ancestor_record_offset":656,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":656,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":656,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":3192,"parent_record_offset":836,"ancestor_record_offset":656,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":656,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":656,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":3228,"parent_record_offset":3192,"ancestor_record_offset":656,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":2184,"parent_record_offset":2120,"ancestor_record_offset":656,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":2120,"parent_record_offset":2092,"ancestor_record_offset":656,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":2092,"parent_record_offset":936,"ancestor_record_offset":656,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":5760,"parent_record_offset":5512,"ancestor_record_offset":656,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":2876,"parent_record_offset":836,"ancestor_record_offset":656,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":656,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":656,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":1660,"parent_record_offset":1624,"ancestor_record_offset":656,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":762,"record_offset":1520,"parent_record_offset":1488,"ancestor_record_offset":656,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":4080,"parent_record_offset":4048,"ancestor_record_offset":656,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":6912,"parent_record_offset":6772,"ancestor_record_offset":656,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":1488,"parent_record_offset":1460,"ancestor_record_offset":656,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":1396,"parent_record_offset":1072,"ancestor_record_offset":656,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":762,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":656,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":4936,"parent_record_offset":4404,"ancestor_record_offset":656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":5160,"parent_record_offset":5064,"ancestor_record_offset":656,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":656,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":656,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":656,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":656,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":656,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":1932,"parent_record_offset":1888,"ancestor_record_offset":656,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":656,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":656,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":1888,"parent_record_offset":1860,"ancestor_record_offset":656,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":1960,"parent_record_offset":1888,"ancestor_record_offset":656,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":3004,"parent_record_offset":2976,"ancestor_record_offset":656,"inlinee":24694,"source_line":171,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1740,"parent_record_offset":1708,"ancestor_record_offset":656,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1708,"parent_record_offset":1552,"ancestor_record_offset":656,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1624,"parent_record_offset":1588,"ancestor_record_offset":656,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1588,"parent_record_offset":1552,"ancestor_record_offset":656,"inlinee":25582,"source_line":1437,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":1860,"parent_record_offset":1460,"ancestor_record_offset":656,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":1804,"parent_record_offset":1740,"ancestor_record_offset":656,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_open::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":1552,"parent_record_offset":1520,"ancestor_record_offset":656,"inlinee":44839,"source_line":184,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":4316,"parent_record_offset":4288,"ancestor_record_offset":656,"inlinee":44841,"source_line":83,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":5804,"parent_record_offset":5760,"ancestor_record_offset":656,"inlinee":44845,"source_line":96,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":656,"inlinee":44846,"source_line":108,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF29
// 首条位置: {"module":762,"inlinee":44841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF2D
// 首条位置: {"module":762,"inlinee":44845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF2E
// 首条位置: {"module":762,"inlinee":44846}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline2D::from_periodic_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":7788,"rva":13185264,"va":5381894384}
// 编译实例 RVAs: 0xc930f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29952459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e8cb09c901","size":5,"soft_fail":false,"target_va":5411847200,"va":5381894736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29638820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e8a440c401","size":5,"soft_fail":false,"target_va":5411534720,"va":5381895895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13293984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e8a0d9ca00","size":5,"soft_fail":false,"target_va":5395189216,"va":5381895227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29639039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e87f41c401","size":5,"soft_fail":false,"target_va":5411535312,"va":5381896268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29645487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e8af5ac401","size":5,"soft_fail":false,"target_va":5411541808,"va":5381896316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29645418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e86a5ac401","size":5,"soft_fail":false,"target_va":5411541776,"va":5381896353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29645767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e8c75bc401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381896292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29645952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e8805cc401","size":5,"soft_fail":false,"target_va":5411542296,"va":5381896339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::natural_cubic::compute_periodic_spline
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381894384,"raw_hex":"e81e1d0000","size":5,"soft_fail":false,"target_va":5381902848,"va":5381895389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":10436,"parent_record_offset":7788,"ancestor_record_offset":7788,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::from_iter_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":11556,"parent_record_offset":7788,"ancestor_record_offset":7788,"inlinee":44842,"source_line":604,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":12624,"parent_record_offset":7788,"ancestor_record_offset":7788,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":12484,"parent_record_offset":7788,"ancestor_record_offset":7788,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":14088,"parent_record_offset":7788,"ancestor_record_offset":7788,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":14124,"parent_record_offset":7788,"ancestor_record_offset":7788,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::build_knots_closed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":7948,"parent_record_offset":7788,"ancestor_record_offset":7788,"inlinee":44847,"source_line":327,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8156,"parent_record_offset":8056,"ancestor_record_offset":7788,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":9676,"parent_record_offset":7948,"ancestor_record_offset":7788,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8028,"parent_record_offset":7948,"ancestor_record_offset":7788,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":7788,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":12224,"parent_record_offset":12168,"ancestor_record_offset":7788,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":12256,"parent_record_offset":12224,"ancestor_record_offset":7788,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8184,"parent_record_offset":8156,"ancestor_record_offset":7788,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8216,"parent_record_offset":8184,"ancestor_record_offset":7788,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":12196,"parent_record_offset":12168,"ancestor_record_offset":7788,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":12168,"parent_record_offset":12144,"ancestor_record_offset":7788,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":7788,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":11820,"parent_record_offset":11788,"ancestor_record_offset":7788,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":7788,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":7788,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":11600,"parent_record_offset":11556,"ancestor_record_offset":7788,"inlinee":19968,"source_line":545,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":11748,"parent_record_offset":11628,"ancestor_record_offset":7788,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":11788,"parent_record_offset":11748,"ancestor_record_offset":7788,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":11664,"parent_record_offset":11628,"ancestor_record_offset":7788,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":11628,"parent_record_offset":11556,"ancestor_record_offset":7788,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":13748,"parent_record_offset":13384,"ancestor_record_offset":7788,"inlinee":15494,"source_line":1865,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8544,"parent_record_offset":8056,"ancestor_record_offset":7788,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":13324,"parent_record_offset":13256,"ancestor_record_offset":7788,"inlinee":39942,"source_line":1556,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8308,"parent_record_offset":8216,"ancestor_record_offset":7788,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":8100,"parent_record_offset":8056,"ancestor_record_offset":7788,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":7788,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":8252,"parent_record_offset":8216,"ancestor_record_offset":7788,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":7788,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":8392,"parent_record_offset":8216,"ancestor_record_offset":7788,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":7788,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":9632,"parent_record_offset":9548,"ancestor_record_offset":7788,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":12556,"parent_record_offset":12532,"ancestor_record_offset":7788,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":762,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":7788,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":762,"record_offset":8972,"parent_record_offset":8948,"ancestor_record_offset":7788,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":762,"record_offset":11200,"parent_record_offset":11172,"ancestor_record_offset":7788,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cycle.rs
//     evidence={"module":762,"record_offset":8672,"parent_record_offset":8636,"ancestor_record_offset":7788,"inlinee":12166,"source_line":34,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":8572,"parent_record_offset":8544,"ancestor_record_offset":7788,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":12508,"parent_record_offset":12484,"ancestor_record_offset":7788,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":7788,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":10348,"parent_record_offset":7948,"ancestor_record_offset":7788,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":12672,"parent_record_offset":12648,"ancestor_record_offset":7788,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":12532,"parent_record_offset":12508,"ancestor_record_offset":7788,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":10384,"parent_record_offset":10348,"ancestor_record_offset":7788,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":9576,"parent_record_offset":9548,"ancestor_record_offset":7788,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":7788,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":9524,"parent_record_offset":8056,"ancestor_record_offset":7788,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":12920,"parent_record_offset":12672,"ancestor_record_offset":7788,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":8996,"parent_record_offset":8972,"ancestor_record_offset":7788,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":7788,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":9600,"parent_record_offset":9576,"ancestor_record_offset":7788,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":9348,"parent_record_offset":9308,"ancestor_record_offset":7788,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":762,"record_offset":9220,"parent_record_offset":9188,"ancestor_record_offset":7788,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":11232,"parent_record_offset":11200,"ancestor_record_offset":7788,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":14020,"parent_record_offset":13880,"ancestor_record_offset":7788,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":9188,"parent_record_offset":8572,"ancestor_record_offset":7788,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":8508,"parent_record_offset":8184,"ancestor_record_offset":7788,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":8948,"parent_record_offset":8924,"ancestor_record_offset":7788,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":12096,"parent_record_offset":11556,"ancestor_record_offset":7788,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":12320,"parent_record_offset":12224,"ancestor_record_offset":7788,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":7788,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":7788,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":8792,"parent_record_offset":8764,"ancestor_record_offset":7788,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":8864,"parent_record_offset":8836,"ancestor_record_offset":7788,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":7788,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":8764,"parent_record_offset":8736,"ancestor_record_offset":7788,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":8708,"parent_record_offset":8672,"ancestor_record_offset":7788,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":8736,"parent_record_offset":8708,"ancestor_record_offset":7788,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9280,"parent_record_offset":9252,"ancestor_record_offset":7788,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9380,"parent_record_offset":9308,"ancestor_record_offset":7788,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9308,"parent_record_offset":9280,"ancestor_record_offset":7788,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$10::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":8600,"parent_record_offset":8572,"ancestor_record_offset":7788,"inlinee":14447,"source_line":276,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":8900,"parent_record_offset":8636,"ancestor_record_offset":7788,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":8636,"parent_record_offset":8600,"ancestor_record_offset":7788,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":9428,"parent_record_offset":9380,"ancestor_record_offset":7788,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_closed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":9252,"parent_record_offset":9220,"ancestor_record_offset":7788,"inlinee":44849,"source_line":333,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_periodic_loop::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":11468,"parent_record_offset":11440,"ancestor_record_offset":7788,"inlinee":44851,"source_line":126,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_periodic_loop::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":12964,"parent_record_offset":12920,"ancestor_record_offset":7788,"inlinee":44852,"source_line":131,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_periodic_loop::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":7788,"inlinee":44853,"source_line":135,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::from_periodic_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF33
// 首条位置: {"module":762,"inlinee":44851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::from_periodic_loop::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF34
// 首条位置: {"module":762,"inlinee":44852}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::from_periodic_loop::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF35
// 首条位置: {"module":762,"inlinee":44853}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline2D::find_segment_and_dt
// 记录形式: inlinee_declaration; instances=1
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF37
// 首条位置: {"module":762,"inlinee":44855}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::find_segment_and_dt::closure$0
// 记录形式: inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF39
// 首条位置: {"module":762,"inlinee":44857}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_open
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF25
// 首条位置: {"module":762,"inlinee":44837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":2256,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":908,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":3192,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":2876,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1044,"parent_record_offset":936,"ancestor_record_offset":836,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":936,"parent_record_offset":908,"ancestor_record_offset":836,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":836,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":836,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1432,"parent_record_offset":936,"ancestor_record_offset":836,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":1196,"parent_record_offset":1104,"ancestor_record_offset":836,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":988,"parent_record_offset":936,"ancestor_record_offset":836,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":836,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":1140,"parent_record_offset":1104,"ancestor_record_offset":836,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":836,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":1280,"parent_record_offset":1104,"ancestor_record_offset":836,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":836,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":2152,"parent_record_offset":2120,"ancestor_record_offset":836,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":3124,"parent_record_offset":2924,"ancestor_record_offset":836,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":836,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":762,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":836,"inlinee":11851,"source_line":91,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":2952,"parent_record_offset":2924,"ancestor_record_offset":836,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":836,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":836,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":3228,"parent_record_offset":3192,"ancestor_record_offset":836,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":2184,"parent_record_offset":2120,"ancestor_record_offset":836,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":2120,"parent_record_offset":2092,"ancestor_record_offset":836,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":2092,"parent_record_offset":936,"ancestor_record_offset":836,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":836,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":836,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":1660,"parent_record_offset":1624,"ancestor_record_offset":836,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":762,"record_offset":1520,"parent_record_offset":1488,"ancestor_record_offset":836,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":1488,"parent_record_offset":1460,"ancestor_record_offset":836,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":1396,"parent_record_offset":1072,"ancestor_record_offset":836,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":762,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":836,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":836,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":836,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":836,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":836,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":836,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":1932,"parent_record_offset":1888,"ancestor_record_offset":836,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":836,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":836,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":1888,"parent_record_offset":1860,"ancestor_record_offset":836,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":1960,"parent_record_offset":1888,"ancestor_record_offset":836,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":3004,"parent_record_offset":2976,"ancestor_record_offset":836,"inlinee":24694,"source_line":171,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1740,"parent_record_offset":1708,"ancestor_record_offset":836,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1708,"parent_record_offset":1552,"ancestor_record_offset":836,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1624,"parent_record_offset":1588,"ancestor_record_offset":836,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1588,"parent_record_offset":1552,"ancestor_record_offset":836,"inlinee":25582,"source_line":1437,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":1860,"parent_record_offset":1460,"ancestor_record_offset":836,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":1804,"parent_record_offset":1740,"ancestor_record_offset":836,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_open::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":1552,"parent_record_offset":1520,"ancestor_record_offset":836,"inlinee":44839,"source_line":184,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_open::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF27
// 首条位置: {"module":762,"inlinee":44839}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1708,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1588,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":25582,"source_line":1437,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":1660,"parent_record_offset":1624,"ancestor_record_offset":1552,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1740,"parent_record_offset":1708,"ancestor_record_offset":1552,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1624,"parent_record_offset":1588,"ancestor_record_offset":1552,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":1804,"parent_record_offset":1740,"ancestor_record_offset":1552,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline3D::from_control_points_with_tangents
// 记录形式: compiled_procedure; instances=1
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":14896,"rva":13187440,"va":5381896560}
// 编译实例 RVAs: 0xc93970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29949883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381896560,"raw_hex":"e8bbffc801","size":5,"soft_fail":false,"target_va":5411847200,"va":5381897312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29635570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381896560,"raw_hex":"e8f233c401","size":5,"soft_fail":false,"target_va":5411534720,"va":5381899145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13291387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381896560,"raw_hex":"e87bcfca00","size":5,"soft_fail":false,"target_va":5395189216,"va":5381897824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29635686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381896560,"raw_hex":"e86634c401","size":5,"soft_fail":false,"target_va":5411535312,"va":5381899621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29642134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381896560,"raw_hex":"e8964dc401","size":5,"soft_fail":false,"target_va":5411541808,"va":5381899669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29642083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381896560,"raw_hex":"e8634dc401","size":5,"soft_fail":false,"target_va":5411541776,"va":5381899688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29642414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381896560,"raw_hex":"e8ae4ec401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381899645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::natural_cubic::solve_clamped_spline
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381896560,"raw_hex":"e89e0a0000","size":5,"soft_fail":false,"target_va":5381900816,"va":5381898093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":17584,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::from_iter_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":18704,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":44842,"source_line":604,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":19764,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":19624,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":20584,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":21848,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":21884,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::build_knots_open3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":15076,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":44861,"source_line":307,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15284,"parent_record_offset":15176,"ancestor_record_offset":14896,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":16564,"parent_record_offset":15076,"ancestor_record_offset":14896,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15148,"parent_record_offset":15076,"ancestor_record_offset":14896,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15176,"parent_record_offset":15148,"ancestor_record_offset":14896,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":19364,"parent_record_offset":19308,"ancestor_record_offset":14896,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":19396,"parent_record_offset":19364,"ancestor_record_offset":14896,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15312,"parent_record_offset":15284,"ancestor_record_offset":14896,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15344,"parent_record_offset":15312,"ancestor_record_offset":14896,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":19336,"parent_record_offset":19308,"ancestor_record_offset":14896,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":19308,"parent_record_offset":19284,"ancestor_record_offset":14896,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":18860,"parent_record_offset":18836,"ancestor_record_offset":14896,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":18960,"parent_record_offset":18932,"ancestor_record_offset":14896,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":18836,"parent_record_offset":18812,"ancestor_record_offset":14896,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":762,"record_offset":19284,"parent_record_offset":19260,"ancestor_record_offset":14896,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":18748,"parent_record_offset":18704,"ancestor_record_offset":14896,"inlinee":19968,"source_line":545,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":18896,"parent_record_offset":18776,"ancestor_record_offset":14896,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":18932,"parent_record_offset":18896,"ancestor_record_offset":14896,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":18812,"parent_record_offset":18776,"ancestor_record_offset":14896,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":762,"record_offset":18776,"parent_record_offset":18704,"ancestor_record_offset":14896,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":21500,"parent_record_offset":21136,"ancestor_record_offset":14896,"inlinee":15494,"source_line":1865,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15672,"parent_record_offset":15176,"ancestor_record_offset":14896,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":21076,"parent_record_offset":21008,"ancestor_record_offset":14896,"inlinee":39942,"source_line":1556,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15436,"parent_record_offset":15344,"ancestor_record_offset":14896,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":15228,"parent_record_offset":15176,"ancestor_record_offset":14896,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":15252,"parent_record_offset":15228,"ancestor_record_offset":14896,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":15380,"parent_record_offset":15344,"ancestor_record_offset":14896,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":14896,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":15520,"parent_record_offset":15344,"ancestor_record_offset":14896,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":17452,"parent_record_offset":17428,"ancestor_record_offset":14896,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":16460,"parent_record_offset":16428,"ancestor_record_offset":14896,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":17428,"parent_record_offset":17228,"ancestor_record_offset":14896,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":762,"record_offset":18348,"parent_record_offset":18320,"ancestor_record_offset":14896,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":15700,"parent_record_offset":15672,"ancestor_record_offset":14896,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":762,"record_offset":17280,"parent_record_offset":17256,"ancestor_record_offset":14896,"inlinee":11851,"source_line":91,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17256,"parent_record_offset":17228,"ancestor_record_offset":14896,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":14896,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":19792,"parent_record_offset":19764,"ancestor_record_offset":14896,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17496,"parent_record_offset":15076,"ancestor_record_offset":14896,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":19820,"parent_record_offset":19792,"ancestor_record_offset":14896,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":14896,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17532,"parent_record_offset":17496,"ancestor_record_offset":14896,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":16492,"parent_record_offset":16428,"ancestor_record_offset":14896,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":16428,"parent_record_offset":16400,"ancestor_record_offset":14896,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":16400,"parent_record_offset":15176,"ancestor_record_offset":14896,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":20080,"parent_record_offset":19820,"ancestor_record_offset":14896,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":17180,"parent_record_offset":15076,"ancestor_record_offset":14896,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":14896,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":14896,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":15916,"parent_record_offset":15864,"ancestor_record_offset":14896,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":762,"record_offset":15760,"parent_record_offset":15728,"ancestor_record_offset":14896,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":18380,"parent_record_offset":18348,"ancestor_record_offset":14896,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":21780,"parent_record_offset":21632,"ancestor_record_offset":14896,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":15728,"parent_record_offset":15700,"ancestor_record_offset":14896,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":15636,"parent_record_offset":15312,"ancestor_record_offset":14896,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":762,"record_offset":17380,"parent_record_offset":17356,"ancestor_record_offset":14896,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":19236,"parent_record_offset":18704,"ancestor_record_offset":14896,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":19460,"parent_record_offset":19364,"ancestor_record_offset":14896,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":17356,"parent_record_offset":17332,"ancestor_record_offset":14896,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":15484,"parent_record_offset":15460,"ancestor_record_offset":14896,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":14896,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":16316,"parent_record_offset":16292,"ancestor_record_offset":14896,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":17332,"parent_record_offset":17308,"ancestor_record_offset":14896,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":16240,"parent_record_offset":16196,"ancestor_record_offset":14896,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":15592,"parent_record_offset":15568,"ancestor_record_offset":14896,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":14896,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":16196,"parent_record_offset":16168,"ancestor_record_offset":14896,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":16268,"parent_record_offset":16196,"ancestor_record_offset":14896,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":17308,"parent_record_offset":17280,"ancestor_record_offset":14896,"inlinee":24694,"source_line":171,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":16040,"parent_record_offset":16000,"ancestor_record_offset":14896,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":16000,"parent_record_offset":15792,"ancestor_record_offset":14896,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":15864,"parent_record_offset":15824,"ancestor_record_offset":14896,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":15824,"parent_record_offset":15792,"ancestor_record_offset":14896,"inlinee":25316,"source_line":1493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":16168,"parent_record_offset":15700,"ancestor_record_offset":14896,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":16112,"parent_record_offset":16040,"ancestor_record_offset":14896,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_open3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":15792,"parent_record_offset":15760,"ancestor_record_offset":14896,"inlinee":44863,"source_line":311,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":18616,"parent_record_offset":18588,"ancestor_record_offset":14896,"inlinee":44865,"source_line":218,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$10
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":21728,"parent_record_offset":21704,"ancestor_record_offset":14896,"inlinee":44868,"source_line":254,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":20124,"parent_record_offset":20080,"ancestor_record_offset":14896,"inlinee":44866,"source_line":231,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":20520,"parent_record_offset":20476,"ancestor_record_offset":14896,"inlinee":44867,"source_line":242,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF41
// 首条位置: {"module":762,"inlinee":44865}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF42
// 首条位置: {"module":762,"inlinee":44866}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF43
// 首条位置: {"module":762,"inlinee":44867}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF44
// 首条位置: {"module":762,"inlinee":44868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline3D::position_at_t
// 记录形式: compiled_procedure; instances=1
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":22656,"rva":13190816,"va":5381899936}
// 编译实例 RVAs: 0xc946a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1010844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381899936,"raw_hex":"e89c6c0f00","size":5,"soft_fail":false,"target_va":5382911104,"va":5381900255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29642021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381899936,"raw_hex":"e8254dc401","size":5,"soft_fail":false,"target_va":5411542296,"va":5381900270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::natural_cubic::Spline3D::find_segment_and_dt
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":22792,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":44870,"source_line":285,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::SplineSegment3D::position_at_local_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":23360,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":44874,"source_line":43,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::select_unpredictable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":762,"record_offset":23160,"parent_record_offset":22976,"ancestor_record_offset":22656,"inlinee":9551,"source_line":109,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":762,"record_offset":22836,"parent_record_offset":22792,"ancestor_record_offset":22656,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":762,"record_offset":22916,"parent_record_offset":22836,"ancestor_record_offset":22656,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":762,"record_offset":22888,"parent_record_offset":22836,"ancestor_record_offset":22656,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":23328,"parent_record_offset":22792,"ancestor_record_offset":22656,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":22976,"parent_record_offset":22952,"ancestor_record_offset":22656,"inlinee":9546,"source_line":2817,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":23028,"parent_record_offset":22976,"ancestor_record_offset":22656,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":22952,"parent_record_offset":22792,"ancestor_record_offset":22656,"inlinee":9545,"source_line":4277,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":23112,"parent_record_offset":22976,"ancestor_record_offset":22656,"inlinee":9548,"source_line":4281,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":762,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22656,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":762,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":22656,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":23448,"parent_record_offset":23360,"ancestor_record_offset":22656,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::eval_cubic
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":23392,"parent_record_offset":23360,"ancestor_record_offset":22656,"inlinee":44835,"source_line":351,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::find_segment_and_dt::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":23132,"parent_record_offset":23112,"ancestor_record_offset":22656,"inlinee":44872,"source_line":291,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline3D::tangent_at_t
// 记录形式: compiled_procedure; instances=1
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":23484,"rva":13191168,"va":5381900288}
// 编译实例 RVAs: 0xc94800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1010521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900288,"raw_hex":"e8596b0f00","size":5,"soft_fail":false,"target_va":5382911104,"va":5381900578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29641698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900288,"raw_hex":"e8e24bc401","size":5,"soft_fail":false,"target_va":5411542296,"va":5381900593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::natural_cubic::Spline3D::find_segment_and_dt
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":23616,"parent_record_offset":23484,"ancestor_record_offset":23484,"inlinee":44870,"source_line":285,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::SplineSegment3D::tangent_at_local_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":24184,"parent_record_offset":23484,"ancestor_record_offset":23484,"inlinee":44875,"source_line":51,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::select_unpredictable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":762,"record_offset":23984,"parent_record_offset":23800,"ancestor_record_offset":23484,"inlinee":9551,"source_line":109,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":762,"record_offset":23660,"parent_record_offset":23616,"ancestor_record_offset":23484,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":762,"record_offset":23740,"parent_record_offset":23660,"ancestor_record_offset":23484,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":762,"record_offset":23712,"parent_record_offset":23660,"ancestor_record_offset":23484,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":24152,"parent_record_offset":23616,"ancestor_record_offset":23484,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":23800,"parent_record_offset":23776,"ancestor_record_offset":23484,"inlinee":9546,"source_line":2817,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":23852,"parent_record_offset":23800,"ancestor_record_offset":23484,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":23776,"parent_record_offset":23616,"ancestor_record_offset":23484,"inlinee":9545,"source_line":4277,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":23936,"parent_record_offset":23800,"ancestor_record_offset":23484,"inlinee":9548,"source_line":4281,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":762,"record_offset":23900,"parent_record_offset":23876,"ancestor_record_offset":23484,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":762,"record_offset":23876,"parent_record_offset":23852,"ancestor_record_offset":23484,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":24272,"parent_record_offset":24184,"ancestor_record_offset":23484,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::eval_cubic_deriv
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":24216,"parent_record_offset":24184,"ancestor_record_offset":23484,"inlinee":44836,"source_line":356,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::find_segment_and_dt::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":23956,"parent_record_offset":23936,"ancestor_record_offset":23484,"inlinee":44872,"source_line":291,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline3D::find_segment_and_dt
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF46
// 首条位置: {"module":762,"inlinee":44870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":762,"record_offset":22836,"parent_record_offset":22792,"ancestor_record_offset":22792,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":23328,"parent_record_offset":22792,"ancestor_record_offset":22792,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":22952,"parent_record_offset":22792,"ancestor_record_offset":22792,"inlinee":9545,"source_line":4277,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::select_unpredictable
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":762,"record_offset":23160,"parent_record_offset":22976,"ancestor_record_offset":22792,"inlinee":9551,"source_line":109,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":762,"record_offset":22916,"parent_record_offset":22836,"ancestor_record_offset":22792,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":762,"record_offset":22888,"parent_record_offset":22836,"ancestor_record_offset":22792,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":22976,"parent_record_offset":22952,"ancestor_record_offset":22792,"inlinee":9546,"source_line":2817,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":23028,"parent_record_offset":22976,"ancestor_record_offset":22792,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":762,"record_offset":23112,"parent_record_offset":22976,"ancestor_record_offset":22792,"inlinee":9548,"source_line":4281,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":762,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22792,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":762,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":22792,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::find_segment_and_dt::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":23132,"parent_record_offset":23112,"ancestor_record_offset":22792,"inlinee":44872,"source_line":291,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$3::find_segment_and_dt::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF48
// 首条位置: {"module":762,"inlinee":44872}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline3D::to_owned
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":24308,"rva":13191488,"va":5381900608}
// 编译实例 RVAs: 0xc94940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6819225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900608,"raw_hex":"e867f297ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381900708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29946419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900608,"raw_hex":"e833f2c801","size":5,"soft_fail":false,"target_va":5411847168,"va":5381900744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29640153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900608,"raw_hex":"e8d945c401","size":5,"soft_fail":false,"target_va":5411540963,"va":5381900805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":762,"record_offset":24432,"parent_record_offset":24308,"ancestor_record_offset":24308,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":762,"record_offset":24832,"parent_record_offset":24808,"ancestor_record_offset":24308,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":762,"record_offset":24856,"parent_record_offset":24832,"ancestor_record_offset":24308,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":762,"record_offset":24808,"parent_record_offset":24632,"ancestor_record_offset":24308,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":762,"record_offset":24576,"parent_record_offset":24548,"ancestor_record_offset":24308,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":762,"record_offset":24632,"parent_record_offset":24604,"ancestor_record_offset":24308,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":762,"record_offset":24604,"parent_record_offset":24576,"ancestor_record_offset":24308,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":762,"record_offset":24668,"parent_record_offset":24632,"ancestor_record_offset":24308,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":762,"record_offset":24516,"parent_record_offset":24488,"ancestor_record_offset":24308,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":762,"record_offset":24488,"parent_record_offset":24460,"ancestor_record_offset":24308,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":762,"record_offset":24460,"parent_record_offset":24432,"ancestor_record_offset":24308,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":762,"record_offset":24548,"parent_record_offset":24516,"ancestor_record_offset":24308,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":24692,"parent_record_offset":24668,"ancestor_record_offset":24308,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":24716,"parent_record_offset":24692,"ancestor_record_offset":24308,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":762,"record_offset":24932,"parent_record_offset":24908,"ancestor_record_offset":24308,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":24740,"parent_record_offset":24716,"ancestor_record_offset":24308,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":24764,"parent_record_offset":24740,"ancestor_record_offset":24308,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":762,"record_offset":24908,"parent_record_offset":24516,"ancestor_record_offset":24308,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_open3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF3D
// 首条位置: {"module":762,"inlinee":44861}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":16564,"parent_record_offset":15076,"ancestor_record_offset":15076,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15148,"parent_record_offset":15076,"ancestor_record_offset":15076,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17496,"parent_record_offset":15076,"ancestor_record_offset":15076,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":17180,"parent_record_offset":15076,"ancestor_record_offset":15076,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15284,"parent_record_offset":15176,"ancestor_record_offset":15076,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15176,"parent_record_offset":15148,"ancestor_record_offset":15076,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15312,"parent_record_offset":15284,"ancestor_record_offset":15076,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15344,"parent_record_offset":15312,"ancestor_record_offset":15076,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15672,"parent_record_offset":15176,"ancestor_record_offset":15076,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":15436,"parent_record_offset":15344,"ancestor_record_offset":15076,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":15228,"parent_record_offset":15176,"ancestor_record_offset":15076,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":15252,"parent_record_offset":15228,"ancestor_record_offset":15076,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":15380,"parent_record_offset":15344,"ancestor_record_offset":15076,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":15076,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":15520,"parent_record_offset":15344,"ancestor_record_offset":15076,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":17452,"parent_record_offset":17428,"ancestor_record_offset":15076,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":16460,"parent_record_offset":16428,"ancestor_record_offset":15076,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":17428,"parent_record_offset":17228,"ancestor_record_offset":15076,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":15700,"parent_record_offset":15672,"ancestor_record_offset":15076,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":762,"record_offset":17280,"parent_record_offset":17256,"ancestor_record_offset":15076,"inlinee":11851,"source_line":91,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17256,"parent_record_offset":17228,"ancestor_record_offset":15076,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":15076,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":15076,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":17532,"parent_record_offset":17496,"ancestor_record_offset":15076,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":16492,"parent_record_offset":16428,"ancestor_record_offset":15076,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":16428,"parent_record_offset":16400,"ancestor_record_offset":15076,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":16400,"parent_record_offset":15176,"ancestor_record_offset":15076,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":15076,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":15076,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":15916,"parent_record_offset":15864,"ancestor_record_offset":15076,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":762,"record_offset":15760,"parent_record_offset":15728,"ancestor_record_offset":15076,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":15728,"parent_record_offset":15700,"ancestor_record_offset":15076,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":15636,"parent_record_offset":15312,"ancestor_record_offset":15076,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":762,"record_offset":17380,"parent_record_offset":17356,"ancestor_record_offset":15076,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":17356,"parent_record_offset":17332,"ancestor_record_offset":15076,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":15484,"parent_record_offset":15460,"ancestor_record_offset":15076,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":15076,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":16316,"parent_record_offset":16292,"ancestor_record_offset":15076,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":17332,"parent_record_offset":17308,"ancestor_record_offset":15076,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":16240,"parent_record_offset":16196,"ancestor_record_offset":15076,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":15592,"parent_record_offset":15568,"ancestor_record_offset":15076,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":15076,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":16196,"parent_record_offset":16168,"ancestor_record_offset":15076,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":16268,"parent_record_offset":16196,"ancestor_record_offset":15076,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":17308,"parent_record_offset":17280,"ancestor_record_offset":15076,"inlinee":24694,"source_line":171,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":16040,"parent_record_offset":16000,"ancestor_record_offset":15076,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":16000,"parent_record_offset":15792,"ancestor_record_offset":15076,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":15864,"parent_record_offset":15824,"ancestor_record_offset":15076,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":15824,"parent_record_offset":15792,"ancestor_record_offset":15076,"inlinee":25316,"source_line":1493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":16168,"parent_record_offset":15700,"ancestor_record_offset":15076,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":16112,"parent_record_offset":16040,"ancestor_record_offset":15076,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_open3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":15792,"parent_record_offset":15760,"ancestor_record_offset":15076,"inlinee":44863,"source_line":311,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_open3::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF3F
// 首条位置: {"module":762,"inlinee":44863}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":16000,"parent_record_offset":15792,"ancestor_record_offset":15792,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":15824,"parent_record_offset":15792,"ancestor_record_offset":15792,"inlinee":25316,"source_line":1493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":15916,"parent_record_offset":15864,"ancestor_record_offset":15792,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":16040,"parent_record_offset":16000,"ancestor_record_offset":15792,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":15864,"parent_record_offset":15824,"ancestor_record_offset":15792,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":16112,"parent_record_offset":16040,"ancestor_record_offset":15792,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_closed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF2F
// 首条位置: {"module":762,"inlinee":44847}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":9676,"parent_record_offset":7948,"ancestor_record_offset":7948,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8028,"parent_record_offset":7948,"ancestor_record_offset":7948,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":10348,"parent_record_offset":7948,"ancestor_record_offset":7948,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8156,"parent_record_offset":8056,"ancestor_record_offset":7948,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":7948,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8184,"parent_record_offset":8156,"ancestor_record_offset":7948,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8216,"parent_record_offset":8184,"ancestor_record_offset":7948,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8544,"parent_record_offset":8056,"ancestor_record_offset":7948,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":8308,"parent_record_offset":8216,"ancestor_record_offset":7948,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":8100,"parent_record_offset":8056,"ancestor_record_offset":7948,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":7948,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":8252,"parent_record_offset":8216,"ancestor_record_offset":7948,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":7948,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":8392,"parent_record_offset":8216,"ancestor_record_offset":7948,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":9632,"parent_record_offset":9548,"ancestor_record_offset":7948,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":762,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":7948,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":762,"record_offset":8972,"parent_record_offset":8948,"ancestor_record_offset":7948,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cycle.rs
//     evidence={"module":762,"record_offset":8672,"parent_record_offset":8636,"ancestor_record_offset":7948,"inlinee":12166,"source_line":34,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":8572,"parent_record_offset":8544,"ancestor_record_offset":7948,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":762,"record_offset":10384,"parent_record_offset":10348,"ancestor_record_offset":7948,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":9576,"parent_record_offset":9548,"ancestor_record_offset":7948,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":7948,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":9524,"parent_record_offset":8056,"ancestor_record_offset":7948,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":762,"record_offset":8996,"parent_record_offset":8972,"ancestor_record_offset":7948,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":7948,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":9600,"parent_record_offset":9576,"ancestor_record_offset":7948,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":9348,"parent_record_offset":9308,"ancestor_record_offset":7948,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":762,"record_offset":9220,"parent_record_offset":9188,"ancestor_record_offset":7948,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":9188,"parent_record_offset":8572,"ancestor_record_offset":7948,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":8508,"parent_record_offset":8184,"ancestor_record_offset":7948,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":8948,"parent_record_offset":8924,"ancestor_record_offset":7948,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":7948,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":7948,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":8792,"parent_record_offset":8764,"ancestor_record_offset":7948,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":762,"record_offset":8864,"parent_record_offset":8836,"ancestor_record_offset":7948,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":7948,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":8764,"parent_record_offset":8736,"ancestor_record_offset":7948,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":8708,"parent_record_offset":8672,"ancestor_record_offset":7948,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":762,"record_offset":8736,"parent_record_offset":8708,"ancestor_record_offset":7948,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9280,"parent_record_offset":9252,"ancestor_record_offset":7948,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9380,"parent_record_offset":9308,"ancestor_record_offset":7948,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9308,"parent_record_offset":9280,"ancestor_record_offset":7948,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$10::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":8600,"parent_record_offset":8572,"ancestor_record_offset":7948,"inlinee":14447,"source_line":276,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":8900,"parent_record_offset":8636,"ancestor_record_offset":7948,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":762,"record_offset":8636,"parent_record_offset":8600,"ancestor_record_offset":7948,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":9428,"parent_record_offset":9380,"ancestor_record_offset":7948,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_closed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":9252,"parent_record_offset":9220,"ancestor_record_offset":7948,"inlinee":44849,"source_line":333,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_closed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF31
// 首条位置: {"module":762,"inlinee":44849}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9280,"parent_record_offset":9252,"ancestor_record_offset":9252,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":9348,"parent_record_offset":9308,"ancestor_record_offset":9252,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9380,"parent_record_offset":9308,"ancestor_record_offset":9252,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":9308,"parent_record_offset":9280,"ancestor_record_offset":9252,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":762,"record_offset":9428,"parent_record_offset":9380,"ancestor_record_offset":9252,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::eval_cubic
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 351
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF23
// 首条位置: {"module":762,"inlinee":44835}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::eval_cubic_deriv
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 356
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF24
// 首条位置: {"module":762,"inlinee":44836}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_clamped_spline
// 记录形式: compiled_procedure; instances=1
// 源码行: 361
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":24984,"rva":13191696,"va":5381900816}
// 编译实例 RVAs: 0xc94a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141941180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13288994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900816,"raw_hex":"e822c6ca00","size":5,"soft_fail":false,"target_va":5395190144,"va":5381901145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141941510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13288752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900816,"raw_hex":"e830c5ca00","size":5,"soft_fail":false,"target_va":5395191056,"va":5381902299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29945946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900816,"raw_hex":"e85af0c801","size":5,"soft_fail":false,"target_va":5411847200,"va":5381901249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29632336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900816,"raw_hex":"e85027c401","size":5,"soft_fail":false,"target_va":5411534720,"va":5381902379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29632818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900816,"raw_hex":"e83229c401","size":5,"soft_fail":false,"target_va":5411535312,"va":5381902489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29639290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900816,"raw_hex":"e87a42c401","size":5,"soft_fail":false,"target_va":5411541808,"va":5381902513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=19
//     evidence={"assembly":"\tcallq\t29640357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900816,"raw_hex":"e8a546c401","size":5,"soft_fail":false,"target_va":5411542296,"va":5381901934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":26160,"parent_record_offset":24984,"ancestor_record_offset":24984,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":25128,"parent_record_offset":24984,"ancestor_record_offset":24984,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":762,"record_offset":27708,"parent_record_offset":24984,"ancestor_record_offset":24984,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":27580,"parent_record_offset":24984,"ancestor_record_offset":24984,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":27516,"parent_record_offset":24984,"ancestor_record_offset":24984,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":25316,"parent_record_offset":25196,"ancestor_record_offset":24984,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":25196,"parent_record_offset":25128,"ancestor_record_offset":24984,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":25344,"parent_record_offset":25316,"ancestor_record_offset":24984,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":25376,"parent_record_offset":25344,"ancestor_record_offset":24984,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":25884,"parent_record_offset":25196,"ancestor_record_offset":24984,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":25472,"parent_record_offset":25376,"ancestor_record_offset":24984,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":25252,"parent_record_offset":25196,"ancestor_record_offset":24984,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":25284,"parent_record_offset":25252,"ancestor_record_offset":24984,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":25416,"parent_record_offset":25376,"ancestor_record_offset":24984,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":25580,"parent_record_offset":25556,"ancestor_record_offset":24984,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":25556,"parent_record_offset":25376,"ancestor_record_offset":24984,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":25784,"parent_record_offset":25744,"ancestor_record_offset":24984,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":25912,"parent_record_offset":25884,"ancestor_record_offset":24984,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":25820,"parent_record_offset":25744,"ancestor_record_offset":24984,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":25744,"parent_record_offset":25708,"ancestor_record_offset":24984,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":27780,"parent_record_offset":27744,"ancestor_record_offset":24984,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":25708,"parent_record_offset":25196,"ancestor_record_offset":24984,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":27744,"parent_record_offset":27708,"ancestor_record_offset":24984,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":762,"record_offset":25168,"parent_record_offset":25128,"ancestor_record_offset":24984,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":25604,"parent_record_offset":25580,"ancestor_record_offset":24984,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":25844,"parent_record_offset":25820,"ancestor_record_offset":24984,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":28692,"parent_record_offset":28524,"ancestor_record_offset":24984,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":26088,"parent_record_offset":25912,"ancestor_record_offset":24984,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":25672,"parent_record_offset":25344,"ancestor_record_offset":24984,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":25520,"parent_record_offset":25496,"ancestor_record_offset":24984,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":25496,"parent_record_offset":25472,"ancestor_record_offset":24984,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":25628,"parent_record_offset":25604,"ancestor_record_offset":24984,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_clamped_spline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1614E
// 首条位置: {"module":2079,"inlinee":90446}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2079,"record_offset":764,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/02079.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_clamped_spline::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":2079,"record_offset":892,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":90449,"source_line":374,"archive":"_pdb/B/modules/02079.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_clamped_spline::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":2079,"record_offset":808,"parent_record_offset":764,"ancestor_record_offset":712,"inlinee":90448,"source_line":376,"archive":"_pdb/B/modules/02079.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_clamped_spline::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 374
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16151
// 首条位置: {"module":2079,"inlinee":90449}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_clamped_spline::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 376
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16150
// 首条位置: {"module":2079,"inlinee":90448}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_clamped_spline::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 408
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16154
// 首条位置: {"module":2079,"inlinee":90452}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::compute_periodic_spline
// 记录形式: compiled_procedure; instances=1
// 源码行: 422
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":28764,"rva":13193728,"va":5381902848}
// 编译实例 RVAs: 0xc95200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141941040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13286672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902848,"raw_hex":"e810bdca00","size":5,"soft_fail":false,"target_va":5395189824,"va":5381903147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141941340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13285648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902848,"raw_hex":"e810b9ca00","size":5,"soft_fail":false,"target_va":5395190592,"va":5381904939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29629632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902848,"raw_hex":"e8c01cc401","size":5,"soft_fail":false,"target_va":5411534720,"va":5381905083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t29629986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902848,"raw_hex":"e8221ec401","size":5,"soft_fail":false,"target_va":5411535312,"va":5381905321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29636458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902848,"raw_hex":"e86a37c401","size":5,"soft_fail":false,"target_va":5411541808,"va":5381905345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t29636928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902848,"raw_hex":"e84039c401","size":5,"soft_fail":false,"target_va":5411542296,"va":5381905363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29638168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902848,"raw_hex":"e8183ec401","size":5,"soft_fail":false,"target_va":5411543584,"va":5381905411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::natural_cubic::solve_tridiagonal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902848,"raw_hex":"e80c060000","size":5,"soft_fail":false,"target_va":5381905504,"va":5381903951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":28900,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::solve_cyclic
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":32432,"parent_record_offset":28764,"ancestor_record_offset":28764,"inlinee":44880,"source_line":484,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":29100,"parent_record_offset":28976,"ancestor_record_offset":28764,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":32520,"parent_record_offset":32432,"ancestor_record_offset":28764,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":28976,"parent_record_offset":28900,"ancestor_record_offset":28764,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":28764,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":29160,"parent_record_offset":29128,"ancestor_record_offset":28764,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":29632,"parent_record_offset":28976,"ancestor_record_offset":28764,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":29256,"parent_record_offset":29160,"ancestor_record_offset":28764,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":29036,"parent_record_offset":28976,"ancestor_record_offset":28764,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":762,"record_offset":29068,"parent_record_offset":29036,"ancestor_record_offset":28764,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":29200,"parent_record_offset":29160,"ancestor_record_offset":28764,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":29364,"parent_record_offset":29340,"ancestor_record_offset":28764,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":29340,"parent_record_offset":29160,"ancestor_record_offset":28764,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":29580,"parent_record_offset":29536,"ancestor_record_offset":28764,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":29656,"parent_record_offset":29632,"ancestor_record_offset":28764,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":31408,"parent_record_offset":31384,"ancestor_record_offset":28764,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":29536,"parent_record_offset":29492,"ancestor_record_offset":28764,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":29492,"parent_record_offset":28976,"ancestor_record_offset":28764,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":762,"record_offset":28948,"parent_record_offset":28900,"ancestor_record_offset":28764,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":29388,"parent_record_offset":29364,"ancestor_record_offset":28764,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":762,"record_offset":31432,"parent_record_offset":31408,"ancestor_record_offset":28764,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":762,"record_offset":30496,"parent_record_offset":30468,"ancestor_record_offset":28764,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":34956,"parent_record_offset":34788,"ancestor_record_offset":28764,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":29768,"parent_record_offset":29656,"ancestor_record_offset":28764,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":29456,"parent_record_offset":29128,"ancestor_record_offset":28764,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":29304,"parent_record_offset":29280,"ancestor_record_offset":28764,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":29280,"parent_record_offset":29256,"ancestor_record_offset":28764,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":29412,"parent_record_offset":29388,"ancestor_record_offset":28764,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::compute_periodic_spline::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":30524,"parent_record_offset":30496,"ancestor_record_offset":28764,"inlinee":44877,"source_line":437,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::compute_periodic_spline::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":31316,"parent_record_offset":31284,"ancestor_record_offset":28764,"inlinee":44878,"source_line":438,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::compute_periodic_spline::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":32228,"parent_record_offset":32196,"ancestor_record_offset":28764,"inlinee":44879,"source_line":439,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_cyclic::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":33268,"parent_record_offset":33240,"ancestor_record_offset":28764,"inlinee":44882,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_cyclic::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":34048,"parent_record_offset":34016,"ancestor_record_offset":28764,"inlinee":44883,"source_line":514,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::compute_periodic_spline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 431
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1614C
// 首条位置: {"module":2079,"inlinee":90444}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::compute_periodic_spline::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 437
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF4D
// 首条位置: {"module":762,"inlinee":44877}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::compute_periodic_spline::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 438
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF4E
// 首条位置: {"module":762,"inlinee":44878}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::compute_periodic_spline::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 439
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF4F
// 首条位置: {"module":762,"inlinee":44879}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::compute_periodic_spline::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 443
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16152
// 首条位置: {"module":2079,"inlinee":90450}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_cyclic
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 484
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF50
// 首条位置: {"module":762,"inlinee":44880}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":32520,"parent_record_offset":32432,"ancestor_record_offset":32432,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":32616,"parent_record_offset":32556,"ancestor_record_offset":32432,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":32556,"parent_record_offset":32520,"ancestor_record_offset":32432,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":32644,"parent_record_offset":32616,"ancestor_record_offset":32432,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":32676,"parent_record_offset":32644,"ancestor_record_offset":32432,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":33156,"parent_record_offset":32556,"ancestor_record_offset":32432,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":762,"record_offset":32772,"parent_record_offset":32676,"ancestor_record_offset":32432,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":32716,"parent_record_offset":32676,"ancestor_record_offset":32432,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":32880,"parent_record_offset":32856,"ancestor_record_offset":32432,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":762,"record_offset":32856,"parent_record_offset":32676,"ancestor_record_offset":32432,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":33116,"parent_record_offset":33080,"ancestor_record_offset":32432,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":762,"record_offset":33184,"parent_record_offset":33156,"ancestor_record_offset":32432,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":33080,"parent_record_offset":33044,"ancestor_record_offset":32432,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":762,"record_offset":33044,"parent_record_offset":32556,"ancestor_record_offset":32432,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":762,"record_offset":32904,"parent_record_offset":32880,"ancestor_record_offset":32432,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":762,"record_offset":33240,"parent_record_offset":33212,"ancestor_record_offset":32432,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":33212,"parent_record_offset":33184,"ancestor_record_offset":32432,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":762,"record_offset":32972,"parent_record_offset":32644,"ancestor_record_offset":32432,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":32820,"parent_record_offset":32796,"ancestor_record_offset":32432,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":762,"record_offset":32796,"parent_record_offset":32772,"ancestor_record_offset":32432,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":762,"record_offset":32928,"parent_record_offset":32904,"ancestor_record_offset":32432,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_cyclic::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":33268,"parent_record_offset":33240,"ancestor_record_offset":32432,"inlinee":44882,"source_line":500,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_cyclic::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":762,"record_offset":34048,"parent_record_offset":34016,"ancestor_record_offset":32432,"inlinee":44883,"source_line":514,"archive":"_pdb/B/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_cyclic::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 500
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF52
// 首条位置: {"module":762,"inlinee":44882}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_cyclic::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 514
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF53
// 首条位置: {"module":762,"inlinee":44883}
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
// 完整结构及原始类型记录: natural_cubic.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::natural_cubic::Spline2D
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::natural_cubic::Spline3D
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<utils::natural_cubic::Spline3D>", "constructor": "core::ptr::drop_in_place", "arguments": ["utils::natural_cubic::Spline3D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::natural_cubic::SplineSegment2D
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::natural_cubic::SplineSegment3D
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: natural_cubic.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: natural_cubic.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline2D::find_segment_and_dt
// 信息 ID: 140b51315a168c80c3d03df9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline2D::from_control_points_with_tangents
// 信息 ID: d4cde472bb044b0e410e5893
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs::generate_playermade_stair_requests<false> @ 0x2796a2 -> 0xc926b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs::generate_playermade_stair_requests<true> @ 0x280cc7 -> 0xc926b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline2D::from_periodic_loop
// 信息 ID: 496545fad69127512041505f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs::generate_playermade_stair_requests<false> @ 0x27927b -> 0xc930f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs::generate_playermade_stair_requests<true> @ 0x28089b -> 0xc930f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::find_segment_and_dt
// 信息 ID: a974b4cfe694131a1ababeb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=762, inline_record=22792; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=762, inline_record=23616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::from_control_points_with_tangents
// 信息 ID: e2e845aa01e30244c5987a3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3bc178 -> 0xc93970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::position_at_t
// 信息 ID: 2f96cc9903985c84ebf3cf3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::RollercoasterState::new @ 0x3b6cdb -> 0xc946a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b8ca7 -> 0xc946a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b9b04 -> 0xc946a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b9f94 -> 0xc946a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::tangent_at_t
// 信息 ID: 0d06271038bba3f154202cbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b8616 -> 0xc94800 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b8cba -> 0xc94800 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b8cd6 -> 0xc94800 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b9b1a -> 0xc94800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::to_owned
// 信息 ID: 018c91676b5bca069836765c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3bc18b -> 0xc94940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::SplineSegment2D::position_at_local_t
// 信息 ID: 010f884a60fa70aa5ed4fe58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::stairs::densely_sample_spline_segment::closure_env$1> > @ 0x1261640 -> 0xc92610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::SplineSegment2D::tangent_at_local_t
// 信息 ID: fe23dbdcca1a057180008dbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs::densely_sample_spline_segment @ 0x809bf0 -> 0xc92660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs::densely_sample_spline_segment @ 0x809c02 -> 0xc92660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::SplineSegment3D::position_at_local_t
// 信息 ID: 537706d5cfe562161d43b4e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=762, inline_record=23360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::SplineSegment3D::tangent_at_local_t
// 信息 ID: 5b28b5b6fef824de537b4dde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=762, inline_record=24184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_closed
// 信息 ID: a64b91476c0e673f758953ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=762, inline_record=7948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_closed::closure$0
// 信息 ID: b823846db28eb61cd7fedc94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::build_knots_closed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=762, inline_record=9252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_open
// 信息 ID: 6d78e574572735250c35d598
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=762, inline_record=836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_open3
// 信息 ID: 100ba79ef4fd1d58090c39bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=762, inline_record=15076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_open3::closure$0
// 信息 ID: b1de6ab119094c79dc41739a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::build_knots_open3", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=762, inline_record=15792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_open::closure$0
// 信息 ID: 7e122521887ece39985598ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::build_knots_open", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=762, inline_record=1552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline
// 信息 ID: ceb9cddefbc630d74a632b5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::natural_cubic::Spline2D::from_periodic_loop @ 0xc934dd -> 0xc95200 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline2D::from_periodic_loop @ 0xc93675 -> 0xc95200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$0
// 信息 ID: b9bf290f06ac59daffa60832
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// [闭包环境类型引用] utils::natural_cubic::compute_periodic_spline::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::compute_periodic_spline::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::compute_periodic_spline::closure_env$0>; module=2079, inline_record=460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$1
// 信息 ID: 5bd0a5f24dd11345807e17a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=762, inline_record=30524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$2
// 信息 ID: f44dfdccd9ad50c6d86fa00f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=762, inline_record=31316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$3
// 信息 ID: 9a1da4bb1f186dc863aec049
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=762, inline_record=32228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$4
// 信息 ID: e995e53615576c5f1b5f3e1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// [闭包环境类型引用] utils::natural_cubic::compute_periodic_spline::closure_env$4 <- core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::compute_periodic_spline::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::compute_periodic_spline::closure_env$4>; module=2079, inline_record=1148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::eval_cubic
// 信息 ID: ba98fb8402949173b967afb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::SplineSegment2D::position_at_local_t; module=762, inline_record=396; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::SplineSegment2D::position_at_local_t; module=762, inline_record=420; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=762, inline_record=23392; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=762, inline_record=23420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::eval_cubic_deriv
// 信息 ID: 6c339bb411f9625ad0ea16fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::SplineSegment2D::tangent_at_local_t; module=762, inline_record=596; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::SplineSegment2D::tangent_at_local_t; module=762, inline_record=624; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=762, inline_record=24216; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=762, inline_record=24244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::find_segment_and_dt::closure$0
// 信息 ID: 97810e75e481eef8810ac9df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::impl$2::find_segment_and_dt", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$0
// 信息 ID: 7d124fcd3199c4688ea414f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::impl$2::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=762, inline_record=4316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$4
// 信息 ID: eeb676ab790fa2fb43b54961
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "utils::natural_cubic::impl$2::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=762, inline_record=5804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$7
// 信息 ID: f4c74c744d551f210e5c5e0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "utils::natural_cubic::impl$2::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=762, inline_record=6868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_periodic_loop::closure$0
// 信息 ID: ea5950806bbb6652be55df20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::impl$2::from_periodic_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=762, inline_record=11468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_periodic_loop::closure$2
// 信息 ID: d43b4a8f6281c90ed18edb28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "utils::natural_cubic::impl$2::from_periodic_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=762, inline_record=12964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_periodic_loop::closure$3
// 信息 ID: 50501a375a43abddfc9c9f56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "utils::natural_cubic::impl$2::from_periodic_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=762, inline_record=13976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::find_segment_and_dt::closure$0
// 信息 ID: 61cc4737952de46bc80b2260
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::impl$3::find_segment_and_dt", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=762, inline_record=23132; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=762, inline_record=23292; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=762, inline_record=23956; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=762, inline_record=24116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$0
// 信息 ID: 4b04ab74b301907de8276f37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::impl$3::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=762, inline_record=18616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$10
// 信息 ID: 1d80ac30795a4d7eaa8aac35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "utils::natural_cubic::impl$3::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=762, inline_record=21728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$4
// 信息 ID: 41be16735c8a279507ae9578
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "utils::natural_cubic::impl$3::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=762, inline_record=20124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$7
// 信息 ID: 11d86712a1b7900f075824dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "utils::natural_cubic::impl$3::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=762, inline_record=20520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline
// 信息 ID: 723cf2769bdbec1940175079
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::natural_cubic::Spline2D::from_control_points_with_tangents @ 0xc92c22 -> 0xc94a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline2D::from_control_points_with_tangents @ 0xc92df7 -> 0xc94a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline3D::from_control_points_with_tangents @ 0xc93f6d -> 0xc94a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline3D::from_control_points_with_tangents @ 0xc94153 -> 0xc94a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline3D::from_control_points_with_tangents @ 0xc94323 -> 0xc94a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline::closure$0
// 信息 ID: 4c84d6467dfa1238b9edaab9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::solve_clamped_spline", "captures": "unknown"}
// [闭包环境类型引用] utils::natural_cubic::solve_clamped_spline::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::solve_clamped_spline::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::solve_clamped_spline::closure_env$0>; module=2079, inline_record=712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline::closure$0::closure$0
// 信息 ID: 0bc872068653aa395717df97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::solve_clamped_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::solve_clamped_spline::closure_env$0>; module=2079, inline_record=892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline::closure$0::closure$1
// 信息 ID: a3d67bd5d7871e4e3bedae12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::solve_clamped_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::solve_clamped_spline::closure_env$0>; module=2079, inline_record=808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline::closure$1
// 信息 ID: 499bbd6206155710a5a3dba8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::natural_cubic::solve_clamped_spline", "captures": "unknown"}
// [闭包环境类型引用] utils::natural_cubic::solve_clamped_spline::closure_env$1 <- core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::solve_clamped_spline::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,utils::natural_cubic::solve_clamped_spline::closure_env$1>; module=2079, inline_record=1428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_cyclic
// 信息 ID: 97a08f629e7a64e2fad78b18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=762, inline_record=32432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_cyclic::closure$0
// 信息 ID: 809b697fbaa73024c9543b83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::solve_cyclic", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=762, inline_record=33268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_cyclic::closure$1
// 信息 ID: ccd648189e85346afb50ed3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::natural_cubic::solve_cyclic", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=762, inline_record=34048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_tridiagonal
// 信息 ID: e2446ccc3be8a549d5a5da7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::natural_cubic::compute_periodic_spline @ 0xc9564f -> 0xc95c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::compute_periodic_spline @ 0xc95682 -> 0xc95c60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
