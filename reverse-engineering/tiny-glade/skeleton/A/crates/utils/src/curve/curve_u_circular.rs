// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/curve/curve_u_circular.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 994452a39e0115358fe55e99f4bf3a9c207b4b56ff592e54dc328072f5bd6550
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: curve_u_circular.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 373, 420, 472, 740, 1206, 1219, 1224, 1225, 1229, 1233, 1249, 1262, 1264, 1274, 1288, 1294, 1302, 1327, 1403, 1904, 2272, 2276, 2698, 2746, 2765, 2766, 3513, 3514, 3524, 4262
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::impl$8::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5445
// 首条位置: {"module":373,"inlinee":21573}
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
// 记录形式: inline_instance, inlinee_declaration; instances=85
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5442
// 首条位置: {"module":373,"inlinee":21570}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":373,"record_offset":6704,"parent_record_offset":6668,"ancestor_record_offset":6668,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=36
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":373,"record_offset":6732,"parent_record_offset":6668,"ancestor_record_offset":6668,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=36
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":373,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":6668,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::add
// 记录形式: inline_instance, inlinee_declaration; instances=38
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5ECD
// 首条位置: {"module":420,"inlinee":24269}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=28
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":15856,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=28
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1206,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":15828,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::subtract
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EFA
// 首条位置: {"module":1206,"inlinee":24314}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":15944,"parent_record_offset":15920,"ancestor_record_offset":15920,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1206,"record_offset":15996,"parent_record_offset":15972,"ancestor_record_offset":15920,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":15972,"parent_record_offset":15944,"ancestor_record_offset":15920,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5446
// 首条位置: {"module":373,"inlinee":21574}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=33
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":373,"record_offset":7264,"parent_record_offset":7208,"ancestor_record_offset":7208,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=46
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":373,"record_offset":7408,"parent_record_offset":7208,"ancestor_record_offset":7208,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":373,"record_offset":7436,"parent_record_offset":7408,"ancestor_record_offset":7208,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=52
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":7348,"parent_record_offset":7324,"ancestor_record_offset":7208,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":7376,"parent_record_offset":7324,"ancestor_record_offset":7208,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":7208,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=16
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":373,"record_offset":7524,"parent_record_offset":7496,"ancestor_record_offset":7208,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=16
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":373,"record_offset":7496,"parent_record_offset":7468,"ancestor_record_offset":7208,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::distance
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE82F
// 首条位置: {"module":1206,"inlinee":59439}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB10
// 首条位置: {"module":1274,"inlinee":60176}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1274,"record_offset":22064,"parent_record_offset":21940,"ancestor_record_offset":21940,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":22296,"parent_record_offset":22224,"ancestor_record_offset":21940,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":22340,"parent_record_offset":22296,"ancestor_record_offset":21940,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1274,"record_offset":22116,"parent_record_offset":22064,"ancestor_record_offset":21940,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::CurveUCircular::wrap_around
// 记录形式: inline_instance, inlinee_declaration; instances=87
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5443
// 首条位置: {"module":373,"inlinee":21571}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=64
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":373,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u_circular::impl$3::from
// 记录形式: inline_instance, inlinee_declaration; instances=60
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68C9
// 首条位置: {"module":740,"inlinee":26825}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=45
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":32132,"parent_record_offset":32104,"ancestor_record_offset":32104,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":32220,"parent_record_offset":32196,"ancestor_record_offset":32104,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":32244,"parent_record_offset":32220,"ancestor_record_offset":32104,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":32280,"parent_record_offset":32132,"ancestor_record_offset":32104,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":32308,"parent_record_offset":32132,"ancestor_record_offset":32104,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":32196,"parent_record_offset":32172,"ancestor_record_offset":32104,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":32172,"parent_record_offset":32132,"ancestor_record_offset":32104,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "constructor": "tuple$", "arguments": ["usize", "utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "constructor": "core::iter::sources::once::Once", "arguments": ["tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "constructor": "tuple$", "arguments": ["tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::circular_tuple_windows<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >,tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "itertools::tuple_impl::circular_tuple_windows", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >", "tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "constructor": "tuple$", "arguments": ["utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>", "utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "constructor": "core::iter::sources::once::Once", "arguments": ["tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh::closure_env$1> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh::closure_env$1> > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> > > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >", "ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> > > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyRequestsPrevFrame>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyRequestsPrevFrame>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyRequestsPrevFrame>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::add
// 信息 ID: 32a082adc4df96876e6e2abc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=15828; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=15944; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=17664; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=19644; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=9416; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=9504; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=14100; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=14188; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::distance
// 信息 ID: 6593538de6d34616fb623529
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=15772; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=17636; 不是独立调用边。
// [PDB 内联宿主] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons; module=2276, inline_record=12920; 不是独立调用边。
// [PDB 内联宿主] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons; module=2276, inline_record=14552; 不是独立调用边。
// [PDB 内联宿主] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons; module=2276, inline_record=14760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::new
// 信息 ID: aeb4d619a0f5f17656576e98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 70 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=6668; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=6820; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=7408; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=7468; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=7648; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=7900; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=8100; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=8160; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::subtract
// 信息 ID: 162826ac752e4f2a6a8f29fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=15920; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=1274, inline_record=21680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::utils_intersect::line_x_shape_overlap; module=1403, inline_record=11208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::wrap_around
// 信息 ID: 84a5c92331ad650b8f5c8034
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 64 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=6732; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=6884; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=7496; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=8184; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=26840; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=11320; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=14312; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=15856; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::wrapping_contains
// 信息 ID: 5acb15d6aebb45ea64f66678
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds; module=1274, inline_record=21940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
// 信息 ID: 9a05e2aa21fffacb62a3711c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=7208; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=7712; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=13900; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets; module=1224, inline_record=12032; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets; module=1225, inline_record=12584; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >; module=1264, inline_record=8228; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<f32> >,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> >; module=1294, inline_record=36444; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<f32> >,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure_env$1<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> >; module=1294, inline_record=36976; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::impl$3::from
// 信息 ID: f436febf4648d8dd5932c6b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 45 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::circular_slice; module=740, inline_record=32104; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::circular_slice; module=740, inline_record=34832; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=14412; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars; module=1219, inline_record=2472; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars; module=1219, inline_record=2884; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets; module=1224, inline_record=12636; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets; module=1225, inline_record=13188; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::circular_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1229, inline_record=5436; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u_circular::impl$8::eq
// 信息 ID: 0e9b2cf5f1d9c6f8a5fac6bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=6948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
