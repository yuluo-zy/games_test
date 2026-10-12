// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/ivy/wall_coord.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b58e831b0b0a439d1feecde0bf814bbca906e8ecf42a3a0cf8379c876f73f82e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_coord.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 512, 524, 579, 583
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::ivy::wall_coord::WallCoord::into_world_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":33732,"rva":11639264,"va":5380348384}
// 编译实例 RVAs: 0xb199e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_coord.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1585907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380348384,"raw_hex":"e8f3321800","size":5,"soft_fail":false,"target_va":5381934528,"va":5380348616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141467ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: wall_coord.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9757292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380348384,"raw_hex":"e86ce29400","size":5,"soft_fail":false,"target_va":5390105824,"va":5380348527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31193055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380348384,"raw_hex":"e8dff7db01","size":5,"soft_fail":false,"target_va":5411542064,"va":5380349004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31193160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380348384,"raw_hex":"e848f8db01","size":5,"soft_fail":false,"target_va":5411542296,"va":5380349131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31192919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380348384,"raw_hex":"e857f7db01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380348980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":579,"record_offset":35512,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":35116,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35180,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35328,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35424,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35148,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":34160,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":34676,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":33924,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":579,"record_offset":33952,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":34244,"parent_record_offset":34188,"ancestor_record_offset":33732,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":34040,"parent_record_offset":34016,"ancestor_record_offset":33732,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":34064,"parent_record_offset":34040,"ancestor_record_offset":33732,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":579,"record_offset":34100,"parent_record_offset":33952,"ancestor_record_offset":33732,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":579,"record_offset":34128,"parent_record_offset":33952,"ancestor_record_offset":33732,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":34628,"parent_record_offset":34604,"ancestor_record_offset":33732,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":34544,"parent_record_offset":34520,"ancestor_record_offset":33732,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":35268,"parent_record_offset":35180,"ancestor_record_offset":33732,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":35360,"parent_record_offset":35328,"ancestor_record_offset":33732,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":35452,"parent_record_offset":35424,"ancestor_record_offset":33732,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":34896,"parent_record_offset":34872,"ancestor_record_offset":33732,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":579,"record_offset":34016,"parent_record_offset":33992,"ancestor_record_offset":33732,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":579,"record_offset":33992,"parent_record_offset":33952,"ancestor_record_offset":33732,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":579,"record_offset":34272,"parent_record_offset":34244,"ancestor_record_offset":33732,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":579,"record_offset":34300,"parent_record_offset":34272,"ancestor_record_offset":33732,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":35032,"parent_record_offset":35000,"ancestor_record_offset":33732,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":35000,"parent_record_offset":34976,"ancestor_record_offset":33732,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":33732,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":34492,"parent_record_offset":34464,"ancestor_record_offset":33732,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":33732,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":34520,"parent_record_offset":34492,"ancestor_record_offset":33732,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":34604,"parent_record_offset":34492,"ancestor_record_offset":33732,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":579,"record_offset":34872,"parent_record_offset":34676,"ancestor_record_offset":33732,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":579,"record_offset":35064,"parent_record_offset":35032,"ancestor_record_offset":33732,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":34716,"parent_record_offset":34676,"ancestor_record_offset":33732,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":34188,"parent_record_offset":34160,"ancestor_record_offset":33732,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":34216,"parent_record_offset":34188,"ancestor_record_offset":33732,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":34340,"parent_record_offset":34188,"ancestor_record_offset":33732,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":579,"record_offset":34464,"parent_record_offset":34188,"ancestor_record_offset":33732,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":579,"record_offset":34928,"parent_record_offset":34676,"ancestor_record_offset":33732,"inlinee":25387,"source_line":68,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ivy::wall_coord::WallCoord::from_wall_u
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72CF
// 首条位置: {"module":512,"inlinee":29391}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72D1
// 首条位置: {"module":512,"inlinee":29393}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":512,"record_offset":37980,"parent_record_offset":37940,"ancestor_record_offset":37940,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00512.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_coord.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::ivy::wall_coord::WallCoord
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_coord.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_coord.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>", "bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::audio::menu_audio::MenuAudioState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::event::reader::EventReader<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::audio::menu_audio::MenuAudioState>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::menu_audio::MenuAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>

// ------------------------------------------------------------------
// 函数: country_core::resources::ivy::wall_coord::WallCoord::from_wall_u
// 信息 ID: 2d1d00199b5472c965a15ae8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=512, inline_record=36980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ivy::wall_coord::WallCoord::into_world_space
// 信息 ID: d7942b2cc753f6429992d217
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0x94b7b9 -> 0xb199e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
// 信息 ID: ef9b6b93166d1d9b18366431
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=512, inline_record=37940; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=512, inline_record=40912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_pruner::ivy_pruner; module=524, inline_record=62496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
