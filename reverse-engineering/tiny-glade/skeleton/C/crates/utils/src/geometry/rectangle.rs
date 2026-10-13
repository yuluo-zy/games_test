// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/geometry/rectangle.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 94aa565632f8541282d61dc9f9718bfcabcaad58c03b2f8483c0a26e137b225f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 38, 85, 418, 437, 445, 446, 449, 450, 452, 455, 468, 469, 470, 472, 476, 478, 481, 482, 492, 495, 503, 509, 527, 541, 551, 553, 561, 574, 580, 603, 609, 764, 1455, 1490, 1495, 1544, 1574, 1576, 1579, 1604, 1631, 1669, 1770, 1795, 2190, 2741, 3184, 3185, 3205, 3208, 3638, 3779, 4054, 4989
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
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
// 首条位置: {"module":764,"record_offset":16684,"rva":14604336,"va":5383313456}
// 编译实例 RVAs: 0xded830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33308139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383313456,"raw_hex":"e8eb3dfc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383314096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":19148,"parent_record_offset":16684,"ancestor_record_offset":16684,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":18728,"parent_record_offset":16684,"ancestor_record_offset":16684,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":17388,"parent_record_offset":16684,"ancestor_record_offset":16684,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17444,"parent_record_offset":16684,"ancestor_record_offset":16684,"inlinee":49209,"source_line":394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":19260,"parent_record_offset":16684,"ancestor_record_offset":16684,"inlinee":49211,"source_line":420,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":16848,"parent_record_offset":16684,"ancestor_record_offset":16684,"inlinee":49200,"source_line":283,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":16684,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":16980,"parent_record_offset":16956,"ancestor_record_offset":16684,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":16684,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":16684,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":17620,"parent_record_offset":17596,"ancestor_record_offset":16684,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":17872,"parent_record_offset":17848,"ancestor_record_offset":16684,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":17912,"parent_record_offset":17744,"ancestor_record_offset":16684,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":16684,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":16684,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":16684,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17596,"parent_record_offset":17572,"ancestor_record_offset":16684,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17572,"parent_record_offset":17492,"ancestor_record_offset":16684,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":17812,"parent_record_offset":17776,"ancestor_record_offset":16684,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":17776,"parent_record_offset":17744,"ancestor_record_offset":16684,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":16684,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":17716,"parent_record_offset":17668,"ancestor_record_offset":16684,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":17848,"parent_record_offset":17812,"ancestor_record_offset":16684,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":16908,"parent_record_offset":16848,"ancestor_record_offset":16684,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":16684,"inlinee":32708,"source_line":6,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17492,"parent_record_offset":17468,"ancestor_record_offset":16684,"inlinee":49208,"source_line":450,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::get_corner_unnormal
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17468,"parent_record_offset":17444,"ancestor_record_offset":16684,"inlinee":49210,"source_line":366,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17516,"parent_record_offset":17492,"ancestor_record_offset":16684,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":16876,"parent_record_offset":16848,"ancestor_record_offset":16684,"inlinee":49199,"source_line":241,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1604,"record_offset":3256,"rva":21960128,"va":5390669248}
// 编译实例 RVAs: 0x14f15c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25950457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390669248,"raw_hex":"e8f9f88b01","size":5,"soft_fail":false,"target_va":5416622240,"va":5390671778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure$0
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390669248,"raw_hex":"e87bf5ffff","size":5,"soft_fail":false,"target_va":5390668048,"va":5390670736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7362265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390669248,"raw_hex":"e827a98fff","size":5,"soft_fail":false,"target_va":5383307344,"va":5390669604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7356159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390669248,"raw_hex":"e801c18fff","size":5,"soft_fail":false,"target_va":5383313312,"va":5390669466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7360300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390669248,"raw_hex":"e8d4b08fff","size":5,"soft_fail":false,"target_va":5383309120,"va":5390669415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7360968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390669248,"raw_hex":"e838ae8fff","size":5,"soft_fail":false,"target_va":5383309664,"va":5390670627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1604,"record_offset":5912,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1604,"record_offset":3536,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1604,"record_offset":3648,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1604,"record_offset":3564,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure$1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/stairs_graph_and_requests.rs
//     evidence={"module":1604,"record_offset":5996,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":80747,"source_line":1742,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$13::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1604,"record_offset":4896,"parent_record_offset":4276,"ancestor_record_offset":3256,"inlinee":80745,"source_line":365,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1604,"record_offset":3900,"parent_record_offset":3836,"ancestor_record_offset":3256,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1604,"record_offset":3940,"parent_record_offset":3900,"ancestor_record_offset":3256,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1604,"record_offset":4036,"parent_record_offset":3980,"ancestor_record_offset":3256,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1604,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":3256,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1604,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":3256,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1604,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":3256,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1604,"record_offset":5496,"parent_record_offset":5472,"ancestor_record_offset":3256,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1604,"record_offset":4336,"parent_record_offset":4276,"ancestor_record_offset":3256,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1604,"record_offset":4232,"parent_record_offset":3836,"ancestor_record_offset":3256,"inlinee":11834,"source_line":225,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1604,"record_offset":4276,"parent_record_offset":4232,"ancestor_record_offset":3256,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1604,"record_offset":4516,"parent_record_offset":4476,"ancestor_record_offset":3256,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1604,"record_offset":4476,"parent_record_offset":4276,"ancestor_record_offset":3256,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1604,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":3256,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1604,"record_offset":3688,"parent_record_offset":3648,"ancestor_record_offset":3256,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1604,"record_offset":3612,"parent_record_offset":3588,"ancestor_record_offset":3256,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1604,"record_offset":3732,"parent_record_offset":3688,"ancestor_record_offset":3256,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1604,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":3256,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1604,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3256,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1604,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":3256,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1604,"record_offset":4188,"parent_record_offset":4148,"ancestor_record_offset":3256,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1604,"record_offset":6076,"parent_record_offset":6048,"ancestor_record_offset":3256,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1604,"record_offset":4388,"parent_record_offset":4336,"ancestor_record_offset":3256,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1604,"record_offset":4148,"parent_record_offset":3980,"ancestor_record_offset":3256,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1604,"record_offset":3980,"parent_record_offset":3940,"ancestor_record_offset":3256,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1604,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":3256,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1604,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":3256,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1604,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":3256,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1604,"record_offset":5876,"parent_record_offset":3648,"ancestor_record_offset":3256,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1604,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":3256,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1604,"record_offset":5836,"parent_record_offset":3732,"ancestor_record_offset":3256,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1604,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":3256,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1604,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":3256,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1604,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":3256,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1604,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":3256,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1604,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":3256,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1604,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":3256,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1604,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":3256,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1604,"record_offset":6048,"parent_record_offset":5996,"ancestor_record_offset":3256,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1604,"record_offset":5424,"parent_record_offset":5024,"ancestor_record_offset":3256,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1604,"record_offset":5340,"parent_record_offset":5024,"ancestor_record_offset":3256,"inlinee":7465,"source_line":388,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1604,"record_offset":5024,"parent_record_offset":3732,"ancestor_record_offset":3256,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1604,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":3256,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01604.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1604,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":3256,"inlinee":27128,"source_line":115,"archive":"_pdb/C/modules/01604.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":22028,"rva":14605504,"va":5383314624}
// 编译实例 RVAs: 0xdedcc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33307275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383314624,"raw_hex":"e88b3afc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383314928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33307331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383314624,"raw_hex":"e8c33afc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383314904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":22184,"parent_record_offset":22028,"ancestor_record_offset":22028,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":22760,"parent_record_offset":22028,"ancestor_record_offset":22028,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":22696,"parent_record_offset":22028,"ancestor_record_offset":22028,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":22368,"parent_record_offset":22028,"ancestor_record_offset":22028,"inlinee":49211,"source_line":420,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":22028,"inlinee":6830,"source_line":2141,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":22028,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":22652,"parent_record_offset":22628,"ancestor_record_offset":22028,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":764,"record_offset":22228,"parent_record_offset":22184,"ancestor_record_offset":22028,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":764,"record_offset":22256,"parent_record_offset":22228,"ancestor_record_offset":22028,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":22784,"parent_record_offset":22760,"ancestor_record_offset":22028,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":22724,"parent_record_offset":22696,"ancestor_record_offset":22028,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":22412,"parent_record_offset":22368,"ancestor_record_offset":22028,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":22436,"parent_record_offset":22412,"ancestor_record_offset":22028,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":22572,"parent_record_offset":22548,"ancestor_record_offset":22028,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":22628,"parent_record_offset":22604,"ancestor_record_offset":22028,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":22604,"parent_record_offset":22520,"ancestor_record_offset":22028,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":22520,"parent_record_offset":22368,"ancestor_record_offset":22028,"inlinee":49208,"source_line":450,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":22548,"parent_record_offset":22520,"ancestor_record_offset":22028,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":28420,"rva":14607792,"va":5383316912}
// 编译实例 RVAs: 0xdee5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33304517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383316912,"raw_hex":"e8c52ffc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383317686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33304573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383316912,"raw_hex":"e8fd2ffc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383317662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383316912,"raw_hex":"e8af060000","size":5,"soft_fail":false,"target_va":5383318688,"va":5383316972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":30020,"parent_record_offset":28420,"ancestor_record_offset":28420,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":764,"record_offset":30052,"parent_record_offset":28420,"ancestor_record_offset":28420,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":30192,"parent_record_offset":28420,"ancestor_record_offset":28420,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":28588,"parent_record_offset":28420,"ancestor_record_offset":28420,"inlinee":49201,"source_line":265,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":29144,"parent_record_offset":28420,"ancestor_record_offset":28420,"inlinee":49200,"source_line":283,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":29732,"parent_record_offset":28420,"ancestor_record_offset":28420,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":29340,"parent_record_offset":29316,"ancestor_record_offset":28420,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":29364,"parent_record_offset":29340,"ancestor_record_offset":28420,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":28828,"parent_record_offset":28804,"ancestor_record_offset":28420,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":29988,"parent_record_offset":29732,"ancestor_record_offset":28420,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":28888,"parent_record_offset":28864,"ancestor_record_offset":28420,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":30104,"parent_record_offset":30052,"ancestor_record_offset":28420,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":29788,"parent_record_offset":29764,"ancestor_record_offset":28420,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":29316,"parent_record_offset":29292,"ancestor_record_offset":28420,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":29292,"parent_record_offset":29268,"ancestor_record_offset":28420,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":28748,"parent_record_offset":28724,"ancestor_record_offset":28420,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":28804,"parent_record_offset":28780,"ancestor_record_offset":28420,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":28780,"parent_record_offset":28588,"ancestor_record_offset":28420,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":29900,"parent_record_offset":29876,"ancestor_record_offset":28420,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":28864,"parent_record_offset":28588,"ancestor_record_offset":28420,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":29764,"parent_record_offset":29732,"ancestor_record_offset":28420,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":29268,"parent_record_offset":29144,"ancestor_record_offset":28420,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":28688,"parent_record_offset":28644,"ancestor_record_offset":28420,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":28644,"parent_record_offset":28588,"ancestor_record_offset":28420,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":28724,"parent_record_offset":28588,"ancestor_record_offset":28420,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":29184,"parent_record_offset":29144,"ancestor_record_offset":28420,"inlinee":49199,"source_line":241,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":29876,"parent_record_offset":29732,"ancestor_record_offset":28420,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":35764,"rva":14610592,"va":5383319712}
// 编译实例 RVAs: 0xdef0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33301944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383319712,"raw_hex":"e8b825fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383320259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":36040,"parent_record_offset":35764,"ancestor_record_offset":35764,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":35984,"parent_record_offset":35764,"ancestor_record_offset":35764,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":35928,"parent_record_offset":35764,"ancestor_record_offset":35764,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":36184,"parent_record_offset":36160,"ancestor_record_offset":35764,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":36224,"parent_record_offset":36064,"ancestor_record_offset":35764,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":36008,"parent_record_offset":35984,"ancestor_record_offset":35764,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":764,"record_offset":37956,"parent_record_offset":37924,"ancestor_record_offset":35764,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":36128,"parent_record_offset":36096,"ancestor_record_offset":35764,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":36096,"parent_record_offset":36064,"ancestor_record_offset":35764,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":36064,"parent_record_offset":36040,"ancestor_record_offset":35764,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":36160,"parent_record_offset":36128,"ancestor_record_offset":35764,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":35956,"parent_record_offset":35928,"ancestor_record_offset":35764,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":20328,"rva":14604992,"va":5383314112}
// 编译实例 RVAs: 0xdedac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33307867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383314112,"raw_hex":"e8db3cfc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383314368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":21008,"parent_record_offset":20328,"ancestor_record_offset":20328,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":20476,"parent_record_offset":20328,"ancestor_record_offset":20328,"inlinee":49200,"source_line":283,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":20604,"parent_record_offset":20580,"ancestor_record_offset":20328,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":20628,"parent_record_offset":20604,"ancestor_record_offset":20328,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":20328,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":20328,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":20532,"parent_record_offset":20476,"ancestor_record_offset":20328,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":20504,"parent_record_offset":20476,"ancestor_record_offset":20328,"inlinee":49199,"source_line":241,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_point_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=23
// 源码行: 0, 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC030
// 首条位置: {"module":764,"inlinee":49200}
// 编译实例 RVAs: 0xdec740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33312920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383309120,"raw_hex":"e89850fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383309283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33312906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383309120,"raw_hex":"e88a50fc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383309329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=22
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":5180,"parent_record_offset":4928,"ancestor_record_offset":4928,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=22
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":5068,"parent_record_offset":4928,"ancestor_record_offset":4928,"inlinee":49199,"source_line":241,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":4928,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":4928,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":5232,"parent_record_offset":5208,"ancestor_record_offset":4928,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":5208,"parent_record_offset":5180,"ancestor_record_offset":4928,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":5764,"rva":14600544,"va":5383309664}
// 编译实例 RVAs: 0xdec960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33311880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383309664,"raw_hex":"e8884cfc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383310323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33311936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383309664,"raw_hex":"e8c04cfc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383310299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t191","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383309664,"raw_hex":"e9bf000000","size":5,"soft_fail":false,"target_va":5383310336,"va":5383310140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":7612,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":6448,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":6504,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":49201,"source_line":265,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":5916,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":49200,"source_line":283,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":5764,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":5764,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":5764,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":5764,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":6980,"parent_record_offset":6888,"ancestor_record_offset":5764,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":6920,"parent_record_offset":6888,"ancestor_record_offset":5764,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":6948,"parent_record_offset":6888,"ancestor_record_offset":5764,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":7012,"parent_record_offset":6844,"ancestor_record_offset":5764,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":5764,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":5992,"parent_record_offset":5968,"ancestor_record_offset":5764,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":6624,"parent_record_offset":6600,"ancestor_record_offset":5764,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":6888,"parent_record_offset":6844,"ancestor_record_offset":5764,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":6844,"parent_record_offset":6816,"ancestor_record_offset":5764,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":5968,"parent_record_offset":5916,"ancestor_record_offset":5764,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":5764,"inlinee":32708,"source_line":6,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":6568,"parent_record_offset":6536,"ancestor_record_offset":5764,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":6536,"parent_record_offset":6504,"ancestor_record_offset":5764,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":6600,"parent_record_offset":6504,"ancestor_record_offset":5764,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":5764,"inlinee":49199,"source_line":241,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":25928,"rva":14607056,"va":5383316176}
// 编译实例 RVAs: 0xdee2d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33305432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383316176,"raw_hex":"e85833fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383316771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":764,"record_offset":26084,"parent_record_offset":25928,"ancestor_record_offset":25928,"inlinee":49104,"source_line":73,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::intersects
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":764,"record_offset":27000,"parent_record_offset":25928,"ancestor_record_offset":25928,"inlinee":49102,"source_line":133,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::local_aabb_of
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":26284,"parent_record_offset":25928,"ancestor_record_offset":25928,"inlinee":49216,"source_line":582,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":26672,"parent_record_offset":26584,"ancestor_record_offset":25928,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":26468,"parent_record_offset":26416,"ancestor_record_offset":25928,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":26440,"parent_record_offset":26416,"ancestor_record_offset":25928,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":26640,"parent_record_offset":26584,"ancestor_record_offset":25928,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":26852,"parent_record_offset":26708,"ancestor_record_offset":25928,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26960,"parent_record_offset":26936,"ancestor_record_offset":25928,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26196,"parent_record_offset":26172,"ancestor_record_offset":25928,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26248,"parent_record_offset":26224,"ancestor_record_offset":25928,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":26360,"parent_record_offset":26332,"ancestor_record_offset":25928,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":26416,"parent_record_offset":26392,"ancestor_record_offset":25928,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":26392,"parent_record_offset":26284,"ancestor_record_offset":25928,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26140,"parent_record_offset":26116,"ancestor_record_offset":25928,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26116,"parent_record_offset":26084,"ancestor_record_offset":25928,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26936,"parent_record_offset":26708,"ancestor_record_offset":25928,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26224,"parent_record_offset":26084,"ancestor_record_offset":25928,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26172,"parent_record_offset":26084,"ancestor_record_offset":25928,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":26708,"parent_record_offset":26284,"ancestor_record_offset":25928,"inlinee":49217,"source_line":643,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":26756,"parent_record_offset":26708,"ancestor_record_offset":25928,"inlinee":49218,"source_line":638,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":26332,"parent_record_offset":26284,"ancestor_record_offset":25928,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":8120,"rva":14601216,"va":5383310336}
// 编译实例 RVAs: 0xdecc00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33311267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310336,"raw_hex":"e8234afc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383310936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33311323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310336,"raw_hex":"e85b4afc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383310912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":9608,"parent_record_offset":8120,"ancestor_record_offset":8120,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":8876,"parent_record_offset":8120,"ancestor_record_offset":8120,"inlinee":49201,"source_line":265,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":8276,"parent_record_offset":8120,"ancestor_record_offset":8120,"inlinee":49200,"source_line":283,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::inverse_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":9304,"parent_record_offset":8120,"ancestor_record_offset":8120,"inlinee":48846,"source_line":257,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":8120,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":8528,"parent_record_offset":8504,"ancestor_record_offset":8120,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":9168,"parent_record_offset":9080,"ancestor_record_offset":8120,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":9108,"parent_record_offset":9080,"ancestor_record_offset":8120,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":9136,"parent_record_offset":9080,"ancestor_record_offset":8120,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":9236,"parent_record_offset":9204,"ancestor_record_offset":8120,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":9668,"parent_record_offset":9636,"ancestor_record_offset":8120,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":8120,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":8120,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":8120,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":9080,"parent_record_offset":9056,"ancestor_record_offset":8120,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":9056,"parent_record_offset":8876,"ancestor_record_offset":8120,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":9636,"parent_record_offset":9608,"ancestor_record_offset":8120,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":9204,"parent_record_offset":8876,"ancestor_record_offset":8120,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":8432,"parent_record_offset":8276,"ancestor_record_offset":8120,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":8968,"parent_record_offset":8924,"ancestor_record_offset":8120,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":8924,"parent_record_offset":8876,"ancestor_record_offset":8120,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":9000,"parent_record_offset":8876,"ancestor_record_offset":8120,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":8320,"parent_record_offset":8276,"ancestor_record_offset":8120,"inlinee":49199,"source_line":241,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$4::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x95C6
// 首条位置: {"module":603,"inlinee":38342}
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
// 首条位置: {"module":764,"record_offset":46432,"rva":14614544,"va":5383323664}
// 编译实例 RVAs: 0xdf0010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33298274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383323664,"raw_hex":"e86217fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383323929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":46560,"parent_record_offset":46432,"ancestor_record_offset":46432,"inlinee":49174,"source_line":141,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":46744,"parent_record_offset":46684,"ancestor_record_offset":46432,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":46604,"parent_record_offset":46560,"ancestor_record_offset":46432,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":46716,"parent_record_offset":46684,"ancestor_record_offset":46432,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":46660,"parent_record_offset":46560,"ancestor_record_offset":46432,"inlinee":49220,"source_line":605,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":46684,"parent_record_offset":46660,"ancestor_record_offset":46432,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":48172,"rva":14615552,"va":5383324672}
// 编译实例 RVAs: 0xdf0400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":48308,"parent_record_offset":48172,"ancestor_record_offset":48172,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":48332,"parent_record_offset":48308,"ancestor_record_offset":48172,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":46172,"rva":14614496,"va":5383323616}
// 编译实例 RVAs: 0xdeffe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":46320,"parent_record_offset":46172,"ancestor_record_offset":46172,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":46344,"parent_record_offset":46320,"ancestor_record_offset":46172,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":45860,"rva":14614448,"va":5383323568}
// 编译实例 RVAs: 0xdeffb0
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
// 首条位置: {"module":764,"record_offset":48044,"rva":14615472,"va":5383324592}
// 编译实例 RVAs: 0xdf03b0
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
// 首条位置: {"module":764,"record_offset":45996,"rva":14614464,"va":5383323584}
// 编译实例 RVAs: 0xdeffc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$6::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":46144,"parent_record_offset":45996,"ancestor_record_offset":45996,"inlinee":49223,"source_line":520,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":45164,"rva":14614320,"va":5383323440}
// 编译实例 RVAs: 0xdeff30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":45308,"parent_record_offset":45164,"ancestor_record_offset":45164,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":45412,"parent_record_offset":45164,"ancestor_record_offset":45164,"inlinee":49221,"source_line":454,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":45356,"parent_record_offset":45332,"ancestor_record_offset":45164,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":45376,"parent_record_offset":45356,"ancestor_record_offset":45164,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":45700,"parent_record_offset":45612,"ancestor_record_offset":45164,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":45788,"parent_record_offset":45764,"ancestor_record_offset":45164,"inlinee":49207,"source_line":530,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":45640,"parent_record_offset":45612,"ancestor_record_offset":45164,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":45668,"parent_record_offset":45612,"ancestor_record_offset":45164,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":45560,"parent_record_offset":45536,"ancestor_record_offset":45164,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":45332,"parent_record_offset":45308,"ancestor_record_offset":45164,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/sse2/bvec4a.rs
//     evidence={"module":764,"record_offset":45820,"parent_record_offset":45436,"ancestor_record_offset":45164,"inlinee":27913,"source_line":91,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":45500,"parent_record_offset":45472,"ancestor_record_offset":45164,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":45612,"parent_record_offset":45588,"ancestor_record_offset":45164,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":45588,"parent_record_offset":45436,"ancestor_record_offset":45164,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":764,"record_offset":45764,"parent_record_offset":45436,"ancestor_record_offset":45164,"inlinee":49206,"source_line":453,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":764,"record_offset":45736,"parent_record_offset":45436,"ancestor_record_offset":45164,"inlinee":11831,"source_line":110,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":45536,"parent_record_offset":45436,"ancestor_record_offset":45164,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":45436,"parent_record_offset":45412,"ancestor_record_offset":45164,"inlinee":49205,"source_line":459,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":45472,"parent_record_offset":45436,"ancestor_record_offset":45164,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::center
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C43
// 首条位置: {"module":85,"inlinee":11331}
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
// 记录形式: inline_instance, inlinee_declaration; instances=77
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C42
// 首条位置: {"module":85,"inlinee":11330}
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
// 记录形式: inline_instance, inlinee_declaration; instances=67
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C40
// 首条位置: {"module":85,"inlinee":11328}
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
// IPI IDs: 0x2C41
// 首条位置: {"module":85,"inlinee":11329}
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
// 首条位置: {"module":764,"record_offset":46788,"rva":14614864,"va":5383323984}
// 编译实例 RVAs: 0xdf0150
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
// 首条位置: {"module":764,"record_offset":46912,"rva":14614928,"va":5383324048}
// 编译实例 RVAs: 0xdf0190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":764,"record_offset":47132,"parent_record_offset":46912,"ancestor_record_offset":46912,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47056,"parent_record_offset":46912,"ancestor_record_offset":46912,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47104,"parent_record_offset":46912,"ancestor_record_offset":46912,"inlinee":11329,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47080,"parent_record_offset":46912,"ancestor_record_offset":46912,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":47164,"rva":14614976,"va":5383324096}
// 编译实例 RVAs: 0xdf01c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33297623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383324096,"raw_hex":"e8d714fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383324580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47296,"parent_record_offset":47164,"ancestor_record_offset":47164,"inlinee":49204,"source_line":199,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":47832,"parent_record_offset":47632,"ancestor_record_offset":47164,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":47748,"parent_record_offset":47632,"ancestor_record_offset":47164,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":47708,"parent_record_offset":47632,"ancestor_record_offset":47164,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":47788,"parent_record_offset":47632,"ancestor_record_offset":47164,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":47384,"parent_record_offset":47360,"ancestor_record_offset":47164,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":47360,"parent_record_offset":47296,"ancestor_record_offset":47164,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":47408,"parent_record_offset":47384,"ancestor_record_offset":47164,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":47932,"parent_record_offset":47880,"ancestor_record_offset":47164,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":47524,"parent_record_offset":47500,"ancestor_record_offset":47164,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":47632,"parent_record_offset":47596,"ancestor_record_offset":47164,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":47596,"parent_record_offset":47456,"ancestor_record_offset":47164,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":47880,"parent_record_offset":47456,"ancestor_record_offset":47164,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47332,"parent_record_offset":47296,"ancestor_record_offset":47164,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47456,"parent_record_offset":47432,"ancestor_record_offset":47164,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47556,"parent_record_offset":47456,"ancestor_record_offset":47164,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47500,"parent_record_offset":47456,"ancestor_record_offset":47164,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$1::as_points3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":47432,"parent_record_offset":47408,"ancestor_record_offset":47164,"inlinee":49179,"source_line":202,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97AE
// 首条位置: {"module":609,"inlinee":38830}
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
// 首条位置: {"module":764,"record_offset":860,"rva":14598192,"va":5383307312}
// 编译实例 RVAs: 0xdec030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1516747","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383307312,"raw_hex":"e9cb241700","size":5,"soft_fail":false,"target_va":5384824080,"va":5383307328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 首条位置: {"module":1669,"record_offset":19140,"rva":25010464,"va":5393719584}
// 编译实例 RVAs: 0x17da120
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
// 首条位置: {"module":1669,"record_offset":19316,"rva":25010784,"va":5393719904}
// 编译实例 RVAs: 0x17da260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1669,"record_offset":19496,"parent_record_offset":19316,"ancestor_record_offset":19316,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":19516,"parent_record_offset":19496,"ancestor_record_offset":19316,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":19536,"parent_record_offset":19516,"ancestor_record_offset":19316,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":19560,"parent_record_offset":19536,"ancestor_record_offset":19316,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14257
// 首条位置: {"module":1669,"inlinee":82519}
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
// IPI IDs: 0x1560A
// 首条位置: {"module":1795,"inlinee":87562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":34744,"parent_record_offset":34664,"ancestor_record_offset":34664,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":35044,"parent_record_offset":35008,"ancestor_record_offset":34664,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":35116,"parent_record_offset":35080,"ancestor_record_offset":34664,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":35080,"parent_record_offset":35044,"ancestor_record_offset":34664,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":35240,"parent_record_offset":35204,"ancestor_record_offset":34664,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":35204,"parent_record_offset":34744,"ancestor_record_offset":34664,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":35008,"parent_record_offset":34972,"ancestor_record_offset":34664,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":34972,"parent_record_offset":34936,"ancestor_record_offset":34664,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":34936,"parent_record_offset":34744,"ancestor_record_offset":34664,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":34880,"parent_record_offset":34744,"ancestor_record_offset":34664,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1795,"record_offset":34904,"parent_record_offset":34880,"ancestor_record_offset":34664,"inlinee":82519,"source_line":114,"archive":"_pdb/C/modules/01795.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":712,"rva":14598160,"va":5383307280}
// 编译实例 RVAs: 0xdec010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1516779","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383307280,"raw_hex":"e9eb241700","size":5,"soft_fail":false,"target_va":5384824080,"va":5383307296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15608
// 首条位置: {"module":1795,"inlinee":87560}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1795,"record_offset":35784,"parent_record_offset":34252,"ancestor_record_offset":34252,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":36404,"parent_record_offset":34252,"ancestor_record_offset":34252,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":38600,"parent_record_offset":34252,"ancestor_record_offset":34252,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":38628,"parent_record_offset":34252,"ancestor_record_offset":34252,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":38768,"parent_record_offset":34252,"ancestor_record_offset":34252,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":34292,"parent_record_offset":34252,"ancestor_record_offset":34252,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":35816,"parent_record_offset":34252,"ancestor_record_offset":34252,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":35044,"parent_record_offset":35008,"ancestor_record_offset":34252,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":35116,"parent_record_offset":35080,"ancestor_record_offset":34252,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":35080,"parent_record_offset":35044,"ancestor_record_offset":34252,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":35240,"parent_record_offset":35204,"ancestor_record_offset":34252,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":35204,"parent_record_offset":34744,"ancestor_record_offset":34252,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":35008,"parent_record_offset":34972,"ancestor_record_offset":34252,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":34972,"parent_record_offset":34936,"ancestor_record_offset":34252,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":34936,"parent_record_offset":34744,"ancestor_record_offset":34252,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1795,"record_offset":37056,"parent_record_offset":36944,"ancestor_record_offset":34252,"inlinee":7711,"source_line":1627,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":36996,"parent_record_offset":36972,"ancestor_record_offset":34252,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":36972,"parent_record_offset":36944,"ancestor_record_offset":34252,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":37020,"parent_record_offset":36996,"ancestor_record_offset":34252,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1795,"record_offset":36304,"parent_record_offset":36280,"ancestor_record_offset":34252,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":34524,"parent_record_offset":34480,"ancestor_record_offset":34252,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":35664,"parent_record_offset":34380,"ancestor_record_offset":34252,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1795,"record_offset":36256,"parent_record_offset":36228,"ancestor_record_offset":34252,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1795,"record_offset":36280,"parent_record_offset":36256,"ancestor_record_offset":34252,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":34480,"parent_record_offset":34436,"ancestor_record_offset":34252,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1795,"record_offset":35928,"parent_record_offset":35900,"ancestor_record_offset":34252,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":36856,"parent_record_offset":36792,"ancestor_record_offset":34252,"inlinee":31975,"source_line":2023,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":34436,"parent_record_offset":34380,"ancestor_record_offset":34252,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":36344,"parent_record_offset":36184,"ancestor_record_offset":34252,"inlinee":17577,"source_line":2482,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":36228,"parent_record_offset":36184,"ancestor_record_offset":34252,"inlinee":17576,"source_line":2469,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_f32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":36792,"parent_record_offset":36728,"ancestor_record_offset":34252,"inlinee":31974,"source_line":2178,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":34744,"parent_record_offset":34664,"ancestor_record_offset":34252,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":35984,"parent_record_offset":35956,"ancestor_record_offset":34252,"inlinee":17629,"source_line":2310,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":35956,"parent_record_offset":35928,"ancestor_record_offset":34252,"inlinee":87550,"source_line":2327,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":34380,"parent_record_offset":34336,"ancestor_record_offset":34252,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":35872,"parent_record_offset":35844,"ancestor_record_offset":34252,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::visit_content_seq_ref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":36184,"parent_record_offset":35984,"ancestor_record_offset":34252,"inlinee":17574,"source_line":2043,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":38700,"parent_record_offset":38676,"ancestor_record_offset":34252,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1795,"record_offset":38676,"parent_record_offset":38652,"ancestor_record_offset":34252,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":38724,"parent_record_offset":38700,"ancestor_record_offset":34252,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":34336,"parent_record_offset":34292,"ancestor_record_offset":34252,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":35844,"parent_record_offset":35816,"ancestor_record_offset":34252,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":34880,"parent_record_offset":34744,"ancestor_record_offset":34252,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":34580,"parent_record_offset":34380,"ancestor_record_offset":34252,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":36728,"parent_record_offset":36664,"ancestor_record_offset":34252,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":36944,"parent_record_offset":36856,"ancestor_record_offset":34252,"inlinee":7710,"source_line":225,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":37148,"parent_record_offset":36856,"ancestor_record_offset":34252,"inlinee":17567,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":37120,"parent_record_offset":36856,"ancestor_record_offset":34252,"inlinee":17565,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":37088,"parent_record_offset":36856,"ancestor_record_offset":34252,"inlinee":7709,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1795,"record_offset":34904,"parent_record_offset":34880,"ancestor_record_offset":34252,"inlinee":82519,"source_line":114,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1795,"record_offset":34664,"parent_record_offset":34580,"ancestor_record_offset":34252,"inlinee":87562,"source_line":114,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1560B
// 首条位置: {"module":1795,"inlinee":87563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":42204,"parent_record_offset":39296,"ancestor_record_offset":39296,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":39348,"parent_record_offset":39296,"ancestor_record_offset":39296,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":39664,"parent_record_offset":39640,"ancestor_record_offset":39296,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":39712,"parent_record_offset":39688,"ancestor_record_offset":39296,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":39688,"parent_record_offset":39664,"ancestor_record_offset":39296,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":39640,"parent_record_offset":39616,"ancestor_record_offset":39296,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":39616,"parent_record_offset":39592,"ancestor_record_offset":39296,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":39592,"parent_record_offset":39564,"ancestor_record_offset":39296,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1795,"record_offset":40548,"parent_record_offset":40436,"ancestor_record_offset":39296,"inlinee":7711,"source_line":1627,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":40488,"parent_record_offset":40464,"ancestor_record_offset":39296,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":40464,"parent_record_offset":40436,"ancestor_record_offset":39296,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":40512,"parent_record_offset":40488,"ancestor_record_offset":39296,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1795,"record_offset":39880,"parent_record_offset":39856,"ancestor_record_offset":39296,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":39460,"parent_record_offset":39436,"ancestor_record_offset":39296,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":39968,"parent_record_offset":39404,"ancestor_record_offset":39296,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1795,"record_offset":39832,"parent_record_offset":39804,"ancestor_record_offset":39296,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1795,"record_offset":39856,"parent_record_offset":39832,"ancestor_record_offset":39296,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":39436,"parent_record_offset":39404,"ancestor_record_offset":39296,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1795,"record_offset":39516,"parent_record_offset":39492,"ancestor_record_offset":39296,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":40328,"parent_record_offset":40284,"ancestor_record_offset":39296,"inlinee":31975,"source_line":2023,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":39920,"parent_record_offset":39760,"ancestor_record_offset":39296,"inlinee":17577,"source_line":2482,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":39804,"parent_record_offset":39760,"ancestor_record_offset":39296,"inlinee":17576,"source_line":2469,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_f32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":40284,"parent_record_offset":40240,"ancestor_record_offset":39296,"inlinee":31974,"source_line":2178,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":39564,"parent_record_offset":39540,"ancestor_record_offset":39296,"inlinee":17629,"source_line":2310,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":39540,"parent_record_offset":39516,"ancestor_record_offset":39296,"inlinee":87550,"source_line":2327,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":39404,"parent_record_offset":39376,"ancestor_record_offset":39296,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::visit_content_seq_ref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":39760,"parent_record_offset":39564,"ancestor_record_offset":39296,"inlinee":17574,"source_line":2043,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":39376,"parent_record_offset":39348,"ancestor_record_offset":39296,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":39492,"parent_record_offset":39404,"ancestor_record_offset":39296,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":40240,"parent_record_offset":40196,"ancestor_record_offset":39296,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":40436,"parent_record_offset":40328,"ancestor_record_offset":39296,"inlinee":7710,"source_line":225,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":40696,"parent_record_offset":40328,"ancestor_record_offset":39296,"inlinee":17567,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":40640,"parent_record_offset":40328,"ancestor_record_offset":39296,"inlinee":17565,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":40408,"parent_record_offset":40328,"ancestor_record_offset":39296,"inlinee":17563,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":40668,"parent_record_offset":40328,"ancestor_record_offset":39296,"inlinee":17566,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":40580,"parent_record_offset":40328,"ancestor_record_offset":39296,"inlinee":17564,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":40608,"parent_record_offset":40328,"ancestor_record_offset":39296,"inlinee":7709,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
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
// 首条位置: {"module":38,"record_offset":47216,"rva":1663712,"va":5370372832}
// 编译实例 RVAs: 0x1962e0, 0x17db940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1454874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370372832,"raw_hex":"e81a331600","size":5,"soft_fail":false,"target_va":5371827984,"va":5370373105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2942710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370372832,"raw_hex":"e8f6e62c00","size":5,"soft_fail":false,"target_va":5373315680,"va":5370372965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140466530
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2949633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370372832,"raw_hex":"e801022d00","size":5,"soft_fail":false,"target_va":5373322544,"va":5370372906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9870
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t909025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393725760,"raw_hex":"e8e1de0d00","size":5,"soft_fail":false,"target_va":5394634864,"va":5393725834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t910774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393725760,"raw_hex":"e8b6e50d00","size":5,"soft_fail":false,"target_va":5394636672,"va":5393725893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45979888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370372832,"raw_hex":"e8f098bd02","size":5,"soft_fail":false,"target_va":5416353040,"va":5370373147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t46184801","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370372832,"raw_hex":"e961b9c002","size":5,"soft_fail":false,"target_va":5416557744,"va":5370372938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":47484,"parent_record_offset":47216,"ancestor_record_offset":47216,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":48152,"parent_record_offset":47216,"ancestor_record_offset":47216,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":48028,"parent_record_offset":47216,"ancestor_record_offset":47216,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":47880,"parent_record_offset":47848,"ancestor_record_offset":47216,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":47748,"parent_record_offset":47720,"ancestor_record_offset":47216,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":47808,"parent_record_offset":47776,"ancestor_record_offset":47216,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":47928,"parent_record_offset":47904,"ancestor_record_offset":47216,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":47904,"parent_record_offset":47880,"ancestor_record_offset":47216,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":47776,"parent_record_offset":47748,"ancestor_record_offset":47216,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":47680,"parent_record_offset":47652,"ancestor_record_offset":47216,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":47848,"parent_record_offset":47680,"ancestor_record_offset":47216,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":47624,"parent_record_offset":47596,"ancestor_record_offset":47216,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":47968,"parent_record_offset":47680,"ancestor_record_offset":47216,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":47720,"parent_record_offset":47680,"ancestor_record_offset":47216,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":38,"record_offset":47652,"parent_record_offset":47624,"ancestor_record_offset":47216,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":47540,"parent_record_offset":47512,"ancestor_record_offset":47216,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":48204,"parent_record_offset":48180,"ancestor_record_offset":47216,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":47512,"parent_record_offset":47484,"ancestor_record_offset":47216,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":48180,"parent_record_offset":48152,"ancestor_record_offset":47216,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":38,"record_offset":47596,"parent_record_offset":47568,"ancestor_record_offset":47216,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":38,"record_offset":47568,"parent_record_offset":47540,"ancestor_record_offset":47216,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00038.jsonl.gz"}
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
// 首条位置: {"module":1669,"record_offset":41120,"rva":25016976,"va":5393726096}
// 编译实例 RVAs: 0x17dba90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10498582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393726096,"raw_hex":"e8eacd5fff","size":5,"soft_fail":false,"target_va":5383227792,"va":5393726369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417db6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393726096,"raw_hex":"e830fbffff","size":5,"soft_fail":false,"target_va":5393725088,"va":5393726315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b84570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3836513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393726096,"raw_hex":"e8618a3a00","size":5,"soft_fail":false,"target_va":5397562736,"va":5393726218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b87af0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3850245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393726096,"raw_hex":"e805c03a00","size":5,"soft_fail":false,"target_va":5397576432,"va":5393726182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":41288,"parent_record_offset":41120,"ancestor_record_offset":41120,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$12::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89A2
// 首条位置: {"module":561,"inlinee":35234}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":32208,"parent_record_offset":32172,"ancestor_record_offset":32172,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C34
// 首条位置: {"module":478,"inlinee":27700}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$9::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x706A
// 首条位置: {"module":503,"record_offset":120192,"rva":9486496,"va":5378195616}
// 编译实例 RVAs: 0x90c0a0, 0xa7cec0, 0xacce20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field4_finish
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6626048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378195616,"raw_hex":"e8001b6500","size":5,"soft_fail":false,"target_va":5384821872,"va":5378195819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 首条位置: {"module":764,"record_offset":31212,"rva":14608816,"va":5383317936}
// 编译实例 RVAs: 0xdee9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33304187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383317936,"raw_hex":"e87b2efc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383318016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":31368,"parent_record_offset":31212,"ancestor_record_offset":31212,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":31420,"parent_record_offset":31212,"ancestor_record_offset":31212,"inlinee":49174,"source_line":141,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::new_axis_aligned
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC015
// 首条位置: {"module":764,"inlinee":49173}
// 编译实例 RVAs: 0xdedbd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33307721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383314384,"raw_hex":"e8493cfc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383314482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":456,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":540,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":49174,"source_line":141,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":416,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC016
// 首条位置: {"module":764,"inlinee":49174}
// 编译实例 RVAs: 0xdef3c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33301537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383320512,"raw_hex":"e82124fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383320666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":39032,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":39084,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":49220,"source_line":605,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":39168,"parent_record_offset":39108,"ancestor_record_offset":38900,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":39140,"parent_record_offset":39108,"ancestor_record_offset":38900,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":39108,"parent_record_offset":39084,"ancestor_record_offset":38900,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=24
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC019
// 首条位置: {"module":764,"inlinee":49177}
// 编译实例 RVAs: 0xdeec40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33303527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383318592,"raw_hex":"e8e72bfc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383318676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":3448,"parent_record_offset":3412,"ancestor_record_offset":3412,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=11
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC01C
// 首条位置: {"module":764,"inlinee":49180}
// 编译实例 RVAs: 0xdec050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33314646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383307344,"raw_hex":"e85657fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383307557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1272,"parent_record_offset":1008,"ancestor_record_offset":1008,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":1428,"parent_record_offset":1008,"ancestor_record_offset":1008,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1160,"parent_record_offset":1008,"ancestor_record_offset":1008,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1240,"parent_record_offset":1008,"ancestor_record_offset":1008,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1184,"parent_record_offset":1008,"ancestor_record_offset":1008,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2116,"parent_record_offset":1924,"ancestor_record_offset":1756,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":1380,"parent_record_offset":1300,"ancestor_record_offset":1008,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":1340,"parent_record_offset":1300,"ancestor_record_offset":1008,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2072,"parent_record_offset":1924,"ancestor_record_offset":1756,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":1456,"parent_record_offset":1428,"ancestor_record_offset":1008,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":1008,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1300,"parent_record_offset":1272,"ancestor_record_offset":1008,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points3
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC034
// 首条位置: {"module":764,"inlinee":49204}
// 编译实例 RVAs: 0xdec130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33314231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383307568,"raw_hex":"e8b755fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383307972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":1660,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1632,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2116,"parent_record_offset":1924,"ancestor_record_offset":1492,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2032,"parent_record_offset":1924,"ancestor_record_offset":1492,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":1992,"parent_record_offset":1924,"ancestor_record_offset":1492,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2072,"parent_record_offset":1924,"ancestor_record_offset":1492,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":1492,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":1492,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":2200,"parent_record_offset":2164,"ancestor_record_offset":1492,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1492,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1924,"parent_record_offset":1892,"ancestor_record_offset":1492,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1892,"parent_record_offset":1756,"ancestor_record_offset":1492,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":2164,"parent_record_offset":1756,"ancestor_record_offset":1492,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1492,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1852,"parent_record_offset":1756,"ancestor_record_offset":1492,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1796,"parent_record_offset":1756,"ancestor_record_offset":1492,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$1::as_points3::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":1492,"inlinee":49179,"source_line":202,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::as_points3::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC01B
// 首条位置: {"module":764,"inlinee":49179}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1732,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2116,"parent_record_offset":1924,"ancestor_record_offset":1732,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2032,"parent_record_offset":1924,"ancestor_record_offset":1732,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":1992,"parent_record_offset":1924,"ancestor_record_offset":1732,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2072,"parent_record_offset":1924,"ancestor_record_offset":1732,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":2200,"parent_record_offset":2164,"ancestor_record_offset":1732,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1732,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1924,"parent_record_offset":1892,"ancestor_record_offset":1732,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1892,"parent_record_offset":1756,"ancestor_record_offset":1732,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":2164,"parent_record_offset":1756,"ancestor_record_offset":1732,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1852,"parent_record_offset":1756,"ancestor_record_offset":1732,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":1796,"parent_record_offset":1756,"ancestor_record_offset":1732,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":41176,"rva":14612512,"va":5383321632}
// 编译实例 RVAs: 0xdef820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7643058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383321632,"raw_hex":"e84e608bff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383321741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7643616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383321632,"raw_hex":"e8205e8bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383322315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14599539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383321632,"raw_hex":"e88d3a21ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383321726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9746438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383321632,"raw_hex":"e8fa476bff","size":5,"soft_fail":false,"target_va":5373575632,"va":5383322065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33298584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383321632,"raw_hex":"e89818fc01","size":5,"soft_fail":false,"target_va":5416620769,"va":5383322180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33058207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383321632,"raw_hex":"e89f6df801","size":5,"soft_fail":false,"target_va":5416380224,"va":5383322012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33300038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383321632,"raw_hex":"e8461efc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383322165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33299395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383321632,"raw_hex":"e8c31bfc01","size":5,"soft_fail":false,"target_va":5416621632,"va":5383322232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":42148,"parent_record_offset":41176,"ancestor_record_offset":41176,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":764,"record_offset":41808,"parent_record_offset":41176,"ancestor_record_offset":41176,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":42420,"parent_record_offset":41176,"ancestor_record_offset":41176,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":764,"record_offset":42352,"parent_record_offset":41176,"ancestor_record_offset":41176,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":41332,"parent_record_offset":41176,"ancestor_record_offset":41176,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":41360,"parent_record_offset":41176,"ancestor_record_offset":41176,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":41988,"parent_record_offset":41964,"ancestor_record_offset":41176,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":42012,"parent_record_offset":41988,"ancestor_record_offset":41176,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":42608,"parent_record_offset":42584,"ancestor_record_offset":41176,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":42632,"parent_record_offset":42608,"ancestor_record_offset":41176,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":42036,"parent_record_offset":42012,"ancestor_record_offset":41176,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":42656,"parent_record_offset":42632,"ancestor_record_offset":41176,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":41964,"parent_record_offset":41932,"ancestor_record_offset":41176,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":42584,"parent_record_offset":42492,"ancestor_record_offset":41176,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":41932,"parent_record_offset":41888,"ancestor_record_offset":41176,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":41888,"parent_record_offset":41848,"ancestor_record_offset":41176,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":42228,"parent_record_offset":42204,"ancestor_record_offset":41176,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":42520,"parent_record_offset":42492,"ancestor_record_offset":41176,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":42492,"parent_record_offset":42468,"ancestor_record_offset":41176,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":42276,"parent_record_offset":42252,"ancestor_record_offset":41176,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":42252,"parent_record_offset":42228,"ancestor_record_offset":41176,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":42468,"parent_record_offset":42444,"ancestor_record_offset":41176,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":764,"record_offset":42084,"parent_record_offset":41848,"ancestor_record_offset":41176,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":42204,"parent_record_offset":42172,"ancestor_record_offset":41176,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":42108,"parent_record_offset":42084,"ancestor_record_offset":41176,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":42172,"parent_record_offset":42148,"ancestor_record_offset":41176,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":41848,"parent_record_offset":41808,"ancestor_record_offset":41176,"inlinee":49203,"source_line":4346,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":41660,"parent_record_offset":41496,"ancestor_record_offset":41176,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":41584,"parent_record_offset":41496,"ancestor_record_offset":41176,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":41548,"parent_record_offset":41496,"ancestor_record_offset":41176,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":41620,"parent_record_offset":41496,"ancestor_record_offset":41176,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":764,"record_offset":42552,"parent_record_offset":42520,"ancestor_record_offset":41176,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":41740,"parent_record_offset":41704,"ancestor_record_offset":41176,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":42444,"parent_record_offset":42420,"ancestor_record_offset":41176,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":42316,"parent_record_offset":42172,"ancestor_record_offset":41176,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":764,"record_offset":42380,"parent_record_offset":42352,"ancestor_record_offset":41176,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":41496,"parent_record_offset":41468,"ancestor_record_offset":41176,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":41468,"parent_record_offset":41360,"ancestor_record_offset":41176,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":41704,"parent_record_offset":41360,"ancestor_record_offset":41176,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":41436,"parent_record_offset":41360,"ancestor_record_offset":41176,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":41408,"parent_record_offset":41360,"ancestor_record_offset":41176,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":10064,"rva":14601824,"va":5383310944}
// 编译实例 RVAs: 0xdece60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7632370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310944,"raw_hex":"e80e8a8bff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383311053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7632848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310944,"raw_hex":"e830888bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383311547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14588851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310944,"raw_hex":"e84d6421ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383311038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9735749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310944,"raw_hex":"e8bb716bff","size":5,"soft_fail":false,"target_va":5373575632,"va":5383311376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33309312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310944,"raw_hex":"e88042fc01","size":5,"soft_fail":false,"target_va":5416620769,"va":5383311452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33068895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310944,"raw_hex":"e85f97f801","size":5,"soft_fail":false,"target_va":5416380224,"va":5383311324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33310766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383310944,"raw_hex":"e82e48fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383311437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":11036,"parent_record_offset":10064,"ancestor_record_offset":10064,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":764,"record_offset":10696,"parent_record_offset":10064,"ancestor_record_offset":10064,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":11240,"parent_record_offset":10064,"ancestor_record_offset":10064,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":10220,"parent_record_offset":10064,"ancestor_record_offset":10064,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":10248,"parent_record_offset":10064,"ancestor_record_offset":10064,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":10064,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":10900,"parent_record_offset":10876,"ancestor_record_offset":10064,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":10064,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":11452,"parent_record_offset":11428,"ancestor_record_offset":10064,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":10924,"parent_record_offset":10900,"ancestor_record_offset":10064,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":10064,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":10852,"parent_record_offset":10820,"ancestor_record_offset":10064,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":11404,"parent_record_offset":11312,"ancestor_record_offset":10064,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":10820,"parent_record_offset":10776,"ancestor_record_offset":10064,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":10776,"parent_record_offset":10736,"ancestor_record_offset":10064,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":11116,"parent_record_offset":11092,"ancestor_record_offset":10064,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":11340,"parent_record_offset":11312,"ancestor_record_offset":10064,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":11312,"parent_record_offset":11288,"ancestor_record_offset":10064,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":11164,"parent_record_offset":11140,"ancestor_record_offset":10064,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":11140,"parent_record_offset":11116,"ancestor_record_offset":10064,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":10064,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":764,"record_offset":10972,"parent_record_offset":10736,"ancestor_record_offset":10064,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":11092,"parent_record_offset":11060,"ancestor_record_offset":10064,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":10064,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":11060,"parent_record_offset":11036,"ancestor_record_offset":10064,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":10736,"parent_record_offset":10696,"ancestor_record_offset":10064,"inlinee":49203,"source_line":4346,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":10548,"parent_record_offset":10384,"ancestor_record_offset":10064,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":10472,"parent_record_offset":10384,"ancestor_record_offset":10064,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":10436,"parent_record_offset":10384,"ancestor_record_offset":10064,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":10508,"parent_record_offset":10384,"ancestor_record_offset":10064,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":764,"record_offset":11372,"parent_record_offset":11340,"ancestor_record_offset":10064,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":10628,"parent_record_offset":10592,"ancestor_record_offset":10064,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":10064,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":11204,"parent_record_offset":11060,"ancestor_record_offset":10064,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":10384,"parent_record_offset":10356,"ancestor_record_offset":10064,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":10356,"parent_record_offset":10248,"ancestor_record_offset":10064,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":10592,"parent_record_offset":10248,"ancestor_record_offset":10064,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":10324,"parent_record_offset":10248,"ancestor_record_offset":10064,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":10296,"parent_record_offset":10248,"ancestor_record_offset":10064,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":42716,"rva":14613248,"va":5383322368}
// 编译实例 RVAs: 0xdefb00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7643807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322368,"raw_hex":"e8615d8bff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383322490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7644461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322368,"raw_hex":"e8d35a8bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383323160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14600288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322368,"raw_hex":"e8a03721ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383322475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141499460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6985564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322368,"raw_hex":"e85c976a00","size":5,"soft_fail":false,"target_va":5390308448,"va":5383322879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33029243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322368,"raw_hex":"e87bfcf701","size":5,"soft_fail":false,"target_va":5416352064,"va":5383322816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33297765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322368,"raw_hex":"e86515fc01","size":5,"soft_fail":false,"target_va":5416620769,"va":5383322999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33299219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322368,"raw_hex":"e8131bfc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383322984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33298576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322368,"raw_hex":"e89018fc01","size":5,"soft_fail":false,"target_va":5416621632,"va":5383323051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":43960,"parent_record_offset":42716,"ancestor_record_offset":42716,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":764,"record_offset":43644,"parent_record_offset":42716,"ancestor_record_offset":42716,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":44232,"parent_record_offset":42716,"ancestor_record_offset":42716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":764,"record_offset":44164,"parent_record_offset":42716,"ancestor_record_offset":42716,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":42872,"parent_record_offset":42716,"ancestor_record_offset":42716,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":42900,"parent_record_offset":42716,"ancestor_record_offset":42716,"inlinee":49204,"source_line":199,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":43800,"parent_record_offset":43776,"ancestor_record_offset":42716,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":43824,"parent_record_offset":43800,"ancestor_record_offset":42716,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":44420,"parent_record_offset":44396,"ancestor_record_offset":42716,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":44444,"parent_record_offset":44420,"ancestor_record_offset":42716,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":43848,"parent_record_offset":43824,"ancestor_record_offset":42716,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":44468,"parent_record_offset":44444,"ancestor_record_offset":42716,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":43776,"parent_record_offset":43744,"ancestor_record_offset":42716,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":44396,"parent_record_offset":44304,"ancestor_record_offset":42716,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":43744,"parent_record_offset":43708,"ancestor_record_offset":42716,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":43708,"parent_record_offset":43676,"ancestor_record_offset":42716,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":44040,"parent_record_offset":44016,"ancestor_record_offset":42716,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":44332,"parent_record_offset":44304,"ancestor_record_offset":42716,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":44304,"parent_record_offset":44280,"ancestor_record_offset":42716,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":44088,"parent_record_offset":44064,"ancestor_record_offset":42716,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":44064,"parent_record_offset":44040,"ancestor_record_offset":42716,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":44280,"parent_record_offset":44256,"ancestor_record_offset":42716,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":764,"record_offset":43896,"parent_record_offset":43676,"ancestor_record_offset":42716,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":44016,"parent_record_offset":43984,"ancestor_record_offset":42716,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":43920,"parent_record_offset":43896,"ancestor_record_offset":42716,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":43984,"parent_record_offset":43960,"ancestor_record_offset":42716,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":43676,"parent_record_offset":43644,"ancestor_record_offset":42716,"inlinee":49203,"source_line":4346,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":43448,"parent_record_offset":43244,"ancestor_record_offset":42716,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":43364,"parent_record_offset":43244,"ancestor_record_offset":42716,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":43324,"parent_record_offset":43244,"ancestor_record_offset":42716,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":43404,"parent_record_offset":43244,"ancestor_record_offset":42716,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":42956,"parent_record_offset":42928,"ancestor_record_offset":42716,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":42928,"parent_record_offset":42900,"ancestor_record_offset":42716,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":42984,"parent_record_offset":42956,"ancestor_record_offset":42716,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":764,"record_offset":44364,"parent_record_offset":44332,"ancestor_record_offset":42716,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":43544,"parent_record_offset":43496,"ancestor_record_offset":42716,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":44256,"parent_record_offset":44232,"ancestor_record_offset":42716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":44128,"parent_record_offset":43984,"ancestor_record_offset":42716,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":764,"record_offset":44192,"parent_record_offset":44164,"ancestor_record_offset":42716,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":43120,"parent_record_offset":43092,"ancestor_record_offset":42716,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":43244,"parent_record_offset":43196,"ancestor_record_offset":42716,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":43196,"parent_record_offset":43040,"ancestor_record_offset":42716,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":43496,"parent_record_offset":43040,"ancestor_record_offset":42716,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":43040,"parent_record_offset":43012,"ancestor_record_offset":42716,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":43152,"parent_record_offset":43040,"ancestor_record_offset":42716,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":43092,"parent_record_offset":43040,"ancestor_record_offset":42716,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$1::as_points3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":43012,"parent_record_offset":42984,"ancestor_record_offset":42716,"inlinee":49179,"source_line":202,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::area
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=22
// 源码行: 237, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC017
// 首条位置: {"module":764,"inlinee":49175}
// 编译实例 RVAs: 0xdef4f0
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=24
// 源码行: 241, 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC02F
// 首条位置: {"module":764,"inlinee":49199}
// 编译实例 RVAs: 0xdeff20
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=38
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC018
// 首条位置: {"module":764,"inlinee":49176}
// 编译实例 RVAs: 0xdedc50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":1184,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":21764,"rva":14605424,"va":5383314544}
// 编译实例 RVAs: 0xdedc70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":764,"record_offset":21996,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":48855,"source_line":94,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":21968,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::swizzlers::impl$1::xvy
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/swizzlers.rs
//     evidence={"module":764,"record_offset":21916,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":49213,"source_line":18,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":764,"record_offset":21940,"parent_record_offset":21916,"ancestor_record_offset":21764,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_transformation_and_size
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC01D
// 首条位置: {"module":764,"inlinee":49181}
// 编译实例 RVAs: 0xdeea20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":2472,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":2528,"parent_record_offset":2500,"ancestor_record_offset":2472,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=17
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC031
// 首条位置: {"module":764,"inlinee":49201}
// 编译实例 RVAs: 0xdefe50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33298815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383323216,"raw_hex":"e87f19fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383323388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":6844,"parent_record_offset":6816,"ancestor_record_offset":6816,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":9204,"parent_record_offset":8876,"ancestor_record_offset":8876,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":6536,"parent_record_offset":6504,"ancestor_record_offset":6504,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":6600,"parent_record_offset":6504,"ancestor_record_offset":6504,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":6980,"parent_record_offset":6888,"ancestor_record_offset":6816,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":6920,"parent_record_offset":6888,"ancestor_record_offset":6816,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":35260,"parent_record_offset":35224,"ancestor_record_offset":35144,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":6948,"parent_record_offset":6888,"ancestor_record_offset":6816,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":7012,"parent_record_offset":6844,"ancestor_record_offset":6816,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":6624,"parent_record_offset":6600,"ancestor_record_offset":6504,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":6888,"parent_record_offset":6844,"ancestor_record_offset":6816,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":6568,"parent_record_offset":6536,"ancestor_record_offset":6504,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":16552,"rva":14604320,"va":5383313440}
// 编译实例 RVAs: 0xded820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383313440,"raw_hex":"e903000000","size":5,"soft_fail":false,"target_va":5383313456,"va":5383313448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 首条位置: {"module":764,"record_offset":38744,"rva":14611376,"va":5383320496}
// 编译实例 RVAs: 0xdef3b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-7053","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383320496,"raw_hex":"e973e4ffff","size":5,"soft_fail":false,"target_va":5383313456,"va":5383320504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_corner_unnormal
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 366
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC03A
// 首条位置: {"module":764,"inlinee":49210}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17492,"parent_record_offset":17468,"ancestor_record_offset":17468,"inlinee":49208,"source_line":450,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":23188,"parent_record_offset":23064,"ancestor_record_offset":22968,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":17620,"parent_record_offset":17596,"ancestor_record_offset":17468,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":23100,"parent_record_offset":23064,"ancestor_record_offset":22968,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":23156,"parent_record_offset":23064,"ancestor_record_offset":22968,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":17468,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17596,"parent_record_offset":17572,"ancestor_record_offset":17468,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17572,"parent_record_offset":17492,"ancestor_record_offset":17468,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17516,"parent_record_offset":17492,"ancestor_record_offset":17468,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_corner_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC039
// 首条位置: {"module":764,"inlinee":49209}
// 编译实例 RVAs: 0xdede00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33307072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383314944,"raw_hex":"e8c039fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383315131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":17948,"parent_record_offset":17668,"ancestor_record_offset":17668,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":17716,"parent_record_offset":17668,"ancestor_record_offset":17668,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":18004,"parent_record_offset":17668,"ancestor_record_offset":17668,"inlinee":49208,"source_line":450,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_corner_unnormal
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17468,"parent_record_offset":17444,"ancestor_record_offset":17444,"inlinee":49210,"source_line":366,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":23188,"parent_record_offset":23064,"ancestor_record_offset":22820,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":17620,"parent_record_offset":17596,"ancestor_record_offset":17444,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":23100,"parent_record_offset":23064,"ancestor_record_offset":22820,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":23156,"parent_record_offset":23064,"ancestor_record_offset":22820,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":17872,"parent_record_offset":17848,"ancestor_record_offset":17668,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":17912,"parent_record_offset":17744,"ancestor_record_offset":17668,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":17444,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17596,"parent_record_offset":17572,"ancestor_record_offset":17444,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":17572,"parent_record_offset":17492,"ancestor_record_offset":17444,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":17812,"parent_record_offset":17776,"ancestor_record_offset":17668,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":17776,"parent_record_offset":17744,"ancestor_record_offset":17668,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":17668,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":17848,"parent_record_offset":17812,"ancestor_record_offset":17668,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17492,"parent_record_offset":17468,"ancestor_record_offset":17444,"inlinee":49208,"source_line":450,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":17516,"parent_record_offset":17492,"ancestor_record_offset":17444,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_pts
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC02E
// 首条位置: {"module":764,"inlinee":49198}
// 编译实例 RVAs: 0xdec820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33312580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383309344,"raw_hex":"e8444ffc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383309623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":4532,"parent_record_offset":4484,"ancestor_record_offset":4484,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":4484,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4596,"parent_record_offset":4532,"ancestor_record_offset":4484,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":4572,"parent_record_offset":4532,"ancestor_record_offset":4484,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 420
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC03B
// 首条位置: {"module":764,"inlinee":49211}
// 编译实例 RVAs: 0xded7a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33308774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383313312,"raw_hex":"e86640fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383313429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":16212,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":16316,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":49208,"source_line":450,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":16260,"parent_record_offset":16236,"ancestor_record_offset":16068,"inlinee":6830,"source_line":2141,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":16280,"parent_record_offset":16260,"ancestor_record_offset":16068,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":16508,"parent_record_offset":16420,"ancestor_record_offset":16068,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":16448,"parent_record_offset":16420,"ancestor_record_offset":16068,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":16476,"parent_record_offset":16420,"ancestor_record_offset":16068,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":16236,"parent_record_offset":16212,"ancestor_record_offset":16068,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":16364,"parent_record_offset":16340,"ancestor_record_offset":16068,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":16068,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":16396,"parent_record_offset":16316,"ancestor_record_offset":16068,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":16340,"parent_record_offset":16316,"ancestor_record_offset":16068,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC038
// 首条位置: {"module":764,"inlinee":49208}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":16396,"parent_record_offset":16316,"ancestor_record_offset":16316,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":16340,"parent_record_offset":16316,"ancestor_record_offset":16316,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":16508,"parent_record_offset":16420,"ancestor_record_offset":16316,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":16448,"parent_record_offset":16420,"ancestor_record_offset":16316,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":23100,"parent_record_offset":23064,"ancestor_record_offset":23008,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":16476,"parent_record_offset":16420,"ancestor_record_offset":16316,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":16364,"parent_record_offset":16340,"ancestor_record_offset":16316,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":16316,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::contains_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 454, 455
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC045
// 首条位置: {"module":764,"inlinee":49221}
// 编译实例 RVAs: 0xded740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":15648,"parent_record_offset":15508,"ancestor_record_offset":15508,"inlinee":49205,"source_line":459,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":15912,"parent_record_offset":15820,"ancestor_record_offset":15508,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":15508,"inlinee":49207,"source_line":530,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":15852,"parent_record_offset":15820,"ancestor_record_offset":15508,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":15880,"parent_record_offset":15820,"ancestor_record_offset":15508,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":15764,"parent_record_offset":15740,"ancestor_record_offset":15508,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/sse2/bvec4a.rs
//     evidence={"module":764,"record_offset":16032,"parent_record_offset":15648,"ancestor_record_offset":15508,"inlinee":27913,"source_line":91,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":15508,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":15820,"parent_record_offset":15792,"ancestor_record_offset":15508,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":15792,"parent_record_offset":15648,"ancestor_record_offset":15508,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":764,"record_offset":15976,"parent_record_offset":15648,"ancestor_record_offset":15508,"inlinee":49206,"source_line":453,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":764,"record_offset":15948,"parent_record_offset":15648,"ancestor_record_offset":15508,"inlinee":11831,"source_line":110,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":15740,"parent_record_offset":15648,"ancestor_record_offset":15508,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":15684,"parent_record_offset":15648,"ancestor_record_offset":15508,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::contains_point_padding
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 459
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC035
// 首条位置: {"module":764,"inlinee":49205}
// 编译实例 RVAs: 0xdee530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/sse2/bvec4a.rs
//     evidence={"module":764,"record_offset":16032,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":27913,"source_line":91,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":15792,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":764,"record_offset":15976,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":49206,"source_line":453,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":764,"record_offset":15948,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":11831,"source_line":110,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":15740,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":15684,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":15912,"parent_record_offset":15820,"ancestor_record_offset":15648,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":15648,"inlinee":49207,"source_line":530,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":15852,"parent_record_offset":15820,"ancestor_record_offset":15648,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":15880,"parent_record_offset":15820,"ancestor_record_offset":15648,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":15764,"parent_record_offset":15740,"ancestor_record_offset":15648,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":15648,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":15820,"parent_record_offset":15792,"ancestor_record_offset":15648,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":24924,"rva":14606800,"va":5383315920}
// 编译实例 RVAs: 0xdee1d0
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
// 首条位置: {"module":764,"record_offset":30576,"rva":14608624,"va":5383317744}
// 编译实例 RVAs: 0xdee8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bc10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33739378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383317744,"raw_hex":"e872d20202","size":5,"soft_fail":false,"target_va":5417057296,"va":5383317913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":31180,"parent_record_offset":30576,"ancestor_record_offset":30576,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":30732,"parent_record_offset":30576,"ancestor_record_offset":30576,"inlinee":49219,"source_line":485,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":30952,"parent_record_offset":30828,"ancestor_record_offset":30576,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":30892,"parent_record_offset":30828,"ancestor_record_offset":30576,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":30860,"parent_record_offset":30828,"ancestor_record_offset":30576,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":31120,"parent_record_offset":30732,"ancestor_record_offset":30576,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":31036,"parent_record_offset":31012,"ancestor_record_offset":30576,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":30796,"parent_record_offset":30772,"ancestor_record_offset":30576,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":30828,"parent_record_offset":30732,"ancestor_record_offset":30576,"inlinee":37204,"source_line":308,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":31064,"parent_record_offset":30732,"ancestor_record_offset":30576,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":31012,"parent_record_offset":30732,"ancestor_record_offset":30576,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":31148,"parent_record_offset":30732,"ancestor_record_offset":30576,"inlinee":14783,"source_line":675,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":30772,"parent_record_offset":30732,"ancestor_record_offset":30576,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 485
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC043
// 首条位置: {"module":764,"inlinee":49219}
// 编译实例 RVAs: 0xdef2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x142e1bc10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t33736804","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383320304,"raw_hex":"e964c80202","size":5,"soft_fail":false,"target_va":5417057296,"va":5383320487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":31120,"parent_record_offset":30732,"ancestor_record_offset":30732,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":30828,"parent_record_offset":30732,"ancestor_record_offset":30732,"inlinee":37204,"source_line":308,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":31064,"parent_record_offset":30732,"ancestor_record_offset":30732,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":31012,"parent_record_offset":30732,"ancestor_record_offset":30732,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":31148,"parent_record_offset":30732,"ancestor_record_offset":30732,"inlinee":14783,"source_line":675,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":30772,"parent_record_offset":30732,"ancestor_record_offset":30732,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":30952,"parent_record_offset":30828,"ancestor_record_offset":30732,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":30892,"parent_record_offset":30828,"ancestor_record_offset":30732,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":30860,"parent_record_offset":30828,"ancestor_record_offset":30732,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":31036,"parent_record_offset":31012,"ancestor_record_offset":30732,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":30796,"parent_record_offset":30772,"ancestor_record_offset":30732,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":25060,"rva":14606832,"va":5383315952}
// 编译实例 RVAs: 0xdee1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":25636,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":25276,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":37204,"source_line":308,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":25512,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":25460,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":25220,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":25604,"parent_record_offset":25512,"ancestor_record_offset":25060,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":25400,"parent_record_offset":25276,"ancestor_record_offset":25060,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":25340,"parent_record_offset":25276,"ancestor_record_offset":25060,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":25308,"parent_record_offset":25276,"ancestor_record_offset":25060,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":25484,"parent_record_offset":25460,"ancestor_record_offset":25060,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":25244,"parent_record_offset":25220,"ancestor_record_offset":25060,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":33680,"rva":14609568,"va":5383318688}
// 编译实例 RVAs: 0xdeeca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33302504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383318688,"raw_hex":"e8e827fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383319699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::distance_segment_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":33836,"parent_record_offset":33680,"ancestor_record_offset":33680,"inlinee":36147,"source_line":169,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":35144,"parent_record_offset":33680,"ancestor_record_offset":33680,"inlinee":49201,"source_line":265,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":35288,"parent_record_offset":35224,"ancestor_record_offset":33680,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":35260,"parent_record_offset":35224,"ancestor_record_offset":33680,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":34428,"parent_record_offset":33896,"ancestor_record_offset":33680,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":34380,"parent_record_offset":33956,"ancestor_record_offset":33680,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":35080,"parent_record_offset":35040,"ancestor_record_offset":33680,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":34736,"parent_record_offset":34696,"ancestor_record_offset":33680,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":34612,"parent_record_offset":34568,"ancestor_record_offset":33680,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":34072,"parent_record_offset":34028,"ancestor_record_offset":33680,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":35224,"parent_record_offset":35188,"ancestor_record_offset":33680,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":35188,"parent_record_offset":35144,"ancestor_record_offset":33680,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34244,"parent_record_offset":34204,"ancestor_record_offset":33680,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34916,"parent_record_offset":34876,"ancestor_record_offset":33680,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34528,"parent_record_offset":34488,"ancestor_record_offset":33680,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34568,"parent_record_offset":34528,"ancestor_record_offset":33680,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34696,"parent_record_offset":34528,"ancestor_record_offset":33680,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":34028,"parent_record_offset":33956,"ancestor_record_offset":33680,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":35040,"parent_record_offset":34956,"ancestor_record_offset":33680,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":35712,"parent_record_offset":35416,"ancestor_record_offset":33680,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":34204,"parent_record_offset":33956,"ancestor_record_offset":33680,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":34876,"parent_record_offset":33836,"ancestor_record_offset":33680,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":34488,"parent_record_offset":33896,"ancestor_record_offset":33680,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":33896,"parent_record_offset":33836,"ancestor_record_offset":33680,"inlinee":26199,"source_line":184,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":33956,"parent_record_offset":33896,"ancestor_record_offset":33680,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":25792,"rva":14607040,"va":5383316160}
// 编译实例 RVAs: 0xdee2c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t2523","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383316160,"raw_hex":"e9db090000","size":5,"soft_fail":false,"target_va":5383318688,"va":5383316160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 首条位置: {"module":764,"record_offset":31656,"rva":14608992,"va":5383318112}
// 编译实例 RVAs: 0xdeea60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33303625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383318112,"raw_hex":"e8492cfc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383318578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383318112,"raw_hex":"e814020000","size":5,"soft_fail":false,"target_va":5383318688,"va":5383318151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":32912,"parent_record_offset":31656,"ancestor_record_offset":31656,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":33256,"parent_record_offset":31656,"ancestor_record_offset":31656,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":31820,"parent_record_offset":31656,"ancestor_record_offset":31656,"inlinee":49201,"source_line":265,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":32624,"parent_record_offset":31656,"ancestor_record_offset":31656,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":32324,"parent_record_offset":32236,"ancestor_record_offset":31656,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":32052,"parent_record_offset":32028,"ancestor_record_offset":31656,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":32292,"parent_record_offset":32236,"ancestor_record_offset":31656,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":32880,"parent_record_offset":32624,"ancestor_record_offset":31656,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":32112,"parent_record_offset":32088,"ancestor_record_offset":31656,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":33312,"parent_record_offset":33284,"ancestor_record_offset":31656,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":32680,"parent_record_offset":32656,"ancestor_record_offset":31656,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":31972,"parent_record_offset":31948,"ancestor_record_offset":31656,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":32028,"parent_record_offset":32004,"ancestor_record_offset":31656,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":32004,"parent_record_offset":31820,"ancestor_record_offset":31656,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":32792,"parent_record_offset":32768,"ancestor_record_offset":31656,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":33284,"parent_record_offset":33256,"ancestor_record_offset":31656,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":32088,"parent_record_offset":31820,"ancestor_record_offset":31656,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":32656,"parent_record_offset":32624,"ancestor_record_offset":31656,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":31916,"parent_record_offset":31872,"ancestor_record_offset":31656,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":31872,"parent_record_offset":31820,"ancestor_record_offset":31656,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":31948,"parent_record_offset":31820,"ancestor_record_offset":31656,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":764,"record_offset":32768,"parent_record_offset":32624,"ancestor_record_offset":31656,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::local_aabb_of
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 582
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC040
// 首条位置: {"module":764,"inlinee":49216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":26392,"parent_record_offset":26284,"ancestor_record_offset":26284,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26504,"parent_record_offset":26284,"ancestor_record_offset":26284,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":26708,"parent_record_offset":26284,"ancestor_record_offset":26284,"inlinee":49217,"source_line":643,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":26332,"parent_record_offset":26284,"ancestor_record_offset":26284,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":26672,"parent_record_offset":26584,"ancestor_record_offset":26284,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":26468,"parent_record_offset":26416,"ancestor_record_offset":26284,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":26440,"parent_record_offset":26416,"ancestor_record_offset":26284,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":26640,"parent_record_offset":26584,"ancestor_record_offset":26284,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":26852,"parent_record_offset":26708,"ancestor_record_offset":26284,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26960,"parent_record_offset":26936,"ancestor_record_offset":26284,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":26284,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26528,"parent_record_offset":26504,"ancestor_record_offset":26284,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":26360,"parent_record_offset":26332,"ancestor_record_offset":26284,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":26416,"parent_record_offset":26392,"ancestor_record_offset":26284,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26936,"parent_record_offset":26708,"ancestor_record_offset":26284,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26880,"parent_record_offset":26708,"ancestor_record_offset":26284,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26784,"parent_record_offset":26756,"ancestor_record_offset":26284,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":26756,"parent_record_offset":26708,"ancestor_record_offset":26284,"inlinee":49218,"source_line":638,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::half_dimensions
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 638
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC042
// 首条位置: {"module":764,"inlinee":49218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26784,"parent_record_offset":26756,"ancestor_record_offset":26756,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":26756,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::aabb
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 643
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC041
// 首条位置: {"module":764,"inlinee":49217}
// 编译实例 RVAs: 0xdef490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":26852,"parent_record_offset":26708,"ancestor_record_offset":26708,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26936,"parent_record_offset":26708,"ancestor_record_offset":26708,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26880,"parent_record_offset":26708,"ancestor_record_offset":26708,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":26756,"parent_record_offset":26708,"ancestor_record_offset":26708,"inlinee":49218,"source_line":638,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26960,"parent_record_offset":26936,"ancestor_record_offset":26708,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":26708,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":26904,"parent_record_offset":26880,"ancestor_record_offset":26708,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":26784,"parent_record_offset":26756,"ancestor_record_offset":26708,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":23616,"rva":14606032,"va":5383315152}
// 编译实例 RVAs: 0xdeded0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..array..iter..iter_inner..PolymorphicIter$LT$$u5b$core..mem..maybe_uninit..MaybeUninit$LT$T$GT$$u3b$$u20$N$u5d$$GT$$u20$as$u20$core..clone..Clone$GT$::clone::clone_into_new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17070839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383315152,"raw_hex":"e8f77a0401","size":5,"soft_fail":false,"target_va":5400386272,"va":5383315428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e35b80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17070879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383315152,"raw_hex":"e81f7b0401","size":5,"soft_fail":false,"target_va":5400386432,"va":5383315548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33306300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383315152,"raw_hex":"e8bc36fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383315903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":764,"record_offset":24624,"parent_record_offset":23616,"ancestor_record_offset":23616,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":24440,"parent_record_offset":23616,"ancestor_record_offset":23616,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":764,"record_offset":24164,"parent_record_offset":23616,"ancestor_record_offset":23616,"inlinee":5526,"source_line":813,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::intersect_ray
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":764,"record_offset":24496,"parent_record_offset":23616,"ancestor_record_offset":23616,"inlinee":49151,"source_line":150,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":23776,"parent_record_offset":23616,"ancestor_record_offset":23616,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$13::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":764,"record_offset":24276,"parent_record_offset":24252,"ancestor_record_offset":23616,"inlinee":49190,"source_line":19,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":764,"record_offset":24380,"parent_record_offset":24348,"ancestor_record_offset":23616,"inlinee":49197,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":764,"record_offset":24348,"parent_record_offset":24324,"ancestor_record_offset":23616,"inlinee":49196,"source_line":101,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":24732,"parent_record_offset":24704,"ancestor_record_offset":23616,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":24756,"parent_record_offset":24732,"ancestor_record_offset":23616,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":24048,"parent_record_offset":23968,"ancestor_record_offset":23616,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":24008,"parent_record_offset":23968,"ancestor_record_offset":23616,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":24588,"parent_record_offset":24564,"ancestor_record_offset":23616,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::Cycle::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cycle.rs
//     evidence={"module":764,"record_offset":24252,"parent_record_offset":24228,"ancestor_record_offset":23616,"inlinee":9883,"source_line":21,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::cycle
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":24228,"parent_record_offset":24196,"ancestor_record_offset":23616,"inlinee":9881,"source_line":3572,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":764,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":23616,"inlinee":49192,"source_line":158,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::impl$3::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":764,"record_offset":24324,"parent_record_offset":24300,"ancestor_record_offset":23616,"inlinee":49194,"source_line":75,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":24124,"parent_record_offset":24096,"ancestor_record_offset":23616,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":24464,"parent_record_offset":24440,"ancestor_record_offset":23616,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":23616,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":24680,"parent_record_offset":24652,"ancestor_record_offset":23616,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":23880,"parent_record_offset":23856,"ancestor_record_offset":23616,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":23968,"parent_record_offset":23944,"ancestor_record_offset":23616,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":23944,"parent_record_offset":23776,"ancestor_record_offset":23616,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":24536,"parent_record_offset":24496,"ancestor_record_offset":23616,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":24096,"parent_record_offset":23776,"ancestor_record_offset":23616,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":764,"record_offset":24196,"parent_record_offset":24164,"ancestor_record_offset":23616,"inlinee":9904,"source_line":256,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":24564,"parent_record_offset":24496,"ancestor_record_offset":23616,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":23828,"parent_record_offset":23776,"ancestor_record_offset":23616,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":23912,"parent_record_offset":23776,"ancestor_record_offset":23616,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":23856,"parent_record_offset":23776,"ancestor_record_offset":23616,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$1::intersects_circle::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":24652,"parent_record_offset":24624,"ancestor_record_offset":23616,"inlinee":49215,"source_line":664,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::intersects_circle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 664
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC03F
// 首条位置: {"module":764,"inlinee":49215}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":24680,"parent_record_offset":24652,"ancestor_record_offset":24652,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":24732,"parent_record_offset":24704,"ancestor_record_offset":24652,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":24756,"parent_record_offset":24732,"ancestor_record_offset":24652,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":24652,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":2300,"rva":14598864,"va":5383307984}
// 编译实例 RVAs: 0xdec2d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..array..iter..iter_inner..PolymorphicIter$LT$$u5b$core..mem..maybe_uninit..MaybeUninit$LT$T$GT$$u3b$$u20$N$u5d$$GT$$u20$as$u20$core..clone..Clone$GT$::clone::clone_into_new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17077922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383307984,"raw_hex":"e8a2960401","size":5,"soft_fail":false,"target_va":5400386272,"va":5383308345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e35ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17078284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383307984,"raw_hex":"e80c980401","size":5,"soft_fail":false,"target_va":5400386784,"va":5383308495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33313455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383307984,"raw_hex":"e8af52fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383308748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":4284,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine2::Affine2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":764,"record_offset":2740,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":49182,"source_line":321,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine2::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":764,"record_offset":3040,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":49184,"source_line":409,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":3768,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":764,"record_offset":3480,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":5526,"source_line":813,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":764,"record_offset":2564,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":49104,"source_line":73,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::intersect_ray
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":764,"record_offset":3824,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":49087,"source_line":148,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::RayBoxIntersection::is_hit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":764,"record_offset":4256,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":49089,"source_line":394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":3412,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":49177,"source_line":154,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_transformation_and_size
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":2472,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":49181,"source_line":258,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$13::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":764,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":2300,"inlinee":49190,"source_line":19,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":764,"record_offset":3704,"parent_record_offset":3672,"ancestor_record_offset":2300,"inlinee":49197,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":764,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":2300,"inlinee":49196,"source_line":101,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2892,"parent_record_offset":2768,"ancestor_record_offset":2300,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2832,"parent_record_offset":2768,"ancestor_record_offset":2300,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":764,"record_offset":2800,"parent_record_offset":2768,"ancestor_record_offset":2300,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::Cycle::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cycle.rs
//     evidence={"module":764,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":2300,"inlinee":9883,"source_line":21,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$13::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":764,"record_offset":3576,"parent_record_offset":3552,"ancestor_record_offset":2300,"inlinee":49188,"source_line":60,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::cycle
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":764,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":2300,"inlinee":9881,"source_line":3572,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":764,"record_offset":3624,"parent_record_offset":3600,"ancestor_record_offset":2300,"inlinee":49192,"source_line":158,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::impl$3::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":764,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":2300,"inlinee":49194,"source_line":75,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":2300,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":2300,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":2300,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":2300,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":2528,"parent_record_offset":2500,"ancestor_record_offset":2300,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":2300,"inlinee":37204,"source_line":308,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":2300,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":3076,"parent_record_offset":3040,"ancestor_record_offset":2300,"inlinee":49186,"source_line":523,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":2952,"parent_record_offset":2740,"ancestor_record_offset":2300,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2300,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":2300,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4112,"parent_record_offset":3824,"ancestor_record_offset":2300,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4140,"parent_record_offset":3824,"ancestor_record_offset":2300,"inlinee":36888,"source_line":264,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4084,"parent_record_offset":3824,"ancestor_record_offset":2300,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4196,"parent_record_offset":3824,"ancestor_record_offset":2300,"inlinee":49081,"source_line":251,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$0::max_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":2300,"inlinee":36890,"source_line":265,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$0::min_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":2300,"inlinee":49083,"source_line":252,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":3916,"parent_record_offset":3824,"ancestor_record_offset":2300,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":3352,"parent_record_offset":3040,"ancestor_record_offset":2300,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":3860,"parent_record_offset":3824,"ancestor_record_offset":2300,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":764,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":2300,"inlinee":9904,"source_line":256,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":3448,"parent_record_offset":3412,"ancestor_record_offset":2300,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":2300,"inlinee":49176,"source_line":245,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::intersects::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 684
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x182A8
// 首条位置: {"module":2190,"inlinee":98984}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":2190,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":27806,"source_line":270,"archive":"_pdb/C/modules/02190.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2190,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":3444,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/02190.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2190,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":3444,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02190.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":2190,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":3444,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/02190.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":2190,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":3444,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/02190.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2190,"record_offset":3576,"parent_record_offset":3468,"ancestor_record_offset":3444,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/02190.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":11536,"rva":14602480,"va":5383311600}
// 编译实例 RVAs: 0xded0f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7633003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383311600,"raw_hex":"e895878bff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383311686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7634293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383311600,"raw_hex":"e88b828bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383312992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14589484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383311600,"raw_hex":"e8d46121ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383311671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33308247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383311600,"raw_hex":"e8573efc01","size":5,"soft_fail":false,"target_va":5416620769,"va":5383312517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t33069830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383311600,"raw_hex":"e8069bf801","size":5,"soft_fail":false,"target_va":5416381792,"va":5383311957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t33309672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383311600,"raw_hex":"e8e843fc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383312563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":12088,"parent_record_offset":11536,"ancestor_record_offset":11536,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":11700,"parent_record_offset":11536,"ancestor_record_offset":11536,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":12756,"parent_record_offset":11536,"ancestor_record_offset":11536,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":764,"record_offset":11880,"parent_record_offset":11536,"ancestor_record_offset":11536,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":15044,"parent_record_offset":11536,"ancestor_record_offset":11536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":13572,"parent_record_offset":11536,"ancestor_record_offset":11536,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":12172,"parent_record_offset":11536,"ancestor_record_offset":11536,"inlinee":49200,"source_line":283,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":11536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":11808,"parent_record_offset":11784,"ancestor_record_offset":11536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":11536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":15140,"parent_record_offset":15116,"ancestor_record_offset":11536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":11832,"parent_record_offset":11808,"ancestor_record_offset":11536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":15164,"parent_record_offset":15140,"ancestor_record_offset":11536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":11760,"parent_record_offset":11728,"ancestor_record_offset":11536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":764,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":11536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":11728,"parent_record_offset":11700,"ancestor_record_offset":11536,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":764,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":11536,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":12836,"parent_record_offset":12812,"ancestor_record_offset":11536,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":15312,"parent_record_offset":15284,"ancestor_record_offset":11536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":11536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":11536,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":12860,"parent_record_offset":12836,"ancestor_record_offset":11536,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":764,"record_offset":15260,"parent_record_offset":15236,"ancestor_record_offset":11536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":764,"record_offset":12112,"parent_record_offset":12088,"ancestor_record_offset":11536,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":12812,"parent_record_offset":12780,"ancestor_record_offset":11536,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":11536,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":764,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":11536,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":11536,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":764,"record_offset":12040,"parent_record_offset":12016,"ancestor_record_offset":11536,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":764,"record_offset":15344,"parent_record_offset":15312,"ancestor_record_offset":11536,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":11992,"parent_record_offset":11968,"ancestor_record_offset":11536,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":764,"record_offset":11968,"parent_record_offset":11936,"ancestor_record_offset":11536,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":15236,"parent_record_offset":15212,"ancestor_record_offset":11536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":764,"record_offset":12924,"parent_record_offset":12780,"ancestor_record_offset":11536,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":764,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":11536,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":764,"record_offset":11908,"parent_record_offset":11880,"ancestor_record_offset":11536,"inlinee":27128,"source_line":115,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":12204,"parent_record_offset":12172,"ancestor_record_offset":11536,"inlinee":49199,"source_line":241,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":39660,"rva":14611712,"va":5383320832}
// 编译实例 RVAs: 0xdef500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33736391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383320832,"raw_hex":"e8c7c60202","size":5,"soft_fail":false,"target_va":5417057696,"va":5383321300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33736739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383320832,"raw_hex":"e823c80202","size":5,"soft_fail":false,"target_va":5417058032,"va":5383321288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33300628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383320832,"raw_hex":"e89420fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383321575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":764,"record_offset":40512,"parent_record_offset":39660,"ancestor_record_offset":39660,"inlinee":35242,"source_line":131,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::angle_to
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":39808,"parent_record_offset":39660,"ancestor_record_offset":39660,"inlinee":35770,"source_line":983,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":41004,"parent_record_offset":39660,"ancestor_record_offset":39660,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":764,"record_offset":40372,"parent_record_offset":39660,"ancestor_record_offset":39660,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":40740,"parent_record_offset":39660,"ancestor_record_offset":39660,"inlinee":49174,"source_line":141,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":40244,"parent_record_offset":40220,"ancestor_record_offset":39660,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":40792,"parent_record_offset":40740,"ancestor_record_offset":39660,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":39912,"parent_record_offset":39864,"ancestor_record_offset":39660,"inlinee":11814,"source_line":1593,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":40080,"parent_record_offset":40056,"ancestor_record_offset":39660,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":40480,"parent_record_offset":40372,"ancestor_record_offset":39660,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":40424,"parent_record_offset":40372,"ancestor_record_offset":39660,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":40396,"parent_record_offset":40372,"ancestor_record_offset":39660,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::acos_approx_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":764,"record_offset":40144,"parent_record_offset":40116,"ancestor_record_offset":39660,"inlinee":35773,"source_line":3,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":764,"record_offset":40220,"parent_record_offset":40144,"ancestor_record_offset":39660,"inlinee":11809,"source_line":161,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::acos_approx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":764,"record_offset":40116,"parent_record_offset":39808,"ancestor_record_offset":39660,"inlinee":35771,"source_line":166,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":764,"record_offset":40536,"parent_record_offset":40512,"ancestor_record_offset":39660,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":764,"record_offset":40276,"parent_record_offset":40144,"ancestor_record_offset":39660,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":40000,"parent_record_offset":39948,"ancestor_record_offset":39660,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":40876,"parent_record_offset":40740,"ancestor_record_offset":39660,"inlinee":49220,"source_line":605,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":39948,"parent_record_offset":39808,"ancestor_record_offset":39660,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp_dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":39864,"parent_record_offset":39808,"ancestor_record_offset":39660,"inlinee":13877,"source_line":1008,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":41028,"parent_record_offset":41004,"ancestor_record_offset":39660,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":41112,"parent_record_offset":41004,"ancestor_record_offset":39660,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":40584,"parent_record_offset":40560,"ancestor_record_offset":39660,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":40560,"parent_record_offset":40536,"ancestor_record_offset":39660,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":40056,"parent_record_offset":39948,"ancestor_record_offset":39660,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":4344,"rva":14599664,"va":5383308784}
// 编译实例 RVAs: 0xdec5f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33313118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383308784,"raw_hex":"e85e51fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383309085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4668,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_pts
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":4484,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":49198,"source_line":409,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":4804,"parent_record_offset":4780,"ancestor_record_offset":4344,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":4344,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":4720,"parent_record_offset":4692,"ancestor_record_offset":4344,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4748,"parent_record_offset":4692,"ancestor_record_offset":4344,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4596,"parent_record_offset":4532,"ancestor_record_offset":4344,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":4692,"parent_record_offset":4668,"ancestor_record_offset":4344,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":764,"record_offset":4780,"parent_record_offset":4748,"ancestor_record_offset":4344,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":4572,"parent_record_offset":4532,"ancestor_record_offset":4344,"inlinee":49175,"source_line":237,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":4532,"parent_record_offset":4484,"ancestor_record_offset":4344,"inlinee":49180,"source_line":183,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// 首条位置: {"module":764,"record_offset":260,"rva":14598016,"va":5383307136}
// 编译实例 RVAs: 0xdebf80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33314957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383307136,"raw_hex":"e88d58fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383307246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":764,"record_offset":576,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":49111,"source_line":101,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":764,"record_offset":392,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":49110,"source_line":174,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new_axis_aligned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":416,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":49173,"source_line":134,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":260,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":764,"record_offset":456,"parent_record_offset":416,"ancestor_record_offset":260,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":764,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":260,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":652,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":764,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00764.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":764,"record_offset":540,"parent_record_offset":416,"ancestor_record_offset":260,"inlinee":49174,"source_line":141,"archive":"_pdb/C/modules/00764.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1>", "constructor": "utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points", "arguments": ["system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<utils::geometry::rectangle::Rectangle2d>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["utils::geometry::rectangle::Rectangle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::rectangle::Rectangle2d>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["utils::geometry::rectangle::Rectangle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::rectangle::Rectangle3d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 15; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<utils::geometry::rectangle::Rectangle3d,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["utils::geometry::rectangle::Rectangle3d", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::shape_overlaps_with<utils::geometry::rectangle::Rectangle3d>", "constructor": "country_core::systems::collision::world::RaycastWorld::shape_overlaps_with", "arguments": ["utils::geometry::rectangle::Rectangle3d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rectangle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rectangle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
// 信息 ID: d3a1942a99812da5175b1408
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_normal_at_u @ 0xded828 -> 0xded830 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners @ 0xdef3b8 -> 0xded830 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::aabb
// 信息 ID: d0a28527a98bff828cb60564
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x8387a4 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::upload_dormer_holes_to_gpu @ 0x86e869 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0xb3e5f9 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0xb3ef58 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0xb3f0c1 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0xb3f488 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0xb3fb44 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xb41dd8 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d4779 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d4505 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d45e2 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::map::HashMap::retain<system_clutter::autoclutter::ClutterRegionId,system_clutter::autoclutter::AutoClutterRegion,std::hash::random::RandomState,alloc::alloc::Global,system_clutter::autoclutter::impl$1::remove_stale_regions::closure_env$0> @ 0x21bec96 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bfdd2 -> 0xdef490 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=26708; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=27620; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
// 信息 ID: df0acf86e5cfe0c05d6bbce1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_edge_normal; module=764, inline_record=16316; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=17492; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18004; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18140; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18420; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18860; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=19284; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=19648; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::area
// 信息 ID: 65a656707995a18609e277e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio @ 0x92b9fd -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio @ 0x92bec4 -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0xab50eb -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::set_parameters @ 0xab69ac -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::get_building_size_01 @ 0xab6c10 -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle @ 0xacbc4a -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle @ 0xacbf26 -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle @ 0xacc000 -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle @ 0xacbda4 -> 0xdef4f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=764, inline_record=1160; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=764, inline_record=1632; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=3448; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::edge_length; module=764, inline_record=4572; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_edge_pts; module=764, inline_record=5592; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6568; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6724; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=7168; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_curve2
// 信息 ID: 4ea7a388b56c4c4ae3a5c101
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle @ 0xa3fdd8 -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2 @ 0xa62467 -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle @ 0xa625f3 -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier::closure$0::closure$0 | core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier::closure$0::closure_env$0,tuple$<> > @ 0xacb184 -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5c0aa -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1 @ 0xbdba93 -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::calc_shape_height::calc_shape_height_and_stone_foundation @ 0xbec250 -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x147bfb8 -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x147bf0c -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x148d7b0 -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x148e28c -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x148e6dc -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f4782c -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f4784e -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x28235ff -> 0xdef820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2823613 -> 0xdef820 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_curve3
// 信息 ID: 39cc4b9726e792c9ba5ae6ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0> @ 0x904ced -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0> @ 0x9057d2 -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3 @ 0xa624ad -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle2d::show @ 0xa75521 -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line @ 0xaf1932 -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f467b3 -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f46b93 -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f47215 -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x23b0d80 -> 0xdefb00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x28d0c0a -> 0xdefb00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points2
// 信息 ID: e4026474b2b424e68fc7ca82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81eb40 -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xb41e5b -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::do_ui_edit_rectangle_rotate @ 0xb42ab6 -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xb4426c -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xb442fb -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc28c -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cd153 -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x143a01a -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x143a03b -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x143a0d4 -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y @ 0x14866f8 -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1> @ 0x14f1724 -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282bdc7 -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282bddd -> 0xdec050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282be86 -> 0xdec050 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=764, inline_record=1756; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::edge_length; module=764, inline_record=4532; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_edge_pts; module=764, inline_record=5552; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::try_as_curve2; module=764, inline_record=10248; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects_circle; module=764, inline_record=23776; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id; module=764, inline_record=35928; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=764, inline_record=41360; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=764, inline_record=43040; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
// 信息 ID: ae791262ed5c78edb3086897
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cbe48 -> 0xdeec40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cc80a -> 0xdeec40 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=764, inline_record=1240; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=764, inline_record=1852; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=3412; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6536; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6692; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=7136; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=7404; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=8924; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points3
// 信息 ID: 4b28485ef1f16c67204a9144
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36d5c3 -> 0xdec130 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37faeb -> 0xdec130 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=764, inline_record=42900; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::points; module=764, inline_record=47296; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_transformation_and_size
// 信息 ID: 15f65a29d59f5173edfcae0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cbd49 -> 0xdeea20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cbd7f -> 0xdeea20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cc6ff -> 0xdeea20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cc731 -> 0xdeea20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=2472; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=2652; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::circular_slice
// 信息 ID: 82372f1fbe927c0131b13e24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e801 -> 0xded0f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1>
// 信息 ID: eb7b63a3e5cbdbfb4752962d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points -> ["system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape @ 0x892d76 -> 0x14f15c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::closest_edge_normal
// 信息 ID: 76bd11e516376baa7d5ffd6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8dd4ed -> 0xdee1f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
// 信息 ID: fccca348f9d81fbdebf79e18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds @ 0x8d754d -> 0xdeea60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x14e5843 -> 0xdeea60 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x20b43b7 -> 0xdeea60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests::closure$0 @ 0x28c8292 -> 0xdeea60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests::closure$0 @ 0x28c83b2 -> 0xdeea60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::contains_point
// 信息 ID: 36fcdea3e7b8a41e78997372
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 33 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x9099af -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x909cd8 -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x909d5f -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x928d27 -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0xa5bf04 -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0xa5bf4c -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point | country_core::resources::glade_border::GladeBorder::is_pos_inside @ 0xa620b0 -> 0xded740 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa620f8 -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa6211b -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa6213e -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62167 -> 0xded740 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa6220f -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62235 -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa6225b -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62281 -> 0xded740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa622a7 -> 0xded740 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::contains_point; module=764, inline_record=45412; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::contains_point_padding
// 信息 ID: 4093c4d103864955dc4bf0bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::trap_doors::generate_trap_doors @ 0x87a5fb -> 0xdee530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::draw_path::DrawState::update_bounded_by_glade @ 0xc1d35f -> 0xdee530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x14d6a31 -> 0xdee530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x14d6a5b -> 0xdee530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28befe9 -> 0xdee530 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::contains_point; module=764, inline_record=15648; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::contains_point; module=764, inline_record=45436; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge
// 信息 ID: aa8d69cb85dc44831e66a0d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x14828de -> 0xdee8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::edge_length
// 信息 ID: ad7e5baa89c04d262c402a8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x14d50f4 -> 0xdec5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x14d5632 -> 0xdec5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x24f1955 -> 0xdec5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x250baa7 -> 0xdec5f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::expand_along_edge
// 信息 ID: fdfaa49b92891d7d40527f2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims @ 0xb40bc7 -> 0xdedcc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
// 信息 ID: dfdd5afbd13c1936e10a5f7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::derive_decorator_subtype::is_half_dormer @ 0x8d89b3 -> 0xdee2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5bcd0 -> 0xdee2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x14d50e9 -> 0xdee2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x14d5627 -> 0xdee2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x24f194a -> 0xdee2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x250ba9c -> 0xdee2c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
// 信息 ID: 43d5523b0eaa3e1848ac5073
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id @ 0xdee2c0 -> 0xdeeca0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos @ 0xdee5ec -> 0xdeeca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter @ 0xdeea87 -> 0xdeeca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
// 信息 ID: 857c5d42d1a859ad529696da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x882869 -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x882b1d -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e1d26 -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e1e12 -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos @ 0xa623c0 -> 0xdee5b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc07d -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc090 -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc1b4 -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc1c6 -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10ccb44 -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10ccb58 -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10ccedb -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cceee -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141abaa -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141acbc -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e7cf -> 0xdee5b0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_corner_normal
// 信息 ID: 52d3e202c10b4272f16c4a1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8cb776 -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0xb55bcb -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0xb55cd7 -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0xb560ed -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0xb5619a -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz @ 0xbf1693 -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams @ 0x14434c3 -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x1486aa5 -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x1486d8c -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x14878eb -> 0xdede00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x148aadb -> 0xdede00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=17444; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=17668; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18092; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18372; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18812; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_corner_unnormal
// 信息 ID: 48b1c7e5642ac47f02748833
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=17468; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18116; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18396; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=18836; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_corner_normal; module=764, inline_record=22968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
// 信息 ID: 551b28c2bb8c0012d36564a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8cb794 -> 0xdef0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0xb55bdb -> 0xdef0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams @ 0x1443b5d -> 0xdef0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x1486d5c -> 0xdef0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u
// 信息 ID: 0a114b778bf7e8b98a933ede
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143df72 -> 0xdedac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143e052 -> 0xdedac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_normal
// 信息 ID: 40a2eca5cd1096c352185d71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81f3fd -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81f6d9 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims @ 0xb406c2 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims @ 0xb40733 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims @ 0xb407a1 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget @ 0xb9f231 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::signifier @ 0xb9faa5 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141c3d7 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141c3f5 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_x @ 0x1490fe4 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1> @ 0x14f169a -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1> @ 0x14f16b9 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1> @ 0x14f16d8 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1> @ 0x14f16f7 -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> @ 0x24dde6a -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> @ 0x24dde7d -> 0xded7a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=19260; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=19624; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=19988; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=764, inline_record=20128; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::expand_along_edge; module=764, inline_record=22368; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_pts
// 信息 ID: 960d064fdde7de12ddb2cc68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5bce2 -> 0xdec820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x293ed34 -> 0xdec820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x293ee1c -> 0xdec820 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::edge_length; module=764, inline_record=4484; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_normal_at_u
// 信息 ID: f8ef7374d222482aa132cd4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83bedc -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u @ 0xa623a0 -> 0xded820 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc09f -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc0b2 -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc1d5 -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc1e7 -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10ccb68 -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10ccb7c -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10ccefe -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10ccf12 -> 0xded820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x1464152 -> 0xded820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners
// 信息 ID: dfa3825bb44642b78dad9fd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141f48f -> 0xdef3b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_point
// 信息 ID: 60c9c15e6557c70f6206163c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 142 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81f202 -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81f220 -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e1ce3 -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0> @ 0x904f26 -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0> @ 0x904f4a -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0> @ 0x9059eb -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0> @ 0x905a0f -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fd577 -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fd5ad -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fd5da -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fd610 -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fd68c -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fd69a -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fd6a8 -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fd6b6 -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa620ea -> 0xdefe50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6504; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6660; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6816; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=7104; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=7372; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=8876; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=9332; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos; module=764, inline_record=28588; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_point_u
// 信息 ID: 638a75f78f9d2bf5e52700e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 117 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x80468a -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x8046b6 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x8046df -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x80470b -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x80b5a1 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x80b5e3 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x80b716 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x80b74f -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x80b864 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x80b89d -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x80b9ae -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x80b9e3 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x8406cd -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x8406e2 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x8406f3 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x840708 -> 0xdec740 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=5916; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6108; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6276; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=8060; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=8276; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=8572; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=9852; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::circular_slice; module=764, inline_record=12172; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_pos_at_u
// 信息 ID: a1eb4f706421bd4995ac1af7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u @ 0xa62090 -> 0xdec960 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1> @ 0x14f1e48 -> 0xdec960 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points<system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape::closure_env$1> @ 0x14f1b23 -> 0xdec960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::half_dimensions
// 信息 ID: 750d82d59061bcd14e9986c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=26756; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::aabb; module=764, inline_record=39400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::intersects
// 信息 ID: 0136e5fe4b4eb8e2f8e56341
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x9290e4 -> 0xdec2d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::intersects_circle
// 信息 ID: 229ea8f489946cc1352b2889
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x9291c3 -> 0xdeded0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x929238 -> 0xdeded0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::lerp
// 信息 ID: 58418e129b96d41d9a1ce39d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xb4465d -> 0xdef500 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xb44711 -> 0xdef500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::local_aabb_of
// 信息 ID: 7989e3f92c552dbfc8500dc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=26284; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=27188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new
// 信息 ID: ee70084d7daf3516cee8c497
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7d9da2e094c5e3aaa2b1ca08: void (*)(bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 21 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81eb2c -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81f150 -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac363d -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0xb3e2e6 -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0xb3ec0c -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0xb3f16f -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xba7d4b -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y @ 0xbe8be4 -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d4767 -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d44f3 -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d45d0 -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y @ 0x14866eb -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::commands::Commands)> > @ 0x193ed8e -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::commands::Commands)> > @ 0x196b494 -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::commands::Commands)> @ 0x19b87c9 -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::map::HashMap::retain<system_clutter::autoclutter::ClutterRegionId,system_clutter::autoclutter::AutoClutterRegion,std::hash::random::RandomState,alloc::alloc::Global,system_clutter::autoclutter::impl$1::remove_stale_regions::closure_env$0> @ 0x21bec8b -> 0xdef3c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=764, inline_record=540; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::new_axis_aligned; module=764, inline_record=21556; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned; module=764, inline_record=31420; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::lerp; module=764, inline_record=40740; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::new; module=764, inline_record=46560; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new_axis_aligned
// 信息 ID: d9c15763883eb32358e9eddd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xc29ea0 -> 0xdedbd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1439ef5 -> 0xdedbd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adeff8 -> 0xdedbd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2822596 -> 0xdedbd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2824cb8 -> 0xdedbd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282bc0c -> 0xdedbd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x293ecf6 -> 0xdedbd0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=764, inline_record=416; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
// 信息 ID: e81352309a6e9be620e4d037
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1 @ 0xbdf5db -> 0xdee9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1 @ 0xbdf5fd -> 0xdee9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141aa82 -> 0xdee9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1439ebd -> 0xdee9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x282254d -> 0xdee9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2824c48 -> 0xdee9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282bbc2 -> 0xdee9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::orientation_mat2
// 信息 ID: e748468fd8c6dc6c47021827
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 24 条；内联宿主 36 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e1a45 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e1a60 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0xabe5c0 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0xabe726 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0xabe993 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0xabeae9 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0xabe8a1 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0xb3f0f4 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0xbe6a7c -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0xbe6ad1 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz @ 0xbe855d -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141ab32 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141ab53 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141c556 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141c5eb -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1439f35 -> 0xdedc50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=764, inline_record=1184; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=764, inline_record=1796; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=2500; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=2676; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6600; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=6756; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=7200; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=7468; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::orientation_mat3
// 信息 ID: cdc3d75303f781cf0e4c407b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0xb55d02 -> 0xdedc70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5a6dd -> 0xdedc70 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f46823 -> 0xdedc70 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f46d5f -> 0xdedc70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d
// 信息 ID: 6f0cdd0b786c0e771eb20759
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bff58 -> 0xdee2d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::perimeter
// 信息 ID: 1c733bc12f0907fdf44b8146
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 31 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x882875 -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x882b29 -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e1d32 -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e1e1d -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0> @ 0x904d3d -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0> @ 0x90581e -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter @ 0xa624f0 -> 0xdeff20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier::closure$0 @ 0xacaf94 -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1 @ 0xbdbb4c -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141abbe -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141abdd -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141acd0 -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141acef -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141adc1 -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141ae2d -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143f6aa -> 0xdeff20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=764, inline_record=5068; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=764, inline_record=5096; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=764, inline_record=5124; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=764, inline_record=5152; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=764, inline_record=5944; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=8320; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=8348; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=764, inline_record=8376; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
// 信息 ID: 6e094041eda5c55cbe721a37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x148e843 -> 0xdef2f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge; module=764, inline_record=30732; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::try_as_curve2
// 信息 ID: c24b9b51799a4d1933ba9c4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x147bbdc -> 0xdece60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::with_expanded_size
// 信息 ID: 2af6f436aa0c4dc519a55098
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0x1485dad -> 0xdee1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x286220c -> 0xdee1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x2861cc6 -> 0xdee1d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::center
// 信息 ID: 98bab508414d1e82702a9e97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6111dd30703fa00f801674f1: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=85, inline_record=28988; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=85, inline_record=51312; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=77264; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator; module=437, inline_record=2756; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=445, inline_record=29576; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=445, inline_record=29636; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=446, inline_record=14952; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=450, inline_record=11592; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::contains_point
// 信息 ID: 54ac1ed11d9ce7246dda7929
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::expand
// 信息 ID: aa7e2ae86d490cc32d01e8b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x928f90 -> 0xdf0150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::extent
// 信息 ID: 42c2c408965bd89dc0942d8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::create_cart_chain @ 0x34f9cb -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::create_cart_chain @ 0x34fb1c -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x8791e8 -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x879341 -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0x88184f -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0x881920 -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x8beebf -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x8bf3dc -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x8bf4f7 -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x8bf6ff -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c27d3 -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c2fd5 -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c33b0 -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c354c -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c3643 -> 0xdf0190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c367c -> 0xdf0190 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::height
// 信息 ID: 363813b57ef91df66f25d431
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6111dd30703fa00f801674f1: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 46 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=85, inline_record=28960; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=85, inline_record=51284; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=77228; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=445, inline_record=29492; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=11104; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=452, inline_record=27440; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=455, inline_record=7128; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=455, inline_record=7328; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::length
// 信息 ID: 57b7c8bceab8efe88a20cc0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6111dd30703fa00f801674f1: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=85, inline_record=28932; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=85, inline_record=51256; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=77196; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=445, inline_record=29548; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=455, inline_record=7184; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins; module=509, inline_record=21096; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::world::update_rapier_colliders; module=527, inline_record=48392; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::extent; module=764, inline_record=47104; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::new
// 信息 ID: 5479d3dbcc93c2d82db593f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6111dd30703fa00f801674f1: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x31b531 -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x31d750 -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports @ 0x82714c -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_subtype::DecoratorCollision::empty @ 0xc57a3d -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x1482c69 -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x1486fd5 -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0 @ 0x21a40e1 -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> @ 0x24f14fc -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> > @ 0x24f1d80 -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf562 -> 0xdf0010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bfaf1 -> 0xdf0010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
// 信息 ID: 149115971963b06c65bed36b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> > @ 0x24f1d8c -> 0xdeffc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf62b -> 0xdeffc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bfb97 -> 0xdeffc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates @ 0x28c9b1b -> 0xdeffc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::points
// 信息 ID: c7048741f35e45d894821f75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::rotate_projective_xz
// 信息 ID: 228d0b3c94fd69df3c510b53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::set_orientation_xz
// 信息 ID: 7a89377c73011df83ec85da2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860db1 -> 0xdeffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8dda7d -> 0xdeffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x929041 -> 0xdeffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter @ 0xad5f97 -> 0xdeffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28be817 -> 0xdeffb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::set_size
// 信息 ID: cfee54b8d27ddc910f8e3d18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter @ 0x88d96e -> 0xdf03b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::decoclutter_focus_bbx @ 0xad84b6 -> 0xdf03b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::translate
// 信息 ID: 05c30a5466c2ba7f3d7d0e55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860ddf -> 0xdf0400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8ddaa7 -> 0xdf0400 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x92905d -> 0xdf0400 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter @ 0xad5fb8 -> 0xdf0400 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::decoclutter_focus_bbx @ 0xad85b5 -> 0xdf0400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28be7c9 -> 0xdf0400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::width
// 信息 ID: 03f1a855c3c28d26682e0316
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6111dd30703fa00f801674f1: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 39 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=85, inline_record=28904; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)>; module=85, inline_record=51228; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=77164; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator; module=437, inline_record=2728; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=445, inline_record=29520; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=446, inline_record=7640; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=11076; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=450, inline_record=7728; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize
// 信息 ID: 320e3bad2f1d3992e958942d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<serde_json::error::Error> >; module=609, inline_record=55148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 89658212123d93ea72aaef0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<utils::geometry::rectangle::_::impl$0::deserialize::__FieldVisitor> @ 0x17db610 -> 0xdec030 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 信息 ID: 6194ad190f58a7d9dc920bda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: b80d40dab53f966396a2f51d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: cf0cc6d483d126d52373cb35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=34904; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=35176; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=35508; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=35540; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=35592; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=35624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: da2d0756f429469fdcd72cc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=34664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: b33449ba800a9fc4308864ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<utils::geometry::rectangle::_::impl$0::deserialize::__Visitor> @ 0x17db600 -> 0xdec010 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 6f7258d321d2b4c9092f99c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=34252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 2f7598cf9f0fc5f289f8b5f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=39296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: e9712e738891a06647d47812
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] utils::geometry::rectangle::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x5a37ce -> 0x1962e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xc85ffe -> 0x17db940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 1d72cfdd80e133f6d3c10d29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::rectangle::Rectangle2d> @ 0x1b7ff9b -> 0x17dba90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$12::eq
// 信息 ID: bdb486fa4ad46644fa0d1ddb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::ui_move_wall_group; module=561, inline_record=32172; 不是独立调用边。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2741, inline_record=13828; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3779, inline_record=4556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::as_points3::closure$0
// 信息 ID: 060e21f0a0cad1921d22cb75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::as_points3", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=764, inline_record=1732; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=764, inline_record=43012; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::points; module=764, inline_record=47432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0
// 信息 ID: 23cfa5147ab55b43a3bd8067
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::get_pos_at_u", "captures": "unknown"}
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_pos_at_u @ 0xdecb3c -> 0xdecc00 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_pos_at_u @ 0xdecb65 -> 0xdecc00 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_pos_at_u @ 0xdecb8e -> 0xdecc00 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_pos_at_u @ 0xdecbad -> 0xdecc00 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::intersects::closure$0
// 信息 ID: 743e8bf7e00bc6b8a271662c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::intersects", "captures": "unknown"}
// [闭包环境类型引用] utils::geometry::rectangle::impl$1::intersects::closure_env$0 <- itertools::tuple_impl::impl$10::next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::rectangle::impl$1::intersects::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$10::next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=2190, inline_record=3444; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0> > > > >; module=3638, inline_record=2240; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl$1::intersects::closure_env$0> > > > >; module=3638, inline_record=4072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::intersects_circle::closure$0
// 信息 ID: 409c94e64d11806f61fa3fc9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::intersects_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects_circle; module=764, inline_record=24652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$2::from
// 信息 ID: 78d5bfccd795407b0c76c9c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac311d -> 0xdebf80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$4::clone
// 信息 ID: 96996100daf5f397e2b48989
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert; module=603, inline_record=10260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$8::clone
// 信息 ID: 7552f3e2bf6cf48e3ddf28ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle; module=574, inline_record=40372; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle; module=574, inline_record=40400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle; module=574, inline_record=40428; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle; module=574, inline_record=40456; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > >; module=1631, inline_record=5728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$9::fmt
// 信息 ID: c753aad5ce8c3fbbf1e0c03e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<utils::geometry::rectangle::Rectangle2d>; module=1669, inline_record=33336; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$80::fmt<utils::geometry::rectangle::Rectangle2d>; module=4989, inline_record=416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
