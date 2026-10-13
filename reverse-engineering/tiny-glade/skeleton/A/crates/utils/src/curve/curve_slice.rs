// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/curve/curve_slice.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 235b8b330237484ba9c05882c2f4b587504e416f027af79d67ce74d4f6127653
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: curve_slice.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 369, 374, 383, 1229, 1235, 1243, 1245, 1249, 1250, 1252, 1257, 1263, 1264, 1265, 1272, 1283, 1284, 2272, 2275, 2704, 2717, 2742, 2761, 2767, 3509, 3525, 3526, 3537, 3865
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use take_until; // namespace candidate
//   package identities: [{"name":"take-until","source":"PDB_registry_path_only","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurvePoints::is_empty
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9BA
// 首条位置: {"module":1243,"inlinee":59834}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":21609,"source_line":48,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1243,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":2968,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$0::iter
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9BD
// 首条位置: {"module":1243,"inlinee":59837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1243,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":3936,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1243,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3936,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1243,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3936,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1243,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3936,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$1::len
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5469
// 首条位置: {"module":374,"inlinee":21609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":19688,"parent_record_offset":19664,"ancestor_record_offset":19664,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$1::iter
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9BB
// 首条位置: {"module":1243,"inlinee":59835}
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
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54EE
// 首条位置: {"module":383,"inlinee":21742}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23352,"parent_record_offset":23324,"ancestor_record_offset":23324,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":27696,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::from_point_range
// 记录形式: inline_instance, inlinee_declaration; instances=30
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5467
// 首条位置: {"module":374,"inlinee":21607}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":27720,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":19664,"parent_record_offset":19620,"ancestor_record_offset":19620,"inlinee":21609,"source_line":48,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":19688,"parent_record_offset":19664,"ancestor_record_offset":19620,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$3::from_point_range::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9B5
// 首条位置: {"module":1243,"inlinee":59829}
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
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5462
// 首条位置: {"module":374,"inlinee":21602}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":374,"record_offset":17124,"parent_record_offset":17056,"ancestor_record_offset":17056,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$11::sub
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":374,"record_offset":17096,"parent_record_offset":17056,"ancestor_record_offset":17056,"inlinee":21604,"source_line":1134,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::remap
// 记录形式: inline_instance, inlinee_declaration; instances=27
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53EC
// 首条位置: {"module":369,"inlinee":21484}
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
// 记录形式: inline_instance, inlinee_declaration; instances=23
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53EB
// 首条位置: {"module":369,"inlinee":21483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":369,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":14508,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$5::remap_2d::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9B7
// 首条位置: {"module":1243,"inlinee":59831}
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
// 记录形式: inline_instance, inlinee_declaration; instances=33
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53ED
// 首条位置: {"module":369,"inlinee":21485}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":14556,"parent_record_offset":14524,"ancestor_record_offset":14524,"inlinee":59845,"source_line":240,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1249,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":14524,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1249,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":14524,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1249,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":14524,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1249,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14524,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":15256,"parent_record_offset":14556,"ancestor_record_offset":14524,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1249,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":14524,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":15108,"parent_record_offset":14908,"ancestor_record_offset":14524,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1249,"record_offset":15060,"parent_record_offset":15028,"ancestor_record_offset":14524,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":15028,"parent_record_offset":15004,"ancestor_record_offset":14524,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":14828,"parent_record_offset":14800,"ancestor_record_offset":14524,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1249,"record_offset":14656,"parent_record_offset":14556,"ancestor_record_offset":14524,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":14884,"parent_record_offset":14556,"ancestor_record_offset":14524,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":14680,"parent_record_offset":14656,"ancestor_record_offset":14524,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":14524,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":14524,"inlinee":59897,"source_line":326,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1250,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":2836,"inlinee":59842,"source_line":94,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$2
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1252,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":5840,"inlinee":59823,"source_line":354,"archive":"_pdb/A/modules/01252.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1249,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":14524,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":14628,"parent_record_offset":14556,"ancestor_record_offset":14524,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":15156,"parent_record_offset":15132,"ancestor_record_offset":14524,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":14524,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1249,"record_offset":15288,"parent_record_offset":14556,"ancestor_record_offset":14524,"inlinee":59843,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9C5
// 首条位置: {"module":1249,"inlinee":59845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":15256,"parent_record_offset":14556,"ancestor_record_offset":14556,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1249,"record_offset":14656,"parent_record_offset":14556,"ancestor_record_offset":14556,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":14884,"parent_record_offset":14556,"ancestor_record_offset":14556,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":14628,"parent_record_offset":14556,"ancestor_record_offset":14556,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1249,"record_offset":15288,"parent_record_offset":14556,"ancestor_record_offset":14556,"inlinee":59843,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1249,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":14556,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1249,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":14556,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1249,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":14556,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1249,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14556,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1249,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":14556,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":15108,"parent_record_offset":14908,"ancestor_record_offset":14556,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1249,"record_offset":15060,"parent_record_offset":15028,"ancestor_record_offset":14556,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":15028,"parent_record_offset":15004,"ancestor_record_offset":14556,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":14828,"parent_record_offset":14800,"ancestor_record_offset":14556,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":14680,"parent_record_offset":14656,"ancestor_record_offset":14556,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":14556,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":14556,"inlinee":59897,"source_line":326,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1250,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":2868,"inlinee":59842,"source_line":94,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$2
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1252,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":5880,"inlinee":59823,"source_line":354,"archive":"_pdb/A/modules/01252.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1249,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":14556,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":15156,"parent_record_offset":15132,"ancestor_record_offset":14556,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":14556,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1283,"record_offset":640,"rva":13658400,"va":5382367520}
// 编译实例 RVAs: 0xd06920, 0x1ade900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141775f80
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10941718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382367520,"raw_hex":"e816f5a600","size":5,"soft_fail":false,"target_va":5393309568,"va":5382367845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20610892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382367520,"raw_hex":"e84c7f3a01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382368255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382367520,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5382367738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1283,"record_offset":2204,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1283,"record_offset":2120,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1283,"record_offset":2712,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1283,"record_offset":1216,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1283,"record_offset":2260,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1283,"record_offset":1980,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":59837,"source_line":36,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1283,"record_offset":2776,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1283,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":640,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1283,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":640,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1283,"record_offset":1300,"parent_record_offset":1252,"ancestor_record_offset":640,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1283,"record_offset":1576,"parent_record_offset":1396,"ancestor_record_offset":640,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1283,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":640,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1283,"record_offset":1364,"parent_record_offset":1332,"ancestor_record_offset":640,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1283,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":640,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1283,"record_offset":1396,"parent_record_offset":1364,"ancestor_record_offset":640,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$10::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1283,"record_offset":1428,"parent_record_offset":1396,"ancestor_record_offset":640,"inlinee":18909,"source_line":1012,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1283,"record_offset":1456,"parent_record_offset":1428,"ancestor_record_offset":640,"inlinee":59841,"source_line":226,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1283,"record_offset":1916,"parent_record_offset":1252,"ancestor_record_offset":640,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1283,"record_offset":1516,"parent_record_offset":1428,"ancestor_record_offset":640,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1283,"record_offset":1608,"parent_record_offset":1576,"ancestor_record_offset":640,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1283,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":640,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1283,"record_offset":2740,"parent_record_offset":2712,"ancestor_record_offset":640,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1283,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":640,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1283,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":640,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1283,"record_offset":1800,"parent_record_offset":1300,"ancestor_record_offset":640,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1283,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":640,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1283,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":640,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1283,"record_offset":2452,"parent_record_offset":2420,"ancestor_record_offset":640,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1283,"record_offset":1752,"parent_record_offset":1720,"ancestor_record_offset":640,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1283,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":640,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1283,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":640,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1283,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":640,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1283,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":640,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1283,"record_offset":1720,"parent_record_offset":1576,"ancestor_record_offset":640,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1283,"record_offset":1692,"parent_record_offset":1576,"ancestor_record_offset":640,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1283,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":640,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1283,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":640,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1283,"record_offset":1252,"parent_record_offset":1216,"ancestor_record_offset":640,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1283,"record_offset":2880,"parent_record_offset":2832,"ancestor_record_offset":640,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1283,"record_offset":1332,"parent_record_offset":1300,"ancestor_record_offset":640,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1283,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":640,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01283.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1243,"record_offset":9512,"rva":13398848,"va":5382107968}
// 编译实例 RVAs: 0xcc7340, 0xd06920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141775f80
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11201270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382107968,"raw_hex":"e8f6eaaa00","size":5,"soft_fail":false,"target_va":5393309568,"va":5382108293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20870444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382107968,"raw_hex":"e82c753e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382108703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382107968,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5382108186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":11096,"parent_record_offset":9512,"ancestor_record_offset":9512,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":11012,"parent_record_offset":9512,"ancestor_record_offset":9512,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":11604,"parent_record_offset":9512,"ancestor_record_offset":9512,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1243,"record_offset":10108,"parent_record_offset":9512,"ancestor_record_offset":9512,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":11152,"parent_record_offset":9512,"ancestor_record_offset":9512,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":10872,"parent_record_offset":9512,"ancestor_record_offset":9512,"inlinee":59837,"source_line":36,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1243,"record_offset":11668,"parent_record_offset":9512,"ancestor_record_offset":9512,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1243,"record_offset":10832,"parent_record_offset":10808,"ancestor_record_offset":9512,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":11120,"parent_record_offset":11096,"ancestor_record_offset":9512,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":10192,"parent_record_offset":10144,"ancestor_record_offset":9512,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1243,"record_offset":10468,"parent_record_offset":10288,"ancestor_record_offset":9512,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":9512,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":10256,"parent_record_offset":10224,"ancestor_record_offset":9512,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":11060,"parent_record_offset":11036,"ancestor_record_offset":9512,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":10288,"parent_record_offset":10256,"ancestor_record_offset":9512,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$10::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1243,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":9512,"inlinee":18909,"source_line":1012,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1243,"record_offset":10348,"parent_record_offset":10320,"ancestor_record_offset":9512,"inlinee":59841,"source_line":226,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":10808,"parent_record_offset":10144,"ancestor_record_offset":9512,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1243,"record_offset":10408,"parent_record_offset":10320,"ancestor_record_offset":9512,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1243,"record_offset":10500,"parent_record_offset":10468,"ancestor_record_offset":9512,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":9512,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1243,"record_offset":11632,"parent_record_offset":11604,"ancestor_record_offset":9512,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":9512,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":9512,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":10692,"parent_record_offset":10192,"ancestor_record_offset":9512,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1243,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":9512,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1243,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":9512,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":11344,"parent_record_offset":11312,"ancestor_record_offset":9512,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":10644,"parent_record_offset":10612,"ancestor_record_offset":9512,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":9512,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1243,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":9512,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1243,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":9512,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1243,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":9512,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1243,"record_offset":10612,"parent_record_offset":10468,"ancestor_record_offset":9512,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1243,"record_offset":10584,"parent_record_offset":10468,"ancestor_record_offset":9512,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":11724,"parent_record_offset":11696,"ancestor_record_offset":9512,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":11696,"parent_record_offset":11668,"ancestor_record_offset":9512,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":10144,"parent_record_offset":10108,"ancestor_record_offset":9512,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1243,"record_offset":11772,"parent_record_offset":11724,"ancestor_record_offset":9512,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1243,"record_offset":10224,"parent_record_offset":10192,"ancestor_record_offset":9512,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":9512,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1243,"record_offset":7532,"rva":13398048,"va":5382107168}
// 编译实例 RVAs: 0xcc7020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417761b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11202640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382107168,"raw_hex":"e850f0aa00","size":5,"soft_fail":false,"target_va":5393310128,"va":5382107483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177e0c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11235307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382107168,"raw_hex":"e8eb6fab00","size":5,"soft_fail":false,"target_va":5393342656,"va":5382107344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20871186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382107168,"raw_hex":"e812783e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382107961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*(%r8)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382107168,"raw_hex":"41ff10","size":3,"soft_fail":false,"va":5382107372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":8172,"parent_record_offset":7532,"ancestor_record_offset":7532,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":8264,"parent_record_offset":7532,"ancestor_record_offset":7532,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":8956,"parent_record_offset":7532,"ancestor_record_offset":7532,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1243,"record_offset":8348,"parent_record_offset":7532,"ancestor_record_offset":7532,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":8616,"parent_record_offset":7532,"ancestor_record_offset":7532,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":8236,"parent_record_offset":7532,"ancestor_record_offset":7532,"inlinee":59835,"source_line":52,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1243,"record_offset":9020,"parent_record_offset":7532,"ancestor_record_offset":7532,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1243,"record_offset":8576,"parent_record_offset":8552,"ancestor_record_offset":7532,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":7532,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":8408,"parent_record_offset":8372,"ancestor_record_offset":7532,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":7532,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":7532,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":8552,"parent_record_offset":8372,"ancestor_record_offset":7532,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1243,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":7532,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1243,"record_offset":8984,"parent_record_offset":8956,"ancestor_record_offset":7532,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":7532,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":7532,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":7532,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":8848,"parent_record_offset":8816,"ancestor_record_offset":7532,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":8880,"parent_record_offset":8816,"ancestor_record_offset":7532,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":9076,"parent_record_offset":9048,"ancestor_record_offset":7532,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":9048,"parent_record_offset":9020,"ancestor_record_offset":7532,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$26::left_shift_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":8792,"parent_record_offset":8616,"ancestor_record_offset":7532,"inlinee":8082,"source_line":318,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":7532,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1243,"record_offset":9124,"parent_record_offset":9076,"ancestor_record_offset":7532,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":8484,"parent_record_offset":8460,"ancestor_record_offset":7532,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1243,"record_offset":12200,"rva":13399600,"va":5382108720}
// 编译实例 RVAs: 0xcc7630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cedc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t156870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382108720,"raw_hex":"e8c6640200","size":5,"soft_fail":false,"target_va":5382265920,"va":5382109045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177dfb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11233487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382108720,"raw_hex":"e8cf68ab00","size":5,"soft_fail":false,"target_va":5393342384,"va":5382108892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20869246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382108720,"raw_hex":"e87e703e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382109901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*(%r9)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382108720,"raw_hex":"41ff11","size":3,"soft_fail":false,"va":5382108928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":12680,"parent_record_offset":12200,"ancestor_record_offset":12200,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":12772,"parent_record_offset":12200,"ancestor_record_offset":12200,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1243,"record_offset":13464,"parent_record_offset":12200,"ancestor_record_offset":12200,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1243,"record_offset":12856,"parent_record_offset":12200,"ancestor_record_offset":12200,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":13124,"parent_record_offset":12200,"ancestor_record_offset":12200,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":12744,"parent_record_offset":12200,"ancestor_record_offset":12200,"inlinee":59835,"source_line":52,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1243,"record_offset":13584,"parent_record_offset":12200,"ancestor_record_offset":12200,"inlinee":22273,"source_line":86,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1243,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":12200,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":12708,"parent_record_offset":12680,"ancestor_record_offset":12200,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":12916,"parent_record_offset":12880,"ancestor_record_offset":12200,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":12200,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":12820,"parent_record_offset":12796,"ancestor_record_offset":12200,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":13060,"parent_record_offset":12880,"ancestor_record_offset":12200,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1243,"record_offset":13336,"parent_record_offset":13304,"ancestor_record_offset":12200,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1243,"record_offset":13508,"parent_record_offset":13464,"ancestor_record_offset":12200,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":12200,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":12968,"parent_record_offset":12940,"ancestor_record_offset":12200,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":12200,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":13368,"parent_record_offset":13336,"ancestor_record_offset":12200,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":13396,"parent_record_offset":13336,"ancestor_record_offset":12200,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1243,"record_offset":13656,"parent_record_offset":13620,"ancestor_record_offset":12200,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1243,"record_offset":13620,"parent_record_offset":13584,"ancestor_record_offset":12200,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$26::left_shift_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":13304,"parent_record_offset":13124,"ancestor_record_offset":12200,"inlinee":8082,"source_line":318,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":12880,"parent_record_offset":12856,"ancestor_record_offset":12200,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1243,"record_offset":13712,"parent_record_offset":13656,"ancestor_record_offset":12200,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":12200,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_right_until::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 251
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E23
// 首条位置: {"module":2717,"inlinee":89635}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":2717,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":1712,"inlinee":89636,"source_line":441,"archive":"_pdb/A/modules/02717.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_right_until::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=73
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9AE
// 首条位置: {"module":1243,"inlinee":59822}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":8484,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":15156,"inlinee":59897,"source_line":326,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1250,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":3032,"inlinee":59842,"source_line":94,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$2
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1252,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":6052,"inlinee":59823,"source_line":354,"archive":"_pdb/A/modules/01252.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":15156,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1243,"record_offset":3444,"rva":13396256,"va":5382105376}
// 编译实例 RVAs: 0xcc6920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cec3a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t154000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382105376,"raw_hex":"e890590200","size":5,"soft_fail":false,"target_va":5382259616,"va":5382105611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20872442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382105376,"raw_hex":"e8fa7c3e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382106433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20872726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382105376,"raw_hex":"e8167e3e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382106421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382105376,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5382105833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_left_until
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":3764,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":59832,"source_line":283,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1243,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":3444,"inlinee":14887,"source_line":1555,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1243,"record_offset":5888,"parent_record_offset":5780,"ancestor_record_offset":3444,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":3444,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":4304,"parent_record_offset":4272,"ancestor_record_offset":3444,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1243,"record_offset":4408,"parent_record_offset":4380,"ancestor_record_offset":3444,"inlinee":7276,"source_line":33,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::nth
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1243,"record_offset":4796,"parent_record_offset":4680,"ancestor_record_offset":3444,"inlinee":23791,"source_line":47,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1243,"record_offset":4680,"parent_record_offset":4352,"ancestor_record_offset":3444,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":3444,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":3444,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":3444,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":3444,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1243,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":3444,"inlinee":25551,"source_line":1198,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$16::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1243,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":3444,"inlinee":25550,"source_line":1345,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1243,"record_offset":4548,"parent_record_offset":4456,"ancestor_record_offset":3444,"inlinee":14451,"source_line":213,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":4188,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":4104,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1243,"record_offset":4608,"parent_record_offset":4456,"ancestor_record_offset":3444,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1243,"record_offset":4712,"parent_record_offset":4680,"ancestor_record_offset":3444,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":5408,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":5444,"parent_record_offset":5408,"ancestor_record_offset":3444,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":3444,"inlinee":14452,"source_line":664,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1243,"record_offset":6380,"parent_record_offset":6352,"ancestor_record_offset":3444,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":3444,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":3444,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1243,"record_offset":4492,"parent_record_offset":4456,"ancestor_record_offset":3444,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":5128,"parent_record_offset":4304,"ancestor_record_offset":3444,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":5488,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1243,"record_offset":4892,"parent_record_offset":4864,"ancestor_record_offset":3444,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1243,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3444,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":3444,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":3444,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":4864,"parent_record_offset":4820,"ancestor_record_offset":3444,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":4996,"parent_record_offset":4960,"ancestor_record_offset":3444,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":5300,"parent_record_offset":5268,"ancestor_record_offset":3444,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":3444,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1243,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3444,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1243,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3444,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1243,"record_offset":5024,"parent_record_offset":4960,"ancestor_record_offset":3444,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1243,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":3444,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1243,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":3444,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1243,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3444,"inlinee":23756,"source_line":407,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":6472,"parent_record_offset":6444,"ancestor_record_offset":3444,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":3444,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":6352,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1243,"record_offset":4244,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$26::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":5244,"parent_record_offset":4272,"ancestor_record_offset":3444,"inlinee":8082,"source_line":318,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":4272,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1243,"record_offset":6520,"parent_record_offset":6472,"ancestor_record_offset":3444,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1243,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":3444,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1243,"record_offset":4076,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":3908,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":59834,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":3936,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":59837,"source_line":36,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_left_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":3444,"inlinee":59825,"source_line":305,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1243,"record_offset":6416,"parent_record_offset":3764,"ancestor_record_offset":3444,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1243,"record_offset":2528,"rva":13395632,"va":5382104752}
// 编译实例 RVAs: 0xcc66b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cec080
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t153775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382104752,"raw_hex":"e8af580200","size":5,"soft_fail":false,"target_va":5382258816,"va":5382105036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cec780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t155580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382104752,"raw_hex":"e8bc5f0200","size":5,"soft_fail":false,"target_va":5382260608,"va":5382105023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20873544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382104752,"raw_hex":"e848813e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382105331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20873792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382104752,"raw_hex":"e840823e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382105355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-76511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382104752,"raw_hex":"e821d5feff","size":5,"soft_fail":false,"target_va":5382028304,"va":5382104810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_left_until
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":2868,"parent_record_offset":2528,"ancestor_record_offset":2528,"inlinee":59832,"source_line":283,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2528,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":2528,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":2528,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":3248,"parent_record_offset":2868,"ancestor_record_offset":2528,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":3164,"parent_record_offset":2868,"ancestor_record_offset":2528,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":3332,"parent_record_offset":2868,"ancestor_record_offset":2528,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":3360,"parent_record_offset":3332,"ancestor_record_offset":2528,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":3396,"parent_record_offset":2868,"ancestor_record_offset":2528,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1243,"record_offset":3304,"parent_record_offset":2868,"ancestor_record_offset":2528,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1243,"record_offset":3052,"parent_record_offset":2868,"ancestor_record_offset":2528,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":2968,"parent_record_offset":2868,"ancestor_record_offset":2528,"inlinee":59834,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":3080,"parent_record_offset":2868,"ancestor_record_offset":2528,"inlinee":59835,"source_line":52,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2528,"inlinee":21609,"source_line":48,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1243,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":2528,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left_until
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9B8
// 首条位置: {"module":1243,"inlinee":59832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":3248,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":3164,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":3332,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":3396,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":6352,"parent_record_offset":3764,"ancestor_record_offset":3764,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1243,"record_offset":3304,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":4272,"parent_record_offset":3764,"ancestor_record_offset":3764,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1243,"record_offset":3052,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":2968,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":59834,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":3936,"parent_record_offset":3764,"ancestor_record_offset":3764,"inlinee":59837,"source_line":36,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":3080,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":59835,"source_line":52,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":3108,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":21609,"source_line":48,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1243,"record_offset":6416,"parent_record_offset":3764,"ancestor_record_offset":3764,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1243,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":3764,"inlinee":14887,"source_line":1555,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1243,"record_offset":5888,"parent_record_offset":5780,"ancestor_record_offset":3764,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2868,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":4304,"parent_record_offset":4272,"ancestor_record_offset":3764,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1243,"record_offset":4408,"parent_record_offset":4380,"ancestor_record_offset":3764,"inlinee":7276,"source_line":33,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::nth
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1243,"record_offset":4796,"parent_record_offset":4680,"ancestor_record_offset":3764,"inlinee":23791,"source_line":47,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1243,"record_offset":4680,"parent_record_offset":4352,"ancestor_record_offset":3764,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":2868,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":3764,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":2868,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":3764,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1243,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":3764,"inlinee":25551,"source_line":1198,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$16::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1243,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":3764,"inlinee":25550,"source_line":1345,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1243,"record_offset":4548,"parent_record_offset":4456,"ancestor_record_offset":3764,"inlinee":14451,"source_line":213,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1243,"record_offset":4608,"parent_record_offset":4456,"ancestor_record_offset":3764,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1243,"record_offset":4712,"parent_record_offset":4680,"ancestor_record_offset":3764,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":3360,"parent_record_offset":3332,"ancestor_record_offset":2868,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":3764,"inlinee":14452,"source_line":664,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1243,"record_offset":6380,"parent_record_offset":6352,"ancestor_record_offset":3764,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::Fn::call
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":3764,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":3764,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1243,"record_offset":4492,"parent_record_offset":4456,"ancestor_record_offset":3764,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":5128,"parent_record_offset":4304,"ancestor_record_offset":3764,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1243,"record_offset":4892,"parent_record_offset":4864,"ancestor_record_offset":3764,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1243,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3764,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":3764,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":3764,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":4864,"parent_record_offset":4820,"ancestor_record_offset":3764,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1243,"record_offset":4996,"parent_record_offset":4960,"ancestor_record_offset":3764,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":5300,"parent_record_offset":5268,"ancestor_record_offset":3764,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1243,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":3764,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1243,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3764,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1243,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3764,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1243,"record_offset":5024,"parent_record_offset":4960,"ancestor_record_offset":3764,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1243,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":3764,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1243,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":3764,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1243,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3764,"inlinee":23756,"source_line":407,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":6472,"parent_record_offset":6444,"ancestor_record_offset":3764,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1243,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":3764,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$26::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1243,"record_offset":5244,"parent_record_offset":4272,"ancestor_record_offset":3764,"inlinee":8082,"source_line":318,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1243,"record_offset":6520,"parent_record_offset":6472,"ancestor_record_offset":3764,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1243,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":3764,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2868,"inlinee":21609,"source_line":48,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_left_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":3764,"inlinee":59825,"source_line":305,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1243,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":2868,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1243,"record_offset":6572,"rva":13397328,"va":5382106448}
// 编译实例 RVAs: 0xcc6d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cebe80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t151554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382106448,"raw_hex":"e802500200","size":5,"soft_fail":false,"target_va":5382258304,"va":5382106745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cec530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: curve_slice.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t153279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382106448,"raw_hex":"e8bf560200","size":5,"soft_fail":false,"target_va":5382260016,"va":5382106732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20871752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382106448,"raw_hex":"e8487a3e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382107123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20872000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382106448,"raw_hex":"e8407b3e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382107147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-78223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382106448,"raw_hex":"e871cefeff","size":5,"soft_fail":false,"target_va":5382028304,"va":5382106522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":7340,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1243,"record_offset":7256,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":7424,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1243,"record_offset":7488,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1243,"record_offset":7396,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1243,"record_offset":7144,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurvePoints::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":7060,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":59834,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":7172,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":59835,"source_line":52,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":7200,"parent_record_offset":6572,"ancestor_record_offset":6572,"inlinee":21609,"source_line":48,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1243,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":6572,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":6572,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1243,"record_offset":7304,"parent_record_offset":7280,"ancestor_record_offset":6572,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1243,"record_offset":7452,"parent_record_offset":7424,"ancestor_record_offset":6572,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1243,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":6572,"inlinee":21609,"source_line":48,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1243,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":6572,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_left_until::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEAAD
// 首条位置: {"module":1263,"inlinee":60077}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":1263,"record_offset":2036,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":60079,"source_line":439,"archive":"_pdb/A/modules/01263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::extend_left_until::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9B1
// 首条位置: {"module":1243,"inlinee":59825}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":5176,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::start
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x546A
// 首条位置: {"module":374,"inlinee":21610}
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
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 335
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x546B
// 首条位置: {"module":374,"inlinee":21611}
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
// 源码行: 342
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5466
// 首条位置: {"module":374,"inlinee":21606}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::iter_coords::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 353
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEAB6
// 首条位置: {"module":1264,"inlinee":60086}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::CurveSlice::measure_length
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 411
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5465
// 首条位置: {"module":374,"inlinee":21605}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":17220,"parent_record_offset":17156,"ancestor_record_offset":17156,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":17304,"parent_record_offset":17156,"ancestor_record_offset":17156,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":374,"record_offset":17276,"parent_record_offset":17156,"ancestor_record_offset":17156,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::iter_coords
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":17184,"parent_record_offset":17156,"ancestor_record_offset":17156,"inlinee":21606,"source_line":342,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":374,"record_offset":17244,"parent_record_offset":17220,"ancestor_record_offset":17156,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":374,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":17156,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::measure_length::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 413
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9B3
// 首条位置: {"module":1243,"inlinee":59827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::Fn::call
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1243,"record_offset":1024,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":6775,"source_line":79,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1243,"record_offset":1168,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1243,"record_offset":932,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":21487,"source_line":225,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":1243,"record_offset":976,"parent_record_offset":932,"ancestor_record_offset":892,"inlinee":21488,"source_line":2775,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1243,"record_offset":1296,"parent_record_offset":1216,"ancestor_record_offset":892,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1243,"record_offset":1324,"parent_record_offset":1216,"ancestor_record_offset":892,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1243,"record_offset":1268,"parent_record_offset":1216,"ancestor_record_offset":892,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1243,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":892,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1243,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":892,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01243.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_slice::impl$6::measure_length::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 418
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9C0
// 首条位置: {"module":1243,"inlinee":59840}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1264,"record_offset":4408,"parent_record_offset":4384,"ancestor_record_offset":4384,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1264,"record_offset":4504,"parent_record_offset":4384,"ancestor_record_offset":4384,"inlinee":22273,"source_line":86,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":4384,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1264,"record_offset":4560,"parent_record_offset":4532,"ancestor_record_offset":4384,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1264,"record_offset":4532,"parent_record_offset":4504,"ancestor_record_offset":4384,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1264,"record_offset":4596,"parent_record_offset":4560,"ancestor_record_offset":4384,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01264.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurvePoints::is_empty
// 信息 ID: 6c896a591f03bb46f493b93b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=2968; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=3908; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1243, inline_record=7060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::end
// 信息 ID: fccf5f14223889d69d7966e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=19864; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23468; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23776; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=54628; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=11168; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::get_coord_at_2d_ws_distance<utils::curve::CurveCoordSemanticThresholded3D>; module=1272, inline_record=9164; 不是独立调用边。
// [PDB 内联宿主] system_clutter::clutter::add_clutter_around_windows::add_clutter_around_windows; module=3509, inline_record=9452; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=3865, inline_record=8544; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 信息 ID: 5ddf9cefe0f7332fbaa01334
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] slice2$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left -> ["glam::f32::vec3::Vec3", "slice2$<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbd380 -> 0xcc6920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>
// 信息 ID: 76ed5be745d4948d9e56b151
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x4e74b2 -> 0xcc66b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left_until
// 信息 ID: ca4366f639c9d051591beedd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=2868; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=3764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>
// 信息 ID: ad46b764563eeda2ae7f7816
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_left_until -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x4f607d -> 0xcc6d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right
// 信息 ID: 15bc906a256e084d075b0ffb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=369, inline_record=14564; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23440; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23748; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=24052; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=3460; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=14524; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=54596; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=2836; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until
// 信息 ID: b99272c8504f82bd1ee04d09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=14556; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=2868; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=4124; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=5484; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=1252, inline_record=5880; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=1252, inline_record=8788; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=1252, inline_record=10156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: 53849e31a8b48344d732b1a9
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
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500c60 -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500cfd -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500d94 -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::Curve::get_coord_at_2d_ws_distance<utils::curve::CurveCoordSemanticThresholded3D> @ 0xcf378a -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x1c66325 -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x1c666e3 -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::add_clutter_around_windows::add_clutter_around_windows @ 0x1acf4c2 -> 0x1ade900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: 9d22ad69959cf034ebb0b62f
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
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbd36a -> 0xcc7340 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3a8a -> 0xcc7340 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0xce3862 -> 0xcc7340 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500c60 -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500cfd -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500d94 -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::Curve::get_coord_at_2d_ws_distance<utils::curve::CurveCoordSemanticThresholded3D> @ 0xcf378a -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x1c66325 -> 0xd06920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x1c666e3 -> 0xd06920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >
// 信息 ID: c870753b5347b313ecc41676
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
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x4e7490 -> 0xcc7020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>
// 信息 ID: bfab4eae87691fd5385308c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 参数类型拼写: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [类型关系] utils::loop_range::LoopSlice -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] utils::curve::curve_slice::CurveSlice::extend_right_until -> ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x4f608e -> 0xcc7630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_coord_range
// 信息 ID: c441944a3930727724ddabd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=374, inline_record=17056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_curve_range
// 信息 ID: 0d8e9a15346a18636a490196
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23324; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23632; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23936; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=27696; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=14180; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=54384; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=2688; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=10952; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::from_point_range
// 信息 ID: 3341391ecc47d2431c2e221b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=19620; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23352; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23660; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23964; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=27720; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=14212; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=54416; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=2716; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::iter_coords
// 信息 ID: dd9e831bed5f664eda6dd1ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=374, inline_record=17184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::measure_length
// 信息 ID: 8ab75a3cf488689b17285cc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=374, inline_record=17156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::remap
// 信息 ID: 558ced489fc925739f5525ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=369, inline_record=14532; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23408; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23716; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=24020; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=3428; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=54524; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=11076; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::get_coord_at_2d_ws_distance<utils::curve::CurveCoordSemanticThresholded3D>; module=1272, inline_record=9104; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::remap_2d
// 信息 ID: 8b77135be6400f4072af66b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=369, inline_record=14508; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23384; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23692; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23996; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=3404; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=54492; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=11048; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::get_coord_at_2d_ws_distance<utils::curve::CurveCoordSemanticThresholded3D>; module=1272, inline_record=9080; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::CurveSlice::start
// 信息 ID: 962106df7c7a6bcc93ad4cb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=19836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$0::iter
// 信息 ID: 03a7a5b2f3a544ce9f02ab99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=3936; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=10872; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1283, inline_record=1980; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1283, inline_record=4668; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=3526, inline_record=1976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$1::iter
// 信息 ID: b355da30412056a66f593947
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=3080; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1243, inline_record=7172; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=8236; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1243, inline_record=12744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$1::len
// 信息 ID: bc0f5ee252d6441479c12235
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=19664; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=19720; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=19776; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=2992; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=3108; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1243, inline_record=7084; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; module=1243, inline_record=7200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$3::from_point_range::closure$0
// 信息 ID: 90f293c749c21162981e700f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$3::from_point_range", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$3::from_point_range::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$3::from_point_range::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$3::from_point_range::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1243, inline_record=1812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$5::remap_2d::closure$0
// 信息 ID: 4ae94d1b611e158007fb6224
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$5::remap_2d", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$5::remap_2d::closure_env$0 <- core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1243, inline_record=2208; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=1283, inline_record=612; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<utils::curve::curve_slice::impl$5::remap_2d::closure_env$0<slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,tuple$<usize,ref$<glam::f32::vec3::Vec3> > >; module=3526, inline_record=608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_left_until::closure$0
// 信息 ID: 9378ae73f15e33b0a1fef101
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_left_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1263, inline_record=2008; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1263, inline_record=12240; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1265, inline_record=6076; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1265, inline_record=7976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_left_until::closure$1
// 信息 ID: 0042939f0daeff5adb6e3bf1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_left_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=5176; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>; module=1243, inline_record=6272; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1263, inline_record=2664; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; module=1263, inline_record=5448; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; module=1263, inline_record=10364; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1263, inline_record=12896; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; module=1263, inline_record=15600; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1265, inline_record=6732; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_right_until::closure$0
// 信息 ID: 7b595d674605b46017a94f60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_right_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; module=2717, inline_record=1712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::extend_right_until::closure$1
// 信息 ID: 0dde57a385b2425ab3f31a3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::extend_right_until", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 42 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=8484; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=8724; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=9276; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=10740; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=11532; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,slice2$<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=12124; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1243, inline_record=12992; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1243, inline_record=13236; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::iter_coords::closure$0
// 信息 ID: dce8dea27aa2f54707ec13ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::iter_coords", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::iter_coords::closure_env$0 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::iter_coords::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::iter_coords::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::iter_coords::closure_env$0 <- core::iter::adapters::chain::impl$1::next<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; module=1264, inline_record=3844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::measure_length::closure$0
// 信息 ID: d298084adf855ec382afe727
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::curve::curve_slice::impl$6::measure_length", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$0 <- core::iter::adapters::chain::impl$1::next<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >; module=1243, inline_record=892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_slice::impl$6::measure_length::closure$1
// 信息 ID: d7f06775f68fb307db4bacd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::curve::curve_slice::impl$6::measure_length", "captures": "unknown"}
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::curve::curve_slice::impl$6::measure_length::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >; module=1264, inline_record=4384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
