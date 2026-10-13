// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/utils/project_cursor_onto_ray.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4ece09d1fa84e16be9675db6c48e5510d2d5b53175ba0dfa1cf387d480fbb1ae
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: project_cursor_onto_ray.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 438, 496, 1447, 2831
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 18, 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x62E8
// 首条位置: {"module":438,"inlinee":25320}
// 编译实例 RVAs: 0x5e4770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x62E9
// 首条位置: {"module":438,"inlinee":25321}
// 编译实例 RVAs: 0x5e47a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::mouse_ss_to_cs
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2052529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374887840,"raw_hex":"e8b1511f00","size":5,"soft_fail":false,"target_va":5376940416,"va":5374887882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/project_cursor_onto_ray.rs
//     evidence={"module":438,"record_offset":30440,"parent_record_offset":30260,"ancestor_record_offset":30260,"inlinee":25317,"source_line":36,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":30488,"parent_record_offset":30464,"ancestor_record_offset":30260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":30464,"parent_record_offset":30440,"ancestor_record_offset":30260,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x62E5
// 首条位置: {"module":438,"inlinee":25317}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":30464,"parent_record_offset":30440,"ancestor_record_offset":30440,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":30488,"parent_record_offset":30464,"ancestor_record_offset":30440,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":30556,"rva":6178800,"va":5374887920}
// 编译实例 RVAs: 0x5e47f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28090097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374887920,"raw_hex":"e8f19eac01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374888842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2051406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374887920,"raw_hex":"e84e4d1f00","size":5,"soft_fail":false,"target_va":5376939952,"va":5374888541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":438,"record_offset":31188,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":438,"record_offset":30764,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":23594,"source_line":1098,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":438,"record_offset":31280,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":15692,"source_line":1335,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":32736,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":32792,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":33276,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":31756,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] utils::perspective_lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":438,"record_offset":33248,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":25318,"source_line":293,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":438,"record_offset":32960,"parent_record_offset":30556,"ancestor_record_offset":30556,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":30980,"parent_record_offset":30896,"ancestor_record_offset":30556,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":30864,"parent_record_offset":30812,"ancestor_record_offset":30556,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":30836,"parent_record_offset":30812,"ancestor_record_offset":30556,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":31384,"parent_record_offset":31344,"ancestor_record_offset":30556,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":438,"record_offset":33216,"parent_record_offset":32960,"ancestor_record_offset":30556,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":438,"record_offset":31248,"parent_record_offset":31188,"ancestor_record_offset":30556,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":32816,"parent_record_offset":32792,"ancestor_record_offset":30556,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":31788,"parent_record_offset":31756,"ancestor_record_offset":30556,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":32760,"parent_record_offset":32736,"ancestor_record_offset":30556,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":30556,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":438,"record_offset":31308,"parent_record_offset":31280,"ancestor_record_offset":30556,"inlinee":15693,"source_line":1157,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":438,"record_offset":30812,"parent_record_offset":30764,"ancestor_record_offset":30556,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$13::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":438,"record_offset":31128,"parent_record_offset":30764,"ancestor_record_offset":30556,"inlinee":15701,"source_line":998,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":33128,"parent_record_offset":33104,"ancestor_record_offset":30556,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":32992,"parent_record_offset":32960,"ancestor_record_offset":30556,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":33300,"parent_record_offset":33276,"ancestor_record_offset":30556,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":438,"record_offset":31476,"parent_record_offset":31308,"ancestor_record_offset":30556,"inlinee":15695,"source_line":803,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":438,"record_offset":31568,"parent_record_offset":31308,"ancestor_record_offset":30556,"inlinee":15696,"source_line":1313,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":438,"record_offset":31344,"parent_record_offset":31308,"ancestor_record_offset":30556,"inlinee":15698,"source_line":1823,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":438,"record_offset":33104,"parent_record_offset":32960,"ancestor_record_offset":30556,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00438.jsonl.gz"}
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
// 完整结构及原始类型记录: project_cursor_onto_ray.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: project_cursor_onto_ray.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: project_cursor_onto_ray.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
// 信息 ID: 58e7898bdeddde9996662f41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length @ 0x198b642 -> 0x5e4770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x198d6eb -> 0x5e4770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x1c756ab -> 0x5e4770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x1c759e7 -> 0x5e4770 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode; module=496, inline_record=20244; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=1447, inline_record=44980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=17596; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=24308; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
// 信息 ID: a2385e99653592961bccdf23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length @ 0x198b657 -> 0x5e47a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x198d70a -> 0x5e47a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x1c756c7 -> 0x5e47a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x1c75a03 -> 0x5e47a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode; module=496, inline_record=20276; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=1447, inline_record=45012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=17628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=24340; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
// 信息 ID: ead5efe88ff7796b04d7ddac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space; module=438, inline_record=30440; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode; module=496, inline_record=20300; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=1447, inline_record=45036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=17652; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=24364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
// 信息 ID: e6a6bc19734213288d053b47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode @ 0x6c6910 -> 0x5e47f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length @ 0x198b6cd -> 0x5e47f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x198d780 -> 0x5e47f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x1c75751 -> 0x5e47f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x1c75a7a -> 0x5e47f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
