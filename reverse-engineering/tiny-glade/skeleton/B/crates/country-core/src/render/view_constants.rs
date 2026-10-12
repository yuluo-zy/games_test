// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/view_constants.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3c00f8e409ebfcc8084db638399df221d7ef5fec2df9c7277790ab183f67bd79
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: view_constants.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 39, 571, 579, 1695
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::view_constants::impl$4::default
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2144
// 首条位置: {"module":39,"inlinee":8516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::view_constants::ViewConstants::builder
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F9F
// 首条位置: {"module":571,"inlinee":32671}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::view_constants::ViewConstants::set_pixel_offset
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8143
// 首条位置: {"module":579,"inlinee":33091}
// 编译实例 RVAs: 0xb18af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat4::impl$7::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":12540,"parent_record_offset":12380,"ancestor_record_offset":12380,"inlinee":18165,"source_line":1390,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":16280,"parent_record_offset":12380,"ancestor_record_offset":12380,"inlinee":30378,"source_line":162,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":579,"record_offset":12928,"parent_record_offset":12832,"ancestor_record_offset":12380,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=76
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":579,"record_offset":12796,"parent_record_offset":12728,"ancestor_record_offset":12380,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=68
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":579,"record_offset":12764,"parent_record_offset":12728,"ancestor_record_offset":12380,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_mat4
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":12580,"parent_record_offset":12540,"ancestor_record_offset":12380,"inlinee":18167,"source_line":1238,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":12676,"parent_record_offset":12636,"ancestor_record_offset":12380,"inlinee":18170,"source_line":1227,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$9::mul
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":12636,"parent_record_offset":12580,"ancestor_record_offset":12380,"inlinee":18169,"source_line":1405,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":579,"record_offset":16304,"parent_record_offset":16280,"ancestor_record_offset":12380,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$19::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":579,"record_offset":15052,"parent_record_offset":15012,"ancestor_record_offset":12380,"inlinee":33088,"source_line":1121,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":579,"record_offset":15452,"parent_record_offset":15360,"ancestor_record_offset":12380,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::wwww
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":579,"record_offset":13088,"parent_record_offset":12676,"ancestor_record_offset":12380,"inlinee":18175,"source_line":2071,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":579,"record_offset":12728,"parent_record_offset":12676,"ancestor_record_offset":12380,"inlinee":18172,"source_line":547,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":579,"record_offset":12832,"parent_record_offset":12676,"ancestor_record_offset":12380,"inlinee":18173,"source_line":1051,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":579,"record_offset":12960,"parent_record_offset":12676,"ancestor_record_offset":12380,"inlinee":18174,"source_line":1561,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::view_constants::ViewportConstantBuilder::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":18692,"rva":11636576,"va":5380345696}
// 编译实例 RVAs: 0xb18f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::view_constants::ViewConstants::set_pixel_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/view_constants.rs
//     evidence={"module":579,"record_offset":23436,"parent_record_offset":18692,"ancestor_record_offset":18692,"inlinee":33091,"source_line":48,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::impl$7::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":18848,"parent_record_offset":18692,"ancestor_record_offset":18692,"inlinee":18165,"source_line":1390,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":579,"record_offset":19160,"parent_record_offset":19064,"ancestor_record_offset":18692,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=86
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":579,"record_offset":19032,"parent_record_offset":18968,"ancestor_record_offset":18692,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=82
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":579,"record_offset":19000,"parent_record_offset":18968,"ancestor_record_offset":18692,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_mat4
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":18692,"inlinee":18167,"source_line":1238,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":18932,"parent_record_offset":18908,"ancestor_record_offset":18692,"inlinee":18170,"source_line":1227,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$7::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":23540,"parent_record_offset":23436,"ancestor_record_offset":18692,"inlinee":18165,"source_line":1390,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$9::mul
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":579,"record_offset":18908,"parent_record_offset":18872,"ancestor_record_offset":18692,"inlinee":18169,"source_line":1405,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":579,"record_offset":27356,"parent_record_offset":27328,"ancestor_record_offset":18692,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$19::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":579,"record_offset":23720,"parent_record_offset":23676,"ancestor_record_offset":18692,"inlinee":33088,"source_line":1121,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":579,"record_offset":26480,"parent_record_offset":26388,"ancestor_record_offset":18692,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":27328,"parent_record_offset":23436,"ancestor_record_offset":18692,"inlinee":30378,"source_line":162,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::wwww
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":579,"record_offset":19320,"parent_record_offset":18932,"ancestor_record_offset":18692,"inlinee":18175,"source_line":2071,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":579,"record_offset":18968,"parent_record_offset":18932,"ancestor_record_offset":18692,"inlinee":18172,"source_line":547,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":579,"record_offset":19064,"parent_record_offset":18932,"ancestor_record_offset":18692,"inlinee":18173,"source_line":1051,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":579,"record_offset":19192,"parent_record_offset":18932,"ancestor_record_offset":18692,"inlinee":18174,"source_line":1561,"archive":"_pdb/B/modules/00579.jsonl.gz"}
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
// 完整结构及原始类型记录: view_constants.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::view_constants::ViewConstants
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::array::impl$56::default<country_core::render::view_constants::ViewConstants>", "constructor": "core::array::impl$56::default", "arguments": ["country_core::render::view_constants::ViewConstants"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::view_constants::ViewportConstantBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: view_constants.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: view_constants.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >,bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "tuple$<bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >", "bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::event::reader::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>", "bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::tree_audio::TreeAudioState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<ref$<country_core::components::TreeAmbience>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::tree_audio::TreeAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::system::query::Query<ref$<country_core::components::TreeAmbience>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::tree_audio::TreeAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::components::TreeAmbience>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::render::view_constants::ViewConstants::builder
// 信息 ID: fc66e60f3f15ac6e9a44eb06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=571, inline_record=138988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=571, inline_record=141780; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=571, inline_record=141948; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=571, inline_record=142116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::view_constants::ViewConstants::set_pixel_offset
// 信息 ID: 7b4cc1e104f532614d19a5da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants @ 0xaf440b -> 0xb18af0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::render::view_constants::ViewportConstantBuilder::build; module=579, inline_record=23436; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::view_constants::ViewportConstantBuilder::build
// 信息 ID: c80fe5b092ac983085c5bf8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants @ 0xaf4363 -> 0xb18f60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants @ 0xaf4821 -> 0xb18f60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants @ 0xaf492a -> 0xb18f60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants @ 0xaf4a3a -> 0xb18f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::view_constants::impl$4::default
// 信息 ID: 2e34d04cdffe5366cbfd8da3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::array::impl$56::default<country_core::render::view_constants::ViewConstants>; module=39, inline_record=22732; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$56::default<country_core::render::view_constants::ViewConstants>; module=39, inline_record=22760; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$56::default<country_core::render::view_constants::ViewConstants>; module=39, inline_record=22788; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$56::default<country_core::render::view_constants::ViewConstants>; module=39, inline_record=22816; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$56::default<country_core::render::view_constants::ViewConstants>; module=39, inline_record=22844; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$56::default<country_core::render::view_constants::ViewConstants>; module=1695, inline_record=146192; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$56::default<country_core::render::view_constants::ViewConstants>; module=1695, inline_record=146220; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$56::default<country_core::render::view_constants::ViewConstants>; module=1695, inline_record=146248; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
