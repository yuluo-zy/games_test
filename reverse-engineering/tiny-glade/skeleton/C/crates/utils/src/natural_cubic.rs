// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/natural_cubic.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d466725da68f127c4b251f2e672449ec145e6330b0fb944b197116265eddef28
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: natural_cubic.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 757
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":757,"record_offset":664,"rva":14574736,"va":5383283856}
// 编译实例 RVAs: 0xde6490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33773130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383283856,"raw_hex":"e84a560302","size":5,"soft_fail":false,"target_va":5417057088,"va":5383283953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33328846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383283856,"raw_hex":"e8ce8efc01","size":5,"soft_fail":false,"target_va":5416613232,"va":5383284381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33329481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383283856,"raw_hex":"e84991fc01","size":5,"soft_fail":false,"target_va":5416613920,"va":5383284434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33337308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383283856,"raw_hex":"e8dcaffc01","size":5,"soft_fail":false,"target_va":5416621762,"va":5383284449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":796,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":48976,"source_line":1681,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":757,"record_offset":2152,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":2056,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":928,"parent_record_offset":828,"ancestor_record_offset":664,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":828,"parent_record_offset":796,"ancestor_record_offset":664,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":956,"parent_record_offset":928,"ancestor_record_offset":664,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":988,"parent_record_offset":956,"ancestor_record_offset":664,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":664,"inlinee":5932,"source_line":1865,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":1084,"parent_record_offset":988,"ancestor_record_offset":664,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":872,"parent_record_offset":828,"ancestor_record_offset":664,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":664,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":1028,"parent_record_offset":988,"ancestor_record_offset":664,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":664,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":1224,"parent_record_offset":988,"ancestor_record_offset":664,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":1404,"parent_record_offset":1376,"ancestor_record_offset":664,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":2268,"parent_record_offset":2224,"ancestor_record_offset":664,"inlinee":10953,"source_line":217,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":664,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":2224,"parent_record_offset":2188,"ancestor_record_offset":664,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":1348,"parent_record_offset":828,"ancestor_record_offset":664,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":2188,"parent_record_offset":2152,"ancestor_record_offset":664,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":664,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":2300,"parent_record_offset":2268,"ancestor_record_offset":664,"inlinee":10954,"source_line":1011,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":1312,"parent_record_offset":956,"ancestor_record_offset":664,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":664,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":664,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":1440,"parent_record_offset":828,"ancestor_record_offset":664,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":757,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":664,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::SplineSegment2D::position_at_local_t
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF66
// 首条位置: {"module":757,"inlinee":48998}
// 编译实例 RVAs: 0xde6440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::natural_cubic::eval_cubic
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":608,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":48975,"source_line":351,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::SplineSegment2D::tangent_at_local_t
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 29, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF65
// 首条位置: {"module":757,"inlinee":48997}
// 编译实例 RVAs: 0xde63f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::natural_cubic::eval_cubic_deriv
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":404,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":48974,"source_line":356,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::SplineSegment3D::position_at_local_t
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF7C
// 首条位置: {"module":757,"inlinee":49020}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":26900,"parent_record_offset":26776,"ancestor_record_offset":26776,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::eval_cubic
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":26812,"parent_record_offset":26776,"ancestor_record_offset":26776,"inlinee":48975,"source_line":351,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::SplineSegment3D::tangent_at_local_t
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF7B
// 首条位置: {"module":757,"inlinee":49019}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":26608,"parent_record_offset":26484,"ancestor_record_offset":26484,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::eval_cubic_deriv
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":26520,"parent_record_offset":26484,"ancestor_record_offset":26484,"inlinee":48974,"source_line":356,"archive":"_pdb/C/modules/00757.jsonl.gz"}
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
// 首条位置: {"module":757,"record_offset":19688,"rva":14582064,"va":5383291184}
// 编译实例 RVAs: 0xde8130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33765406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383291184,"raw_hex":"e81e380302","size":5,"soft_fail":false,"target_va":5417057088,"va":5383291677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33320537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383291184,"raw_hex":"e8596efc01","size":5,"soft_fail":false,"target_va":5416613232,"va":5383292690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17081796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383291184,"raw_hex":"e8c4a50401","size":5,"soft_fail":false,"target_va":5400373936,"va":5383292135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33320880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383291184,"raw_hex":"e8b06ffc01","size":5,"soft_fail":false,"target_va":5416613920,"va":5383293035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33328431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383291184,"raw_hex":"e82f8dfc01","size":5,"soft_fail":false,"target_va":5416621536,"va":5383293100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33329144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383291184,"raw_hex":"e8f88ffc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383293059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::natural_cubic::solve_clamped_spline
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383291184,"raw_hex":"e85ce1ffff","size":5,"soft_fail":false,"target_va":5383284512,"va":5383292351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":22204,"parent_record_offset":19688,"ancestor_record_offset":19688,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::from_iter_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":23164,"parent_record_offset":19688,"ancestor_record_offset":19688,"inlinee":49002,"source_line":604,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":24192,"parent_record_offset":19688,"ancestor_record_offset":19688,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":24052,"parent_record_offset":19688,"ancestor_record_offset":19688,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":24532,"parent_record_offset":19688,"ancestor_record_offset":19688,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":25640,"parent_record_offset":19688,"ancestor_record_offset":19688,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::build_knots_open
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":19864,"parent_record_offset":19688,"ancestor_record_offset":19688,"inlinee":49012,"source_line":180,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20080,"parent_record_offset":19964,"ancestor_record_offset":19688,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":21344,"parent_record_offset":19864,"ancestor_record_offset":19688,"inlinee":48976,"source_line":1681,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":19936,"parent_record_offset":19864,"ancestor_record_offset":19688,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":19964,"parent_record_offset":19936,"ancestor_record_offset":19688,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":23820,"parent_record_offset":23764,"ancestor_record_offset":19688,"inlinee":21951,"source_line":2228,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":23852,"parent_record_offset":23820,"ancestor_record_offset":19688,"inlinee":21952,"source_line":2221,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20108,"parent_record_offset":20080,"ancestor_record_offset":19688,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20140,"parent_record_offset":20108,"ancestor_record_offset":19688,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":23792,"parent_record_offset":23764,"ancestor_record_offset":19688,"inlinee":21950,"source_line":158,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":23764,"parent_record_offset":23740,"ancestor_record_offset":19688,"inlinee":21949,"source_line":701,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":23320,"parent_record_offset":23296,"ancestor_record_offset":19688,"inlinee":21942,"source_line":607,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":23420,"parent_record_offset":23392,"ancestor_record_offset":19688,"inlinee":21939,"source_line":137,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":23296,"parent_record_offset":23272,"ancestor_record_offset":19688,"inlinee":21941,"source_line":429,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$6::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":23740,"parent_record_offset":23716,"ancestor_record_offset":19688,"inlinee":21948,"source_line":713,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":23208,"parent_record_offset":23164,"ancestor_record_offset":19688,"inlinee":21935,"source_line":545,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":23356,"parent_record_offset":23236,"ancestor_record_offset":19688,"inlinee":21936,"source_line":1451,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":23392,"parent_record_offset":23356,"ancestor_record_offset":19688,"inlinee":21937,"source_line":1422,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":23272,"parent_record_offset":23236,"ancestor_record_offset":19688,"inlinee":21940,"source_line":736,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$18::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":23236,"parent_record_offset":23164,"ancestor_record_offset":19688,"inlinee":49004,"source_line":2311,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":25328,"parent_record_offset":24936,"ancestor_record_offset":19688,"inlinee":5932,"source_line":1865,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20500,"parent_record_offset":19964,"ancestor_record_offset":19688,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":24876,"parent_record_offset":24808,"ancestor_record_offset":19688,"inlinee":49009,"source_line":1556,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20236,"parent_record_offset":20140,"ancestor_record_offset":19688,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":20024,"parent_record_offset":19964,"ancestor_record_offset":19688,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":20048,"parent_record_offset":20024,"ancestor_record_offset":19688,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":20180,"parent_record_offset":20140,"ancestor_record_offset":19688,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":19688,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":20376,"parent_record_offset":20140,"ancestor_record_offset":19688,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":22076,"parent_record_offset":22052,"ancestor_record_offset":19688,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":21240,"parent_record_offset":21204,"ancestor_record_offset":19688,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":22052,"parent_record_offset":21972,"ancestor_record_offset":19688,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":20936,"parent_record_offset":20904,"ancestor_record_offset":19688,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":757,"record_offset":22760,"parent_record_offset":22732,"ancestor_record_offset":19688,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":20532,"parent_record_offset":20500,"ancestor_record_offset":19688,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":757,"record_offset":22020,"parent_record_offset":21996,"ancestor_record_offset":19688,"inlinee":10930,"source_line":91,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":21996,"parent_record_offset":21972,"ancestor_record_offset":19688,"inlinee":5521,"source_line":587,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":21948,"parent_record_offset":21924,"ancestor_record_offset":19688,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":24216,"parent_record_offset":24192,"ancestor_record_offset":19688,"inlinee":9710,"source_line":99,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":22120,"parent_record_offset":19864,"ancestor_record_offset":19688,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":19688,"inlinee":9711,"source_line":275,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":21972,"parent_record_offset":21948,"ancestor_record_offset":19688,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":22152,"parent_record_offset":22120,"ancestor_record_offset":19688,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":21272,"parent_record_offset":21204,"ancestor_record_offset":19688,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":21204,"parent_record_offset":21172,"ancestor_record_offset":19688,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":21172,"parent_record_offset":19964,"ancestor_record_offset":19688,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":24456,"parent_record_offset":24240,"ancestor_record_offset":19688,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":21924,"parent_record_offset":19864,"ancestor_record_offset":19688,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":20424,"parent_record_offset":20400,"ancestor_record_offset":19688,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":19688,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":20736,"parent_record_offset":20700,"ancestor_record_offset":19688,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":20600,"parent_record_offset":20564,"ancestor_record_offset":19688,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":22888,"parent_record_offset":22760,"ancestor_record_offset":19688,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":20564,"parent_record_offset":20532,"ancestor_record_offset":19688,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":20464,"parent_record_offset":20108,"ancestor_record_offset":19688,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":23692,"parent_record_offset":23164,"ancestor_record_offset":19688,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":23916,"parent_record_offset":23820,"ancestor_record_offset":19688,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":20284,"parent_record_offset":20260,"ancestor_record_offset":19688,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":20260,"parent_record_offset":20236,"ancestor_record_offset":19688,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":21092,"parent_record_offset":21028,"ancestor_record_offset":19688,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":21064,"parent_record_offset":21028,"ancestor_record_offset":19688,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":21136,"parent_record_offset":19964,"ancestor_record_offset":19688,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":757,"record_offset":21028,"parent_record_offset":20996,"ancestor_record_offset":19688,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20840,"parent_record_offset":20808,"ancestor_record_offset":19688,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20808,"parent_record_offset":20636,"ancestor_record_offset":19688,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20700,"parent_record_offset":20668,"ancestor_record_offset":19688,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20668,"parent_record_offset":20636,"ancestor_record_offset":19688,"inlinee":27235,"source_line":1548,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":757,"record_offset":20996,"parent_record_offset":20532,"ancestor_record_offset":19688,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":20904,"parent_record_offset":20840,"ancestor_record_offset":19688,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_open::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":20636,"parent_record_offset":20600,"ancestor_record_offset":19688,"inlinee":49014,"source_line":184,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":24484,"parent_record_offset":24456,"ancestor_record_offset":19688,"inlinee":49016,"source_line":96,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":25556,"parent_record_offset":25532,"ancestor_record_offset":19688,"inlinee":49017,"source_line":108,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF78
// 首条位置: {"module":757,"inlinee":49016}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF79
// 首条位置: {"module":757,"inlinee":49017}
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
// 首条位置: {"module":757,"record_offset":13348,"rva":14580320,"va":5383289440}
// 编译实例 RVAs: 0xde7a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33767358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383289440,"raw_hex":"e8be3f0302","size":5,"soft_fail":false,"target_va":5417057088,"va":5383289725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33322646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383289440,"raw_hex":"e89676fc01","size":5,"soft_fail":false,"target_va":5416613232,"va":5383290581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17083793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383289440,"raw_hex":"e891ad0401","size":5,"soft_fail":false,"target_va":5400373936,"va":5383290138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33323006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383289440,"raw_hex":"e8fe77fc01","size":5,"soft_fail":false,"target_va":5416613920,"va":5383290909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33330557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383289440,"raw_hex":"e87d95fc01","size":5,"soft_fail":false,"target_va":5416621536,"va":5383290974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33331270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383289440,"raw_hex":"e84698fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383290933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::natural_cubic::compute_periodic_spline
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383289440,"raw_hex":"e8f4f1ffff","size":5,"soft_fail":false,"target_va":5383286704,"va":5383290295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":15576,"parent_record_offset":13348,"ancestor_record_offset":13348,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::from_iter_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":16544,"parent_record_offset":13348,"ancestor_record_offset":13348,"inlinee":49002,"source_line":604,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":17580,"parent_record_offset":13348,"ancestor_record_offset":13348,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":17440,"parent_record_offset":13348,"ancestor_record_offset":13348,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":18972,"parent_record_offset":13348,"ancestor_record_offset":13348,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::build_knots_closed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":13504,"parent_record_offset":13348,"ancestor_record_offset":13348,"inlinee":48999,"source_line":327,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13704,"parent_record_offset":13604,"ancestor_record_offset":13348,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":14912,"parent_record_offset":13504,"ancestor_record_offset":13348,"inlinee":48976,"source_line":1681,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13576,"parent_record_offset":13504,"ancestor_record_offset":13348,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13604,"parent_record_offset":13576,"ancestor_record_offset":13348,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":17208,"parent_record_offset":17152,"ancestor_record_offset":13348,"inlinee":21951,"source_line":2228,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":17240,"parent_record_offset":17208,"ancestor_record_offset":13348,"inlinee":21952,"source_line":2221,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13732,"parent_record_offset":13704,"ancestor_record_offset":13348,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13764,"parent_record_offset":13732,"ancestor_record_offset":13348,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":17180,"parent_record_offset":17152,"ancestor_record_offset":13348,"inlinee":21950,"source_line":158,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":17152,"parent_record_offset":17128,"ancestor_record_offset":13348,"inlinee":21949,"source_line":701,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":16700,"parent_record_offset":16676,"ancestor_record_offset":13348,"inlinee":21942,"source_line":607,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":16808,"parent_record_offset":16776,"ancestor_record_offset":13348,"inlinee":21939,"source_line":137,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":16676,"parent_record_offset":16652,"ancestor_record_offset":13348,"inlinee":21941,"source_line":429,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$6::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":17128,"parent_record_offset":17104,"ancestor_record_offset":13348,"inlinee":21948,"source_line":713,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":16588,"parent_record_offset":16544,"ancestor_record_offset":13348,"inlinee":21935,"source_line":545,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":16736,"parent_record_offset":16616,"ancestor_record_offset":13348,"inlinee":21936,"source_line":1451,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":16776,"parent_record_offset":16736,"ancestor_record_offset":13348,"inlinee":21937,"source_line":1422,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":16652,"parent_record_offset":16616,"ancestor_record_offset":13348,"inlinee":21940,"source_line":736,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$18::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":16616,"parent_record_offset":16544,"ancestor_record_offset":13348,"inlinee":49004,"source_line":2311,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":18660,"parent_record_offset":18268,"ancestor_record_offset":13348,"inlinee":5932,"source_line":1865,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":14124,"parent_record_offset":13604,"ancestor_record_offset":13348,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":18208,"parent_record_offset":18140,"ancestor_record_offset":13348,"inlinee":49009,"source_line":1556,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13860,"parent_record_offset":13764,"ancestor_record_offset":13348,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":13648,"parent_record_offset":13604,"ancestor_record_offset":13348,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":13348,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":13804,"parent_record_offset":13764,"ancestor_record_offset":13348,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":14024,"parent_record_offset":14000,"ancestor_record_offset":13348,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":14000,"parent_record_offset":13764,"ancestor_record_offset":13348,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":17536,"parent_record_offset":17512,"ancestor_record_offset":13348,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":14868,"parent_record_offset":14784,"ancestor_record_offset":13348,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":17512,"parent_record_offset":17488,"ancestor_record_offset":13348,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":14440,"parent_record_offset":14412,"ancestor_record_offset":13348,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":757,"record_offset":16308,"parent_record_offset":16280,"ancestor_record_offset":13348,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cycle.rs
//     evidence={"module":757,"record_offset":14564,"parent_record_offset":14532,"ancestor_record_offset":13348,"inlinee":9870,"source_line":34,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":14152,"parent_record_offset":14124,"ancestor_record_offset":13348,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":17464,"parent_record_offset":17440,"ancestor_record_offset":13348,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":13348,"inlinee":9710,"source_line":99,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":15492,"parent_record_offset":13504,"ancestor_record_offset":13348,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":17628,"parent_record_offset":17604,"ancestor_record_offset":13348,"inlinee":9711,"source_line":275,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":17488,"parent_record_offset":17464,"ancestor_record_offset":13348,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":15524,"parent_record_offset":15492,"ancestor_record_offset":13348,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":14812,"parent_record_offset":14784,"ancestor_record_offset":13348,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":13348,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":14760,"parent_record_offset":13604,"ancestor_record_offset":13348,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":17844,"parent_record_offset":17628,"ancestor_record_offset":13348,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":13348,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":14836,"parent_record_offset":14812,"ancestor_record_offset":13348,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":14336,"parent_record_offset":14296,"ancestor_record_offset":13348,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":14212,"parent_record_offset":14180,"ancestor_record_offset":13348,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":16340,"parent_record_offset":16308,"ancestor_record_offset":13348,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":14180,"parent_record_offset":14152,"ancestor_record_offset":13348,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":14088,"parent_record_offset":13732,"ancestor_record_offset":13348,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":17080,"parent_record_offset":16544,"ancestor_record_offset":13348,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":17304,"parent_record_offset":17208,"ancestor_record_offset":13348,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":13348,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":13348,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":13348,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":14620,"parent_record_offset":14596,"ancestor_record_offset":13348,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":14728,"parent_record_offset":13604,"ancestor_record_offset":13348,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":757,"record_offset":14596,"parent_record_offset":14564,"ancestor_record_offset":13348,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":13348,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14364,"parent_record_offset":14296,"ancestor_record_offset":13348,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14296,"parent_record_offset":14268,"ancestor_record_offset":13348,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$10::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":757,"record_offset":14500,"parent_record_offset":14152,"ancestor_record_offset":13348,"inlinee":9887,"source_line":276,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":757,"record_offset":14532,"parent_record_offset":14500,"ancestor_record_offset":13348,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":14412,"parent_record_offset":14364,"ancestor_record_offset":13348,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_closed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":14244,"parent_record_offset":14212,"ancestor_record_offset":13348,"inlinee":49001,"source_line":333,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_periodic_loop::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":17872,"parent_record_offset":17844,"ancestor_record_offset":13348,"inlinee":49006,"source_line":131,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$2::from_periodic_loop::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":18888,"parent_record_offset":18864,"ancestor_record_offset":13348,"inlinee":49007,"source_line":135,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$2::from_periodic_loop::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF6E
// 首条位置: {"module":757,"inlinee":49006}
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
// IPI IDs: 0xBF6F
// 首条位置: {"module":757,"inlinee":49007}
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
// 记录形式: inlinee_declaration; instances=1
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF73
// 首条位置: {"module":757,"inlinee":49011}
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
// IPI IDs: 0xBF74
// 首条位置: {"module":757,"inlinee":49012}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":21344,"parent_record_offset":19864,"ancestor_record_offset":19864,"inlinee":48976,"source_line":1681,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":19936,"parent_record_offset":19864,"ancestor_record_offset":19864,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":22120,"parent_record_offset":19864,"ancestor_record_offset":19864,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":21924,"parent_record_offset":19864,"ancestor_record_offset":19864,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20080,"parent_record_offset":19964,"ancestor_record_offset":19864,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":19964,"parent_record_offset":19936,"ancestor_record_offset":19864,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20108,"parent_record_offset":20080,"ancestor_record_offset":19864,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20140,"parent_record_offset":20108,"ancestor_record_offset":19864,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20500,"parent_record_offset":19964,"ancestor_record_offset":19864,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":20236,"parent_record_offset":20140,"ancestor_record_offset":19864,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":20024,"parent_record_offset":19964,"ancestor_record_offset":19864,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":20048,"parent_record_offset":20024,"ancestor_record_offset":19864,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":20180,"parent_record_offset":20140,"ancestor_record_offset":19864,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":19864,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":20376,"parent_record_offset":20140,"ancestor_record_offset":19864,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":22076,"parent_record_offset":22052,"ancestor_record_offset":19864,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":21240,"parent_record_offset":21204,"ancestor_record_offset":19864,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":22052,"parent_record_offset":21972,"ancestor_record_offset":19864,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":20936,"parent_record_offset":20904,"ancestor_record_offset":19864,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":20532,"parent_record_offset":20500,"ancestor_record_offset":19864,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":757,"record_offset":22020,"parent_record_offset":21996,"ancestor_record_offset":19864,"inlinee":10930,"source_line":91,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":21996,"parent_record_offset":21972,"ancestor_record_offset":19864,"inlinee":5521,"source_line":587,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":21948,"parent_record_offset":21924,"ancestor_record_offset":19864,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":21972,"parent_record_offset":21948,"ancestor_record_offset":19864,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":22152,"parent_record_offset":22120,"ancestor_record_offset":19864,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":21272,"parent_record_offset":21204,"ancestor_record_offset":19864,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":21204,"parent_record_offset":21172,"ancestor_record_offset":19864,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":21172,"parent_record_offset":19964,"ancestor_record_offset":19864,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":20424,"parent_record_offset":20400,"ancestor_record_offset":19864,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":19864,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":20736,"parent_record_offset":20700,"ancestor_record_offset":19864,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":20600,"parent_record_offset":20564,"ancestor_record_offset":19864,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":20564,"parent_record_offset":20532,"ancestor_record_offset":19864,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":20464,"parent_record_offset":20108,"ancestor_record_offset":19864,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":20284,"parent_record_offset":20260,"ancestor_record_offset":19864,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":20260,"parent_record_offset":20236,"ancestor_record_offset":19864,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":21092,"parent_record_offset":21028,"ancestor_record_offset":19864,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":21064,"parent_record_offset":21028,"ancestor_record_offset":19864,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":21136,"parent_record_offset":19964,"ancestor_record_offset":19864,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":757,"record_offset":21028,"parent_record_offset":20996,"ancestor_record_offset":19864,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20840,"parent_record_offset":20808,"ancestor_record_offset":19864,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20808,"parent_record_offset":20636,"ancestor_record_offset":19864,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20700,"parent_record_offset":20668,"ancestor_record_offset":19864,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20668,"parent_record_offset":20636,"ancestor_record_offset":19864,"inlinee":27235,"source_line":1548,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":757,"record_offset":20996,"parent_record_offset":20532,"ancestor_record_offset":19864,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":20904,"parent_record_offset":20840,"ancestor_record_offset":19864,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_open::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":20636,"parent_record_offset":20600,"ancestor_record_offset":19864,"inlinee":49014,"source_line":184,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_open::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF76
// 首条位置: {"module":757,"inlinee":49014}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20808,"parent_record_offset":20636,"ancestor_record_offset":20636,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20668,"parent_record_offset":20636,"ancestor_record_offset":20636,"inlinee":27235,"source_line":1548,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":20936,"parent_record_offset":20904,"ancestor_record_offset":20636,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":20736,"parent_record_offset":20700,"ancestor_record_offset":20636,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20840,"parent_record_offset":20808,"ancestor_record_offset":20636,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":20700,"parent_record_offset":20668,"ancestor_record_offset":20636,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":20904,"parent_record_offset":20840,"ancestor_record_offset":20636,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
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
// 首条位置: {"module":757,"record_offset":27540,"rva":14584736,"va":5383293856}
// 编译实例 RVAs: 0xde8ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33762492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293856,"raw_hex":"e8bc2c0302","size":5,"soft_fail":false,"target_va":5417057088,"va":5383294591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33317160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293856,"raw_hex":"e82861fc01","size":5,"soft_fail":false,"target_va":5416613232,"va":5383296067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17078878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293856,"raw_hex":"e85e9a0401","size":5,"soft_fail":false,"target_va":5400373936,"va":5383295053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33317416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293856,"raw_hex":"e82862fc01","size":5,"soft_fail":false,"target_va":5416613920,"va":5383296499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33324967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293856,"raw_hex":"e8a77ffc01","size":5,"soft_fail":false,"target_va":5416621536,"va":5383296564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33325680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293856,"raw_hex":"e87082fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383296523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::natural_cubic::solve_clamped_spline
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293856,"raw_hex":"e8ddd5ffff","size":5,"soft_fail":false,"target_va":5383284512,"va":5383295294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":30080,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::from_iter_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":31040,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":49002,"source_line":604,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":32068,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":31928,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":32768,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":34024,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::build_knots_open3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":27716,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":49023,"source_line":307,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27932,"parent_record_offset":27816,"ancestor_record_offset":27540,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":29220,"parent_record_offset":27716,"ancestor_record_offset":27540,"inlinee":48976,"source_line":1681,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27788,"parent_record_offset":27716,"ancestor_record_offset":27540,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27816,"parent_record_offset":27788,"ancestor_record_offset":27540,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":31696,"parent_record_offset":31640,"ancestor_record_offset":27540,"inlinee":21951,"source_line":2228,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":31728,"parent_record_offset":31696,"ancestor_record_offset":27540,"inlinee":21952,"source_line":2221,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27960,"parent_record_offset":27932,"ancestor_record_offset":27540,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27992,"parent_record_offset":27960,"ancestor_record_offset":27540,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":31668,"parent_record_offset":31640,"ancestor_record_offset":27540,"inlinee":21950,"source_line":158,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":31640,"parent_record_offset":31616,"ancestor_record_offset":27540,"inlinee":21949,"source_line":701,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":27540,"inlinee":21942,"source_line":607,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":31296,"parent_record_offset":31268,"ancestor_record_offset":27540,"inlinee":21939,"source_line":137,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":31172,"parent_record_offset":31148,"ancestor_record_offset":27540,"inlinee":21941,"source_line":429,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$6::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":757,"record_offset":31616,"parent_record_offset":31592,"ancestor_record_offset":27540,"inlinee":21948,"source_line":713,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":31084,"parent_record_offset":31040,"ancestor_record_offset":27540,"inlinee":21935,"source_line":545,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":31232,"parent_record_offset":31112,"ancestor_record_offset":27540,"inlinee":21936,"source_line":1451,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":31268,"parent_record_offset":31232,"ancestor_record_offset":27540,"inlinee":21937,"source_line":1422,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":31148,"parent_record_offset":31112,"ancestor_record_offset":27540,"inlinee":21940,"source_line":736,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$18::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":757,"record_offset":31112,"parent_record_offset":31040,"ancestor_record_offset":27540,"inlinee":49004,"source_line":2311,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":33704,"parent_record_offset":33312,"ancestor_record_offset":27540,"inlinee":5932,"source_line":1865,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":28352,"parent_record_offset":27816,"ancestor_record_offset":27540,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":33252,"parent_record_offset":33184,"ancestor_record_offset":27540,"inlinee":49009,"source_line":1556,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":28088,"parent_record_offset":27992,"ancestor_record_offset":27540,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":27876,"parent_record_offset":27816,"ancestor_record_offset":27540,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":27900,"parent_record_offset":27876,"ancestor_record_offset":27540,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":28032,"parent_record_offset":27992,"ancestor_record_offset":27540,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":28252,"parent_record_offset":28228,"ancestor_record_offset":27540,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":28228,"parent_record_offset":27992,"ancestor_record_offset":27540,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":29952,"parent_record_offset":29928,"ancestor_record_offset":27540,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":29116,"parent_record_offset":29080,"ancestor_record_offset":27540,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":29928,"parent_record_offset":29848,"ancestor_record_offset":27540,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":28852,"parent_record_offset":28820,"ancestor_record_offset":27540,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":757,"record_offset":30636,"parent_record_offset":30608,"ancestor_record_offset":27540,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":28384,"parent_record_offset":28352,"ancestor_record_offset":27540,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":757,"record_offset":29896,"parent_record_offset":29872,"ancestor_record_offset":27540,"inlinee":10930,"source_line":91,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":29872,"parent_record_offset":29848,"ancestor_record_offset":27540,"inlinee":5521,"source_line":587,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":29824,"parent_record_offset":29800,"ancestor_record_offset":27540,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":32096,"parent_record_offset":32068,"ancestor_record_offset":27540,"inlinee":9710,"source_line":99,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":29996,"parent_record_offset":27716,"ancestor_record_offset":27540,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":32124,"parent_record_offset":32096,"ancestor_record_offset":27540,"inlinee":9711,"source_line":275,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":29848,"parent_record_offset":29824,"ancestor_record_offset":27540,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":30028,"parent_record_offset":29996,"ancestor_record_offset":27540,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":29148,"parent_record_offset":29080,"ancestor_record_offset":27540,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":29080,"parent_record_offset":29048,"ancestor_record_offset":27540,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":29048,"parent_record_offset":27816,"ancestor_record_offset":27540,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":32356,"parent_record_offset":32124,"ancestor_record_offset":27540,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":29800,"parent_record_offset":27716,"ancestor_record_offset":27540,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":28276,"parent_record_offset":28252,"ancestor_record_offset":27540,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":29172,"parent_record_offset":29148,"ancestor_record_offset":27540,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":28600,"parent_record_offset":28552,"ancestor_record_offset":27540,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":28448,"parent_record_offset":28416,"ancestor_record_offset":27540,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":30764,"parent_record_offset":30636,"ancestor_record_offset":27540,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":28416,"parent_record_offset":28384,"ancestor_record_offset":27540,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":28316,"parent_record_offset":27960,"ancestor_record_offset":27540,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":31568,"parent_record_offset":31040,"ancestor_record_offset":27540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":31792,"parent_record_offset":31696,"ancestor_record_offset":27540,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":28136,"parent_record_offset":28112,"ancestor_record_offset":27540,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":28112,"parent_record_offset":28088,"ancestor_record_offset":27540,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":30732,"parent_record_offset":30668,"ancestor_record_offset":27540,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":28968,"parent_record_offset":28940,"ancestor_record_offset":27540,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":29012,"parent_record_offset":27816,"ancestor_record_offset":27540,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":757,"record_offset":28940,"parent_record_offset":28912,"ancestor_record_offset":27540,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28748,"parent_record_offset":28708,"ancestor_record_offset":27540,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28708,"parent_record_offset":28480,"ancestor_record_offset":27540,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28552,"parent_record_offset":28512,"ancestor_record_offset":27540,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28512,"parent_record_offset":28480,"ancestor_record_offset":27540,"inlinee":27392,"source_line":1749,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":757,"record_offset":28912,"parent_record_offset":28384,"ancestor_record_offset":27540,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":28820,"parent_record_offset":28748,"ancestor_record_offset":27540,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_open3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":28480,"parent_record_offset":28448,"ancestor_record_offset":27540,"inlinee":49025,"source_line":311,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$10
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":33932,"parent_record_offset":33908,"ancestor_record_offset":27540,"inlinee":49029,"source_line":254,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":32384,"parent_record_offset":32356,"ancestor_record_offset":27540,"inlinee":49027,"source_line":231,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":32720,"parent_record_offset":32692,"ancestor_record_offset":27540,"inlinee":49028,"source_line":242,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF83
// 首条位置: {"module":757,"inlinee":49027}
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
// IPI IDs: 0xBF84
// 首条位置: {"module":757,"inlinee":49028}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF85
// 首条位置: {"module":757,"inlinee":49029}
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
// 首条位置: {"module":757,"record_offset":26644,"rva":14584336,"va":5383293456}
// 编译实例 RVAs: 0xde8a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::natural_cubic::Spline3D::find_segment_and_dt
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293456,"raw_hex":"e888000000","size":5,"soft_fail":false,"target_va":5383293616,"va":5383293475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::natural_cubic::SplineSegment3D::position_at_local_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":26776,"parent_record_offset":26644,"ancestor_record_offset":26644,"inlinee":49020,"source_line":43,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":26900,"parent_record_offset":26776,"ancestor_record_offset":26644,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::eval_cubic
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":26812,"parent_record_offset":26776,"ancestor_record_offset":26644,"inlinee":48975,"source_line":351,"archive":"_pdb/C/modules/00757.jsonl.gz"}
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
// 首条位置: {"module":757,"record_offset":26356,"rva":14584192,"va":5383293312}
// 编译实例 RVAs: 0xde8980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::natural_cubic::Spline3D::find_segment_and_dt
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293312,"raw_hex":"e81d010000","size":5,"soft_fail":false,"target_va":5383293616,"va":5383293326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::natural_cubic::SplineSegment3D::tangent_at_local_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":26484,"parent_record_offset":26356,"ancestor_record_offset":26356,"inlinee":49019,"source_line":51,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":26608,"parent_record_offset":26484,"ancestor_record_offset":26356,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::eval_cubic_deriv
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":26520,"parent_record_offset":26484,"ancestor_record_offset":26356,"inlinee":48974,"source_line":356,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::Spline3D::find_segment_and_dt
// 记录形式: compiled_procedure; instances=1
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":757,"record_offset":26936,"rva":14584496,"va":5383293616}
// 编译实例 RVAs: 0xde8ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33327908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293616,"raw_hex":"e8248bfc01","size":5,"soft_fail":false,"target_va":5416621762,"va":5383293849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33328398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383293616,"raw_hex":"e80e8dfc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383293837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":27080,"parent_record_offset":26936,"ancestor_record_offset":26936,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":27508,"parent_record_offset":26936,"ancestor_record_offset":26936,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":757,"record_offset":27188,"parent_record_offset":26936,"ancestor_record_offset":26936,"inlinee":13180,"source_line":4854,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":757,"record_offset":27120,"parent_record_offset":27080,"ancestor_record_offset":26936,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":757,"record_offset":27148,"parent_record_offset":27120,"ancestor_record_offset":26936,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::select_unpredictable
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":757,"record_offset":27368,"parent_record_offset":27212,"ancestor_record_offset":26936,"inlinee":9469,"source_line":829,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":757,"record_offset":27212,"parent_record_offset":27188,"ancestor_record_offset":26936,"inlinee":13154,"source_line":2970,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":757,"record_offset":27264,"parent_record_offset":27212,"ancestor_record_offset":26936,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":757,"record_offset":27320,"parent_record_offset":27212,"ancestor_record_offset":26936,"inlinee":13156,"source_line":4858,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":757,"record_offset":27288,"parent_record_offset":27264,"ancestor_record_offset":26936,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::impl$3::find_segment_and_dt::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":27340,"parent_record_offset":27320,"ancestor_record_offset":26936,"inlinee":49022,"source_line":291,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::impl$3::find_segment_and_dt::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF7E
// 首条位置: {"module":757,"inlinee":49022}
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
// 首条位置: {"module":757,"record_offset":34740,"rva":14587744,"va":5383296864}
// 编译实例 RVAs: 0xde9760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7618302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383296864,"raw_hex":"e802c18bff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383296985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14574779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383296864,"raw_hex":"e8459b21ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383296966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33760024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383296864,"raw_hex":"e818230302","size":5,"soft_fail":false,"target_va":5417057056,"va":5383297027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33323839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383296864,"raw_hex":"e83f7bfc01","size":5,"soft_fail":false,"target_va":5416620794,"va":5383296950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":757,"record_offset":34864,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":757,"record_offset":35232,"parent_record_offset":35208,"ancestor_record_offset":34740,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":757,"record_offset":35256,"parent_record_offset":35232,"ancestor_record_offset":34740,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":757,"record_offset":35280,"parent_record_offset":35256,"ancestor_record_offset":34740,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":757,"record_offset":35208,"parent_record_offset":35028,"ancestor_record_offset":34740,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":757,"record_offset":34972,"parent_record_offset":34948,"ancestor_record_offset":34740,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":757,"record_offset":35028,"parent_record_offset":34996,"ancestor_record_offset":34740,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":757,"record_offset":34996,"parent_record_offset":34972,"ancestor_record_offset":34740,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":757,"record_offset":35064,"parent_record_offset":35028,"ancestor_record_offset":34740,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":757,"record_offset":34888,"parent_record_offset":34864,"ancestor_record_offset":34740,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":757,"record_offset":34912,"parent_record_offset":34888,"ancestor_record_offset":34740,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":757,"record_offset":34948,"parent_record_offset":34912,"ancestor_record_offset":34740,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":35088,"parent_record_offset":35064,"ancestor_record_offset":34740,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":757,"record_offset":35168,"parent_record_offset":35112,"ancestor_record_offset":34740,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":35112,"parent_record_offset":35088,"ancestor_record_offset":34740,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":35140,"parent_record_offset":35112,"ancestor_record_offset":34740,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":757,"record_offset":35336,"parent_record_offset":34912,"ancestor_record_offset":34740,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":35360,"parent_record_offset":35336,"ancestor_record_offset":34740,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_open3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF7F
// 首条位置: {"module":757,"inlinee":49023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":29220,"parent_record_offset":27716,"ancestor_record_offset":27716,"inlinee":48976,"source_line":1681,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27788,"parent_record_offset":27716,"ancestor_record_offset":27716,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":29996,"parent_record_offset":27716,"ancestor_record_offset":27716,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":757,"record_offset":29800,"parent_record_offset":27716,"ancestor_record_offset":27716,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27932,"parent_record_offset":27816,"ancestor_record_offset":27716,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27816,"parent_record_offset":27788,"ancestor_record_offset":27716,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27960,"parent_record_offset":27932,"ancestor_record_offset":27716,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":27992,"parent_record_offset":27960,"ancestor_record_offset":27716,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":28352,"parent_record_offset":27816,"ancestor_record_offset":27716,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":28088,"parent_record_offset":27992,"ancestor_record_offset":27716,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":27876,"parent_record_offset":27816,"ancestor_record_offset":27716,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":27900,"parent_record_offset":27876,"ancestor_record_offset":27716,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":28032,"parent_record_offset":27992,"ancestor_record_offset":27716,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":28252,"parent_record_offset":28228,"ancestor_record_offset":27716,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":28228,"parent_record_offset":27992,"ancestor_record_offset":27716,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":29952,"parent_record_offset":29928,"ancestor_record_offset":27716,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":29116,"parent_record_offset":29080,"ancestor_record_offset":27716,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":29928,"parent_record_offset":29848,"ancestor_record_offset":27716,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":28852,"parent_record_offset":28820,"ancestor_record_offset":27716,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":28384,"parent_record_offset":28352,"ancestor_record_offset":27716,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":757,"record_offset":29896,"parent_record_offset":29872,"ancestor_record_offset":27716,"inlinee":10930,"source_line":91,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":29872,"parent_record_offset":29848,"ancestor_record_offset":27716,"inlinee":5521,"source_line":587,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":29824,"parent_record_offset":29800,"ancestor_record_offset":27716,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":29848,"parent_record_offset":29824,"ancestor_record_offset":27716,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":30028,"parent_record_offset":29996,"ancestor_record_offset":27716,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":29148,"parent_record_offset":29080,"ancestor_record_offset":27716,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":29080,"parent_record_offset":29048,"ancestor_record_offset":27716,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":29048,"parent_record_offset":27816,"ancestor_record_offset":27716,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":28276,"parent_record_offset":28252,"ancestor_record_offset":27716,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":29172,"parent_record_offset":29148,"ancestor_record_offset":27716,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":28600,"parent_record_offset":28552,"ancestor_record_offset":27716,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":28448,"parent_record_offset":28416,"ancestor_record_offset":27716,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":28416,"parent_record_offset":28384,"ancestor_record_offset":27716,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":28316,"parent_record_offset":27960,"ancestor_record_offset":27716,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":28136,"parent_record_offset":28112,"ancestor_record_offset":27716,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":28112,"parent_record_offset":28088,"ancestor_record_offset":27716,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":28968,"parent_record_offset":28940,"ancestor_record_offset":27716,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":29012,"parent_record_offset":27816,"ancestor_record_offset":27716,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":757,"record_offset":28940,"parent_record_offset":28912,"ancestor_record_offset":27716,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28748,"parent_record_offset":28708,"ancestor_record_offset":27716,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28708,"parent_record_offset":28480,"ancestor_record_offset":27716,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28552,"parent_record_offset":28512,"ancestor_record_offset":27716,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28512,"parent_record_offset":28480,"ancestor_record_offset":27716,"inlinee":27392,"source_line":1749,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":757,"record_offset":28912,"parent_record_offset":28384,"ancestor_record_offset":27716,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":28820,"parent_record_offset":28748,"ancestor_record_offset":27716,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_open3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":28480,"parent_record_offset":28448,"ancestor_record_offset":27716,"inlinee":49025,"source_line":311,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_open3::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF81
// 首条位置: {"module":757,"inlinee":49025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28708,"parent_record_offset":28480,"ancestor_record_offset":28480,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$52::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28512,"parent_record_offset":28480,"ancestor_record_offset":28480,"inlinee":27392,"source_line":1749,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":28852,"parent_record_offset":28820,"ancestor_record_offset":28480,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":28600,"parent_record_offset":28552,"ancestor_record_offset":28480,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28748,"parent_record_offset":28708,"ancestor_record_offset":28480,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":28552,"parent_record_offset":28512,"ancestor_record_offset":28480,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":28820,"parent_record_offset":28748,"ancestor_record_offset":28480,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_closed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF67
// 首条位置: {"module":757,"inlinee":48999}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":14912,"parent_record_offset":13504,"ancestor_record_offset":13504,"inlinee":48976,"source_line":1681,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13576,"parent_record_offset":13504,"ancestor_record_offset":13504,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":15492,"parent_record_offset":13504,"ancestor_record_offset":13504,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13704,"parent_record_offset":13604,"ancestor_record_offset":13504,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13604,"parent_record_offset":13576,"ancestor_record_offset":13504,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13732,"parent_record_offset":13704,"ancestor_record_offset":13504,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13764,"parent_record_offset":13732,"ancestor_record_offset":13504,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":14124,"parent_record_offset":13604,"ancestor_record_offset":13504,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":13860,"parent_record_offset":13764,"ancestor_record_offset":13504,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":13648,"parent_record_offset":13604,"ancestor_record_offset":13504,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":13504,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":13804,"parent_record_offset":13764,"ancestor_record_offset":13504,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":14024,"parent_record_offset":14000,"ancestor_record_offset":13504,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":14000,"parent_record_offset":13764,"ancestor_record_offset":13504,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":14868,"parent_record_offset":14784,"ancestor_record_offset":13504,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":14440,"parent_record_offset":14412,"ancestor_record_offset":13504,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cycle.rs
//     evidence={"module":757,"record_offset":14564,"parent_record_offset":14532,"ancestor_record_offset":13504,"inlinee":9870,"source_line":34,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":14152,"parent_record_offset":14124,"ancestor_record_offset":13504,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":757,"record_offset":15524,"parent_record_offset":15492,"ancestor_record_offset":13504,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":14812,"parent_record_offset":14784,"ancestor_record_offset":13504,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":13504,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":14760,"parent_record_offset":13604,"ancestor_record_offset":13504,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":13504,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":14836,"parent_record_offset":14812,"ancestor_record_offset":13504,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":14336,"parent_record_offset":14296,"ancestor_record_offset":13504,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":14212,"parent_record_offset":14180,"ancestor_record_offset":13504,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":14180,"parent_record_offset":14152,"ancestor_record_offset":13504,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":14088,"parent_record_offset":13732,"ancestor_record_offset":13504,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":13504,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":13504,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":13504,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":757,"record_offset":14620,"parent_record_offset":14596,"ancestor_record_offset":13504,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":14728,"parent_record_offset":13604,"ancestor_record_offset":13504,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":757,"record_offset":14596,"parent_record_offset":14564,"ancestor_record_offset":13504,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":13504,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14364,"parent_record_offset":14296,"ancestor_record_offset":13504,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14296,"parent_record_offset":14268,"ancestor_record_offset":13504,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$10::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":757,"record_offset":14500,"parent_record_offset":14152,"ancestor_record_offset":13504,"inlinee":9887,"source_line":276,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":757,"record_offset":14532,"parent_record_offset":14500,"ancestor_record_offset":13504,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":14412,"parent_record_offset":14364,"ancestor_record_offset":13504,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::build_knots_closed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":14244,"parent_record_offset":14212,"ancestor_record_offset":13504,"inlinee":49001,"source_line":333,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::build_knots_closed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF69
// 首条位置: {"module":757,"inlinee":49001}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":14244,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":757,"record_offset":14440,"parent_record_offset":14412,"ancestor_record_offset":14244,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":757,"record_offset":14336,"parent_record_offset":14296,"ancestor_record_offset":14244,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14364,"parent_record_offset":14296,"ancestor_record_offset":14244,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":757,"record_offset":14296,"parent_record_offset":14268,"ancestor_record_offset":14244,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":757,"record_offset":14412,"parent_record_offset":14364,"ancestor_record_offset":14244,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::eval_cubic
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 351
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF4F
// 首条位置: {"module":757,"inlinee":48975}
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
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 356
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF4E
// 首条位置: {"module":757,"inlinee":48974}
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
// 首条位置: {"module":757,"record_offset":2496,"rva":14575392,"va":5383284512}
// 编译实例 RVAs: 0xde6720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: natural_cubic.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33772273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383284512,"raw_hex":"e8f1520302","size":5,"soft_fail":false,"target_va":5417057088,"va":5383284810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33326917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383284512,"raw_hex":"e84587fc01","size":5,"soft_fail":false,"target_va":5416613232,"va":5383286310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33327494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383284512,"raw_hex":"e88689fc01","size":5,"soft_fail":false,"target_va":5416613920,"va":5383286421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t33336577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383284512,"raw_hex":"e801adfc01","size":5,"soft_fail":false,"target_va":5416621762,"va":5383285180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":3948,"parent_record_offset":2496,"ancestor_record_offset":2496,"inlinee":48976,"source_line":1681,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":2640,"parent_record_offset":2496,"ancestor_record_offset":2496,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":757,"record_offset":5268,"parent_record_offset":2496,"ancestor_record_offset":2496,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":5140,"parent_record_offset":2496,"ancestor_record_offset":2496,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":5108,"parent_record_offset":2496,"ancestor_record_offset":2496,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":2824,"parent_record_offset":2692,"ancestor_record_offset":2496,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":2692,"parent_record_offset":2640,"ancestor_record_offset":2496,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":2852,"parent_record_offset":2824,"ancestor_record_offset":2496,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":2884,"parent_record_offset":2852,"ancestor_record_offset":2496,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":3436,"parent_record_offset":2692,"ancestor_record_offset":2496,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":2980,"parent_record_offset":2884,"ancestor_record_offset":2496,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":2768,"parent_record_offset":2692,"ancestor_record_offset":2496,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":2496,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":2924,"parent_record_offset":2884,"ancestor_record_offset":2496,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":2496,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":3120,"parent_record_offset":2884,"ancestor_record_offset":2496,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":3328,"parent_record_offset":3284,"ancestor_record_offset":2496,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":3480,"parent_record_offset":3436,"ancestor_record_offset":2496,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":3372,"parent_record_offset":3284,"ancestor_record_offset":2496,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":3284,"parent_record_offset":3244,"ancestor_record_offset":2496,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":5340,"parent_record_offset":5304,"ancestor_record_offset":2496,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":3244,"parent_record_offset":2692,"ancestor_record_offset":2496,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":5304,"parent_record_offset":5268,"ancestor_record_offset":2496,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":2496,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":2496,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":3568,"parent_record_offset":3524,"ancestor_record_offset":2496,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":3524,"parent_record_offset":3480,"ancestor_record_offset":2496,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":3676,"parent_record_offset":3612,"ancestor_record_offset":2496,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":3208,"parent_record_offset":2852,"ancestor_record_offset":2496,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2496,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2496,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":3896,"parent_record_offset":2692,"ancestor_record_offset":2496,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_clamped_spline::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":3612,"parent_record_offset":3568,"ancestor_record_offset":2496,"inlinee":48981,"source_line":372,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_clamped_spline::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":3708,"parent_record_offset":3676,"ancestor_record_offset":2496,"inlinee":48983,"source_line":374,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_clamped_spline::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":3784,"parent_record_offset":3744,"ancestor_record_offset":2496,"inlinee":48984,"source_line":376,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_clamped_spline::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":6360,"parent_record_offset":6320,"ancestor_record_offset":2496,"inlinee":48985,"source_line":408,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_clamped_spline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF55
// 首条位置: {"module":757,"inlinee":48981}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":3676,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_clamped_spline::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":3708,"parent_record_offset":3676,"ancestor_record_offset":3612,"inlinee":48983,"source_line":374,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_clamped_spline::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":3784,"parent_record_offset":3744,"ancestor_record_offset":3612,"inlinee":48984,"source_line":376,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_clamped_spline::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 374
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF57
// 首条位置: {"module":757,"inlinee":48983}
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
// IPI IDs: 0xBF58
// 首条位置: {"module":757,"inlinee":48984}
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
// IPI IDs: 0xBF59
// 首条位置: {"module":757,"inlinee":48985}
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
// 首条位置: {"module":757,"record_offset":6508,"rva":14577584,"va":5383286704}
// 编译实例 RVAs: 0xde6fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33324300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383286704,"raw_hex":"e80c7dfc01","size":5,"soft_fail":false,"target_va":5416613232,"va":5383288927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33324717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383286704,"raw_hex":"e8ad7efc01","size":5,"soft_fail":false,"target_va":5416613920,"va":5383289198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33332309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383286704,"raw_hex":"e8559cfc01","size":5,"soft_fail":false,"target_va":5416621536,"va":5383289222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t33332517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383286704,"raw_hex":"e8259dfc01","size":5,"soft_fail":false,"target_va":5416621762,"va":5383289240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::natural_cubic::solve_tridiagonal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383286704,"raw_hex":"e8c7efffff","size":5,"soft_fail":false,"target_va":5383283856,"va":5383288004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":6644,"parent_record_offset":6508,"ancestor_record_offset":6508,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] utils::natural_cubic::solve_cyclic
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":10348,"parent_record_offset":6508,"ancestor_record_offset":6508,"inlinee":48991,"source_line":484,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":6812,"parent_record_offset":6692,"ancestor_record_offset":6508,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":10436,"parent_record_offset":10348,"ancestor_record_offset":6508,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":6692,"parent_record_offset":6644,"ancestor_record_offset":6508,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":6840,"parent_record_offset":6812,"ancestor_record_offset":6508,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":6872,"parent_record_offset":6840,"ancestor_record_offset":6508,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":7368,"parent_record_offset":6692,"ancestor_record_offset":6508,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":6968,"parent_record_offset":6872,"ancestor_record_offset":6508,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":6756,"parent_record_offset":6692,"ancestor_record_offset":6508,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":757,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":6508,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":6912,"parent_record_offset":6872,"ancestor_record_offset":6508,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":6508,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":7108,"parent_record_offset":6872,"ancestor_record_offset":6508,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":7320,"parent_record_offset":7276,"ancestor_record_offset":6508,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":7404,"parent_record_offset":7368,"ancestor_record_offset":6508,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":8244,"parent_record_offset":8212,"ancestor_record_offset":6508,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":7276,"parent_record_offset":7232,"ancestor_record_offset":6508,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":7232,"parent_record_offset":6692,"ancestor_record_offset":6508,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":7156,"parent_record_offset":7132,"ancestor_record_offset":6508,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":6508,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":7532,"parent_record_offset":7496,"ancestor_record_offset":6508,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":7496,"parent_record_offset":7404,"ancestor_record_offset":6508,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":7196,"parent_record_offset":6840,"ancestor_record_offset":6508,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6508,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":6508,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":7640,"parent_record_offset":6692,"ancestor_record_offset":6508,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::compute_periodic_spline::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":7568,"parent_record_offset":7532,"ancestor_record_offset":6508,"inlinee":48987,"source_line":431,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::compute_periodic_spline::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":8448,"parent_record_offset":8420,"ancestor_record_offset":6508,"inlinee":48988,"source_line":437,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::compute_periodic_spline::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":9348,"parent_record_offset":9316,"ancestor_record_offset":6508,"inlinee":48989,"source_line":438,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::compute_periodic_spline::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":10256,"parent_record_offset":10224,"ancestor_record_offset":6508,"inlinee":48990,"source_line":439,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::compute_periodic_spline::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":13216,"parent_record_offset":13184,"ancestor_record_offset":6508,"inlinee":48995,"source_line":443,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_cyclic::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":11228,"parent_record_offset":11196,"ancestor_record_offset":6508,"inlinee":48993,"source_line":500,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_cyclic::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":12100,"parent_record_offset":12072,"ancestor_record_offset":6508,"inlinee":48994,"source_line":514,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::compute_periodic_spline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 431
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF5B
// 首条位置: {"module":757,"inlinee":48987}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 437
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF5C
// 首条位置: {"module":757,"inlinee":48988}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 438
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF5D
// 首条位置: {"module":757,"inlinee":48989}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 439
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF5E
// 首条位置: {"module":757,"inlinee":48990}
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
// IPI IDs: 0xBF63
// 首条位置: {"module":757,"inlinee":48995}
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
// IPI IDs: 0xBF5F
// 首条位置: {"module":757,"inlinee":48991}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":10436,"parent_record_offset":10348,"ancestor_record_offset":10348,"inlinee":48977,"source_line":1747,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":10532,"parent_record_offset":10472,"ancestor_record_offset":10348,"inlinee":5927,"source_line":1864,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":10472,"parent_record_offset":10436,"ancestor_record_offset":10348,"inlinee":10357,"source_line":1552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":10560,"parent_record_offset":10532,"ancestor_record_offset":10348,"inlinee":5928,"source_line":1879,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":10592,"parent_record_offset":10560,"ancestor_record_offset":10348,"inlinee":5929,"source_line":1888,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":11100,"parent_record_offset":10472,"ancestor_record_offset":10348,"inlinee":48979,"source_line":1753,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":757,"record_offset":10688,"parent_record_offset":10592,"ancestor_record_offset":10348,"inlinee":5930,"source_line":450,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":10632,"parent_record_offset":10592,"ancestor_record_offset":10348,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":10852,"parent_record_offset":10828,"ancestor_record_offset":10348,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":757,"record_offset":10828,"parent_record_offset":10592,"ancestor_record_offset":10348,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":757,"record_offset":11060,"parent_record_offset":11028,"ancestor_record_offset":10348,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":757,"record_offset":11132,"parent_record_offset":11100,"ancestor_record_offset":10348,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":11880,"parent_record_offset":11836,"ancestor_record_offset":10348,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":11028,"parent_record_offset":10996,"ancestor_record_offset":10348,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":757,"record_offset":10996,"parent_record_offset":10472,"ancestor_record_offset":10348,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":757,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":10348,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":757,"record_offset":11912,"parent_record_offset":11880,"ancestor_record_offset":10348,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":757,"record_offset":11196,"parent_record_offset":11164,"ancestor_record_offset":10348,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":11164,"parent_record_offset":11132,"ancestor_record_offset":10348,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":757,"record_offset":10916,"parent_record_offset":10560,"ancestor_record_offset":10348,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":10736,"parent_record_offset":10712,"ancestor_record_offset":10348,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":757,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":10348,"inlinee":5227,"source_line":1224,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":757,"record_offset":10952,"parent_record_offset":10472,"ancestor_record_offset":10348,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_cyclic::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":11228,"parent_record_offset":11196,"ancestor_record_offset":10348,"inlinee":48993,"source_line":500,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   [pdb_inline_transitive] utils::natural_cubic::solve_cyclic::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/natural_cubic.rs
//     evidence={"module":757,"record_offset":12100,"parent_record_offset":12072,"ancestor_record_offset":10348,"inlinee":48994,"source_line":514,"archive":"_pdb/C/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::natural_cubic::solve_cyclic::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 500
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF61
// 首条位置: {"module":757,"inlinee":48993}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 514
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF62
// 首条位置: {"module":757,"inlinee":48994}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 6
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline2D::from_control_points_with_tangents
// 信息 ID: c8e863736c3fad1f124d41b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::generate_playermade_stair_requests<true> @ 0x442a80 -> 0xde8130 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::generate_playermade_stair_requests<false> @ 0x449bde -> 0xde8130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline2D::from_periodic_loop
// 信息 ID: 68234bccc040d005bf0126b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::generate_playermade_stair_requests<true> @ 0x4426be -> 0xde7a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::generate_playermade_stair_requests<false> @ 0x449814 -> 0xde7a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::find_segment_and_dt
// 信息 ID: af2f59e4cf77f9c00b2fad29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::natural_cubic::Spline3D::tangent_at_t @ 0xde898e -> 0xde8ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline3D::position_at_t @ 0xde8a23 -> 0xde8ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::from_control_points_with_tangents
// 信息 ID: 8c14132e9f446eb722ba1bc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x35608e -> 0xde8ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::position_at_t
// 信息 ID: 6a47818c73adbc35494c590d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::RollercoasterState::new @ 0x35090b -> 0xde8a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x352bde -> 0xde8a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x353a86 -> 0xde8a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x354084 -> 0xde8a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::tangent_at_t
// 信息 ID: 5b61113834e774a6e84dd249
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x35258b -> 0xde8980 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x352bf2 -> 0xde8980 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x352c0f -> 0xde8980 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x353a9d -> 0xde8980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::Spline3D::to_owned
// 信息 ID: 8e5838e719c64ed2c208c31c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3560a2 -> 0xde9760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::SplineSegment2D::position_at_local_t
// 信息 ID: ca81d87ec9305b09eed4d767
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::densely_sample_spline_segment::closure_env$1> > @ 0x14a3ee1 -> 0xde6440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::SplineSegment2D::tangent_at_local_t
// 信息 ID: c7ea2f36ff3eb49b28359bf2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::densely_sample_spline_segment @ 0x898d2a -> 0xde63f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::densely_sample_spline_segment @ 0x898d3d -> 0xde63f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::SplineSegment3D::position_at_local_t
// 信息 ID: 93b14bdfc41335da4e5971bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=757, inline_record=26776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::SplineSegment3D::tangent_at_local_t
// 信息 ID: d106d7450e5b70d72bcf85b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=757, inline_record=26484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_closed
// 信息 ID: d3115e81f6f71a9cf2a9c239
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=757, inline_record=13504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_closed::closure$0
// 信息 ID: f0b177a6ef10e04341fd8220
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::build_knots_closed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=757, inline_record=14244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_open
// 信息 ID: b3c2d3bab268f9f54583500d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=757, inline_record=19864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_open3
// 信息 ID: 7c65f9065541636d8a1ea899
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=757, inline_record=27716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_open3::closure$0
// 信息 ID: 3b02c041972ff54a4bafb6f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::build_knots_open3", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=757, inline_record=28480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::build_knots_open::closure$0
// 信息 ID: 282ca872029d239884bb7ec5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::build_knots_open", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=757, inline_record=20636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline
// 信息 ID: f54edd29edb59fbfd7497db2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::natural_cubic::Spline2D::from_periodic_loop @ 0xde7db7 -> 0xde6fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline2D::from_periodic_loop @ 0xde7e7c -> 0xde6fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$0
// 信息 ID: 8c83cb5f69907b9166485fb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=757, inline_record=7568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$1
// 信息 ID: 9ba3ad3aefe7fe4dd11d2919
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=757, inline_record=8448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$2
// 信息 ID: c8809614f8b57ae20096ad31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=757, inline_record=9348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$3
// 信息 ID: 91b3ec460479a778008cbffe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=757, inline_record=10256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::compute_periodic_spline::closure$4
// 信息 ID: 31b13f20945cb7abd45b9ce6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "utils::natural_cubic::compute_periodic_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=757, inline_record=13216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::eval_cubic
// 信息 ID: ca04c7816973cbe05e23d4dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::SplineSegment2D::position_at_local_t; module=757, inline_record=608; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::SplineSegment2D::position_at_local_t; module=757, inline_record=632; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=757, inline_record=26812; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=757, inline_record=26840; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::position_at_t; module=757, inline_record=26872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::eval_cubic_deriv
// 信息 ID: 714eacc166c93cd94eaa01b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::SplineSegment2D::tangent_at_local_t; module=757, inline_record=404; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::SplineSegment2D::tangent_at_local_t; module=757, inline_record=432; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=757, inline_record=26520; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=757, inline_record=26548; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::tangent_at_t; module=757, inline_record=26580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::find_segment_and_dt::closure$0
// 信息 ID: 8eca696f2c0040565ba95534
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::impl$2::find_segment_and_dt", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$4
// 信息 ID: a4b77a8cd70bbb01fecf7155
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "utils::natural_cubic::impl$2::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=757, inline_record=24484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_control_points_with_tangents::closure$7
// 信息 ID: a2f5f6814a1be54ed62fcfd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "utils::natural_cubic::impl$2::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_control_points_with_tangents; module=757, inline_record=25556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_periodic_loop::closure$2
// 信息 ID: 137df025f2afef68371fb929
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "utils::natural_cubic::impl$2::from_periodic_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=757, inline_record=17872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$2::from_periodic_loop::closure$3
// 信息 ID: 4c56ba918afae46e6b6f5ad1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "utils::natural_cubic::impl$2::from_periodic_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline2D::from_periodic_loop; module=757, inline_record=18888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::find_segment_and_dt::closure$0
// 信息 ID: 1ad79d845339cda3bd8ea8aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::impl$3::find_segment_and_dt", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::find_segment_and_dt; module=757, inline_record=27340; 不是独立调用边。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::find_segment_and_dt; module=757, inline_record=27472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$10
// 信息 ID: e0d7f4861638dcde6bb299b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "utils::natural_cubic::impl$3::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=757, inline_record=33932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$4
// 信息 ID: 59d1d7bfa3ee4784969e3b41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "utils::natural_cubic::impl$3::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=757, inline_record=32384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::impl$3::from_control_points_with_tangents::closure$7
// 信息 ID: 6fabf538ceffb6450af96abe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "utils::natural_cubic::impl$3::from_control_points_with_tangents", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::Spline3D::from_control_points_with_tangents; module=757, inline_record=32720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline
// 信息 ID: 9e3781b4a82ec9578001eaf9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::natural_cubic::Spline2D::from_control_points_with_tangents @ 0xde85bf -> 0xde6720 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline2D::from_control_points_with_tangents @ 0xde86c0 -> 0xde6720 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline3D::from_control_points_with_tangents @ 0xde913e -> 0xde6720 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline3D::from_control_points_with_tangents @ 0xde9292 -> 0xde6720 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::Spline3D::from_control_points_with_tangents @ 0xde93d2 -> 0xde6720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline::closure$0
// 信息 ID: 854814d6b6e0aaa4494a41eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::solve_clamped_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::solve_clamped_spline; module=757, inline_record=3612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline::closure$0::closure$0
// 信息 ID: 2efbe37e648f88fd32a4ea3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::solve_clamped_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::solve_clamped_spline; module=757, inline_record=3708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline::closure$0::closure$1
// 信息 ID: 6e3f28582c96932c3fab8ae0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::solve_clamped_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::solve_clamped_spline; module=757, inline_record=3784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_clamped_spline::closure$1
// 信息 ID: 9b7323ee6f51bf8f4be734ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::natural_cubic::solve_clamped_spline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::solve_clamped_spline; module=757, inline_record=6360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_cyclic
// 信息 ID: 57de332ec7672c63c84ac4e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=757, inline_record=10348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_cyclic::closure$0
// 信息 ID: 13851fd7d06e681a5a5b4cd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::natural_cubic::solve_cyclic", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=757, inline_record=11228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_cyclic::closure$1
// 信息 ID: 62cdc8a7cbf5156f18f077e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::natural_cubic::solve_cyclic", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::natural_cubic::compute_periodic_spline; module=757, inline_record=12100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::natural_cubic::solve_tridiagonal
// 信息 ID: de243c45a8150cff1c0a0dde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::natural_cubic::compute_periodic_spline @ 0xde74c4 -> 0xde6490 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::natural_cubic::compute_periodic_spline @ 0xde74f8 -> 0xde6490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
