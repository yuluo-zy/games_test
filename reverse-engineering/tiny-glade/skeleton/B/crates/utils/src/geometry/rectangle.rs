// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/geometry/rectangle.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 94aa565632f8541282d61dc9f9718bfcabcaad58c03b2f8483c0a26e137b225f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 39, 73, 420, 428, 429, 432, 433, 435, 439, 441, 442, 443, 445, 450, 453, 456, 457, 466, 496, 527, 547, 575, 582, 768, 1415, 1435, 1451, 1497, 1517, 1518, 1522, 1532, 1566, 1576, 1606, 1618, 1623, 1673, 2085, 2599, 3007, 3051, 3053, 3063, 3064, 3376, 3465, 3811, 4659
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":17212,"rva":13213232,"va":5381922352}
// 编译实例 RVAs: 0xc99e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29618938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381922352,"raw_hex":"e8faf2c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381923121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29618802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381922352,"raw_hex":"e872f2c301","size":5,"soft_fail":false,"target_va":5411541904,"va":5381923097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":19860,"parent_record_offset":17212,"ancestor_record_offset":17212,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":18008,"parent_record_offset":17212,"ancestor_record_offset":17212,"inlinee":24624,"source_line":6,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18064,"parent_record_offset":17212,"ancestor_record_offset":17212,"inlinee":45008,"source_line":394,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":19972,"parent_record_offset":17212,"ancestor_record_offset":17212,"inlinee":45011,"source_line":420,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":17372,"parent_record_offset":17212,"ancestor_record_offset":17212,"inlinee":45006,"source_line":283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":20308,"parent_record_offset":20284,"ancestor_record_offset":17212,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":17508,"parent_record_offset":17484,"ancestor_record_offset":17212,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":17212,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":17212,"inlinee":5882,"source_line":1777,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18612,"parent_record_offset":18544,"ancestor_record_offset":17212,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":17212,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18300,"parent_record_offset":18244,"ancestor_record_offset":17212,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":17568,"parent_record_offset":17432,"ancestor_record_offset":17212,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":768,"record_offset":21168,"parent_record_offset":21140,"ancestor_record_offset":17212,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":19600,"parent_record_offset":19472,"ancestor_record_offset":17212,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":17212,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":17460,"parent_record_offset":17432,"ancestor_record_offset":17212,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18188,"parent_record_offset":18136,"ancestor_record_offset":17212,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":17212,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18220,"parent_record_offset":18112,"ancestor_record_offset":17212,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18728,"parent_record_offset":18704,"ancestor_record_offset":17212,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":17212,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18680,"parent_record_offset":18656,"ancestor_record_offset":17212,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18656,"parent_record_offset":18352,"ancestor_record_offset":17212,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":17212,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":19564,"parent_record_offset":19532,"ancestor_record_offset":17212,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":17432,"parent_record_offset":17372,"ancestor_record_offset":17212,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":20332,"parent_record_offset":20308,"ancestor_record_offset":17212,"inlinee":27913,"source_line":6,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18112,"parent_record_offset":18088,"ancestor_record_offset":17212,"inlinee":45010,"source_line":450,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::get_corner_unnormal
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18088,"parent_record_offset":18064,"ancestor_record_offset":17212,"inlinee":45009,"source_line":366,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":17212,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":17400,"parent_record_offset":17372,"ancestor_record_offset":17212,"inlinee":45005,"source_line":241,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1566,"record_offset":264,"rva":19602272,"va":5388311392}
// 编译实例 RVAs: 0x12b1b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23228145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388311392,"raw_hex":"e8f16e6201","size":5,"soft_fail":false,"target_va":5411542064,"va":5388313914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23228009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388311392,"raw_hex":"e8696e6201","size":5,"soft_fail":false,"target_va":5411541904,"va":5388313890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::stairs::walk_shape::closure$0
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388311392,"raw_hex":"e89b040000","size":5,"soft_fail":false,"target_va":5388313920,"va":5388312736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6394383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388311392,"raw_hex":"e8f16d9eff","size":5,"soft_fail":false,"target_va":5381917360,"va":5388311738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6387352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388311392,"raw_hex":"e868899eff","size":5,"soft_fail":false,"target_va":5381924256,"va":5388311603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6391096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388311392,"raw_hex":"e8c87a9eff","size":5,"soft_fail":false,"target_va":5381920464,"va":5388311555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6391878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388311392,"raw_hex":"e8ba779eff","size":5,"soft_fail":false,"target_va":5381920768,"va":5388312641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1566,"record_offset":2992,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1566,"record_offset":508,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1566,"record_offset":704,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1566,"record_offset":536,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::stairs::walk_shape::closure$1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs.rs
//     evidence={"module":1566,"record_offset":3076,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":72029,"source_line":1558,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$13::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":1566,"record_offset":1696,"parent_record_offset":1216,"ancestor_record_offset":264,"inlinee":72028,"source_line":340,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1566,"record_offset":872,"parent_record_offset":844,"ancestor_record_offset":264,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1566,"record_offset":1072,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1566,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":264,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1566,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":264,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1566,"record_offset":3684,"parent_record_offset":3544,"ancestor_record_offset":264,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1566,"record_offset":3940,"parent_record_offset":3912,"ancestor_record_offset":264,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1566,"record_offset":2540,"parent_record_offset":2512,"ancestor_record_offset":264,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1566,"record_offset":2596,"parent_record_offset":2568,"ancestor_record_offset":264,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1566,"record_offset":1256,"parent_record_offset":1216,"ancestor_record_offset":264,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$24::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1566,"record_offset":1192,"parent_record_offset":844,"ancestor_record_offset":264,"inlinee":11083,"source_line":280,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1566,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":264,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1566,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1566,"record_offset":1432,"parent_record_offset":1216,"ancestor_record_offset":264,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1566,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":264,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1566,"record_offset":740,"parent_record_offset":704,"ancestor_record_offset":264,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1566,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":264,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1566,"record_offset":776,"parent_record_offset":740,"ancestor_record_offset":264,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1566,"record_offset":932,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1566,"record_offset":2624,"parent_record_offset":2596,"ancestor_record_offset":264,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1566,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":264,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1566,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":264,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1566,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":264,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1566,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":264,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1566,"record_offset":1316,"parent_record_offset":1256,"ancestor_record_offset":264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1566,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":264,"inlinee":11824,"source_line":46,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1566,"record_offset":1016,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1566,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":264,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1566,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":264,"inlinee":13189,"source_line":163,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1566,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":264,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1566,"record_offset":3572,"parent_record_offset":3544,"ancestor_record_offset":264,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1566,"record_offset":1284,"parent_record_offset":1256,"ancestor_record_offset":264,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1566,"record_offset":2956,"parent_record_offset":704,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1566,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":264,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1566,"record_offset":2920,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1566,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":264,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1566,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1566,"record_offset":1388,"parent_record_offset":1344,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1566,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":264,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1566,"record_offset":2420,"parent_record_offset":2364,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1566,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1566,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1566,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1566,"record_offset":3124,"parent_record_offset":3076,"ancestor_record_offset":264,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1566,"record_offset":2512,"parent_record_offset":1968,"ancestor_record_offset":264,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1566,"record_offset":2340,"parent_record_offset":1968,"ancestor_record_offset":264,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1566,"record_offset":1968,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1566,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":264,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1566,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":264,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/01566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::expand_along_edge
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":45420,"rva":13223456,"va":5381932576}
// 编译实例 RVAs: 0xc9c620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t978295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381932576,"raw_hex":"e877ed0e00","size":5,"soft_fail":false,"target_va":5382911104,"va":5381932804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29609231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381932576,"raw_hex":"e80fcdc301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381932828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":45576,"parent_record_offset":45420,"ancestor_record_offset":45420,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":46204,"parent_record_offset":45420,"ancestor_record_offset":45420,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":46140,"parent_record_offset":45420,"ancestor_record_offset":45420,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":45728,"parent_record_offset":45420,"ancestor_record_offset":45420,"inlinee":45011,"source_line":420,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":45816,"parent_record_offset":45792,"ancestor_record_offset":45420,"inlinee":9733,"source_line":1939,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":45840,"parent_record_offset":45816,"ancestor_record_offset":45420,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":46096,"parent_record_offset":46008,"ancestor_record_offset":45420,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":46036,"parent_record_offset":46008,"ancestor_record_offset":45420,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":46064,"parent_record_offset":46008,"ancestor_record_offset":45420,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":768,"record_offset":46288,"parent_record_offset":46260,"ancestor_record_offset":45420,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":768,"record_offset":45620,"parent_record_offset":45576,"ancestor_record_offset":45420,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":46228,"parent_record_offset":46204,"ancestor_record_offset":45420,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":46168,"parent_record_offset":46140,"ancestor_record_offset":45420,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":45768,"parent_record_offset":45728,"ancestor_record_offset":45420,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":45792,"parent_record_offset":45768,"ancestor_record_offset":45420,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":45952,"parent_record_offset":45900,"ancestor_record_offset":45420,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":46008,"parent_record_offset":45984,"ancestor_record_offset":45420,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":45984,"parent_record_offset":45876,"ancestor_record_offset":45420,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":45924,"parent_record_offset":45900,"ancestor_record_offset":45420,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":45876,"parent_record_offset":45728,"ancestor_record_offset":45420,"inlinee":45010,"source_line":450,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":45900,"parent_record_offset":45876,"ancestor_record_offset":45420,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":33644,"rva":13217792,"va":5381926912}
// 编译实例 RVAs: 0xc9b000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29614247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381926912,"raw_hex":"e8a7e0c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381927812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29614111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381926912,"raw_hex":"e81fe0c301","size":5,"soft_fail":false,"target_va":5411541904,"va":5381927788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381926912,"raw_hex":"e880f9ffff","size":5,"soft_fail":false,"target_va":5381925328,"va":5381926987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":768,"record_offset":34688,"parent_record_offset":33644,"ancestor_record_offset":33644,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":34828,"parent_record_offset":33644,"ancestor_record_offset":33644,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":33804,"parent_record_offset":33644,"ancestor_record_offset":33644,"inlinee":45007,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":34032,"parent_record_offset":33644,"ancestor_record_offset":33644,"inlinee":45006,"source_line":283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":34232,"parent_record_offset":34208,"ancestor_record_offset":33644,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":34256,"parent_record_offset":34232,"ancestor_record_offset":33644,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":34296,"parent_record_offset":34156,"ancestor_record_offset":33644,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":768,"record_offset":35008,"parent_record_offset":34828,"ancestor_record_offset":33644,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":34796,"parent_record_offset":34688,"ancestor_record_offset":33644,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":34740,"parent_record_offset":34688,"ancestor_record_offset":33644,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":34712,"parent_record_offset":34688,"ancestor_record_offset":33644,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":34208,"parent_record_offset":34184,"ancestor_record_offset":33644,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":34184,"parent_record_offset":34156,"ancestor_record_offset":33644,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":34156,"parent_record_offset":34032,"ancestor_record_offset":33644,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":33904,"parent_record_offset":33852,"ancestor_record_offset":33644,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":33852,"parent_record_offset":33804,"ancestor_record_offset":33644,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":34072,"parent_record_offset":34032,"ancestor_record_offset":33644,"inlinee":45005,"source_line":241,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":21952,"rva":13214224,"va":5381923344}
// 编译实例 RVAs: 0xc9a210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29618147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381923344,"raw_hex":"e8e3efc301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381923912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":22232,"parent_record_offset":21952,"ancestor_record_offset":21952,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":22176,"parent_record_offset":21952,"ancestor_record_offset":21952,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":22116,"parent_record_offset":21952,"ancestor_record_offset":21952,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":22384,"parent_record_offset":22256,"ancestor_record_offset":21952,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":22200,"parent_record_offset":22176,"ancestor_record_offset":21952,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":768,"record_offset":23908,"parent_record_offset":23876,"ancestor_record_offset":21952,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":22316,"parent_record_offset":22284,"ancestor_record_offset":21952,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":22284,"parent_record_offset":22256,"ancestor_record_offset":21952,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":22256,"parent_record_offset":22232,"ancestor_record_offset":21952,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":22348,"parent_record_offset":22316,"ancestor_record_offset":21952,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":22144,"parent_record_offset":22116,"ancestor_record_offset":21952,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":30432,"rva":13216960,"va":5381926080}
// 编译实例 RVAs: 0xc9acc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29615633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381926080,"raw_hex":"e811e6c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381926426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29615497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381926080,"raw_hex":"e889e5c301","size":5,"soft_fail":false,"target_va":5411541904,"va":5381926402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":31216,"parent_record_offset":30432,"ancestor_record_offset":30432,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":30580,"parent_record_offset":30432,"ancestor_record_offset":30432,"inlinee":45006,"source_line":283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":30716,"parent_record_offset":30692,"ancestor_record_offset":30432,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":30740,"parent_record_offset":30716,"ancestor_record_offset":30432,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":30776,"parent_record_offset":30640,"ancestor_record_offset":30432,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":768,"record_offset":31556,"parent_record_offset":31528,"ancestor_record_offset":30432,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":30692,"parent_record_offset":30668,"ancestor_record_offset":30432,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":30668,"parent_record_offset":30640,"ancestor_record_offset":30432,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":30640,"parent_record_offset":30580,"ancestor_record_offset":30432,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":30608,"parent_record_offset":30580,"ancestor_record_offset":30432,"inlinee":45005,"source_line":241,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_pos_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":12324,"rva":13211648,"va":5381920768}
// 编译实例 RVAs: 0xc99800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29620525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920768,"raw_hex":"e82df9c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381921534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29620389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920768,"raw_hex":"e8a5f8c301","size":5,"soft_fail":false,"target_va":5411541904,"va":5381921510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920768,"raw_hex":"e892000000","size":5,"soft_fail":false,"target_va":5381921568,"va":5381921417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":14384,"parent_record_offset":12324,"ancestor_record_offset":12324,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":13108,"parent_record_offset":12324,"ancestor_record_offset":12324,"inlinee":24624,"source_line":6,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":13164,"parent_record_offset":12324,"ancestor_record_offset":12324,"inlinee":45007,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":12476,"parent_record_offset":12324,"ancestor_record_offset":12324,"inlinee":45006,"source_line":283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":14572,"parent_record_offset":14548,"ancestor_record_offset":12324,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":12324,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":12632,"parent_record_offset":12608,"ancestor_record_offset":12324,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":14620,"parent_record_offset":14596,"ancestor_record_offset":12324,"inlinee":5882,"source_line":1777,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":13696,"parent_record_offset":13604,"ancestor_record_offset":12324,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":13636,"parent_record_offset":13604,"ancestor_record_offset":12324,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":13664,"parent_record_offset":13604,"ancestor_record_offset":12324,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":12668,"parent_record_offset":12532,"ancestor_record_offset":12324,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":768,"record_offset":14892,"parent_record_offset":14864,"ancestor_record_offset":12324,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":13728,"parent_record_offset":13560,"ancestor_record_offset":12324,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":12584,"parent_record_offset":12560,"ancestor_record_offset":12324,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":12560,"parent_record_offset":12532,"ancestor_record_offset":12324,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":13312,"parent_record_offset":13260,"ancestor_record_offset":12324,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":13604,"parent_record_offset":13560,"ancestor_record_offset":12324,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":13560,"parent_record_offset":13532,"ancestor_record_offset":12324,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":12324,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":12532,"parent_record_offset":12476,"ancestor_record_offset":12324,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":14596,"parent_record_offset":14572,"ancestor_record_offset":12324,"inlinee":27913,"source_line":6,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":13228,"parent_record_offset":13196,"ancestor_record_offset":12324,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":13196,"parent_record_offset":13164,"ancestor_record_offset":12324,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":13260,"parent_record_offset":13164,"ancestor_record_offset":12324,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":12504,"parent_record_offset":12476,"ancestor_record_offset":12324,"inlinee":45005,"source_line":241,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":35188,"rva":13218848,"va":5381927968}
// 编译实例 RVAs: 0xc9b420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29613489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381927968,"raw_hex":"e8b1ddc301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381928570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":768,"record_offset":35344,"parent_record_offset":35188,"ancestor_record_offset":35188,"inlinee":45014,"source_line":73,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::intersects
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":768,"record_offset":36328,"parent_record_offset":35188,"ancestor_record_offset":35188,"inlinee":45018,"source_line":133,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::local_aabb_of
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":35516,"parent_record_offset":35188,"ancestor_record_offset":35188,"inlinee":45015,"source_line":582,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":35804,"parent_record_offset":35684,"ancestor_record_offset":35188,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":35744,"parent_record_offset":35684,"ancestor_record_offset":35188,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":35716,"parent_record_offset":35684,"ancestor_record_offset":35188,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":35772,"parent_record_offset":35684,"ancestor_record_offset":35188,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":36200,"parent_record_offset":36056,"ancestor_record_offset":35188,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":37260,"parent_record_offset":37236,"ancestor_record_offset":35188,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":35484,"parent_record_offset":35460,"ancestor_record_offset":35188,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":35864,"parent_record_offset":35840,"ancestor_record_offset":35188,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":35628,"parent_record_offset":35572,"ancestor_record_offset":35188,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":35684,"parent_record_offset":35660,"ancestor_record_offset":35188,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":35660,"parent_record_offset":35516,"ancestor_record_offset":35188,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35396,"parent_record_offset":35372,"ancestor_record_offset":35188,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35372,"parent_record_offset":35344,"ancestor_record_offset":35188,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35600,"parent_record_offset":35572,"ancestor_record_offset":35188,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":37236,"parent_record_offset":37072,"ancestor_record_offset":35188,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35840,"parent_record_offset":35516,"ancestor_record_offset":35188,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35460,"parent_record_offset":35344,"ancestor_record_offset":35188,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":35424,"parent_record_offset":35396,"ancestor_record_offset":35188,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":36056,"parent_record_offset":35516,"ancestor_record_offset":35188,"inlinee":45016,"source_line":643,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":36104,"parent_record_offset":36056,"ancestor_record_offset":35188,"inlinee":45017,"source_line":638,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":35572,"parent_record_offset":35516,"ancestor_record_offset":35188,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":14932,"rva":13212448,"va":5381921568}
// 编译实例 RVAs: 0xc99b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29619871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381921568,"raw_hex":"e89ff6c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381922188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29619623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381921568,"raw_hex":"e8a7f5c301","size":5,"soft_fail":false,"target_va":5411541904,"va":5381922276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":16504,"parent_record_offset":14932,"ancestor_record_offset":14932,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":15684,"parent_record_offset":14932,"ancestor_record_offset":14932,"inlinee":45007,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":15088,"parent_record_offset":14932,"ancestor_record_offset":14932,"inlinee":45006,"source_line":283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::inverse_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":16184,"parent_record_offset":14932,"ancestor_record_offset":14932,"inlinee":44711,"source_line":257,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":15292,"parent_record_offset":15268,"ancestor_record_offset":14932,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":14932,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":16048,"parent_record_offset":15960,"ancestor_record_offset":14932,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":15988,"parent_record_offset":15960,"ancestor_record_offset":14932,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":16016,"parent_record_offset":15960,"ancestor_record_offset":14932,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":15356,"parent_record_offset":15216,"ancestor_record_offset":14932,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":768,"record_offset":16800,"parent_record_offset":16772,"ancestor_record_offset":14932,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":16116,"parent_record_offset":16084,"ancestor_record_offset":14932,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":16564,"parent_record_offset":16532,"ancestor_record_offset":14932,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":15268,"parent_record_offset":15244,"ancestor_record_offset":14932,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":15244,"parent_record_offset":15216,"ancestor_record_offset":14932,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":15900,"parent_record_offset":15844,"ancestor_record_offset":14932,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":15960,"parent_record_offset":15936,"ancestor_record_offset":14932,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":15936,"parent_record_offset":15684,"ancestor_record_offset":14932,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":15872,"parent_record_offset":15844,"ancestor_record_offset":14932,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":16532,"parent_record_offset":16504,"ancestor_record_offset":14932,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":16084,"parent_record_offset":15684,"ancestor_record_offset":14932,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":15216,"parent_record_offset":15088,"ancestor_record_offset":14932,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":15808,"parent_record_offset":15752,"ancestor_record_offset":14932,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":15752,"parent_record_offset":15684,"ancestor_record_offset":14932,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":15844,"parent_record_offset":15684,"ancestor_record_offset":14932,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":15132,"parent_record_offset":15088,"ancestor_record_offset":14932,"inlinee":45005,"source_line":241,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81B5
// 首条位置: {"module":582,"inlinee":33205}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":252,"rva":13206400,"va":5381915520}
// 编译实例 RVAs: 0xc98380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29626274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915520,"raw_hex":"e8a20fc401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381915785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":380,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44986,"source_line":141,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":564,"parent_record_offset":504,"ancestor_record_offset":252,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":424,"parent_record_offset":380,"ancestor_record_offset":252,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":536,"parent_record_offset":504,"ancestor_record_offset":252,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":480,"parent_record_offset":380,"ancestor_record_offset":252,"inlinee":44987,"source_line":525,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":252,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::translate
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":608,"rva":13206720,"va":5381915840}
// 编译实例 RVAs: 0xc984c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":744,"parent_record_offset":608,"ancestor_record_offset":608,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":608,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::rotate_projective_xz
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":804,"rva":13206768,"va":5381915888}
// 编译实例 RVAs: 0xc984f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":952,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":804,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::set_orientation_xz
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":1064,"rva":13206816,"va":5381915936}
// 编译实例 RVAs: 0xc98520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::set_size
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":1200,"rva":13206832,"va":5381915952}
// 编译实例 RVAs: 0xc98530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":1328,"rva":13206912,"va":5381916032}
// 编译实例 RVAs: 0xc98580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$6::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":1476,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":44989,"source_line":520,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::contains_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":1504,"rva":13206944,"va":5381916064}
// 编译实例 RVAs: 0xc985a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":1648,"parent_record_offset":1504,"ancestor_record_offset":1504,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":1752,"parent_record_offset":1504,"ancestor_record_offset":1504,"inlinee":44990,"source_line":454,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":1504,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":1716,"parent_record_offset":1696,"ancestor_record_offset":1504,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2108,"parent_record_offset":2020,"ancestor_record_offset":1504,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1504,"inlinee":44994,"source_line":510,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2048,"parent_record_offset":2020,"ancestor_record_offset":1504,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2076,"parent_record_offset":2020,"ancestor_record_offset":1504,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1504,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":1504,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/sse2/bvec4a.rs
//     evidence={"module":768,"record_offset":2200,"parent_record_offset":1776,"ancestor_record_offset":1504,"inlinee":25838,"source_line":91,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":1880,"parent_record_offset":1816,"ancestor_record_offset":1504,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1504,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":1996,"parent_record_offset":1776,"ancestor_record_offset":1504,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":768,"record_offset":2144,"parent_record_offset":1776,"ancestor_record_offset":1504,"inlinee":44993,"source_line":364,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":768,"record_offset":1912,"parent_record_offset":1776,"ancestor_record_offset":1504,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":1848,"parent_record_offset":1816,"ancestor_record_offset":1504,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":1944,"parent_record_offset":1776,"ancestor_record_offset":1504,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":1504,"inlinee":44991,"source_line":459,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":1816,"parent_record_offset":1776,"ancestor_record_offset":1504,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::center
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2837
// 首条位置: {"module":73,"inlinee":10295}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::height
// 记录形式: inline_instance, inlinee_declaration; instances=74
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2836
// 首条位置: {"module":73,"inlinee":10294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::width
// 记录形式: inline_instance, inlinee_declaration; instances=68
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2834
// 首条位置: {"module":73,"inlinee":10292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::length
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2835
// 首条位置: {"module":73,"inlinee":10293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::expand
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":2240,"rva":13207072,"va":5381916192}
// 编译实例 RVAs: 0xc98620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::extent
// 记录形式: compiled_procedure; instances=1
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":2364,"rva":13207136,"va":5381916256}
// 编译实例 RVAs: 0xc98660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":768,"record_offset":2584,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2508,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2556,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":10293,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2532,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::points
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":2616,"rva":13207184,"va":5381916304}
// 编译实例 RVAs: 0xc98690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29625256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381916304,"raw_hex":"e8a80bc401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381916803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2748,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":44995,"source_line":199,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3332,"parent_record_offset":3124,"ancestor_record_offset":2616,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3248,"parent_record_offset":3124,"ancestor_record_offset":2616,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3204,"parent_record_offset":3124,"ancestor_record_offset":2616,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3288,"parent_record_offset":3124,"ancestor_record_offset":2616,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":2616,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":2812,"parent_record_offset":2748,"ancestor_record_offset":2616,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2616,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":3432,"parent_record_offset":3380,"ancestor_record_offset":2616,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3000,"parent_record_offset":2948,"ancestor_record_offset":2616,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3124,"parent_record_offset":3072,"ancestor_record_offset":2616,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3072,"parent_record_offset":2908,"ancestor_record_offset":2616,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2616,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":3380,"parent_record_offset":2908,"ancestor_record_offset":2616,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2784,"parent_record_offset":2748,"ancestor_record_offset":2616,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":2616,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":3032,"parent_record_offset":2908,"ancestor_record_offset":2616,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2948,"parent_record_offset":2908,"ancestor_record_offset":2616,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$1::as_points3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2616,"inlinee":44998,"source_line":202,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1182E
// 首条位置: {"module":1532,"inlinee":71726}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":48664,"rva":13224992,"va":5381934112}
// 编译实例 RVAs: 0xc9cc20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t958299","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381934112,"raw_hex":"e95b9f0e00","size":5,"soft_fail":false,"target_va":5382892432,"va":5381934128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1623,"record_offset":2148,"rva":22526256,"va":5391235376}
// 编译实例 RVAs: 0x157b930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1623,"record_offset":2324,"rva":22526576,"va":5391235696}
// 编译实例 RVAs: 0x157ba70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1623,"record_offset":2504,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1623,"record_offset":2524,"parent_record_offset":2504,"ancestor_record_offset":2324,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1623,"record_offset":2544,"parent_record_offset":2524,"ancestor_record_offset":2324,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1623,"record_offset":2564,"parent_record_offset":2544,"ancestor_record_offset":2324,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11817
// 首条位置: {"module":1532,"inlinee":71703}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11816
// 首条位置: {"module":1532,"inlinee":71702}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":20076,"parent_record_offset":20032,"ancestor_record_offset":20032,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":20288,"parent_record_offset":20264,"ancestor_record_offset":20032,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":20336,"parent_record_offset":20312,"ancestor_record_offset":20032,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":20312,"parent_record_offset":20288,"ancestor_record_offset":20032,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":20032,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":20192,"parent_record_offset":20076,"ancestor_record_offset":20032,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":20264,"parent_record_offset":20240,"ancestor_record_offset":20032,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":20240,"parent_record_offset":20216,"ancestor_record_offset":20032,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":20416,"parent_record_offset":20076,"ancestor_record_offset":20032,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":20136,"parent_record_offset":20076,"ancestor_record_offset":20032,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1532,"record_offset":20160,"parent_record_offset":20136,"ancestor_record_offset":20032,"inlinee":71703,"source_line":114,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":48812,"rva":13225024,"va":5381934144}
// 编译实例 RVAs: 0xc9cc40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t958267","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381934144,"raw_hex":"e93b9f0e00","size":5,"soft_fail":false,"target_va":5382892432,"va":5381934160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11813
// 首条位置: {"module":1532,"inlinee":71699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":21372,"parent_record_offset":19644,"ancestor_record_offset":19644,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":21544,"parent_record_offset":19644,"ancestor_record_offset":19644,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":22632,"parent_record_offset":19644,"ancestor_record_offset":19644,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":19732,"parent_record_offset":19644,"ancestor_record_offset":19644,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":20664,"parent_record_offset":19644,"ancestor_record_offset":19644,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":20288,"parent_record_offset":20264,"ancestor_record_offset":19644,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":20336,"parent_record_offset":20312,"ancestor_record_offset":19644,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":20312,"parent_record_offset":20288,"ancestor_record_offset":19644,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":19644,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":20192,"parent_record_offset":20076,"ancestor_record_offset":19644,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":20264,"parent_record_offset":20240,"ancestor_record_offset":19644,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":20240,"parent_record_offset":20216,"ancestor_record_offset":19644,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":20416,"parent_record_offset":20076,"ancestor_record_offset":19644,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1532,"record_offset":21140,"parent_record_offset":21056,"ancestor_record_offset":19644,"inlinee":6570,"source_line":1505,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":21108,"parent_record_offset":21084,"ancestor_record_offset":19644,"inlinee":6857,"source_line":716,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":21084,"parent_record_offset":21056,"ancestor_record_offset":19644,"inlinee":6856,"source_line":682,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":19908,"parent_record_offset":19876,"ancestor_record_offset":19644,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":19876,"parent_record_offset":19844,"ancestor_record_offset":19644,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":19940,"parent_record_offset":19908,"ancestor_record_offset":19644,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":20596,"parent_record_offset":19804,"ancestor_record_offset":19644,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_serde.rs
//     evidence={"module":1532,"record_offset":21500,"parent_record_offset":21476,"ancestor_record_offset":19644,"inlinee":29596,"source_line":18,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":20976,"parent_record_offset":20924,"ancestor_record_offset":19644,"inlinee":71685,"source_line":1665,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":20924,"parent_record_offset":20872,"ancestor_record_offset":19644,"inlinee":71684,"source_line":1826,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":20076,"parent_record_offset":20032,"ancestor_record_offset":19644,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":22704,"parent_record_offset":22680,"ancestor_record_offset":19644,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1532,"record_offset":22792,"parent_record_offset":22768,"ancestor_record_offset":19644,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1532,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":19644,"inlinee":6648,"source_line":126,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":19768,"parent_record_offset":19732,"ancestor_record_offset":19644,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":20716,"parent_record_offset":20664,"ancestor_record_offset":19644,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":20136,"parent_record_offset":20076,"ancestor_record_offset":19644,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":19988,"parent_record_offset":19804,"ancestor_record_offset":19644,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":20872,"parent_record_offset":20820,"ancestor_record_offset":19644,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":21056,"parent_record_offset":20976,"ancestor_record_offset":19644,"inlinee":6569,"source_line":227,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":21288,"parent_record_offset":20976,"ancestor_record_offset":19644,"inlinee":11516,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":21316,"parent_record_offset":20976,"ancestor_record_offset":19644,"inlinee":11521,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":21232,"parent_record_offset":20976,"ancestor_record_offset":19644,"inlinee":11519,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":22316,"parent_record_offset":22228,"ancestor_record_offset":19644,"inlinee":11517,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":21260,"parent_record_offset":20976,"ancestor_record_offset":19644,"inlinee":11520,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":21172,"parent_record_offset":20976,"ancestor_record_offset":19644,"inlinee":11518,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":21200,"parent_record_offset":20976,"ancestor_record_offset":19644,"inlinee":6568,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::MapDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":19844,"parent_record_offset":19804,"ancestor_record_offset":19644,"inlinee":6979,"source_line":1146,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":19804,"parent_record_offset":19768,"ancestor_record_offset":19644,"inlinee":7004,"source_line":1210,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":20768,"parent_record_offset":20716,"ancestor_record_offset":19644,"inlinee":7006,"source_line":1223,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1532,"record_offset":20160,"parent_record_offset":20136,"ancestor_record_offset":19644,"inlinee":71703,"source_line":114,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1532,"record_offset":20032,"parent_record_offset":19988,"ancestor_record_offset":19644,"inlinee":71702,"source_line":114,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11814
// 首条位置: {"module":1532,"inlinee":71700}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":23468,"parent_record_offset":23416,"ancestor_record_offset":23416,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1532,"record_offset":24276,"parent_record_offset":24192,"ancestor_record_offset":23416,"inlinee":6570,"source_line":1505,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":24244,"parent_record_offset":24220,"ancestor_record_offset":23416,"inlinee":6857,"source_line":716,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":24220,"parent_record_offset":24192,"ancestor_record_offset":23416,"inlinee":6856,"source_line":682,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":23576,"parent_record_offset":23552,"ancestor_record_offset":23416,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":23552,"parent_record_offset":23524,"ancestor_record_offset":23416,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":23416,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":23692,"parent_record_offset":23524,"ancestor_record_offset":23416,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_serde.rs
//     evidence={"module":1532,"record_offset":23660,"parent_record_offset":23636,"ancestor_record_offset":23416,"inlinee":29596,"source_line":18,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":24084,"parent_record_offset":24040,"ancestor_record_offset":23416,"inlinee":71685,"source_line":1665,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":24040,"parent_record_offset":23996,"ancestor_record_offset":23416,"inlinee":71684,"source_line":1826,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":23496,"parent_record_offset":23468,"ancestor_record_offset":23416,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":23636,"parent_record_offset":23524,"ancestor_record_offset":23416,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":23996,"parent_record_offset":23952,"ancestor_record_offset":23416,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":24192,"parent_record_offset":24084,"ancestor_record_offset":23416,"inlinee":6569,"source_line":227,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":24424,"parent_record_offset":24084,"ancestor_record_offset":23416,"inlinee":11521,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":24368,"parent_record_offset":24084,"ancestor_record_offset":23416,"inlinee":11519,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":24164,"parent_record_offset":24084,"ancestor_record_offset":23416,"inlinee":11517,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":24396,"parent_record_offset":24084,"ancestor_record_offset":23416,"inlinee":11520,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":24308,"parent_record_offset":24084,"ancestor_record_offset":23416,"inlinee":11518,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":24336,"parent_record_offset":24084,"ancestor_record_offset":23416,"inlinee":6568,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":23524,"parent_record_offset":23496,"ancestor_record_offset":23416,"inlinee":7008,"source_line":963,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":24228,"rva":982496,"va":5369691616}
// 编译实例 RVAs: 0xefde0, 0x157c4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1857967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369691616,"raw_hex":"e8af591c00","size":5,"soft_fail":false,"target_va":5371549808,"va":5369691836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bd170
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3986243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369691616,"raw_hex":"e843d33c00","size":5,"soft_fail":false,"target_va":5373677936,"va":5369691688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3989039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369691616,"raw_hex":"e82fde3c00","size":5,"soft_fail":false,"target_va":5373680768,"va":5369691724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f4520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2654337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238336,"raw_hex":"e87f7fd7ff","size":5,"soft_fail":false,"target_va":5388584224,"va":5391238556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238336,"raw_hex":"e85f9b0000","size":5,"soft_fail":false,"target_va":5391278224,"va":5391238444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158ce20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t67859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238336,"raw_hex":"e813090100","size":5,"soft_fail":false,"target_va":5391306272,"va":5391238408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41626354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369691616,"raw_hex":"e8f22a7b02","size":5,"soft_fail":false,"target_va":5411318240,"va":5369691881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":24496,"parent_record_offset":24228,"ancestor_record_offset":24228,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":25148,"parent_record_offset":24228,"ancestor_record_offset":24228,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":25036,"parent_record_offset":24228,"ancestor_record_offset":24228,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":24888,"parent_record_offset":24856,"ancestor_record_offset":24228,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":24756,"parent_record_offset":24728,"ancestor_record_offset":24228,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":24816,"parent_record_offset":24784,"ancestor_record_offset":24228,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":24936,"parent_record_offset":24912,"ancestor_record_offset":24228,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":24912,"parent_record_offset":24888,"ancestor_record_offset":24228,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":24784,"parent_record_offset":24756,"ancestor_record_offset":24228,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":24692,"parent_record_offset":24664,"ancestor_record_offset":24228,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":24856,"parent_record_offset":24692,"ancestor_record_offset":24228,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":24636,"parent_record_offset":24608,"ancestor_record_offset":24228,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":24976,"parent_record_offset":24692,"ancestor_record_offset":24228,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":24728,"parent_record_offset":24692,"ancestor_record_offset":24228,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":39,"record_offset":24664,"parent_record_offset":24636,"ancestor_record_offset":24228,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":24552,"parent_record_offset":24524,"ancestor_record_offset":24228,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":25204,"parent_record_offset":25176,"ancestor_record_offset":24228,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":24524,"parent_record_offset":24496,"ancestor_record_offset":24228,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":25176,"parent_record_offset":25148,"ancestor_record_offset":24228,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":39,"record_offset":24608,"parent_record_offset":24580,"ancestor_record_offset":24228,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":39,"record_offset":24580,"parent_record_offset":24552,"ancestor_record_offset":24228,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1623,"record_offset":15472,"rva":22529504,"va":5391238624}
// 编译实例 RVAs: 0x157c5e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16157369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238624,"raw_hex":"e8477509ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391238868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085db60
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13757482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238624,"raw_hex":"e8d6132eff","size":5,"soft_fail":false,"target_va":5377481568,"va":5391239045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13514620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238624,"raw_hex":"e884c831ff","size":5,"soft_fail":false,"target_va":5377724224,"va":5391238839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b864c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10445391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238624,"raw_hex":"e8b19d60ff","size":5,"soft_fail":false,"target_va":5380793536,"va":5391238922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141595a20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t103362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238624,"raw_hex":"e8c2930100","size":5,"soft_fail":false,"target_va":5391342112,"va":5391238745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141596670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t106549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391238624,"raw_hex":"e835a00100","size":5,"soft_fail":false,"target_va":5391345264,"va":5391238710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1623,"record_offset":15672,"parent_record_offset":15472,"ancestor_record_offset":15472,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1623,"record_offset":15644,"parent_record_offset":15472,"ancestor_record_offset":15472,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1623,"record_offset":15900,"parent_record_offset":15876,"ancestor_record_offset":15472,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1623,"record_offset":15876,"parent_record_offset":15820,"ancestor_record_offset":15472,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1623,"record_offset":15848,"parent_record_offset":15820,"ancestor_record_offset":15472,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1623,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":15472,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1623,"record_offset":15796,"parent_record_offset":15772,"ancestor_record_offset":15472,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1623,"record_offset":15700,"parent_record_offset":15672,"ancestor_record_offset":15472,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01623.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$10::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17E68
// 首条位置: {"module":2599,"inlinee":97896}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2599,"record_offset":22528,"parent_record_offset":22496,"ancestor_record_offset":22496,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$6::clone
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6413
// 首条位置: {"module":453,"inlinee":25619}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$7::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8067
// 首条位置: {"module":575,"record_offset":16864,"rva":11536448,"va":5380245568}
// 编译实例 RVAs: 0xb00840, 0x1559ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field4_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2647456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380245568,"raw_hex":"e8a0652800","size":5,"soft_fail":false,"target_va":5382893232,"va":5380245771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
// 记录形式: compiled_procedure; instances=1
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":3544,"rva":13207696,"va":5381916816}
// 编译实例 RVAs: 0xc98890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29625163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381916816,"raw_hex":"e84b0bc401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381916896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":3700,"parent_record_offset":3544,"ancestor_record_offset":3544,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":3752,"parent_record_offset":3544,"ancestor_record_offset":3544,"inlinee":44986,"source_line":141,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::new_axis_aligned
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFEB
// 首条位置: {"module":768,"inlinee":45035}
// 编译实例 RVAs: 0xc98900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29625033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381916928,"raw_hex":"e8c90ac401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381917026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":3932,"parent_record_offset":3784,"ancestor_record_offset":3784,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":3984,"parent_record_offset":3784,"ancestor_record_offset":3784,"inlinee":44986,"source_line":141,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":48460,"parent_record_offset":48436,"ancestor_record_offset":48368,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFBA
// 首条位置: {"module":768,"inlinee":44986}
// 编译实例 RVAs: 0xc98980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29624849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917056,"raw_hex":"e8110ac401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381917210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":424,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":480,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":44987,"source_line":525,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":564,"parent_record_offset":504,"ancestor_record_offset":380,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":536,"parent_record_offset":504,"ancestor_record_offset":380,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":380,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=25
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFC8
// 首条位置: {"module":768,"inlinee":45000}
// 编译实例 RVAs: 0xc98a50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29624711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917264,"raw_hex":"e88709c401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381917348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":4476,"parent_record_offset":4324,"ancestor_record_offset":4324,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=11
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFC7
// 首条位置: {"module":768,"inlinee":44999}
// 编译实例 RVAs: 0xc98ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29624481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917360,"raw_hex":"e8a108c401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381917578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3072,"parent_record_offset":2908,"ancestor_record_offset":2908,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":3380,"parent_record_offset":2908,"ancestor_record_offset":2908,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":4656,"parent_record_offset":4504,"ancestor_record_offset":4504,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":3032,"parent_record_offset":2908,"ancestor_record_offset":2908,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2948,"parent_record_offset":2908,"ancestor_record_offset":2908,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3332,"parent_record_offset":3124,"ancestor_record_offset":2908,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3248,"parent_record_offset":3124,"ancestor_record_offset":2908,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3204,"parent_record_offset":3124,"ancestor_record_offset":2908,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3288,"parent_record_offset":3124,"ancestor_record_offset":2908,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":3432,"parent_record_offset":3380,"ancestor_record_offset":2908,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3000,"parent_record_offset":2948,"ancestor_record_offset":2908,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3124,"parent_record_offset":3072,"ancestor_record_offset":2908,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2908,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points3
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFC3
// 首条位置: {"module":768,"inlinee":44995}
// 编译实例 RVAs: 0xc98b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29624141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917584,"raw_hex":"e84d07c401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381917918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":2812,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2784,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3332,"parent_record_offset":3124,"ancestor_record_offset":2748,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3248,"parent_record_offset":3124,"ancestor_record_offset":2748,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3204,"parent_record_offset":3124,"ancestor_record_offset":2748,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3288,"parent_record_offset":3124,"ancestor_record_offset":2748,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":2748,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2748,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":3432,"parent_record_offset":3380,"ancestor_record_offset":2748,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3000,"parent_record_offset":2948,"ancestor_record_offset":2748,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3124,"parent_record_offset":3072,"ancestor_record_offset":2748,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3072,"parent_record_offset":2908,"ancestor_record_offset":2748,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2748,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":3380,"parent_record_offset":2908,"ancestor_record_offset":2748,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":2748,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":3032,"parent_record_offset":2908,"ancestor_record_offset":2748,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2948,"parent_record_offset":2908,"ancestor_record_offset":2748,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$1::as_points3::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2748,"inlinee":44998,"source_line":202,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::as_points3::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFC6
// 首条位置: {"module":768,"inlinee":44998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":2884,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3332,"parent_record_offset":3124,"ancestor_record_offset":2884,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3248,"parent_record_offset":3124,"ancestor_record_offset":2884,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3204,"parent_record_offset":3124,"ancestor_record_offset":2884,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":3288,"parent_record_offset":3124,"ancestor_record_offset":2884,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":3432,"parent_record_offset":3380,"ancestor_record_offset":2884,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3000,"parent_record_offset":2948,"ancestor_record_offset":2884,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3124,"parent_record_offset":3072,"ancestor_record_offset":2884,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":3072,"parent_record_offset":2908,"ancestor_record_offset":2884,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2884,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":3380,"parent_record_offset":2908,"ancestor_record_offset":2884,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":3032,"parent_record_offset":2908,"ancestor_record_offset":2884,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":2948,"parent_record_offset":2908,"ancestor_record_offset":2884,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_curve2
// 记录形式: compiled_procedure; instances=1
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":5856,"rva":13208816,"va":5381917936}
// 编译实例 RVAs: 0xc98cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6836562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917936,"raw_hex":"e8aeae97ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381918045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6837072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917936,"raw_hex":"e8b0ac97ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381918571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419401b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13267777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917936,"raw_hex":"e84173ca00","size":5,"soft_fail":false,"target_va":5395186096,"va":5381918314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29622559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917936,"raw_hex":"e81f01c401","size":5,"soft_fail":false,"target_va":5411540992,"va":5381918428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7561162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917936,"raw_hex":"e836a08cff","size":5,"soft_fail":false,"target_va":5374357104,"va":5381918261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29623646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917936,"raw_hex":"e85e05c401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381918413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29624395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381917936,"raw_hex":"e84b08c401","size":5,"soft_fail":false,"target_va":5411542880,"va":5381918480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":6796,"parent_record_offset":5856,"ancestor_record_offset":5856,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":768,"record_offset":6432,"parent_record_offset":5856,"ancestor_record_offset":5856,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":7040,"parent_record_offset":5856,"ancestor_record_offset":5856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":768,"record_offset":6972,"parent_record_offset":5856,"ancestor_record_offset":5856,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":6012,"parent_record_offset":5856,"ancestor_record_offset":5856,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":6040,"parent_record_offset":5856,"ancestor_record_offset":5856,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":5856,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":5856,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":5856,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":6552,"parent_record_offset":6512,"ancestor_record_offset":5856,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":6584,"parent_record_offset":6552,"ancestor_record_offset":5856,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":7204,"parent_record_offset":7112,"ancestor_record_offset":5856,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":768,"record_offset":6512,"parent_record_offset":6472,"ancestor_record_offset":5856,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":5856,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":7140,"parent_record_offset":7112,"ancestor_record_offset":5856,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":5856,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":5856,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":5856,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":5856,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":768,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":5856,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":768,"record_offset":6704,"parent_record_offset":6472,"ancestor_record_offset":5856,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":6828,"parent_record_offset":6796,"ancestor_record_offset":5856,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":5856,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":6472,"parent_record_offset":6432,"ancestor_record_offset":5856,"inlinee":45002,"source_line":3923,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":6320,"parent_record_offset":6176,"ancestor_record_offset":5856,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":6252,"parent_record_offset":6176,"ancestor_record_offset":5856,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":6216,"parent_record_offset":6176,"ancestor_record_offset":5856,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":6288,"parent_record_offset":6176,"ancestor_record_offset":5856,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":768,"record_offset":7172,"parent_record_offset":7140,"ancestor_record_offset":5856,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":6388,"parent_record_offset":6356,"ancestor_record_offset":5856,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":5856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":5856,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":6940,"parent_record_offset":6796,"ancestor_record_offset":5856,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":768,"record_offset":7000,"parent_record_offset":6972,"ancestor_record_offset":5856,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":6176,"parent_record_offset":6148,"ancestor_record_offset":5856,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":6148,"parent_record_offset":6040,"ancestor_record_offset":5856,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":6356,"parent_record_offset":6040,"ancestor_record_offset":5856,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":6116,"parent_record_offset":6040,"ancestor_record_offset":5856,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":6088,"parent_record_offset":6040,"ancestor_record_offset":5856,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::try_as_curve2
// 记录形式: compiled_procedure; instances=1
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":7280,"rva":13209504,"va":5381918624}
// 编译实例 RVAs: 0xc98fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6837250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918624,"raw_hex":"e8feab97ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381918733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6837664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918624,"raw_hex":"e860aa97ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381919163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419401b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13267090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918624,"raw_hex":"e89270ca00","size":5,"soft_fail":false,"target_va":5395186096,"va":5381919001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29621911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918624,"raw_hex":"e897fec301","size":5,"soft_fail":false,"target_va":5411540992,"va":5381919076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7561850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918624,"raw_hex":"e8869d8cff","size":5,"soft_fail":false,"target_va":5374357104,"va":5381918949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29622998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918624,"raw_hex":"e8d602c401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381919061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":8220,"parent_record_offset":7280,"ancestor_record_offset":7280,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":768,"record_offset":7856,"parent_record_offset":7280,"ancestor_record_offset":7280,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":8396,"parent_record_offset":7280,"ancestor_record_offset":7280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":7436,"parent_record_offset":7280,"ancestor_record_offset":7280,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":7464,"parent_record_offset":7280,"ancestor_record_offset":7280,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":7280,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":7280,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":7280,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":7976,"parent_record_offset":7936,"ancestor_record_offset":7280,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":8008,"parent_record_offset":7976,"ancestor_record_offset":7280,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":8560,"parent_record_offset":8468,"ancestor_record_offset":7280,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":768,"record_offset":7936,"parent_record_offset":7896,"ancestor_record_offset":7280,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":8276,"parent_record_offset":8252,"ancestor_record_offset":7280,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":8496,"parent_record_offset":8468,"ancestor_record_offset":7280,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":7280,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":7280,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":7280,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":7280,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":768,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":7280,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":768,"record_offset":8128,"parent_record_offset":7896,"ancestor_record_offset":7280,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":8252,"parent_record_offset":8220,"ancestor_record_offset":7280,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":8176,"parent_record_offset":8152,"ancestor_record_offset":7280,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":7896,"parent_record_offset":7856,"ancestor_record_offset":7280,"inlinee":45002,"source_line":3923,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":7744,"parent_record_offset":7600,"ancestor_record_offset":7280,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":7676,"parent_record_offset":7600,"ancestor_record_offset":7280,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":7640,"parent_record_offset":7600,"ancestor_record_offset":7280,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":7712,"parent_record_offset":7600,"ancestor_record_offset":7280,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":768,"record_offset":8528,"parent_record_offset":8496,"ancestor_record_offset":7280,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":7812,"parent_record_offset":7780,"ancestor_record_offset":7280,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":7280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":7280,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":8364,"parent_record_offset":8220,"ancestor_record_offset":7280,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":7280,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":7572,"parent_record_offset":7464,"ancestor_record_offset":7280,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":7780,"parent_record_offset":7464,"ancestor_record_offset":7280,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":7540,"parent_record_offset":7464,"ancestor_record_offset":7280,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":7512,"parent_record_offset":7464,"ancestor_record_offset":7280,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_curve3
// 记录形式: compiled_procedure; instances=1
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":8636,"rva":13210096,"va":5381919216}
// 编译实例 RVAs: 0xc991f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6837852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919216,"raw_hex":"e8a4a997ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381919335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6838477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919216,"raw_hex":"e833a797ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381919976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141940470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13267085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919216,"raw_hex":"e88d70ca00","size":5,"soft_fail":false,"target_va":5395186800,"va":5381919710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29621158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919216,"raw_hex":"e8a6fbc301","size":5,"soft_fail":false,"target_va":5411540992,"va":5381919829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7562933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919216,"raw_hex":"e84b998cff","size":5,"soft_fail":false,"target_va":5374356720,"va":5381919648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29622245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919216,"raw_hex":"e8e5ffc301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381919814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29622994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919216,"raw_hex":"e8d202c401","size":5,"soft_fail":false,"target_va":5411542880,"va":5381919881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":9896,"parent_record_offset":8636,"ancestor_record_offset":8636,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":768,"record_offset":9556,"parent_record_offset":8636,"ancestor_record_offset":8636,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":10140,"parent_record_offset":8636,"ancestor_record_offset":8636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":768,"record_offset":10072,"parent_record_offset":8636,"ancestor_record_offset":8636,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":8792,"parent_record_offset":8636,"ancestor_record_offset":8636,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":8820,"parent_record_offset":8636,"ancestor_record_offset":8636,"inlinee":44995,"source_line":199,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":8636,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":8636,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":10328,"parent_record_offset":10304,"ancestor_record_offset":8636,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":9652,"parent_record_offset":9620,"ancestor_record_offset":8636,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":9684,"parent_record_offset":9652,"ancestor_record_offset":8636,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":10304,"parent_record_offset":10212,"ancestor_record_offset":8636,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":768,"record_offset":9620,"parent_record_offset":9588,"ancestor_record_offset":8636,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":8636,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":10240,"parent_record_offset":10212,"ancestor_record_offset":8636,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":10212,"parent_record_offset":10188,"ancestor_record_offset":8636,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":8636,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":8636,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":10188,"parent_record_offset":10164,"ancestor_record_offset":8636,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":768,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":8636,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":768,"record_offset":9804,"parent_record_offset":9588,"ancestor_record_offset":8636,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":9928,"parent_record_offset":9896,"ancestor_record_offset":8636,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":8636,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":9588,"parent_record_offset":9556,"ancestor_record_offset":8636,"inlinee":45002,"source_line":3923,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":9392,"parent_record_offset":9196,"ancestor_record_offset":8636,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":9308,"parent_record_offset":9196,"ancestor_record_offset":8636,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":9264,"parent_record_offset":9196,"ancestor_record_offset":8636,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":9348,"parent_record_offset":9196,"ancestor_record_offset":8636,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":8876,"parent_record_offset":8848,"ancestor_record_offset":8636,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":8848,"parent_record_offset":8820,"ancestor_record_offset":8636,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":8904,"parent_record_offset":8876,"ancestor_record_offset":8636,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":768,"record_offset":10272,"parent_record_offset":10240,"ancestor_record_offset":8636,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":9472,"parent_record_offset":9440,"ancestor_record_offset":8636,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":10164,"parent_record_offset":10140,"ancestor_record_offset":8636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":9756,"parent_record_offset":9732,"ancestor_record_offset":8636,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":10040,"parent_record_offset":9896,"ancestor_record_offset":8636,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":768,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":8636,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":9080,"parent_record_offset":9020,"ancestor_record_offset":8636,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":9196,"parent_record_offset":9156,"ancestor_record_offset":8636,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":9156,"parent_record_offset":8960,"ancestor_record_offset":8636,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":9052,"parent_record_offset":9020,"ancestor_record_offset":8636,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":9440,"parent_record_offset":8960,"ancestor_record_offset":8636,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":8960,"parent_record_offset":8932,"ancestor_record_offset":8636,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":9112,"parent_record_offset":8960,"ancestor_record_offset":8636,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":9020,"parent_record_offset":8960,"ancestor_record_offset":8636,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$1::as_points3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":8932,"parent_record_offset":8904,"ancestor_record_offset":8636,"inlinee":44998,"source_line":202,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::area
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=22
// 源码行: 237, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFC4
// 首条位置: {"module":768,"inlinee":44996}
// 编译实例 RVAs: 0xc99520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::perimeter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=20
// 源码行: 241, 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFCD
// 首条位置: {"module":768,"inlinee":45005}
// 编译实例 RVAs: 0xc99530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::orientation_mat2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=37
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFC0
// 首条位置: {"module":768,"inlinee":44992}
// 编译实例 RVAs: 0xc99540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":1880,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":19756,"parent_record_offset":19692,"ancestor_record_offset":19692,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":1848,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":19816,"parent_record_offset":19756,"ancestor_record_offset":19692,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":19784,"parent_record_offset":19756,"ancestor_record_offset":19692,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::orientation_mat3
// 记录形式: compiled_procedure; instances=1
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":10808,"rva":13210992,"va":5381920112}
// 编译实例 RVAs: 0xc99570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":768,"record_offset":11092,"parent_record_offset":10808,"ancestor_record_offset":10808,"inlinee":14186,"source_line":86,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":11008,"parent_record_offset":10808,"ancestor_record_offset":10808,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::swizzlers::impl$1::xvy
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/swizzlers.rs
//     evidence={"module":768,"record_offset":10960,"parent_record_offset":10808,"ancestor_record_offset":10808,"inlinee":45004,"source_line":18,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":768,"record_offset":10980,"parent_record_offset":10960,"ancestor_record_offset":10808,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_transformation_and_size
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFE2
// 首条位置: {"module":768,"inlinee":45026}
// 编译实例 RVAs: 0xc995c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":11276,"parent_record_offset":11124,"ancestor_record_offset":11124,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":11296,"parent_record_offset":11276,"ancestor_record_offset":11124,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":39128,"parent_record_offset":39100,"ancestor_record_offset":39072,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=17
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFCF
// 首条位置: {"module":768,"inlinee":45007}
// 编译实例 RVAs: 0xc99600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29621628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920256,"raw_hex":"e87cfdc301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381920431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":11624,"parent_record_offset":11328,"ancestor_record_offset":11328,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":11772,"parent_record_offset":11328,"ancestor_record_offset":11328,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":11476,"parent_record_offset":11328,"ancestor_record_offset":11328,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":11540,"parent_record_offset":11328,"ancestor_record_offset":11328,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":11736,"parent_record_offset":11648,"ancestor_record_offset":11328,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":11676,"parent_record_offset":11648,"ancestor_record_offset":11328,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":11704,"parent_record_offset":11648,"ancestor_record_offset":11328,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":11800,"parent_record_offset":11772,"ancestor_record_offset":11328,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":11592,"parent_record_offset":11540,"ancestor_record_offset":11328,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":11648,"parent_record_offset":11624,"ancestor_record_offset":11328,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":11328,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":11512,"parent_record_offset":11476,"ancestor_record_offset":11328,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_point_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=23
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFCE
// 首条位置: {"module":768,"inlinee":45006}
// 编译实例 RVAs: 0xc996d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29621416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920464,"raw_hex":"e8a8fcc301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381920643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29621168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920464,"raw_hex":"e8b0fbc301","size":5,"soft_fail":false,"target_va":5411541904,"va":5381920731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=22
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":12088,"parent_record_offset":11864,"ancestor_record_offset":11864,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":12004,"parent_record_offset":11864,"ancestor_record_offset":11864,"inlinee":45005,"source_line":241,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":12172,"parent_record_offset":12148,"ancestor_record_offset":11864,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":11864,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":12232,"parent_record_offset":12088,"ancestor_record_offset":11864,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":768,"record_offset":12260,"parent_record_offset":12088,"ancestor_record_offset":11864,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":12148,"parent_record_offset":12124,"ancestor_record_offset":11864,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":12124,"parent_record_offset":12088,"ancestor_record_offset":11864,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_normal_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 332
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":16924,"rva":13213200,"va":5381922320}
// 编译实例 RVAs: 0xc99e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t19","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381922320,"raw_hex":"e913000000","size":5,"soft_fail":false,"target_va":5381922352,"va":5381922328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners
// 记录形式: compiled_procedure; instances=1
// 源码行: 336
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":17056,"rva":13213216,"va":5381922336}
// 编译实例 RVAs: 0xc99e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381922336,"raw_hex":"e903000000","size":5,"soft_fail":false,"target_va":5381922352,"va":5381922344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_corner_unnormal
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 366
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFD1
// 首条位置: {"module":768,"inlinee":45009}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":19472,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18112,"parent_record_offset":18088,"ancestor_record_offset":18088,"inlinee":45010,"source_line":450,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18612,"parent_record_offset":18544,"ancestor_record_offset":18376,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":18088,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":21568,"parent_record_offset":21536,"ancestor_record_offset":21356,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18300,"parent_record_offset":18244,"ancestor_record_offset":18088,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":19600,"parent_record_offset":19472,"ancestor_record_offset":19292,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18188,"parent_record_offset":18136,"ancestor_record_offset":18088,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":18088,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18220,"parent_record_offset":18112,"ancestor_record_offset":18088,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":19532,"parent_record_offset":19500,"ancestor_record_offset":19292,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":19500,"parent_record_offset":19472,"ancestor_record_offset":19292,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":18088,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":19564,"parent_record_offset":19532,"ancestor_record_offset":19292,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":18088,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_corner_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFD0
// 首条位置: {"module":768,"inlinee":45008}
// 编译实例 RVAs: 0xc9a140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29618733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381923136,"raw_hex":"e82df2c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381923326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":19636,"parent_record_offset":19248,"ancestor_record_offset":19248,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18656,"parent_record_offset":18352,"ancestor_record_offset":18352,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":19664,"parent_record_offset":19248,"ancestor_record_offset":19248,"inlinee":45010,"source_line":450,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_corner_unnormal
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18088,"parent_record_offset":18064,"ancestor_record_offset":18064,"inlinee":45009,"source_line":366,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18612,"parent_record_offset":18544,"ancestor_record_offset":18352,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":18064,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":21568,"parent_record_offset":21536,"ancestor_record_offset":21208,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18300,"parent_record_offset":18244,"ancestor_record_offset":18064,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":19600,"parent_record_offset":19472,"ancestor_record_offset":19248,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18188,"parent_record_offset":18136,"ancestor_record_offset":18064,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":18064,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18220,"parent_record_offset":18112,"ancestor_record_offset":18064,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18728,"parent_record_offset":18704,"ancestor_record_offset":18352,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":18352,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18680,"parent_record_offset":18656,"ancestor_record_offset":18352,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":18064,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":19564,"parent_record_offset":19532,"ancestor_record_offset":19248,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18112,"parent_record_offset":18088,"ancestor_record_offset":18064,"inlinee":45010,"source_line":450,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":18064,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_pts
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFE9
// 首条位置: {"module":768,"inlinee":45033}
// 编译实例 RVAs: 0xc9a470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29617839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381923952,"raw_hex":"e8afeec301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381924220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":24176,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":24204,"parent_record_offset":24176,"ancestor_record_offset":24036,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 420
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFD3
// 首条位置: {"module":768,"inlinee":45011}
// 编译实例 RVAs: 0xc9a5a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29617680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381924256,"raw_hex":"e810eec301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381924379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":24412,"parent_record_offset":24268,"ancestor_record_offset":24268,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":19996,"parent_record_offset":19972,"ancestor_record_offset":19972,"inlinee":45010,"source_line":450,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":24460,"parent_record_offset":24436,"ancestor_record_offset":24268,"inlinee":9733,"source_line":1939,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":24480,"parent_record_offset":24460,"ancestor_record_offset":24268,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":24736,"parent_record_offset":24648,"ancestor_record_offset":24268,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":20156,"parent_record_offset":20128,"ancestor_record_offset":19972,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":20184,"parent_record_offset":20128,"ancestor_record_offset":19972,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":24436,"parent_record_offset":24412,"ancestor_record_offset":24268,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":20072,"parent_record_offset":20020,"ancestor_record_offset":19972,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":19972,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":20104,"parent_record_offset":19996,"ancestor_record_offset":19972,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":20044,"parent_record_offset":20020,"ancestor_record_offset":19972,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":20020,"parent_record_offset":19996,"ancestor_record_offset":19972,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFD2
// 首条位置: {"module":768,"inlinee":45010}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18220,"parent_record_offset":18112,"ancestor_record_offset":18112,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":18112,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18612,"parent_record_offset":18544,"ancestor_record_offset":18400,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":18112,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":21568,"parent_record_offset":21536,"ancestor_record_offset":21396,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":18300,"parent_record_offset":18244,"ancestor_record_offset":18112,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18188,"parent_record_offset":18136,"ancestor_record_offset":18112,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":18112,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":18112,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::contains_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 454
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFBE
// 首条位置: {"module":768,"inlinee":44990}
// 编译实例 RVAs: 0xc9a630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":1752,"inlinee":44991,"source_line":459,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2108,"parent_record_offset":2020,"ancestor_record_offset":1752,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1752,"inlinee":44994,"source_line":510,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2048,"parent_record_offset":2020,"ancestor_record_offset":1752,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2076,"parent_record_offset":2020,"ancestor_record_offset":1752,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1752,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/sse2/bvec4a.rs
//     evidence={"module":768,"record_offset":2200,"parent_record_offset":1776,"ancestor_record_offset":1752,"inlinee":25838,"source_line":91,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":1880,"parent_record_offset":1816,"ancestor_record_offset":1752,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1752,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":1996,"parent_record_offset":1776,"ancestor_record_offset":1752,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":768,"record_offset":2144,"parent_record_offset":1776,"ancestor_record_offset":1752,"inlinee":44993,"source_line":364,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":768,"record_offset":1912,"parent_record_offset":1776,"ancestor_record_offset":1752,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":1848,"parent_record_offset":1816,"ancestor_record_offset":1752,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":1944,"parent_record_offset":1776,"ancestor_record_offset":1752,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":1816,"parent_record_offset":1776,"ancestor_record_offset":1752,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::contains_point_padding
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 459
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFBF
// 首条位置: {"module":768,"inlinee":44991}
// 编译实例 RVAs: 0xc9a6a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/sse2/bvec4a.rs
//     evidence={"module":768,"record_offset":2200,"parent_record_offset":1776,"ancestor_record_offset":1776,"inlinee":25838,"source_line":91,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":1996,"parent_record_offset":1776,"ancestor_record_offset":1776,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":768,"record_offset":2144,"parent_record_offset":1776,"ancestor_record_offset":1776,"inlinee":44993,"source_line":364,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":768,"record_offset":1912,"parent_record_offset":1776,"ancestor_record_offset":1776,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":1944,"parent_record_offset":1776,"ancestor_record_offset":1776,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":1816,"parent_record_offset":1776,"ancestor_record_offset":1776,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2108,"parent_record_offset":2020,"ancestor_record_offset":1776,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1776,"inlinee":44994,"source_line":510,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2048,"parent_record_offset":2020,"ancestor_record_offset":1776,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":2076,"parent_record_offset":2020,"ancestor_record_offset":1776,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1776,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":1880,"parent_record_offset":1816,"ancestor_record_offset":1776,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1776,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":1848,"parent_record_offset":1816,"ancestor_record_offset":1776,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::with_expanded_size
// 记录形式: compiled_procedure; instances=1
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":25972,"rva":13215536,"va":5381924656}
// 编译实例 RVAs: 0xc9a730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge
// 记录形式: compiled_procedure; instances=1
// 源码行: 481
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":26108,"rva":13215568,"va":5381924688}
// 编译实例 RVAs: 0xc9a750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29922541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381924688,"raw_hex":"e8ed94c801","size":5,"soft_fail":false,"target_va":5411847408,"va":5381924862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":26840,"parent_record_offset":26108,"ancestor_record_offset":26108,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":26264,"parent_record_offset":26108,"ancestor_record_offset":26108,"inlinee":45012,"source_line":485,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":26748,"parent_record_offset":26660,"ancestor_record_offset":26108,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":26516,"parent_record_offset":26392,"ancestor_record_offset":26108,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":26456,"parent_record_offset":26392,"ancestor_record_offset":26108,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":26424,"parent_record_offset":26392,"ancestor_record_offset":26108,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":26780,"parent_record_offset":26264,"ancestor_record_offset":26108,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":26632,"parent_record_offset":26608,"ancestor_record_offset":26108,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":26360,"parent_record_offset":26308,"ancestor_record_offset":26108,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":26392,"parent_record_offset":26264,"ancestor_record_offset":26108,"inlinee":45013,"source_line":300,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":26660,"parent_record_offset":26264,"ancestor_record_offset":26108,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":26332,"parent_record_offset":26308,"ancestor_record_offset":26108,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":26608,"parent_record_offset":26264,"ancestor_record_offset":26108,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":26808,"parent_record_offset":26264,"ancestor_record_offset":26108,"inlinee":10999,"source_line":627,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":26308,"parent_record_offset":26264,"ancestor_record_offset":26108,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 485
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFD4
// 首条位置: {"module":768,"inlinee":45012}
// 编译实例 RVAs: 0xc9a820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x142923cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t29922319","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381924896,"raw_hex":"e90f94c801","size":5,"soft_fail":false,"target_va":5411847408,"va":5381925084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":26780,"parent_record_offset":26264,"ancestor_record_offset":26264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":26392,"parent_record_offset":26264,"ancestor_record_offset":26264,"inlinee":45013,"source_line":300,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":26660,"parent_record_offset":26264,"ancestor_record_offset":26264,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":26608,"parent_record_offset":26264,"ancestor_record_offset":26264,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":26808,"parent_record_offset":26264,"ancestor_record_offset":26264,"inlinee":10999,"source_line":627,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":26308,"parent_record_offset":26264,"ancestor_record_offset":26264,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":26748,"parent_record_offset":26660,"ancestor_record_offset":26264,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":26516,"parent_record_offset":26392,"ancestor_record_offset":26264,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":26456,"parent_record_offset":26392,"ancestor_record_offset":26264,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":26424,"parent_record_offset":26392,"ancestor_record_offset":26264,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":26632,"parent_record_offset":26608,"ancestor_record_offset":26264,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":26360,"parent_record_offset":26308,"ancestor_record_offset":26264,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":26332,"parent_record_offset":26308,"ancestor_record_offset":26264,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::closest_edge_normal
// 记录形式: compiled_procedure; instances=1
// 源码行: 499
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":27580,"rva":13215984,"va":5381925104}
// 编译实例 RVAs: 0xc9a8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":28216,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":27824,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":45013,"source_line":300,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":28092,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":28040,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":27740,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":28184,"parent_record_offset":28092,"ancestor_record_offset":27580,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":27948,"parent_record_offset":27824,"ancestor_record_offset":27580,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":27888,"parent_record_offset":27824,"ancestor_record_offset":27580,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":27856,"parent_record_offset":27824,"ancestor_record_offset":27580,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":28064,"parent_record_offset":28040,"ancestor_record_offset":27580,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":27792,"parent_record_offset":27740,"ancestor_record_offset":27580,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":27764,"parent_record_offset":27740,"ancestor_record_offset":27580,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
// 记录形式: compiled_procedure; instances=1
// 源码行: 518
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":28372,"rva":13216208,"va":5381925328}
// 编译实例 RVAs: 0xc9a9d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29616011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381925328,"raw_hex":"e88be7c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381926048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":768,"record_offset":28532,"parent_record_offset":28372,"ancestor_record_offset":28372,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::distance_segment_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":29536,"parent_record_offset":28372,"ancestor_record_offset":28372,"inlinee":29744,"source_line":169,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":28980,"parent_record_offset":28372,"ancestor_record_offset":28372,"inlinee":45007,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":28656,"parent_record_offset":28596,"ancestor_record_offset":28372,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":29172,"parent_record_offset":29084,"ancestor_record_offset":28372,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":29112,"parent_record_offset":29084,"ancestor_record_offset":28372,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":29140,"parent_record_offset":29084,"ancestor_record_offset":28372,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":29880,"parent_record_offset":29564,"ancestor_record_offset":28372,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":29848,"parent_record_offset":29592,"ancestor_record_offset":28372,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":768,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":28372,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":768,"record_offset":28820,"parent_record_offset":28796,"ancestor_record_offset":28372,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":768,"record_offset":28596,"parent_record_offset":28572,"ancestor_record_offset":28372,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":768,"record_offset":28572,"parent_record_offset":28532,"ancestor_record_offset":28372,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":768,"record_offset":28684,"parent_record_offset":28596,"ancestor_record_offset":28372,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":28844,"parent_record_offset":28820,"ancestor_record_offset":28372,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":768,"record_offset":28708,"parent_record_offset":28684,"ancestor_record_offset":28372,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":29232,"parent_record_offset":29208,"ancestor_record_offset":28372,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":29984,"parent_record_offset":29960,"ancestor_record_offset":28372,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":29648,"parent_record_offset":29624,"ancestor_record_offset":28372,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":28628,"parent_record_offset":28596,"ancestor_record_offset":28372,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":768,"record_offset":28796,"parent_record_offset":28772,"ancestor_record_offset":28372,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":29084,"parent_record_offset":29060,"ancestor_record_offset":28372,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":29060,"parent_record_offset":28980,"ancestor_record_offset":28372,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":28372,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":30192,"parent_record_offset":30168,"ancestor_record_offset":28372,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29936,"parent_record_offset":29912,"ancestor_record_offset":28372,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29960,"parent_record_offset":29936,"ancestor_record_offset":28372,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29208,"parent_record_offset":28980,"ancestor_record_offset":28372,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":29624,"parent_record_offset":29592,"ancestor_record_offset":28372,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":768,"record_offset":28748,"parent_record_offset":28532,"ancestor_record_offset":28372,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":30248,"parent_record_offset":30216,"ancestor_record_offset":28372,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":29024,"parent_record_offset":28980,"ancestor_record_offset":28372,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":29736,"parent_record_offset":29592,"ancestor_record_offset":28372,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":30168,"parent_record_offset":29536,"ancestor_record_offset":28372,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":29912,"parent_record_offset":29564,"ancestor_record_offset":28372,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":29564,"parent_record_offset":29536,"ancestor_record_offset":28372,"inlinee":24440,"source_line":184,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":29592,"parent_record_offset":29564,"ancestor_record_offset":28372,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 537
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":30296,"rva":13216944,"va":5381926064}
// 编译实例 RVAs: 0xc9acb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-741","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381926064,"raw_hex":"e91bfdffff","size":5,"soft_fail":false,"target_va":5381925328,"va":5381926064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
// 记录形式: compiled_procedure; instances=1
// 源码行: 556
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":31596,"rva":13217312,"va":5381926432}
// 编译实例 RVAs: 0xc9ae20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29615158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381926432,"raw_hex":"e836e4c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381926901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381926432,"raw_hex":"e88dfbffff","size":5,"soft_fail":false,"target_va":5381925328,"va":5381926462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":33056,"parent_record_offset":31596,"ancestor_record_offset":31596,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":33400,"parent_record_offset":31596,"ancestor_record_offset":31596,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":31760,"parent_record_offset":31596,"ancestor_record_offset":31596,"inlinee":45007,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":32768,"parent_record_offset":31596,"ancestor_record_offset":31596,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":32112,"parent_record_offset":32020,"ancestor_record_offset":31596,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":32052,"parent_record_offset":32020,"ancestor_record_offset":31596,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":32080,"parent_record_offset":32020,"ancestor_record_offset":31596,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":33024,"parent_record_offset":32768,"ancestor_record_offset":31596,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":32172,"parent_record_offset":32148,"ancestor_record_offset":31596,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":33456,"parent_record_offset":33428,"ancestor_record_offset":31596,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":32824,"parent_record_offset":32800,"ancestor_record_offset":31596,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":31960,"parent_record_offset":31908,"ancestor_record_offset":31596,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":32020,"parent_record_offset":31992,"ancestor_record_offset":31596,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":31992,"parent_record_offset":31760,"ancestor_record_offset":31596,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":32936,"parent_record_offset":32912,"ancestor_record_offset":31596,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":31596,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":33428,"parent_record_offset":33400,"ancestor_record_offset":31596,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":32148,"parent_record_offset":31760,"ancestor_record_offset":31596,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":32800,"parent_record_offset":32768,"ancestor_record_offset":31596,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":31876,"parent_record_offset":31824,"ancestor_record_offset":31596,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":31824,"parent_record_offset":31760,"ancestor_record_offset":31596,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":31908,"parent_record_offset":31760,"ancestor_record_offset":31596,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":768,"record_offset":32912,"parent_record_offset":32768,"ancestor_record_offset":31596,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::local_aabb_of
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 582
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFD7
// 首条位置: {"module":768,"inlinee":45015}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":35660,"parent_record_offset":35516,"ancestor_record_offset":35516,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35840,"parent_record_offset":35516,"ancestor_record_offset":35516,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":36056,"parent_record_offset":35516,"ancestor_record_offset":35516,"inlinee":45016,"source_line":643,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":35572,"parent_record_offset":35516,"ancestor_record_offset":35516,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":35804,"parent_record_offset":35684,"ancestor_record_offset":35516,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":35744,"parent_record_offset":35684,"ancestor_record_offset":35516,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":35716,"parent_record_offset":35684,"ancestor_record_offset":35516,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":35772,"parent_record_offset":35684,"ancestor_record_offset":35516,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":36200,"parent_record_offset":36056,"ancestor_record_offset":35516,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":37260,"parent_record_offset":37236,"ancestor_record_offset":36548,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":36160,"parent_record_offset":36132,"ancestor_record_offset":35516,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":35864,"parent_record_offset":35840,"ancestor_record_offset":35516,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":35628,"parent_record_offset":35572,"ancestor_record_offset":35516,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":35684,"parent_record_offset":35660,"ancestor_record_offset":35516,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35600,"parent_record_offset":35572,"ancestor_record_offset":35516,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":37236,"parent_record_offset":37072,"ancestor_record_offset":36548,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":36228,"parent_record_offset":36056,"ancestor_record_offset":35516,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":36132,"parent_record_offset":36104,"ancestor_record_offset":35516,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":36104,"parent_record_offset":36056,"ancestor_record_offset":35516,"inlinee":45017,"source_line":638,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::half_dimensions
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 638
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFD9
// 首条位置: {"module":768,"inlinee":45017}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":36132,"parent_record_offset":36104,"ancestor_record_offset":36104,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":36160,"parent_record_offset":36132,"ancestor_record_offset":36104,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::aabb
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 643
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFD8
// 首条位置: {"module":768,"inlinee":45016}
// 编译实例 RVAs: 0xc9b680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":36200,"parent_record_offset":36056,"ancestor_record_offset":36056,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":37236,"parent_record_offset":37072,"ancestor_record_offset":37072,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":36228,"parent_record_offset":36056,"ancestor_record_offset":36056,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":36104,"parent_record_offset":36056,"ancestor_record_offset":36056,"inlinee":45017,"source_line":638,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":37260,"parent_record_offset":37236,"ancestor_record_offset":37072,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":36160,"parent_record_offset":36132,"ancestor_record_offset":36056,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":36260,"parent_record_offset":36228,"ancestor_record_offset":36056,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":36132,"parent_record_offset":36104,"ancestor_record_offset":36056,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::intersects_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 659
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":37692,"rva":13219552,"va":5381928672}
// 编译实例 RVAs: 0xc9b6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141942600
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13266396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381928672,"raw_hex":"e8dc6dca00","size":5,"soft_fail":false,"target_va":5395195392,"va":5381928991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29612716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381928672,"raw_hex":"e8acdac301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381929343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":768,"record_offset":38600,"parent_record_offset":37692,"ancestor_record_offset":37692,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":38444,"parent_record_offset":37692,"ancestor_record_offset":37692,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":768,"record_offset":38296,"parent_record_offset":37692,"ancestor_record_offset":37692,"inlinee":4884,"source_line":813,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::intersect_ray
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":768,"record_offset":38500,"parent_record_offset":37692,"ancestor_record_offset":37692,"inlinee":44965,"source_line":150,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":37852,"parent_record_offset":37692,"ancestor_record_offset":37692,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":38708,"parent_record_offset":38680,"ancestor_record_offset":37692,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":38732,"parent_record_offset":38708,"ancestor_record_offset":37692,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":38180,"parent_record_offset":38096,"ancestor_record_offset":37692,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":38140,"parent_record_offset":38096,"ancestor_record_offset":37692,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::Cycle::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cycle.rs
//     evidence={"module":768,"record_offset":38396,"parent_record_offset":38364,"ancestor_record_offset":37692,"inlinee":4888,"source_line":21,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::cycle
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":38364,"parent_record_offset":38328,"ancestor_record_offset":37692,"inlinee":4886,"source_line":3442,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":38256,"parent_record_offset":38228,"ancestor_record_offset":37692,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":38468,"parent_record_offset":38444,"ancestor_record_offset":37692,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":38680,"parent_record_offset":38656,"ancestor_record_offset":37692,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":38656,"parent_record_offset":38628,"ancestor_record_offset":37692,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":37992,"parent_record_offset":37940,"ancestor_record_offset":37692,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":38096,"parent_record_offset":38064,"ancestor_record_offset":37692,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":38064,"parent_record_offset":37852,"ancestor_record_offset":37692,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":38540,"parent_record_offset":38500,"ancestor_record_offset":37692,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":37964,"parent_record_offset":37940,"ancestor_record_offset":37692,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":38228,"parent_record_offset":37852,"ancestor_record_offset":37692,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":768,"record_offset":38328,"parent_record_offset":38296,"ancestor_record_offset":37692,"inlinee":4885,"source_line":256,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":38568,"parent_record_offset":38500,"ancestor_record_offset":37692,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":37912,"parent_record_offset":37852,"ancestor_record_offset":37692,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":38024,"parent_record_offset":37852,"ancestor_record_offset":37692,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":37940,"parent_record_offset":37852,"ancestor_record_offset":37692,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$1::intersects_circle::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":38628,"parent_record_offset":38600,"ancestor_record_offset":37692,"inlinee":45025,"source_line":664,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::intersects_circle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 664
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFE1
// 首条位置: {"module":768,"inlinee":45025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":38656,"parent_record_offset":38628,"ancestor_record_offset":38628,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":38708,"parent_record_offset":38680,"ancestor_record_offset":38628,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":38732,"parent_record_offset":38708,"ancestor_record_offset":38628,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":38680,"parent_record_offset":38656,"ancestor_record_offset":38628,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::intersects
// 记录形式: compiled_procedure; instances=1
// 源码行: 674
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":38900,"rva":13220240,"va":5381929360}
// 编译实例 RVAs: 0xc9b990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1419423f0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13265008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381929360,"raw_hex":"e87068ca00","size":5,"soft_fail":false,"target_va":5395194864,"va":5381929851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29611714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381929360,"raw_hex":"e8c2d6c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381930345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":41144,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine2::Affine2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine2.rs
//     evidence={"module":768,"record_offset":39400,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":45027,"source_line":306,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine2::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine2.rs
//     evidence={"module":768,"record_offset":39800,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":45029,"source_line":387,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40496,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":768,"record_offset":40328,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4884,"source_line":813,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":768,"record_offset":39192,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":45014,"source_line":73,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::intersect_ray
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":768,"record_offset":40552,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":45020,"source_line":148,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::RayBoxIntersection::is_hit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":768,"record_offset":41116,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":45023,"source_line":394,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":40260,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":45000,"source_line":154,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_transformation_and_size
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":39072,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":45026,"source_line":258,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":39760,"parent_record_offset":39672,"ancestor_record_offset":38900,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":39556,"parent_record_offset":39428,"ancestor_record_offset":38900,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":39496,"parent_record_offset":39428,"ancestor_record_offset":38900,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":768,"record_offset":39464,"parent_record_offset":39428,"ancestor_record_offset":38900,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":40940,"parent_record_offset":40908,"ancestor_record_offset":38900,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":40848,"parent_record_offset":40820,"ancestor_record_offset":38900,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::Cycle::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cycle.rs
//     evidence={"module":768,"record_offset":40412,"parent_record_offset":40380,"ancestor_record_offset":38900,"inlinee":4888,"source_line":21,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::cycle
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":768,"record_offset":40380,"parent_record_offset":40352,"ancestor_record_offset":38900,"inlinee":4886,"source_line":3442,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":40224,"parent_record_offset":40200,"ancestor_record_offset":38900,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":40676,"parent_record_offset":40652,"ancestor_record_offset":38900,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":40520,"parent_record_offset":40496,"ancestor_record_offset":38900,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":41168,"parent_record_offset":41144,"ancestor_record_offset":38900,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":39156,"parent_record_offset":39100,"ancestor_record_offset":38900,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":39428,"parent_record_offset":39400,"ancestor_record_offset":38900,"inlinee":45013,"source_line":300,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_mat2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":39856,"parent_record_offset":39832,"ancestor_record_offset":38900,"inlinee":45032,"source_line":336,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":39672,"parent_record_offset":39648,"ancestor_record_offset":38900,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":39832,"parent_record_offset":39800,"ancestor_record_offset":38900,"inlinee":45031,"source_line":457,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":39648,"parent_record_offset":39400,"ancestor_record_offset":38900,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":39244,"parent_record_offset":39220,"ancestor_record_offset":38900,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":39220,"parent_record_offset":39192,"ancestor_record_offset":38900,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40908,"parent_record_offset":40552,"ancestor_record_offset":38900,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":41000,"parent_record_offset":40552,"ancestor_record_offset":38900,"inlinee":30569,"source_line":229,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40820,"parent_record_offset":40552,"ancestor_record_offset":38900,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":41056,"parent_record_offset":40552,"ancestor_record_offset":38900,"inlinee":45021,"source_line":220,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":39128,"parent_record_offset":39100,"ancestor_record_offset":38900,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40652,"parent_record_offset":40552,"ancestor_record_offset":38900,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40200,"parent_record_offset":39800,"ancestor_record_offset":38900,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40596,"parent_record_offset":40552,"ancestor_record_offset":38900,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":768,"record_offset":40352,"parent_record_offset":40328,"ancestor_record_offset":38900,"inlinee":4885,"source_line":256,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":768,"record_offset":40452,"parent_record_offset":40352,"ancestor_record_offset":38900,"inlinee":4534,"source_line":176,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":39272,"parent_record_offset":39244,"ancestor_record_offset":38900,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":40296,"parent_record_offset":40260,"ancestor_record_offset":38900,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":39100,"parent_record_offset":39072,"ancestor_record_offset":38900,"inlinee":44992,"source_line":245,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::intersects::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 684
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16164
// 首条位置: {"module":2085,"inlinee":90468}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine2.rs
//     evidence={"module":2085,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":1064,"inlinee":25731,"source_line":255,"archive":"_pdb/B/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2085,"record_offset":2172,"parent_record_offset":2084,"ancestor_record_offset":2012,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2085,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":1064,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2085,"record_offset":2140,"parent_record_offset":2084,"ancestor_record_offset":2012,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2085,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":2012,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":2085,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":1064,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":2085,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":1064,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2085,"record_offset":2208,"parent_record_offset":2036,"ancestor_record_offset":2012,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/02085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::circular_slice
// 记录形式: compiled_procedure; instances=1
// 源码行: 705
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":41204,"rva":13221264,"va":5381930384}
// 编译实例 RVAs: 0xc9bd90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6848989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381930384,"raw_hex":"e8237e97ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381930472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6850853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381930384,"raw_hex":"e8db7697ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381932352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29608723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381930384,"raw_hex":"e813cbc301","size":5,"soft_fail":false,"target_va":5411540992,"va":5381932264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t-1296060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381930384,"raw_hex":"e84439ecff","size":5,"soft_fail":false,"target_va":5380634736,"va":5381930791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t29610506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381930384,"raw_hex":"e80ad2c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381931553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t29610379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381930384,"raw_hex":"e88bd1c301","size":5,"soft_fail":false,"target_va":5411541904,"va":5381931520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":41368,"parent_record_offset":41204,"ancestor_record_offset":41204,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":768,"record_offset":41788,"parent_record_offset":41204,"ancestor_record_offset":41204,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":42648,"parent_record_offset":41204,"ancestor_record_offset":41204,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":768,"record_offset":41516,"parent_record_offset":41204,"ancestor_record_offset":41204,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":45068,"parent_record_offset":41204,"ancestor_record_offset":41204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":43544,"parent_record_offset":41204,"ancestor_record_offset":41204,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":41872,"parent_record_offset":41204,"ancestor_record_offset":41204,"inlinee":45006,"source_line":283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":41424,"parent_record_offset":41400,"ancestor_record_offset":41204,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":41448,"parent_record_offset":41424,"ancestor_record_offset":41204,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":45140,"parent_record_offset":45116,"ancestor_record_offset":41204,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":41400,"parent_record_offset":41368,"ancestor_record_offset":41204,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":768,"record_offset":45116,"parent_record_offset":45092,"ancestor_record_offset":41204,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":768,"record_offset":45092,"parent_record_offset":45068,"ancestor_record_offset":41204,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":42704,"parent_record_offset":42680,"ancestor_record_offset":41204,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":45280,"parent_record_offset":45252,"ancestor_record_offset":41204,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":41204,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":42752,"parent_record_offset":42728,"ancestor_record_offset":41204,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":42728,"parent_record_offset":42704,"ancestor_record_offset":41204,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":768,"record_offset":45228,"parent_record_offset":45204,"ancestor_record_offset":41204,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":768,"record_offset":41812,"parent_record_offset":41788,"ancestor_record_offset":41204,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":42680,"parent_record_offset":42648,"ancestor_record_offset":41204,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":768,"record_offset":41836,"parent_record_offset":41812,"ancestor_record_offset":41204,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":41660,"parent_record_offset":41636,"ancestor_record_offset":41204,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":768,"record_offset":41684,"parent_record_offset":41660,"ancestor_record_offset":41204,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":41720,"parent_record_offset":41572,"ancestor_record_offset":41204,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":768,"record_offset":41748,"parent_record_offset":41572,"ancestor_record_offset":41204,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":768,"record_offset":45312,"parent_record_offset":45280,"ancestor_record_offset":41204,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":41636,"parent_record_offset":41612,"ancestor_record_offset":41204,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":768,"record_offset":41612,"parent_record_offset":41572,"ancestor_record_offset":41204,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":45204,"parent_record_offset":45180,"ancestor_record_offset":41204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":41472,"parent_record_offset":41448,"ancestor_record_offset":41204,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":768,"record_offset":42792,"parent_record_offset":42648,"ancestor_record_offset":41204,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":768,"record_offset":41572,"parent_record_offset":41544,"ancestor_record_offset":41204,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":768,"record_offset":41544,"parent_record_offset":41516,"ancestor_record_offset":41204,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":41904,"parent_record_offset":41872,"ancestor_record_offset":41204,"inlinee":45005,"source_line":241,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::lerp
// 记录形式: compiled_procedure; instances=1
// 源码行: 1072
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":46328,"rva":13223728,"va":5381932848}
// 编译实例 RVAs: 0xc9c730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29914487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381932848,"raw_hex":"e87775c801","size":5,"soft_fail":false,"target_va":5411847808,"va":5381933316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29914835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381932848,"raw_hex":"e8d376c801","size":5,"soft_fail":false,"target_va":5411848144,"va":5381933304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29608467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381932848,"raw_hex":"e813cac301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381933592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":768,"record_offset":47124,"parent_record_offset":46328,"ancestor_record_offset":46328,"inlinee":32466,"source_line":123,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::angle_to
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":46476,"parent_record_offset":46328,"ancestor_record_offset":46328,"inlinee":32462,"source_line":880,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":47612,"parent_record_offset":46328,"ancestor_record_offset":46328,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":768,"record_offset":46984,"parent_record_offset":46328,"ancestor_record_offset":46328,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":47352,"parent_record_offset":46328,"ancestor_record_offset":46328,"inlinee":44986,"source_line":141,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":46884,"parent_record_offset":46860,"ancestor_record_offset":46328,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":47404,"parent_record_offset":47352,"ancestor_record_offset":46328,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":46580,"parent_record_offset":46532,"ancestor_record_offset":46328,"inlinee":4989,"source_line":1471,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":47092,"parent_record_offset":46984,"ancestor_record_offset":46328,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":47036,"parent_record_offset":46984,"ancestor_record_offset":46328,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":47008,"parent_record_offset":46984,"ancestor_record_offset":46328,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::acos_approx_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":768,"record_offset":46784,"parent_record_offset":46756,"ancestor_record_offset":46328,"inlinee":32465,"source_line":3,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":768,"record_offset":46860,"parent_record_offset":46784,"ancestor_record_offset":46328,"inlinee":13667,"source_line":146,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::acos_approx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":768,"record_offset":46756,"parent_record_offset":46476,"ancestor_record_offset":46328,"inlinee":32463,"source_line":151,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":768,"record_offset":47148,"parent_record_offset":47124,"ancestor_record_offset":46328,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":768,"record_offset":46916,"parent_record_offset":46784,"ancestor_record_offset":46328,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":46668,"parent_record_offset":46616,"ancestor_record_offset":46328,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":47488,"parent_record_offset":47352,"ancestor_record_offset":46328,"inlinee":44987,"source_line":525,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":46616,"parent_record_offset":46476,"ancestor_record_offset":46328,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp_dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":46532,"parent_record_offset":46476,"ancestor_record_offset":46328,"inlinee":11288,"source_line":905,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":47636,"parent_record_offset":47612,"ancestor_record_offset":46328,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":47720,"parent_record_offset":47612,"ancestor_record_offset":46328,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":47196,"parent_record_offset":47172,"ancestor_record_offset":46328,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":47172,"parent_record_offset":47148,"ancestor_record_offset":46328,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":46724,"parent_record_offset":46616,"ancestor_record_offset":46328,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::edge_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 1084
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":47784,"rva":13224528,"va":5381933648}
// 编译实例 RVAs: 0xc9ca50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29608121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933648,"raw_hex":"e8b9c8c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381933938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":48028,"parent_record_offset":47784,"ancestor_record_offset":47784,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_pts
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":47924,"parent_record_offset":47784,"ancestor_record_offset":47784,"inlinee":45033,"source_line":409,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":48080,"parent_record_offset":48052,"ancestor_record_offset":47784,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":48108,"parent_record_offset":48052,"ancestor_record_offset":47784,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":48052,"parent_record_offset":48028,"ancestor_record_offset":47784,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":48140,"parent_record_offset":48108,"ancestor_record_offset":47784,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":47992,"parent_record_offset":47964,"ancestor_record_offset":47784,"inlinee":44996,"source_line":237,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":47964,"parent_record_offset":47924,"ancestor_record_offset":47784,"inlinee":44999,"source_line":183,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$2::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 1097
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":768,"record_offset":48212,"rva":13224848,"va":5381933968}
// 编译实例 RVAs: 0xc9cb90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29607980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933968,"raw_hex":"e82cc8c301","size":5,"soft_fail":false,"target_va":5411542064,"va":5381934079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":768,"record_offset":48528,"parent_record_offset":48212,"ancestor_record_offset":48212,"inlinee":45036,"source_line":101,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":768,"record_offset":48344,"parent_record_offset":48212,"ancestor_record_offset":48212,"inlinee":45034,"source_line":174,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new_axis_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":48368,"parent_record_offset":48212,"ancestor_record_offset":48212,"inlinee":45035,"source_line":134,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":48460,"parent_record_offset":48436,"ancestor_record_offset":48212,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":48408,"parent_record_offset":48368,"ancestor_record_offset":48212,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":48576,"parent_record_offset":48552,"ancestor_record_offset":48212,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":48628,"parent_record_offset":48604,"ancestor_record_offset":48212,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":48604,"parent_record_offset":48528,"ancestor_record_offset":48212,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":48552,"parent_record_offset":48528,"ancestor_record_offset":48212,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":768,"record_offset":48492,"parent_record_offset":48368,"ancestor_record_offset":48212,"inlinee":44986,"source_line":141,"archive":"_pdb/B/modules/00768.jsonl.gz"}
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
// 完整结构及原始类型记录: rectangle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::rectangle::Rectangle2d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 52; 过程引用数: 47
// [PDB 泛型关系，非字段关系] {"type": "utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1>", "constructor": "utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points", "arguments": ["system_decorator::decorator_visual::stairs::walk_shape::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<utils::geometry::rectangle::Rectangle2d>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["utils::geometry::rectangle::Rectangle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::rectangle::Rectangle2d>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["utils::geometry::rectangle::Rectangle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::rectangle::Rectangle3d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 15; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<utils::geometry::rectangle::Rectangle3d,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["utils::geometry::rectangle::Rectangle3d", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::shape_overlaps_with<utils::geometry::rectangle::Rectangle3d>", "constructor": "country_core::systems::collision::world::RaycastWorld::shape_overlaps_with", "arguments": ["utils::geometry::rectangle::Rectangle3d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rectangle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-freeform-cache/B-shape-height-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::rectangle::Rectangle2d", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-gather/B-gable-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::rectangle::Rectangle2d", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-gather/B-spatial-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::rectangle::Rectangle2d", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-platform-stairs/B-extra-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::rectangle::Rectangle2d", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/B-platform-geometry/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::rectangle::Rectangle2d", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-wall-history/B-edit-extra-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::rectangle::Rectangle2d", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::rectangle::Rectangle2d", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rectangle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
// 信息 ID: 32e2a3b499eaf2f8f4dbe523
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_normal_at_u @ 0xc99e18 -> 0xc99e30 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners @ 0xc99e28 -> 0xc99e30 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::aabb
// 信息 ID: db136e9a6cc5ec386aa64a7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x787466 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::upload_dormer_holes_to_gpu @ 0x7b2f02 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0x9ef293 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0x9ef675 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0x9efe98 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0x9f05cd -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0xad67be -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xad84a7 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0xee45bc -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0xee42f7 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0xee43f0 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::remove_region_from_lazy_grid @ 0xee4b83 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22dad99 -> 0xc9b680 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=36056; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=37072; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
// 信息 ID: c227c9710e072c99fc95b15d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18112; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18400; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18848; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=19044; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=19324; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=19664; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=19996; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=20428; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::area
// 信息 ID: 14802432927b5b392be5c6fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle @ 0x155e1dd -> 0xc99520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle @ 0x155e3ce -> 0xc99520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x17d2ee4 -> 0xc99520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x17d32c3 -> 0xc99520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::get_building_size_01 @ 0x17d3f20 -> 0xc99520 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::points; module=768, inline_record=2784; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned; module=768, inline_record=4476; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=768, inline_record=4656; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=768, inline_record=5172; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=768, inline_record=6012; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::try_as_curve2; module=768, inline_record=7436; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=768, inline_record=8792; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point; module=768, inline_record=11512; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_curve2
// 信息 ID: fdcc7c222eba3982f47c30bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95af71 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97c1ef -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle @ 0x97f716 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle @ 0xa93c15 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::calc_shape_height::calc_shape_height @ 0xac812d -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2 @ 0x11ff2c7 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x1244933 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x124488c -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1248bff -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x12495ad -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1249b60 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier::closure$0::closure_env$0,tuple$<> > | country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier::closure$0::closure$0 @ 0x1559a34 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a4306c -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a4308d -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a46b6 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a46c9 -> 0xc98cf0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_curve3
// 信息 ID: 54ac2d9e090a5a94fde8323d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line @ 0x8dfd47 -> 0xc991f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::show @ 0xb02fa7 -> 0xc991f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle2d::show @ 0xb04681 -> 0xc991f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3 @ 0x11ff30d -> 0xc991f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a41fcf -> 0xc991f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a423a4 -> 0xc991f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a42a0f -> 0xc991f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x1eac9df -> 0xc991f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x22d4dea -> 0xc991f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points2
// 信息 ID: 9f5aca1a54808c03b3aa56bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77e802 -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad29eb -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2a98 -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xad8523 -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::do_ui_edit_rectangle_rotate @ 0xad91d0 -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedc19c -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd46c -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217f2f -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217f4f -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217fe2 -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y @ 0x125088f -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1> @ 0x12b1cba -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21af1d4 -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21af1f4 -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21af283 -> 0xc98ab0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::points; module=768, inline_record=2908; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=768, inline_record=5312; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=768, inline_record=6040; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::try_as_curve2; module=768, inline_record=7464; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=768, inline_record=8960; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id; module=768, inline_record=22116; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_edge_pts; module=768, inline_record=24176; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects_circle; module=768, inline_record=37852; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
// 信息 ID: 66dd42d48b9a63735794639b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedc120 -> 0xc98a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd3f3 -> 0xc98a50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::points; module=768, inline_record=3032; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=768, inline_record=4764; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=768, inline_record=5440; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=768, inline_record=6116; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::try_as_curve2; module=768, inline_record=7540; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=768, inline_record=9112; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point; module=768, inline_record=11476; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=13196; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points3
// 信息 ID: 91c7f999da98c299000258ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xe4d2 -> 0xc98b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1d912 -> 0xc98b90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::points; module=768, inline_record=2748; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=768, inline_record=8820; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_transformation_and_size
// 信息 ID: c8b16359c78baf210019f0a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedc00e -> 0xc995c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedc040 -> 0xc995c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd2f8 -> 0xc995c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd32e -> 0xc995c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=768, inline_record=39072; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=768, inline_record=39312; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::circular_slice
// 信息 ID: 0274835d9c30a66c78cb291c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7764b6 -> 0xc9bd90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1>
// 信息 ID: 408728853520f357deaada5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points -> ["system_decorator::decorator_visual::stairs::walk_shape::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs::walk_shape @ 0x80a189 -> 0x12b1b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::closest_edge_normal
// 信息 ID: 92ef64a2d1c702843c160c63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x815d8d -> 0xc9a8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
// 信息 ID: 283bedadfc4b7f624aa704e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds @ 0x811654 -> 0xc9ae20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1290c79 -> 0xc9ae20 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x1bf1d7e -> 0xc9ae20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests::closure$0 @ 0x22caa42 -> 0xc9ae20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests::closure$0 @ 0x22cab60 -> 0xc9ae20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::contains_point
// 信息 ID: 8f53d5510b7cd1dc648c4c74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 27 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls @ 0x978f89 -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls @ 0x978fc5 -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0x997a6c -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0x997abf -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_continue_wall::ui_continue_wall @ 0xaae9f0 -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0xab001d -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0xab037b -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2a01 -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2a1f -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2a3d -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2a5b -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2c2a -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff368 -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff38b -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff3ae -> 0xc9a630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff3d7 -> 0xc9a630 (direct_jump_outside_procedure); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::contains_point; module=768, inline_record=1752; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::contains_point_padding
// 信息 ID: a5a55a95d0c3dacc1e449fb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::trap_doors::generate_trap_doors @ 0x7beeba -> 0xc9a6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x127f6f2 -> 0xc9a6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x127f71c -> 0xc9a6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22da153 -> 0xc9a6a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::contains_point; module=768, inline_record=1776; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::contains_point; module=768, inline_record=24920; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge
// 信息 ID: dff2b5e174df5bbd35f69a8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124bf2d -> 0xc9a750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::edge_length
// 信息 ID: 9f38a7b499aecc78e7259766
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x127ac1a -> 0xc9ca50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x127c884 -> 0xc9ca50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x1fe6d26 -> 0xc9ca50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x200148d -> 0xc9ca50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::expand_along_edge
// 信息 ID: d6201b777ee886938aa1aeea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims @ 0xad78e4 -> 0xc9c620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
// 信息 ID: e517b30c058b5c96bc21d920
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::derive_decorator_subtype::is_half_dormer @ 0x814536 -> 0xc9acb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97be42 -> 0xc9acb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x127ac0f -> 0xc9acb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x127c879 -> 0xc9acb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x1fe6d1b -> 0xc9acb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x2001482 -> 0xc9acb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
// 信息 ID: ce15f36e0a18a8581bb02577
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id @ 0xc9acb0 -> 0xc9a9d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter @ 0xc9ae3e -> 0xc9a9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos @ 0xc9b04b -> 0xc9a9d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
// 信息 ID: 94e71844a23cf8531c77a551
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7760e6 -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7760ff -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x7ca85b -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x7caa5a -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x81a154 -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x81a267 -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedcbde -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedcbf1 -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedcf79 -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedcf8c -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedda0b -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedda1d -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xeddb42 -> 0xc9b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xeddb54 -> 0xc9b000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_corner_normal
// 信息 ID: 99e792f05833642daca5eb81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f6d54 -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0x83d55c -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x83da9e -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x83de0d -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz @ 0xb2a850 -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121e829 -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x124ff65 -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x12501f7 -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x12513a8 -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x12546a8 -> 0xc9a140 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18064; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18352; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18800; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18996; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=19248; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_corner_unnormal
// 信息 ID: 8cab7eb9c675597ffb9da2e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18088; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18376; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=18824; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=19020; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=19292; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_corner_normal; module=768, inline_record=21356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
// 信息 ID: 765e37156c3a8c0cf252a45e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f6d71 -> 0xc9a210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0x83d577 -> 0xc9a210 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121edb4 -> 0xc9a210 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x12501d6 -> 0xc9a210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u
// 信息 ID: e2a4b72841c3db80e7ac964e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121bfe1 -> 0xc9acc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121bed6 -> 0xc9acc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_normal
// 信息 ID: 7b188e7bd9d3753da1aa836c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77ef96 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77f2b4 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims @ 0xad73c3 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims @ 0xad7436 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims @ 0xad74a2 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1> @ 0x12b1c33 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1> @ 0x12b1c52 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1> @ 0x12b1c71 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1> @ 0x12b1c90 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::signifier @ 0x155ecdf -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget @ 0x155fe6e -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_assemble_bricks::construct_crossbrick_chimney<system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks::closure$0::closure$0::closure_env$0> @ 0x200c301 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> > @ 0x2419673 -> 0xc9a5a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=19972; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=20404; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=20836; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=20976; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::expand_along_edge; module=768, inline_record=45728; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_pts
// 信息 ID: 2df8e395167c94695aa55e9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97be54 -> 0xc9a470 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x231dc0e -> 0xc9a470 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x231dcf6 -> 0xc9a470 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::edge_length; module=768, inline_record=47924; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_normal_at_u
// 信息 ID: 4a93e5a65d2a2f3778f7d7bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedcc01 -> 0xc99e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedcc15 -> 0xc99e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedcf9c -> 0xc99e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedcfb0 -> 0xc99e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedda2c -> 0xc99e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedda3e -> 0xc99e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xeddb63 -> 0xc99e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xeddb75 -> 0xc99e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x122771b -> 0xc99e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners
// 信息 ID: 8a88f309baa0cb313543697c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x776a0b -> 0xc99e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_point
// 信息 ID: 0265f919d6c702541c19a634
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 115 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77ed97 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77edb4 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x81a111 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0x83d45e -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0x83d47e -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x83da66 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x83dddf -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d301c -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d304f -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d3082 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d30b5 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d30e5 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d30f2 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d30ff -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d310c -> 0xc99600 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95c314 -> 0xc99600 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=13164; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=13348; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=13532; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=13820; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=14116; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=15684; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=16212; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=768, inline_record=28980; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_point_u
// 信息 ID: afae164a0cc5918c33c439c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 66 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758792 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x7587a5 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x7587b9 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x7587d0 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75de5a -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75dea5 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75dfeb -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75e02f -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75e156 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75e19a -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75e2c7 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75e306 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78bdd6 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78bde9 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78bdf8 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78be0b -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=12476; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=12704; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=12904; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=14840; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=15088; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=15392; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=16748; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=768, inline_record=17372; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_pos_at_u
// 信息 ID: 63550c94a3478231935b55a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1> @ 0x12b2041 -> 0xc99800 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env$1> @ 0x12b239d -> 0xc99800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::half_dimensions
// 信息 ID: 6bda473260cee6bf9802308b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=36104; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::aabb; module=768, inline_record=37512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::intersects
// 信息 ID: 226547fe1a0f66bc4bef92f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x1236914 -> 0xc9b990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::intersects_circle
// 信息 ID: ce5d0c670e4ec10202da5600
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x1236a3c -> 0xc9b6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x1236a4e -> 0xc9b6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::lerp
// 信息 ID: 0fb4fda2e5829c4f07aacc68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xad2d85 -> 0xc9c730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xad2e55 -> 0xc9c730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::local_aabb_of
// 信息 ID: c0c5ac025cba78f3b76c96c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=35516; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=36548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new
// 信息 ID: 5482b163159835e6d9a5fc6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7d9da2e094c5e3aaa2b1ca08: void (*)(bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 21 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77e7f2 -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77ece8 -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y @ 0x8e3abd -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x8ec66c -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0x9ef338 -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0x9efb7e -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0x9f02de -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::commands::Commands)> > @ 0xaf2288 -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0xee45ab -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0xee42e6 -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0xee43df -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::remove_region_from_lazy_grid @ 0xee4b73 -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y @ 0x1250882 -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::commands::Commands)> > > @ 0x14d99ea -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::commands::Commands)> > > @ 0x150626d -> 0xc98980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::glade_themes::olden::startup_olden @ 0x1553e33 -> 0xc98980 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::new; module=768, inline_record=380; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned; module=768, inline_record=3752; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::new_axis_aligned; module=768, inline_record=3984; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::lerp; module=768, inline_record=47352; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=768, inline_record=48492; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new_axis_aligned
// 信息 ID: fe3c43345604b4ece4a21c69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xa07af0 -> 0xc98900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217e28 -> 0xc98900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a3648 -> 0xc98900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21af047 -> 0xc98900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x231dbd0 -> 0xc98900 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=768, inline_record=48368; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
// 信息 ID: 5002c5c952c5c0323296a62c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95c27c -> 0xc98890 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95c29d -> 0xc98890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217df2 -> 0xc98890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a3601 -> 0xc98890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21aeff0 -> 0xc98890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::orientation_mat2
// 信息 ID: c8c75fc474a33dbc72cf5f4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 19 条；内联宿主 35 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x819e4f -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x819e70 -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x8e283a -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x8e2994 -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x8e2b30 -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x8e2c7f -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x8e2ac2 -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz @ 0x8e37ad -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0x9ef2c3 -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217e65 -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x1251bec -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x1251c0c -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x1254df3 -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x1254e16 -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x175ae1c -> 0xc99540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x175ae6f -> 0xc99540 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::contains_point; module=768, inline_record=1816; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::points; module=768, inline_record=2948; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=768, inline_record=4680; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=768, inline_record=5356; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=768, inline_record=6088; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::try_as_curve2; module=768, inline_record=7512; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=768, inline_record=9020; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_transformation_and_size; module=768, inline_record=11276; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::orientation_mat3
// 信息 ID: 0eec37a992b0118ae47a294c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0x83d681 -> 0xc99570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97a9fe -> 0xc99570 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a42020 -> 0xc99570 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a4259b -> 0xc99570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d
// 信息 ID: e26dfdbce68cffdf19c6a6c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22daf14 -> 0xc9b420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::perimeter
// 信息 ID: 6f0c0658466a4497bb441311
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 23 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x7ca867 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x7caa66 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x81a160 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x81a272 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95b031 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::show @ 0xb03002 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121d7a3 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121e900 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x12445ba -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x12447f6 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x1244816 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1249506 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x124954b -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1249557 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1249af3 -> 0xc99530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1249aff -> 0xc99530 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=768, inline_record=12004; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=768, inline_record=12032; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=768, inline_record=12060; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=768, inline_record=12504; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=15132; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=15160; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=15188; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=768, inline_record=15448; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
// 信息 ID: 08270f89e79fca171e9a13eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1249ce7 -> 0xc9a820 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge; module=768, inline_record=26264; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::try_as_curve2
// 信息 ID: 218236ab1d25bea938458c64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x12445d0 -> 0xc98fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::with_expanded_size
// 信息 ID: e32ac5d5889116036647f265
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x226ad33 -> 0xc9a730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::center
// 信息 ID: 9c62a477801795f20a649778
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0ec546db294e804e3e737063: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=73, inline_record=35940; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=73, inline_record=54972; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator; module=420, inline_record=5148; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=428, inline_record=21200; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=428, inline_record=21340; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=429, inline_record=15712; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=433, inline_record=14280; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=441, inline_record=17828; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::contains_point
// 信息 ID: 9ff0720e0e8b6ad00db90aaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance @ 0x7b096e -> 0xc985a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance @ 0x7b0c45 -> 0xc985a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::expand
// 信息 ID: 1c97b8a2ddefb3e6d4221876
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x12367c7 -> 0xc98620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::extent
// 信息 ID: fb7777d6255be24c55c25bea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::create_cart_chain @ 0x3be721 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::create_cart_chain @ 0x3be801 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x7bd856 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x7bd9b3 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0x7c6bc1 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0x7c6ca4 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x7ea112 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x7ea50a -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ecda0 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ed99c -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7eda9e -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7edacc -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f1501 -> 0xc98660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$41::into_transformed_shape @ 0x155d35c -> 0xc98660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::height
// 信息 ID: 575969aad56adaaa578a2def
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0ec546db294e804e3e737063: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 44 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=73, inline_record=35912; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=73, inline_record=54944; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=428, inline_record=21116; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=428, inline_record=21256; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=14340; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=435, inline_record=31032; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=439, inline_record=9788; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=439, inline_record=9988; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::length
// 信息 ID: 3dd35fe94f6de0c335cd88c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0ec546db294e804e3e737063: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=73, inline_record=35884; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=73, inline_record=54916; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=428, inline_record=21172; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=428, inline_record=21312; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=439, inline_record=9844; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::world::update_rapier_colliders; module=496, inline_record=75548; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::extent; module=768, inline_record=2556; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1415, inline_record=4840; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::new
// 信息 ID: 98093b0a182fa63f4a4b74b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0ec546db294e804e3e737063: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x179204 -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x17ae68 -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance @ 0x7b0940 -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_subtype::DecoratorCollision::empty @ 0xb25d0d -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124c35e -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x125046b -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> > @ 0x1fe71d6 -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> @ 0x1fe73bd -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0 @ 0x22c1d89 -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22da4a9 -> 0xc98380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22daa66 -> 0xc98380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
// 信息 ID: d4440fccc6101daa4bd5850b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> > @ 0x1fe71e2 -> 0xc98580 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates @ 0x22ce64b -> 0xc98580 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22da56d -> 0xc98580 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22dab07 -> 0xc98580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::points
// 信息 ID: 7d0bf7d11ed8b98096d4e630
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::rotate_projective_xz
// 信息 ID: cec4905d5ac62fae59fdf7b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance @ 0x7b0c07 -> 0xc984f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::set_orientation_xz
// 信息 ID: cf434316b948640c1de2df0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a4ece -> 0xc98520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x816338 -> 0xc98520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter @ 0x9a983d -> 0xc98520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x1236875 -> 0xc98520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d868b -> 0xc98520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::set_size
// 信息 ID: 16ae4861a2cfa3e3867d236f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter @ 0x7e3d08 -> 0xc98530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter @ 0x9a97d6 -> 0xc98530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::translate
// 信息 ID: e871e0981dbcfda17b2ad8f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a4eed -> 0xc984c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance @ 0x7b0c17 -> 0xc984c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x81635d -> 0xc984c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter @ 0x9a9828 -> 0xc984c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter @ 0x9a986a -> 0xc984c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x1236896 -> 0xc984c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d863e -> 0xc984c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::width
// 信息 ID: 5b4e469b3ba3ed6bdc52126a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0ec546db294e804e3e737063: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 41 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=73, inline_record=35856; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=73, inline_record=54888; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator; module=420, inline_record=5120; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=428, inline_record=21144; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=428, inline_record=21284; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=429, inline_record=10280; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=14312; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=433, inline_record=10208; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize
// 信息 ID: ddbad069b6e85378f65c501d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >; module=1673, inline_record=69672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 09149f63b22c2134a33fe33b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::geometry::rectangle::_::impl$0::deserialize::__FieldVisitor> @ 0x157bb70 -> 0xc9cc20 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 信息 ID: 1ecd2ee5005078d22c115031
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: af1360fdf05b99e937be6daa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: a8a304c817d6962538c8a146
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=20160; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=20388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: c3e523b3f618715318a14465
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=20032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 78122e9e220a17910086a40b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::geometry::rectangle::_::impl$0::deserialize::__Visitor> @ 0x157bbe0 -> 0xc9cc40 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: c7bd2b57354d997715a9492b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=19644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 9b2ee1099fae040d18d5eb50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=23416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 63272e3f4e35986a43ec9164
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] utils::geometry::rectangle::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x2595ce -> 0xefde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x16ec367 -> 0x157c4c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: f1559068bdf200c53d11d2df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::rectangle::Rectangle2d> @ 0x15913ef -> 0x157c5e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$10::eq
// 信息 ID: 3bb9ba48c196f24861cadd8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2599, inline_record=22496; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3465, inline_record=4228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::as_points3::closure$0
// 信息 ID: d1f7d5e3b2a657d43a02f539
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::as_points3", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::points; module=768, inline_record=2884; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=768, inline_record=5284; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=768, inline_record=8932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0
// 信息 ID: 2d0eab2ce9c86bac7897da37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::get_pos_at_u", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_pos_at_u @ 0xc99a89 -> 0xc99b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::intersects::closure$0
// 信息 ID: 40491b8b981df7e3a61c89da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::intersects", "captures": "unknown"}
// [闭包环境类型引用] utils::geometry::rectangle::impl$1::intersects::closure_env$0 <- itertools::tuple_impl::impl$10::next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::rectangle::impl$1::intersects::closure_env$0 <- core::iter::adapters::chain::impl$1::next<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$10::next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=2085, inline_record=1064; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$10::next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=2085, inline_record=2012; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::next<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0> > > >; module=3376, inline_record=1496; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::next<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0> > > >; module=3376, inline_record=2540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::intersects_circle::closure$0
// 信息 ID: 1ade6df34a5552420853708b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::intersects_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects_circle; module=768, inline_record=38628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$2::from
// 信息 ID: a6bb263d6ef9d60aaebeb992
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x8ec16d -> 0xc9cb90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$3::clone
// 信息 ID: 470eb49a03cc278563e7bc74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert_collision_and_interaction_as_same; module=582, inline_record=8420; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert; module=582, inline_record=11496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$6::clone
// 信息 ID: 80ac49ce5e8ad5cf95b34f5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > >; module=1576, inline_record=10044; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle; module=1606, inline_record=107064; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle; module=1606, inline_record=107092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle; module=1606, inline_record=107120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle; module=1606, inline_record=107148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$7::fmt
// 信息 ID: 8995ec0bcd9e1b4b95236feb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<utils::geometry::rectangle::Rectangle2d>; module=1623, inline_record=4600; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<utils::geometry::rectangle::Rectangle2d>; module=4659, inline_record=408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
