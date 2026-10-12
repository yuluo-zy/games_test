// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/utils/project_cursor_onto_ray.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 87e0697c08490a0bd08ddc778bff39f185a6933a1b7a07afae724c42d389c2b0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: project_cursor_onto_ray.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 513, 527, 559, 565, 579, 1640
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=16
// 源码行: 18, 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x730D
// 首条位置: {"module":513,"inlinee":29453}
// 编译实例 RVAs: 0xac8420
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=16
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x730E
// 首条位置: {"module":513,"inlinee":29454}
// 编译实例 RVAs: 0xac8450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::mouse_ss_to_cs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1868369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380015184,"raw_hex":"e851821c00","size":5,"soft_fail":false,"target_va":5381883600,"va":5380015226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/project_cursor_onto_ray.rs
//     evidence={"module":513,"record_offset":65832,"parent_record_offset":65808,"ancestor_record_offset":65808,"inlinee":29456,"source_line":36,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":513,"record_offset":65880,"parent_record_offset":65856,"ancestor_record_offset":65808,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":513,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":65808,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7310
// 首条位置: {"module":513,"inlinee":29456}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":513,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":65832,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":513,"record_offset":65880,"parent_record_offset":65856,"ancestor_record_offset":65832,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00513.jsonl.gz"}
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
// 首条位置: {"module":565,"record_offset":65800,"rva":11306144,"va":5380015264}
// 编译实例 RVAs: 0xac84a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2895118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380015264,"raw_hex":"e80e2d2c00","size":5,"soft_fail":false,"target_va":5382911104,"va":5380015981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1867291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380015264,"raw_hex":"e81b7e1c00","size":5,"soft_fail":false,"target_va":5381883088,"va":5380015792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":565,"record_offset":66008,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":565,"record_offset":66128,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":29550,"source_line":1151,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":565,"record_offset":67072,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":18169,"source_line":1405,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":565,"record_offset":68108,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":565,"record_offset":68052,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":565,"record_offset":68620,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":565,"record_offset":66560,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] utils::perspective_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":565,"record_offset":68564,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":32389,"source_line":300,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":565,"record_offset":68276,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":565,"record_offset":66268,"parent_record_offset":66176,"ancestor_record_offset":65800,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":565,"record_offset":66240,"parent_record_offset":66176,"ancestor_record_offset":65800,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":565,"record_offset":66212,"parent_record_offset":66176,"ancestor_record_offset":65800,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":565,"record_offset":67176,"parent_record_offset":67136,"ancestor_record_offset":65800,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":565,"record_offset":66092,"parent_record_offset":66008,"ancestor_record_offset":65800,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":565,"record_offset":68532,"parent_record_offset":68276,"ancestor_record_offset":65800,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":565,"record_offset":66064,"parent_record_offset":66008,"ancestor_record_offset":65800,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":565,"record_offset":66592,"parent_record_offset":66560,"ancestor_record_offset":65800,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":565,"record_offset":68132,"parent_record_offset":68108,"ancestor_record_offset":65800,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":565,"record_offset":68076,"parent_record_offset":68052,"ancestor_record_offset":65800,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":565,"record_offset":68332,"parent_record_offset":68308,"ancestor_record_offset":65800,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":565,"record_offset":67100,"parent_record_offset":67072,"ancestor_record_offset":65800,"inlinee":18170,"source_line":1227,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":565,"record_offset":66176,"parent_record_offset":66128,"ancestor_record_offset":65800,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":565,"record_offset":66500,"parent_record_offset":66128,"ancestor_record_offset":65800,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":565,"record_offset":68444,"parent_record_offset":68420,"ancestor_record_offset":65800,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":565,"record_offset":68308,"parent_record_offset":68276,"ancestor_record_offset":65800,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":565,"record_offset":68644,"parent_record_offset":68620,"ancestor_record_offset":65800,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":565,"record_offset":68792,"parent_record_offset":68620,"ancestor_record_offset":65800,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":565,"record_offset":67268,"parent_record_offset":67100,"ancestor_record_offset":65800,"inlinee":18172,"source_line":547,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":565,"record_offset":67364,"parent_record_offset":67100,"ancestor_record_offset":65800,"inlinee":18173,"source_line":1051,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":565,"record_offset":67136,"parent_record_offset":67100,"ancestor_record_offset":65800,"inlinee":18174,"source_line":1561,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":565,"record_offset":68420,"parent_record_offset":68276,"ancestor_record_offset":65800,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00565.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
// 信息 ID: 16f96b6d2e8a451751127d4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x7ae533 -> 0xac8420 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x7ae800 -> 0xac8420 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length @ 0x21a95a9 -> 0xac8420 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x21ade3c -> 0xac8420 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=513, inline_record=65776; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=30812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=31328; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line; module=559, inline_record=76036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode; module=565, inline_record=50872; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=579, inline_record=70452; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=579, inline_record=71028; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=1640, inline_record=51672; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
// 信息 ID: 494762f93f8b89f29cf21253
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x7ae54f -> 0xac8450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x7ae81c -> 0xac8450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length @ 0x21a95c1 -> 0xac8450 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x21ade5f -> 0xac8450 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=513, inline_record=65808; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=30844; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=31360; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line; module=559, inline_record=76068; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode; module=565, inline_record=50904; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=579, inline_record=70484; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=579, inline_record=71060; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=1640, inline_record=51704; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::from_screen_space
// 信息 ID: 6d366b8dd93338e91b3a2e0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=513, inline_record=65832; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=30868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=527, inline_record=31384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line; module=559, inline_record=76092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode; module=565, inline_record=50928; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space; module=565, inline_record=65692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=579, inline_record=70508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>; module=579, inline_record=71084; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
// 信息 ID: c39ff0fe2dfac04e30440405
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x7ae5e0 -> 0xac84a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x7ae894 -> 0xac84a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x9a6515 -> 0xac84a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line @ 0xa9bdfc -> 0xac84a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode @ 0xac727f -> 0xac84a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0> @ 0xb1cd99 -> 0xac84a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0> @ 0xb1cefa -> 0xac84a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length @ 0x21a9637 -> 0xac84a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x21aded7 -> 0xac84a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
