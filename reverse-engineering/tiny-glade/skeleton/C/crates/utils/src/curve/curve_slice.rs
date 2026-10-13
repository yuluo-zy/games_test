// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/curve/curve_slice.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5ba0f0897347ce603678933b5ffef35ef3d65581fa41dbb973788865b0634b8d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: curve_slice.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 42, 63, 196, 249, 396, 399, 401, 418, 423, 449, 470, 478, 492, 557, 584, 585, 1459, 1478, 1479, 1482, 1483, 1490, 1491, 1493, 1495, 1499, 1504, 1511, 1525, 1541, 1559, 1579, 1593, 1603, 1608, 1662, 1699, 1705, 2743, 2746, 3147, 3148, 4061, 4075, 4078
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// IPI IDs: 0x136E3
// 首条位置: {"module":1478,"inlinee":79587}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2692,"inlinee":26197,"source_line":51,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1478,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2692,"inlinee":26182,"source_line":199,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$0::iter
// 记录形式: inline_instance, inlinee_declaration; instances=19
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2219
// 首条位置: {"module":196,"inlinee":8729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":196,"record_offset":53644,"parent_record_offset":53620,"ancestor_record_offset":53620,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":196,"record_offset":53716,"parent_record_offset":53692,"ancestor_record_offset":53620,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":196,"record_offset":53668,"parent_record_offset":53644,"ancestor_record_offset":53620,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":196,"record_offset":53692,"parent_record_offset":53668,"ancestor_record_offset":53620,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$1::len
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6655
// 首条位置: {"module":399,"inlinee":26197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":399,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":6156,"inlinee":26182,"source_line":199,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$1::iter
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136E4
// 首条位置: {"module":1478,"inlinee":79588}
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
// 记录形式: inline_instance, inlinee_declaration; instances=50
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E5A
// 首条位置: {"module":396,"inlinee":15962}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=27
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":396,"record_offset":13996,"parent_record_offset":13960,"ancestor_record_offset":13960,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":396,"record_offset":14036,"parent_record_offset":13996,"ancestor_record_offset":13960,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::from_point_range
// 记录形式: inline_instance, inlinee_declaration; instances=58
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E4C
// 首条位置: {"module":196,"inlinee":15948}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":396,"record_offset":14036,"parent_record_offset":13996,"ancestor_record_offset":13996,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":399,"record_offset":6156,"parent_record_offset":6112,"ancestor_record_offset":6112,"inlinee":26197,"source_line":51,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":399,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":6112,"inlinee":26182,"source_line":199,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$3::from_point_range::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9171
// 首条位置: {"module":584,"inlinee":37233}
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
// IPI IDs: 0x664E
// 首条位置: {"module":399,"inlinee":26190}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::impl$11::sub
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":399,"record_offset":4472,"parent_record_offset":4436,"ancestor_record_offset":4436,"inlinee":26192,"source_line":1358,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::remap
// 记录形式: inline_instance, inlinee_declaration; instances=69
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E59
// 首条位置: {"module":249,"inlinee":15961}
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
// 记录形式: inline_instance, inlinee_declaration; instances=48
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x662A
// 首条位置: {"module":396,"inlinee":26154}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=25
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":396,"record_offset":14096,"parent_record_offset":14072,"ancestor_record_offset":14072,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$5::remap_2d::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9173
// 首条位置: {"module":584,"inlinee":37235}
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
// 记录形式: inline_instance, inlinee_declaration; instances=78
// 源码行: 236
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x45A4
// 首条位置: {"module":249,"inlinee":17828}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14592,"parent_record_offset":14560,"ancestor_record_offset":14560,"inlinee":15959,"source_line":243,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":14964,"parent_record_offset":14936,"ancestor_record_offset":14560,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1490,"record_offset":14844,"parent_record_offset":14820,"ancestor_record_offset":14560,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1490,"record_offset":14868,"parent_record_offset":14844,"ancestor_record_offset":14560,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":14820,"parent_record_offset":14592,"ancestor_record_offset":14560,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":14904,"parent_record_offset":14592,"ancestor_record_offset":14560,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14560,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":14560,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":14560,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":14560,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14680,"parent_record_offset":14592,"ancestor_record_offset":14560,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1490,"record_offset":14936,"parent_record_offset":14592,"ancestor_record_offset":14560,"inlinee":15946,"source_line":121,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E57
// 首条位置: {"module":1490,"inlinee":15959}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":14820,"parent_record_offset":14592,"ancestor_record_offset":14592,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":14904,"parent_record_offset":14592,"ancestor_record_offset":14592,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14680,"parent_record_offset":14592,"ancestor_record_offset":14592,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1490,"record_offset":14936,"parent_record_offset":14592,"ancestor_record_offset":14592,"inlinee":15946,"source_line":121,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":14964,"parent_record_offset":14936,"ancestor_record_offset":14592,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1490,"record_offset":14844,"parent_record_offset":14820,"ancestor_record_offset":14592,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1490,"record_offset":14868,"parent_record_offset":14844,"ancestor_record_offset":14592,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14592,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":14592,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":14592,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":14592,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1478,"record_offset":6872,"rva":21147600,"va":5389856720}
// 编译实例 RVAs: 0x142afd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26765213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389856720,"raw_hex":"e89d679801","size":5,"soft_fail":false,"target_va":5416622208,"va":5389856990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1478,"record_offset":8536,"parent_record_offset":6872,"ancestor_record_offset":6872,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1478,"record_offset":7580,"parent_record_offset":6872,"ancestor_record_offset":6872,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1478,"record_offset":8560,"parent_record_offset":6872,"ancestor_record_offset":6872,"inlinee":15946,"source_line":121,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1478,"record_offset":8588,"parent_record_offset":8560,"ancestor_record_offset":6872,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1478,"record_offset":7764,"parent_record_offset":7732,"ancestor_record_offset":6872,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1478,"record_offset":7928,"parent_record_offset":7896,"ancestor_record_offset":6872,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1478,"record_offset":7992,"parent_record_offset":7960,"ancestor_record_offset":6872,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1478,"record_offset":7620,"parent_record_offset":7580,"ancestor_record_offset":6872,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":1478,"record_offset":7732,"parent_record_offset":7704,"ancestor_record_offset":6872,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":7676,"parent_record_offset":7648,"ancestor_record_offset":6872,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":7704,"parent_record_offset":7676,"ancestor_record_offset":6872,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$10::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1478,"record_offset":8280,"parent_record_offset":8148,"ancestor_record_offset":6872,"inlinee":9853,"source_line":1023,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1478,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":6872,"inlinee":9856,"source_line":230,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":8020,"parent_record_offset":7992,"ancestor_record_offset":6872,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1478,"record_offset":8328,"parent_record_offset":8304,"ancestor_record_offset":6872,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1478,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":6872,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":8376,"parent_record_offset":8052,"ancestor_record_offset":6872,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":7960,"parent_record_offset":7928,"ancestor_record_offset":6872,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1478,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":6872,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1478,"record_offset":7792,"parent_record_offset":7732,"ancestor_record_offset":6872,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1478,"record_offset":7848,"parent_record_offset":7732,"ancestor_record_offset":6872,"inlinee":7948,"source_line":229,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1478,"record_offset":7896,"parent_record_offset":7580,"ancestor_record_offset":6872,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1478,"record_offset":8444,"parent_record_offset":8424,"ancestor_record_offset":6872,"inlinee":9860,"source_line":375,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1478,"record_offset":7648,"parent_record_offset":7620,"ancestor_record_offset":6872,"inlinee":9851,"source_line":89,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":6872,"inlinee":9858,"source_line":255,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1478,"record_offset":8464,"parent_record_offset":8444,"ancestor_record_offset":6872,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":196,"record_offset":53944,"rva":5609584,"va":5374318704}
// 编译实例 RVAs: 0x559870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401f2cd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3566675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374318704,"raw_hex":"e8ad93c9ff","size":5,"soft_fail":false,"target_va":5370752208,"va":5374318878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42303085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374318704,"raw_hex":"e86d7e8502","size":5,"soft_fail":false,"target_va":5416622208,"va":5374319118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":196,"record_offset":54884,"parent_record_offset":53944,"ancestor_record_offset":53944,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":196,"record_offset":54968,"parent_record_offset":53944,"ancestor_record_offset":53944,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":196,"record_offset":54744,"parent_record_offset":53944,"ancestor_record_offset":53944,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":196,"record_offset":55000,"parent_record_offset":53944,"ancestor_record_offset":53944,"inlinee":15946,"source_line":121,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":55028,"parent_record_offset":55000,"ancestor_record_offset":53944,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":196,"record_offset":54908,"parent_record_offset":54884,"ancestor_record_offset":53944,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":196,"record_offset":54932,"parent_record_offset":54908,"ancestor_record_offset":53944,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":196,"record_offset":54840,"parent_record_offset":54816,"ancestor_record_offset":53944,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":196,"record_offset":54792,"parent_record_offset":54768,"ancestor_record_offset":53944,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":196,"record_offset":54816,"parent_record_offset":54792,"ancestor_record_offset":53944,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":196,"record_offset":54768,"parent_record_offset":54744,"ancestor_record_offset":53944,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":196,"record_offset":52820,"rva":5609584,"va":5374318704}
// 编译实例 RVAs: 0x559870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401f2cd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3566675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374318704,"raw_hex":"e8ad93c9ff","size":5,"soft_fail":false,"target_va":5370752208,"va":5374318878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42303085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374318704,"raw_hex":"e86d7e8502","size":5,"soft_fail":false,"target_va":5416622208,"va":5374319118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":196,"record_offset":53760,"parent_record_offset":52820,"ancestor_record_offset":52820,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":196,"record_offset":53844,"parent_record_offset":52820,"ancestor_record_offset":52820,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":196,"record_offset":53620,"parent_record_offset":52820,"ancestor_record_offset":52820,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":196,"record_offset":53876,"parent_record_offset":52820,"ancestor_record_offset":52820,"inlinee":15946,"source_line":121,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":53904,"parent_record_offset":53876,"ancestor_record_offset":52820,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":196,"record_offset":53784,"parent_record_offset":53760,"ancestor_record_offset":52820,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":196,"record_offset":53808,"parent_record_offset":53784,"ancestor_record_offset":52820,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":196,"record_offset":53716,"parent_record_offset":53692,"ancestor_record_offset":52820,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":196,"record_offset":53668,"parent_record_offset":53644,"ancestor_record_offset":52820,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":196,"record_offset":53692,"parent_record_offset":53668,"ancestor_record_offset":52820,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":196,"record_offset":53644,"parent_record_offset":53620,"ancestor_record_offset":52820,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1478,"record_offset":10112,"rva":21148352,"va":5389857472}
// 编译实例 RVAs: 0x142b2c0, 0x14dff60, 0x28d4ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1414f0e10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t809540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389857472,"raw_hex":"e8445a0c00","size":5,"soft_fail":false,"target_va":5390667280,"va":5389857735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26763980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389857472,"raw_hex":"e8cc629801","size":5,"soft_fail":false,"target_va":5416622208,"va":5389858223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5389857472,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5389857671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":11560,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":11644,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1478,"record_offset":10680,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":11420,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1478,"record_offset":11736,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1478,"record_offset":11868,"parent_record_offset":11840,"ancestor_record_offset":10112,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1478,"record_offset":11032,"parent_record_offset":11000,"ancestor_record_offset":10112,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1478,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":10112,"inlinee":8137,"source_line":36,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1478,"record_offset":10736,"parent_record_offset":10680,"ancestor_record_offset":10112,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":1478,"record_offset":11000,"parent_record_offset":10848,"ancestor_record_offset":10112,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":11584,"parent_record_offset":11560,"ancestor_record_offset":10112,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":10812,"parent_record_offset":10776,"ancestor_record_offset":10112,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":11608,"parent_record_offset":11584,"ancestor_record_offset":10112,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":10848,"parent_record_offset":10812,"ancestor_record_offset":10112,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$10::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1478,"record_offset":10884,"parent_record_offset":10848,"ancestor_record_offset":10112,"inlinee":9853,"source_line":1023,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1478,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10112,"inlinee":9856,"source_line":230,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":11332,"parent_record_offset":10680,"ancestor_record_offset":10112,"inlinee":8135,"source_line":507,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1478,"record_offset":10968,"parent_record_offset":10884,"ancestor_record_offset":10112,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1478,"record_offset":11116,"parent_record_offset":11000,"ancestor_record_offset":10112,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1478,"record_offset":10936,"parent_record_offset":10912,"ancestor_record_offset":10112,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1478,"record_offset":11676,"parent_record_offset":11644,"ancestor_record_offset":10112,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1478,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":10112,"inlinee":5608,"source_line":79,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1478,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":10112,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":11216,"parent_record_offset":10736,"ancestor_record_offset":10112,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":11388,"parent_record_offset":10680,"ancestor_record_offset":10112,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1478,"record_offset":12452,"parent_record_offset":12428,"ancestor_record_offset":10112,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1478,"record_offset":11516,"parent_record_offset":11492,"ancestor_record_offset":10112,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1478,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":10112,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1478,"record_offset":12404,"parent_record_offset":12372,"ancestor_record_offset":10112,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1478,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":10112,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1478,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10112,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1478,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":10112,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1478,"record_offset":11060,"parent_record_offset":11000,"ancestor_record_offset":10112,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::nth
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1478,"record_offset":11172,"parent_record_offset":11000,"ancestor_record_offset":10112,"inlinee":7948,"source_line":229,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1478,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":10112,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":10112,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":11764,"parent_record_offset":11736,"ancestor_record_offset":10112,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1478,"record_offset":12624,"parent_record_offset":11916,"ancestor_record_offset":10112,"inlinee":7465,"source_line":388,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1478,"record_offset":11840,"parent_record_offset":11792,"ancestor_record_offset":10112,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1478,"record_offset":10776,"parent_record_offset":10736,"ancestor_record_offset":10112,"inlinee":9851,"source_line":89,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":10112,"inlinee":9858,"source_line":255,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":584,"record_offset":29476,"rva":12489472,"va":5381198592}
// 编译实例 RVAs: 0xbe9300, 0x142b2c0, 0x14dff60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1414f0e10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9468420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381198592,"raw_hex":"e8047a9000","size":5,"soft_fail":false,"target_va":5390667280,"va":5381198855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35422860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381198592,"raw_hex":"e88c821c02","size":5,"soft_fail":false,"target_va":5416622208,"va":5381199343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381198592,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5381198791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":584,"record_offset":30944,"parent_record_offset":29476,"ancestor_record_offset":29476,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":584,"record_offset":31028,"parent_record_offset":29476,"ancestor_record_offset":29476,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":584,"record_offset":30064,"parent_record_offset":29476,"ancestor_record_offset":29476,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":584,"record_offset":30804,"parent_record_offset":29476,"ancestor_record_offset":29476,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":584,"record_offset":31120,"parent_record_offset":29476,"ancestor_record_offset":29476,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":584,"record_offset":31252,"parent_record_offset":31224,"ancestor_record_offset":29476,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":584,"record_offset":30416,"parent_record_offset":30384,"ancestor_record_offset":29476,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":584,"record_offset":30740,"parent_record_offset":30716,"ancestor_record_offset":29476,"inlinee":8137,"source_line":36,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":584,"record_offset":30120,"parent_record_offset":30064,"ancestor_record_offset":29476,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":584,"record_offset":30384,"parent_record_offset":30232,"ancestor_record_offset":29476,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":584,"record_offset":30968,"parent_record_offset":30944,"ancestor_record_offset":29476,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":584,"record_offset":30196,"parent_record_offset":30160,"ancestor_record_offset":29476,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":584,"record_offset":30992,"parent_record_offset":30968,"ancestor_record_offset":29476,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":584,"record_offset":30232,"parent_record_offset":30196,"ancestor_record_offset":29476,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$10::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":584,"record_offset":30268,"parent_record_offset":30232,"ancestor_record_offset":29476,"inlinee":9853,"source_line":1023,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":584,"record_offset":30296,"parent_record_offset":30268,"ancestor_record_offset":29476,"inlinee":9856,"source_line":230,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":584,"record_offset":30716,"parent_record_offset":30064,"ancestor_record_offset":29476,"inlinee":8135,"source_line":507,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":584,"record_offset":30352,"parent_record_offset":30268,"ancestor_record_offset":29476,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":584,"record_offset":30500,"parent_record_offset":30384,"ancestor_record_offset":29476,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":584,"record_offset":30320,"parent_record_offset":30296,"ancestor_record_offset":29476,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":584,"record_offset":31060,"parent_record_offset":31028,"ancestor_record_offset":29476,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":584,"record_offset":30672,"parent_record_offset":30648,"ancestor_record_offset":29476,"inlinee":5608,"source_line":79,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":584,"record_offset":30624,"parent_record_offset":30600,"ancestor_record_offset":29476,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":584,"record_offset":30600,"parent_record_offset":30120,"ancestor_record_offset":29476,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":584,"record_offset":30772,"parent_record_offset":30064,"ancestor_record_offset":29476,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":584,"record_offset":31836,"parent_record_offset":31812,"ancestor_record_offset":29476,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":584,"record_offset":30900,"parent_record_offset":30876,"ancestor_record_offset":29476,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":584,"record_offset":31812,"parent_record_offset":31788,"ancestor_record_offset":29476,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":584,"record_offset":31788,"parent_record_offset":31756,"ancestor_record_offset":29476,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":584,"record_offset":30468,"parent_record_offset":30444,"ancestor_record_offset":29476,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":584,"record_offset":30852,"parent_record_offset":30828,"ancestor_record_offset":29476,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":584,"record_offset":30876,"parent_record_offset":30852,"ancestor_record_offset":29476,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":584,"record_offset":30444,"parent_record_offset":30384,"ancestor_record_offset":29476,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::nth
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":584,"record_offset":30556,"parent_record_offset":30384,"ancestor_record_offset":29476,"inlinee":7948,"source_line":229,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":584,"record_offset":30828,"parent_record_offset":30804,"ancestor_record_offset":29476,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":584,"record_offset":31176,"parent_record_offset":31148,"ancestor_record_offset":29476,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":584,"record_offset":31148,"parent_record_offset":31120,"ancestor_record_offset":29476,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":584,"record_offset":32008,"parent_record_offset":31300,"ancestor_record_offset":29476,"inlinee":7465,"source_line":388,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":584,"record_offset":31224,"parent_record_offset":31176,"ancestor_record_offset":29476,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":584,"record_offset":30160,"parent_record_offset":30120,"ancestor_record_offset":29476,"inlinee":9851,"source_line":89,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":584,"record_offset":30648,"parent_record_offset":30624,"ancestor_record_offset":29476,"inlinee":9858,"source_line":255,"archive":"_pdb/C/modules/00584.jsonl.gz"}
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
// 首条位置: {"module":1478,"record_offset":9124,"rva":21147904,"va":5389857024}
// 编译实例 RVAs: 0x142b100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141473160
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t294830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389857024,"raw_hex":"e8ae7f0400","size":5,"soft_fail":false,"target_va":5390152032,"va":5389857197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26764751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389857024,"raw_hex":"e8cf659801","size":5,"soft_fail":false,"target_va":5416622208,"va":5389857452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":9780,"parent_record_offset":9124,"ancestor_record_offset":9124,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":9864,"parent_record_offset":9124,"ancestor_record_offset":9124,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":9752,"parent_record_offset":9124,"ancestor_record_offset":9124,"inlinee":79588,"source_line":55,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1478,"record_offset":9928,"parent_record_offset":9124,"ancestor_record_offset":9124,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1478,"record_offset":10060,"parent_record_offset":10032,"ancestor_record_offset":9124,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":9804,"parent_record_offset":9780,"ancestor_record_offset":9124,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":9124,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1478,"record_offset":9892,"parent_record_offset":9864,"ancestor_record_offset":9124,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":9984,"parent_record_offset":9956,"ancestor_record_offset":9124,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":9124,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1478,"record_offset":10032,"parent_record_offset":9984,"ancestor_record_offset":9124,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01478.jsonl.gz"}
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
// 首条位置: {"module":1478,"record_offset":4468,"rva":21146608,"va":5389855728}
// 编译实例 RVAs: 0x142abf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1424e1120
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17523818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389855728,"raw_hex":"e86a640b01","size":5,"soft_fail":false,"target_va":5407379744,"va":5389855921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26765500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389855728,"raw_hex":"e8bc689801","size":5,"soft_fail":false,"target_va":5416622208,"va":5389856703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*(%r9)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5389855728,"raw_hex":"41ff11","size":3,"soft_fail":false,"va":5389855954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":4968,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1478,"record_offset":5816,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1478,"record_offset":5052,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":4940,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":79588,"source_line":55,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1478,"record_offset":5920,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":10939,"source_line":89,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":1478,"record_offset":5328,"parent_record_offset":5052,"ancestor_record_offset":4468,"inlinee":6914,"source_line":236,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1478,"record_offset":6052,"parent_record_offset":6024,"ancestor_record_offset":4468,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1478,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":4468,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1478,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":4468,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1478,"record_offset":5096,"parent_record_offset":5052,"ancestor_record_offset":4468,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":4468,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":4468,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":4468,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1478,"record_offset":5264,"parent_record_offset":5240,"ancestor_record_offset":4468,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1478,"record_offset":5852,"parent_record_offset":5816,"ancestor_record_offset":4468,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1478,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":4468,"inlinee":5608,"source_line":79,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1478,"record_offset":5148,"parent_record_offset":5120,"ancestor_record_offset":4468,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":4468,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":4468,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":5700,"parent_record_offset":5052,"ancestor_record_offset":4468,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1478,"record_offset":5976,"parent_record_offset":5948,"ancestor_record_offset":4468,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1478,"record_offset":5948,"parent_record_offset":5920,"ancestor_record_offset":4468,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$102::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1478,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":4468,"inlinee":9872,"source_line":20,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1478,"record_offset":5412,"parent_record_offset":5052,"ancestor_record_offset":4468,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1478,"record_offset":5240,"parent_record_offset":5052,"ancestor_record_offset":4468,"inlinee":7465,"source_line":388,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1478,"record_offset":6024,"parent_record_offset":5976,"ancestor_record_offset":4468,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":4468,"inlinee":9858,"source_line":255,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_right_until::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC11
// 首条位置: {"module":3147,"inlinee":117777}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":3147,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":117778,"source_line":442,"archive":"_pdb/C/modules/03147.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_right_until::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=59
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2682
// 首条位置: {"module":63,"inlinee":9858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":584,"record_offset":30672,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":5608,"source_line":79,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$4
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":63,"record_offset":2724,"parent_record_offset":2704,"ancestor_record_offset":2704,"inlinee":9860,"source_line":375,"archive":"_pdb/C/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":63,"record_offset":2744,"parent_record_offset":2724,"ancestor_record_offset":2704,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00063.jsonl.gz"}
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
// 首条位置: {"module":1478,"record_offset":1168,"rva":21144976,"va":5389854096}
// 编译实例 RVAs: 0x142a590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141473450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t298485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389854096,"raw_hex":"e8f58d0400","size":5,"soft_fail":false,"target_va":5390152784,"va":5389854294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26767069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389854096,"raw_hex":"e8dd6e9801","size":5,"soft_fail":false,"target_va":5416621600,"va":5389854526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26767653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389854096,"raw_hex":"e825719801","size":5,"soft_fail":false,"target_va":5416622208,"va":5389854550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_left_until
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":1488,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":79585,"source_line":286,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1478,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":1168,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1168,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1168,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":1812,"parent_record_offset":1488,"ancestor_record_offset":1168,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1478,"record_offset":1896,"parent_record_offset":1488,"ancestor_record_offset":1168,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1478,"record_offset":2004,"parent_record_offset":1976,"ancestor_record_offset":1168,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":1936,"parent_record_offset":1488,"ancestor_record_offset":1168,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1478,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1168,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1478,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":1168,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1478,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1168,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1478,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1168,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":2096,"parent_record_offset":2068,"ancestor_record_offset":1168,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":1168,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":1976,"parent_record_offset":1488,"ancestor_record_offset":1168,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1478,"record_offset":2144,"parent_record_offset":2096,"ancestor_record_offset":1168,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1478,"record_offset":1784,"parent_record_offset":1488,"ancestor_record_offset":1168,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":1616,"parent_record_offset":1488,"ancestor_record_offset":1168,"inlinee":79587,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":1644,"parent_record_offset":1488,"ancestor_record_offset":1168,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1478,"record_offset":2040,"parent_record_offset":1488,"ancestor_record_offset":1168,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/01478.jsonl.gz"}
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
// 首条位置: {"module":1478,"record_offset":2228,"rva":21145440,"va":5389854560}
// 编译实例 RVAs: 0x142a760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1414735d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t298350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389854560,"raw_hex":"e86e8d0400","size":5,"soft_fail":false,"target_va":5390153168,"va":5389854813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26766537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389854560,"raw_hex":"e8c96c9801","size":5,"soft_fail":false,"target_va":5416621600,"va":5389855058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26767121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389854560,"raw_hex":"e8116f9801","size":5,"soft_fail":false,"target_va":5416622208,"va":5389855082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-152895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389854560,"raw_hex":"e8c1aafdff","size":5,"soft_fail":false,"target_va":5389701728,"va":5389854618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_left_until
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":2568,"parent_record_offset":2228,"ancestor_record_offset":2228,"inlinee":79585,"source_line":286,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1478,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":2228,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":2228,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":2228,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":2888,"parent_record_offset":2568,"ancestor_record_offset":2228,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1478,"record_offset":2972,"parent_record_offset":2568,"ancestor_record_offset":2228,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1478,"record_offset":3080,"parent_record_offset":3052,"ancestor_record_offset":2228,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":3012,"parent_record_offset":2568,"ancestor_record_offset":2228,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":3172,"parent_record_offset":3144,"ancestor_record_offset":2228,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":3144,"parent_record_offset":3116,"ancestor_record_offset":2228,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":3052,"parent_record_offset":2568,"ancestor_record_offset":2228,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1478,"record_offset":3220,"parent_record_offset":3172,"ancestor_record_offset":2228,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1478,"record_offset":2776,"parent_record_offset":2568,"ancestor_record_offset":2228,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":2692,"parent_record_offset":2568,"ancestor_record_offset":2228,"inlinee":79587,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":2804,"parent_record_offset":2568,"ancestor_record_offset":2228,"inlinee":79588,"source_line":55,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2228,"inlinee":26197,"source_line":51,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1478,"record_offset":3116,"parent_record_offset":2568,"ancestor_record_offset":2228,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1478,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2228,"inlinee":26182,"source_line":199,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left_until
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136E1
// 首条位置: {"module":1478,"inlinee":79585}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":1812,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1478,"record_offset":1896,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":1936,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":1976,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1478,"record_offset":1784,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":1616,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":79587,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":1644,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":2804,"parent_record_offset":2568,"ancestor_record_offset":2568,"inlinee":79588,"source_line":55,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":2832,"parent_record_offset":2568,"ancestor_record_offset":2568,"inlinee":26197,"source_line":51,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1478,"record_offset":2040,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1478,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":1488,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1488,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1488,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1478,"record_offset":2004,"parent_record_offset":1976,"ancestor_record_offset":1488,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1478,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1488,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1478,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":1488,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1478,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1488,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1478,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1488,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":2096,"parent_record_offset":2068,"ancestor_record_offset":1488,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1478,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":1488,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1478,"record_offset":2144,"parent_record_offset":2096,"ancestor_record_offset":1488,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2568,"inlinee":26197,"source_line":51,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1478,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2568,"inlinee":26182,"source_line":199,"archive":"_pdb/C/modules/01478.jsonl.gz"}
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
// 首条位置: {"module":1478,"record_offset":3304,"rva":21145968,"va":5389855088}
// 编译实例 RVAs: 0x142a970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141473a80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t299000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389855088,"raw_hex":"e8f88f0400","size":5,"soft_fail":false,"target_va":5390154368,"va":5389855363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26765897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389855088,"raw_hex":"e8496a9801","size":5,"soft_fail":false,"target_va":5416621600,"va":5389855698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26766481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389855088,"raw_hex":"e8916c9801","size":5,"soft_fail":false,"target_va":5416622208,"va":5389855722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-153433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389855088,"raw_hex":"e8a7a8fdff","size":5,"soft_fail":false,"target_va":5389701728,"va":5389855156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1478,"record_offset":3988,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1478,"record_offset":4072,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1478,"record_offset":4112,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1478,"record_offset":4152,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1478,"record_offset":3876,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":3792,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":79587,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":3904,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":79588,"source_line":55,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":3932,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":26197,"source_line":51,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1478,"record_offset":4260,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":10939,"source_line":89,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1478,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":3304,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":3304,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1478,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":3304,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1478,"record_offset":4192,"parent_record_offset":4152,"ancestor_record_offset":3304,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1478,"record_offset":4332,"parent_record_offset":4296,"ancestor_record_offset":3304,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1478,"record_offset":4296,"parent_record_offset":4260,"ancestor_record_offset":3304,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1478,"record_offset":4388,"parent_record_offset":4332,"ancestor_record_offset":3304,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1478,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":3304,"inlinee":26197,"source_line":51,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1478,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":3304,"inlinee":26182,"source_line":199,"archive":"_pdb/C/modules/01478.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_left_until::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13822
// 首条位置: {"module":1504,"inlinee":79906}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":1504,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":17764,"inlinee":79908,"source_line":440,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_left_until::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1381E
// 首条位置: {"module":1504,"inlinee":79902}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1504,"record_offset":10264,"parent_record_offset":10244,"ancestor_record_offset":10244,"inlinee":5608,"source_line":79,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::start
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6656
// 首条位置: {"module":399,"inlinee":26198}
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
// 记录形式: inline_instance, inlinee_declaration; instances=79
// 源码行: 338
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E58
// 首条位置: {"module":249,"inlinee":15960}
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
// IPI IDs: 0x6652
// 首条位置: {"module":399,"inlinee":26194}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":399,"record_offset":4568,"parent_record_offset":4536,"ancestor_record_offset":4536,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::measure_length
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 414
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6651
// 首条位置: {"module":399,"inlinee":26193}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":399,"record_offset":4600,"parent_record_offset":4504,"ancestor_record_offset":4504,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":399,"record_offset":4656,"parent_record_offset":4504,"ancestor_record_offset":4504,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::iter_coords
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":399,"record_offset":4536,"parent_record_offset":4504,"ancestor_record_offset":4504,"inlinee":26194,"source_line":345,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":399,"record_offset":4568,"parent_record_offset":4536,"ancestor_record_offset":4504,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":399,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":4504,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":399,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4504,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::measure_length::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 416
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1381F
// 首条位置: {"module":1504,"inlinee":79903}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1504,"record_offset":15320,"parent_record_offset":15188,"ancestor_record_offset":15188,"inlinee":5608,"source_line":79,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1504,"record_offset":15468,"parent_record_offset":15188,"ancestor_record_offset":15188,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1504,"record_offset":15228,"parent_record_offset":15188,"ancestor_record_offset":15188,"inlinee":26187,"source_line":226,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1504,"record_offset":15272,"parent_record_offset":15228,"ancestor_record_offset":15188,"inlinee":26188,"source_line":3227,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1504,"record_offset":15700,"parent_record_offset":15672,"ancestor_record_offset":15188,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1504,"record_offset":15552,"parent_record_offset":15516,"ancestor_record_offset":15188,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1504,"record_offset":15492,"parent_record_offset":15468,"ancestor_record_offset":15188,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1504,"record_offset":15516,"parent_record_offset":15492,"ancestor_record_offset":15188,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1504,"record_offset":15672,"parent_record_offset":15492,"ancestor_record_offset":15188,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::measure_length::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136E6
// 首条位置: {"module":1478,"inlinee":79590}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1482,"record_offset":32344,"parent_record_offset":32320,"ancestor_record_offset":32320,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1482,"record_offset":32428,"parent_record_offset":32320,"ancestor_record_offset":32320,"inlinee":10939,"source_line":89,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1482,"record_offset":32532,"parent_record_offset":32508,"ancestor_record_offset":32320,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":32372,"parent_record_offset":32344,"ancestor_record_offset":32320,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1482,"record_offset":32476,"parent_record_offset":32452,"ancestor_record_offset":32320,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1482,"record_offset":32452,"parent_record_offset":32428,"ancestor_record_offset":32320,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1482,"record_offset":32508,"parent_record_offset":32476,"ancestor_record_offset":32320,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01482.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 15; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_left", "arguments": ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_left", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_left_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> > >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemantic3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: curve_slice.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: curve_slice.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>", "tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurvePoints::is_empty
// 信息 ID: 1abbfc6050a70b3b60cf7411
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1478, inline_record=1616; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1478, inline_record=2692; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1478, inline_record=3792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::end
// 信息 ID: a7decf63d2304a74e6a12923
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 47 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=13988; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=14884; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=15272; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=38336; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=39232; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=39620; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14156; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=15020; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 信息 ID: fc88518840d395d119edbe36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] slice2$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left -> ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x147b568 -> 0x142a590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 信息 ID: edc2b1d07091c1254006c406
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ff74e -> 0x142a760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left_until
// 信息 ID: da12939f24fa906a82a51172
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1478, inline_record=1488; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1478, inline_record=2568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>
// 信息 ID: 97c2315943705acc3d81ae24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left_until -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x7fb872 -> 0x142a970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right
// 信息 ID: 57ec484ec276fc4d27214da7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 47 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=13960; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=14856; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=15244; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=38308; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=39204; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=39592; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14128; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14992; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until
// 信息 ID: eda767be0fcf62795d15c225
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=14592; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1491, inline_record=3300; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1491, inline_record=4272; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1491, inline_record=4980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> > >
// 信息 ID: cc7937b4199becab5fd83fbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 -> ["system_wall_constructor::construct_gates::place_brick_steps::closure_env$0"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 -> ["system_wall_constructor::construct_gates::place_brick_steps::closure_env$0"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::impl$6::extend_right::closure_env$0 -> ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3> @ 0x1449dbb -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3> @ 0x144a0ed -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3> @ 0x144a169 -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x144d5c1 -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x144d89a -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x144d8fa -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1450e2b -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x145115d -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14511d9 -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> @ 0x145462a -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> @ 0x145491e -> 0x142afd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> @ 0x145497e -> 0x142afd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >
// 信息 ID: c797d0ac78383b8ae2b6de34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9 -> ["false"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 -> ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false>"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9 -> ["false"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 -> ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false>"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::impl$6::extend_right::closure_env$0 -> ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x679ec1 -> 0x559870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67a1c8 -> 0x559870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67a245 -> 0x559870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > >
// 信息 ID: 18c2d3150afa69e1aabc9ae2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9 -> ["true"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 -> ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true>"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9 -> ["true"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 -> ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true>"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::impl$6::extend_right::closure_env$0 -> ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec2::Vec2", "slice2$<glam::f32::vec2::Vec2>", "utils::curve::CurveCoordSemantic2D", "f32", "system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x679ec1 -> 0x559870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67a1c8 -> 0x559870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67a245 -> 0x559870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: 568e0c9ae026532856c1b0b2
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
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel @ 0x7f70b0 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel @ 0x7f71ed -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel @ 0x7f7285 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel @ 0x7f7369 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports @ 0x82629c -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83bd78 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpiece_params<system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel::closure$0::closure_env$0> @ 0x142b953 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpiece_params<system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel::closure$0::closure_env$0> @ 0x142bc68 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams @ 0x144368a -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1460d19 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x1464026 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x147b557 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x87de8f -> 0x14dff60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x87e24b -> 0x14dff60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x8c79fe -> 0x14dff60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x8e48cf -> 0x14dff60 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: 1314064b9ec086035107958e
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
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::public_wall_state::PublicWallState::wooden_stone_foundation_surface_top_y_at @ 0xbf1ebf -> 0xbe9300 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel @ 0x7f70b0 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel @ 0x7f71ed -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel @ 0x7f7285 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel @ 0x7f7369 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports @ 0x82629c -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83bd78 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpiece_params<system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel::closure$0::closure_env$0> @ 0x142b953 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpiece_params<system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel::closure$0::closure_env$0> @ 0x142bc68 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams @ 0x144368a -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1460d19 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x1464026 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x147b557 -> 0x142b2c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x87de8f -> 0x14dff60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x87e24b -> 0x14dff60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x8c79fe -> 0x14dff60 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: 168b0fa5545da1cf7467bfa6
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
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ff721 -> 0x142b100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>
// 信息 ID: dc36e76c08ab8c9901792253
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x7fb883 -> 0x142abf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_coord_range
// 信息 ID: 0f6b088c1841ce362819b711
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=4436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_curve_range
// 信息 ID: be1ada28db6c7a16bd12549f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=13960; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14824; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=15292; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=16156; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=66536; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=18024; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=6896; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=9508; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_point_range
// 信息 ID: 661dd689bdd11b38bc86938f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=13996; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14860; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=15328; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=16192; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6112; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=66572; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=18056; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=6932; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::iter_coords
// 信息 ID: 3785253b653bbe9a912accd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=4536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::measure_length
// 信息 ID: cfdcba2fa4c97b29bab17b3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=4504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::remap
// 信息 ID: 28bfdfbc90262463f97772b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 33 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=13876; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=38224; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14096; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14960; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=15428; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=16292; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=401, inline_record=11268; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=66676; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::remap_2d
// 信息 ID: ab68f49d1b964ab70622921a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14072; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=14936; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=15404; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::wooden_arch_frame::create_wooden_arch_cornerpieces_for_lintel; module=396, inline_record=16268; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=401, inline_record=11244; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports; module=418, inline_record=66652; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=18128; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=449, inline_record=7008; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::start
// 信息 ID: 33c9b5d227d8fe969201dc55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6328; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=401, inline_record=11356; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1511, inline_record=8056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$0::iter
// 信息 ID: 5a2622fd615ad2ade4816491
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > >; module=196, inline_record=53620; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >; module=196, inline_record=54744; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=584, inline_record=30804; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1478, inline_record=1644; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1478, inline_record=11420; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1478, inline_record=14016; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=14680; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1491, inline_record=4360; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$1::iter
// 信息 ID: cef99eb7e29bbf3805b07db9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1478, inline_record=2804; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1478, inline_record=3904; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1478, inline_record=4940; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1478, inline_record=9752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$1::len
// 信息 ID: a13e03ffaa528fb72ac2c745
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6156; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6212; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6268; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1478, inline_record=2716; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1478, inline_record=2832; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1478, inline_record=3816; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1478, inline_record=3932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$3::from_point_range::closure$0
// 信息 ID: 7fd368c40a8f7e42add85bf6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$3::from_point_range", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$3::from_point_range::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$3::from_point_range::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$3::from_point_range::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1478, inline_record=744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$5::remap_2d::closure$0
// 信息 ID: 7ad8ebf23d02abce4abc1b6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$5::remap_2d", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=584, inline_record=28828; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1478, inline_record=1140; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1593, inline_record=628; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=4078, inline_record=624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_left_until::closure$0
// 信息 ID: 2f8a40c85a7efe08809492b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_left_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1504, inline_record=17764; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=5768; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=6872; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=7580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_left_until::closure$1
// 信息 ID: 30a406b4365b991ce5aabca1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_left_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; module=1504, inline_record=10244; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; module=1504, inline_record=12748; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1504, inline_record=18172; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=6148; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=7252; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; module=3148, inline_record=6464; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; module=3148, inline_record=7596; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; module=3148, inline_record=10424; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_right_until::closure$0
// 信息 ID: 30c205ffeb78ae7188723ff0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_right_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; module=3147, inline_record=1452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_right_until::closure$1
// 信息 ID: cc0439eb7f44b002bfd93bc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_right_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 33 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; module=63, inline_record=2704; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; module=63, inline_record=3356; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=584, inline_record=30648; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=584, inline_record=31940; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1478, inline_record=5172; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1478, inline_record=5612; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1478, inline_record=6396; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1478, inline_record=6664; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::measure_length::closure$0
// 信息 ID: c61193c6fb6a8ec91a5e09fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::measure_length", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::iter::traits::iterator::impl$2::next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; module=1504, inline_record=15188; 不是独立调用边。
// [PDB 内联宿主] core::iter::traits::iterator::impl$2::next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >; module=1525, inline_record=9948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::measure_length::closure$1
// 信息 ID: e341669943627732607ac7c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::measure_length", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; module=1482, inline_record=32320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
