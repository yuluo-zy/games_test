// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/geometry/intersect.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d1401be11038f39ba16cb1913a7f2ce158a39e412e779b7e397147153895ac51
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: intersect.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1266, 1378, 3829
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::impl$1::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFB72
// 首条位置: {"module":1378,"inlinee":64370}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::u32::uvec3::impl$76::hash
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec3.rs
//     evidence={"module":1378,"record_offset":3364,"parent_record_offset":3324,"ancestor_record_offset":3324,"inlinee":22469,"source_line":18,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1378,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":3324,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1378,"record_offset":3396,"parent_record_offset":3364,"ancestor_record_offset":3324,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1378,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":3324,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::points_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1266,"record_offset":244,"rva":4440720,"va":5373149840}
// 编译实例 RVAs: 0x43c290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1266,"record_offset":368,"rva":13557808,"va":5382266928}
// 编译实例 RVAs: 0xcee030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEAC9
// 首条位置: {"module":1266,"record_offset":488,"rva":13557856,"va":5382266976}
// 编译实例 RVAs: 0xcee060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":812,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":844,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":624,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1266,"record_offset":1000,"parent_record_offset":868,"ancestor_record_offset":488,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1266,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":488,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":932,"parent_record_offset":900,"ancestor_record_offset":488,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":900,"parent_record_offset":868,"ancestor_record_offset":488,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":488,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1266,"record_offset":964,"parent_record_offset":932,"ancestor_record_offset":488,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::aabb
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1266,"record_offset":1068,"rva":13558032,"va":5382267152}
// 编译实例 RVAs: 0xcee110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":1392,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":60096,"source_line":235,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":1200,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":60095,"source_line":222,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1266,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":1068,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1266,"record_offset":1236,"parent_record_offset":1200,"ancestor_record_offset":1068,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::intersect_triangle
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1266,"record_offset":1580,"rva":13558272,"va":5382267392}
// 编译实例 RVAs: 0xcee200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11108659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382267392,"raw_hex":"e83381a900","size":5,"soft_fail":false,"target_va":5393376128,"va":5382267464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::ray_triangle_intersection
// 记录形式: compiled_procedure; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1266,"record_offset":1712,"rva":13558368,"va":5382267488}
// 编译实例 RVAs: 0xcee260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1266,"record_offset":2212,"parent_record_offset":1712,"ancestor_record_offset":1712,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":2036,"parent_record_offset":1712,"ancestor_record_offset":1712,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":2068,"parent_record_offset":1712,"ancestor_record_offset":1712,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":1864,"parent_record_offset":1712,"ancestor_record_offset":1712,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1266,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":1712,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1266,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":1712,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1266,"record_offset":1892,"parent_record_offset":1864,"ancestor_record_offset":1712,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1266,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1712,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::SphereIntersection::closest_intersection_distance
// 记录形式: compiled_procedure; instances=1
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1266,"record_offset":2420,"rva":13558992,"va":5382268112}
// 编译实例 RVAs: 0xcee4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1266,"record_offset":2584,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::ray_sphere_intersection
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEAC2
// 首条位置: {"module":1266,"inlinee":60098}
// 编译实例 RVAs: 0xcee500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1266,"record_offset":2752,"parent_record_offset":2612,"ancestor_record_offset":2612,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01266.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1266,"record_offset":2784,"parent_record_offset":2612,"ancestor_record_offset":2612,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01266.jsonl.gz"}
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
// 完整结构及原始类型记录: intersect.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::intersect::Intersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::intersect::Sphere
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["utils::geometry::intersect::Sphere", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>", "alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::Itertools::multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>,tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > >", "constructor": "itertools::Itertools::multiunzip", "arguments": ["core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>", "tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::intersect::SphereIntersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::intersect::Triangle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 36
// [PDB 泛型关系，非字段关系] {"type": "slice2$<utils::geometry::intersect::Triangle>", "constructor": "slice2$", "arguments": ["utils::geometry::intersect::Triangle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<slice2$<utils::geometry::intersect::Triangle> >", "constructor": "ref$", "arguments": ["slice2$<utils::geometry::intersect::Triangle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > >", "constructor": "tuple$", "arguments": ["ref$<slice2$<utils::geometry::intersect::Triangle> >", "ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::calculate_hash<tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > > >", "constructor": "utils::calculate_hash", "arguments": ["tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >", "alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "constructor": "core::slice::iter::Iter", "arguments": ["utils::geometry::intersect::Triangle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::intersect::Triangle", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>", "alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>", "array$<glam::f32::vec3::Vec3,3>", "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>", "array$<glam::f32::vec3::Vec3,3>", "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$5::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "system_wall_constructor::construct_walls::construct_walls::closure$5::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$5::closure_env$0>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$5::closure_env$0>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >", "alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$2>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "array$<glam::f32::vec3::Vec3,3>", "country_core::systems::collision::world::impl$16::new_unindexed::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$2> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::intersect::Triangle", "core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env$2>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$1>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>", "country_core::systems::collision::world::impl$16::new_unindexed::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$1>,nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$1>", "nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> >,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$1>,nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> >", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$1>,nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::intersect::Triangle", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::intersect::Triangle", "core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>", "array$<glam::f32::vec3::Vec3,3>", "country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>", "country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["array$<f32,3>", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["utils::geometry::intersect::Triangle", "core::slice::iter::ChunksExact<u32>", "country_core::render::mesh::impl$0::as_triangles::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["array$<f32,3>", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["array$<f32,3>", "alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["array$<f32,3>", "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>", "country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["utils::geometry::intersect::Triangle", "core::slice::iter::Chunks<usize>", "country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::Hash::hash_slice<utils::geometry::intersect::Triangle,std::hash::random::DefaultHasher>", "constructor": "core::hash::Hash::hash_slice", "arguments": ["utils::geometry::intersect::Triangle", "std::hash::random::DefaultHasher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$55::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$2> >", "constructor": "alloc::boxed::impl$55::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$16::new_unindexed::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$0>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$0>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "alloc::alloc::Global", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > >", "constructor": "core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0", "arguments": ["core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$33::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > >", "constructor": "core::iter::adapters::flatten::impl$33::iter_fold::flatten::closure_env$0", "arguments": ["array$<glam::f32::vec3::Vec3,3>", "tuple$<>", "core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<array$<glam::f32::vec3::Vec3,3>,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3,tuple$<>,core::iter::adapters::flatten::impl$33::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["array$<glam::f32::vec3::Vec3,3>", "core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3", "tuple$<>", "core::iter::adapters::flatten::impl$33::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["utils::geometry::intersect::Triangle", "core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$0::triangles::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: intersect.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: intersect.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::SphereIntersection::closest_intersection_distance
// 信息 ID: d57bdfa5104ea01b4c27883a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::hover_intersection::hover_intersection @ 0x113b793 -> 0xcee4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::hover_intersection::hover_intersection @ 0x113b7bd -> 0xcee4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::aabb
// 信息 ID: e8f523fbfc6aef3fa87fb92f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53deab -> 0xcee110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53dedc -> 0xcee110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::intersect_triangle
// 信息 ID: 9d593fa1e5875aacf9ac8711
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53de7d -> 0xcee200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::new
// 信息 ID: c7385a79dc1308deaaa67fc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls @ 0x4f70bc -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls @ 0x4f7139 -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::ui_arch_controls::ui_arch_controls @ 0x4f980d -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::ui_arch_controls::ui_arch_controls @ 0x4f988a -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::construct_arch_geo @ 0x5919fe -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::construct_arch_geo @ 0x591af5 -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_splitter::wall_splitter @ 0x6374bf -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_splitter::wall_splitter @ 0x63753a -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2> @ 0x66c8ae -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2> @ 0x66c99e -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2> @ 0x66cc96 -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2> @ 0x66cd78 -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$2> @ 0x66d31a -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$2> @ 0x66d3ea -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$2> @ 0x66d5c8 -> 0xcee030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$2> @ 0x66d689 -> 0xcee030 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::normal
// 信息 ID: 86c507948114e0a0e84c6adf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53dfcb -> 0xcee060 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53dfd7 -> 0xcee060 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::construct_arch_geo @ 0x591a09 -> 0xcee060 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::construct_arch_geo @ 0x591b05 -> 0xcee060 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x1985977 -> 0xcee060 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=1164; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=1592; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::points_mut
// 信息 ID: 989604761b0dff83b521dbeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 179 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9d9 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dbad -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfc0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1c2 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5b0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e804 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebf7 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1edab -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f187 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f33b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f712 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f86a -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbb3 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fdac -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2018b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2030b -> 0x43c290 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::impl$1::hash
// 信息 ID: 4c94ec88bd7553582c21c812
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::Hash::hash_slice<utils::geometry::intersect::Triangle,std::hash::random::DefaultHasher>; module=1378, inline_record=3324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::ray_sphere_intersection
// 信息 ID: 52e2aee7c65df3e1d8aa55df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::hover_intersection::hover_intersection @ 0x113b776 -> 0xcee500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::ray_triangle_intersection
// 信息 ID: 8e1526729bd13ec267d6c5e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
