// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/shapes/utils/shape_signifier_lerp.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 02b4c548fb790fa27d042feb7c6d1f5e685aa023f34c7cd95c474b87d350fbe7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shape_signifier_lerp.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 535, 567
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeSignifierLerp::lerp_radius
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x781A
// 首条位置: {"module":567,"inlinee":30746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::get_radius
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/shapes/utils/shape_signifier_lerp.rs
//     evidence={"module":567,"record_offset":83572,"parent_record_offset":83536,"ancestor_record_offset":83536,"inlinee":30688,"source_line":80,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::interpolate
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":535,"record_offset":36596,"rva":10416144,"va":5379125264}
// 编译实例 RVAs: 0x9ef010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shape_signifier_lerp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32722535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379125264,"raw_hex":"e8674ef301","size":5,"soft_fail":false,"target_va":5411847872,"va":5379125332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32416291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379125264,"raw_hex":"e823a2ee01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379125768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":37648,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":37676,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":37616,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":4989,"source_line":1471,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":36996,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":24543,"source_line":320,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":37496,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":30660,"source_line":291,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":37216,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::select
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":37552,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":30691,"source_line":97,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::signum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":37092,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":30689,"source_line":334,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":37272,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":36840,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":36892,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":535,"record_offset":36812,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":37520,"parent_record_offset":37496,"ancestor_record_offset":36596,"inlinee":27520,"source_line":1792,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":37052,"parent_record_offset":37024,"ancestor_record_offset":36596,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":37176,"parent_record_offset":37148,"ancestor_record_offset":36596,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":37240,"parent_record_offset":37216,"ancestor_record_offset":36596,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":37148,"parent_record_offset":37120,"ancestor_record_offset":36596,"inlinee":4989,"source_line":1471,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":36864,"parent_record_offset":36840,"ancestor_record_offset":36596,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":36916,"parent_record_offset":36892,"ancestor_record_offset":36596,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":535,"record_offset":37024,"parent_record_offset":36996,"ancestor_record_offset":36596,"inlinee":13667,"source_line":146,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::signum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":535,"record_offset":37120,"parent_record_offset":37092,"ancestor_record_offset":36596,"inlinee":30690,"source_line":187,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::get_radius
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77E0
// 首条位置: {"module":535,"inlinee":30688}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::get_rectangle_dims
// 记录形式: inlinee_declaration; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77DF
// 首条位置: {"module":535,"inlinee":30687}
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
// 完整结构及原始类型记录: shape_signifier_lerp.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeSignifierLerp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shape_signifier_lerp.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shape_signifier_lerp.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeSignifierLerp::lerp_radius
// 信息 ID: 48e68746117832ddddc44d99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::do_ui_edit_rectangle_rotate; module=567, inline_record=83536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::get_radius
// 信息 ID: 5f72bb65cd53c0ccb94db4ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::do_ui_edit_rectangle_rotate; module=567, inline_record=83572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::get_rectangle_dims
// 信息 ID: add54d6c9c049ce32ecc9bd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::interpolate
// 信息 ID: d5facbf9fe17366dd1d8253d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
