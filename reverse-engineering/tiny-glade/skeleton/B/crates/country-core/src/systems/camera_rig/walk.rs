// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/camera_rig/walk.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9469091ddde30f22fe6622e379e6cbbce0bd7603b0361c78448b5d56ad1db35d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: walk.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 524
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use dolly; // namespace candidate
//   package identities: [{"name":"dolly","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$2::translate_screen_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":14704,"rva":10045184,"va":5378754304}
// 编译实例 RVAs: 0x994700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3178772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378754304,"raw_hex":"e814813000","size":5,"soft_fail":false,"target_va":5381934560,"va":5378755783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33092326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378754304,"raw_hex":"e8e6f2f801","size":5,"soft_fail":false,"target_va":5411847872,"va":5378755541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32785624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378754304,"raw_hex":"e8d844f401","size":5,"soft_fail":false,"target_va":5411541904,"va":5378756275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::Raycasting::spherecast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-820902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378754304,"raw_hex":"e85a79f3ff","size":5,"soft_fail":false,"target_va":5377934224,"va":5378755121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*56(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378754304,"raw_hex":"ff5038","size":3,"soft_fail":false,"va":5378754900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":17012,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":524,"record_offset":18120,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":524,"record_offset":18408,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":28571,"source_line":138,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::position::Position::translate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":524,"record_offset":20468,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":26681,"source_line":34,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":16356,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":29974,"source_line":48,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":20024,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":28518,"source_line":31,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":524,"record_offset":14996,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":25347,"source_line":923,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":16980,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":16096,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":14940,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":17400,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":15900,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4891,"source_line":573,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::project_onto_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":17660,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":29989,"source_line":627,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":16180,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":17308,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":13663,"source_line":1280,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":18292,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":17220,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":16924,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":17788,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":16292,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":524,"record_offset":18380,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":14704,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":14704,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":14704,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":14704,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":14704,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":16420,"parent_record_offset":16384,"ancestor_record_offset":14704,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":20280,"parent_record_offset":20256,"ancestor_record_offset":14704,"inlinee":12656,"source_line":261,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":16792,"parent_record_offset":16768,"ancestor_record_offset":14704,"inlinee":10550,"source_line":225,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":14704,"inlinee":10551,"source_line":193,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":16884,"parent_record_offset":16856,"ancestor_record_offset":14704,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":18168,"parent_record_offset":18144,"ancestor_record_offset":14704,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":524,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":14704,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":524,"record_offset":15776,"parent_record_offset":15128,"ancestor_record_offset":14704,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":524,"record_offset":15684,"parent_record_offset":15628,"ancestor_record_offset":14704,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":524,"record_offset":15224,"parent_record_offset":15200,"ancestor_record_offset":14704,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":524,"record_offset":15256,"parent_record_offset":15172,"ancestor_record_offset":14704,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":524,"record_offset":15320,"parent_record_offset":15128,"ancestor_record_offset":14704,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":18780,"parent_record_offset":18752,"ancestor_record_offset":14704,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":18808,"parent_record_offset":18752,"ancestor_record_offset":14704,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":524,"record_offset":20428,"parent_record_offset":20372,"ancestor_record_offset":14704,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":524,"record_offset":18144,"parent_record_offset":18120,"ancestor_record_offset":14704,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":18320,"parent_record_offset":18292,"ancestor_record_offset":14704,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":16204,"parent_record_offset":16180,"ancestor_record_offset":14704,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":16324,"parent_record_offset":16292,"ancestor_record_offset":14704,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":17248,"parent_record_offset":17220,"ancestor_record_offset":14704,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":17816,"parent_record_offset":17788,"ancestor_record_offset":14704,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":17340,"parent_record_offset":17308,"ancestor_record_offset":14704,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":14704,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":524,"record_offset":20348,"parent_record_offset":20024,"ancestor_record_offset":14704,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":16728,"parent_record_offset":16704,"ancestor_record_offset":14704,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":16652,"parent_record_offset":16616,"ancestor_record_offset":14704,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":14704,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":20192,"parent_record_offset":20168,"ancestor_record_offset":14704,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":16588,"parent_record_offset":16384,"ancestor_record_offset":14704,"inlinee":11874,"source_line":313,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":16616,"parent_record_offset":16588,"ancestor_record_offset":14704,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":16680,"parent_record_offset":16616,"ancestor_record_offset":14704,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":20076,"parent_record_offset":20052,"ancestor_record_offset":14704,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":20104,"parent_record_offset":20076,"ancestor_record_offset":14704,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":20168,"parent_record_offset":20104,"ancestor_record_offset":14704,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":524,"record_offset":18448,"parent_record_offset":18408,"ancestor_record_offset":14704,"inlinee":28491,"source_line":159,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":524,"record_offset":18692,"parent_record_offset":18408,"ancestor_record_offset":14704,"inlinee":28493,"source_line":164,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":16384,"parent_record_offset":16356,"ancestor_record_offset":14704,"inlinee":29975,"source_line":58,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":20052,"parent_record_offset":20024,"ancestor_record_offset":14704,"inlinee":28519,"source_line":41,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":16856,"parent_record_offset":16384,"ancestor_record_offset":14704,"inlinee":29981,"source_line":49,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":14704,"inlinee":28524,"source_line":32,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":16768,"parent_record_offset":16588,"ancestor_record_offset":14704,"inlinee":29977,"source_line":61,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":20256,"parent_record_offset":20076,"ancestor_record_offset":14704,"inlinee":28522,"source_line":44,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":524,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":14704,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":524,"record_offset":15020,"parent_record_offset":14996,"ancestor_record_offset":14704,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":524,"record_offset":15128,"parent_record_offset":15020,"ancestor_record_offset":14704,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":524,"record_offset":15376,"parent_record_offset":15128,"ancestor_record_offset":14704,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":524,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":14704,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":524,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":14704,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":524,"record_offset":18728,"parent_record_offset":18692,"ancestor_record_offset":14704,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":524,"record_offset":18472,"parent_record_offset":18448,"ancestor_record_offset":14704,"inlinee":28492,"source_line":764,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":524,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":14704,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":524,"record_offset":18504,"parent_record_offset":18472,"ancestor_record_offset":14704,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":524,"record_offset":18752,"parent_record_offset":18728,"ancestor_record_offset":14704,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":524,"record_offset":18984,"parent_record_offset":18692,"ancestor_record_offset":14704,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":524,"record_offset":19040,"parent_record_offset":18692,"ancestor_record_offset":14704,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":524,"record_offset":18844,"parent_record_offset":18692,"ancestor_record_offset":14704,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":14964,"parent_record_offset":14940,"ancestor_record_offset":14704,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":15984,"parent_record_offset":15960,"ancestor_record_offset":14704,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":15960,"parent_record_offset":15928,"ancestor_record_offset":14704,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":15928,"parent_record_offset":15900,"ancestor_record_offset":14704,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":20496,"parent_record_offset":20468,"ancestor_record_offset":14704,"inlinee":26640,"source_line":1081,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":524,"record_offset":15200,"parent_record_offset":15172,"ancestor_record_offset":14704,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":524,"record_offset":15172,"parent_record_offset":15128,"ancestor_record_offset":14704,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":524,"record_offset":16048,"parent_record_offset":16016,"ancestor_record_offset":14704,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::WalkCamera::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":10312,"rva":10042912,"va":5378752032}
// 编译实例 RVAs: 0x993e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=2
//     target_aliases=130 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9852792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378752032,"raw_hex":"e888a869ff","size":5,"soft_fail":false,"target_va":5368899840,"va":5378752627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cfe90
//     owner=unresolved_namespace; count=3
//     target_aliases=40 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3948977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378752032,"raw_hex":"e84fbec3ff","size":5,"soft_fail":false,"target_va":5374803600,"va":5378752572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3670624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378752032,"raw_hex":"e8a0fdc7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378752107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32788481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378752032,"raw_hex":"e80150f401","size":5,"soft_fail":false,"target_va":5411540992,"va":5378752506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4397168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378752032,"raw_hex":"e890e7bcff","size":5,"soft_fail":false,"target_va":5374354992,"va":5378752155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378752032,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5378752384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::builder
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":10460,"parent_record_offset":10312,"ancestor_record_offset":10312,"inlinee":28500,"source_line":86,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRigBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":11392,"parent_record_offset":10312,"ancestor_record_offset":10312,"inlinee":28503,"source_line":105,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRigBuilder::with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":10488,"parent_record_offset":10312,"ancestor_record_offset":10312,"inlinee":28502,"source_line":100,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":10312,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":10312,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":10312,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":10596,"parent_record_offset":10564,"ancestor_record_offset":10312,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":524,"record_offset":10536,"parent_record_offset":10488,"ancestor_record_offset":10312,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":10312,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":10824,"parent_record_offset":10800,"ancestor_record_offset":10312,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":10312,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":10752,"parent_record_offset":10716,"ancestor_record_offset":10312,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":10716,"parent_record_offset":10488,"ancestor_record_offset":10312,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":10896,"parent_record_offset":10488,"ancestor_record_offset":10312,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":10312,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":11624,"parent_record_offset":11496,"ancestor_record_offset":10312,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":10312,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":10864,"parent_record_offset":10716,"ancestor_record_offset":10312,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":10312,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":11496,"parent_record_offset":11444,"ancestor_record_offset":10312,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":11540,"parent_record_offset":11496,"ancestor_record_offset":10312,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":11444,"parent_record_offset":11392,"ancestor_record_offset":10312,"inlinee":28504,"source_line":67,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::WalkCamera::teleport
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":11696,"rva":10043696,"va":5378752816}
// 编译实例 RVAs: 0x994130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$glam..euler..EulerRot$u20$as$u20$glam..euler..EulerFromQuaternion$LT$glam..f32..sse2..quat..Quat$GT$$GT$::convert_quat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14493765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378752816,"raw_hex":"e84528dd00","size":5,"soft_fail":false,"target_va":5393246864,"va":5378753094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140334d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6681788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378752816,"raw_hex":"e8440b9aff","size":5,"soft_fail":false,"target_va":5372071296,"va":5378753079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32788624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378752816,"raw_hex":"e89050f401","size":5,"soft_fail":false,"target_va":5411541904,"va":5378753275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*64(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378752816,"raw_hex":"ff5040","size":3,"soft_fail":false,"va":5378752932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":524,"record_offset":11852,"parent_record_offset":11696,"ancestor_record_offset":11696,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::yaw_pitch::YawPitch::set_rotation_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/yaw_pitch.rs
//     evidence={"module":524,"record_offset":12724,"parent_record_offset":11696,"ancestor_record_offset":11696,"inlinee":29986,"source_line":75,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":11908,"parent_record_offset":11696,"ancestor_record_offset":11696,"inlinee":28518,"source_line":31,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":12156,"parent_record_offset":12136,"ancestor_record_offset":11696,"inlinee":12656,"source_line":261,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":11696,"inlinee":10551,"source_line":193,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":11696,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_degrees
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":12780,"parent_record_offset":12724,"ancestor_record_offset":11696,"inlinee":29988,"source_line":847,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":524,"record_offset":12224,"parent_record_offset":11908,"ancestor_record_offset":11696,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":11696,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":12020,"parent_record_offset":11988,"ancestor_record_offset":11696,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":12072,"parent_record_offset":12048,"ancestor_record_offset":11696,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":11960,"parent_record_offset":11936,"ancestor_record_offset":11696,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":11988,"parent_record_offset":11960,"ancestor_record_offset":11696,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":12048,"parent_record_offset":11988,"ancestor_record_offset":11696,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":11696,"inlinee":28519,"source_line":41,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":11696,"inlinee":28524,"source_line":32,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":12136,"parent_record_offset":11960,"ancestor_record_offset":11696,"inlinee":28522,"source_line":44,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::to_euler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":524,"record_offset":12752,"parent_record_offset":12724,"ancestor_record_offset":11696,"inlinee":29987,"source_line":397,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$34::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_mint.rs
//     evidence={"module":524,"record_offset":11876,"parent_record_offset":11852,"ancestor_record_offset":11696,"inlinee":26639,"source_line":47,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::WalkCamera::rotation
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":12816,"rva":10044176,"va":5378753296}
// 编译实例 RVAs: 0x994310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":524,"record_offset":12964,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":524,"record_offset":13004,"parent_record_offset":12984,"ancestor_record_offset":12816,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_mint.rs
//     evidence={"module":524,"record_offset":12984,"parent_record_offset":12964,"ancestor_record_offset":12816,"inlinee":12705,"source_line":104,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":5568,"rva":10041168,"va":5378750288}
// 编译实例 RVAs: 0x993750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33097488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378750288,"raw_hex":"e81007f901","size":5,"soft_fail":false,"target_va":5411847952,"va":5378750459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32791265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378750288,"raw_hex":"e8e15af401","size":5,"soft_fail":false,"target_va":5411541904,"va":5378750634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*64(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378750288,"raw_hex":"ff5040","size":3,"soft_fail":false,"va":5378750388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::drivers::yaw_pitch::YawPitch::rotate_yaw_pitch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/yaw_pitch.rs
//     evidence={"module":524,"record_offset":6160,"parent_record_offset":5568,"ancestor_record_offset":5568,"inlinee":29983,"source_line":68,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":5724,"parent_record_offset":5568,"ancestor_record_offset":5568,"inlinee":28518,"source_line":31,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":5972,"parent_record_offset":5952,"ancestor_record_offset":5568,"inlinee":12656,"source_line":261,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":5568,"inlinee":10551,"source_line":193,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":5568,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":5568,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":524,"record_offset":6120,"parent_record_offset":6064,"ancestor_record_offset":5568,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":524,"record_offset":6040,"parent_record_offset":5724,"ancestor_record_offset":5568,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":5568,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":5836,"parent_record_offset":5804,"ancestor_record_offset":5568,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":5568,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":5568,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":5804,"parent_record_offset":5776,"ancestor_record_offset":5568,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":5864,"parent_record_offset":5804,"ancestor_record_offset":5568,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":5752,"parent_record_offset":5724,"ancestor_record_offset":5568,"inlinee":28519,"source_line":41,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":5568,"inlinee":28524,"source_line":32,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":5952,"parent_record_offset":5776,"ancestor_record_offset":5568,"inlinee":28522,"source_line":44,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$2::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":13040,"rva":10041168,"va":5378750288}
// 编译实例 RVAs: 0x993750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33097488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378750288,"raw_hex":"e81007f901","size":5,"soft_fail":false,"target_va":5411847952,"va":5378750459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32791265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378750288,"raw_hex":"e8e15af401","size":5,"soft_fail":false,"target_va":5411541904,"va":5378750634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*64(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378750288,"raw_hex":"ff5040","size":3,"soft_fail":false,"va":5378750388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::drivers::yaw_pitch::YawPitch::rotate_yaw_pitch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/yaw_pitch.rs
//     evidence={"module":524,"record_offset":13632,"parent_record_offset":13040,"ancestor_record_offset":13040,"inlinee":29983,"source_line":68,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":13196,"parent_record_offset":13040,"ancestor_record_offset":13040,"inlinee":28518,"source_line":31,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":13444,"parent_record_offset":13424,"ancestor_record_offset":13040,"inlinee":12656,"source_line":261,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":13468,"parent_record_offset":13444,"ancestor_record_offset":13040,"inlinee":10551,"source_line":193,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":13564,"parent_record_offset":13536,"ancestor_record_offset":13040,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":13660,"parent_record_offset":13632,"ancestor_record_offset":13040,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":524,"record_offset":13592,"parent_record_offset":13536,"ancestor_record_offset":13040,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":524,"record_offset":13512,"parent_record_offset":13196,"ancestor_record_offset":13040,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":13384,"parent_record_offset":13360,"ancestor_record_offset":13040,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":13308,"parent_record_offset":13276,"ancestor_record_offset":13040,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":13360,"parent_record_offset":13336,"ancestor_record_offset":13040,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":13040,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":13276,"parent_record_offset":13248,"ancestor_record_offset":13040,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":13336,"parent_record_offset":13276,"ancestor_record_offset":13040,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":13224,"parent_record_offset":13196,"ancestor_record_offset":13040,"inlinee":28519,"source_line":41,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":13536,"parent_record_offset":13512,"ancestor_record_offset":13040,"inlinee":28524,"source_line":32,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":13424,"parent_record_offset":13248,"ancestor_record_offset":13040,"inlinee":28522,"source_line":44,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":13700,"rva":10044192,"va":5378753312}
// 编译实例 RVAs: 0x994320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t526348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378753312,"raw_hex":"e80c080800","size":5,"soft_fail":false,"target_va":5379280368,"va":5378754015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32787603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378753312,"raw_hex":"e8934cf401","size":5,"soft_fail":false,"target_va":5411541904,"va":5378754296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::Raycasting::spherecast
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-819495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378753312,"raw_hex":"e8d97ef3ff","size":5,"soft_fail":false,"target_va":5377934224,"va":5378753714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3128992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378753312,"raw_hex":"e8a0be2f00","size":5,"soft_fail":false,"target_va":5381883104,"va":5378754107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*64(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378753312,"raw_hex":"ff5040","size":3,"soft_fail":false,"va":5378753460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":14384,"parent_record_offset":13700,"ancestor_record_offset":13700,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":14448,"parent_record_offset":13700,"ancestor_record_offset":13700,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":524,"record_offset":14476,"parent_record_offset":13700,"ancestor_record_offset":13700,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":13884,"parent_record_offset":13700,"ancestor_record_offset":13700,"inlinee":28518,"source_line":31,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":524,"record_offset":14328,"parent_record_offset":13700,"ancestor_record_offset":13700,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":524,"record_offset":14504,"parent_record_offset":13700,"ancestor_record_offset":13700,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":13700,"inlinee":12656,"source_line":261,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":13700,"inlinee":10551,"source_line":193,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":524,"record_offset":14260,"parent_record_offset":14232,"ancestor_record_offset":13700,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":524,"record_offset":14288,"parent_record_offset":14232,"ancestor_record_offset":13700,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":13700,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":14556,"parent_record_offset":14504,"ancestor_record_offset":13700,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":13700,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":524,"record_offset":14208,"parent_record_offset":13884,"ancestor_record_offset":13700,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":14076,"parent_record_offset":14052,"ancestor_record_offset":13700,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":14000,"parent_record_offset":13964,"ancestor_record_offset":13700,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":14052,"parent_record_offset":14028,"ancestor_record_offset":13700,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":13936,"parent_record_offset":13912,"ancestor_record_offset":13700,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":13964,"parent_record_offset":13936,"ancestor_record_offset":13700,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":14028,"parent_record_offset":13964,"ancestor_record_offset":13700,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":13912,"parent_record_offset":13884,"ancestor_record_offset":13700,"inlinee":28519,"source_line":41,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":14232,"parent_record_offset":14208,"ancestor_record_offset":13700,"inlinee":28524,"source_line":32,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":14116,"parent_record_offset":13936,"ancestor_record_offset":13700,"inlinee":28522,"source_line":44,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":9952,"rva":10042672,"va":5378751792}
// 编译实例 RVAs: 0x993d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378751792,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5378751906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":10096,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":28504,"source_line":67,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] dolly::transform::Transform::into_position_rotation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/transform.rs
//     evidence={"module":524,"record_offset":10280,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":29985,"source_line":31,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":9952,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":10144,"parent_record_offset":10096,"ancestor_record_offset":9952,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":10240,"parent_record_offset":10200,"ancestor_record_offset":9952,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":10200,"parent_record_offset":10096,"ancestor_record_offset":9952,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$2::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":20704,"rva":10042672,"va":5378751792}
// 编译实例 RVAs: 0x993d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378751792,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5378751906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":524,"record_offset":20844,"parent_record_offset":20704,"ancestor_record_offset":20704,"inlinee":28504,"source_line":67,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":20916,"parent_record_offset":20892,"ancestor_record_offset":20704,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":20892,"parent_record_offset":20844,"ancestor_record_offset":20704,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":20988,"parent_record_offset":20948,"ancestor_record_offset":20704,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":20948,"parent_record_offset":20844,"ancestor_record_offset":20704,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
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
// 完整结构及原始类型记录: walk.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::walk::WalkCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::camera_rig::walk::WalkCamera>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::camera_rig::walk::WalkCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: walk.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::camera_rig::walk::WalkCamera", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: walk.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::rotate_yaw_pitch
// 信息 ID: bc8c6078f6142ed61d317163
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::update
// 信息 ID: bfa3b5550509cf377dcfb365
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::WalkCamera::new
// 信息 ID: b5c16563368d5ee09b172fc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::camera_rig::MainCamera::new @ 0x90f62f -> 0x993e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::WalkCamera::rotation
// 信息 ID: 7617c357345b7b08929fb1b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b9bae -> 0x994310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::WalkCamera::teleport
// 信息 ID: b07ce38c706f290038ae27db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3bab24 -> 0x994130 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3bc65b -> 0x994130 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::camera_rig::MainCamera::init_walk_camera @ 0x90e6c9 -> 0x994130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain
// 信息 ID: c85a33e67e39ddfc5397f273
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$2::rotate_yaw_pitch
// 信息 ID: 4489d353570df458f6889fad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$2::translate_screen_space
// 信息 ID: e61b9540a68f637d28ecdf66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$2::update
// 信息 ID: e7b102884b6c78f513dade6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
