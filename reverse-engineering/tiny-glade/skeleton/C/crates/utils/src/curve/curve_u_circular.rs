// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/curve/curve_u_circular.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 994452a39e0115358fe55e99f4bf3a9c207b4b56ff592e54dc328072f5bd6550
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: curve_u_circular.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 395, 451, 458, 557, 584, 763, 764, 1459, 1467, 1472, 1473, 1474, 1482, 1490, 1494, 1513, 1528, 1532, 1533, 1535, 1540, 1594, 1595, 1604, 1605, 1608, 1699, 1701, 2179, 2188, 2746, 2750, 3140, 3153, 4064, 4065, 4077, 4813
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::impl$8::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B4
// 首条位置: {"module":1472,"inlinee":79540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::new
// 记录形式: inline_instance, inlinee_declaration; instances=112
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x660F
// 首条位置: {"module":395,"inlinee":26127}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=61
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":395,"record_offset":21212,"parent_record_offset":21176,"ancestor_record_offset":21176,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=58
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":395,"record_offset":21240,"parent_record_offset":21176,"ancestor_record_offset":21176,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=58
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":395,"record_offset":21264,"parent_record_offset":21240,"ancestor_record_offset":21176,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::add
// 记录形式: inline_instance, inlinee_declaration; instances=46
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69F0
// 首条位置: {"module":451,"inlinee":27120}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=36
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":451,"record_offset":964,"parent_record_offset":928,"ancestor_record_offset":928,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=36
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":451,"record_offset":992,"parent_record_offset":964,"ancestor_record_offset":928,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::subtract
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69EF
// 首条位置: {"module":451,"inlinee":27119}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":451,"record_offset":928,"parent_record_offset":900,"ancestor_record_offset":900,"inlinee":27120,"source_line":41,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":451,"record_offset":992,"parent_record_offset":964,"ancestor_record_offset":900,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":451,"record_offset":964,"parent_record_offset":928,"ancestor_record_offset":900,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B5
// 首条位置: {"module":1472,"inlinee":79541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=45
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1472,"record_offset":14800,"parent_record_offset":14744,"ancestor_record_offset":14744,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=45
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":14944,"parent_record_offset":14744,"ancestor_record_offset":14744,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":14972,"parent_record_offset":14944,"ancestor_record_offset":14744,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":14912,"parent_record_offset":14860,"ancestor_record_offset":14744,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":14744,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":14824,"parent_record_offset":14800,"ancestor_record_offset":14744,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=19
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1472,"record_offset":15060,"parent_record_offset":15032,"ancestor_record_offset":14744,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=19
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":15032,"parent_record_offset":15004,"ancestor_record_offset":14744,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::distance
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136CF
// 首条位置: {"module":1490,"inlinee":79567}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::wrapping_contains
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69F1
// 首条位置: {"module":451,"inlinee":27121}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":451,"record_offset":1212,"parent_record_offset":1156,"ancestor_record_offset":1156,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":451,"record_offset":1276,"parent_record_offset":1240,"ancestor_record_offset":1156,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":451,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":1156,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":451,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":1156,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::wrap_around
// 记录形式: inline_instance, inlinee_declaration; instances=124
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6610
// 首条位置: {"module":395,"inlinee":26128}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=94
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":395,"record_offset":21264,"parent_record_offset":21240,"ancestor_record_offset":21240,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::impl$3::from
// 记录形式: inline_instance, inlinee_declaration; instances=71
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69F8
// 首条位置: {"module":451,"inlinee":27128}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=53
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":451,"record_offset":10280,"parent_record_offset":10252,"ancestor_record_offset":10252,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":451,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":10252,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":451,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":10252,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":451,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":10252,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":451,"record_offset":10312,"parent_record_offset":10280,"ancestor_record_offset":10252,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00451.jsonl.gz"}
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
// 完整结构及原始类型记录: curve_u_circular.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::curve::curve_u_circular::CurveUCircular
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>", "constructor": "utils::curve::curve_u_circular::CurveUCircular", "arguments": ["utils::curve::CurveCoordSemantic2D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "constructor": "core::iter::sources::once::Once", "arguments": ["utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > > >", "constructor": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf", "arguments": ["utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>", "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "constructor": "tuple$", "arguments": ["usize", "utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "constructor": "core::iter::sources::once::Once", "arguments": ["tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >", "constructor": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf", "arguments": ["tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "constructor": "tuple$", "arguments": ["tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$10::next<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >,tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "itertools::tuple_impl::impl$10::next", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >", "tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "constructor": "tuple$", "arguments": ["utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>", "utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "constructor": "tuple$", "arguments": ["tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> >,tuple$<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "itertools::tuple_impl::impl$7::next", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$8> >", "tuple$<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> >,tuple$<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "itertools::tuple_impl::impl$7::next", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$4> >", "tuple$<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh::closure_env$1>,tuple$<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "itertools::tuple_impl::impl$7::next", "arguments": ["core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh::closure_env$1>", "tuple$<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: curve_u_circular.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: curve_u_circular.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::add
// 信息 ID: b8c4d5901f99460733f73022
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 36 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=928; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=1044; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=10048; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=10136; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=15092; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=15180; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::utils_intersect::line_x_shape_overlap; module=584, inline_record=1808; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::utils_intersect::line_x_shape_overlap; module=584, inline_record=1924; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::distance
// 信息 ID: 488d747c5eb8b78302b2b721
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=44552; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=44584; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=44616; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=44648; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=44680; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=44712; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=44744; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=44776; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::new
// 信息 ID: 6845e85aa80e1e09baceaac8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 91 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=395, inline_record=21176; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape; module=458, inline_record=3436; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape; module=458, inline_record=3564; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=557, inline_record=23852; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=557, inline_record=23980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=557, inline_record=24216; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=557, inline_record=24312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=557, inline_record=26604; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::subtract
// 信息 ID: 7c90fa4939791b6f980b0b62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=900; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::utils_intersect::line_x_shape_overlap; module=584, inline_record=1900; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1535, inline_record=12740; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=5468; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=5888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::wrap_around
// 信息 ID: 088968af2b26ecf5d3a2076f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 94 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=395, inline_record=21240; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=964; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=1080; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=10076; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=10164; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=15120; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=15208; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::walk_shape; module=458, inline_record=3500; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::wrapping_contains
// 信息 ID: b9a34928bb50a90226bb755a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=1156; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=1624; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=2012; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=451, inline_record=2400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
// 信息 ID: 43ecc0247eca57c6ea7eb82b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 26 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=14744; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=15248; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=21940; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1473, inline_record=2028; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1473, inline_record=2532; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > > >; module=1482, inline_record=28348; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets; module=1532, inline_record=11900; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets; module=1533, inline_record=12160; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::impl$3::from
// 信息 ID: 41d52848ddcc52bb16e7fdcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 53 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=10252; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=10960; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=13640; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=15296; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=451, inline_record=16004; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::circular_slice; module=764, inline_record=11908; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::circular_slice; module=764, inline_record=14656; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::circular_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1459, inline_record=6016; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::impl$8::eq
// 信息 ID: 28dd2dc93da372e72a75af3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=14240; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1473, inline_record=1564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
