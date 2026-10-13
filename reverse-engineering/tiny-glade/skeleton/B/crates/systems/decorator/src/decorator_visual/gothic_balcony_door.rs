// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3208afeac133bbf1c5213b3a63bfb58aca341c04293cb492de37cbd96ea46e9e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gothic_balcony_door.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 443
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 0, 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6342
// 首条位置: {"module":443,"inlinee":25410}
// 编译实例 RVAs: 0x7f3340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140257ba0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5879893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377045312,"raw_hex":"e8ab47a6ff","size":5,"soft_fail":false,"target_va":5371165600,"va":5377045488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1964771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377045312,"raw_hex":"e81d05e2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377046254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1965004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377045312,"raw_hex":"e83404e2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377046503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1529481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377045312,"raw_hex":"e877a9e8ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5377045508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2234774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377045312,"raw_hex":"e896192200","size":5,"soft_fail":false,"target_va":5379280368,"va":5377045589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34494350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377045312,"raw_hex":"e88e570e02","size":5,"soft_fail":false,"target_va":5411540992,"va":5377046637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":443,"record_offset":17672,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":443,"record_offset":16924,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":443,"record_offset":14880,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":4643,"source_line":118,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":443,"record_offset":15000,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":443,"record_offset":15056,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":17456,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":15564,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":16508,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":15772,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":15828,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::add_door_stairs
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":443,"record_offset":23880,"parent_record_offset":21548,"ancestor_record_offset":21548,"inlinee":25413,"source_line":258,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":443,"record_offset":15028,"parent_record_offset":14764,"ancestor_record_offset":14764,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":443,"record_offset":22828,"parent_record_offset":21548,"ancestor_record_offset":21548,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":21548,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":21548,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":24208,"parent_record_offset":24184,"ancestor_record_offset":21548,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":23956,"parent_record_offset":23916,"ancestor_record_offset":21548,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":23988,"parent_record_offset":23956,"ancestor_record_offset":21548,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":24184,"parent_record_offset":24160,"ancestor_record_offset":21548,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":443,"record_offset":23916,"parent_record_offset":23880,"ancestor_record_offset":21548,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":443,"record_offset":24160,"parent_record_offset":24136,"ancestor_record_offset":21548,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":443,"record_offset":14912,"parent_record_offset":14880,"ancestor_record_offset":14764,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":443,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":14764,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":443,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":14764,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":443,"record_offset":15908,"parent_record_offset":15884,"ancestor_record_offset":14764,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":15320,"parent_record_offset":15080,"ancestor_record_offset":14764,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":15228,"parent_record_offset":15172,"ancestor_record_offset":14764,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":15120,"parent_record_offset":15080,"ancestor_record_offset":14764,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":15464,"parent_record_offset":15348,"ancestor_record_offset":14764,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":15436,"parent_record_offset":15348,"ancestor_record_offset":14764,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":443,"record_offset":15592,"parent_record_offset":15564,"ancestor_record_offset":14764,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":443,"record_offset":15796,"parent_record_offset":15772,"ancestor_record_offset":14764,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":443,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":14764,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":443,"record_offset":16536,"parent_record_offset":16508,"ancestor_record_offset":14764,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":443,"record_offset":24112,"parent_record_offset":23880,"ancestor_record_offset":21548,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":443,"record_offset":24064,"parent_record_offset":24036,"ancestor_record_offset":21548,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":443,"record_offset":15080,"parent_record_offset":15056,"ancestor_record_offset":14764,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":443,"record_offset":15348,"parent_record_offset":15080,"ancestor_record_offset":14764,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":443,"record_offset":15956,"parent_record_offset":15932,"ancestor_record_offset":14764,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":443,"record_offset":15932,"parent_record_offset":15908,"ancestor_record_offset":14764,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":17560,"parent_record_offset":17516,"ancestor_record_offset":14764,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":17516,"parent_record_offset":17484,"ancestor_record_offset":14764,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":17484,"parent_record_offset":17456,"ancestor_record_offset":14764,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":443,"record_offset":15172,"parent_record_offset":15148,"ancestor_record_offset":14764,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":443,"record_offset":15148,"parent_record_offset":15080,"ancestor_record_offset":14764,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":443,"record_offset":17600,"parent_record_offset":17560,"ancestor_record_offset":14764,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":443,"record_offset":1924,"rva":8325376,"va":5377034496}
// 编译实例 RVAs: 0x7f0900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7771798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e86a6989ff","size":5,"soft_fail":false,"target_va":5369267664,"va":5377039457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140257ba0
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5874952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8f85aa6ff","size":5,"soft_fail":false,"target_va":5371165600,"va":5377040547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2359726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e852fedbff","size":5,"soft_fail":false,"target_va":5374676336,"va":5377036057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2359003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e82501dcff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377035686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1956595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e80d25e2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377038078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1957193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8b722e2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377038692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1524533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8cbbce8ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5377040560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t270935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e857220400","size":5,"soft_fail":false,"target_va":5377307216,"va":5377036276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t833166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e88eb60c00","size":5,"soft_fail":false,"target_va":5377869088,"va":5377035917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t866800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8f0390d00","size":5,"soft_fail":false,"target_va":5377906288,"va":5377039483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2239705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8d92c2200","size":5,"soft_fail":false,"target_va":5379280368,"va":5377040658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2270509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e82da52200","size":5,"soft_fail":false,"target_va":5379305728,"va":5377035214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3361926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8864c3300","size":5,"soft_fail":false,"target_va":5380398224,"va":5377036293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3366586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8ba5e3300","size":5,"soft_fail":false,"target_va":5380401776,"va":5377035185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3577314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8e2953600","size":5,"soft_fail":false,"target_va":5380621856,"va":5377044537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4897937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e891bc4a00","size":5,"soft_fail":false,"target_va":5381934528,"va":5377036586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7dc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10513439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e81f6ca000","size":5,"soft_fail":false,"target_va":5387550144,"va":5377036700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10927979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e86bbfa600","size":5,"soft_fail":false,"target_va":5387964304,"va":5377036320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412898e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11105224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8c873a900","size":5,"soft_fail":false,"target_va":5388146912,"va":5377041683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34497422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e88e630e02","size":5,"soft_fail":false,"target_va":5411540992,"va":5377043565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-2679306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8f61dd7ff","size":5,"soft_fail":false,"target_va":5374359040,"va":5377038341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34498325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e815670e02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377043254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34498878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e83e690e02","size":5,"soft_fail":false,"target_va":5411541808,"va":5377042925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34498875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e83b690e02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377042896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34498711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e897680e02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377043348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34498678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e876680e02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377043613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34498861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e82d690e02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377043038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8dbfaffff","size":5,"soft_fail":false,"target_va":5377033344,"va":5377034656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34508622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e84e8f0e02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377034733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorDst::as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t267482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8da140400","size":5,"soft_fail":false,"target_va":5377307920,"va":5377040433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t270985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e889220400","size":5,"soft_fail":false,"target_va":5377307232,"va":5377036242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2270111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e89fa32200","size":5,"soft_fail":false,"target_va":5379305584,"va":5377035468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3365636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8045b3300","size":5,"soft_fail":false,"target_va":5380401520,"va":5377035879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4350371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8a3614200","size":5,"soft_fail":false,"target_va":5381385536,"va":5377035160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4355560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8e8754200","size":5,"soft_fail":false,"target_va":5381390640,"va":5377035075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4343699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e893474200","size":5,"soft_fail":false,"target_va":5381386480,"va":5377042776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4350073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e879604200","size":5,"soft_fail":false,"target_va":5381384704,"va":5377034626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34601620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e894fa0f02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377043175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34493857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8a1550e02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377043242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::add_door_stairs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t209803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e88b330300","size":5,"soft_fail":false,"target_va":5377251376,"va":5377041568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t212125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e89d3c0300","size":5,"soft_fail":false,"target_va":5377249968,"va":5377037838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t215542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8f6490300","size":5,"soft_fail":false,"target_va":5377251504,"va":5377035957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t15942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e8463e0000","size":5,"soft_fail":false,"target_va":5377056368,"va":5377040421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e887a4ffff","size":5,"soft_fail":false,"target_va":5377012992,"va":5377036404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4878682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e85a714a00","size":5,"soft_fail":false,"target_va":5381916256,"va":5377037569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4849806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377034496,"raw_hex":"e88e004a00","size":5,"soft_fail":false,"target_va":5381886416,"va":5377036605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":443,"record_offset":13588,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":443,"record_offset":13888,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":443,"record_offset":7436,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":443,"record_offset":7492,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":443,"record_offset":4712,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":443,"record_offset":4684,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":443,"record_offset":13984,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":443,"record_offset":7520,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":14666,"source_line":26,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":443,"record_offset":18688,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4643,"source_line":118,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":8624,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25099,"source_line":291,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":9120,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24795,"source_line":298,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":10500,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24802,"source_line":309,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":4628,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":443,"record_offset":11992,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":8904,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":9092,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":11936,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":11880,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":13651,"source_line":1297,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":12860,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":8816,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":443,"record_offset":3644,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":443,"record_offset":2220,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":443,"record_offset":2668,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":443,"record_offset":7548,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":443,"record_offset":12948,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24805,"source_line":262,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
//     evidence={"module":443,"record_offset":14764,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25410,"source_line":195,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
//     evidence={"module":443,"record_offset":13072,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25409,"source_line":252,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":443,"record_offset":14264,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":443,"record_offset":14236,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":443,"record_offset":17704,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25127,"source_line":1171,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":443,"record_offset":18004,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":443,"record_offset":8788,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":13184,"parent_record_offset":13160,"ancestor_record_offset":1924,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":1924,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":1924,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":13128,"parent_record_offset":13072,"ancestor_record_offset":1924,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":13160,"parent_record_offset":13128,"ancestor_record_offset":1924,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":14152,"parent_record_offset":14128,"ancestor_record_offset":1924,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":443,"record_offset":13736,"parent_record_offset":13668,"ancestor_record_offset":1924,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":443,"record_offset":14128,"parent_record_offset":14104,"ancestor_record_offset":1924,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":443,"record_offset":13800,"parent_record_offset":13768,"ancestor_record_offset":1924,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":443,"record_offset":13768,"parent_record_offset":13736,"ancestor_record_offset":1924,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":443,"record_offset":14104,"parent_record_offset":14080,"ancestor_record_offset":1924,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":443,"record_offset":21164,"parent_record_offset":21128,"ancestor_record_offset":1924,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":443,"record_offset":10320,"parent_record_offset":9156,"ancestor_record_offset":1924,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":443,"record_offset":20976,"parent_record_offset":20952,"ancestor_record_offset":1924,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":443,"record_offset":13668,"parent_record_offset":13588,"ancestor_record_offset":1924,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":443,"record_offset":18040,"parent_record_offset":18004,"ancestor_record_offset":1924,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":443,"record_offset":14008,"parent_record_offset":13984,"ancestor_record_offset":1924,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":443,"record_offset":14056,"parent_record_offset":14032,"ancestor_record_offset":1924,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":443,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":1924,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":443,"record_offset":14912,"parent_record_offset":14880,"ancestor_record_offset":1924,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":443,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":1924,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":443,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":1924,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":443,"record_offset":3980,"parent_record_offset":3952,"ancestor_record_offset":1924,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":443,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":1924,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":443,"record_offset":4008,"parent_record_offset":3980,"ancestor_record_offset":1924,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":443,"record_offset":3928,"parent_record_offset":3900,"ancestor_record_offset":1924,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":443,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":1924,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":443,"record_offset":4124,"parent_record_offset":3952,"ancestor_record_offset":1924,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":443,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":1924,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":443,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":1924,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":443,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":1924,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":443,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":1924,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":443,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":1924,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":443,"record_offset":5216,"parent_record_offset":5180,"ancestor_record_offset":1924,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":12520,"parent_record_offset":12100,"ancestor_record_offset":1924,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":12428,"parent_record_offset":12372,"ancestor_record_offset":1924,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":1924,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":12228,"parent_record_offset":12144,"ancestor_record_offset":1924,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":443,"record_offset":12292,"parent_record_offset":12100,"ancestor_record_offset":1924,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":443,"record_offset":17672,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":443,"record_offset":16924,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":443,"record_offset":13040,"parent_record_offset":12948,"ancestor_record_offset":1924,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":443,"record_offset":8744,"parent_record_offset":8720,"ancestor_record_offset":1924,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":443,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":1924,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":443,"record_offset":7216,"parent_record_offset":6520,"ancestor_record_offset":1924,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":443,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":1924,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":443,"record_offset":2368,"parent_record_offset":2336,"ancestor_record_offset":1924,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":443,"record_offset":5024,"parent_record_offset":4996,"ancestor_record_offset":1924,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":443,"record_offset":4996,"parent_record_offset":4968,"ancestor_record_offset":1924,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":443,"record_offset":12888,"parent_record_offset":12860,"ancestor_record_offset":1924,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":443,"record_offset":9056,"parent_record_offset":8928,"ancestor_record_offset":1924,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":443,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":1924,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":443,"record_offset":8844,"parent_record_offset":8816,"ancestor_record_offset":1924,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":443,"record_offset":3796,"parent_record_offset":3752,"ancestor_record_offset":1924,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":443,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":1924,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":443,"record_offset":12980,"parent_record_offset":12948,"ancestor_record_offset":1924,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":443,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":1924,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":443,"record_offset":5408,"parent_record_offset":4712,"ancestor_record_offset":1924,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":443,"record_offset":7392,"parent_record_offset":6324,"ancestor_record_offset":1924,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":443,"record_offset":8688,"parent_record_offset":8660,"ancestor_record_offset":1924,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":443,"record_offset":2616,"parent_record_offset":2588,"ancestor_record_offset":1924,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":443,"record_offset":4584,"parent_record_offset":3752,"ancestor_record_offset":1924,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":443,"record_offset":4188,"parent_record_offset":3900,"ancestor_record_offset":1924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":443,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":1924,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":443,"record_offset":21196,"parent_record_offset":21164,"ancestor_record_offset":1924,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":443,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":1924,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":443,"record_offset":13916,"parent_record_offset":13888,"ancestor_record_offset":1924,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":443,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":1924,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":443,"record_offset":3264,"parent_record_offset":3236,"ancestor_record_offset":1924,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":443,"record_offset":13944,"parent_record_offset":13916,"ancestor_record_offset":1924,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":443,"record_offset":13236,"parent_record_offset":13208,"ancestor_record_offset":1924,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":443,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":1924,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":443,"record_offset":18072,"parent_record_offset":18040,"ancestor_record_offset":1924,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":443,"record_offset":18104,"parent_record_offset":18072,"ancestor_record_offset":1924,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":443,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":1924,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":443,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":1924,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":443,"record_offset":3336,"parent_record_offset":3308,"ancestor_record_offset":1924,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":443,"record_offset":2984,"parent_record_offset":2868,"ancestor_record_offset":1924,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":443,"record_offset":2336,"parent_record_offset":2308,"ancestor_record_offset":1924,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":443,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":1924,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":443,"record_offset":2836,"parent_record_offset":2804,"ancestor_record_offset":1924,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":443,"record_offset":2804,"parent_record_offset":2768,"ancestor_record_offset":1924,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":443,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":1924,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":443,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":1924,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":443,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":1924,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":443,"record_offset":3308,"parent_record_offset":2768,"ancestor_record_offset":1924,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":443,"record_offset":2308,"parent_record_offset":2276,"ancestor_record_offset":1924,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":443,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1924,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":443,"record_offset":2468,"parent_record_offset":2436,"ancestor_record_offset":1924,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":443,"record_offset":2564,"parent_record_offset":2468,"ancestor_record_offset":1924,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":443,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1924,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":443,"record_offset":2252,"parent_record_offset":2220,"ancestor_record_offset":1924,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":443,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":1924,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":443,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":1924,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":443,"record_offset":3152,"parent_record_offset":2868,"ancestor_record_offset":1924,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":443,"record_offset":3124,"parent_record_offset":2868,"ancestor_record_offset":1924,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":443,"record_offset":2928,"parent_record_offset":2868,"ancestor_record_offset":1924,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":443,"record_offset":10348,"parent_record_offset":10320,"ancestor_record_offset":1924,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":443,"record_offset":21012,"parent_record_offset":20976,"ancestor_record_offset":1924,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":443,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":1924,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":443,"record_offset":21036,"parent_record_offset":21012,"ancestor_record_offset":1924,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":443,"record_offset":21068,"parent_record_offset":20976,"ancestor_record_offset":1924,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":443,"record_offset":14880,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":4643,"source_line":118,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":443,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":1924,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":443,"record_offset":6324,"parent_record_offset":6288,"ancestor_record_offset":1924,"inlinee":24794,"source_line":77,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":9156,"parent_record_offset":9120,"ancestor_record_offset":1924,"inlinee":24797,"source_line":249,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":8660,"parent_record_offset":8624,"ancestor_record_offset":1924,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":10540,"parent_record_offset":10500,"ancestor_record_offset":1924,"inlinee":24803,"source_line":258,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$22::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":10092,"parent_record_offset":10064,"ancestor_record_offset":1924,"inlinee":24799,"source_line":94,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":1924,"inlinee":24813,"source_line":252,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":8720,"parent_record_offset":8688,"ancestor_record_offset":1924,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":443,"record_offset":11812,"parent_record_offset":11788,"ancestor_record_offset":1924,"inlinee":24815,"source_line":261,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":443,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":1924,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":443,"record_offset":15000,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":443,"record_offset":15056,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":443,"record_offset":12100,"parent_record_offset":11992,"ancestor_record_offset":1924,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":443,"record_offset":12548,"parent_record_offset":12100,"ancestor_record_offset":1924,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":443,"record_offset":12064,"parent_record_offset":12040,"ancestor_record_offset":1924,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":443,"record_offset":12040,"parent_record_offset":12016,"ancestor_record_offset":1924,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":8988,"parent_record_offset":8956,"ancestor_record_offset":1924,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":8956,"parent_record_offset":8928,"ancestor_record_offset":1924,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":1924,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":17456,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":15564,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":443,"record_offset":16508,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":18312,"parent_record_offset":18284,"ancestor_record_offset":1924,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":15772,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":18552,"parent_record_offset":18312,"ancestor_record_offset":1924,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":443,"record_offset":15828,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":443,"record_offset":12172,"parent_record_offset":12144,"ancestor_record_offset":1924,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":443,"record_offset":12144,"parent_record_offset":12100,"ancestor_record_offset":1924,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":443,"record_offset":4932,"parent_record_offset":4896,"ancestor_record_offset":1924,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":443,"record_offset":4968,"parent_record_offset":4932,"ancestor_record_offset":1924,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":443,"record_offset":5064,"parent_record_offset":4896,"ancestor_record_offset":1924,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":443,"record_offset":4896,"parent_record_offset":4844,"ancestor_record_offset":1924,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":443,"record_offset":6632,"parent_record_offset":6520,"ancestor_record_offset":1924,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":443,"record_offset":5096,"parent_record_offset":4844,"ancestor_record_offset":1924,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":443,"record_offset":7244,"parent_record_offset":6520,"ancestor_record_offset":1924,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":443,"record_offset":5180,"parent_record_offset":5152,"ancestor_record_offset":1924,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":443,"record_offset":5152,"parent_record_offset":4844,"ancestor_record_offset":1924,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":443,"record_offset":6716,"parent_record_offset":6520,"ancestor_record_offset":1924,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":443,"record_offset":6604,"parent_record_offset":6520,"ancestor_record_offset":1924,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":443,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":1924,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":443,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":1924,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":443,"record_offset":6412,"parent_record_offset":6384,"ancestor_record_offset":1924,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":443,"record_offset":7100,"parent_record_offset":6932,"ancestor_record_offset":1924,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":443,"record_offset":4768,"parent_record_offset":4740,"ancestor_record_offset":1924,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":443,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":1924,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":443,"record_offset":6444,"parent_record_offset":6412,"ancestor_record_offset":1924,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":7040,"parent_record_offset":6956,"ancestor_record_offset":1924,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":6984,"parent_record_offset":6956,"ancestor_record_offset":1924,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":5252,"parent_record_offset":4844,"ancestor_record_offset":1924,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":7336,"parent_record_offset":6520,"ancestor_record_offset":1924,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":4844,"parent_record_offset":4804,"ancestor_record_offset":1924,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":1924,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":1924,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":6472,"parent_record_offset":6412,"ancestor_record_offset":1924,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":1924,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":6576,"parent_record_offset":6520,"ancestor_record_offset":1924,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":4804,"parent_record_offset":4768,"ancestor_record_offset":1924,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":443,"record_offset":6932,"parent_record_offset":6520,"ancestor_record_offset":1924,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":443,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":1924,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":443,"record_offset":3680,"parent_record_offset":3644,"ancestor_record_offset":1924,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":443,"record_offset":19120,"parent_record_offset":19088,"ancestor_record_offset":1924,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":443,"record_offset":19636,"parent_record_offset":19376,"ancestor_record_offset":1924,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":443,"record_offset":3824,"parent_record_offset":3796,"ancestor_record_offset":1924,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":443,"record_offset":3900,"parent_record_offset":3752,"ancestor_record_offset":1924,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":443,"record_offset":4160,"parent_record_offset":3900,"ancestor_record_offset":1924,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":443,"record_offset":6356,"parent_record_offset":6324,"ancestor_record_offset":1924,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":443,"record_offset":4740,"parent_record_offset":4712,"ancestor_record_offset":1924,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":443,"record_offset":5528,"parent_record_offset":5496,"ancestor_record_offset":1924,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":443,"record_offset":9020,"parent_record_offset":8988,"ancestor_record_offset":1924,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":443,"record_offset":3852,"parent_record_offset":3824,"ancestor_record_offset":1924,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":443,"record_offset":3752,"parent_record_offset":3716,"ancestor_record_offset":1924,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":443,"record_offset":3716,"parent_record_offset":3680,"ancestor_record_offset":1924,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":443,"record_offset":18148,"parent_record_offset":18004,"ancestor_record_offset":1924,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":443,"record_offset":17972,"parent_record_offset":17704,"ancestor_record_offset":1924,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":443,"record_offset":17944,"parent_record_offset":17704,"ancestor_record_offset":1924,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":443,"record_offset":17748,"parent_record_offset":17704,"ancestor_record_offset":1924,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":443,"record_offset":17780,"parent_record_offset":17748,"ancestor_record_offset":1924,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":443,"record_offset":17880,"parent_record_offset":17852,"ancestor_record_offset":1924,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":443,"record_offset":17852,"parent_record_offset":17704,"ancestor_record_offset":1924,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":443,"record_offset":15028,"parent_record_offset":14764,"ancestor_record_offset":1924,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":443,"record_offset":18284,"parent_record_offset":18004,"ancestor_record_offset":1924,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6341
// 首条位置: {"module":443,"inlinee":25409}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":13128,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":13184,"parent_record_offset":13160,"ancestor_record_offset":13072,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":13072,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":443,"record_offset":13160,"parent_record_offset":13128,"ancestor_record_offset":13072,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":443,"record_offset":13236,"parent_record_offset":13208,"ancestor_record_offset":13072,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00443.jsonl.gz"}
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
// 完整结构及原始类型记录: gothic_balcony_door.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gothic_balcony_door.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gothic_balcony_door.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs
// 信息 ID: bb82f8942a7e248eb0b79ddc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f5221 -> 0x7f3340 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=14764; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors
// 信息 ID: 61d7e19333ee1602e133c69a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dc172a4ba3b8f88c1aeeacf2: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> > @ 0x256f7b -> 0x7f0900 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> > > @ 0x42fb7c -> 0x7f0900 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> > > @ 0x481f55 -> 0x7f0900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
// 信息 ID: b547b2b24598963888211554
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=13072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
