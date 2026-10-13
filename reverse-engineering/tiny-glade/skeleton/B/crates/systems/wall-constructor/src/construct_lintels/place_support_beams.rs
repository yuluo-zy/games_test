// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 02d27541243a24b61d777f7ca49031eb5e38a3ad8eca0d9f1bc74fc99b93792e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: place_support_beams.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1450, 1467, 3035
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1467,"record_offset":2788,"rva":19150656,"va":5387859776}
// 编译实例 RVAs: 0x1243740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18593087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387859776,"raw_hex":"e8c14ae4fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387860746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10118565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387859776,"raw_hex":"e85b9a65ff","size":5,"soft_fail":false,"target_va":5377742176,"va":5387860736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141206ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-248325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387859776,"raw_hex":"e8fb35fcff","size":5,"soft_fail":false,"target_va":5387611856,"va":5387860176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141207c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-244911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387859776,"raw_hex":"e85143fcff","size":5,"soft_fail":false,"target_va":5387615248,"va":5387860154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-227070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387859776,"raw_hex":"e80289fcff","size":5,"soft_fail":false,"target_va":5387633712,"va":5387860777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t23681405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387859776,"raw_hex":"e87d596901","size":5,"soft_fail":false,"target_va":5411542296,"va":5387860886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1467,"record_offset":4760,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1467,"record_offset":5504,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":24469,"source_line":129,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1467,"record_offset":5632,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":24474,"source_line":538,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1467,"record_offset":4844,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1467,"record_offset":5028,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":44715,"source_line":550,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4512,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4612,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4272,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4328,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4420,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1467,"record_offset":6812,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":71007,"source_line":92,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1467,"record_offset":3572,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1467,"record_offset":3516,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1467,"record_offset":3088,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1467,"record_offset":5896,"parent_record_offset":5840,"ancestor_record_offset":2788,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1467,"record_offset":5204,"parent_record_offset":5028,"ancestor_record_offset":2788,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1467,"record_offset":4908,"parent_record_offset":4844,"ancestor_record_offset":2788,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1467,"record_offset":4876,"parent_record_offset":4844,"ancestor_record_offset":2788,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1467,"record_offset":5176,"parent_record_offset":5028,"ancestor_record_offset":2788,"inlinee":44716,"source_line":129,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1467,"record_offset":4964,"parent_record_offset":4844,"ancestor_record_offset":2788,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1467,"record_offset":4668,"parent_record_offset":4636,"ancestor_record_offset":2788,"inlinee":14178,"source_line":829,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1467,"record_offset":4208,"parent_record_offset":4176,"ancestor_record_offset":2788,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1467,"record_offset":4116,"parent_record_offset":4084,"ancestor_record_offset":2788,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1467,"record_offset":4024,"parent_record_offset":3992,"ancestor_record_offset":2788,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":3152,"parent_record_offset":3088,"ancestor_record_offset":2788,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":2788,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1467,"record_offset":5532,"parent_record_offset":5504,"ancestor_record_offset":2788,"inlinee":24471,"source_line":152,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1467,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":2788,"inlinee":24477,"source_line":646,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1467,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":2788,"inlinee":24480,"source_line":636,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1467,"record_offset":5560,"parent_record_offset":5532,"ancestor_record_offset":2788,"inlinee":24472,"source_line":70,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1467,"record_offset":5668,"parent_record_offset":5632,"ancestor_record_offset":2788,"inlinee":24476,"source_line":781,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1467,"record_offset":5728,"parent_record_offset":5696,"ancestor_record_offset":2788,"inlinee":24479,"source_line":796,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1467,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":2788,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1467,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":2788,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1467,"record_offset":5784,"parent_record_offset":5752,"ancestor_record_offset":2788,"inlinee":24482,"source_line":1199,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1467,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":2788,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4540,"parent_record_offset":4512,"ancestor_record_offset":2788,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":2788,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4084,"parent_record_offset":3796,"ancestor_record_offset":2788,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":4176,"parent_record_offset":3796,"ancestor_record_offset":2788,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":3992,"parent_record_offset":3796,"ancestor_record_offset":2788,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1467,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":2788,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1467,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":2788,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1467,"record_offset":6132,"parent_record_offset":6048,"ancestor_record_offset":2788,"inlinee":24486,"source_line":172,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1467,"record_offset":4576,"parent_record_offset":4540,"ancestor_record_offset":2788,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1467,"record_offset":3124,"parent_record_offset":3088,"ancestor_record_offset":2788,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1467,"record_offset":3216,"parent_record_offset":3088,"ancestor_record_offset":2788,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1467,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":2788,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1467,"record_offset":272,"rva":19146032,"va":5387855152}
// 编译实例 RVAs: 0x1242530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12339264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387855152,"raw_hex":"e8c0b743ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5387855291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23682517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387855152,"raw_hex":"e8d55d6901","size":5,"soft_fail":false,"target_va":5411542064,"va":5387859542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t23682724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387855152,"raw_hex":"e8a45e6901","size":5,"soft_fail":false,"target_va":5411542296,"va":5387859567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387855152,"raw_hex":"e8260e0000","size":5,"soft_fail":false,"target_va":5387861024,"va":5387857397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14278874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387855152,"raw_hex":"e8dae0d900","size":5,"soft_fail":false,"target_va":5402134976,"va":5387856097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387855152,"raw_hex":"e866100000","size":5,"soft_fail":false,"target_va":5387859776,"va":5387855573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::peekable::Peekable::peek
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\peekable.rs
//     evidence={"module":1467,"record_offset":868,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":40592,"source_line":216,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::peekable::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\peekable.rs
//     evidence={"module":1467,"record_offset":560,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":40590,"source_line":38,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1467,"record_offset":1364,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":1560,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":2588,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1467,"record_offset":2224,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7576,"source_line":378,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":1308,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::accept
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1467,"record_offset":1832,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":71202,"source_line":532,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1467,"record_offset":1228,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":71199,"source_line":474,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1467,"record_offset":2104,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":71203,"source_line":130,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1467,"record_offset":2164,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":71204,"source_line":139,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1467,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":272,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1467,"record_offset":592,"parent_record_offset":560,"ancestor_record_offset":272,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::peekable::impl$5::peek::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\peekable.rs
//     evidence={"module":1467,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":272,"inlinee":40594,"source_line":218,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":272,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":272,"inlinee":12495,"source_line":209,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":272,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":272,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":672,"parent_record_offset":640,"ancestor_record_offset":272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":272,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":272,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":768,"parent_record_offset":640,"ancestor_record_offset":272,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":272,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":272,"inlinee":12496,"source_line":750,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1467,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":272,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1467,"record_offset":1936,"parent_record_offset":1904,"ancestor_record_offset":272,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":272,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":272,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1467,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":272,"inlinee":71197,"source_line":60,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1467,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":272,"inlinee":71200,"source_line":80,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1467,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":272,"inlinee":71201,"source_line":83,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1161D
// 首条位置: {"module":1467,"inlinee":71197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11620
// 首条位置: {"module":1467,"inlinee":71200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11621
// 首条位置: {"module":1467,"inlinee":71201}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11623
// 首条位置: {"module":1467,"inlinee":71203}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11624
// 首条位置: {"module":1467,"inlinee":71204}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":2192,"parent_record_offset":2164,"ancestor_record_offset":2164,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 430
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1467,"record_offset":7036,"rva":19151904,"va":5387861024}
// 编译实例 RVAs: 0x1243c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12780036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e8fcfd3cff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387861519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12779801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e8e7fe3cff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387861300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141233380
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-68465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e88ff4feff","size":5,"soft_fail":false,"target_va":5387793280,"va":5387861740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe0b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14274187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e88bced900","size":5,"soft_fail":false,"target_va":5402135440,"va":5387861248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23985596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e8bcfd6d01","size":5,"soft_fail":false,"target_va":5411847200,"va":5387861599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23678495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e81f4e6901","size":5,"soft_fail":false,"target_va":5411540963,"va":5387862463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23680365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e86d556901","size":5,"soft_fail":false,"target_va":5411541808,"va":5387861438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23680764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e8fc566901","size":5,"soft_fail":false,"target_va":5411543104,"va":5387862335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_overflow_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23680896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e880576901","size":5,"soft_fail":false,"target_va":5411543200,"va":5387862299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23680798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e81e576901","size":5,"soft_fail":false,"target_va":5411543120,"va":5387862317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::u_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7443925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387861024,"raw_hex":"e82b6a8eff","size":5,"soft_fail":false,"target_va":5380417392,"va":5387861312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":7308,"parent_record_offset":7036,"ancestor_record_offset":7036,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1467,"record_offset":7756,"parent_record_offset":7036,"ancestor_record_offset":7036,"inlinee":71206,"source_line":542,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":9532,"parent_record_offset":9504,"ancestor_record_offset":7036,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":7036,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7036,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":9504,"parent_record_offset":9260,"ancestor_record_offset":7036,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":8072,"parent_record_offset":7980,"ancestor_record_offset":7036,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":9176,"parent_record_offset":9136,"ancestor_record_offset":7036,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":7036,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":7036,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":9260,"parent_record_offset":9216,"ancestor_record_offset":7036,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":9216,"parent_record_offset":9176,"ancestor_record_offset":7036,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":7036,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":9324,"parent_record_offset":9260,"ancestor_record_offset":7036,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":9616,"parent_record_offset":9084,"ancestor_record_offset":7036,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":9136,"parent_record_offset":9084,"ancestor_record_offset":7036,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":9044,"parent_record_offset":7756,"ancestor_record_offset":7036,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1467,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":7036,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1467,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":7036,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1467,"record_offset":8188,"parent_record_offset":8148,"ancestor_record_offset":7036,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1467,"record_offset":9084,"parent_record_offset":9044,"ancestor_record_offset":7036,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1467,"record_offset":9356,"parent_record_offset":9324,"ancestor_record_offset":7036,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1467,"record_offset":9388,"parent_record_offset":9356,"ancestor_record_offset":7036,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1467,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":7036,"inlinee":7523,"source_line":1826,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1467,"record_offset":9760,"parent_record_offset":9720,"ancestor_record_offset":7036,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1467,"record_offset":8352,"parent_record_offset":8328,"ancestor_record_offset":7036,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1467,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":7036,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":8792,"parent_record_offset":7756,"ancestor_record_offset":7036,"inlinee":71208,"source_line":1266,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::count::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":8944,"parent_record_offset":8904,"ancestor_record_offset":7036,"inlinee":71210,"source_line":1273,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":9720,"parent_record_offset":9680,"ancestor_record_offset":7036,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":9680,"parent_record_offset":9616,"ancestor_record_offset":7036,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":7036,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":8972,"parent_record_offset":8944,"ancestor_record_offset":7036,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":9420,"parent_record_offset":9388,"ancestor_record_offset":7036,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":7448,"parent_record_offset":7408,"ancestor_record_offset":7036,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":9452,"parent_record_offset":9420,"ancestor_record_offset":7036,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":8040,"parent_record_offset":8008,"ancestor_record_offset":7036,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1467,"record_offset":7380,"parent_record_offset":7340,"ancestor_record_offset":7036,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1467,"record_offset":8840,"parent_record_offset":8792,"ancestor_record_offset":7036,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":8904,"parent_record_offset":8792,"ancestor_record_offset":7036,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":9012,"parent_record_offset":8792,"ancestor_record_offset":7036,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":8376,"parent_record_offset":8352,"ancestor_record_offset":7036,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":7908,"parent_record_offset":7880,"ancestor_record_offset":7036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":8268,"parent_record_offset":8188,"ancestor_record_offset":7036,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":8228,"parent_record_offset":8188,"ancestor_record_offset":7036,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":7036,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":9804,"parent_record_offset":9616,"ancestor_record_offset":7036,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":7036,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":7688,"parent_record_offset":7652,"ancestor_record_offset":7036,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":8588,"parent_record_offset":7756,"ancestor_record_offset":7036,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":7340,"parent_record_offset":7308,"ancestor_record_offset":7036,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":7408,"parent_record_offset":7340,"ancestor_record_offset":7036,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1467,"record_offset":8328,"parent_record_offset":8148,"ancestor_record_offset":7036,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1467,"record_offset":8148,"parent_record_offset":7756,"ancestor_record_offset":7036,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::arch_walker::ArchWalker::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1467,"record_offset":9884,"parent_record_offset":7756,"ancestor_record_offset":7036,"inlinee":24391,"source_line":22,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::Rdp::into_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":1467,"record_offset":7880,"parent_record_offset":7756,"ancestor_record_offset":7036,"inlinee":71207,"source_line":59,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::new
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 474
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1161F
// 首条位置: {"module":1467,"inlinee":71199}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose
// 记录形式: compiled_procedure; instances=1
// 源码行: 484
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3035,"record_offset":272,"rva":33425856,"va":5402134976}
// 编译实例 RVAs: 0x1fe09c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9406879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402134976,"raw_hex":"e89f898f00","size":5,"soft_fail":false,"target_va":5411542296,"va":5402135412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3035,"record_offset":868,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3035,"record_offset":460,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3035,"record_offset":896,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3035,"record_offset":664,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3035,"record_offset":832,"parent_record_offset":692,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3035,"record_offset":536,"parent_record_offset":492,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3035,"record_offset":560,"parent_record_offset":492,"ancestor_record_offset":272,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3035,"record_offset":492,"parent_record_offset":460,"ancestor_record_offset":272,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3035,"record_offset":760,"parent_record_offset":724,"ancestor_record_offset":272,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3035,"record_offset":724,"parent_record_offset":692,"ancestor_record_offset":272,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3035,"record_offset":692,"parent_record_offset":664,"ancestor_record_offset":272,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3035,"record_offset":600,"parent_record_offset":560,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::accept
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 532
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11622
// 首条位置: {"module":1467,"inlinee":71202}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 542
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11626
// 首条位置: {"module":1467,"inlinee":71206}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":9044,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":8792,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":71208,"source_line":1266,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":9916,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":8588,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1467,"record_offset":8148,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1467,"record_offset":9884,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":24391,"source_line":22,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] utils::rdp::Rdp::into_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":1467,"record_offset":7880,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":71207,"source_line":59,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":9532,"parent_record_offset":9504,"ancestor_record_offset":7756,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":7756,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7756,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":9504,"parent_record_offset":9260,"ancestor_record_offset":7756,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":8072,"parent_record_offset":7980,"ancestor_record_offset":7756,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":9176,"parent_record_offset":9136,"ancestor_record_offset":7756,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":7756,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":7756,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":9260,"parent_record_offset":9216,"ancestor_record_offset":7756,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":9216,"parent_record_offset":9176,"ancestor_record_offset":7756,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":7756,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":9324,"parent_record_offset":9260,"ancestor_record_offset":7756,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":9616,"parent_record_offset":9084,"ancestor_record_offset":7756,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":9136,"parent_record_offset":9084,"ancestor_record_offset":7756,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1467,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":7756,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1467,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":7756,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1467,"record_offset":8188,"parent_record_offset":8148,"ancestor_record_offset":7756,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1467,"record_offset":9084,"parent_record_offset":9044,"ancestor_record_offset":7756,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1467,"record_offset":9356,"parent_record_offset":9324,"ancestor_record_offset":7756,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1467,"record_offset":9388,"parent_record_offset":9356,"ancestor_record_offset":7756,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1467,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":7756,"inlinee":7523,"source_line":1826,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1467,"record_offset":9760,"parent_record_offset":9720,"ancestor_record_offset":7756,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1467,"record_offset":8352,"parent_record_offset":8328,"ancestor_record_offset":7756,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1467,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":7756,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::count::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":8944,"parent_record_offset":8904,"ancestor_record_offset":7756,"inlinee":71210,"source_line":1273,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":9720,"parent_record_offset":9680,"ancestor_record_offset":7756,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1467,"record_offset":9680,"parent_record_offset":9616,"ancestor_record_offset":7756,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":7756,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":8972,"parent_record_offset":8944,"ancestor_record_offset":7756,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":9420,"parent_record_offset":9388,"ancestor_record_offset":7756,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":8748,"parent_record_offset":8700,"ancestor_record_offset":7756,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":9452,"parent_record_offset":9420,"ancestor_record_offset":7756,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":8040,"parent_record_offset":8008,"ancestor_record_offset":7756,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1467,"record_offset":8668,"parent_record_offset":8624,"ancestor_record_offset":7756,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1467,"record_offset":8840,"parent_record_offset":8792,"ancestor_record_offset":7756,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":8904,"parent_record_offset":8792,"ancestor_record_offset":7756,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":9012,"parent_record_offset":8792,"ancestor_record_offset":7756,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":8376,"parent_record_offset":8352,"ancestor_record_offset":7756,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":7908,"parent_record_offset":7880,"ancestor_record_offset":7756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":8268,"parent_record_offset":8188,"ancestor_record_offset":7756,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":8228,"parent_record_offset":8188,"ancestor_record_offset":7756,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":7756,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":9804,"parent_record_offset":9616,"ancestor_record_offset":7756,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":8624,"parent_record_offset":8588,"ancestor_record_offset":7756,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":8700,"parent_record_offset":8624,"ancestor_record_offset":7756,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1467,"record_offset":8328,"parent_record_offset":8148,"ancestor_record_offset":7756,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 578
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1155E
// 首条位置: {"module":1450,"inlinee":71006}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":13476,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1450,"record_offset":13748,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":24469,"source_line":129,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1450,"record_offset":13860,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":24474,"source_line":538,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13560,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":13228,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":13328,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":13140,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1450,"record_offset":14908,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":71007,"source_line":92,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":14060,"parent_record_offset":14004,"ancestor_record_offset":13072,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":13072,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":13616,"parent_record_offset":13560,"ancestor_record_offset":13072,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":13384,"parent_record_offset":13352,"ancestor_record_offset":13072,"inlinee":14178,"source_line":829,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":13412,"parent_record_offset":13352,"ancestor_record_offset":13072,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":13168,"parent_record_offset":13140,"ancestor_record_offset":13072,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":13072,"inlinee":24471,"source_line":152,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":13072,"inlinee":24477,"source_line":646,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":13072,"inlinee":24480,"source_line":636,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13796,"parent_record_offset":13772,"ancestor_record_offset":13072,"inlinee":24472,"source_line":70,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13896,"parent_record_offset":13860,"ancestor_record_offset":13072,"inlinee":24476,"source_line":781,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13952,"parent_record_offset":13920,"ancestor_record_offset":13072,"inlinee":24479,"source_line":796,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13524,"parent_record_offset":13500,"ancestor_record_offset":13072,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":13072,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":14004,"parent_record_offset":13976,"ancestor_record_offset":13072,"inlinee":24482,"source_line":1199,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":13072,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":13256,"parent_record_offset":13228,"ancestor_record_offset":13072,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":13352,"parent_record_offset":13328,"ancestor_record_offset":13072,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1450,"record_offset":14240,"parent_record_offset":14212,"ancestor_record_offset":13072,"inlinee":24486,"source_line":172,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1450,"record_offset":13292,"parent_record_offset":13256,"ancestor_record_offset":13072,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01450.jsonl.gz"}
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
// 完整结构及原始类型记录: place_support_beams.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>", "constructor": "system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams", "arguments": ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: place_support_beams.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/B-core/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/B-emitters/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: place_support_beams.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 信息 ID: f05c8e835be5e18a6bc5ef0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12432fe -> 0x1243c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1243575 -> 0x1243c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1242df5 -> 0x1243c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1242f16 -> 0x1243c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x124310e -> 0x1243c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::accept
// 信息 ID: cd92aff90618ccf8fa2a06b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=1832; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=2028; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=2476; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=2684; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=2756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::new
// 信息 ID: 9c00b2174303a5f095505ba7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=1228; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=1268; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=2308; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=2548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose
// 信息 ID: 99519f2dd208261991f563e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12431fb -> 0x1fe09c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x124345b -> 0x1fe09c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12428e1 -> 0x1fe09c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x124291c -> 0x1fe09c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1242d1a -> 0x1fe09c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1242d6c -> 0x1fe09c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12429aa -> 0x1fe09c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1242b51 -> 0x1fe09c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 信息 ID: ecf1eb7fe1ab8857bce310dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x124337f -> 0x1243740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12435f6 -> 0x1243740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1242eb5 -> 0x1243740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12426d5 -> 0x1243740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1243008 -> 0x1243740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1243072 -> 0x1243740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams
// 信息 ID: b3cf25bb913e0b625d391e4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=7756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
// 信息 ID: cf0fb5d26b9469b1e6c6b6cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0 <- system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1450, inline_record=13072; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1450, inline_record=14976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$0
// 信息 ID: fb14320f1e26fb84f07adcbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=816; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=1168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$1
// 信息 ID: c7b8db981249367e235d9321
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=1332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$2
// 信息 ID: 2913cfc515148edeec163cd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=1528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$3
// 信息 ID: 13e0647d663b99424e46ac10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=2104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$4
// 信息 ID: e5c0ba77c3f059e90b247903
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=2164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 信息 ID: 557b6930672d98533963007f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76ea14 -> 0x1242530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
