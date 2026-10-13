// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/geometry/aabb.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d670d116920871a6885a44f7f17a806a70351169a671f5b27c92e06f96b94d8c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: aabb.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 404, 446, 486, 740, 1229, 1234, 1327, 1349, 1365, 3468, 3473, 3474
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
// 函数符号: utils::geometry::aabb::intersection_aabb_segment2
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":7076,"rva":15713104,"va":5384422224}
// 编译实例 RVAs: 0xefc350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":8016,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":63175,"source_line":282,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":7960,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":63174,"source_line":304,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":7760,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":7848,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":45804,"source_line":220,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":7656,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":23218,"source_line":173,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":7904,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":45805,"source_line":211,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":8128,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":7248,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":7392,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":8072,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":8040,"parent_record_offset":8016,"ancestor_record_offset":7076,"inlinee":23826,"source_line":1521,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":7984,"parent_record_offset":7960,"ancestor_record_offset":7076,"inlinee":5481,"source_line":1515,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":7788,"parent_record_offset":7760,"ancestor_record_offset":7076,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":7692,"parent_record_offset":7656,"ancestor_record_offset":7076,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7076,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":7076,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":7076,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":7076,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::intersection_aabb_segment3
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":8188,"rva":15713616,"va":5384422736}
// 编译实例 RVAs: 0xefc550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/bvec3.rs
//     evidence={"module":1349,"record_offset":9416,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":25013,"source_line":66,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":9444,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":63167,"source_line":333,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":9356,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":25011,"source_line":355,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":9120,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":60096,"source_line":235,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":9244,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":13198,"source_line":271,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":8932,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":60095,"source_line":222,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":9300,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":23254,"source_line":262,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":9620,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":8364,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":8568,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":63173,"source_line":973,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":9532,"parent_record_offset":8188,"ancestor_record_offset":8188,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":9472,"parent_record_offset":9444,"ancestor_record_offset":8188,"inlinee":23826,"source_line":1521,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":9384,"parent_record_offset":9356,"ancestor_record_offset":8188,"inlinee":5481,"source_line":1515,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":9156,"parent_record_offset":9120,"ancestor_record_offset":8188,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":8988,"parent_record_offset":8932,"ancestor_record_offset":8188,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":8188,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":9648,"parent_record_offset":9620,"ancestor_record_offset":8188,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":8600,"parent_record_offset":8568,"ancestor_record_offset":8188,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":8188,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::is_inside_bbx
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":244,"rva":15710832,"va":5384419952}
// 编译实例 RVAs: 0xefba70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_center_dims
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E5
// 首条位置: {"module":740,"inlinee":45797}
// 编译实例 RVAs: 0xefbaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18559043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384420000,"raw_hex":"e843301b01","size":5,"soft_fail":false,"target_va":5402979152,"va":5384420104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29216,"parent_record_offset":29180,"ancestor_record_offset":29180,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29364,"parent_record_offset":29180,"ancestor_record_offset":29180,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":30536,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29308,"parent_record_offset":29180,"ancestor_record_offset":29180,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":29396,"parent_record_offset":29364,"ancestor_record_offset":29180,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":29332,"parent_record_offset":29308,"ancestor_record_offset":29180,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":30564,"parent_record_offset":30536,"ancestor_record_offset":30300,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29240,"parent_record_offset":29216,"ancestor_record_offset":29180,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":29272,"parent_record_offset":29240,"ancestor_record_offset":29180,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_center_radius
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":696,"rva":15710992,"va":5384420112}
// 编译实例 RVAs: 0xefbb10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18558971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384420112,"raw_hex":"e8fb2f1b01","size":5,"soft_fail":false,"target_va":5402979152,"va":5384420176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$19::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":828,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":45882,"source_line":1070,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":848,"parent_record_offset":828,"ancestor_record_offset":696,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_points
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5AB4
// 首条位置: {"module":486,"inlinee":23220}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":486,"record_offset":37308,"parent_record_offset":37260,"ancestor_record_offset":37260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":486,"record_offset":37452,"parent_record_offset":37260,"ancestor_record_offset":37260,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":486,"record_offset":37516,"parent_record_offset":37260,"ancestor_record_offset":37260,"inlinee":23218,"source_line":173,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":486,"record_offset":37480,"parent_record_offset":37452,"ancestor_record_offset":37260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":486,"record_offset":37540,"parent_record_offset":37516,"ancestor_record_offset":37260,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":486,"record_offset":37336,"parent_record_offset":37308,"ancestor_record_offset":37260,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1234,"record_offset":1116,"parent_record_offset":996,"ancestor_record_offset":916,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":486,"record_offset":37408,"parent_record_offset":37364,"ancestor_record_offset":37260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":486,"record_offset":37364,"parent_record_offset":37336,"ancestor_record_offset":37260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 100, 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB306
// 首条位置: {"module":740,"inlinee":45830}
// 编译实例 RVAs: 0xefbb60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":38648,"parent_record_offset":38624,"ancestor_record_offset":38624,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":38704,"parent_record_offset":38624,"ancestor_record_offset":38624,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":38728,"parent_record_offset":38704,"ancestor_record_offset":38624,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":38672,"parent_record_offset":38648,"ancestor_record_offset":38624,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::half_dims
// 记录形式: compiled_procedure; instances=1
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":1180,"rva":15711120,"va":5384420240}
// 编译实例 RVAs: 0xefbb90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":1308,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":1392,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":1416,"parent_record_offset":1392,"ancestor_record_offset":1180,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":1180,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::area
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":1480,"rva":15711168,"va":5384420288}
// 编译实例 RVAs: 0xefbbc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::contains
// 记录形式: compiled_procedure; instances=1
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":1592,"rva":15711200,"va":5384420320}
// 编译实例 RVAs: 0xefbbe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1349,"record_offset":1716,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":1592,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":1792,"parent_record_offset":1772,"ancestor_record_offset":1592,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1349,"record_offset":1744,"parent_record_offset":1716,"ancestor_record_offset":1592,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::intersects
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E9
// 首条位置: {"module":740,"inlinee":45801}
// 编译实例 RVAs: 0xefbc10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::intersect_ray
// 记录形式: compiled_procedure, inlinee_declaration; instances=3
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2EB
// 首条位置: {"module":740,"inlinee":45803}
// 编译实例 RVAs: 0xefbc60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":2644,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":2732,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":45804,"source_line":220,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":2540,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":23218,"source_line":173,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":2788,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":45805,"source_line":211,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":2204,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":2288,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":2672,"parent_record_offset":2644,"ancestor_record_offset":2056,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":2576,"parent_record_offset":2540,"ancestor_record_offset":2056,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":2056,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":2056,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::overlaps_segment
// 记录形式: inlinee_declaration; instances=1
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2EA
// 首条位置: {"module":740,"inlinee":45802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::max_extent
// 记录形式: compiled_procedure; instances=1
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":2848,"rva":15711568,"va":5384420688}
// 编译实例 RVAs: 0xefbd50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":3060,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":45804,"source_line":220,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":2976,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2848,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":2848,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 173, 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB304
// 首条位置: {"module":740,"inlinee":45828}
// 编译实例 RVAs: 0xefbd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::height
// 记录形式: compiled_procedure; instances=1
// 源码行: 178
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":3232,"rva":15711632,"va":5384420752}
// 编译实例 RVAs: 0xefbd90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::expanded
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":3344,"rva":15711648,"va":5384420768}
// 编译实例 RVAs: 0xefbda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":3524,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1349,"record_offset":3472,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":3492,"parent_record_offset":3472,"ancestor_record_offset":3344,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":3548,"parent_record_offset":3524,"ancestor_record_offset":3344,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::from_center_extent
// 记录形式: inlinee_declaration; instances=1
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF6BE
// 首条位置: {"module":1349,"inlinee":63166}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::from_points
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5AB6
// 首条位置: {"module":446,"inlinee":23222}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1365,"record_offset":21972,"parent_record_offset":21916,"ancestor_record_offset":21916,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1365,"record_offset":22000,"parent_record_offset":21972,"ancestor_record_offset":21916,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1365,"record_offset":22344,"parent_record_offset":22320,"ancestor_record_offset":21916,"inlinee":4401,"source_line":166,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1365,"record_offset":22320,"parent_record_offset":22296,"ancestor_record_offset":21916,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1365,"record_offset":22092,"parent_record_offset":22000,"ancestor_record_offset":21916,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1365,"record_offset":22296,"parent_record_offset":22128,"ancestor_record_offset":21916,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1365,"record_offset":22060,"parent_record_offset":22028,"ancestor_record_offset":21916,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1365,"record_offset":22028,"parent_record_offset":22000,"ancestor_record_offset":21916,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1365,"record_offset":22392,"parent_record_offset":22368,"ancestor_record_offset":21916,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1365,"record_offset":22368,"parent_record_offset":22344,"ancestor_record_offset":21916,"inlinee":22717,"source_line":1357,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::expand_to_contain_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":3584,"rva":15711696,"va":5384420816}
// 编译实例 RVAs: 0xefbdd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":3812,"parent_record_offset":3584,"ancestor_record_offset":3584,"inlinee":60096,"source_line":235,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":3728,"parent_record_offset":3584,"ancestor_record_offset":3584,"inlinee":60095,"source_line":222,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":3840,"parent_record_offset":3812,"ancestor_record_offset":3584,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":3756,"parent_record_offset":3728,"ancestor_record_offset":3584,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::expand
// 记录形式: compiled_procedure; instances=1
// 源码行: 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":3904,"rva":15711840,"va":5384420960}
// 编译实例 RVAs: 0xefbe60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":4180,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":4116,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":4032,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":3904,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":3904,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":3904,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::center
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 270, 271
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF6BC
// 首条位置: {"module":1349,"inlinee":63164}
// 编译实例 RVAs: 0xefbec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":4364,"parent_record_offset":4240,"ancestor_record_offset":4240,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":4452,"parent_record_offset":4240,"ancestor_record_offset":4240,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":4480,"parent_record_offset":4452,"ancestor_record_offset":4240,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":4392,"parent_record_offset":4364,"ancestor_record_offset":4240,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::extent
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 274
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF6BD
// 首条位置: {"module":1349,"inlinee":63165}
// 编译实例 RVAs: 0xefbf00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":4664,"parent_record_offset":4544,"ancestor_record_offset":4544,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":4692,"parent_record_offset":4664,"ancestor_record_offset":4544,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::scaled
// 记录形式: compiled_procedure; instances=1
// 源码行: 278
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":4756,"rva":15712048,"va":5384421168}
// 编译实例 RVAs: 0xefbf30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":4876,"parent_record_offset":4756,"ancestor_record_offset":4756,"inlinee":22360,"source_line":1027,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":4756,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::translated
// 记录形式: compiled_procedure; instances=1
// 源码行: 289
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":4996,"rva":15712096,"va":5384421216}
// 编译实例 RVAs: 0xefbf60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":5120,"parent_record_offset":4996,"ancestor_record_offset":4996,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":4996,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::inclusive_contains_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 303
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":5240,"rva":15712144,"va":5384421264}
// 编译实例 RVAs: 0xefbf90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/bvec3.rs
//     evidence={"module":1349,"record_offset":5544,"parent_record_offset":5240,"ancestor_record_offset":5240,"inlinee":25013,"source_line":66,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::bool::bvec3::impl$2::bitand
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/bvec3.rs
//     evidence={"module":1349,"record_offset":5516,"parent_record_offset":5240,"ancestor_record_offset":5240,"inlinee":63169,"source_line":124,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":5392,"parent_record_offset":5240,"ancestor_record_offset":5240,"inlinee":62403,"source_line":322,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":5456,"parent_record_offset":5240,"ancestor_record_offset":5240,"inlinee":63170,"source_line":344,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":5420,"parent_record_offset":5392,"ancestor_record_offset":5240,"inlinee":17312,"source_line":1519,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":5484,"parent_record_offset":5456,"ancestor_record_offset":5240,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::intersect_segment
// 记录形式: compiled_procedure; instances=1
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":5576,"rva":15712272,"va":5384421392}
// 编译实例 RVAs: 0xefc010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::aabb::intersection_aabb_segment3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384421392,"raw_hex":"e824050000","size":5,"soft_fail":false,"target_va":5384422736,"va":5384421415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::classify_ray_intersection
// 记录形式: compiled_procedure; instances=1
// 源码行: 332
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1349,"record_offset":5700,"rva":15712320,"va":5384421440}
// 编译实例 RVAs: 0xefc040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/bvec3.rs
//     evidence={"module":1349,"record_offset":6828,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":25013,"source_line":66,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":6856,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":63167,"source_line":333,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":6768,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":25011,"source_line":355,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":6532,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":60096,"source_line":235,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":6656,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":13198,"source_line":271,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":6344,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":60095,"source_line":222,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":6712,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":23254,"source_line":262,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":5872,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1349,"record_offset":5988,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":63173,"source_line":973,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":6884,"parent_record_offset":6856,"ancestor_record_offset":5700,"inlinee":23826,"source_line":1521,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1349,"record_offset":6796,"parent_record_offset":6768,"ancestor_record_offset":5700,"inlinee":5481,"source_line":1515,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":6568,"parent_record_offset":6532,"ancestor_record_offset":5700,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1349,"record_offset":6400,"parent_record_offset":6344,"ancestor_record_offset":5700,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":6016,"parent_record_offset":5988,"ancestor_record_offset":5700,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1349,"record_offset":5900,"parent_record_offset":5872,"ancestor_record_offset":5700,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::RayBoxIntersection::is_hit
// 记录形式: compiled_procedure, inlinee_declaration; instances=3
// 源码行: 392, 393
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2EF
// 首条位置: {"module":740,"inlinee":45807}
// 编译实例 RVAs: 0xefc330
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
// 完整结构及原始类型记录: aabb.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::aabb::Aabb2
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 18; 过程引用数: 27
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::ops::range::Range<usize>", "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<utils::geometry::aabb::Aabb2>", "constructor": "core::slice::iter::Iter", "arguments": ["utils::geometry::aabb::Aabb2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::slice::iter::Iter<utils::geometry::aabb::Aabb2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >", "alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$11"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["core::ops::range::Range<usize>", "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<utils::geometry::aabb::Aabb2>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$8"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "constructor": "tuple$", "arguments": ["utils::geometry::aabb::Aabb2", "country_core::resources::walls::wall_holes::HoleType", "country_core::resources::walls::wall_holes::HoleOrigin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["utils::geometry::aabb::Aabb2", "system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>", "alloc::slice::stable_sort::closure_env$0<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "alloc::slice::stable_sort::closure_env$1<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "alloc::slice::stable_sort::closure_env$2<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "alloc::slice::stable_sort::closure_env$3<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >", "constructor": "system_wall_constructor::plaster_systems::plaster_mesh", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>", "alloc::slice::stable_sort::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "alloc::slice::stable_sort::closure_env$1<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "alloc::slice::stable_sort::closure_env$2<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "alloc::slice::stable_sort::closure_env$3<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<utils::geometry::aabb::Aabb2>", "constructor": "ref$", "arguments": ["utils::geometry::aabb::Aabb2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,ref$<utils::geometry::aabb::Aabb2> >", "constructor": "tuple$", "arguments": ["usize", "ref$<utils::geometry::aabb::Aabb2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >", "constructor": "tuple$", "arguments": ["tuple$<usize,ref$<utils::geometry::aabb::Aabb2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>", "constructor": "core::ops::function::impls::impl$4::call_once", "arguments": ["tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$11"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::aabb::Aabb3
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 21; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "slotmap::basic::impl$9::next<country_core::asset_libraries::audio_trigger_library::AudioTriggerCollectionHandle,utils::geometry::aabb::Aabb3>", "constructor": "slotmap::basic::impl$9::next", "arguments": ["country_core::asset_libraries::audio_trigger_library::AudioTriggerCollectionHandle", "utils::geometry::aabb::Aabb3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::aabb::RayBoxIntersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: aabb.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: aabb.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<country_core::systems::ui_edit_freehand_walls::EditFreehandWallCache>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::events::OnFreehandWallEdited>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesNextToWallCmd>)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<country_core::systems::ui_edit_freehand_walls::EditFreehandWallCache>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::EventWriter<country_core::resources::events::OnFreehandWallEdited>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesNextToWallCmd>"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_edit_freehand_walls::EditFreehandWallCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::OnFreehandWallEdited>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesNextToWallCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::area
// 信息 ID: 99a76f747980174f5e004a2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze @ 0x69c125 -> 0xefbbc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::center
// 信息 ID: ce737af4e521945aeedf3f70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 85 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca07c5 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb425 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb447 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb45e -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb518 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb54f -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb568 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb59c -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb5b3 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb5cc -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb5e3 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbdf7 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbe07 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb5fa -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb636 -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb64f -> 0xefbb60 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=740, inline_record=38624; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::contains
// 信息 ID: 6e2072acb21609957343fd0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::is_point_still_valid @ 0x5d2034 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf24d -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf37f -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf466 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd2296 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3d81 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0xcd72ad -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0xcd720a -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_auto_hide @ 0x165e763 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_auto_hide @ 0x165f255 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x177e783 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x177e799 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x177e7b6 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x177e7cb -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::ui_slider::ui_slider @ 0x1b5a58d -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::ui_slider::ui_slider @ 0x1b5aaa1 -> 0xefbbe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::expanded
// 信息 ID: 6bf9e1eef4995165172c131a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdb82d -> 0xefbda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdcf20 -> 0xefbda0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_center_dims
// 信息 ID: eaa8669151d29fb6b41d98fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 41 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoleStorage::finalize @ 0x62961a -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoleStorage::finalize @ 0x629799 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b71d7 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b7dd4 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4f1d -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7c49 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb293 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb34d -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb401 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb58d -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb674 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb768 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb822 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb8d4 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf354 -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf43b -> 0xefbaa0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=29180; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=30300; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=31396; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_center_radius
// 信息 ID: 016629c6219ee35c1f8aed67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x14e489a -> 0xefbb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad8deb -> 0xefbb10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_points
// 信息 ID: 9162783d7d6999dc6eb38c9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze; module=486, inline_record=37260; 不是独立调用边。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1234, inline_record=916; 不是独立调用边。
// [PDB 内联宿主] system_color::set_wall_color_id::set_wall_color_id; module=3468, inline_record=5340; 不是独立调用边。
// [PDB 内联宿主] system_color::set_gate_color_id::set_gate_color_id; module=3473, inline_record=3216; 不是独立调用边。
// [PDB 内联宿主] system_color::set_halftimber_color_id::set_halftimber_color_id; module=3474, inline_record=3288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::half_dims
// 信息 ID: c36c0de14b252215d6e34caf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0xce2c35 -> 0xefbb90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::height
// 信息 ID: ae1b40edb3a28f0435a50945
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 25 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca07b2 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbdde -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbe6a -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbf50 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbf6b -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccc11e -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf037 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf32e -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf415 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd05f3 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd2693 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd275e -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0xcd6ce1 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0xcd6d94 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0xcd71e1 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0xcd7f15 -> 0xefbd90 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::intersect_ray
// 信息 ID: 51a2a6bfa596d08c988cdc2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0x550846 -> 0xefbc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b74fe -> 0xefbc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b811c -> 0xefbc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4f30 -> 0xefbc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7c5c -> 0xefbc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x1044a99 -> 0xefbc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0x112ecde -> 0xefbc60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::intersects
// 信息 ID: 8b04ce673bbce883b1245b32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb77e -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd08fa -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0xcd6c3e -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0xcd6d0c -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0xcd6dbf -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0xcd7f62 -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdb84c -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdcf2f -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdcff9 -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_rectangle2d::closure_env$0> @ 0x104514a -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure_env$0> @ 0x1045b6f -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::add_segment::closure_env$0> @ 0x10467d9 -> 0xefbc10 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=30264; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=31356; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::max_extent
// 信息 ID: 16ff20603c8aca258a1c50bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0xcc9491 -> 0xefbd50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::overlaps_segment
// 信息 ID: e11eed0f60e9d5e242df3407
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::width
// 信息 ID: 060062603bc24e2765bc6c15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 37 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca07a2 -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb52b -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbdc5 -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbe5a -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbfea -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccc00c -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccc10e -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb60d -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb7c4 -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb86d -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf2be -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf31b -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf3ad -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf402 -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd04c6 -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd263e -> 0xefbd80 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=740, inline_record=38408; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::center
// 信息 ID: b40e226ed5196fdfe96e49bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_rt @ 0x55cd4a -> 0xefbec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xf20004 -> 0xefbec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::classify_ray_intersection
// 信息 ID: 5ffbe0cb6bcf5b89a6b062de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::terrain_height::impl$4::intersect_ray @ 0xc8dcfc -> 0xefc040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::expand
// 信息 ID: b92fae3936feb6e10423423f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53ded0 -> 0xefbe60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53df04 -> 0xefbe60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::expand_to_contain_point
// 信息 ID: 364d56fb7eb793317dffaa17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::mesh::Mesh::aabb_bounds @ 0xf1ff20 -> 0xefbdd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::extent
// 信息 ID: 641363a6784e13b70b71f7e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_rt @ 0x55cd57 -> 0xefbf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xf1ffde -> 0xefbf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xf1fff1 -> 0xefbf00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::from_center_extent
// 信息 ID: 71eff5773c11189ed25ca6b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::from_points
// 信息 ID: c85f4996e6870150923a6846
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::mesh::Mesh::aabb_bounds; module=1365, inline_record=21916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::inclusive_contains_point
// 信息 ID: 7939e9034a01065e0c488c45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53df3c -> 0xefbf90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53df7a -> 0xefbf90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53df95 -> 0xefbf90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0x53dfb0 -> 0xefbf90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::intersect_segment
// 信息 ID: 6dc8d293fc28dd19b7d4a8f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::trigger_audio::trigger_audio_system @ 0x618d35 -> 0xefc010 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::tree_animation::handplaced_tree_animation @ 0x1a9ad65 -> 0xefc010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::scaled
// 信息 ID: 119705f8535fbd3e5a478591
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x55c55c -> 0xefbf30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::update_prefabs @ 0x55d79b -> 0xefbf30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs @ 0x1112f7c -> 0xefbf30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs @ 0x11130ce -> 0xefbf30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::translated
// 信息 ID: fa831b40f5598cf676e0f8e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x55c59b -> 0xefbf60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::update_prefabs @ 0x55d7c5 -> 0xefbf60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::RayBoxIntersection::is_hit
// 信息 ID: 4d7b5ba7e7ae4b9ca4f4b972
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0x550862 -> 0xefc330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b751d -> 0xefc330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b813b -> 0xefc330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4f4c -> 0xefc330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7c80 -> 0xefc330 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x1044ab7 -> 0xefc330 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0x112ecfa -> 0xefc330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::intersection_aabb_segment2
// 信息 ID: 7e12303c23b5544dd34c676b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment @ 0x620ff8 -> 0xefc350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::intersection_aabb_segment3
// 信息 ID: d05b49a73b83735888d60288
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::aabb::Aabb3::intersect_segment @ 0xefc027 -> 0xefc550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::is_inside_bbx
// 信息 ID: 59a8750b7688406d512f54c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_spawner::ivy_spawner @ 0x51e615 -> 0xefba70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure_env$0> @ 0x1045644 -> 0xefba70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure_env$0> @ 0x104567d -> 0xefba70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::add_segment::closure_env$0> @ 0x10462f4 -> 0xefba70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::add_segment::closure_env$0> @ 0x104632d -> 0xefba70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
