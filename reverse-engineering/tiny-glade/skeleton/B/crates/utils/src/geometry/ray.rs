// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/geometry/ray.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e7203270dc4bae751c0ffed81f5fb70c2b15ffc08712d285a385b0ad74ed258c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ray.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1521
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::ray::Ray::from_mouse_ss
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1521,"record_offset":252,"rva":19385008,"va":5388094128}
// 编译实例 RVAs: 0x127cab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::project_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1521,"record_offset":2892,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":28542,"source_line":1128,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1521,"record_offset":632,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":18165,"source_line":1390,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1521,"record_offset":480,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1521,"record_offset":400,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1521,"record_offset":556,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1521,"record_offset":3700,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1521,"record_offset":3636,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1521,"record_offset":944,"parent_record_offset":848,"ancestor_record_offset":252,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1521,"record_offset":3304,"parent_record_offset":3248,"ancestor_record_offset":252,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1521,"record_offset":816,"parent_record_offset":752,"ancestor_record_offset":252,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1521,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":252,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1521,"record_offset":784,"parent_record_offset":752,"ancestor_record_offset":252,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1521,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":252,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1521,"record_offset":428,"parent_record_offset":400,"ancestor_record_offset":252,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1521,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":252,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_mat4
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1521,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":252,"inlinee":18167,"source_line":1238,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1521,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":252,"inlinee":18170,"source_line":1227,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1521,"record_offset":692,"parent_record_offset":656,"ancestor_record_offset":252,"inlinee":18169,"source_line":1405,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1521,"record_offset":2936,"parent_record_offset":2892,"ancestor_record_offset":252,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1521,"record_offset":3136,"parent_record_offset":2892,"ancestor_record_offset":252,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$9::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1521,"record_offset":3248,"parent_record_offset":2892,"ancestor_record_offset":252,"inlinee":28546,"source_line":1043,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1521,"record_offset":3800,"parent_record_offset":3764,"ancestor_record_offset":252,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1521,"record_offset":3764,"parent_record_offset":3728,"ancestor_record_offset":252,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1521,"record_offset":3728,"parent_record_offset":3700,"ancestor_record_offset":252,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::wwww
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1521,"record_offset":1104,"parent_record_offset":716,"ancestor_record_offset":252,"inlinee":18175,"source_line":2071,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1521,"record_offset":752,"parent_record_offset":716,"ancestor_record_offset":252,"inlinee":18172,"source_line":547,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1521,"record_offset":3336,"parent_record_offset":2892,"ancestor_record_offset":252,"inlinee":30322,"source_line":199,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1521,"record_offset":848,"parent_record_offset":716,"ancestor_record_offset":252,"inlinee":18173,"source_line":1051,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1521,"record_offset":976,"parent_record_offset":716,"ancestor_record_offset":252,"inlinee":18174,"source_line":1561,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1521,"record_offset":3836,"parent_record_offset":3800,"ancestor_record_offset":252,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::ray::Ray::xz
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1521,"record_offset":3940,"rva":9279648,"va":5377988768}
// 编译实例 RVAs: 0x8d98a0
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
// 完整结构及原始类型记录: ray.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::ray::Ray
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::geometry::ray::Ray,f32>", "constructor": "tuple$", "arguments": ["utils::geometry::ray::Ray", "f32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$0::call<tuple$<utils::geometry::ray::Ray,f32>,system_critters::wren_boids::sim_wren_boids::closure_env$3>", "constructor": "core::ops::function::impls::impl$0::call", "arguments": ["tuple$<utils::geometry::ray::Ray,f32>", "system_critters::wren_boids::sim_wren_boids::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ray.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::ray::Ray", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::ray::Ray", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ray.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::ray::Ray::from_mouse_ss
// 信息 ID: 6848e67b168d21cbd838d6b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a6a02 -> 0x127cab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac1fe -> 0x127cab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::hover_intersection::hover_intersection @ 0x838679 -> 0x127cab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::mouse_terrain_raycast::camera_mouse_ray @ 0x8ef204 -> 0x127cab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::main_camera::main_camera_update @ 0x912f3d -> 0x127cab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset @ 0x12780c6 -> 0x127cab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::ray::Ray::xz
// 信息 ID: 9ed10b2cf7ed871ebfe6e53b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x2531fe -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x253c42 -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0xad67ce -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xad84b7 -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x124594f -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x1246362 -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x1294ec2 -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x12958d4 -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform> @ 0x1296a7e -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform> @ 0x12974c2 -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform> @ 0x1298652 -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform> @ 0x1299064 -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform> @ 0x129a14a -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform> @ 0x129ab6e -> 0x8d98a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
