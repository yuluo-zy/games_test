// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/curve/curve_slice.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5ba0f0897347ce603678933b5ffef35ef3d65581fa41dbb973788865b0634b8d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: curve_slice.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 390, 392, 401, 432, 443, 444, 453, 466, 1407, 1416, 1417, 1419, 1435, 1436, 1443, 1452, 1455, 1467, 1506, 1510, 1522, 1538, 1552, 1562, 2603, 2604, 3001, 3011, 3805, 3813, 3814
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use take_until; // namespace candidate
//   package identities: [{"name":"take-until","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurvePoints::is_empty
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114A2
// 首条位置: {"module":1416,"inlinee":70818}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":2688,"inlinee":24437,"source_line":51,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1416,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":2688,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$0::iter
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114A5
// 首条位置: {"module":1416,"inlinee":70821}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1416,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1416,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":3496,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1416,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":3496,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1416,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":3496,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$1::len
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F75
// 首条位置: {"module":390,"inlinee":24437}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":19676,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$1::iter
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114A3
// 首条位置: {"module":1416,"inlinee":70819}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::from_curve_range
// 记录形式: inline_instance, inlinee_declaration; instances=33
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6228
// 首条位置: {"module":432,"inlinee":25128}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=17
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":9816,"parent_record_offset":9780,"ancestor_record_offset":9780,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":9856,"parent_record_offset":9816,"ancestor_record_offset":9780,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::from_point_range
// 记录形式: inline_instance, inlinee_declaration; instances=39
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F73
// 首条位置: {"module":390,"inlinee":24435}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":9856,"parent_record_offset":9816,"ancestor_record_offset":9816,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":19676,"parent_record_offset":19632,"ancestor_record_offset":19632,"inlinee":24437,"source_line":51,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":19632,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$3::from_point_range::closure$0
// 记录形式: inlinee_declaration; instances=2
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11853
// 首条位置: {"module":1538,"inlinee":71763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$3::from_coord::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1149D
// 首条位置: {"module":1416,"inlinee":70813}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::from_coord_range
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F6C
// 首条位置: {"module":390,"inlinee":24428}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":390,"record_offset":17004,"parent_record_offset":16940,"ancestor_record_offset":16940,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$11::sub
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":390,"record_offset":16976,"parent_record_offset":16940,"ancestor_record_offset":16940,"inlinee":24430,"source_line":1358,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::remap
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F90
// 首条位置: {"module":392,"inlinee":24464}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::remap_2d
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F8F
// 首条位置: {"module":392,"inlinee":24463}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":392,"record_offset":14844,"parent_record_offset":14820,"ancestor_record_offset":14820,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$5::remap_2d::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1149F
// 首条位置: {"module":1416,"inlinee":70815}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right
// 记录形式: inline_instance, inlinee_declaration; instances=41
// 源码行: 236
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F91
// 首条位置: {"module":392,"inlinee":24465}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14400,"parent_record_offset":14364,"ancestor_record_offset":14364,"inlinee":24631,"source_line":243,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":401,"record_offset":14880,"parent_record_offset":14848,"ancestor_record_offset":14364,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":401,"record_offset":16728,"parent_record_offset":16696,"ancestor_record_offset":16396,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":14364,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":14364,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":14364,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":401,"record_offset":15092,"parent_record_offset":15060,"ancestor_record_offset":14364,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":15028,"parent_record_offset":14996,"ancestor_record_offset":14364,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":15060,"parent_record_offset":15028,"ancestor_record_offset":14364,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":14364,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":14820,"parent_record_offset":14400,"ancestor_record_offset":14364,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":401,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":14364,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":14364,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":14364,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":15164,"parent_record_offset":15124,"ancestor_record_offset":14364,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":15124,"parent_record_offset":15092,"ancestor_record_offset":14364,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":16756,"parent_record_offset":16696,"ancestor_record_offset":16396,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":14364,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":14484,"parent_record_offset":14400,"ancestor_record_offset":14364,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":13644,"inlinee":70931,"source_line":340,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1436,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":2936,"inlinee":70811,"source_line":94,"archive":"_pdb/B/modules/01436.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":14364,"inlinee":24634,"source_line":372,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":401,"record_offset":14996,"parent_record_offset":14964,"ancestor_record_offset":14364,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":16516,"parent_record_offset":16436,"ancestor_record_offset":16396,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14700,"parent_record_offset":14676,"ancestor_record_offset":14364,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":401,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":14364,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":14848,"parent_record_offset":14400,"ancestor_record_offset":14364,"inlinee":24636,"source_line":121,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6037
// 首条位置: {"module":401,"inlinee":24631}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":14820,"parent_record_offset":14400,"ancestor_record_offset":14400,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":14484,"parent_record_offset":14400,"ancestor_record_offset":14400,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":16516,"parent_record_offset":16436,"ancestor_record_offset":16436,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":14848,"parent_record_offset":14400,"ancestor_record_offset":14400,"inlinee":24636,"source_line":121,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":401,"record_offset":14880,"parent_record_offset":14848,"ancestor_record_offset":14400,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":401,"record_offset":16728,"parent_record_offset":16696,"ancestor_record_offset":16436,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":14400,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":14400,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":14400,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":401,"record_offset":15092,"parent_record_offset":15060,"ancestor_record_offset":14400,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":15028,"parent_record_offset":14996,"ancestor_record_offset":14400,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":15060,"parent_record_offset":15028,"ancestor_record_offset":14400,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":14400,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":401,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":14400,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":14400,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":14400,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":15164,"parent_record_offset":15124,"ancestor_record_offset":14400,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":15124,"parent_record_offset":15092,"ancestor_record_offset":14400,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":16756,"parent_record_offset":16696,"ancestor_record_offset":16436,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":14400,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":13680,"inlinee":70931,"source_line":340,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1436,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":2968,"inlinee":70811,"source_line":94,"archive":"_pdb/B/modules/01436.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":14400,"inlinee":24634,"source_line":372,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":401,"record_offset":14996,"parent_record_offset":14964,"ancestor_record_offset":14400,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14700,"parent_record_offset":14676,"ancestor_record_offset":14400,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":401,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":14400,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1538,"record_offset":660,"rva":19437792,"va":5388146912}
// 编译实例 RVAs: 0x12898e0, 0x22e1e40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412affb0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t157119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388146912,"raw_hex":"e8bf650200","size":5,"soft_fail":false,"target_va":5388304304,"va":5388147180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23394261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388146912,"raw_hex":"e8d5f76401","size":5,"soft_fail":false,"target_va":5411542064,"va":5388147798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5388146912,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5388147134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1538,"record_offset":2196,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1538,"record_offset":2280,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1538,"record_offset":1228,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1538,"record_offset":2056,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":70821,"source_line":36,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1538,"record_offset":2372,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":1538,"record_offset":3236,"parent_record_offset":3208,"ancestor_record_offset":660,"inlinee":6194,"source_line":336,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1538,"record_offset":1608,"parent_record_offset":1576,"ancestor_record_offset":660,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1538,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":660,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1538,"record_offset":1284,"parent_record_offset":1228,"ancestor_record_offset":660,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1538,"record_offset":1576,"parent_record_offset":1396,"ancestor_record_offset":660,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1538,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":660,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1538,"record_offset":1360,"parent_record_offset":1324,"ancestor_record_offset":660,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1538,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":660,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1538,"record_offset":1396,"parent_record_offset":1360,"ancestor_record_offset":660,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$10::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1538,"record_offset":1432,"parent_record_offset":1396,"ancestor_record_offset":660,"inlinee":21737,"source_line":1015,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1538,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":660,"inlinee":70823,"source_line":222,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":1538,"record_offset":1940,"parent_record_offset":1228,"ancestor_record_offset":660,"inlinee":14445,"source_line":56,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1538,"record_offset":1968,"parent_record_offset":1228,"ancestor_record_offset":660,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1538,"record_offset":1516,"parent_record_offset":1432,"ancestor_record_offset":660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1538,"record_offset":1696,"parent_record_offset":1576,"ancestor_record_offset":660,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1538,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":660,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1538,"record_offset":2312,"parent_record_offset":2280,"ancestor_record_offset":660,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1538,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":660,"inlinee":7193,"source_line":79,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1538,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":660,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1538,"record_offset":1824,"parent_record_offset":1284,"ancestor_record_offset":660,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1538,"record_offset":2024,"parent_record_offset":1228,"ancestor_record_offset":660,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1538,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":660,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1538,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":660,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1538,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":660,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1538,"record_offset":3116,"parent_record_offset":3076,"ancestor_record_offset":660,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1538,"record_offset":1664,"parent_record_offset":1636,"ancestor_record_offset":660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1538,"record_offset":2624,"parent_record_offset":2596,"ancestor_record_offset":660,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1538,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1538,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1538,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":660,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1538,"record_offset":1636,"parent_record_offset":1576,"ancestor_record_offset":660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1538,"record_offset":1780,"parent_record_offset":1576,"ancestor_record_offset":660,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1538,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":660,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1538,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":660,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1538,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":660,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1538,"record_offset":2572,"parent_record_offset":2520,"ancestor_record_offset":660,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1538,"record_offset":2476,"parent_record_offset":2428,"ancestor_record_offset":660,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1538,"record_offset":1324,"parent_record_offset":1284,"ancestor_record_offset":660,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1538,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":660,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/01538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1416,"record_offset":10112,"rva":18906128,"va":5387615248}
// 编译实例 RVAs: 0x1207c10, 0x12898e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412affb0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t688783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387615248,"raw_hex":"e88f820a00","size":5,"soft_fail":false,"target_va":5388304304,"va":5387615516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23925925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387615248,"raw_hex":"e8a5146d01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387616134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387615248,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5387615470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":11668,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":11752,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1416,"record_offset":10700,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":11528,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":70821,"source_line":36,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1416,"record_offset":11844,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":1416,"record_offset":12708,"parent_record_offset":12680,"ancestor_record_offset":10112,"inlinee":6194,"source_line":336,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1416,"record_offset":11080,"parent_record_offset":11048,"ancestor_record_offset":10112,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1416,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":10112,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1416,"record_offset":10756,"parent_record_offset":10700,"ancestor_record_offset":10112,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1416,"record_offset":11048,"parent_record_offset":10868,"ancestor_record_offset":10112,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":11692,"parent_record_offset":11668,"ancestor_record_offset":10112,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":10832,"parent_record_offset":10796,"ancestor_record_offset":10112,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":10112,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":10868,"parent_record_offset":10832,"ancestor_record_offset":10112,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$10::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1416,"record_offset":10904,"parent_record_offset":10868,"ancestor_record_offset":10112,"inlinee":21737,"source_line":1015,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1416,"record_offset":10932,"parent_record_offset":10904,"ancestor_record_offset":10112,"inlinee":70823,"source_line":222,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":1416,"record_offset":11412,"parent_record_offset":10700,"ancestor_record_offset":10112,"inlinee":14445,"source_line":56,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":11440,"parent_record_offset":10700,"ancestor_record_offset":10112,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1416,"record_offset":10988,"parent_record_offset":10904,"ancestor_record_offset":10112,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1416,"record_offset":11168,"parent_record_offset":11048,"ancestor_record_offset":10112,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1416,"record_offset":10956,"parent_record_offset":10932,"ancestor_record_offset":10112,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1416,"record_offset":11784,"parent_record_offset":11752,"ancestor_record_offset":10112,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":10112,"inlinee":7193,"source_line":79,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":11320,"parent_record_offset":11296,"ancestor_record_offset":10112,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":11296,"parent_record_offset":10756,"ancestor_record_offset":10112,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":11496,"parent_record_offset":10700,"ancestor_record_offset":10112,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1416,"record_offset":12636,"parent_record_offset":12612,"ancestor_record_offset":10112,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1416,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":10112,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1416,"record_offset":12612,"parent_record_offset":12588,"ancestor_record_offset":10112,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":12588,"parent_record_offset":12548,"ancestor_record_offset":10112,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":11136,"parent_record_offset":11108,"ancestor_record_offset":10112,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1416,"record_offset":12096,"parent_record_offset":12068,"ancestor_record_offset":10112,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1416,"record_offset":11012,"parent_record_offset":10988,"ancestor_record_offset":10112,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1416,"record_offset":11576,"parent_record_offset":11552,"ancestor_record_offset":10112,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1416,"record_offset":11600,"parent_record_offset":11576,"ancestor_record_offset":10112,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":11108,"parent_record_offset":11048,"ancestor_record_offset":10112,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":11252,"parent_record_offset":11048,"ancestor_record_offset":10112,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1416,"record_offset":11552,"parent_record_offset":11528,"ancestor_record_offset":10112,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":11900,"parent_record_offset":11872,"ancestor_record_offset":10112,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":11872,"parent_record_offset":11844,"ancestor_record_offset":10112,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1416,"record_offset":12044,"parent_record_offset":11992,"ancestor_record_offset":10112,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1416,"record_offset":11948,"parent_record_offset":11900,"ancestor_record_offset":10112,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1416,"record_offset":10796,"parent_record_offset":10756,"ancestor_record_offset":10112,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":11344,"parent_record_offset":11320,"ancestor_record_offset":10112,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1416,"record_offset":9156,"rva":18905616,"va":5387614736}
// 编译实例 RVAs: 0x1207a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141238280
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t198575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387614736,"raw_hex":"e8af070300","size":5,"soft_fail":false,"target_va":5387813504,"va":5387614924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23926829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387614736,"raw_hex":"e82d186d01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387615230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":9812,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":9896,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":9784,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":70819,"source_line":55,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1416,"record_offset":9960,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":9836,"parent_record_offset":9812,"ancestor_record_offset":9156,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":9860,"parent_record_offset":9836,"ancestor_record_offset":9156,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1416,"record_offset":9924,"parent_record_offset":9896,"ancestor_record_offset":9156,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":10016,"parent_record_offset":9988,"ancestor_record_offset":9156,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":9988,"parent_record_offset":9960,"ancestor_record_offset":9156,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1416,"record_offset":10064,"parent_record_offset":10016,"ancestor_record_offset":9156,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1416,"record_offset":7152,"rva":18904544,"va":5387613664}
// 编译实例 RVAs: 0x12075e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14123a910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t209286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387613664,"raw_hex":"e886310300","size":5,"soft_fail":false,"target_va":5387823376,"va":5387614085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fd2ff0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14465342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387613664,"raw_hex":"e83eb9dc00","size":5,"soft_fail":false,"target_va":5402079216,"va":5387613869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23927331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387613664,"raw_hex":"e8231a6d01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387614728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*(%r9)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387613664,"raw_hex":"41ff11","size":3,"soft_fail":false,"va":5387613921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":7652,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1416,"record_offset":8456,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1416,"record_offset":7736,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":7624,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":70819,"source_line":55,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1416,"record_offset":8568,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":9543,"source_line":89,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":1416,"record_offset":8056,"parent_record_offset":7736,"ancestor_record_offset":7152,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1416,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":7152,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1416,"record_offset":7792,"parent_record_offset":7736,"ancestor_record_offset":7152,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":7152,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":7152,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":1416,"record_offset":8196,"parent_record_offset":7736,"ancestor_record_offset":7152,"inlinee":14445,"source_line":56,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":8224,"parent_record_offset":7736,"ancestor_record_offset":7152,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1416,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":7152,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1416,"record_offset":8496,"parent_record_offset":8456,"ancestor_record_offset":7152,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":7152,"inlinee":7193,"source_line":79,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":7152,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":7820,"parent_record_offset":7792,"ancestor_record_offset":7152,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":8280,"parent_record_offset":7736,"ancestor_record_offset":7152,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1416,"record_offset":7992,"parent_record_offset":7964,"ancestor_record_offset":7152,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1416,"record_offset":8020,"parent_record_offset":7964,"ancestor_record_offset":7152,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1416,"record_offset":8632,"parent_record_offset":8600,"ancestor_record_offset":7152,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1416,"record_offset":8600,"parent_record_offset":8568,"ancestor_record_offset":7152,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$101::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1416,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":7152,"inlinee":25047,"source_line":17,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1416,"record_offset":7940,"parent_record_offset":7736,"ancestor_record_offset":7152,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1416,"record_offset":8684,"parent_record_offset":8632,"ancestor_record_offset":7152,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":7872,"parent_record_offset":7848,"ancestor_record_offset":7152,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_right_until::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19E6A
// 首条位置: {"module":3011,"inlinee":106090}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":3011,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":1500,"inlinee":106091,"source_line":442,"archive":"_pdb/B/modules/03011.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_right_until::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=56
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6039
// 首条位置: {"module":401,"inlinee":24633}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":7872,"inlinee":7193,"source_line":79,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":14468,"inlinee":70931,"source_line":340,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1436,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":3264,"inlinee":70811,"source_line":94,"archive":"_pdb/B/modules/01436.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":24634,"source_line":372,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":401,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":14700,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1416,"record_offset":3048,"rva":18902736,"va":5387611856}
// 编译实例 RVAs: 0x1206ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141fcf5b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14452156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387611856,"raw_hex":"e8bc85dc00","size":5,"soft_fail":false,"target_va":5402064304,"va":5387612143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23928840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387611856,"raw_hex":"e808206d01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387612931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23929196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387611856,"raw_hex":"e86c216d01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387612863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387611856,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5387612079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_left_until
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":3368,"parent_record_offset":3048,"ancestor_record_offset":3048,"inlinee":70816,"source_line":286,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1416,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":3048,"inlinee":7523,"source_line":1826,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1416,"record_offset":3848,"parent_record_offset":3808,"ancestor_record_offset":3048,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1416,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":3048,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1416,"record_offset":3692,"parent_record_offset":3636,"ancestor_record_offset":3048,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1416,"record_offset":3876,"parent_record_offset":3808,"ancestor_record_offset":3048,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::nth
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1416,"record_offset":4128,"parent_record_offset":3808,"ancestor_record_offset":3048,"inlinee":29465,"source_line":47,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1416,"record_offset":3808,"parent_record_offset":3768,"ancestor_record_offset":3048,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":3048,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":3732,"parent_record_offset":3692,"ancestor_record_offset":3048,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":3048,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":3768,"parent_record_offset":3732,"ancestor_record_offset":3048,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1416,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3048,"inlinee":31726,"source_line":1201,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$16::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1416,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":3048,"inlinee":31725,"source_line":1347,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1416,"record_offset":5068,"parent_record_offset":4976,"ancestor_record_offset":3048,"inlinee":12495,"source_line":209,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":1416,"record_offset":4480,"parent_record_offset":3636,"ancestor_record_offset":3048,"inlinee":14445,"source_line":56,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":4508,"parent_record_offset":3636,"ancestor_record_offset":3048,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":4596,"parent_record_offset":3368,"ancestor_record_offset":3048,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1416,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3048,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1416,"record_offset":4044,"parent_record_offset":3808,"ancestor_record_offset":3048,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1416,"record_offset":4680,"parent_record_offset":3368,"ancestor_record_offset":3048,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1416,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":3048,"inlinee":12496,"source_line":750,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1416,"record_offset":6156,"parent_record_offset":6132,"ancestor_record_offset":3048,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":3048,"inlinee":7193,"source_line":79,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":3048,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1416,"record_offset":5012,"parent_record_offset":4976,"ancestor_record_offset":3048,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":4364,"parent_record_offset":3692,"ancestor_record_offset":3048,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":4716,"parent_record_offset":3368,"ancestor_record_offset":3048,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":4564,"parent_record_offset":3636,"ancestor_record_offset":3048,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1416,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":3048,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1416,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":3048,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1416,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":3048,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":3048,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":3972,"parent_record_offset":3948,"ancestor_record_offset":3048,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":5624,"parent_record_offset":5580,"ancestor_record_offset":3048,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":5324,"parent_record_offset":5284,"ancestor_record_offset":3048,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1416,"record_offset":5896,"parent_record_offset":5868,"ancestor_record_offset":3048,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1416,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":3048,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1416,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":3048,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1416,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":3048,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":3048,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":3948,"parent_record_offset":3924,"ancestor_record_offset":3048,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3048,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::nth_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":3048,"inlinee":29438,"source_line":407,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1416,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":3048,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":6236,"parent_record_offset":6212,"ancestor_record_offset":3048,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":3048,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":6132,"parent_record_offset":3368,"ancestor_record_offset":3048,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1416,"record_offset":5844,"parent_record_offset":4756,"ancestor_record_offset":3048,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1416,"record_offset":3636,"parent_record_offset":3368,"ancestor_record_offset":3048,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1416,"record_offset":6268,"parent_record_offset":6236,"ancestor_record_offset":3048,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1416,"record_offset":4188,"parent_record_offset":3768,"ancestor_record_offset":3048,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":3468,"parent_record_offset":3368,"ancestor_record_offset":3048,"inlinee":70818,"source_line":23,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":3496,"parent_record_offset":3368,"ancestor_record_offset":3048,"inlinee":70821,"source_line":36,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_left_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":3048,"inlinee":70808,"source_line":308,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1416,"record_offset":6188,"parent_record_offset":3368,"ancestor_record_offset":3048,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1416,"record_offset":2248,"rva":18902096,"va":5387611216}
// 编译实例 RVAs: 0x1206c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412380b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t201526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387611216,"raw_hex":"e836130300","size":5,"soft_fail":false,"target_va":5387813040,"va":5387611509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23929933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387611216,"raw_hex":"e84d246d01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387611838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23930233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387611216,"raw_hex":"e879256d01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387611826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387611216,"raw_hex":"e8c196ffff","size":5,"soft_fail":false,"target_va":5387584336,"va":5387611274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_left_until
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2588,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":70816,"source_line":286,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":2248,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":2248,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":2884,"parent_record_offset":2588,"ancestor_record_offset":2248,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1416,"record_offset":2968,"parent_record_offset":2588,"ancestor_record_offset":2248,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":3000,"parent_record_offset":2588,"ancestor_record_offset":2248,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1416,"record_offset":2772,"parent_record_offset":2588,"ancestor_record_offset":2248,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2688,"parent_record_offset":2588,"ancestor_record_offset":2248,"inlinee":70818,"source_line":23,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2800,"parent_record_offset":2588,"ancestor_record_offset":2248,"inlinee":70819,"source_line":55,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":2248,"inlinee":24437,"source_line":51,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1416,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":2248,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left_until
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114A0
// 首条位置: {"module":1416,"inlinee":70816}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":2884,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1416,"record_offset":2968,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":3000,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":6132,"parent_record_offset":3368,"ancestor_record_offset":3368,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1416,"record_offset":3636,"parent_record_offset":3368,"ancestor_record_offset":3368,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1416,"record_offset":2772,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2688,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":70818,"source_line":23,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":3496,"parent_record_offset":3368,"ancestor_record_offset":3368,"inlinee":70821,"source_line":36,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2800,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":70819,"source_line":55,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2828,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":24437,"source_line":51,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1416,"record_offset":6188,"parent_record_offset":3368,"ancestor_record_offset":3368,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1416,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":3368,"inlinee":7523,"source_line":1826,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1416,"record_offset":3848,"parent_record_offset":3808,"ancestor_record_offset":3368,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1416,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":3368,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1416,"record_offset":3692,"parent_record_offset":3636,"ancestor_record_offset":3368,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1416,"record_offset":3876,"parent_record_offset":3808,"ancestor_record_offset":3368,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::nth
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1416,"record_offset":4128,"parent_record_offset":3808,"ancestor_record_offset":3368,"inlinee":29465,"source_line":47,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1416,"record_offset":3808,"parent_record_offset":3768,"ancestor_record_offset":3368,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":2588,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":3732,"parent_record_offset":3692,"ancestor_record_offset":3368,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":2588,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":3768,"parent_record_offset":3732,"ancestor_record_offset":3368,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1416,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3368,"inlinee":31726,"source_line":1201,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$16::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1416,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":3368,"inlinee":31725,"source_line":1347,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1416,"record_offset":5068,"parent_record_offset":4976,"ancestor_record_offset":3368,"inlinee":12495,"source_line":209,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":1416,"record_offset":4480,"parent_record_offset":3636,"ancestor_record_offset":3368,"inlinee":14445,"source_line":56,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":4508,"parent_record_offset":3636,"ancestor_record_offset":3368,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1416,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3368,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1416,"record_offset":4044,"parent_record_offset":3808,"ancestor_record_offset":3368,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1416,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":3368,"inlinee":12496,"source_line":750,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1416,"record_offset":6156,"parent_record_offset":6132,"ancestor_record_offset":3368,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":3368,"inlinee":7193,"source_line":79,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":3368,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1416,"record_offset":5012,"parent_record_offset":4976,"ancestor_record_offset":3368,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":4364,"parent_record_offset":3692,"ancestor_record_offset":3368,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":4564,"parent_record_offset":3636,"ancestor_record_offset":3368,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1416,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":3368,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1416,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":3368,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1416,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":3368,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":3368,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":3972,"parent_record_offset":3948,"ancestor_record_offset":3368,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":5624,"parent_record_offset":5580,"ancestor_record_offset":3368,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1416,"record_offset":5324,"parent_record_offset":5284,"ancestor_record_offset":3368,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1416,"record_offset":5896,"parent_record_offset":5868,"ancestor_record_offset":3368,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1416,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":3368,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1416,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":3368,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1416,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":3368,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":3368,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":3948,"parent_record_offset":3924,"ancestor_record_offset":3368,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3368,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::nth_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1416,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":3368,"inlinee":29438,"source_line":407,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1416,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":3368,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":6236,"parent_record_offset":6212,"ancestor_record_offset":3368,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1416,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":3368,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1416,"record_offset":5844,"parent_record_offset":4756,"ancestor_record_offset":3368,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1416,"record_offset":6268,"parent_record_offset":6236,"ancestor_record_offset":3368,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1416,"record_offset":4188,"parent_record_offset":3768,"ancestor_record_offset":3368,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":2588,"inlinee":24437,"source_line":51,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_left_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":3368,"inlinee":70808,"source_line":308,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1416,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":2588,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1416,"record_offset":6316,"rva":18903824,"va":5387612944}
// 编译实例 RVAs: 0x1207310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141237c20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t198602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387612944,"raw_hex":"e8ca070300","size":5,"soft_fail":false,"target_va":5387811872,"va":5387613265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23928113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387612944,"raw_hex":"e8311d6d01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387613658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23928413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387612944,"raw_hex":"e85d1e6d01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387613646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-28681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387612944,"raw_hex":"e8f78fffff","size":5,"soft_fail":false,"target_va":5387584336,"va":5387613012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1416,"record_offset":6992,"parent_record_offset":6316,"ancestor_record_offset":6316,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1416,"record_offset":7076,"parent_record_offset":6316,"ancestor_record_offset":6316,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1416,"record_offset":7108,"parent_record_offset":6316,"ancestor_record_offset":6316,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1416,"record_offset":6880,"parent_record_offset":6316,"ancestor_record_offset":6316,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":6796,"parent_record_offset":6316,"ancestor_record_offset":6316,"inlinee":70818,"source_line":23,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":6908,"parent_record_offset":6316,"ancestor_record_offset":6316,"inlinee":70819,"source_line":55,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":6936,"parent_record_offset":6316,"ancestor_record_offset":6316,"inlinee":24437,"source_line":51,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6316,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1416,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":6316,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1416,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":6316,"inlinee":24437,"source_line":51,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1416,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":6316,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_left_until::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1158A
// 首条位置: {"module":1452,"inlinee":71050}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":1452,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":4724,"inlinee":71052,"source_line":440,"archive":"_pdb/B/modules/01452.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_left_until::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11498
// 首条位置: {"module":1416,"inlinee":70808}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":4412,"inlinee":7193,"source_line":79,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::start
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F76
// 首条位置: {"module":390,"inlinee":24438}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::end
// 记录形式: inline_instance, inlinee_declaration; instances=28
// 源码行: 338
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F77
// 首条位置: {"module":390,"inlinee":24439}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::iter_coords
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F70
// 首条位置: {"module":390,"inlinee":24432}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":17092,"parent_record_offset":17060,"ancestor_record_offset":17060,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::measure_length
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 414
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F6F
// 首条位置: {"module":390,"inlinee":24431}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":17124,"parent_record_offset":17036,"ancestor_record_offset":17036,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":17180,"parent_record_offset":17036,"ancestor_record_offset":17036,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::iter_coords
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":17036,"inlinee":24432,"source_line":345,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":17092,"parent_record_offset":17060,"ancestor_record_offset":17036,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":390,"record_offset":17148,"parent_record_offset":17124,"ancestor_record_offset":17036,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":390,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":17036,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::measure_length::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 416
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1149A
// 首条位置: {"module":1416,"inlinee":70810}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1416,"record_offset":1036,"parent_record_offset":904,"ancestor_record_offset":904,"inlinee":7193,"source_line":79,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1416,"record_offset":1176,"parent_record_offset":904,"ancestor_record_offset":904,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1416,"record_offset":944,"parent_record_offset":904,"ancestor_record_offset":904,"inlinee":24424,"source_line":226,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1416,"record_offset":988,"parent_record_offset":944,"ancestor_record_offset":904,"inlinee":24425,"source_line":3058,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1416,"record_offset":1420,"parent_record_offset":1388,"ancestor_record_offset":904,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1416,"record_offset":1264,"parent_record_offset":1224,"ancestor_record_offset":904,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1416,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":904,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1416,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":904,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1416,"record_offset":1388,"parent_record_offset":1200,"ancestor_record_offset":904,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::measure_length::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114A6
// 首条位置: {"module":1416,"inlinee":70822}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1417,"record_offset":3156,"parent_record_offset":3136,"ancestor_record_offset":3136,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1417,"record_offset":3240,"parent_record_offset":3136,"ancestor_record_offset":3136,"inlinee":9543,"source_line":89,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1417,"record_offset":3184,"parent_record_offset":3156,"ancestor_record_offset":3136,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1417,"record_offset":3296,"parent_record_offset":3268,"ancestor_record_offset":3136,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1417,"record_offset":3268,"parent_record_offset":3240,"ancestor_record_offset":3136,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1417,"record_offset":3332,"parent_record_offset":3296,"ancestor_record_offset":3136,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01417.jsonl.gz"}
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
// 完整结构及原始类型记录: curve_slice.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::curve::curve_slice::CurvePoints
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::curve::curve_slice::CurveSlice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 15; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_left", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_left", "arguments": ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_left_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemantic3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: curve_slice.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/B-process/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::curve::curve_slice::CurveSlice", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/B-profile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::curve::curve_slice::CurveSlice", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: curve_slice.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurvePoints::is_empty
// 信息 ID: 2993d95dcdb89db9e48b2420
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=2688; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=3468; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1416, inline_record=6796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::end
// 信息 ID: 98fbcc8cf1c1f96c00ab1583
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=19876; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=10012; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=12616; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=17972; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=444, inline_record=15064; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32732; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=33040; 不是独立调用边。
// [PDB 内联宿主] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors; module=466, inline_record=12944; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 信息 ID: e03d50dfdbc3263fff399ec0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] slice2$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left -> ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12438d0 -> 0x1206ed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 信息 ID: 49accd417918a008cadad147
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7541b2 -> 0x1206c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left_until
// 信息 ID: 701cd256ab7f931468395912
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=2588; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=3368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>
// 信息 ID: 478f5445cb7e31668a522ff3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left_until -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x751712 -> 0x1207310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right
// 信息 ID: c7c24bb0a1090fcc72dca6ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=392, inline_record=14876; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=14364; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=16396; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=17956; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=9984; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=12588; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=17944; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=444, inline_record=15036; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until
// 信息 ID: d2e0cf23e1f922e742033db3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=14400; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=16436; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=17992; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=13680; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1436, inline_record=2968; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1436, inline_record=4796; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1436, inline_record=6360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: a511b0165ae79a047ff0938f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] slice2$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::impl$6::extend_right::closure_env$0 -> ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemantic3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemantic3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x7c2ba5 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x7c2f4b -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f2513 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f529e -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x81d359 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x127faa0 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x127f822 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows @ 0x22cf845 -> 0x22e1e40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: b2efae537170f7aef8e57043
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] slice2$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::impl$6::extend_right::closure_env$0 -> ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121e975 -> 0x1207c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x12275dc -> 0x1207c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12438ba -> 0x1207c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x7c2ba5 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x7c2f4b -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f2513 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f529e -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x81d359 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x127faa0 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x127f822 -> 0x12898e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: 1d0be0954df7c0b32887e286
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::impl$6::extend_right::closure_env$0 -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x754190 -> 0x1207a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>
// 信息 ID: 1c04cfc5492f05d8ab2a2d2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x751723 -> 0x12075e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_coord_range
// 信息 ID: 45dc93b5d738d09a889f867f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=390, inline_record=16940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_curve_range
// 信息 ID: 60e53e3429cc9011fdcef477
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 17 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=9780; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=12420; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=17748; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=444, inline_record=14840; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32588; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32896; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=33204; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=35148; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_point_range
// 信息 ID: 4519828a9940e2445d338bd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=19632; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=9816; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=12456; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=17780; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=444, inline_record=14872; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32616; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32924; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=33232; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::iter_coords
// 信息 ID: 1496a04a53cd1ab2f973e4e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=390, inline_record=17060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::measure_length
// 信息 ID: 72754070048bc7b3aababf35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=390, inline_record=17036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::remap
// 信息 ID: 833fe1288de37a81fd4b2055
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=392, inline_record=14844; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=9920; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=12556; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=17880; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=444, inline_record=14972; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32672; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32980; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=33288; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::remap_2d
// 信息 ID: 8d96e2334a4a1f8f46eaa8cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=392, inline_record=14820; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=9892; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=12532; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=17852; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=444, inline_record=14944; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32648; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32956; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=33264; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::start
// 信息 ID: bcfb867089a73eb5d0aa76f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=19848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$0::iter
// 信息 ID: 3b770c84fa4c183f3200fdc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=3496; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1416, inline_record=11528; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1538, inline_record=2056; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1538, inline_record=5044; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=3814, inline_record=2040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$1::iter
// 信息 ID: 66e5efdfe5ddb80406d79d4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=2800; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1416, inline_record=6908; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1416, inline_record=7624; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1416, inline_record=9784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$1::len
// 信息 ID: 842107630d3b7105b396adee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=19676; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=19732; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=390, inline_record=19788; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=2712; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=2828; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1416, inline_record=6820; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1416, inline_record=6936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$3::from_coord::closure$0
// 信息 ID: 531bd5cab766ac09f35fef5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$3::from_coord", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$3::from_coord::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$3::from_coord::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$3::from_coord::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1416, inline_record=1824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$3::from_point_range::closure$0
// 信息 ID: 2ed864fb0d75ef86768ee8e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$3::from_point_range", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$5::remap_2d::closure$0
// 信息 ID: fee7c3e5c2b5792e3159182f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$5::remap_2d", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1416, inline_record=2220; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1538, inline_record=632; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=3814, inline_record=616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_left_until::closure$0
// 信息 ID: 27b26a5a8a960bcd1fa7f4d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_left_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1452, inline_record=4724; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1455, inline_record=7180; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1455, inline_record=7940; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1455, inline_record=8844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_left_until::closure$1
// 信息 ID: f28e49d0833c77128fe5007d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_left_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=4412; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1416, inline_record=5776; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1452, inline_record=5160; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; module=1452, inline_record=10408; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1455, inline_record=7588; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1455, inline_record=9252; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; module=3001, inline_record=6688; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; module=3001, inline_record=7912; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_right_until::closure$0
// 信息 ID: 2c019431973118927abcc22a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_right_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; module=3011, inline_record=1500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_right_until::closure$1
// 信息 ID: c488be75f4075761207ac456
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_right_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=14700; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=15268; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=17224; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=17752; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=18764; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=19292; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1416, inline_record=7872; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1416, inline_record=8968; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::measure_length::closure$0
// 信息 ID: c18bc29f30485c35c66078c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::measure_length", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1416, inline_record=904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::measure_length::closure$1
// 信息 ID: 162f058d9c86073217beaaae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::measure_length", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; module=1417, inline_record=3136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
