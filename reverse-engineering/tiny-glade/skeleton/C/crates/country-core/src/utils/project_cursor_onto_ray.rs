// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/utils/project_cursor_onto_ray.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 87e0697c08490a0bd08ddc778bff39f185a6933a1b7a07afae724c42d389c2b0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: project_cursor_onto_ray.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 549, 569, 574, 585, 1762
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=17
// 源码行: 18, 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8555
// 首条位置: {"module":549,"inlinee":34133}
// 编译实例 RVAs: 0xbe9dc0
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=17
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8556
// 首条位置: {"module":549,"inlinee":34134}
// 编译实例 RVAs: 0xbe9df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::mouse_ss_to_cs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2072945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381201392,"raw_hex":"e871a11f00","size":5,"soft_fail":false,"target_va":5383274384,"va":5381201434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/project_cursor_onto_ray.rs
//     evidence={"module":549,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":3868,"inlinee":34136,"source_line":36,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3868,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3868,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8558
// 首条位置: {"module":549,"inlinee":34136}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3892,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3892,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
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
// 首条位置: {"module":585,"record_offset":3512,"rva":12492352,"va":5381201472}
// 编译实例 RVAs: 0xbe9e40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35419970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381201472,"raw_hex":"e842771c02","size":5,"soft_fail":false,"target_va":5416622240,"va":5381202265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2079554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381201472,"raw_hex":"e842bb1f00","size":5,"soft_fail":false,"target_va":5383281568,"va":5381202009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":585,"record_offset":3720,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":585,"record_offset":3836,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":585,"record_offset":4948,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":30714,"source_line":1636,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":585,"record_offset":5984,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":585,"record_offset":5928,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":585,"record_offset":6496,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":585,"record_offset":4352,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] utils::perspective_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":585,"record_offset":6440,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":37308,"source_line":300,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":585,"record_offset":6152,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":585,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3512,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":4004,"parent_record_offset":3884,"ancestor_record_offset":3512,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":3976,"parent_record_offset":3884,"ancestor_record_offset":3512,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":3920,"parent_record_offset":3884,"ancestor_record_offset":3512,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":5052,"parent_record_offset":5012,"ancestor_record_offset":3512,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":585,"record_offset":3768,"parent_record_offset":3720,"ancestor_record_offset":3512,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":585,"record_offset":3796,"parent_record_offset":3768,"ancestor_record_offset":3512,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":585,"record_offset":6408,"parent_record_offset":6152,"ancestor_record_offset":3512,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":585,"record_offset":4384,"parent_record_offset":4352,"ancestor_record_offset":3512,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":585,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":3512,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":585,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":3512,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":585,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":3512,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":585,"record_offset":4976,"parent_record_offset":4948,"ancestor_record_offset":3512,"inlinee":20184,"source_line":1357,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":585,"record_offset":3884,"parent_record_offset":3836,"ancestor_record_offset":3512,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":585,"record_offset":4292,"parent_record_offset":3836,"ancestor_record_offset":3512,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":585,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":3512,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":585,"record_offset":6184,"parent_record_offset":6152,"ancestor_record_offset":3512,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":585,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":3512,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":585,"record_offset":6668,"parent_record_offset":6496,"ancestor_record_offset":3512,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":585,"record_offset":5144,"parent_record_offset":4976,"ancestor_record_offset":3512,"inlinee":20185,"source_line":646,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":585,"record_offset":5240,"parent_record_offset":4976,"ancestor_record_offset":3512,"inlinee":20186,"source_line":1066,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":585,"record_offset":5012,"parent_record_offset":4976,"ancestor_record_offset":3512,"inlinee":20187,"source_line":1491,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":585,"record_offset":6296,"parent_record_offset":6152,"ancestor_record_offset":3512,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00585.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
// 信息 ID: 67cc5b0b44255911408e3ec0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x86ad04 -> 0xbe9dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x86aff9 -> 0xbe9dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length @ 0x1f20093 -> 0xbe9dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x1f37090 -> 0xbe9dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x217136b -> 0xbe9dc0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=549, inline_record=3836; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=549, inline_record=4344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4>; module=549, inline_record=5104; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4>; module=549, inline_record=5352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=569, inline_record=59212; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line; module=569, inline_record=85268; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=15852; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=22096; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
// 信息 ID: 84020056bde1b70ad904ae0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x86ad20 -> 0xbe9df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x86b015 -> 0xbe9df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length @ 0x1f200ac -> 0xbe9df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x1f370b1 -> 0xbe9df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2171395 -> 0xbe9df0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=549, inline_record=3868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=549, inline_record=4376; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4>; module=549, inline_record=5136; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4>; module=549, inline_record=5384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=569, inline_record=59244; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line; module=569, inline_record=85300; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=15884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=22128; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
// 信息 ID: a6b4377a9ad57c2e30261a46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=549, inline_record=3892; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=549, inline_record=4400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4>; module=549, inline_record=5160; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4>; module=549, inline_record=5408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=569, inline_record=59268; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line; module=569, inline_record=85324; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=15908; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=22152; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
// 信息 ID: b2f27f2085085f298b50de34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x86adaa -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x86b08d -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0> @ 0xab27d6 -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0> @ 0xab294d -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4> @ 0xab2b4c -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group::closure_env$4> @ 0xab2c18 -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_raise_shape::impl_raise_shape @ 0xb522a8 -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line @ 0xb5584c -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7cef8 -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7dd0a -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode @ 0xbeb62e -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x1abd75a -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x1abda65 -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length @ 0x1f20123 -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x1f37129 -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2171483 -> 0xbe9e40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
